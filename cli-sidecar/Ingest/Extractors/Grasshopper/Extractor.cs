// Grasshopper extractor — drives the crawl of the McNeel Grasshopper SDK docs github repo to
// produce a host-coverage IR for the Grasshopper SDK at versions 7 and 8.
//
// Source rationale (mirrors Rhino's; verified 2026-05-19):
//   - Versions 7 and 8 are maintained in dedicated branches of mcneel/grasshopper-api-docs:
//       7        → Grasshopper 1.0 for Rhino 7
//       gh-pages → Grasshopper for Rhino 8.31 (current 8.x)
//     The path layout differs from RhinoCommon's: Grasshopper's html dir lives at
//     `api/grasshopper/html/` (both branches), not `html/`.
//   - Identical Sandcastle layout to RhinoCommon — same `table#namespaceList`, same
//     `table.members#classList|...`, same h1/title shape on member pages. We delegate ALL parsing
//     to AwareSidecar.Ingest.Extractors.Rhino.PageParser + MemberPageParser. The only
//     Grasshopper-specific bits are:
//       1. URL bases (different repo, different path)
//       2. Root page filename — Grasshopper's root index uses a guid filename
//          (723c01da-9986-4db2-8f53-6f3a7494df75.htm) rather than R_Project_RhinoCommon.htm
//       3. The host name in the IR ("grasshopper" vs "rhino") and the errors-log filename.
//
// The orchestration pipeline (5 phases) is identical to Rhino. Snapshotting + resume + concurrency
// constants are inherited verbatim — the GitHub raw CDN does not throttle differently for the
// Grasshopper repo than for the RhinoCommon repo.

using System.Text.Json;
using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Common;
using RhinoPageParser = AwareSidecar.Ingest.Extractors.Rhino.PageParser;
using RhinoMemberPageParser = AwareSidecar.Ingest.Extractors.Rhino.MemberPageParser;

namespace AwareSidecar.Ingest.Extractors.Grasshopper;

public static class Extractor
{
    /// <summary>
    /// Per-version source-of-truth roots. Each entry holds (display version, raw github base URL
    /// for the html/ tree, root page filename). The root page in both versions is the same GUID-
    /// named file Sandcastle generates for the project root.
    /// </summary>
    static readonly Dictionary<string, (string version, string baseUrl, string rootPage)> KnownVersions = new()
    {
        ["7.0"] = ("7.0",
                   "https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/",
                   "723c01da-9986-4db2-8f53-6f3a7494df75.htm"),
        ["8.0"] = ("8.0",
                   "https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/",
                   "723c01da-9986-4db2-8f53-6f3a7494df75.htm"),
    };

    // See Rhino/Extractor.cs for the rationale — github raw rate-limits at 8-way concurrency.
    // Conservative 2/4 settings stay under the 429 trigger point.
    const int TypePageConcurrency = 2;
    const int MemberPageConcurrency = 4;
    const int SnapshotEvery = 200;

    enum MemberKind { Constructor, Method, Property, Event }

    sealed record MemberWorkItem(int TypeIdx, MemberKind Kind, int MemberIdx, string Url);

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
                $"Grasshopper extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var rootUrl = v.baseUrl + v.rootPage;
        var state = new SnapshotState();
        state.SourceUrls.Add(rootUrl);

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));

        List<RhinoPageParser.ParseResult>? parsed = TryLoadSnapshot(version, outPath, rootUrl, state);

        if (parsed is null)
        {
            await using var typeScraper = await HttpScraper.Launch();
            Console.Error.WriteLine($"[grasshopper-extract] launched HTTP scraper; crawling {rootUrl}");

            var rootHtml = await typeScraper.FetchHtml(rootUrl);
            state.PageCount++;
            if (rootHtml.Length < 3000)
                throw new InvalidDataException(
                    $"Grasshopper root page suspiciously short ({rootHtml.Length} bytes). " +
                    $"GitHub raw not serving? Head: {rootHtml.Substring(0, Math.Min(200, rootHtml.Length))}");
            var namespaceUrls = ExtractNamespaceUrls(rootHtml, v.baseUrl);
            Console.Error.WriteLine($"[grasshopper-extract] discovered {namespaceUrls.Count} namespaces");

            var typeUrls = new List<string>();
            foreach (var nsUrl in namespaceUrls)
            {
                try
                {
                    var nsHtml = await typeScraper.FetchHtml(nsUrl);
                    state.PageCount++;
                    var nsTypes = ExtractTypeUrls(nsHtml, v.baseUrl);
                    Console.Error.WriteLine($"[grasshopper-extract]   {nsUrl}: {nsTypes.Count} types");
                    typeUrls.AddRange(nsTypes);
                }
                catch (Exception ex)
                {
                    Console.Error.WriteLine($"[grasshopper-extract] WARN failed namespace {nsUrl}: {ex.Message}");
                    errorsLog.Append("namespace-fetch", nsUrl, ex.Message);
                }
            }

            typeUrls = typeUrls.Distinct().ToList();
            Console.Error.WriteLine($"[grasshopper-extract] total unique type URLs: {typeUrls.Count}");

            parsed = await CrawlTypePages(typeScraper, typeUrls, state, errorsLog, v.baseUrl);

            if (outPath is not null) WriteSnapshot(version, state, parsed, outPath);
        }
        else
        {
            Console.Error.WriteLine($"[grasshopper-extract] resumed from snapshot: {parsed.Count} types already on disk");
        }

        var enriched = await EnrichAllMembers(parsed, state, version, outPath, errorsLog);

        var builder = new IRBuilder("grasshopper", version, "scrape");
        foreach (var u in state.SourceUrls) builder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) builder.IncrementPageCount();
        foreach (var t in enriched) builder.AddType(t);

        Console.Error.WriteLine(
            $"[grasshopper-extract] done: types={enriched.Count}");

        return builder.Build();
    }

    // ── snapshot read/write ───────────────────────────────────────────────

    static List<RhinoPageParser.ParseResult>? TryLoadSnapshot(string version, string? outPath, string rootUrl, SnapshotState state)
    {
        if (string.IsNullOrEmpty(outPath) || !File.Exists(outPath)) return null;
        try
        {
            var json = File.ReadAllText(outPath);
            var ir = JsonSerializer.Deserialize(json, IrJsonContext.Default.CoverageIR);
            if (ir is null || ir.host != "grasshopper" || ir.host_version != version) return null;

            var results = new List<RhinoPageParser.ParseResult>(ir.types.Count);
            foreach (var t in ir.types)
            {
                var ctorUrls = t.constructors.Select(c => c.doc_url ?? "").ToList();
                var methodUrls = t.methods.Select(m => m.doc_url ?? "").ToList();
                var propUrls = t.properties.Select(p => p.doc_url ?? "").ToList();
                var eventUrls = t.events.Select(e => e.doc_url ?? "").ToList();
                var links = new RhinoPageParser.MemberLinks(ctorUrls, methodUrls, propUrls, eventUrls);
                results.Add(new RhinoPageParser.ParseResult(t, links));
            }
            state.PageCount = ir.metadata.page_count;
            state.SourceUrls.Clear();
            foreach (var u in ir.source.urls) state.SourceUrls.Add(u);
            return results;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[grasshopper-extract] snapshot load failed; will re-crawl: {ex.Message}");
            return null;
        }
    }

    sealed class SnapshotState
    {
        public int PageCount;
        public List<string> SourceUrls { get; } = new();
    }

    static void WriteSnapshot(string version, SnapshotState state, IReadOnlyList<RhinoPageParser.ParseResult> parsed, string outPath)
    {
        var snapshotBuilder = new IRBuilder("grasshopper", version, "scrape");
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

    // ── type-page crawl ───────────────────────────────────────────────────

    static async Task<List<RhinoPageParser.ParseResult>> CrawlTypePages(
        HttpScraper scraper, IReadOnlyList<string> typeUrls, SnapshotState state, ErrorsLog errorsLog, string baseUrl)
    {
        var parsed = new List<RhinoPageParser.ParseResult>();
        var skippedUrls = new List<string>();
        var failedUrls = new List<(string url, string error)>();
        var addLock = new object();
        int parsedOk = 0;

        var opts = new ParallelOptions { MaxDegreeOfParallelism = TypePageConcurrency };
        await Parallel.ForEachAsync(typeUrls, opts, async (url, ct) =>
        {
            try
            {
                var html = await scraper.FetchHtml(url);
                var result = RhinoPageParser.Parse(html, url, baseUrl);
                lock (addLock)
                {
                    state.PageCount++;
                    if (result is null) { skippedUrls.Add(url); }
                    else
                    {
                        parsed.Add(result);
                        parsedOk++;
                        if (parsedOk % 100 == 0)
                            Console.Error.WriteLine($"[grasshopper-extract] type-page progress {parsedOk}/{typeUrls.Count}");
                    }
                }
            }
            catch (Exception ex)
            {
                lock (addLock) failedUrls.Add((url, ex.Message));
                errorsLog.Append("type-page-fetch", url, ex.Message);
            }
        });

        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[grasshopper-extract] retry hard type-page failures: {failedUrls.Count}");
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
                        var result = RhinoPageParser.Parse(html, failedUrl, baseUrl);
                        state.PageCount++;
                        if (result is null) skippedUrls.Add(failedUrl);
                        else { parsed.Add(result); parsedOk++; }
                        recovered = true;
                        Console.Error.WriteLine($"[grasshopper-extract] RETRY ok ({attempt}/2): {failedUrl}");
                        break;
                    }
                    catch (Exception ex)
                    {
                        if (attempt == 2) stillFailing.Add((failedUrl, ex.Message));
                    }
                }
                if (!recovered) Console.Error.WriteLine($"[grasshopper-extract] RETRY still failing: {failedUrl}");
            }
            if (stillFailing.Count > 0)
            {
                Console.Error.WriteLine($"[grasshopper-extract] hard type-page failures after retry ({stillFailing.Count}):");
                foreach (var (u, e) in stillFailing.Take(20)) Console.Error.WriteLine($"  FAIL {u}: {e}");
                if (stillFailing.Count > 20) Console.Error.WriteLine($"  ... +{stillFailing.Count - 20} more");
                foreach (var (u, e) in stillFailing) errorsLog.Append("type-page-retry-final", u, e);
            }
        }

        if (skippedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[grasshopper-extract] skipped non-type pages ({skippedUrls.Count}):");
            foreach (var u in skippedUrls.Take(20)) Console.Error.WriteLine($"  SKIP {u}");
            if (skippedUrls.Count > 20) Console.Error.WriteLine($"  ... +{skippedUrls.Count - 20} more");
        }

        Console.Error.WriteLine($"[grasshopper-extract] type-page pass done: ok={parsedOk} skip={skippedUrls.Count} fail={failedUrls.Count} of {typeUrls.Count}");
        return parsed;
    }

    // ── per-member enrichment with disk snapshots ─────────────────────────

    static bool IsMethodPlaceholder(MethodInfo m, string typeName) =>
        m.@params.Count == 0
        && m.returns is null
        && m.throws.Count == 0
        && (m.signature == m.name + "(...)" || m.signature == typeName + "(...)");

    static bool IsPropertyPlaceholder(PropertyInfo p) =>
        p.type == "object" && p.getter && p.remarks is null;

    static bool IsEventPlaceholder(EventInfo e) =>
        e.@delegate == "EventHandler"
        && e.fires_when is null
        && e.handler_thread == "unknown";

    static async Task<List<TypeInfo>> EnrichAllMembers(
        IReadOnlyList<RhinoPageParser.ParseResult> parsed, SnapshotState state, string version, string? outPath, ErrorsLog errorsLog)
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
                if (IsMethodPlaceholder(ctors[i][m], typeName)) work.Add(new(i, MemberKind.Constructor, m, u));
                else alreadyEnriched++;
            }
            for (int m = 0; m < methods[i].Count; m++)
            {
                var u = m < p.Links.Methods.Count ? p.Links.Methods[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsMethodPlaceholder(methods[i][m], typeName)) work.Add(new(i, MemberKind.Method, m, u));
                else alreadyEnriched++;
            }
            for (int m = 0; m < properties[i].Count; m++)
            {
                var u = m < p.Links.Properties.Count ? p.Links.Properties[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsPropertyPlaceholder(properties[i][m])) work.Add(new(i, MemberKind.Property, m, u));
                else alreadyEnriched++;
            }
            for (int m = 0; m < events[i].Count; m++)
            {
                var u = m < p.Links.Events.Count ? p.Links.Events[m] : "";
                if (string.IsNullOrEmpty(u)) continue;
                if (IsEventPlaceholder(events[i][m])) work.Add(new(i, MemberKind.Event, m, u));
                else alreadyEnriched++;
            }
        }

        Console.Error.WriteLine($"[grasshopper-extract] enrich queue size: {work.Count} (already-enriched skipped: {alreadyEnriched})");

        var errors = new List<(string typeName, string memberUrl, string error)>();
        var errorLock = new object();
        var slotLock = new object();
        int doneCount = 0;
        int fetchCount = 0;
        int parseSkipCount = 0;
        int totalItems = work.Count;

        try
        {
            void Snapshot()
            {
                if (outPath is null) return;
                lock (slotLock)
                {
                    var snapshot = new List<RhinoPageParser.ParseResult>(parsed.Count);
                    for (int i = 0; i < parsed.Count; i++)
                    {
                        var t = parsed[i].Type with
                        {
                            constructors = ctors[i].ToList(),
                            methods = methods[i].ToList(),
                            properties = properties[i].ToList(),
                            events = events[i].ToList()
                        };
                        snapshot.Add(new RhinoPageParser.ParseResult(t, parsed[i].Links));
                    }
                    WriteSnapshot(version, state, snapshot, outPath);
                }
            }

            var opts = new ParallelOptions { MaxDegreeOfParallelism = MemberPageConcurrency };
            await Parallel.ForEachAsync(work, opts, async (item, ct) =>
            {
                string html;
                try
                {
                    html = await FetchWithFreshScraper(item.Url);
                    Interlocked.Increment(ref fetchCount);
                }
                catch (Exception ex)
                {
                    var typeName = parsed[item.TypeIdx].Type.@namespace + "." + parsed[item.TypeIdx].Type.name;
                    lock (errorLock) errors.Add((typeName, item.Url, ex.Message));
                    errorsLog.Append("member-page-fetch", item.Url, ex.Message);
                    var d = Interlocked.Increment(ref doneCount);
                    if (d % 200 == 0)
                    {
                        Console.Error.WriteLine(
                            $"[grasshopper-extract] enrich progress {d}/{totalItems} fetched={fetchCount} errors={errors.Count}");
                        Snapshot();
                    }
                    return;
                }

                try
                {
                    switch (item.Kind)
                    {
                        case MemberKind.Constructor:
                            EnrichSlot(
                                ctors[item.TypeIdx], item.MemberIdx, html, slotLock,
                                parseFn: h => RhinoMemberPageParser.ParseMethod(h, isCtor: true),
                                merge: (row, e) => row with
                                {
                                    signature = string.IsNullOrEmpty(e.signature) ? row.signature : e.signature,
                                    @params = e.@params,
                                    returns = e.returns,
                                    throws = e.throws,
                                    remarks = e.remarks
                                },
                                onSkip: () => Interlocked.Increment(ref parseSkipCount));
                            break;
                        case MemberKind.Method:
                            EnrichSlot(
                                methods[item.TypeIdx], item.MemberIdx, html, slotLock,
                                parseFn: h => RhinoMemberPageParser.ParseMethod(h, isCtor: false),
                                merge: (row, e) => row with
                                {
                                    signature = string.IsNullOrEmpty(e.signature) ? row.signature : e.signature,
                                    @params = e.@params,
                                    returns = e.returns,
                                    throws = e.throws,
                                    remarks = e.remarks
                                },
                                onSkip: () => Interlocked.Increment(ref parseSkipCount));
                            break;
                        case MemberKind.Property:
                            EnrichSlot(
                                properties[item.TypeIdx], item.MemberIdx, html, slotLock,
                                parseFn: RhinoMemberPageParser.ParseProperty,
                                merge: (row, e) => row with
                                {
                                    type = e.type,
                                    getter = e.getter,
                                    setter = e.setter,
                                    remarks = e.remarks
                                },
                                onSkip: () => Interlocked.Increment(ref parseSkipCount));
                            break;
                        case MemberKind.Event:
                            EnrichSlot(
                                events[item.TypeIdx], item.MemberIdx, html, slotLock,
                                parseFn: RhinoMemberPageParser.ParseEvent,
                                merge: (row, e) => row with
                                {
                                    @delegate = e.@delegate,
                                    signature = string.IsNullOrEmpty(e.signature) ? row.signature : e.signature
                                },
                                onSkip: () => Interlocked.Increment(ref parseSkipCount));
                            break;
                    }
                }
                catch (Exception ex)
                {
                    var typeName = parsed[item.TypeIdx].Type.@namespace + "." + parsed[item.TypeIdx].Type.name;
                    lock (errorLock) errors.Add((typeName, item.Url, "parse: " + ex.Message));
                    errorsLog.Append("member-page-parse", item.Url, ex.Message);
                }

                var donePost = Interlocked.Increment(ref doneCount);
                if (donePost % 200 == 0)
                {
                    Console.Error.WriteLine(
                        $"[grasshopper-extract] enrich progress {donePost}/{totalItems} fetched={fetchCount} errors={errors.Count}");
                    Snapshot();
                }
            });

            Snapshot();
        }
        finally
        {
        }

        Console.Error.WriteLine(
            $"[grasshopper-extract] enrich pass done: total={totalItems} fetched={fetchCount} parse-skipped={parseSkipCount} errors={errors.Count}");

        if (errors.Count > 0)
        {
            Console.Error.WriteLine($"[grasshopper-extract] member-page errors ({errors.Count}):");
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

    static void EnrichSlot<TRow, TEnrich>(
        IList<TRow> list, int idx, string html, object slotLock,
        Func<string, TEnrich?> parseFn,
        Func<TRow, TEnrich, TRow> merge,
        Action onSkip)
        where TEnrich : class
    {
        var enrichment = parseFn(html);
        if (enrichment is null) { onSkip(); return; }
        lock (slotLock)
        {
            list[idx] = merge(list[idx], enrichment);
        }
    }

    /// <summary>
    /// Fetch with exponential backoff to ride out github raw's 429 rate-limit responses.
    /// See Rhino/Extractor.cs for the rationale.
    /// </summary>
    static async Task<string> FetchWithFreshScraper(string url)
    {
        Exception? lastException = null;
        int[] backoffMs = [1500, 4000, 10000];
        for (int attempt = 1; attempt <= 3; attempt++)
        {
            await using var scraper = await HttpScraper.Launch();
            try
            {
                return await scraper.FetchHtml(url);
            }
            catch (Exception ex) when (attempt < 3)
            {
                lastException = ex;
                await Task.Delay(backoffMs[attempt - 1]);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException(
                    $"FetchWithFreshScraper exhausted retries for {url}", ex);
            }
        }
        throw new InvalidOperationException(
            $"FetchWithFreshScraper exhausted retries for {url}", lastException);
    }

    // ── namespace URL discovery from the Grasshopper root index page ──────

    /// <summary>
    /// Parse the Grasshopper SDK root page (723c01da-...htm) for namespace URLs.
    /// Unlike RhinoCommon — which lists namespaces in the main body's
    /// <c>table.members#namespaceList</c> — the Grasshopper root only exposes namespaces via the
    /// LEFT-SIDEBAR TOC (`div#tocNav`). The main body is a brief intro that doesn't enumerate
    /// namespaces at all. We extract every anchor whose href targets an <c>N_...htm</c> page
    /// from the TOC nav. This is verified empirically against branches 7 and gh-pages
    /// (Grasshopper for Rhino 7 and Rhino 8.31 — 2026-05-19).
    /// </summary>
    internal static List<string> ExtractNamespaceUrls(string html, string baseUrl)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        var tocNav = doc.QuerySelector("div#tocNav");
        if (tocNav is null)
            throw new InvalidDataException(
                "Grasshopper root: div#tocNav not found — Sandcastle TOC layout changed?");

        var urls = new List<string>();
        foreach (var a in tocNav.QuerySelectorAll("a"))
        {
            var href = a.GetAttribute("href") ?? "";
            if (string.IsNullOrEmpty(href)) continue;
            // We only want namespace anchors — those whose filename starts with `N_`.
            // The TOC also carries the root anchor and "Welcome" / "Development" / "Examples"
            // section anchors (non-namespace pages); skip anything that isn't N_.
            var fileName = Path.GetFileName(href);
            if (!fileName.StartsWith("N_", StringComparison.Ordinal)) continue;
            urls.Add(AbsoluteUrl(href, baseUrl));
        }
        return urls.Distinct().ToList();
    }

    internal static List<string> ExtractTypeUrls(string html, string baseUrl)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        var urls = new List<string>();
        string[] tableIds = ["classList", "interfaceList", "structureList", "enumerationList", "delegateList"];
        foreach (var id in tableIds)
        {
            var table = doc.QuerySelector($"table.members#{id}");
            if (table is null) continue;
            foreach (var tr in table.QuerySelectorAll("tr"))
            {
                if (tr.QuerySelector("th") != null) continue;
                var a = tr.QuerySelectorAll("td").Skip(1).FirstOrDefault()?.QuerySelector("a");
                if (a is null) continue;
                var href = a.GetAttribute("href") ?? "";
                if (string.IsNullOrEmpty(href)) continue;
                urls.Add(AbsoluteUrl(href, baseUrl));
            }
        }
        return urls;
    }

    static string AbsoluteUrl(string href, string baseUrl)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        // Grasshopper hrefs are typically "../html/N_XX.htm" (relative from the root). The baseUrl
        // points to "...html/", so we need to resolve "../html/..." paths cleanly. Use URI math
        // for correctness — Grasshopper Sandcastle emits relative hrefs that walk up one level.
        try
        {
            var uri = new Uri(new Uri(baseUrl), href);
            return uri.ToString();
        }
        catch
        {
            // Fallback to naive concatenation if Uri math fails.
            if (baseUrl.EndsWith("/", StringComparison.Ordinal)) return baseUrl + href;
            var slash = baseUrl.LastIndexOf('/');
            return slash < 0 ? href : baseUrl.Substring(0, slash + 1) + href;
        }
    }
}
