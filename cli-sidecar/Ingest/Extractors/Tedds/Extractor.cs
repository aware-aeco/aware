// Tedds extractor — drives the crawl of developer.tekla.com/doc/tekla-tedds/<ver>/
// to produce a host-coverage IR for Tekla Tedds (versions 2025 and 2026).
//
// Surface size:
// -------------
// TSD ships ~50-60 types across 3 namespaces, totalling ~250-350 members. This is
// roughly 100x smaller than Tekla Structures (~1320 types, ~12000 members). The
// surface is small enough that we can crawl serially with no concurrency without
// running into a wall-clock concern (expected ~5-10 min end-to-end).
//
// Pipeline:
//
//   Phase 1 — namespace discovery (1 fetch):
//     Fetch the root page (/doc/tekla-tedds/<ver>/<rootId>), parse the sidebar tree
//     for namespace-leaf URLs (3 namespaces per version).
//
//   Phase 2 — type URL harvest (3 fetches):
//     For each namespace page, walk the sidebar tree's child entries (leaf-id="<id>"
//     <a href="/doc/tekla-tedds/<ver>/<slug>-<id>">) and collect every type URL.
//     We deduplicate before crawling.
//
//   Phase 3 — type-page crawl (sequential, ~50 fetches):
//     Fetch each type page and run PageParser.Parse() to produce a (TypeInfo,
//     MemberLinks) pair. After phase 3 we write a snapshot of the IR — types are
//     fully populated at type level but every member carries a placeholder signature
//     until enrichment.
//
//   Phase 4 — per-member enrichment (sequential, ~250-350 fetches):
//     For each member URL, fetch the per-member page and run MemberPageParser. The
//     member page URLs harvested from type pages are `/topic/en/...` hash redirects;
//     HttpClient follows the 302 automatically. We snapshot the IR every 100
//     enrichments so a kill-9 mid-run can resume.
//
//   Phase 5 — final IR emit:
//     Stamp metadata + extracted_at and return the final IR.
//
// Resumability mirrors the Tekla.Extractor design: outPath existence triggers a
// snapshot reload, member rows that still match the placeholder shape are re-enqueued.
//
// HttpClient discipline:
// ----------------------
// developer.tekla.com hosts both Tekla Structures (huge) and Tekla Tedds (tiny) on the
// same CDN. The CDN throttles after ~3000 fetches per IP/session. With ~400 fetches
// total per version, we are well within budget and use a single long-lived HttpScraper.

using System.Text.Json;
using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Tedds;

public static class Extractor
{
    // The Tekla Tedds root page is at a version-specific numeric ID, stable per release.
    // Verified 2026-05-19 against developer.tekla.com.
    static readonly Dictionary<string, (string version, int rootId)> KnownVersions = new()
    {
        ["25.0"] = ("2025", 41820),
        ["26.0"] = ("2026", 64124),
    };

    const string BaseHost = "https://developer.tekla.com";
    // Snapshot every 100 enrichments. With ~300 members total, this gives ~3 snapshots
    // per run. At 1-2 fetches/sec the cost is negligible (each snapshot is ~50-200 KB).
    const int SnapshotEvery = 100;

    enum MemberKind { Constructor, Method, Property, Event }

    sealed record MemberWorkItem(int TypeIdx, MemberKind Kind, int MemberIdx, string Url, string MemberName);

    /// <summary>
    /// Thread-safe append-only writer for the dedicated extraction-errors log.
    /// Created lazily on first write so a clean run leaves no on-disk file.
    /// </summary>
    sealed class ErrorsLog
    {
        readonly string? _path;
        readonly object _lock = new();
        public ErrorsLog(string? path) { _path = path; }

        public void Append(string phase, string url, string message)
        {
            if (string.IsNullOrEmpty(_path)) return;
            var safeMsg = (message ?? "").Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' ');
            var safeUrl = (url ?? "").Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' ');
            var line = $"{DateTime.UtcNow:O}\t{phase}\t{safeUrl}\t{safeMsg}{Environment.NewLine}";
            lock (_lock)
            {
                var dir = Path.GetDirectoryName(_path);
                if (!string.IsNullOrEmpty(dir)) Directory.CreateDirectory(dir);
                File.AppendAllText(_path, line);
            }
        }
    }

    static string? DeriveErrorsLogPath(string? outPath)
    {
        if (string.IsNullOrEmpty(outPath)) return null;
        var dir = Path.GetDirectoryName(outPath) ?? "";
        var fileName = Path.GetFileName(outPath);
        if (fileName.EndsWith(".ir.json", StringComparison.Ordinal))
            fileName = fileName[..^".ir.json".Length] + "-errors.log";
        else
            fileName += "-errors.log";
        return Path.Combine(dir, fileName);
    }

    public static Task<CoverageIR> Extract(string version) =>
        Extract(version, outPath: null);

    public static async Task<CoverageIR> Extract(string version, string? outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var v))
            throw new ArgumentException(
                $"Tedds extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var rootUrl = $"{BaseHost}/doc/tekla-tedds/{v.version}/{v.rootId}";
        var state = new SnapshotState();
        state.SourceUrls.Add(rootUrl);

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));

        // Try to resume from an existing snapshot.
        List<PageParser.ParseResult>? parsed = TryLoadSnapshot(version, outPath, rootUrl, state);

        await using var scraper = await HttpScraper.Launch();

        if (parsed is null)
        {
            Console.Error.WriteLine($"[tedds-extract] launched HTTP scraper; crawling {rootUrl}");

            var rootHtml = await scraper.FetchHtml(rootUrl);
            state.PageCount++;
            if (rootHtml.Length < 5000)
                throw new InvalidDataException(
                    $"Tedds root page suspiciously short ({rootHtml.Length} bytes; expected ~35KB). " +
                    $"Bot block or decompression regression? Head: {rootHtml.Substring(0, Math.Min(200, rootHtml.Length))}");
            var namespaceUrls = ExtractNamespaceUrls(rootHtml, v.version);
            Console.Error.WriteLine($"[tedds-extract] discovered {namespaceUrls.Count} namespaces");

            var typeUrls = new List<string>();
            foreach (var nsUrl in namespaceUrls)
            {
                try
                {
                    var nsHtml = await scraper.FetchHtml(nsUrl);
                    state.PageCount++;
                    var nsTypes = ExtractTypeUrls(nsHtml, v.version, nsUrl);
                    Console.Error.WriteLine($"[tedds-extract]   {nsUrl}: {nsTypes.Count} types");
                    typeUrls.AddRange(nsTypes);
                }
                catch (Exception ex)
                {
                    Console.Error.WriteLine($"[tedds-extract] WARN failed namespace {nsUrl}: {ex.Message}");
                    errorsLog.Append("namespace-fetch", nsUrl, ex.Message);
                }
            }

            typeUrls = typeUrls.Distinct().ToList();
            Console.Error.WriteLine($"[tedds-extract] total unique type URLs: {typeUrls.Count}");

            parsed = await CrawlTypePages(scraper, typeUrls, state, errorsLog);

            if (outPath is not null) WriteSnapshot(version, state, parsed, outPath);
        }
        else
        {
            Console.Error.WriteLine($"[tedds-extract] resumed from snapshot: {parsed.Count} types already on disk");
        }

        var enriched = await EnrichAllMembers(scraper, parsed, state, version, outPath, errorsLog);

        var builder = new IRBuilder("tedds", version, "scrape");
        foreach (var u in state.SourceUrls) builder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) builder.IncrementPageCount();
        foreach (var t in enriched) builder.AddType(t);

        Console.Error.WriteLine(
            $"[tedds-extract] done: types={enriched.Count}");

        return builder.Build();
    }

    // ── snapshot read/write ───────────────────────────────────────────────

    static List<PageParser.ParseResult>? TryLoadSnapshot(string version, string? outPath, string rootUrl, SnapshotState state)
    {
        if (string.IsNullOrEmpty(outPath) || !File.Exists(outPath)) return null;
        try
        {
            var json = File.ReadAllText(outPath);
            var ir = JsonSerializer.Deserialize(json, IrJsonContext.Default.CoverageIR);
            if (ir is null || ir.host != "tedds" || ir.host_version != version) return null;

            var results = new List<PageParser.ParseResult>(ir.types.Count);
            foreach (var t in ir.types)
            {
                var ctorUrls = t.constructors.Select(c => c.doc_url ?? "").ToList();
                var methodUrls = t.methods.Select(m => m.doc_url ?? "").ToList();
                var propUrls = t.properties.Select(p => p.doc_url ?? "").ToList();
                var eventUrls = t.events.Select(e => e.doc_url ?? "").ToList();
                var links = new PageParser.MemberLinks(ctorUrls, methodUrls, propUrls, eventUrls);
                results.Add(new PageParser.ParseResult(t, links));
            }
            state.PageCount = ir.metadata.page_count;
            state.SourceUrls.Clear();
            foreach (var u in ir.source.urls) state.SourceUrls.Add(u);
            return results;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[tedds-extract] snapshot load failed; will re-crawl: {ex.Message}");
            return null;
        }
    }

    sealed class SnapshotState
    {
        public int PageCount;
        public List<string> SourceUrls { get; } = new();
    }

    static void WriteSnapshot(string version, SnapshotState state, IReadOnlyList<PageParser.ParseResult> parsed, string outPath)
    {
        var snapshotBuilder = new IRBuilder("tedds", version, "scrape");
        foreach (var u in state.SourceUrls) snapshotBuilder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) snapshotBuilder.IncrementPageCount();
        foreach (var p in parsed) snapshotBuilder.AddType(p.Type);
        var ir = snapshotBuilder.Build();
        var outDir = Path.GetDirectoryName(outPath);
        if (!string.IsNullOrEmpty(outDir)) Directory.CreateDirectory(outDir);
        var json = JsonSerializer.Serialize(ir, IrJsonContext.Default.CoverageIR);
        var tmp = outPath + ".partial";
        File.WriteAllText(tmp, json);
        if (File.Exists(outPath)) File.Delete(outPath);
        File.Move(tmp, outPath);
    }

    // ── type-page crawl (sequential) ──────────────────────────────────────

    static async Task<List<PageParser.ParseResult>> CrawlTypePages(
        HttpScraper scraper, IReadOnlyList<string> typeUrls, SnapshotState state, ErrorsLog errorsLog)
    {
        var parsed = new List<PageParser.ParseResult>();
        var skippedUrls = new List<string>();
        var failedUrls = new List<(string url, string error)>();
        int parsedOk = 0;

        foreach (var url in typeUrls)
        {
            try
            {
                var html = await scraper.FetchHtml(url);
                state.PageCount++;
                var result = PageParser.Parse(html, url);
                if (result is null)
                {
                    skippedUrls.Add(url);
                }
                else
                {
                    parsed.Add(result);
                    parsedOk++;
                    if (parsedOk % 10 == 0)
                        Console.Error.WriteLine($"[tedds-extract] type-page progress {parsedOk}/{typeUrls.Count}");
                }
            }
            catch (Exception ex)
            {
                failedUrls.Add((url, ex.Message));
                errorsLog.Append("type-page-fetch", url, ex.Message);
            }
        }

        // Retry hard failures sequentially.
        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[tedds-extract] retry hard type-page failures: {failedUrls.Count}");
            var stillFailing = new List<(string url, string error)>();
            foreach (var (failedUrl, _) in failedUrls)
            {
                bool recovered = false;
                for (int attempt = 1; attempt <= 2; attempt++)
                {
                    try
                    {
                        await Task.Delay(500);
                        var html = await scraper.FetchHtml(failedUrl);
                        state.PageCount++;
                        var result = PageParser.Parse(html, failedUrl);
                        if (result is null) skippedUrls.Add(failedUrl);
                        else { parsed.Add(result); parsedOk++; }
                        recovered = true;
                        Console.Error.WriteLine($"[tedds-extract] RETRY ok ({attempt}/2): {failedUrl}");
                        break;
                    }
                    catch (Exception ex)
                    {
                        if (attempt == 2) stillFailing.Add((failedUrl, ex.Message));
                    }
                }
                if (!recovered) Console.Error.WriteLine($"[tedds-extract] RETRY still failing: {failedUrl}");
            }
            if (stillFailing.Count > 0)
            {
                foreach (var (u, e) in stillFailing) errorsLog.Append("type-page-retry-final", u, e);
            }
        }

        Console.Error.WriteLine($"[tedds-extract] type-page pass done: ok={parsedOk} skip={skippedUrls.Count} fail={failedUrls.Count} of {typeUrls.Count}");
        return parsed;
    }

    // ── per-member enrichment ─────────────────────────────────────────────

    static bool IsMethodPlaceholder(MethodInfo m, string typeName) =>
        m.@params.Count == 0
        && m.returns is null
        && m.throws.Count == 0
        && (m.signature == m.name + "(...)" || m.signature == typeName + "(...)");

    static bool IsPropertyPlaceholder(PropertyInfo p) =>
        p.type == "object" && p.getter && p.setter && p.remarks is null;

    static bool IsEventPlaceholder(EventInfo e) =>
        e.@delegate == "EventHandler"
        && e.fires_when is null
        && e.handler_thread == "unknown";

    static async Task<List<TypeInfo>> EnrichAllMembers(
        HttpScraper scraper, IReadOnlyList<PageParser.ParseResult> parsed, SnapshotState state, string version, string? outPath, ErrorsLog errorsLog)
    {
        var ctors = parsed.Select(p => p.Type.constructors.ToList()).ToList();
        var methods = parsed.Select(p => p.Type.methods.ToList()).ToList();
        var properties = parsed.Select(p => p.Type.properties.ToList()).ToList();
        var events = parsed.Select(p => p.Type.events.ToList()).ToList();

        var work = new List<MemberWorkItem>();
        int alreadyEnriched = 0;
        for (int i = 0; i < parsed.Count; i++)
        {
            var p = parsed[i];
            var typeName = p.Type.name;
            for (int m = 0; m < ctors[i].Count; m++)
            {
                var u = m < p.Links.Constructors.Count ? p.Links.Constructors[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsMethodPlaceholder(ctors[i][m], typeName))
                    work.Add(new(i, MemberKind.Constructor, m, u, ctors[i][m].name));
                else alreadyEnriched++;
            }
            for (int m = 0; m < methods[i].Count; m++)
            {
                var u = m < p.Links.Methods.Count ? p.Links.Methods[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsMethodPlaceholder(methods[i][m], typeName))
                    work.Add(new(i, MemberKind.Method, m, u, methods[i][m].name));
                else alreadyEnriched++;
            }
            for (int m = 0; m < properties[i].Count; m++)
            {
                var u = m < p.Links.Properties.Count ? p.Links.Properties[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsPropertyPlaceholder(properties[i][m]))
                    work.Add(new(i, MemberKind.Property, m, u, properties[i][m].name));
                else alreadyEnriched++;
            }
            for (int m = 0; m < events[i].Count; m++)
            {
                var u = m < p.Links.Events.Count ? p.Links.Events[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsEventPlaceholder(events[i][m]))
                    work.Add(new(i, MemberKind.Event, m, u, events[i][m].name));
                else alreadyEnriched++;
            }
        }

        Console.Error.WriteLine($"[tedds-extract] enrich queue size: {work.Count} (already-enriched skipped: {alreadyEnriched})");

        var errors = new List<(string typeName, string memberUrl, string error)>();
        int doneCount = 0;
        int fetchCount = 0;
        int parseSkipCount = 0;
        int totalItems = work.Count;

        void Snapshot()
        {
            if (outPath is null) return;
            var snapshot = new List<PageParser.ParseResult>(parsed.Count);
            for (int i = 0; i < parsed.Count; i++)
            {
                var t = parsed[i].Type with
                {
                    constructors = ctors[i].ToList(),
                    methods = methods[i].ToList(),
                    properties = properties[i].ToList(),
                    events = events[i].ToList()
                };
                snapshot.Add(new PageParser.ParseResult(t, parsed[i].Links));
            }
            WriteSnapshot(version, state, snapshot, outPath);
        }

        // Sequential — single HttpScraper, single per-member fetch, no concurrency.
        // TSD surface is small enough that 8x serial throughput (8 fetches/sec) finishes
        // in 30-60 seconds end-to-end.
        foreach (var item in work)
        {
            string html;
            try
            {
                html = await scraper.FetchHtml(item.Url);
                fetchCount++;
            }
            catch (Exception ex)
            {
                var typeName = parsed[item.TypeIdx].Type.@namespace + "." + parsed[item.TypeIdx].Type.name;
                errors.Add((typeName, item.Url, ex.Message));
                errorsLog.Append("member-page-fetch", item.Url, ex.Message);
                doneCount++;
                if (doneCount % SnapshotEvery == 0)
                {
                    Console.Error.WriteLine($"[tedds-extract] enrich progress {doneCount}/{totalItems} fetched={fetchCount} errors={errors.Count}");
                    Snapshot();
                }
                continue;
            }

            try
            {
                switch (item.Kind)
                {
                    case MemberKind.Constructor:
                    {
                        var e = MemberPageParser.ParseMethod(html, isCtor: true, memberRowText: item.MemberName);
                        if (e is null) { parseSkipCount++; break; }
                        ctors[item.TypeIdx][item.MemberIdx] = ctors[item.TypeIdx][item.MemberIdx] with
                        {
                            signature = string.IsNullOrEmpty(e.signature) ? ctors[item.TypeIdx][item.MemberIdx].signature : e.signature,
                            @params = e.@params,
                            returns = e.returns,
                            throws = e.throws,
                            remarks = e.remarks
                        };
                        break;
                    }
                    case MemberKind.Method:
                    {
                        var e = MemberPageParser.ParseMethod(html, isCtor: false, memberRowText: item.MemberName);
                        if (e is null) { parseSkipCount++; break; }
                        methods[item.TypeIdx][item.MemberIdx] = methods[item.TypeIdx][item.MemberIdx] with
                        {
                            signature = string.IsNullOrEmpty(e.signature) ? methods[item.TypeIdx][item.MemberIdx].signature : e.signature,
                            @params = e.@params,
                            returns = e.returns,
                            throws = e.throws,
                            remarks = e.remarks
                        };
                        break;
                    }
                    case MemberKind.Property:
                    {
                        var e = MemberPageParser.ParseProperty(html);
                        if (e is null) { parseSkipCount++; break; }
                        properties[item.TypeIdx][item.MemberIdx] = properties[item.TypeIdx][item.MemberIdx] with
                        {
                            type = e.type,
                            getter = e.getter,
                            setter = e.setter,
                            remarks = e.remarks
                        };
                        break;
                    }
                    case MemberKind.Event:
                    {
                        var e = MemberPageParser.ParseEvent(html);
                        if (e is null) { parseSkipCount++; break; }
                        events[item.TypeIdx][item.MemberIdx] = events[item.TypeIdx][item.MemberIdx] with
                        {
                            @delegate = e.@delegate,
                            signature = string.IsNullOrEmpty(e.signature) ? events[item.TypeIdx][item.MemberIdx].signature : e.signature
                        };
                        break;
                    }
                }
            }
            catch (Exception ex)
            {
                var typeName = parsed[item.TypeIdx].Type.@namespace + "." + parsed[item.TypeIdx].Type.name;
                errors.Add((typeName, item.Url, "parse: " + ex.Message));
                errorsLog.Append("member-page-parse", item.Url, ex.Message);
            }

            doneCount++;
            if (doneCount % SnapshotEvery == 0)
            {
                Console.Error.WriteLine($"[tedds-extract] enrich progress {doneCount}/{totalItems} fetched={fetchCount} errors={errors.Count}");
                Snapshot();
            }
        }

        Snapshot();

        Console.Error.WriteLine(
            $"[tedds-extract] enrich pass done: total={totalItems} fetched={fetchCount} parse-skipped={parseSkipCount} errors={errors.Count}");

        if (errors.Count > 0)
        {
            Console.Error.WriteLine($"[tedds-extract] member-page errors ({errors.Count}):");
            foreach (var (t, u, e) in errors.Take(40))
                Console.Error.WriteLine($"  MEMBER-ERR {t} {u}: {e}");
            if (errors.Count > 40)
                Console.Error.WriteLine($"  ... +{errors.Count - 40} more");
        }

        var output = new List<TypeInfo>(parsed.Count);
        for (int i = 0; i < parsed.Count; i++)
        {
            var t = parsed[i].Type;
            output.Add(t with
            {
                constructors = ctors[i],
                methods = methods[i],
                properties = properties[i],
                events = events[i]
            });
        }
        return output;
    }

    // ── namespace URL discovery ─────────────────────────────────────────────

    /// <summary>
    /// Parse the sidebar tree for namespace-leaf entries. Tekla Tedds nav structure:
    ///   <ul><li leaf-id="<rootId>"><a href="/doc/tekla-tedds/<ver>/<rootId>">API Namespaces</a>
    ///     <ul>
    ///       <li leaf-id="..."><a href="/doc/tekla-tedds/<ver>/tekla-structural-...">Tekla.Structural.X</a>
    ///       ...
    ///     </ul>
    ///   </li></ul>
    /// We collect every direct-child <a> of the "API Namespaces" <ul> that has the
    /// `/doc/tekla-tedds/<ver>/...` pattern AND ends in `-<digits>` (canonical type-page URL).
    /// </summary>
    internal static List<string> ExtractNamespaceUrls(string html, string verYear)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        var tree = doc.QuerySelector("#block-trimble2017-devcenter-apipackagenavigation");
        if (tree is null)
            throw new InvalidDataException("Tedds sidebar tree not found — page layout changed?");

        // The root <li> under tree contains "API Namespaces" and has children that ARE namespace leaves.
        // We find every <a> with href matching `/doc/tekla-tedds/<verYear>/<slug>-<id>` where the slug
        // contains at least one dash-separated path beyond "tekla-structural"/"tekla-structural-interopassemblies"
        // BUT excluding deeper type leaves. The cleanest filter: a namespace href's text is "Tekla.X[.Y.Z]"
        // (contains dots, no parens, starts with "Tekla.").
        var urls = new List<string>();
        var pattern = $"/doc/tekla-tedds/{verYear}/";
        foreach (var a in tree.QuerySelectorAll("a"))
        {
            var href = a.GetAttribute("href") ?? "";
            if (!href.StartsWith(pattern, StringComparison.Ordinal)) continue;
            var text = a.TextContent.Trim();
            // Namespace anchors carry dotted names like "Tekla.Structural.ExpressoAddIn".
            // Type anchors carry bare names like "AliasAttribute" or "TdsCalcStatus".
            // The root anchor is "API Namespaces" — skip it.
            if (text == "API Namespaces") continue;
            if (!text.Contains('.', StringComparison.Ordinal)) continue;
            if (text.Contains('(', StringComparison.Ordinal)) continue;  // method overload UIs may render with parens.
            urls.Add(AbsoluteUrl(href));
        }
        return urls.Distinct().ToList();
    }

    // ── type URL extraction ────────────────────────────────────────────────

    /// <summary>
    /// Collect every type URL from a namespace page. The sidebar tree (when on a namespace
    /// page) is expanded to show every type in that namespace as a leaf <li><a href=...>.
    /// Type URLs have the shape `/doc/tekla-tedds/<ver>/<slug>-<id>` where slug encodes the
    /// namespace + type name. We filter to: hrefs whose anchor text is a bare identifier
    /// (no dots, no parens, starts with a capital letter or underscore — `_IApplicationEvents`
    /// is a real type name) AND whose URL is NOT the namespace URL itself.
    /// </summary>
    internal static List<string> ExtractTypeUrls(string html, string verYear, string namespaceUrl)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        var tree = doc.QuerySelector("#block-trimble2017-devcenter-apipackagenavigation");
        if (tree is null) return new List<string>();

        var urls = new List<string>();
        var pattern = $"/doc/tekla-tedds/{verYear}/";
        foreach (var a in tree.QuerySelectorAll("a"))
        {
            var href = a.GetAttribute("href") ?? "";
            if (!href.StartsWith(pattern, StringComparison.Ordinal)) continue;
            var absHref = AbsoluteUrl(href);
            if (absHref == namespaceUrl) continue;
            var text = a.TextContent.Trim();
            if (string.IsNullOrEmpty(text)) continue;
            if (text == "API Namespaces") continue;
            // Namespace anchors carry dotted names — skip them (they should already be in our
            // namespace list, and they'd recurse).
            if (text.Contains('.', StringComparison.Ordinal)) continue;
            // Anchor text shouldn't have parens (those would be member overload spec — but
            // members live deeper in the sidebar tree and we want them via member tables on
            // each type page, not via root-tree walk).
            if (text.Contains('(', StringComparison.Ordinal)) continue;
            // Only keep URLs whose final slug ends with `-<digits>` (canonical type-page form).
            // This excludes any future structural changes to the URL scheme.
            if (!System.Text.RegularExpressions.Regex.IsMatch(absHref, @"-\d+$")) continue;
            // The URL must NOT contain "-ctor-" (those are constructor overload pages, which
            // we discover via type-page member tables, not the nav tree).
            if (absHref.Contains("-ctor-", StringComparison.Ordinal)) continue;

            // Filter: type-page URLs have a slug structure like
            //   /doc/tekla-tedds/<ver>/<ns-slug>-<typename>-<id>
            // Member-page URLs have a deeper slug structure
            //   /doc/tekla-tedds/<ver>/<ns-slug>-<typename>-<membername>-<id>
            // We can't disambiguate from URL alone reliably (both are dash-separated kebab),
            // but the nav tree on the namespace page only shows TYPES (members are nested
            // under their parent type's <li> only when that type is expanded). Since the
            // namespace page renders all types collapsed, no member leaves appear at this
            // depth — anchor text "TdsCalcStatus" etc. are TYPES.
            urls.Add(absHref);
        }
        return urls.Distinct().ToList();
    }

    static string AbsoluteUrl(string href)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        if (href.StartsWith("/", StringComparison.Ordinal)) return BaseHost + href;
        return href;
    }
}
