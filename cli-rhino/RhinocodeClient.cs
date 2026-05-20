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

        // 2. Standard install locations. McNeel ships the CLI as RhinoCode.exe;
        // older builds used rhinocode.exe — probe both.
        foreach (var stdPath in new[]
        {
            @"C:\Program Files\Rhino 8\System\RhinoCode.exe",
            @"C:\Program Files\Rhino 8\System\rhinocode.exe",
        })
        {
            if (fileExists(stdPath))
            {
                RhinocodeExePath = stdPath;
                return;
            }
        }

        // 3. PATH lookup.
        var found = pathLookup("rhinocode.exe") ?? pathLookup("rhinocode") ?? pathLookup("RhinoCode.exe");
        if (!string.IsNullOrEmpty(found) && fileExists(found))
        {
            RhinocodeExePath = found;
            return;
        }

        throw new FileNotFoundException(
            "Could not find rhinocode/RhinoCode.exe. Install Rhino 8.11+ (standard path: " +
            @"C:\Program Files\Rhino 8\System\RhinoCode.exe), add it to PATH, or set RHINOCODE_EXE env var.");
    }

    public (int exitCode, string stdout, string stderr) RunListJson()
    {
        return Run(new[] { "list", "--json" }, argsJson: null);
    }

    // Runs a Python script via `rhinocode script <path.py>`. resultPath is the
    // file the script writes its result to; we set it as an env var for
    // best-effort compatibility, but the canonical channel is the path baked
    // into the script source by ScriptWrapper (rhinocode runs the script inside
    // the live Rhino process, which does NOT inherit this child's environment).
    public (int exitCode, string stdout, string stderr) RunScript(
        string scriptPath, string? rhinoId, string argsJson, string? resultPath = null)
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
        return Run(args.ToArray(), argsJson, resultPath);
    }

    (int exitCode, string stdout, string stderr) Run(string[] args, string? argsJson, string? resultPath = null)
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
        // Set AWARE_RESULT_PATH so the test stub (which DOES run as this child)
        // knows where to write its canned result. Real rhinocode ignores it
        // because the script runs in a different process — that path is baked
        // into the script source instead — but setting it here is harmless and
        // lets the stub faithfully simulate the file-write behaviour.
        if (!string.IsNullOrEmpty(resultPath)) psi.Environment["AWARE_RESULT_PATH"] = resultPath;
        // Pass through AWARE_STUB_DUMP_DIR if set — test fixtures use this to
        // give the stub an unambiguous write location independent of %TEMP%.
        var dumpDir = Environment.GetEnvironmentVariable("AWARE_STUB_DUMP_DIR");
        if (!string.IsNullOrEmpty(dumpDir)) psi.Environment["AWARE_STUB_DUMP_DIR"] = dumpDir;

        using var p = Process.Start(psi)
            ?? throw new InvalidOperationException($"Process.Start returned null for {RhinocodeExePath}");
        var stdoutTask = p.StandardOutput.ReadToEndAsync();
        var stderrTask = p.StandardError.ReadToEndAsync();

        // Defense-in-depth: `rhinocode` itself returns fast (fire-and-forget for
        // `script`, bounded output for `list`), so a process that hasn't exited
        // within a generous bound is hung. Kill it and report a non-zero exit so
        // the caller surfaces an infra fault instead of blocking forever.
        // Override the bound with AWARE_RHINOCODE_WAIT_MS.
        var waitMs = ParseWaitMs(Environment.GetEnvironmentVariable("AWARE_RHINOCODE_WAIT_MS"), 30_000);
        if (!p.WaitForExit(waitMs))
        {
            try { p.Kill(entireProcessTree: true); } catch { /* best-effort */ }
            var partialErr = SafeResult(stderrTask);
            return (124, SafeResult(stdoutTask),
                (string.IsNullOrEmpty(partialErr) ? "" : partialErr + "\n")
                + $"rhinocode did not exit within {waitMs}ms; killed");
        }
        // WaitForExit(ms) can return before async stdout/stderr flush — the
        // parameterless overload guarantees the streams are drained.
        p.WaitForExit();
        return (p.ExitCode, SafeResult(stdoutTask), SafeResult(stderrTask));
    }

    static int ParseWaitMs(string? raw, int fallback)
    {
        if (!string.IsNullOrWhiteSpace(raw) && int.TryParse(raw, out var v) && v > 0) return v;
        return fallback;
    }

    // Reads an already-running read task without throwing; a killed process can
    // fault the stream read, which shouldn't mask the real "process hung" signal.
    static string SafeResult(Task<string> t)
    {
        try { return t.GetAwaiter().GetResult(); } catch { return ""; }
    }

    // Naive PATH lookup using `where` shipped with Windows. Synchronous read
    // is safe here because `where`'s output is bounded (one path per match,
    // typically 1-2 matches). Run() above uses the async pattern for the
    // larger rhinocode payloads.
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
