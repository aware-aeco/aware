// close verb: clean shutdown via the add-in (PostCommand(Close) +
// Application.Exit()) followed by a Process.HasExited wait. Force-kill is
// gated behind explicit `force: true`. Mirrors cli-tekla's data-loss
// acknowledgement pattern.

using System.Diagnostics;
using System.IO;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class Close
{
    public static async Task<int> RunAsync(JsonNode? input)
    {
        var version = input?["version"]?.GetValue<string>();
        int? hostPid = null;
        if (input?["host_pid"] is JsonNode pidNode && pidNode.GetValueKind() == System.Text.Json.JsonValueKind.Number)
            hostPid = pidNode.GetValue<int>();
        var force = input?["force"]?.GetValue<bool>() ?? false;

        var all = RevitProcessFinder.Enumerate();
        var matches = RevitProcessFinder.Filter(all, version, hostPid);
        if (matches.Count == 0)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                all.Count == 0 ? "no Revit instance running" : "no matching target").ToJsonString());
            return 1;
        }
        if (matches.Count > 1)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                $"ambiguous target ({matches.Count} matches). Pass version or host_pid.").ToJsonString());
            return 4;
        }
        var target = matches[0];

        if (force)
        {
            try { Process.GetProcessById(target.Pid).Kill(); }
            catch (Exception e)
            {
                Console.WriteLine(Receipts.GenericFail("close", $"Process.Kill failed: {e.Message}").ToJsonString());
                return 2;
            }
            await WaitForExitAsync(target.Pid, TimeSpan.FromSeconds(15));
            Console.WriteLine(Receipts.CloseOk(target.Pid, target.Version, "force").ToJsonString());
            return 0;
        }

        // Clean path: ask the add-in to close docs + exit via an exec request.
        var execInput = new JsonObject
        {
            ["verb"]    = "exec",
            ["host_pid"] = target.Pid,
            ["code"]    =
                """
                // Close active docs then exit Revit. PostCommand schedules on the UI
                // thread; the exec call returns immediately after the post.
                var cmdId = Autodesk.Revit.UI.RevitCommandId
                    .LookupPostableCommandId(Autodesk.Revit.UI.PostableCommand.Close);
                if (cmdId != null) uiapp.PostCommand(cmdId);
                uiapp.Application.Exit();
                return new { exit_posted = true };
                """,
            ["args"]    = new JsonObject(),
        };

        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        try { await Exec.RunAsync(execInput); }
        finally { Console.SetOut(originalOut); }

        // Exec may fail because Application.Exit() races against the pipe write.
        // Treat any response (or pipe-closed mid-write) as success-pending and
        // verify via Process.HasExited.
        var exited = await WaitForExitAsync(target.Pid, TimeSpan.FromSeconds(30));
        if (!exited)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                "Revit did not exit within 30s of Application.Exit(). " +
                "Re-issue with force=true if data loss is acceptable.").ToJsonString());
            return 2;
        }
        Console.WriteLine(Receipts.CloseOk(target.Pid, target.Version, "clean").ToJsonString());
        return 0;
    }

    static async Task<bool> WaitForExitAsync(int pid, TimeSpan timeout)
    {
        var deadline = DateTime.UtcNow + timeout;
        while (DateTime.UtcNow < deadline)
        {
            try
            {
                var p = Process.GetProcessById(pid);
                if (p.HasExited) return true;
            }
            catch (ArgumentException) { return true; /* process gone */ }
            await Task.Delay(500);
        }
        return false;
    }
}
