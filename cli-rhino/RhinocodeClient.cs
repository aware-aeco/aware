// Discovers and shells out to McNeel's rhinocode CLI. Single place that knows
// about the binary's location and command-line shape. Tests inject overrides
// via the constructor to stub the binary or fake PATH lookups.

using System.Diagnostics;
using System.Text;

namespace AwareRhino;

internal sealed class RhinocodeClient
{
    public string RhinocodeExePath { get; }

    public RhinocodeClient(
        string? rhinocodeExeOverride = null,
        Func<string, bool>? fileExists = null,
        Func<string, string?>? pathLookup = null)
    {
        fileExists ??= File.Exists;
        pathLookup ??= WhereExe;

        // 1. Explicit override (RHINOCODE_EXE env var, or constructor arg from a test).
        var envOverride = rhinocodeExeOverride
            ?? Environment.GetEnvironmentVariable("RHINOCODE_EXE");
        if (!string.IsNullOrEmpty(envOverride) && fileExists(envOverride))
        {
            RhinocodeExePath = envOverride;
            return;
        }

        // 2. Standard install location.
        var stdPath = @"C:\Program Files\Rhino 8\System\rhinocode.exe";
        if (fileExists(stdPath))
        {
            RhinocodeExePath = stdPath;
            return;
        }

        // 3. PATH lookup.
        var found = pathLookup("rhinocode.exe") ?? pathLookup("rhinocode");
        if (!string.IsNullOrEmpty(found) && fileExists(found))
        {
            RhinocodeExePath = found;
            return;
        }

        throw new FileNotFoundException(
            "Could not find rhinocode.exe. Install Rhino 8.11+ (standard path: " +
            @"C:\Program Files\Rhino 8\System\rhinocode.exe), add it to PATH, or set RHINOCODE_EXE env var.");
    }

    public (int exitCode, string stdout, string stderr) RunListJson()
    {
        return Run(new[] { "list", "--json" }, argsJson: null);
    }

    public (int exitCode, string stdout, string stderr) RunScript(
        string scriptPath, string? rhinoId, string argsJson)
    {
        var args = new List<string>();
        // --rhino must come BEFORE the subcommand (it's a global flag).
        if (!string.IsNullOrEmpty(rhinoId))
        {
            args.Add("--rhino");
            args.Add(rhinoId!);
        }
        args.Add("script");
        args.Add(scriptPath);
        return Run(args.ToArray(), argsJson);
    }

    (int exitCode, string stdout, string stderr) Run(string[] args, string? argsJson)
    {
        var psi = new ProcessStartInfo
        {
            FileName               = RhinocodeExePath,
            UseShellExecute        = false,
            RedirectStandardOutput = true,
            RedirectStandardError  = true,
            CreateNoWindow         = true,
            StandardOutputEncoding = Encoding.UTF8,
            StandardErrorEncoding  = Encoding.UTF8,
        };
        foreach (var a in args) psi.ArgumentList.Add(a);

        // Inject AWARE_ARGS_JSON for the user's script to read. Always set (even
        // for `list`) — harmless when unused.
        psi.Environment["AWARE_ARGS_JSON"] = argsJson ?? "{}";
        // Pass through AWARE_STUB_DUMP_DIR if set — test fixtures use this to
        // give the stub an unambiguous write location independent of %TEMP%.
        var dumpDir = Environment.GetEnvironmentVariable("AWARE_STUB_DUMP_DIR");
        if (!string.IsNullOrEmpty(dumpDir)) psi.Environment["AWARE_STUB_DUMP_DIR"] = dumpDir;

        using var p = Process.Start(psi)
            ?? throw new InvalidOperationException($"Process.Start returned null for {RhinocodeExePath}");
        var stdoutTask = p.StandardOutput.ReadToEndAsync();
        var stderrTask = p.StandardError.ReadToEndAsync();
        p.WaitForExit();
        return (p.ExitCode, stdoutTask.GetAwaiter().GetResult(), stderrTask.GetAwaiter().GetResult());
    }

    // Naive PATH lookup using `where` shipped with Windows.
    static string? WhereExe(string name)
    {
        try
        {
            var psi = new ProcessStartInfo
            {
                FileName               = "where",
                Arguments              = name,
                UseShellExecute        = false,
                RedirectStandardOutput = true,
                RedirectStandardError  = true,
                CreateNoWindow         = true,
            };
            using var p = Process.Start(psi);
            if (p is null) return null;
            var output = p.StandardOutput.ReadToEnd().Trim();
            p.WaitForExit();
            if (p.ExitCode != 0) return null;
            // `where` prints one path per line; first line wins.
            return output.Split('\n')[0].Trim();
        }
        catch
        {
            return null;
        }
    }
}
