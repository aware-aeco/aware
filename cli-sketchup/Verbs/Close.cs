// `close` verb: shut SketchUp down. Two modes:
//   - clean (default): ship a `Sketchup.active_model.save` via exec, then
//     CloseMainWindow + wait up to 10s, escalate to Kill on timeout
//   - force (force=true): Process.Kill() immediately (data loss possible)
//
// Filter: pid (exact), version (startswith), all (every running instance).

using System.Diagnostics;
using System.Text.Json.Nodes;

namespace AwareSketchup.Verbs;

internal static class Close
{
    public static int Run(JsonNode? input, SketchupClient? clientOverride = null)
    {
        bool force          = input?["force"]?.GetValue<bool?>() ?? false;
        bool all            = input?["all"]?.GetValue<bool?>() ?? false;
        string? sketchupId  = input?["sketchup_id"]?.GetValue<string>()
                              ?? input?["pid"]?.GetValue<int?>()?.ToString();
        string? version     = input?["version"]?.GetValue<string>();

        var client = clientOverride ?? new SketchupClient();
        var instances = client.ListInstances();

        if (instances.Count == 0)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                "no SketchUp instance running (nothing to close)").ToJsonString());
            return 1;
        }

        List<SketchupInstance> targets;
        if (!string.IsNullOrEmpty(sketchupId))
        {
            if (!int.TryParse(sketchupId, out var pidInt))
            {
                Console.WriteLine(Receipts.GenericFail("close",
                    $"sketchup_id '{sketchupId}' is not a valid PID").ToJsonString());
                return 2;
            }
            targets = instances.Where(i => i.Pid == pidInt).ToList();
        }
        else if (!string.IsNullOrEmpty(version))
        {
            targets = instances.Where(i => i.Version.StartsWith(version!, StringComparison.Ordinal)).ToList();
        }
        else if (all)
        {
            targets = instances;
        }
        else if (instances.Count == 1)
        {
            targets = instances;
        }
        else
        {
            Console.WriteLine(Receipts.GenericFail("close",
                $"ambiguous target ({instances.Count} SketchUp instances running). " +
                "Pass `sketchup_id`, `version`, or `all: true` in stdin JSON.").ToJsonString());
            return 4;
        }

        if (targets.Count == 0)
        {
            Console.WriteLine(Receipts.GenericFail("close",
                "no SketchUp instance matched the supplied filter").ToJsonString());
            return 1;
        }

        var results = new JsonArray();
        foreach (var t in targets)
        {
            try
            {
                CloseOne(t, force, client);
                results.Add(new JsonObject
                {
                    ["status"]       = "ok",
                    ["host_pid"]     = t.Pid,
                    ["host_version"] = t.Version,
                    ["mode"]         = force ? "force" : "clean",
                });
            }
            catch (Exception e)
            {
                results.Add(new JsonObject
                {
                    ["status"]       = "err",
                    ["host_pid"]     = t.Pid,
                    ["host_version"] = t.Version,
                    ["error"]        = e.Message,
                });
            }
        }

        Console.WriteLine(Receipts.CloseOk(results).ToJsonString());
        return 0;
    }

    static void CloseOne(SketchupInstance inst, bool force, SketchupClient client)
    {
        Process p;
        try { p = Process.GetProcessById(inst.Pid); }
        catch (ArgumentException) { return;  /* already gone */ }

        if (force)
        {
            p.Kill();
            p.WaitForExit(5_000);
            return;
        }

        // Clean: ask the bridge to save first.
        try
        {
            var saveRequest = new JsonObject
            {
                ["type"] = "exec",
                ["code"] = "m = Sketchup.active_model; m.path.empty? ? 'unsaved' : (m.save; 'saved')",
                ["args"] = new JsonObject(),
            };
            client.SendRequest(inst.Port, saveRequest, timeoutMs: 15_000);
        }
        catch
        {
            // Best-effort save; fall through to window close.
        }

        // Politely ask the main window to close.
        bool closed = false;
        try { closed = p.CloseMainWindow(); } catch { }

        if (!closed || !p.WaitForExit(10_000))
        {
            // Escalate.
            try { p.Kill(); } catch { }
            p.WaitForExit(5_000);
        }
    }
}
