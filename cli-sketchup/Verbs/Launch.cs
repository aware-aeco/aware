// `launch` verb: spawn a fresh SketchUp process. Optionally opens a model.
// The bridge auto-loads (it's a .rb in Plugins/), so the caller can poll
// `list-instances` to detect when the new PID is ready.
//
// Does NOT wait for bridge readiness — the caller is expected to poll.

using System.Diagnostics;
using System.Text.Json.Nodes;

namespace AwareSketchup.Verbs;

internal static class Launch
{
    public static int Run(JsonNode? input)
    {
        int targetYear           = TryInt(input, "target_year") ?? 2026;
        string? modelPath        = Exec.TryString(input, "model_path");
        string? sketchupExeOverride = Exec.TryString(input, "sketchup_exe");

        var exe = sketchupExeOverride ?? DiscoverSketchupExe(targetYear);
        if (string.IsNullOrEmpty(exe) || !File.Exists(exe))
        {
            Console.WriteLine(Receipts.GenericFail("launch",
                $"Could not find SketchUp.exe for year {targetYear} " +
                $"(looked at standard paths under 'C:\\Program Files\\SketchUp\\SketchUp {targetYear}\\...'); " +
                "pass `sketchup_exe` in stdin JSON to override.").ToJsonString());
            return 3;
        }

        if (!string.IsNullOrEmpty(modelPath) && !File.Exists(modelPath))
        {
            Console.WriteLine(Receipts.GenericFail("launch",
                $"model_path '{modelPath}' does not exist").ToJsonString());
            return 2;
        }

        var psi = new ProcessStartInfo
        {
            FileName        = exe,
            UseShellExecute = true,
            WindowStyle     = ProcessWindowStyle.Normal,
        };
        if (!string.IsNullOrEmpty(modelPath))
        {
            // SketchUp accepts a single positional arg = path to .skp.
            psi.ArgumentList.Add(modelPath!);
        }

        Process? p;
        try { p = Process.Start(psi); }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.GenericFail("launch",
                $"Process.Start failed: {e.Message}").ToJsonString());
            return 2;
        }
        if (p is null)
        {
            Console.WriteLine(Receipts.GenericFail("launch",
                "Process.Start returned null").ToJsonString());
            return 2;
        }

        // Note: `p.Id` may be the parent process PID if SketchUp's installer
        // wraps it in a launcher. The reliable PID is whichever one writes
        // a fresh discovery file. We report Process.Start's PID for the
        // caller's information; instruct them to poll list-instances.
        var receipt = Receipts.LaunchOk(p.Id, new JsonObject
        {
            ["target_year"]   = targetYear,
            ["model_path"]    = modelPath ?? "",
            ["sketchup_exe"]  = exe,
            ["note"]          = "SketchUp is starting; the AWARE bridge auto-loads from Plugins/. " +
                                "Poll `list-instances` until a new PID appears with a registered bridge " +
                                "(typically 8-15s for a cold SketchUp boot).",
        });
        Console.WriteLine(receipt.ToJsonString());
        return 0;
    }

    static int? TryInt(JsonNode? input, string key)
    {
        var node = input?[key];
        if (node is null) return null;
        try { return node.GetValue<int>(); }
        catch { return null; }
    }

    internal static string? DiscoverSketchupExe(int targetYear)
    {
        var programFiles = Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles);
        // SketchUp 2022+ ships with a nested SketchUp\SketchUp.exe layout.
        // Older versions sat directly in SketchUp\SketchUp <year>\SketchUp.exe.
        // Try both.
        var candidates = new[]
        {
            Path.Combine(programFiles, "SketchUp", $"SketchUp {targetYear}", "SketchUp", "SketchUp.exe"),
            Path.Combine(programFiles, "SketchUp", $"SketchUp {targetYear}", "SketchUp.exe"),
        };
        foreach (var c in candidates)
            if (File.Exists(c)) return c;

        // Registry fallback (HKLM\SOFTWARE\SketchUp\SketchUp <year>\InstallLocation).
        try
        {
            using var key = Microsoft.Win32.Registry.LocalMachine.OpenSubKey(
                $@"SOFTWARE\SketchUp\SketchUp {targetYear}");
            var installPath = key?.GetValue("InstallLocation") as string;
            if (!string.IsNullOrEmpty(installPath))
            {
                var nested = Path.Combine(installPath, "SketchUp", "SketchUp.exe");
                if (File.Exists(nested)) return nested;
                var flat = Path.Combine(installPath, "SketchUp.exe");
                if (File.Exists(flat)) return flat;
            }
        }
        catch { /* registry errors → null */ }

        return null;
    }
}
