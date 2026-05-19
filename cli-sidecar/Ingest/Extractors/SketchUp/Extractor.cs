// SketchUp extractor — drives the crawl of the SketchUp Ruby API documentation
// (https://ruby.sketchup.com/) to produce a host-coverage IR for SketchUp at versions 2025
// and 2026.
//
// Source rationale:
//   • Trimble's SketchUp Extensibility Team publishes the Ruby API docs at
//     https://ruby.sketchup.com/ — a unified YARD-generated site that covers the LATEST
//     shipping SketchUp version (currently 2026.1 as of 2026-05-19). There is NO version
//     selector and there are NO separate doc trees for older versions; every class/module
//     listed is current-state, with members tagged via a `<ul class="version">` block when
//     they were introduced in a specific release.
//   • Per the v0.30 Phase B brief we produce TWO IRs (sketchup-2025.0 + sketchup-2026.0) from
//     this single unified source. The `host_version` is stamped accordingly; the content is
//     identical between the two IRs because the docs are unified. See EXTRACTION-NOTES.md.
//   • The C SDK (SketchUp file-format reading/writing outside SketchUp) is intentionally NOT
//     included in this agent — different surface, different license (EULA + login gate, see
//     project memory `project_sketchup_csdk_skipped`).
//
// Pipeline (5 phases — same shape as the Sandcastle extractors, but Phase 4 is a no-op
// because YARD inlines every member's detail block onto the parent type page):
//
//   Phase 1 — root index fetch + type-URL discovery (sequential):
//     Fetch _index.html, parse <a href="...">...</a> within the alphabetic listing to get
//     every class/module URL on the site.
//
//   Phase 2 — N/A:
//     There are no per-namespace landing pages in the YARD layout that we need to crawl
//     separately; _index.html already lists every type by name.
//
//   Phase 3 — type-page crawl (4-way concurrent):
//     Fetch each type page and run PageParser.Parse() to produce a complete TypeInfo
//     (including all members, constants, and inheritance metadata). Snapshot every 50 types.
//
//   Phase 4 — N/A:
//     YARD inlines every method/property/constant detail on the same page as the type
//     header, so there's no second pass to enrich members. The TypeInfo emitted by Phase 3
//     is already fully populated.
//
//   Phase 5 — final IR emit.
//
// Resumability: same shape as Rhino/Tekla — load `outPath` if it exists for this host_version
// and skip already-crawled type URLs.
//
// Concurrency: ruby.sketchup.com is hosted on Trimble's standard CDN. Empirically tolerates
// 4-way concurrent fetches without throttling for the ~155-type corpus. We use a single
// HttpScraper (not a fresh-per-fetch one — there's no need for the Sandcastle-style
// per-fetch reset since the corpus is small).

using System.Text.Json;
using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.SketchUp;

public static class Extractor
{
    /// <summary>
    /// Per-version source-of-truth roots. SketchUp's docs are unified — both 2025 and 2026
    /// resolve to the same base URL. The version field exists only so the IR's `host_version`
    /// is stamped correctly and the snapshot/resume logic can tell two IR files apart.
    /// </summary>
    static readonly Dictionary<string, (string version, string baseUrl, string rootPage)> KnownVersions = new()
    {
        ["2025.0"] = ("2025.0", "https://ruby.sketchup.com/", "_index.html"),
        ["2026.0"] = ("2026.0", "https://ruby.sketchup.com/", "_index.html"),
    };

    // Trimble's CDN serves the SketchUp Ruby API docs from a small corpus (~155 types). 4-way
    // concurrency completes the full extraction in ~30 seconds without throttling on a fresh
    // run. Lower than Rhino's 4-way (we don't share a single long-lived HttpClient — we go
    // with one shared scraper here because the corpus is small enough for HttpClient pool
    // reuse to be a non-issue).
    const int TypePageConcurrency = 4;
    const int SnapshotEvery = 50;

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
                $"SketchUp extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var rootUrl = v.baseUrl + v.rootPage;
        var state = new SnapshotState();
        state.SourceUrls.Add(rootUrl);

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));

        List<PageParser.ParseResult>? parsed = TryLoadSnapshot(version, outPath, rootUrl, state);

        if (parsed is null)
        {
            await using var scraper = await HttpScraper.Launch();
            Console.Error.WriteLine($"[sketchup-extract] launched HTTP scraper; crawling {rootUrl}");

            var rootHtml = await scraper.FetchHtml(rootUrl);
            state.PageCount++;
            if (rootHtml.Length < 3000)
                throw new InvalidDataException(
                    $"SketchUp _index.html suspiciously short ({rootHtml.Length} bytes; expected ~30KB+). " +
                    $"Head: {rootHtml.Substring(0, Math.Min(200, rootHtml.Length))}");

            var typeUrls = ExtractTypeUrls(rootHtml, v.baseUrl);
            Console.Error.WriteLine($"[sketchup-extract] discovered {typeUrls.Count} type URLs from _index.html");

            parsed = await CrawlTypePages(scraper, typeUrls, state, errorsLog, version, outPath);

            if (outPath is not null) WriteSnapshot(version, state, parsed, outPath);
        }
        else
        {
            Console.Error.WriteLine($"[sketchup-extract] resumed from snapshot: {parsed.Count} types already on disk");
        }

        var builder = new IRBuilder("sketchup", version, "scrape");
        foreach (var u in state.SourceUrls) builder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) builder.IncrementPageCount();
        foreach (var p in parsed) builder.AddType(p.Type);

        Console.Error.WriteLine($"[sketchup-extract] done: types={parsed.Count}");

        return builder.Build();
    }

    // ── snapshot read/write ───────────────────────────────────────────────────

    static List<PageParser.ParseResult>? TryLoadSnapshot(string version, string? outPath, string rootUrl, SnapshotState state)
    {
        if (string.IsNullOrEmpty(outPath) || !File.Exists(outPath)) return null;
        try
        {
            var json = File.ReadAllText(outPath);
            var ir = JsonSerializer.Deserialize(json, IrJsonContext.Default.CoverageIR);
            if (ir is null || ir.host != "sketchup" || ir.host_version != version) return null;

            var results = new List<PageParser.ParseResult>(ir.types.Count);
            foreach (var t in ir.types) results.Add(new PageParser.ParseResult(t));
            state.PageCount = ir.metadata.page_count;
            state.SourceUrls.Clear();
            foreach (var u in ir.source.urls) state.SourceUrls.Add(u);
            return results;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[sketchup-extract] snapshot load failed; will re-crawl: {ex.Message}");
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
        var snapshotBuilder = new IRBuilder("sketchup", version, "scrape");
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

    // ── type-page crawl ───────────────────────────────────────────────────────

    static async Task<List<PageParser.ParseResult>> CrawlTypePages(
        HttpScraper scraper, IReadOnlyList<string> typeUrls, SnapshotState state, ErrorsLog errorsLog,
        string version, string? outPath)
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
                        if (parsedOk % 25 == 0)
                            Console.Error.WriteLine($"[sketchup-extract] type-page progress {parsedOk}/{typeUrls.Count}");
                        if (parsedOk % SnapshotEvery == 0 && outPath is not null)
                        {
                            WriteSnapshot(version, state, parsed, outPath);
                        }
                    }
                }
            }
            catch (Exception ex)
            {
                lock (addLock) failedUrls.Add((url, ex.Message));
                errorsLog.Append("type-page-fetch", url, ex.Message);
            }
        });

        // Retry hard failures up to 2 times each with backoff. Trimble's CDN rarely throttles,
        // but transient 5xx happens.
        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[sketchup-extract] retry hard type-page failures: {failedUrls.Count}");
            var stillFailing = new List<(string url, string error)>();
            foreach (var (failedUrl, _) in failedUrls)
            {
                bool recovered = false;
                for (int attempt = 1; attempt <= 2; attempt++)
                {
                    try
                    {
                        await Task.Delay(750);
                        var html = await scraper.FetchHtml(failedUrl);
                        var result = PageParser.Parse(html, failedUrl);
                        state.PageCount++;
                        if (result is null) skippedUrls.Add(failedUrl);
                        else { parsed.Add(result); parsedOk++; }
                        recovered = true;
                        Console.Error.WriteLine($"[sketchup-extract] RETRY ok ({attempt}/2): {failedUrl}");
                        break;
                    }
                    catch (Exception ex)
                    {
                        if (attempt == 2) stillFailing.Add((failedUrl, ex.Message));
                    }
                }
                if (!recovered) Console.Error.WriteLine($"[sketchup-extract] RETRY still failing: {failedUrl}");
            }
            if (stillFailing.Count > 0)
            {
                Console.Error.WriteLine($"[sketchup-extract] hard type-page failures after retry ({stillFailing.Count}):");
                foreach (var (u, e) in stillFailing.Take(20)) Console.Error.WriteLine($"  FAIL {u}: {e}");
                foreach (var (u, e) in stillFailing) errorsLog.Append("type-page-retry-final", u, e);
            }
        }

        if (skippedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[sketchup-extract] skipped non-type pages ({skippedUrls.Count}):");
            foreach (var u in skippedUrls.Take(20)) Console.Error.WriteLine($"  SKIP {u}");
            if (skippedUrls.Count > 20) Console.Error.WriteLine($"  ... +{skippedUrls.Count - 20} more");
        }

        Console.Error.WriteLine(
            $"[sketchup-extract] type-page pass done: ok={parsedOk} skip={skippedUrls.Count} fail={failedUrls.Count} of {typeUrls.Count}");
        return parsed;
    }

    // ── type URL discovery from the alphabetic index page ─────────────────────

    /// <summary>
    /// Parse the SketchUp _index.html alphabetic listing for class/module URLs. YARD's index
    /// page enumerates every documented type via &lt;span class="object_link"&gt; anchors. We
    /// pull every such anchor; the actual type pages are .html files (no folder layout, just
    /// `Foo/Bar.html` for nested types under namespace `Foo`).
    /// </summary>
    internal static List<string> ExtractTypeUrls(string html, string baseUrl)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        var urls = new HashSet<string>(StringComparer.Ordinal);
        // Approach: pick every <a href="X.html"> inside <span class="object_link"> across the
        // page. _index.html lists every type once in the alphabetic body; YARD also includes
        // top-level navigation links (e.g. file.X.html) under <li class="r1"> — we filter
        // those by skipping any href starting with "file." (those are docs pages, not types)
        // or that includes any of the YARD-internal prefixes (`_index`, `class_list`,
        // `method_list`).
        foreach (var link in doc.QuerySelectorAll("span.object_link a"))
        {
            var href = link.GetAttribute("href") ?? "";
            if (string.IsNullOrEmpty(href)) continue;
            if (!href.EndsWith(".html", StringComparison.Ordinal)) continue;
            // Skip file.* docs pages (e.g. file.ReleaseNotes.html, file.LayOut.html) — they
            // describe topic narratives, not API types.
            var fileNameOnly = Path.GetFileName(href);
            if (fileNameOnly.StartsWith("file.", StringComparison.Ordinal)) continue;
            // Skip YARD-internal pages.
            if (fileNameOnly is "_index.html" or "class_list.html" or "method_list.html"
                or "top-level-namespace.html" or "frames.html") continue;
            // Skip anchor links (`href="#..."`) — they point inside the current page.
            if (href.StartsWith("#", StringComparison.Ordinal)) continue;
            urls.Add(AbsoluteUrl(href, baseUrl));
        }

        return urls.OrderBy(u => u, StringComparer.Ordinal).ToList();
    }

    static string AbsoluteUrl(string href, string baseUrl)
    {
        if (string.IsNullOrEmpty(href)) return "";
        if (href.StartsWith("http", StringComparison.Ordinal)) return href;
        try
        {
            var uri = new Uri(new Uri(baseUrl), href);
            return uri.ToString();
        }
        catch
        {
            if (baseUrl.EndsWith("/", StringComparison.Ordinal)) return baseUrl + href;
            var slash = baseUrl.LastIndexOf('/');
            return slash < 0 ? href : baseUrl.Substring(0, slash + 1) + href;
        }
    }
}
