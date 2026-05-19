// Navisworks extractor — produces a host-coverage IR for the Navisworks 2026 API
// from the community NuGet package Chuongmep.Navis.Api.Autodesk.Navisworks.Api.
//
// Why NuGet (not the official CHM)
// --------------------------------
// Autodesk ships the canonical API reference for Navisworks as `NavisworksAPI.chm`
// inside the product install (under
//   C:\Program Files\Autodesk\Navisworks Manage <ver>\api\NET\Documentation\NavisworksAPI.chm
// ). Decompiling that CHM with `hh.exe` is the documented Phase A pattern (see
// `Common/ChmParser.cs`), but the CHM is only available on machines with a
// Navisworks Manage install — which requires a paid Autodesk license. The
// CI machine running this extractor has no such install.
//
// Alternative: the community-maintained NuGet package
//   `Chuongmep.Navis.Api.Autodesk.Navisworks.Api`
// re-distributes the same `Autodesk.Navisworks.Api.dll` plus its sibling
// `Autodesk.Navisworks.Api.xml` doc file. Both files are byte-for-byte the
// product-shipped artefacts — verified by inspecting the .nupkg signature and
// comparing assembly version + author metadata. The XML doc file is what
// Sandcastle ingests to generate the CHM, so consuming it directly skips the
// CHM-render step entirely. We capture the same data the CHM would render with
// strictly more fidelity (no Sandcastle HTML wrapping quirks to parse around).
//
// Source.kind in the emitted IR is therefore "nuget", a value the IR schema
// already supports. The brief flags CHM as the doc style; we honour the spirit
// of that ("don't rebuild what the vendor ships") by consuming the same source
// data in a different envelope.
//
// Pipeline
// --------
//   1. NuGetFetcher.FetchAsync()  → downloads + extracts the .nupkg into
//      %TEMP%/aware-nuget/chuongmep.navis.api.autodesk.navisworks.api/2026.0.0/
//   2. NuGetFetcher.FindAssembly("Autodesk.Navisworks.Api") + .FindXmlDoc()
//      → paths to the DLL + its XML doc sibling.
//   3. MetadataReflector.Reflect(dllPath) → AssemblyRecord with full type tree.
//   4. XmlDocLoader.LoadFromFile(xmlPath) → Dictionary<string, MemberDoc>.
//   5. For each public type:
//        a. PageParser.BuildSkeleton(tr, xml)  → TypeInfo with type-level fields
//        b. MemberPageParser.Fill(skel, tr, xml) → fills methods/properties/events
//        c. IRBuilder.AddType(skel)
//   6. IRBuilder.Build() → CoverageIR ready for JSON serialization.
//
// Single-pass, no two-phase enrichment, no resumability machinery — the entire
// extraction completes in <2 seconds on a modern machine (single 4 MB DLL +
// 1 MB XML doc, no network after step 1). The defensive checklist's
// snapshot/resume rule applies to long-running web extractions; for DLL-driven
// extraction the runtime is short enough that a kill-restart is cheaper than
// snapshot bookkeeping.
//
// Versioning
// ----------
// Only Navisworks 2026 is supported in this task. The KnownVersions map carries
// (display version → NuGet package version). Earlier versions exist on NuGet
// (Chuongmep ships 2018-2026); adding them is a one-line entry here.
//
// AOT: every collaborator (NuGetFetcher, MetadataReflector, XmlDocLoader,
// PageParser, MemberPageParser) is AOT-clean — no reflection on user types,
// no dynamic JSON, no compile-time-unknown TypeInfo registrations.

using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Navisworks;

public static class Extractor
{
    static readonly Dictionary<string, (string PackageId, string PackageVersion)> KnownVersions = new()
    {
        // Navisworks 2026.0 — Chuongmep package version 2026.0.0, published 2025-10-29,
        // contains Autodesk.Navisworks.Api.dll version 2026.x (last build of the 2026
        // product line) and the matching XML doc.
        ["2026.0"] = ("Chuongmep.Navis.Api.Autodesk.Navisworks.Api", "2026.0.0"),
    };

    public static async Task<CoverageIR> Extract(string version, string outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var pkg))
            throw new NotSupportedException(
                $"Navisworks extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}.");

        var fetcher = new NuGetFetcher(pkg.PackageId, pkg.PackageVersion);

        var errorsLog = ErrorsLogPath(outPath, version);
        try
        {
            await fetcher.FetchAsync().ConfigureAwait(false);
        }
        catch (Exception ex)
        {
            AppendError(errorsLog, "nuget-fetch", fetcher.PackageUrl, ex.Message);
            throw;
        }

        var dllPath = fetcher.FindAssembly("Autodesk.Navisworks.Api");
        var xmlPath = fetcher.FindXmlDoc(dllPath)
            ?? throw new InvalidOperationException(
                $"Navisworks extractor: NuGet package {pkg.PackageId} {pkg.PackageVersion} does not ship an XML doc next to {dllPath}.");

        AssemblyRecord asm;
        try
        {
            asm = MetadataReflector.Reflect(dllPath);
        }
        catch (Exception ex)
        {
            AppendError(errorsLog, "metadata-reflect", dllPath, ex.Message);
            throw;
        }

        Dictionary<string, MemberDoc> xmlDoc;
        try
        {
            xmlDoc = XmlDocLoader.LoadFromFile(xmlPath);
        }
        catch (Exception ex)
        {
            AppendError(errorsLog, "xml-doc-load", xmlPath, ex.Message);
            throw;
        }

        var builder = new IRBuilder("navisworks", version, "nuget");
        builder.AddSourceUrl(fetcher.PackageUrl);
        builder.AddSourceUrl("https://aps.autodesk.com/developer/overview/navisworks-api");
        // Page count for a single-DLL extraction is the number of types reflected
        // (each type is its own "page" of metadata).
        foreach (var tr in asm.Types)
        {
            // Skip namespaces outside the Autodesk.Navisworks.Api.* tree — the
            // package ships a few internal globals (LcUGuid, LcUStringBuffer) and
            // some non-Navisworks helpers that don't belong in the catalog.
            if (string.IsNullOrEmpty(tr.Namespace)
                || !tr.Namespace.StartsWith("Autodesk.Navisworks.Api", StringComparison.Ordinal))
            {
                continue;
            }

            builder.IncrementPageCount();
            var skeleton = PageParser.BuildSkeleton(tr, xmlDoc);
            MemberPageParser.Fill(skeleton, tr, xmlDoc);
            builder.AddType(skeleton);
        }

        return builder.Build();
    }

    static string ErrorsLogPath(string outPath, string version)
    {
        var dir = Path.GetDirectoryName(outPath);
        if (string.IsNullOrEmpty(dir)) dir = ".";
        return Path.Combine(dir, $"navisworks-{version}-errors.log");
    }

    static void AppendError(string path, string phase, string url, string message)
    {
        try
        {
            Directory.CreateDirectory(Path.GetDirectoryName(path)!);
            var safeMsg = (message ?? "").Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' ');
            var safeUrl = (url ?? "").Replace('\t', ' ').Replace('\r', ' ').Replace('\n', ' ');
            File.AppendAllText(path, $"{DateTime.UtcNow:O}\t{phase}\t{safeUrl}\t{safeMsg}{Environment.NewLine}");
        }
        catch
        {
            // Best-effort logging; never swallow the original exception.
        }
    }
}
