// Tekla extractor — drives the crawl of developer.tekla.com to produce a host-coverage
// IR for Tekla Structures (versions 2025 and 2026).
//
// Crawl shape (verified 2026-05-18; see EXTRACTION-NOTES.md for the live evidence):
//
//   1. Start from the version root namespace page:
//        2026 → /doc/tekla-structures/2026/tekla-structures-64304
//        2025 → /doc/tekla-structures/2025/tekla-structures-45473
//
//   2. The sidebar inside <section class="api-packages-tree"> contains the full list of
//      Tekla.Structures.* namespaces. Each namespace anchor's text ends with " Namespace".
//
//   3. For each namespace page we fetch the rendered HTML and extract every type link by
//      walking the <table class="members"> tables with ids classList / interfaceList /
//      structureList / enumerationList / delegateList.
//
//   4. For each type page we hand the rendered HTML to PageParser.Parse() which returns
//      a TypeInfo (or null for non-type pages — see PageParser).
//
//   5. The IRBuilder accumulates the types and stamps metadata (counts + extracted_at).
//
// We use Common.HttpScraper for every fetch because the Tekla docs site is fully
// server-rendered — the sidebar tree, the type tables, and every member row are all
// in the initial HTML response. Empirically verified 2026-05-18 against the live site:
// the sidebar's <ul> tree carries every namespace URL, and each namespace landing page
// carries the entire <table class="members"> for every type in that namespace. No JS
// hydration is needed for the data we extract.
//
// (Phase A handoff notes claimed Playwright was AOT-compatible. It is — for the Node
// driver process — but Playwright's .NET transport uses reflection-based JSON
// internally, which `<PublishAot>true</PublishAot>` disables, throwing
// InvalidOperationException at CreateAsync time. Switching to HttpClient avoids the
// AOT trap entirely and is the right tool for a server-rendered site. The original
// Playwright Scraper is left in place for vendor sites that genuinely need JS
// hydration; those will need a separate AOT story.)
//
// The plan budgets ~850 types per version. Sequential HttpClient fetches with a
// short polite-bot delay run the full crawl in roughly 10-15 minutes per version
// on a normal home connection. We log progress to STDERR so the sidecar STDOUT
// stays a clean JSON envelope when invoked via the protocol.

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

    /// <summary>
    /// Extract a complete host-coverage IR for Tekla Structures at the given version.
    /// `version` accepts plan-input form ("2026.0", "2025.0"); we map to the doc-site
    /// year form internally.
    /// </summary>
    public static async Task<CoverageIR> Extract(string version)
    {
        if (!KnownVersions.TryGetValue(version, out var v))
            throw new ArgumentException(
                $"Tekla extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var builder = new IRBuilder("tekla", version, "scrape");
        var rootUrl = $"{BaseHost}/doc/tekla-structures/{v.version}/tekla-structures-{v.rootId}";
        builder.AddSourceUrl(rootUrl);

        await using var scraper = await HttpScraper.Launch();
        Console.Error.WriteLine($"[tekla-extract] launched HTTP scraper; crawling {rootUrl}");

        // 1. Discover all namespace URLs from the sidebar of the root page.
        var rootHtml = await scraper.FetchHtml(rootUrl);
        builder.IncrementPageCount();
        // Sanity-check: the Tekla root page is ~55KB. Anything noticeably smaller usually
        // means a server-side bot block returning a stub, or the response body was not
        // properly decompressed (HttpScraper auto-handles gzip via AutomaticDecompression
        // but if a future vendor regresses that flag, we want a fast error).
        if (rootHtml.Length < 5000)
            throw new InvalidDataException(
                $"Tekla root page suspiciously short ({rootHtml.Length} bytes; expected ~55KB). " +
                $"Bot block or decompression regression? Head: {rootHtml.Substring(0, Math.Min(200, rootHtml.Length))}");
        var namespaceUrls = ExtractNamespaceUrls(rootHtml);
        Console.Error.WriteLine($"[tekla-extract] discovered {namespaceUrls.Count} namespaces");

        // 2. For each namespace page, harvest every type URL.
        var typeUrls = new List<string>();
        foreach (var nsUrl in namespaceUrls)
        {
            try
            {
                var nsHtml = await scraper.FetchHtml(nsUrl);
                builder.IncrementPageCount();
                var nsTypes = ExtractTypeUrls(nsHtml);
                Console.Error.WriteLine($"[tekla-extract]   {nsUrl}: {nsTypes.Count} types");
                typeUrls.AddRange(nsTypes);
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[tekla-extract] WARN failed namespace {nsUrl}: {ex.Message}");
            }
        }

        // De-duplicate (some types are linked from multiple namespaces via dotted-naming).
        typeUrls = typeUrls.Distinct().ToList();
        Console.Error.WriteLine($"[tekla-extract] total unique type URLs: {typeUrls.Count}");

        // 3. Fetch type pages concurrently with a fan-out semaphore so we don't hammer
        //    the Tekla docs server. Parsing is CPU-light; the constraint is server politeness
        //    + bandwidth. A fan-out of 4 with no inter-request delay typically completes
        //    ~850 types in 3-5 minutes against developer.tekla.com.
        int parsedOk = 0, parsedSkip = 0, parseFailed = 0;
        var skippedUrls = new List<string>();
        var failedUrls = new List<(string url, string error)>();
        var resultLock = new object();
        var sem = new SemaphoreSlim(4);
        var tasks = typeUrls.Select(async url =>
        {
            await sem.WaitAsync();
            try
            {
                var html = await scraper.FetchHtml(url);
                var typeInfo = PageParser.Parse(html, url);
                lock (resultLock)
                {
                    builder.IncrementPageCount();
                    if (typeInfo is null)
                    {
                        parsedSkip++;
                        skippedUrls.Add(url);
                        return;
                    }
                    builder.AddType(typeInfo);
                    parsedOk++;
                    if (parsedOk % 25 == 0)
                        Console.Error.WriteLine($"[tekla-extract] progress {parsedOk}/{typeUrls.Count} types parsed");
                }
            }
            catch (Exception ex)
            {
                lock (resultLock)
                {
                    parseFailed++;
                    failedUrls.Add((url, ex.Message));
                    Console.Error.WriteLine($"[tekla-extract] ERR type {url}: {ex.Message}");
                }
            }
            finally
            {
                sem.Release();
            }
        }).ToArray();
        await Task.WhenAll(tasks);

        // Retry any hard failures sequentially. Common cause is a transient timeout when many
        // concurrent fetches saturate the server briefly; with a single worker and a short
        // back-off the same URL usually succeeds. We retry up to 2 times per URL.
        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[tekla-extract] first pass hard failures: {failedUrls.Count}; retrying sequentially");
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
                        var typeInfo = PageParser.Parse(html, failedUrl);
                        builder.IncrementPageCount();
                        if (typeInfo is null)
                        {
                            parsedSkip++;
                            skippedUrls.Add(failedUrl);
                        }
                        else
                        {
                            builder.AddType(typeInfo);
                            parsedOk++;
                        }
                        parseFailed--;
                        recovered = true;
                        Console.Error.WriteLine($"[tekla-extract] RETRY ok ({attempt}/2): {failedUrl}");
                        break;
                    }
                    catch (Exception ex)
                    {
                        if (attempt == 2)
                            stillFailing.Add((failedUrl, ex.Message));
                    }
                }
                if (!recovered) Console.Error.WriteLine($"[tekla-extract] RETRY still failing: {failedUrl}");
            }
            failedUrls = stillFailing;
        }

        if (skippedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[tekla-extract] skipped non-type pages ({skippedUrls.Count}):");
            foreach (var u in skippedUrls.Take(20)) Console.Error.WriteLine($"  SKIP {u}");
            if (skippedUrls.Count > 20) Console.Error.WriteLine($"  ... +{skippedUrls.Count - 20} more");
        }
        if (failedUrls.Count > 0)
        {
            Console.Error.WriteLine($"[tekla-extract] hard failures after retry ({failedUrls.Count}):");
            foreach (var (u, e) in failedUrls.Take(20)) Console.Error.WriteLine($"  FAIL {u}: {e}");
            if (failedUrls.Count > 20) Console.Error.WriteLine($"  ... +{failedUrls.Count - 20} more");
        }

        Console.Error.WriteLine(
            $"[tekla-extract] done: ok={parsedOk} skip={parsedSkip} fail={parseFailed} of {typeUrls.Count}");

        return builder.Build();
    }

    // ── namespace URL discovery from the root page sidebar ──────────────────

    internal static List<string> ExtractNamespaceUrls(string html)
    {
        var parser = new HtmlParser();
        using var doc = parser.ParseDocument(html);
        // The sidebar tree is <section ... id="block-trimble2017-devcenter-apipackagenavigation">.
        var tree = doc.QuerySelector("#block-trimble2017-devcenter-apipackagenavigation");
        if (tree is null)
            throw new InvalidDataException("Tekla sidebar tree not found — page layout changed?");

        var urls = new List<string>();
        foreach (var a in tree.QuerySelectorAll("a"))
        {
            var text = a.TextContent.Trim();
            // Anchor text in the tree is "Tekla.Structures Namespace" / "ClassName Class" / etc.
            // We keep only the namespace entries.
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
        // Type tables on a namespace page have these ids — pulled from the member-table convention.
        string[] tableIds = ["classList", "interfaceList", "structureList", "enumerationList", "delegateList"];
        foreach (var id in tableIds)
        {
            var table = doc.QuerySelector($"table.members#{id}");
            if (table is null) continue;
            foreach (var tr in table.QuerySelectorAll("tr"))
            {
                if (tr.QuerySelector("th") != null) continue;
                // Member-row cell layout: [icon] [<a>Name</a>] [summary]
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
