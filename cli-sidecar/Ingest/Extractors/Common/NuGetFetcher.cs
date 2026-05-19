// NuGetFetcher — shared helper used by vendor extractors whose source-of-truth
// is a NuGet package on nuget.org rather than a web doc site or a CHM.
//
// Strategy: download the .nupkg via nuget.org's v2 flat container URL
// `https://www.nuget.org/api/v2/package/{id}/{version}` (a permanent redirect
// to the canonical CDN at api.nuget.org), unpack into a per-package cache
// directory under %TEMP% / $TMPDIR, and return the local paths to the extracted
// content. Callers (extractors) then pick the assembly + sibling .xml under
// `lib/<tfm>/`.
//
// Why NuGet for the Navisworks extractor:
// ---------------------------------------
// Autodesk ships the Navisworks API reference as `NavisworksAPI.chm` inside the
// product install (under `C:\Program Files\Autodesk\Navisworks Manage 2026\api\NET\Documentation\`).
// Getting that CHM out-of-band requires either a Navisworks Manage license install
// or a logged-in Autodesk Account download — neither is usable from a clean CI
// machine. The community-maintained NuGet package
// `Chuongmep.Navis.Api.Autodesk.Navisworks.Api` re-distributes the same
// `Autodesk.Navisworks.Api.dll` plus its auto-generated `Autodesk.Navisworks.Api.xml`
// doc file. Both come from the same docstrings the CHM is built from (Sandcastle
// emits both formats from the XML doc commentary), so the catalog data we
// extract is identical to what a CHM-based extractor would produce — minus the
// Sandcastle HTML wrapping. Source kind is stamped as "nuget".
//
// Cache layout: `%TEMP%/aware-nuget/{id-lower}/{version}/`.
// - `package.nupkg` — the downloaded file (kept for resumability + audit trail)
// - extracted contents under the same dir (NuGet packages are ZIP archives)
//
// AOT: System.Net.Http.HttpClient + System.IO.Compression.ZipFile. Both fully
// AOT-compatible.
//
// Concurrency: each call instantiates a fresh HttpClient. NuGet's CDN is heavily
// cached and tolerates concurrent fetches, but we don't need parallelism here —
// a single package per vendor is enough for the source-of-truth extraction.
//
// Idempotency: if the target `.nupkg` already exists under the cache dir we skip
// the download. This lets resumed runs (or re-runs against the same version) be
// effectively free after the first fetch.

using System.IO.Compression;
using System.Net.Http;

namespace AwareSidecar.Ingest.Extractors.Common;

public sealed class NuGetFetcher
{
    readonly string _packageId;
    readonly string _version;
    readonly string _cacheDir;

    /// <summary>
    /// Constructs a fetcher for the given (id, version) pair. The cache directory
    /// is derived from %TEMP% + package id (lower-cased) + version, matching the
    /// convention used by ChmParser.
    /// </summary>
    public NuGetFetcher(string packageId, string version)
    {
        _packageId = packageId ?? throw new ArgumentNullException(nameof(packageId));
        _version = version ?? throw new ArgumentNullException(nameof(version));
        _cacheDir = Path.Combine(
            Path.GetTempPath(),
            "aware-nuget",
            packageId.ToLowerInvariant(),
            version);
    }

    /// <summary>
    /// Local cache directory for this (id, version). Always created by Fetch().
    /// </summary>
    public string CacheDir => _cacheDir;

    /// <summary>
    /// Path to the downloaded .nupkg under the cache dir (file may not exist
    /// until Fetch() is called).
    /// </summary>
    public string NupkgPath => Path.Combine(_cacheDir, "package.nupkg");

    /// <summary>
    /// Public URL on nuget.org for the package's flat container endpoint. We
    /// surface this so the extractor can include the canonical source URL in the
    /// IR's `source.urls[]`.
    /// </summary>
    public string PackageUrl =>
        $"https://www.nuget.org/api/v2/package/{_packageId}/{_version}";

    /// <summary>
    /// Downloads the .nupkg (if not already cached) and extracts its ZIP contents
    /// into the cache directory. Returns the directory path. Throws on HTTP
    /// failures (including 404 → unknown package/version).
    /// </summary>
    public async Task<string> FetchAsync(CancellationToken ct = default)
    {
        Directory.CreateDirectory(_cacheDir);

        if (!File.Exists(NupkgPath))
        {
            using var http = new HttpClient
            {
                // NuGet's API v2 redirects to api.nuget.org once; HttpClient follows
                // redirects by default. 30s is generous; the standard nupkg is
                // single-digit megabytes.
                Timeout = TimeSpan.FromSeconds(60),
            };
            http.DefaultRequestHeaders.UserAgent.ParseAdd("aware-cli-sidecar/0.30");

            using var resp = await http.GetAsync(PackageUrl, HttpCompletionOption.ResponseHeadersRead, ct)
                .ConfigureAwait(false);
            resp.EnsureSuccessStatusCode();

            var tmpPath = NupkgPath + ".part";
            using (var fileStream = new FileStream(tmpPath, FileMode.Create, FileAccess.Write, FileShare.None))
            {
                await resp.Content.CopyToAsync(fileStream, ct).ConfigureAwait(false);
            }
            File.Move(tmpPath, NupkgPath, overwrite: true);
        }

        // Always re-extract (cheap; idempotent). ZipFile.ExtractToDirectory throws
        // if the destination contains conflicting files, so we use the per-entry
        // overload that overwrites by default.
        ExtractZipOverwrite(NupkgPath, _cacheDir);
        return _cacheDir;
    }

    /// <summary>
    /// Locates the first DLL under `lib/<tfm>/` that matches the given assembly
    /// name. Throws if no match is found.
    /// </summary>
    public string FindAssembly(string assemblyName)
    {
        var libDir = Path.Combine(_cacheDir, "lib");
        if (!Directory.Exists(libDir))
            throw new FileNotFoundException(
                $"NuGet package {_packageId} {_version} does not contain a lib/ directory under {_cacheDir}");

        foreach (var dll in Directory.EnumerateFiles(libDir, $"{assemblyName}.dll", SearchOption.AllDirectories))
        {
            return dll;
        }
        throw new FileNotFoundException(
            $"assembly {assemblyName}.dll not found under {libDir} (NuGet package {_packageId} {_version})");
    }

    /// <summary>
    /// Locates the XML doc file that sits next to the given assembly DLL. Returns
    /// null if no XML doc is shipped — the caller should treat this as a
    /// "no-summaries" run and decide whether to abort or continue.
    /// </summary>
    public string? FindXmlDoc(string dllPath)
    {
        var xmlPath = Path.ChangeExtension(dllPath, ".xml");
        return File.Exists(xmlPath) ? xmlPath : null;
    }

    static void ExtractZipOverwrite(string zipPath, string destDir)
    {
        using var archive = ZipFile.OpenRead(zipPath);
        foreach (var entry in archive.Entries)
        {
            // Skip directory entries (zero length, name ends in /).
            if (string.IsNullOrEmpty(entry.Name) && entry.FullName.EndsWith("/", StringComparison.Ordinal))
                continue;

            // ZipArchiveEntry.FullName uses forward slashes; Path.Combine on Windows
            // handles them transparently. Guard against directory-traversal entries.
            var safeName = entry.FullName.Replace('\\', '/');
            if (safeName.Contains("..", StringComparison.Ordinal))
                continue;
            var destPath = Path.GetFullPath(Path.Combine(destDir, safeName));
            if (!destPath.StartsWith(destDir, StringComparison.OrdinalIgnoreCase))
                continue;

            Directory.CreateDirectory(Path.GetDirectoryName(destPath)!);
            entry.ExtractToFile(destPath, overwrite: true);
        }
    }
}
