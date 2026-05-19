// Bluebeam extractor — produces a host-coverage IR for the Bluebeam Studio API.
//
// Source rationale:
// ----------------
// Bluebeam's developer portal at https://developers.bluebeam.com/studio-api/ is
// auth-walled (Salesforce-hosted dev network requires sign-in to view API refs).
// However, Bluebeam ALSO publishes a fully-public Postman v2.1 collection at
// https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json
// (referenced from the public "Getting Started" guide at
// https://support.bluebeam.com/developer/getting-started-dev-portal.html). The
// collection is the canonical machine-readable description of the Studio API:
// 123 operations across 4 top-level folders.
//
// Coverage:
// ---------
// • Sessions (24 ops): /publicapi/v1/sessions plus Files, Permissions, Users,
//   Activities subfolders, and Single Session rollup ops.
// • Projects (51 ops): /publicapi/v1/projects plus Folders, Files, Users,
//   Permissions, Shared Links, healthcheck, and Single Project rollup ops.
// • Jobs (49 ops): Project File Jobs (44 PDF/document-processing ops) + Job
//   lifecycle (5 ops: list / get / cancel / results).
// • Misc (4 ops): healthcheck / users-me / api-version.
//
// Pipeline:
// ---------
//   Phase 1 — fetch the Postman collection JSON once (a single ~45KB GET against
//             support.bluebeam.com). No auth required.
//   Phase 2 — parse the collection (PageParser.ParseCollection) into a list of
//             TypeInfo records: one synthetic *Api class per (top-level, subfolder)
//             pair (~14 *Api classes), plus one synthesised Request DTO per
//             operation that has a request body (~60 DTOs).
//   Phase 3 — assemble the IR via IRBuilder; stamp source.kind="scrape" and
//             source.urls=[the Postman JSON URL].
//
// Resumability:
// -------------
// The entire pipeline is one HTTP fetch + one in-memory parse. End-to-end wall
// time is <5 seconds. There's no concurrency, no per-member pass, and nothing
// to snapshot — a kill-9 mid-run just costs you those 5 seconds.
//
// Version stamping:
// -----------------
// The Studio API path prefix is `/publicapi/v1/` and Bluebeam does not version
// the surface in any visible way (no v2, no per-product release notes for the
// API itself). We stamp host_version="v1" to match the path prefix. Per the
// brief's sample-size adaptation rule, the strict-verify sampler will still run
// against the full 50-type sample (we have ~70 types in the IR, comfortably
// above the 50 threshold).

using System.Text.Json;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Bluebeam;

public static class Extractor
{
    /// <summary>
    /// Known version selector → (display version, source URL). Bluebeam ships ONE
    /// version of the Studio API today. We accept "v1" (the path prefix) and the
    /// bare "1.0" alias for ergonomics.
    /// </summary>
    static readonly Dictionary<string, string> KnownVersions = new()
    {
        ["v1"]  = "v1",
        ["1.0"] = "v1",   // alias; some callers prefer the SemVer-like form
    };

    /// <summary>
    /// Errors log mirroring the other vendors. Created on demand.
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

    public static Task<CoverageIR> Extract(string version) => Extract(version, outPath: null);

    public static async Task<CoverageIR> Extract(string version, string? outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var displayVersion))
            throw new ArgumentException(
                $"Bluebeam extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}");

        var errorsLog = new ErrorsLog(DeriveErrorsLogPath(outPath));
        Console.Error.WriteLine($"[bluebeam-extract] starting for host_version={displayVersion}");

        // Resume support: a previous snapshot is unlikely to be stale (the source is
        // single-shot), but if a partial file exists we still re-extract so the user
        // doesn't end up with stale data after a vendor JSON update.
        if (outPath is not null && File.Exists(outPath))
        {
            try
            {
                var oldJson = File.ReadAllText(outPath);
                var loaded = JsonSerializer.Deserialize(oldJson, IrJsonContext.Default.CoverageIR);
                if (loaded is not null && loaded.host == "bluebeam" && loaded.host_version == displayVersion)
                {
                    Console.Error.WriteLine($"[bluebeam-extract] existing IR found at {outPath} ({loaded.types.Count} types); overwriting with fresh extraction (Bluebeam source is single-shot, so re-parse is cheap)");
                }
            }
            catch
            {
                // Bad snapshot — overwrite freshly.
            }
        }

        // Phase 1 — fetch the Postman collection JSON.
        string collectionJson;
        var collectionUrl = PageParser.CollectionJsonUrl;
        try
        {
            await using var scraper = await HttpScraper.Launch();
            collectionJson = await scraper.FetchHtml(collectionUrl);
            Console.Error.WriteLine($"[bluebeam-extract] fetched {collectionJson.Length:N0} bytes from {collectionUrl}");
        }
        catch (Exception ex)
        {
            errorsLog.Append("postman-fetch", collectionUrl, ex.Message);
            throw new InvalidOperationException($"Failed to fetch Postman collection from {collectionUrl}: {ex.Message}", ex);
        }

        // Sanity: a meaningful collection is > 5KB; if we got back tiny payload, something
        // is wrong (auth redirect HTML, 404 page, etc.) and we should fail hard.
        if (collectionJson.Length < 5_000)
        {
            errorsLog.Append("postman-validate", collectionUrl, $"suspiciously short response: {collectionJson.Length} bytes");
            throw new InvalidDataException(
                $"Postman collection JSON suspiciously short ({collectionJson.Length} bytes). " +
                $"Head: {collectionJson.Substring(0, Math.Min(200, collectionJson.Length))}");
        }
        if (!collectionJson.TrimStart().StartsWith("{"))
        {
            errorsLog.Append("postman-validate", collectionUrl, "response does not start with `{` — not JSON");
            throw new InvalidDataException(
                $"Postman collection response is not JSON. Head: {collectionJson.Substring(0, Math.Min(200, collectionJson.Length))}");
        }

        // Phase 2 — parse into TypeInfo records.
        List<TypeInfo> types;
        try
        {
            types = PageParser.ParseCollection(collectionJson);
        }
        catch (Exception ex)
        {
            errorsLog.Append("postman-parse", collectionUrl, ex.Message);
            throw new InvalidOperationException($"Failed to parse Postman collection: {ex.Message}", ex);
        }

        Console.Error.WriteLine($"[bluebeam-extract] parsed {types.Count} types from collection");

        // Phase 3 — assemble the IR.
        var builder = new IRBuilder("bluebeam", displayVersion, "scrape");
        builder.AddSourceUrl(collectionUrl);
        builder.IncrementPageCount();  // one HTTP round-trip
        foreach (var t in types) builder.AddType(t);
        var ir = builder.Build();

        Console.Error.WriteLine(
            $"[bluebeam-extract] done: types={ir.types.Count} methods={ir.metadata.method_count} properties={ir.metadata.property_count} events={ir.metadata.event_count}");
        return ir;
    }
}
