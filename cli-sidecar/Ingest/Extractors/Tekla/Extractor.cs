// Tekla extractor — drives the crawl of developer.tekla.com to produce a host-coverage
// IR for Tekla Structures (versions 2025 and 2026).
//
// Pipeline:
//
//   Phase 1 — namespace discovery (sequential):
//     Fetch the version root page, parse the sidebar tree to list all
//     Tekla.Structures.* namespaces.
//
//   Phase 2 — type URL harvest (sequential per-namespace):
//     For each namespace page, walk the type-tables (classList / interfaceList / ...)
//     to collect every type URL.
//
//   Phase 3 — type-page crawl (4-way concurrent):
//     Fetch each type page and run PageParser.Parse() to produce a (TypeInfo,
//     MemberLinks) pair. After phase 3 we write a snapshot of the IR to outPath —
//     types are fully populated at type level but every member has placeholder
//     signature / params / returns / etc.
//
//   Phase 4 — per-member enrichment (4-way concurrent):
//     Flatten member URLs across all types into a single work list. For each
//     member, fetch the per-member page and run MemberPageParser to extract real
//     signature, parameter types + docs, return type + doc, exceptions, remarks,
//     property type + getter/setter, event delegate. Write a fresh IR snapshot
//     every SnapshotEvery enrichments so the work is RESUMABLE if the process dies
//     under CDN throttling.
//
//   Phase 5 — final IR emit:
//     Stamp metadata + extracted_at and write the final IR.
//
// Resumability:
// -------------
// At Extract() entry, if `outPath` exists AND deserialises as a CoverageIR for this
// host_version, we read it and seed the enrich pass with the already-populated types.
// Members whose `signature == name + "(...)"` and `params.Count == 0` and `returns == null`
// (the type-page placeholder shape) are re-enqueued for fetching; everything else is
// preserved verbatim. This lets the user kill -9 the sidecar at any time and re-run
// the same command to pick up where it left off.
//
// HttpClient discipline:
// ---------------------
// We do not modify the Phase A Common.HttpScraper. Instead we instantiate one
// HttpScraper per worker so each gets its own HttpClient + connection pool. The
// Tekla CDN throttles a single client's pool aggressively (anything past ~500
// fetches stalls); independent clients sidestep the per-pool limit and let each
// worker make steady progress.

using System.Text.Json;
using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Tekla;

public static class Extractor
{
    // The Tekla.Structures namespace root page is at a version-specific numeric ID.
    // These are stable per release (verified for 2025 + 2026 — see EXTRACTION-NOTES.md).
    static readonly Dictionary<string, (string version, int rootId)> KnownVersions = new()
    {
        ["2025.0"] = ("2025", 45473),
        ["2026.0"] = ("2026", 64304),
    };

    const string BaseHost = "https://developer.tekla.com";
    // Type-page concurrency = 4. Empirical: the Tekla CDN throttles aggressively after
    // ~3k fetches/IP. At the type-page stage we have a single long-lived HttpClient
    // (per-pool connection re-use), so we stay conservative: 4 in-flight requests keep
    // the pool warm without tripping the throttle threshold on the first ~1320 type pages.
    // For B2-B14, vendors should tune this per-CDN behavior — measure type-page
    // throughput vs throttle-time on a first run and document the chosen value in
    // EXTRACTION-NOTES.md under the "Concurrency tuning" section.
    const int TypePageConcurrency = 4;
    // Member-page concurrency = 8. Empirical: with fresh per-fetch HttpScraper instances
    // (each its own HttpClient + connection pool) the CDN sees uniform short-lived
    // connections. We measured 8 as the sweet spot — it stays below the throttle
    // threshold longer than 16 (which trips throttling at ~1k fetches) and is faster
    // than 4 (which under-utilises the per-IP request budget). 12 of 12417 member pages
    // on tekla-2026 completed in a single uninterrupted run at this setting.
    // For B2-B14, vendors should tune this per-CDN behavior — start at 8, halve on
    // throttle, double on consistent steady-state. Document the chosen value + reasoning
    // in EXTRACTION-NOTES.md under the "Concurrency tuning" section.
    const int MemberPageConcurrency = 8;
    // Snapshot every 200 enrichments. Writing 5-6 MB of JSON every 200 enrichments costs
    // ~50-100 ms and gives a generous resumability window: with 8-way concurrency we burn
    // through 200 members in 20-40 s, so a kill -9 at any point loses <40 s of work.
    // For B2-B14, vendors with larger IRs (>50 MB) may need a higher snapshot interval
    // to keep the write-amplification cost below 1-2% of total wall-clock time.
    const int SnapshotEvery = 200;

    enum MemberKind { Constructor, Method, Property, Event }

    sealed record MemberWorkItem(int TypeIdx, MemberKind Kind, int MemberIdx, string Url);

    /// <summary>
    /// Thread-safe append-only writer for the dedicated extraction-errors log
    /// (`tekla-&lt;version&gt;-errors.log`). Created on first write, never pre-created
    /// empty. Format: tab-separated UTC ISO-8601 timestamp + phase + url + message,
    /// one line per error.
    /// </summary>
    sealed class ErrorsLog
    {
        readonly string? _path;
        readonly object _lock = new();
        public ErrorsLog(string? path) { _path = path; }

        public void Append(string phase, string url, string message)
        {
            if (string.IsNullOrEmpty(_path)) return;
            // Strip CR/LF/TAB from the free-text fields so each record is a single line.
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

    /// <summary>
    /// Derive the errors-log path from the output IR path: replace `.ir.json` with
    /// `-errors.log`. Returns null when no outPath was provided.
    /// </summary>
    static string? DeriveErrorsLogPath(string? outPath)
    {
        if (string.IsNullOrEmpty(outPath)) return null;
        var dir = Path.GetDirectoryName(outPath) ?? "";
        var fileName = Path.GetFileName(outPath);
        // outPath is e.g. "tekla-2026.0.ir.json"; we want "tekla-2026.0-errors.log".
        if (fileName.EndsWith(".ir.json", StringComparison.Ordinal))
            fileName = fileName[..^".ir.json".Length] + "-errors.log";
        else
            fileName += "-errors.log";
        return Path.Combine(dir, fileName);
    }

    /// <summary>
    /// Backwards-compatible API for callers that don't pass an outPath. They get a fresh
    /// crawl + in-memory IR; no resumability.
    /// </summary>
    public static Task<CoverageIR> Extract(string version) =>
        Extract(version, outPath: null);

    /// <summary>
    /// Extract a complete host-coverage IR for Tekla Structures at the given version.
    /// If <paramref name="outPath"/> is non-null, periodic snapshots are written so the run
    /// is resumable across process restarts.
    /// </summary>
    public static async Task<CoverageIR> Extract(string version, string? outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var v))
            throw new ArgumentException(
                $"Tekla extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var rootUrl = $"{BaseHost}/doc/tekla-structures/{v.version}/tekla-structures-{v.rootId}";
        var state = new SnapshotState();
        state.SourceUrls.Add(rootUrl);

        // Dedicated extraction-errors log alongside outPath. Created lazily on first
        // append so a clean run leaves no on-disk file (matches the 0-errors-so-far
        // story documented in EXTRACTION-NOTES.md). Errors from prior resumed runs are
        // APPENDED to so the history of throttle events survives across restarts.
        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));

        // Try to resume from an existing snapshot.
        List<PageParser.ParseResult>? parsed = TryLoadSnapshot(version, outPath, rootUrl, state);

        if (parsed is null)
        {
            await using var typeScraper = await HttpScraper.Launch();
            Console.Error.WriteLine($"[tekla-extract] launched HTTP scraper; crawling {rootUrl}");

            var rootHtml = await typeScraper.FetchHtml(rootUrl);
            state.PageCount++;
            if (rootHtml.Length < 5000)
                throw new InvalidDataException(
                    $"Tekla root page suspiciously short ({rootHtml.Length} bytes; expected ~55KB). " +
                    $"Bot block or decompression regression? Head: {rootHtml.Substring(0, Math.Min(200, rootHtml.Length))}");
            var namespaceUrls = ExtractNamespaceUrls(rootHtml);
            Console.Error.WriteLine($"[tekla-extract] discovered {namespaceUrls.Count} namespaces");

            var typeUrls = new List<string>();
            foreach (var nsUrl in namespaceUrls)
            {
                try
                {
                    var nsHtml = await typeScraper.FetchHtml(nsUrl);
                    state.PageCount++;
                    var nsTypes = ExtractTypeUrls(nsHtml);
                    Console.Error.WriteLine($"[tekla-extract]   {nsUrl}: {nsTypes.Count} types");
                    typeUrls.AddRange(nsTypes);
                }
                catch (Exception ex)
                {
                    Console.Error.WriteLine($"[tekla-extract] WARN failed namespace {nsUrl}: {ex.Message}");
                    errorsLog.Append("namespace-fetch", nsUrl, ex.Message);
                }
            }

            typeUrls = typeUrls.Distinct().ToList();
            Console.Error.WriteLine($"[tekla-extract] total unique type URLs: {typeUrls.Count}");

            parsed = await CrawlTypePages(typeScraper, typeUrls, state, errorsLog);

            // Snapshot after type-page pass so the enrich loop is fully resumable.
            if (outPath is not null) WriteSnapshot(version, state, parsed, outPath);
        }
        else
        {
            Console.Error.WriteLine($"[tekla-extract] resumed from snapshot: {parsed.Count} types already on disk");
        }

        // Phase 4 — enrich members.
        var enriched = await EnrichAllMembers(parsed, state, version, outPath, errorsLog);

        // Final IRBuilder assembly.
        var builder = new IRBuilder("tekla", version, "scrape");
        foreach (var u in state.SourceUrls) builder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) builder.IncrementPageCount();
        foreach (var t in enriched) builder.AddType(t);

        Console.Error.WriteLine(
            $"[tekla-extract] done: types={enriched.Count}");

        return builder.Build();
    }

    // ── snapshot read/write ───────────────────────────────────────────────

    /// <summary>
    /// Snapshot shape: a regular CoverageIR. We re-derive the member URLs from each
    /// member's <c>doc_url</c> so we don't need a separate side-table on disk.
    /// Page count + source URL list from the snapshot are reflected back into <paramref name="state"/>.
    /// </summary>
    static List<PageParser.ParseResult>? TryLoadSnapshot(string version, string? outPath, string rootUrl, SnapshotState state)
    {
        if (string.IsNullOrEmpty(outPath) || !File.Exists(outPath)) return null;
        try
        {
            var json = File.ReadAllText(outPath);
            var ir = JsonSerializer.Deserialize(json, IrJsonContext.Default.CoverageIR);
            if (ir is null || ir.host != "tekla" || ir.host_version != version) return null;

            // Re-create PageParser.ParseResult bundles from the persisted TypeInfo. Each member
            // already carries a doc_url which we use as the per-member URL — that mirrors what
            // PageParser would have stashed in MemberLinks. Members lacking a doc_url get an
            // empty string (those will be skipped in the enrich pass).
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
            Console.Error.WriteLine($"[tekla-extract] snapshot load failed; will re-crawl: {ex.Message}");
            return null;
        }
    }

    /// <summary>
    /// Mutable state shared between the extractor's outer flow and the snapshot reader/writer.
    /// IRBuilder doesn't expose its internals, so we mirror the bits we need.
    /// </summary>
    sealed class SnapshotState
    {
        public int PageCount;
        public List<string> SourceUrls { get; } = new();
    }

    /// <summary>
    /// Write a partial IR to <paramref name="outPath"/> reflecting the current state of the
    /// type list. Called after the type-page pass and during the enrich pass.
    /// </summary>
    static void WriteSnapshot(string version, SnapshotState state, IReadOnlyList<PageParser.ParseResult> parsed, string outPath)
    {
        var snapshotBuilder = new IRBuilder("tekla", version, "scrape");
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

    // ── type-page crawl (Parallel.ForEachAsync) ────────────────────────────

    static async Task<List<PageParser.ParseResult>> CrawlTypePages(
        HttpScraper scraper, IReadOnlyList<string> typeUrls, SnapshotState state, ErrorsLog errorsLog)
    {
        var parsed = new List<PageParser.ParseResult>();
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
                var result = PageParser.Parse(html, url);
                lock (addLock)
                {
                    state.PageCount++;
                    if (result is null) { skippedUrls.Add(url); }
                    else
                    {
                        parsed.Add(result);
                        parsedOk++;
                        if (parsedOk % 100 == 0)
                            Console.Error.WriteLine($"[tekla-extract] type-page progress {parsedOk}/{typeUrls.Count}");
                    }
                }
            }
            catch (Exception ex)
            {
                lock (addLock) failedUrls.Add((url, ex.Message));
                errorsLog.Append("type-page-fetch", url, ex.Message);
            }
        });

        // Sequential retry of hard failures.
        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[tekla-extract] retry hard type-page failures: {failedUrls.Count}");
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
                        var result = PageParser.Parse(html, failedUrl);
                        state.PageCount++;
                        if (result is null) skippedUrls.Add(failedUrl);
                        else { parsed.Add(result); parsedOk++; }
                        recovered = true;
                        Console.Error.WriteLine($"[tekla-extract] RETRY ok ({attempt}/2): {failedUrl}");
                        break;
                    }
                    catch (Exception ex)
                    {
                        if (attempt == 2) stillFailing.Add((failedUrl, ex.Message));
                    }
                }
                if (!recovered) Console.Error.WriteLine($"[tekla-extract] RETRY still failing: {failedUrl}");
            }
            if (stillFailing.Count > 0)
            {
                Console.Error.WriteLine($"[tekla-extract] hard type-page failures after retry ({stillFailing.Count}):");
                foreach (var (u, e) in stillFailing.Take(20)) Console.Error.WriteLine($"  FAIL {u}: {e}");
                if (stillFailing.Count > 20) Console.Error.WriteLine($"  ... +{stillFailing.Count - 20} more");
                foreach (var (u, e) in stillFailing) errorsLog.Append("type-page-retry-final", u, e);
            }
        }

        if (skippedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[tekla-extract] skipped non-type pages ({skippedUrls.Count}):");
            foreach (var u in skippedUrls.Take(20)) Console.Error.WriteLine($"  SKIP {u}");
            if (skippedUrls.Count > 20) Console.Error.WriteLine($"  ... +{skippedUrls.Count - 20} more");
            // Note: skipped-non-type pages are intentional (e.g. method-overload landing pages with
            // an <h1> ending in "Methods"); they aren't logged as errors. They're filtered before
            // reaching this point — only counted here for the summary line.
        }

        Console.Error.WriteLine($"[tekla-extract] type-page pass done: ok={parsedOk} skip={skippedUrls.Count} fail={failedUrls.Count} of {typeUrls.Count}");
        return parsed;
    }

    // ── per-member enrichment with disk snapshots ─────────────────────────

    /// <summary>
    /// Determine whether a method/constructor record is the type-page placeholder shape
    /// (signature is "name(...)" and params/returns are empty). Used to skip already-enriched
    /// members on a resumed run.
    /// </summary>
    static bool IsMethodPlaceholder(MethodInfo m, string typeName) =>
        m.@params.Count == 0
        && m.returns is null
        && m.throws.Count == 0
        && (m.signature == m.name + "(...)" || m.signature == typeName + "(...)");

    /// <summary>
    /// Property placeholder shape.
    /// </summary>
    static bool IsPropertyPlaceholder(PropertyInfo p) =>
        p.type == "object" && p.getter && p.setter && p.remarks is null;

    /// <summary>
    /// Event placeholder shape. Structural check (not signature string equality) so it stays
    /// robust if the type-page placeholder constructor in PageParser changes its signature
    /// template — the three field combo (delegate=EventHandler, fires_when=null,
    /// handler_thread="unknown") only appears together in the placeholder writer at
    /// PageParser.cs:ExtractEventTable. Real enriched events overwrite delegate to the real
    /// type name and never leave handler_thread as "unknown".
    /// </summary>
    static bool IsEventPlaceholder(EventInfo e) =>
        e.@delegate == "EventHandler"
        && e.fires_when is null
        && e.handler_thread == "unknown";

    static async Task<List<TypeInfo>> EnrichAllMembers(
        IReadOnlyList<PageParser.ParseResult> parsed, SnapshotState state, string version, string? outPath, ErrorsLog errorsLog)
    {
        // Mutable per-type buffers we overwrite as members are enriched.
        var ctors = parsed.Select(p => p.Type.constructors.ToList()).ToList();
        var methods = parsed.Select(p => p.Type.methods.ToList()).ToList();
        var properties = parsed.Select(p => p.Type.properties.ToList()).ToList();
        var events = parsed.Select(p => p.Type.events.ToList()).ToList();

        // Flatten ONLY the unenriched members into the work list. On a fresh run that's every
        // member; on a resumed run we skip members whose IR fields are already populated.
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

        Console.Error.WriteLine($"[tekla-extract] enrich queue size: {work.Count} (already-enriched skipped: {alreadyEnriched})");

        var errors = new List<(string typeName, string memberUrl, string error)>();
        var errorLock = new object();
        var slotLock = new object();
        int doneCount = 0;
        int fetchCount = 0;
        int parseSkipCount = 0;
        int totalItems = work.Count;

        // Per-fetch HttpScraper instances — each work item instantiates a fresh scraper,
        // does its fetch, and disposes immediately. This eliminates the connection-pool
        // starvation pattern observed with shared/per-worker pooled scrapers: under that
        // pattern Tekla's CDN progressively throttled long-lived connections to ~1/sec each
        // after the first 1000-2000 requests. Per-fetch scrapers see short-lived TCP
        // connections that the CDN handles uniformly across the IP. The cost is one extra
        // TCP+TLS handshake per request (~50-100ms) but throughput at 8-way concurrency is
        // higher than the throttled pooled-scraper alternative.
        try
        {
            // Local writer for snapshot-on-progress. The Snapshot lambda holds a snapshot of the
            // current parsed list re-stitched with the latest member buffers.
            void Snapshot()
            {
                if (outPath is null) return;
                lock (slotLock)
                {
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
                            $"[tekla-extract] enrich progress {d}/{totalItems} fetched={fetchCount} errors={errors.Count}");
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
                                parseFn: h => MemberPageParser.ParseMethod(h, isCtor: true),
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
                                parseFn: h => MemberPageParser.ParseMethod(h, isCtor: false),
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
                                parseFn: MemberPageParser.ParseProperty,
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
                                parseFn: MemberPageParser.ParseEvent,
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
                        $"[tekla-extract] enrich progress {donePost}/{totalItems} fetched={fetchCount} errors={errors.Count}");
                    Snapshot();
                }
            });

            // Final snapshot.
            Snapshot();
        }
        finally
        {
            // No persistent scrapers to dispose; each FetchWithFreshScraper call disposes its own.
        }

        Console.Error.WriteLine(
            $"[tekla-extract] enrich pass done: total={totalItems} fetched={fetchCount} parse-skipped={parseSkipCount} errors={errors.Count}");

        if (errors.Count > 0)
        {
            Console.Error.WriteLine($"[tekla-extract] member-page errors ({errors.Count}):");
            foreach (var (t, u, e) in errors.Take(40))
                Console.Error.WriteLine($"  MEMBER-ERR {t} {u}: {e}");
            if (errors.Count > 40)
                Console.Error.WriteLine($"  ... +{errors.Count - 40} more");
        }

        // Re-stitch into the final type list.
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

    /// <summary>
    /// Apply a per-member enrichment to slot <paramref name="idx"/> of <paramref name="list"/>:
    /// run <paramref name="parseFn"/> on the HTML; if it returns null, invoke <paramref name="onSkip"/>;
    /// otherwise re-build the row via <paramref name="merge"/> under <paramref name="slotLock"/>.
    /// Pulled out of the per-member switch so the four arms (ctor/method/property/event) share the
    /// same lock + null-guard scaffolding and differ only in <paramref name="parseFn"/> +
    /// <paramref name="merge"/>.
    /// </summary>
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
    /// Fetch a single member page using a fresh HttpScraper (and therefore a fresh HttpClient
    /// and connection pool). One retry on transient failure with a 750ms backoff. The
    /// HttpScraper is disposed before the method returns so its TCP connection drops to
    /// TIME_WAIT instead of being parked in a pool — the CDN sees uniform short-lived
    /// connections, which avoids the progressive-throttling pattern of a long-lived pool.
    /// </summary>
    static async Task<string> FetchWithFreshScraper(string url)
    {
        Exception? lastException = null;
        for (int attempt = 1; attempt <= 2; attempt++)
        {
            await using var scraper = await HttpScraper.Launch();
            try
            {
                return await scraper.FetchHtml(url);
            }
            catch (Exception ex) when (attempt < 2)
            {
                lastException = ex;
                await Task.Delay(750);
            }
            catch (Exception ex)
            {
                // Second-iteration failure — surface with the underlying cause attached.
                throw new InvalidOperationException(
                    $"FetchWithFreshScraper exhausted retries for {url}", ex);
            }
        }
        // Defensive: the second iteration either returns or throws above. If we somehow get
        // here, surface the last seen exception as the cause rather than losing it.
        throw new InvalidOperationException(
            $"FetchWithFreshScraper exhausted retries for {url}", lastException);
    }

    // ── namespace URL discovery from the root page sidebar ──────────────────

    internal static List<string> ExtractNamespaceUrls(string html)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        var tree = doc.QuerySelector("#block-trimble2017-devcenter-apipackagenavigation");
        if (tree is null)
            throw new InvalidDataException("Tekla sidebar tree not found — page layout changed?");

        var urls = new List<string>();
        foreach (var a in tree.QuerySelectorAll("a"))
        {
            var text = a.TextContent.Trim();
            if (!text.EndsWith(" Namespace", StringComparison.Ordinal)) continue;
            var href = a.GetAttribute("href") ?? "";
            if (string.IsNullOrEmpty(href)) continue;
            urls.Add(AbsoluteUrl(href));
        }
        return urls.Distinct().ToList();
    }

    // ── type URL extraction from a namespace landing page ──────────────────

    internal static List<string> ExtractTypeUrls(string html)
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
                urls.Add(AbsoluteUrl(href));
            }
        }
        return urls;
    }

    static string AbsoluteUrl(string href)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        if (href.StartsWith("/", StringComparison.Ordinal)) return BaseHost + href;
        return href;
    }
}
