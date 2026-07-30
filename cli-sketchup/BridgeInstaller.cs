// BridgeInstaller — copies the SketchUp Ruby bridge into the user's
// %APPDATA%\SketchUp\SketchUp <year>\SketchUp\Plugins\ folder so SketchUp
// auto-loads it on next launch.
//
// Layout deposited (matches the cli-sketchup\BridgeAssets\ shape):
//   <Plugins>\aware_sketchup_bridge.rb              (loader)
//   <Plugins>\aware_sketchup_bridge\core.rb         (the actual bridge)
//
// This is the only file in the sidecar that knows the SketchUp plugin folder
// convention. Tests inject an override location.

using System.Text.Json.Nodes;

namespace AwareSketchup;

internal static class BridgeInstaller
{
    public const string LoaderFileName = "aware_sketchup_bridge.rb";

    /// <summary>SketchUp year assumed when no --target-year is given.</summary>
    public const int DefaultTargetYear = 2026;
    public const string SupportFolder  = "aware_sketchup_bridge";

    /// <summary>
    /// Public sidecar entry point: aware-sketchup --install-bridge [--target-year YYYY] [--plugins-dir DIR]
    /// </summary>
    public static int Install(string[] args)
    {
        var (targetYear, pluginsOverride) = ParseArgs(args);
        try
        {
            var assetDir = LocateAssetDir();
            var pluginsDir = pluginsOverride ?? DefaultPluginsDir(targetYear);
            Directory.CreateDirectory(pluginsDir);

            var loaderSrc = Path.Combine(assetDir, LoaderFileName);
            var coreSrcDir = Path.Combine(assetDir, SupportFolder);
            if (!File.Exists(loaderSrc) || !Directory.Exists(coreSrcDir))
            {
                EmitErr("install-bridge",
                    $"bridge assets not found next to aware-sketchup.exe (looked in '{assetDir}')");
                return 3;
            }

            var loaderDst = Path.Combine(pluginsDir, LoaderFileName);
            var coreDstDir = Path.Combine(pluginsDir, SupportFolder);

            File.Copy(loaderSrc, loaderDst, overwrite: true);
            CopyDirectory(coreSrcDir, coreDstDir);

            EmitOk("install-bridge", new JsonObject
            {
                ["target_year"]  = targetYear,
                ["plugins_dir"]  = pluginsDir,
                ["loader_path"]  = loaderDst,
                ["support_dir"]  = coreDstDir,
                ["next_steps"]   = "Restart SketchUp — the bridge auto-loads at startup. Then run `aware-sketchup list-instances` to confirm.",
            });
            return 0;
        }
        catch (Exception e)
        {
            EmitErr("install-bridge", e.Message);
            return 2;
        }
    }

    public static int Uninstall(string[] args)
    {
        var (targetYear, pluginsOverride) = ParseArgs(args);
        try
        {
            var pluginsDir = pluginsOverride ?? DefaultPluginsDir(targetYear);
            var loader = Path.Combine(pluginsDir, LoaderFileName);
            var support = Path.Combine(pluginsDir, SupportFolder);
            bool loaderRemoved = false, supportRemoved = false;
            if (File.Exists(loader)) { File.Delete(loader); loaderRemoved = true; }
            if (Directory.Exists(support)) { Directory.Delete(support, recursive: true); supportRemoved = true; }
            EmitOk("uninstall-bridge", new JsonObject
            {
                ["target_year"]     = targetYear,
                ["plugins_dir"]     = pluginsDir,
                ["loader_removed"]  = loaderRemoved,
                ["support_removed"] = supportRemoved,
            });
            return 0;
        }
        catch (Exception e)
        {
            EmitErr("uninstall-bridge", e.Message);
            return 2;
        }
    }

    public static int Status(string[] args)
    {
        var (targetYear, pluginsOverride) = ParseArgs(args);
        try
        {
            var pluginsDir = pluginsOverride ?? DefaultPluginsDir(targetYear);
            var loader = Path.Combine(pluginsDir, LoaderFileName);
            var support = Path.Combine(pluginsDir, SupportFolder);
            var assetDir = LocateAssetDir();
            var installedVersion = ReadInstalledVersion(loader);
            var packagedVersion  = ReadInstalledVersion(Path.Combine(assetDir, LoaderFileName));
            EmitOk("bridge-status", new JsonObject
            {
                ["target_year"]       = targetYear,
                ["plugins_dir"]       = pluginsDir,
                ["loader_present"]    = File.Exists(loader),
                ["support_present"]   = Directory.Exists(support),
                ["installed_version"] = installedVersion,
                ["packaged_version"]  = packagedVersion,
                ["up_to_date"]        = installedVersion is not null && installedVersion == packagedVersion,
            });
            return 0;
        }
        catch (Exception e)
        {
            EmitErr("bridge-status", e.Message);
            return 2;
        }
    }

    /// <summary>
    /// Locates the BridgeAssets/ directory next to aware-sketchup.exe.
    /// Uses AppContext.BaseDirectory which is safe under single-file publish
    /// (Assembly.Location returns "" under PublishSingleFile=true).
    /// </summary>
    internal static string LocateAssetDir()
    {
        return Path.Combine(AppContext.BaseDirectory, "BridgeAssets");
    }

    /// <summary>
    /// Default Plugins folder per SketchUp year. Uses %APPDATA% so a per-user
    /// install doesn't need admin rights. SketchUp's bundled plugins live
    /// under Program Files, but Trimble's own docs (and the existing
    /// extensions on Pawel's machine) place 3rd-party plugins here.
    /// </summary>
    internal static string DefaultPluginsDir(int targetYear)
    {
        var appdata = Environment.GetEnvironmentVariable("APPDATA")
                      ?? throw new InvalidOperationException("APPDATA env var not set");
        return Path.Combine(appdata, "SketchUp", $"SketchUp {targetYear}", "SketchUp", "Plugins");
    }

    /// <summary>
    /// Parses --target-year and --plugins-dir off the trailing args.
    /// Defaults to 2026 since that's what's on Pawel's machine.
    /// </summary>
    internal static (int targetYear, string? pluginsOverride) ParseArgs(string[] args)
    {
        int year = DefaultTargetYear;
        string? pluginsOverride = null;
        for (int i = 1; i < args.Length; i++)
        {
            if (args[i] == "--target-year" && i + 1 < args.Length)
            {
                if (int.TryParse(args[i + 1], out var y)) year = y;
                i++;
            }
            else if (args[i] == "--plugins-dir" && i + 1 < args.Length)
            {
                pluginsOverride = args[i + 1];
                i++;
            }
        }
        return (year, pluginsOverride);
    }

    /// <summary>
    /// Recursively copies a directory tree, overwriting existing files.
    /// </summary>
    internal static void CopyDirectory(string src, string dst)
    {
        Directory.CreateDirectory(dst);
        foreach (var file in Directory.EnumerateFiles(src))
        {
            var dstFile = Path.Combine(dst, Path.GetFileName(file));
            File.Copy(file, dstFile, overwrite: true);
        }
        foreach (var sub in Directory.EnumerateDirectories(src))
        {
            var dstSub = Path.Combine(dst, Path.GetFileName(sub));
            CopyDirectory(sub, dstSub);
        }
    }

    /// <summary>
    /// BRIDGE_VERSION of the loader shipped beside this exe — what
    /// <c>--install-bridge</c> would put in place. Null when it can't be read.
    /// </summary>
    internal static string? PackagedVersion()
        => ReadInstalledVersion(Path.Combine(LocateAssetDir(), LoaderFileName));

    /// <summary>
    /// BRIDGE_VERSION of the loader actually sitting in SketchUp's Plugins folder —
    /// what the next SketchUp start would load, which is NOT necessarily what this
    /// sidecar ships (upgrading the CLI does not re-run <c>--install-bridge</c>).
    /// Null when the year is unknown or nothing is installed there — better to say
    /// nothing than to compare a 2025 session against the 2026 folder.
    /// </summary>
    internal static string? InstalledVersion(int? targetYear)
        => ReadInstalledVersion(LoaderPathForYear(targetYear) ?? "");

    /// <summary>
    /// Where the loader for a SketchUp year lives by default, or null when the year is
    /// unknown. Callers report this path rather than asserting a bare version, because a
    /// user who installed with <c>--plugins-dir</c> must be able to see we looked elsewhere.
    /// </summary>
    internal static string? LoaderPathForYear(int? targetYear)
    {
        if (targetYear is null) return null;
        try { return Path.Combine(DefaultPluginsDir(targetYear.Value), LoaderFileName); }
        catch { return null; }
    }

    /// <summary>
    /// The SketchUp YEAR a running instance belongs to, from its version string:
    /// SketchUp numbers its majors by year, so 26.1.189 is SketchUp 2026. Null when
    /// the version doesn't parse into a plausible year.
    /// </summary>
    internal static int? PluginYearForHostVersion(string? hostVersion)
    {
        if (string.IsNullOrWhiteSpace(hostVersion)) return null;
        var dot  = hostVersion!.IndexOf('.');
        var head = dot < 0 ? hostVersion : hostVersion[..dot];
        if (!int.TryParse(head, out var major)) return null;
        var year = 2000 + major;
        return year is >= 2015 and <= 2099 ? year : null;
    }

    /// <summary>
    /// Orders two dotted-number bridge versions (-1/0/1), or null when either side
    /// isn't one. Direction matters: recommending <c>--install-bridge</c> when the
    /// INSTALLED bridge is the newer one would overwrite it with an older copy.
    /// </summary>
    internal static int? CompareBridgeVersions(string? a, string? b)
    {
        var pa = ParseVersion(a);
        var pb = ParseVersion(b);
        if (pa is null || pb is null) return null;
        for (int i = 0; i < Math.Max(pa.Length, pb.Length); i++)
        {
            int x = i < pa.Length ? pa[i] : 0;
            int y = i < pb.Length ? pb[i] : 0;
            if (x != y) return x < y ? -1 : 1;
        }
        return 0;
    }

    static int[]? ParseVersion(string? v)
    {
        if (string.IsNullOrWhiteSpace(v)) return null;
        var parts = v!.Split('.');
        var nums = new int[parts.Length];
        for (int i = 0; i < parts.Length; i++)
        {
            if (!int.TryParse(parts[i], out nums[i])) return null;
        }
        return nums;
    }

    /// <summary>
    /// Reads `BRIDGE_VERSION = '...'` out of a loader .rb. Returns null on
    /// any failure (missing file, no version line, parse glitch).
    /// </summary>
    internal static string? ReadInstalledVersion(string loaderPath)
    {
        if (!File.Exists(loaderPath)) return null;
        try
        {
            foreach (var line in File.ReadLines(loaderPath))
            {
                var idx = line.IndexOf("BRIDGE_VERSION", StringComparison.Ordinal);
                if (idx < 0) continue;
                var eq = line.IndexOf('=', idx);
                if (eq < 0) continue;
                var rhs = line[(eq + 1)..].Trim();
                // Strip the leading + trailing quote.
                rhs = rhs.Trim('\'', '"', ' ', ';');
                if (rhs.EndsWith(".freeze"))
                    rhs = rhs[..^".freeze".Length].Trim('\'', '"', ' ', ';', '.');
                return rhs;
            }
        }
        catch { /* fall through */ }
        return null;
    }

    static void EmitOk(string verb, JsonObject body)
    {
        var receipt = new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = Receipts.Host,
            ["verb"]         = verb,
            ["verb_result"]  = body,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
    }

    static void EmitErr(string verb, string error)
    {
        Console.WriteLine(Receipts.GenericFail(verb, error).ToJsonString());
    }
}
