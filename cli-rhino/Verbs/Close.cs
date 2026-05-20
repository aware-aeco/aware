// `close` verb: shut a Rhino 8 instance down cleanly. Default path issues
// `_-Exit _N` through rhinocode (saves nothing, no dialog). Force path
// (`force: true` in stdin JSON) calls Process.Kill — explicit data-loss
// acknowledgement.

using System.Diagnostics;
using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class Close
{
    public static int Run(RhinocodeClient client, JsonNode? input)
    {
        var rhinoId   = input?["rhino_id"]?.GetValue<string>();
        var version   = input?["version"]?.GetValue<string>();
        var pidNum    = input?["pid"]?.GetValue<int?>();
        bool force    = input?["force"]?.GetValue<bool>() ?? false;
        bool save     = input?["save"]?.GetValue<bool>() ?? false;

        if (force)
        {
            // Force path: find process by PID (preferred) or by name+id resolution, then Kill.
            int? targetPid = pidNum;
            if (targetPid is null)
            {
                try
                {
                    var resolved = Exec.ResolveInstance(client, version, rhinoId);
                    targetPid = resolved?.pid;
                }
                catch (Exception e)
                {
                    Console.WriteLine(Receipts.GenericFail("close",
                        $"force close: could not resolve target via rhinocode list: {e.Message}. " +
                        "Pass `pid` in stdin JSON to skip resolution.").ToJsonString());
                    return 2;
                }
            }
            if (targetPid is null)
            {
                Console.WriteLine(Receipts.GenericFail("close",
                    "force close: no target identified (need pid, rhino_id, or version).")
                    .ToJsonString());
                return 1;
            }

            try
            {
                using var p = Process.GetProcessById(targetPid.Value);
                p.Kill(entireProcessTree: true);
                p.WaitForExit(15_000);
            }
            catch (ArgumentException)
            {
                Console.WriteLine(Receipts.GenericFail("close",
                    $"no process with pid {targetPid} found (already exited?).")
                    .ToJsonString());
                return 1;
            }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.GenericFail("close",
                    $"Process.Kill failed: {e.Message}").ToJsonString());
                return 2;
            }

            var killReceipt = new JsonObject
            {
                ["status"]       = "ok",
                ["host"]         = "rhino",
                ["host_pid"]     = targetPid,
                ["verb"]         = "close",
                ["verb_result"]  = new JsonObject
                {
                    ["mode"] = "force",
                    ["note"] = "Process.Kill issued; any unsaved model state was lost.",
                },
                ["delivered_at"] = DateTime.UtcNow.ToString("o"),
            };
            Console.WriteLine(killReceipt.ToJsonString());
            return 0;
        }

        // Clean path: issue `_-Exit _N` (or `_-Exit _Y`) through rhinocode.
        // Rhino's command-line `Exit` honours the `_-` script-engine prefix
        // which suppresses the GUI prompt; the trailing `_N` / `_Y` answers
        // the "save changes?" dialog for the currently-active document.
        //
        // Note: rhinocode's `command` subcommand passes the macro as a single
        // string to the running Rhino's RunScript pipeline.
        var saveFlag = save ? "_Y" : "_N";
        var macro    = $"_-Exit {saveFlag}";

        var (exit, stdout, stderr) = RunRhinocodeCommand(client, rhinoId, macro);

        // Rhino's Exit kills the script server before the command returns, so
        // we frequently see a non-zero exit code on rhinocode side — the
        // process IS exiting, the IPC channel just died mid-call. Treat any
        // exit code as "best-effort: command issued" and poll for process
        // exit via PID (resolved before the command if possible).

        int? targetPidAfter = pidNum;
        if (targetPidAfter is null)
        {
            try
            {
                var resolved = Exec.ResolveInstance(client, version, rhinoId);
                targetPidAfter = resolved?.pid;
            }
            catch { /* best-effort: list may now fail because we just killed the script server */ }
        }

        bool exitedCleanly = false;
        if (targetPidAfter is { } pidVal)
        {
            var deadline = DateTime.UtcNow.AddSeconds(30);
            while (DateTime.UtcNow < deadline)
            {
                try
                {
                    using var p = Process.GetProcessById(pidVal);
                    if (p.HasExited) { exitedCleanly = true; break; }
                }
                catch (ArgumentException) { exitedCleanly = true; break; }
                Thread.Sleep(500);
            }
        }

        var receipt = new JsonObject
        {
            ["status"]       = exitedCleanly ? "ok" : (targetPidAfter is null ? "ok" : "partial"),
            ["host"]         = "rhino",
            ["host_pid"]     = targetPidAfter,
            ["verb"]         = "close",
            ["verb_result"]  = new JsonObject
            {
                ["mode"]              = "clean",
                ["save"]              = save,
                ["macro"]             = macro,
                ["rhinocode_exit"]    = exit,
                ["rhinocode_stderr"]  = stderr.Trim(),
                ["exited_cleanly"]    = exitedCleanly,
                ["note"] = exitedCleanly
                    ? "Rhino exited within 30s."
                    : "Exit command issued but process didn't exit within 30s. Pass `force: true` to force-kill (will lose unsaved state).",
            },
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        Console.WriteLine(receipt.ToJsonString());
        return exitedCleanly ? 0 : 1;
    }

    static (int exit, string stdout, string stderr) RunRhinocodeCommand(
        RhinocodeClient client, string? rhinoId, string macro)
    {
        // rhinocode command <macro> — issues a Rhino script command in the
        // active Script Server context.
        var psi = new ProcessStartInfo
        {
            FileName               = client.RhinocodeExePath,
            UseShellExecute        = false,
            RedirectStandardOutput = true,
            RedirectStandardError  = true,
            CreateNoWindow         = true,
            StandardOutputEncoding = System.Text.Encoding.UTF8,
            StandardErrorEncoding  = System.Text.Encoding.UTF8,
        };
        if (!string.IsNullOrEmpty(rhinoId))
        {
            psi.ArgumentList.Add("--rhino");
            psi.ArgumentList.Add(rhinoId!);
        }
        psi.ArgumentList.Add("command");
        psi.ArgumentList.Add(macro);

        try
        {
            using var p = Process.Start(psi)
                ?? throw new InvalidOperationException("Process.Start returned null");
            var stdoutTask = p.StandardOutput.ReadToEndAsync();
            var stderrTask = p.StandardError.ReadToEndAsync();
            p.WaitForExit(15_000);
            return (p.ExitCode, stdoutTask.GetAwaiter().GetResult(), stderrTask.GetAwaiter().GetResult());
        }
        catch (Exception e)
        {
            // The IPC channel often dies mid-call for Exit. Surface but don't fail hard.
            return (-1, "", e.Message);
        }
    }
}
