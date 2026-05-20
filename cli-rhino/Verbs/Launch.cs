// `launch` verb: spawn a fresh Rhino 8 process. Optionally opens a model and
// auto-starts the Script Server so subsequent `exec` calls work without manual
// setup. Does NOT wait for Script Server readiness — caller polls
// `list-instances` to confirm.

using System.Diagnostics;
using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class Launch
{
    public static int Run(JsonNode? input)
    {
        string? modelPath        = input?["model_path"]?.GetValue<string>();
        bool startScriptServer   = input?["start_script_server"]?.GetValue<bool>() ?? true;
        string? rhinoExeOverride = input?["rhino_exe"]?.GetValue<string>();

        var exe = rhinoExeOverride ?? DiscoverRhinoExe();
        if (string.IsNullOrEmpty(exe) || !File.Exists(exe))
        {
            Console.WriteLine(Receipts.GenericFail("launch",
                "Could not find Rhino.exe. Install Rhino 8 (standard path: " +
                @"C:\Program Files\Rhino 8\System\Rhino.exe), or pass `rhino_exe` in stdin JSON.")
                .ToJsonString());
            return 3;
        }

        if (!string.IsNullOrEmpty(modelPath) && !File.Exists(modelPath))
        {
            Console.WriteLine(Receipts.GenericFail("launch",
                $"model_path '{modelPath}' does not exist").ToJsonString());
            return 2;
        }

        // Rhino flags:
        //   /nosplash             — skip splash screen for cleaner spawn
        //   /runscript="<cmd>"    — execute a command after load; we use this
        //                            to auto-start the Script Server so the
        //                            spawned Rhino is immediately usable by
        //                            subsequent `exec` calls
        //   <model_path>          — open this 3dm
        var argList = new List<string> { "/nosplash" };
        if (startScriptServer)
        {
            // Underscore prefix forces English-locale command name regardless
            // of Rhino's UI language. The macro pipes to a no-op so the script
            // engine exits cleanly after StartScriptServer fires.
            argList.Add("/runscript=_StartScriptServer");
        }
        if (!string.IsNullOrEmpty(modelPath))
        {
            argList.Add(modelPath!);
        }

        var psi = new ProcessStartInfo
        {
            FileName        = exe,
            UseShellExecute = true,
            WindowStyle     = ProcessWindowStyle.Normal,
        };
        foreach (var a in argList) psi.ArgumentList.Add(a);

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

        var receipt = new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = "rhino",
            ["host_pid"]     = p.Id,
            ["verb"]         = "launch",
            ["verb_result"]  = new JsonObject
            {
                ["model_path"]          = modelPath ?? "",
                ["script_server"]       = startScriptServer,
                ["rhino_exe"]           = exe,
                ["note"]                = "Rhino is starting; poll `list-instances` until the new PID's rhino_id appears to confirm Script Server is ready (typically 5-15s).",
            },
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
        return 0;
    }

    internal static string? DiscoverRhinoExe()
    {
        var stdPath = @"C:\Program Files\Rhino 8\System\Rhino.exe";
        if (File.Exists(stdPath)) return stdPath;

        // Registry fallback (Rhino 8 writes its install path under HKLM\Software\McNeel\Rhinoceros\8.0\Install).
        try
        {
            using var key = Microsoft.Win32.Registry.LocalMachine.OpenSubKey(
                @"SOFTWARE\McNeel\Rhinoceros\8.0\Install");
            var installPath = key?.GetValue("Path") as string;
            if (!string.IsNullOrEmpty(installPath))
            {
                var candidate = Path.Combine(installPath, "Rhino.exe");
                if (File.Exists(candidate)) return candidate;
            }
        }
        catch { /* registry errors → fall through to null */ }

        return null;
    }
}
