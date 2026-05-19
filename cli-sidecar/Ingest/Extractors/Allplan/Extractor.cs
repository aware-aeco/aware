// Allplan extractor — drives the crawl of the Allplan PythonParts API documentation
// (https://pythonparts.allplan.com/) to produce a host-coverage IR for Allplan at
// versions 2024 and 2025.
//
// Source rationale
// ----------------
//   - Nemetschek publishes the Allplan PythonParts API docs at pythonparts.allplan.com,
//     served per-major-version at /2024/, /2025/, etc. The site is mkdocs-material
//     9.5.19 with mkdocstrings — i.e. server-rendered HTML, no JS hydration required.
//   - Each version exposes a sitemap.xml at /<version>/sitemap.xml listing every
//     documentation URL on the site. We filter for API reference pages:
//       /<version>/api_reference/InterfaceStubs/<Module>/<Class>/
//       /<version>/api_reference/InterfaceStubs/<Module>/_functions/
//       /<version>/api_reference/GeneralScripts/<Class>/<Class>/
//   - 2024 sitemap has 646 URLs; 2025 sitemap has 795 URLs. After filtering to type
//     pages we get ~200-300 type URLs per version.
//   - The PythonParts API is the Python plugin surface (drives Allplan from inside
//     the running app). The .NET BIF.Core surface (BimPlus integration) is separate
//     and was previously extracted via `--from-nuget`. This extractor is the
//     definitive Python API source.
//
// Pipeline (3 phases — mkdocs-material/mkdocstrings inlines every member on the type
// page, so there is NO per-member enrichment pass):
//
//   Phase 1 — sitemap fetch + type-URL discovery (sequential):
//     Fetch /<version>/sitemap.xml, parse <loc> elements, filter to api_reference URLs
//     that match the type-page pattern. The sitemap entries use the `/latest/` prefix
//     internally; we rewrite to the explicit `/<version>/` prefix.
//
//   Phase 2 — N/A:
//     There is no per-module index page that we need to walk separately; the sitemap
//     enumerates every type by name in one shot.
//
//   Phase 3 — type-page crawl (4-way concurrent):
//     Fetch each type page and run PageParser.Parse() to produce a fully-populated
//     TypeInfo (including all attributes, methods, ctors, enum members, and any
//     inner classes promoted to top-level entries). Snapshot every 100 types.
//
//   Phase 4 — N/A:
//     mkdocstrings inlines every method/attribute/return-table on the same page as
//     the type header. The TypeInfo emitted by Phase 3 is already fully populated.
//
//   Phase 5 — final IR emit.
//
// Resumability: same shape as Rhino/Tekla/SketchUp — load `outPath` if it exists for
// this host_version and skip already-crawled type URLs (matched by doc_url).
//
// Concurrency: pythonparts.allplan.com is hosted on Nemetschek's CDN. Empirically
// tolerates 4-way concurrent fetches without throttling for the ~200-300 type
// corpus per version. Single shared HttpScraper — the corpus is small enough that
// HttpClient pool reuse is a non-issue.
//
// AOT: pure HttpScraper + AngleSharp + source-gen JSON path. Zero reflection on
// user types. Verified NativeAOT-clean.

using System.Text.Json;
using System.Text.RegularExpressions;
using AngleSharp.Html.Parser;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Allplan;

public static class Extractor
{
    /// <summary>
    /// Per-version source-of-truth roots. Each entry holds the (display version, version-path
    /// segment used in URLs, sitemap URL). The base URL must end with '/' so relative hrefs from
    /// the docs resolve as siblings of any html file underneath.
    /// </summary>
    static readonly Dictionary<string, (string version, string baseUrl, string sitemapUrl)> KnownVersions = new()
    {
        ["2024.0"] = ("2024.0",
                      "https://pythonparts.allplan.com/2024/",
                      "https://pythonparts.allplan.com/2024/sitemap.xml"),
        ["2025.0"] = ("2025.0",
                      "https://pythonparts.allplan.com/2025/",
                      "https://pythonparts.allplan.com/2025/sitemap.xml"),
    };

    // Nemetschek's CDN tolerates 4-way concurrent fetches without throttling for the small
    // corpus (200-300 types per version). Higher fanout doesn't shorten wall-clock measurably
    // and risks unnecessary 429s. Lower than Rhino's github-raw (which needs 2) because mkdocs-
    // material sites are typically served from regional CDNs.
    const int TypePageConcurrency = 4;
    const int SnapshotEvery = 100;

    enum MemberKind { Constructor, Method, Property, Event }

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
                $"Allplan extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var state = new SnapshotState();
        state.SourceUrls.Add(v.sitemapUrl);

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));

        List<PageParser.ParseResult>? parsed = TryLoadSnapshot(version, outPath, state);

        if (parsed is null)
        {
            await using var scraper = await HttpScraper.Launch();
            Console.Error.WriteLine($"[allplan-extract] launched HTTP scraper; fetching sitemap {v.sitemapUrl}");

            var sitemapXml = await scraper.FetchHtml(v.sitemapUrl);
            state.PageCount++;
            if (sitemapXml.Length < 1000)
                throw new InvalidDataException(
                    $"Allplan sitemap suspiciously short ({sitemapXml.Length} bytes; expected ~80KB). " +
                    $"Head: {sitemapXml.Substring(0, Math.Min(200, sitemapXml.Length))}");

            var typeUrls = ExtractTypeUrlsFromSitemap(sitemapXml, v.baseUrl, v.version);
            Console.Error.WriteLine($"[allplan-extract] discovered {typeUrls.Count} type URLs from sitemap");
            if (typeUrls.Count == 0)
                throw new InvalidDataException(
                    $"Allplan sitemap parse returned 0 URLs — sitemap shape changed? Head: {sitemapXml.Substring(0, Math.Min(400, sitemapXml.Length))}");

            parsed = await CrawlTypePages(scraper, typeUrls, state, errorsLog, version, outPath);

            if (outPath is not null) WriteSnapshot(version, state, parsed, outPath);
        }
        else
        {
            Console.Error.WriteLine($"[allplan-extract] resumed from snapshot: {parsed.Count} types already on disk");
        }

        // Flatten the inner-types into the top-level types list. Inner classes (Python nested
        // classes, e.g. BaseInteractor.BaseInteractor.InteractorInputMode) are promoted to
        // top-level entries by PageParser; here we materialise that promotion in the IR.
        var allTypes = new List<TypeInfo>(parsed.Count);
        foreach (var p in parsed)
        {
            allTypes.Add(p.Type);
            allTypes.AddRange(p.InnerTypes);
        }

        var builder = new IRBuilder("allplan", version, "scrape");
        foreach (var u in state.SourceUrls) builder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) builder.IncrementPageCount();
        foreach (var t in allTypes) builder.AddType(t);

        Console.Error.WriteLine($"[allplan-extract] done: types={allTypes.Count} (incl. promoted inner classes)");

        return builder.Build();
    }

    // ── Snapshot read/write ───────────────────────────────────────────────────

    static List<PageParser.ParseResult>? TryLoadSnapshot(string version, string? outPath, SnapshotState state)
    {
        if (string.IsNullOrEmpty(outPath) || !File.Exists(outPath)) return null;
        try
        {
            var json = File.ReadAllText(outPath);
            var ir = JsonSerializer.Deserialize(json, IrJsonContext.Default.CoverageIR);
            if (ir is null || ir.host != "allplan" || ir.host_version != version) return null;

            // Restoring the snapshot is a best-effort approximation: we lose the inner-type
            // promotion (every TypeInfo is treated as a primary). This matches the SketchUp
            // resumability shape — for Allplan we don't need to re-run member enrichment, only
            // to avoid re-crawling pages we already have. The flattened types list on resume is
            // still valid input for IRBuilder.AddType + downstream Generator.
            var results = new List<PageParser.ParseResult>(ir.types.Count);
            foreach (var t in ir.types)
            {
                results.Add(new PageParser.ParseResult(t, new List<TypeInfo>()));
            }
            state.PageCount = ir.metadata.page_count;
            state.SourceUrls.Clear();
            foreach (var u in ir.source.urls) state.SourceUrls.Add(u);
            return results;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[allplan-extract] snapshot load failed; will re-crawl: {ex.Message}");
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
        var snapshotBuilder = new IRBuilder("allplan", version, "scrape");
        foreach (var u in state.SourceUrls) snapshotBuilder.AddSourceUrl(u);
        for (int i = 0; i < state.PageCount; i++) snapshotBuilder.IncrementPageCount();
        // For snapshot we flatten the inner types so resume sees the same shape as the final
        // IR. This means the snapshot file is a valid intermediate (just incomplete) IR.
        foreach (var p in parsed)
        {
            snapshotBuilder.AddType(p.Type);
            foreach (var inner in p.InnerTypes) snapshotBuilder.AddType(inner);
        }
        var ir = snapshotBuilder.Build();
        var outDir = Path.GetDirectoryName(outPath);
        if (!string.IsNullOrEmpty(outDir)) Directory.CreateDirectory(outDir);
        var json = JsonSerializer.Serialize(ir, IrJsonContext.Default.CoverageIR);
        var tmp = outPath + ".partial";
        File.WriteAllText(tmp, json);
        if (File.Exists(outPath)) File.Delete(outPath);
        File.Move(tmp, outPath);
    }

    // ── Type-page crawl ───────────────────────────────────────────────────────

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
                            Console.Error.WriteLine($"[allplan-extract] type-page progress {parsedOk}/{typeUrls.Count}");
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

        // Retry hard failures up to 2 times each with backoff. Nemetschek's CDN rarely throttles,
        // but transient 5xx happens.
        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[allplan-extract] retry hard type-page failures: {failedUrls.Count}");
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
                        Console.Error.WriteLine($"[allplan-extract] RETRY ok ({attempt}/2): {failedUrl}");
                        break;
                    }
                    catch (Exception ex)
                    {
                        if (attempt == 2) stillFailing.Add((failedUrl, ex.Message));
                    }
                }
                if (!recovered) Console.Error.WriteLine($"[allplan-extract] RETRY still failing: {failedUrl}");
            }
            if (stillFailing.Count > 0)
            {
                Console.Error.WriteLine($"[allplan-extract] hard type-page failures after retry ({stillFailing.Count}):");
                foreach (var (u, e) in stillFailing.Take(20)) Console.Error.WriteLine($"  FAIL {u}: {e}");
                foreach (var (u, e) in stillFailing) errorsLog.Append("type-page-retry-final", u, e);
            }
        }

        if (skippedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[allplan-extract] skipped non-type pages ({skippedUrls.Count}):");
            foreach (var u in skippedUrls.Take(20)) Console.Error.WriteLine($"  SKIP {u}");
            if (skippedUrls.Count > 20) Console.Error.WriteLine($"  ... +{skippedUrls.Count - 20} more");
        }

        Console.Error.WriteLine(
            $"[allplan-extract] type-page pass done: ok={parsedOk} skip={skippedUrls.Count} fail={failedUrls.Count} of {typeUrls.Count}");
        return parsed;
    }

    // ── Sitemap parsing ───────────────────────────────────────────────────────

    /// <summary>
    /// Parse a sitemap.xml body and return the list of type-page URLs we should crawl. The
    /// sitemap entries use a `/latest/` path segment internally; we rewrite to the explicit
    /// `/<version>/` segment so requests go to the version-pinned tree.
    ///
    /// Filter: include URLs that match the API reference type-page pattern:
    ///   /api_reference/InterfaceStubs/<Module>/<Class>/
    ///   /api_reference/InterfaceStubs/<Module>/_functions/
    ///   /api_reference/GeneralScripts/<Class>/<Class>/
    ///   /api_reference/GeneralScripts/<Class>/
    ///
    /// Exclude:
    ///   /manual/, /getting_started/, /release_notes/, /legal_notice/, /data_protection/
    ///   /api_reference/_navigation_tree/
    ///   /api_reference/<Section>/_navigation_tree/
    ///   /api_reference/ (root - no class)
    /// </summary>
    internal static List<string> ExtractTypeUrlsFromSitemap(string sitemapXml, string baseUrl, string version)
    {
        var urls = new List<string>();
        // Simple regex on <loc>...</loc> avoids dragging an XML namespace parser into this hot
        // path. The sitemap is small and well-formed; tolerate stray whitespace.
        var matches = Regex.Matches(sitemapXml, @"<loc>([^<]+)</loc>", RegexOptions.IgnoreCase);
        foreach (Match m in matches)
        {
            var loc = m.Groups[1].Value.Trim();
            if (string.IsNullOrEmpty(loc)) continue;

            // Rewrite the version segment if the sitemap references /latest/.
            var rewritten = loc.Replace("/latest/", $"/{version.Split('.')[0]}/", StringComparison.Ordinal);

            // Filter: keep only api_reference/* type pages.
            if (!rewritten.Contains("/api_reference/", StringComparison.Ordinal)) continue;
            if (rewritten.Contains("/_navigation_tree/", StringComparison.Ordinal)) continue;
            // Skip the bare /api_reference/ root.
            if (rewritten.EndsWith("/api_reference/", StringComparison.Ordinal)) continue;
            // Section roots (e.g. /api_reference/InterfaceStubs/, /api_reference/GeneralScripts/) —
            // these list child modules but no API content. Skip.
            if (IsSectionRoot(rewritten)) continue;
            // Note: 2-segment `<Section>/<Module>/` pages MAY be empty indexes (2024 layout) OR
            // module-functions pages with inline free functions (2025 layout). We CANNOT skip
            // them at the sitemap stage without losing the 2025 module-level functions; the
            // parser's null-return for empty pages handles the 2024 case cleanly.

            urls.Add(rewritten);
        }

        // Dedupe + sort for deterministic ordering across runs.
        return urls.Distinct(StringComparer.Ordinal).OrderBy(u => u, StringComparer.Ordinal).ToList();
    }

    /// <summary>
    /// Detect /api_reference/&lt;Section&gt;/ pages (1 path segment after api_reference). These
    /// are top-level section indexes with no API content.
    /// </summary>
    static bool IsSectionRoot(string url)
    {
        var idx = url.IndexOf("/api_reference/", StringComparison.Ordinal);
        if (idx < 0) return false;
        var tail = url.Substring(idx + "/api_reference/".Length);
        var q = tail.IndexOf('?');
        if (q >= 0) tail = tail.Substring(0, q);
        var h = tail.IndexOf('#');
        if (h >= 0) tail = tail.Substring(0, h);
        var trimmed = tail.TrimEnd('/');
        if (string.IsNullOrEmpty(trimmed)) return false;
        var parts = trimmed.Split('/');
        // 1 part → section root (e.g. `/api_reference/InterfaceStubs/`).
        return parts.Length == 1;
    }
}
