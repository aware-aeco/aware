// TeklaSdk extractor — produces a host-coverage IR for the Tekla plugin-authoring
// SDK NuGets that are NOT in developer.tekla.com's scraped Open API reference tree:
//
//   • Tekla.Structures.Plugins.DirectManipulation  — the in-model handle / picking /
//     graphics API (ships an XML doc → rich prose).
//   • TeklaFusion                                   — Trimble's WPF UI framework that
//     Tekla Structures is built on (ships NO XML doc → signatures-only).
//
// Why a separate extractor / agent
// --------------------------------
// The `tekla` extractor (Ingest/Extractors/Tekla) scrapes the published reference
// tree, which contains Tekla.Structures.* but neither DirectManipulation nor
// Fusion. Those live only in their own NuGets. This extractor reflects those
// NuGets directly (System.Reflection.Metadata via MetadataReflector — no assembly
// load, so the net48 DLLs' dependencies need not be resolvable) and merges the two
// into one IR. Source.kind = "nuget".
//
// Scoping (deliberate, not accidental)
// ------------------------------------
//   • DM:     the public plugin surface only — Tekla.Structures.Plugins.DirectManipulation.*
//             minus *.Internal*. The engine/adapter assemblies
//             (Tekla.BIM.DirectManipulation, Tekla.Structures.DirectManipulation,
//             ~950 types) are implementation detail and are NOT fetched.
//   • Fusion: a "dialog-tier" subset (App / UI / CommandHandler / Formatting /
//             Data / Features.Panels / Models / Extensions + the root Fusion
//             namespace). Fusion is ~900 types; the GPU / Testing / RemoteObject /
//             Win32 / Native machinery is irrelevant to plugin-dialog authoring and
//             is excluded. Tunable via the allow-list below.
//
// Licensing: reflecting public API metadata + consuming the Trimble-shipped DM XML
// doc is fine. Decompile-and-summarise of the proprietary Fusion assembly is NOT
// done here (it would violate license-aware-extraction's permissive-only rule), so
// Fusion entries are signatures-only.

using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.TeklaSdk;

public static class Extractor
{
    // display TS version → (DM nupkg version, TeklaFusion nupkg version).
    // DM 2026.0.3 depends on TeklaFusion >= 4.1.36; DM 2025.0.0 pairs with the 4.1.10 line.
    static readonly Dictionary<string, (string Dm, string Fusion)> KnownVersions = new()
    {
        ["2025.0"] = ("2025.0.0", "4.1.10"),
        ["2026.0"] = ("2026.0.3", "4.1.36"),
    };

    const string DmPackageId = "Tekla.Structures.Plugins.DirectManipulation";
    const string DmAssembly = "Tekla.Structures.Plugins.DirectManipulation";
    const string FusionPackageId = "TeklaFusion";
    const string FusionAssembly = "Fusion";

    public static async Task<CoverageIR> Extract(string version, string outPath)
    {
        if (!KnownVersions.TryGetValue(version, out var pkg))
            throw new NotSupportedException(
                $"TeklaSdk extractor: unsupported version '{version}'. Known: {string.Join(", ", KnownVersions.Keys)}.");

        var errorsLog = ErrorsLogPath(outPath, version);
        var builder = new IRBuilder("tekla-plugin-sdk", version, "nuget");
        builder.AddSourceUrl("https://developer.tekla.com/documentation/direct-manipulation-api-plugins");

        // ---- DirectManipulation (public plugin surface, XML-doc-backed) ----
        var dmFetcher = new NuGetFetcher(DmPackageId, pkg.Dm);
        await FetchOrThrow(dmFetcher, errorsLog).ConfigureAwait(false);
        builder.AddSourceUrl(dmFetcher.PackageUrl);

        var dmDll = FindNet48Assembly(dmFetcher, DmAssembly);
        var dmAsm = Reflect(dmDll, errorsLog);
        var dmXmlPath = dmFetcher.FindXmlDoc(dmDll);
        var dmDoc = dmXmlPath is null ? new Dictionary<string, MemberDoc>() : LoadXml(dmXmlPath, errorsLog);

        foreach (var tr in dmAsm.Types)
        {
            if (!IsDmPublic(tr.Namespace)) continue;
            AddType(builder, tr, dmDoc);
        }

        // ---- TeklaFusion (dialog-tier subset, signatures-only) ----
        var fusionFetcher = new NuGetFetcher(FusionPackageId, pkg.Fusion);
        await FetchOrThrow(fusionFetcher, errorsLog).ConfigureAwait(false);
        builder.AddSourceUrl(fusionFetcher.PackageUrl);

        // Pin to the net48 assembly: TeklaFusion ships lib/net48 + lib/net6.0-windows,
        // and a plugin/runtime targets net48. FindAssembly() returns the first recursive
        // match in filesystem order (nondeterministic across TFMs), so select net48
        // explicitly to keep the reflected surface deterministic + plugin-accurate.
        var fusionDll = FindNet48Assembly(fusionFetcher, FusionAssembly);
        var fusionAsm = Reflect(fusionDll, errorsLog);
        // Fusion ships no XML doc — empty dictionary → generated signature-only summaries.
        var fusionDoc = new Dictionary<string, MemberDoc>();

        foreach (var tr in fusionAsm.Types)
        {
            if (!IsFusionDialogTier(tr)) continue;
            AddType(builder, tr, fusionDoc);
        }

        return builder.Build();
    }

    static void AddType(IRBuilder builder, TypeRecord tr, Dictionary<string, MemberDoc> xmlDoc)
    {
        builder.IncrementPageCount();
        var skeleton = PageParser.BuildSkeleton(tr, xmlDoc);
        MemberPageParser.Fill(skeleton, tr, xmlDoc);
        builder.AddType(skeleton);
    }

    static bool IsDmPublic(string ns) =>
        !string.IsNullOrEmpty(ns)
        && ns.StartsWith("Tekla.Structures.Plugins.DirectManipulation", StringComparison.Ordinal)
        && !ns.Contains(".Internal", StringComparison.Ordinal);

    // Dialog-tier Fusion allow-list. Tune here if the forge needs deeper coverage.
    static readonly string[] FusionDialogPrefixes =
    {
        "Fusion.App", "Fusion.UI", "Fusion.CommandHandler", "Fusion.Formatting",
        "Fusion.Data", "Fusion.Features.Panels", "Fusion.Models", "Fusion.Extensions",
    };

    // Non-dialog infrastructure that lives directly in the root `Fusion` namespace
    // (auth / remoting / web). The root namespace also holds the core authoring types
    // (App, ViewModel, CommandHandler) and value/formatting types we DO want, so we
    // keep the root namespace but deny these infra type-name markers.
    static readonly string[] FusionRootInfraMarkers =
    {
        "Authentication", "AccessControl", "RemoteObject", "WebView", "Credential",
    };

    static bool IsFusionDialogTier(TypeRecord tr)
    {
        var ns = tr.Namespace;
        if (string.IsNullOrEmpty(ns)) return false;
        if (ns.Contains(".Internal", StringComparison.Ordinal)
            || ns.Contains(".Native", StringComparison.Ordinal)
            || ns.EndsWith(".Win32", StringComparison.Ordinal)) return false;
        if (ns == "Fusion")
            // core authoring + value/formatting types, minus root infra (auth/remoting/web).
            return !FusionRootInfraMarkers.Any(m => tr.Name.Contains(m, StringComparison.Ordinal));
        foreach (var p in FusionDialogPrefixes)
            if (ns == p || ns.StartsWith(p + ".", StringComparison.Ordinal)) return true;
        return false;
    }

    // Prefer the lib/net48 assembly (the plugin/runtime TFM) for deterministic,
    // plugin-accurate reflection; fall back to the first match if net48 is absent.
    static string FindNet48Assembly(NuGetFetcher fetcher, string assemblyName)
    {
        var net48 = Path.Combine(fetcher.CacheDir, "lib", "net48", assemblyName + ".dll");
        return File.Exists(net48) ? net48 : fetcher.FindAssembly(assemblyName);
    }

    static async Task FetchOrThrow(NuGetFetcher fetcher, string errorsLog)
    {
        try { await fetcher.FetchAsync().ConfigureAwait(false); }
        catch (Exception ex) { AppendError(errorsLog, "nuget-fetch", fetcher.PackageUrl, ex.Message); throw; }
    }

    static AssemblyRecord Reflect(string dllPath, string errorsLog)
    {
        try { return MetadataReflector.Reflect(dllPath); }
        catch (Exception ex) { AppendError(errorsLog, "metadata-reflect", dllPath, ex.Message); throw; }
    }

    static Dictionary<string, MemberDoc> LoadXml(string xmlPath, string errorsLog)
    {
        try { return XmlDocLoader.LoadFromFile(xmlPath); }
        catch (Exception ex) { AppendError(errorsLog, "xml-doc-load", xmlPath, ex.Message); throw; }
    }

    static string ErrorsLogPath(string outPath, string version)
    {
        var dir = Path.GetDirectoryName(outPath);
        if (string.IsNullOrEmpty(dir)) dir = ".";
        return Path.Combine(dir, $"tekla-plugin-sdk-{version}-errors.log");
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
        catch { /* best-effort */ }
    }
}
