// `exec` verb: resolves a target SketchUp instance via SketchupClient,
// ships the user's Ruby `code` + `args` over TCP, parses the bridge's
// response, emits an AWARE-shaped receipt.

using System.Text.Json.Nodes;

namespace AwareSketchup.Verbs;

internal static class Exec
{
    public static int Run(JsonNode? input, SketchupClient? clientOverride = null, int timeoutMs = 30_000)
    {
        var code = input?["code"]?.GetValue<string>();
        if (string.IsNullOrEmpty(code))
        {
            Console.WriteLine(Receipts.ExecFail(
                "missing required field: code (Ruby snippet to evaluate against Sketchup.active_model)",
                "", "").ToJsonString());
            return 2;
        }

        var sketchupId = input?["sketchup_id"]?.GetValue<string>();
        var version    = input?["version"]?.GetValue<string>();
        var argsObj    = input?["args"] as JsonObject;

        var client = clientOverride ?? new SketchupClient();
        var inst = client.Resolve(sketchupId, version);
        if (inst is null)
        {
            var all = client.ListInstances();
            string msg = all.Count == 0
                ? "no SketchUp instance found; launch SketchUp and run `aware-sketchup --install-bridge` if you haven't yet"
                : !string.IsNullOrEmpty(sketchupId)
                    ? $"no SketchUp instance with sketchup_id='{sketchupId}'; running: [{string.Join(", ", all.Select(i => i.Pid))}]"
                    : !string.IsNullOrEmpty(version)
                        ? $"no SketchUp instance matches version='{version}'; running: [{string.Join(", ", all.Select(i => i.Version))}]"
                        : $"ambiguous target — {all.Count} SketchUp instances running; pass sketchup_id=<pid> in the request";
            Console.WriteLine(Receipts.ExecFail(msg, "", "").ToJsonString());
            return 1;
        }

        var request = new JsonObject
        {
            ["type"] = "exec",
            ["code"] = code,
            ["args"] = argsObj?.DeepClone() ?? new JsonObject(),
        };

        JsonNode response;
        try { response = client.SendRequest(inst.Port, request, timeoutMs); }
        catch (TimeoutException te)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"timeout talking to SketchUp bridge on port {inst.Port}: {te.Message}",
                "", "", inst.Version, inst.Pid, inst.Pid.ToString()).ToJsonString());
            return 2;
        }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"bridge I/O failed (port {inst.Port}, pid {inst.Pid}): {e.Message}",
                e.StackTrace ?? "", "", inst.Version, inst.Pid, inst.Pid.ToString()).ToJsonString());
            return 2;
        }

        if (response is not JsonObject body)
        {
            Console.WriteLine(Receipts.ExecFail(
                "bridge returned a non-object response",
                "", response.ToString() ?? "", inst.Version, inst.Pid, inst.Pid.ToString()).ToJsonString());
            return 2;
        }

        bool ok = body["ok"]?.GetValue<bool?>() ?? false;
        var stdoutLog = body["stdout_log"]?.GetValue<string>() ?? "";

        if (!ok)
        {
            Console.WriteLine(Receipts.ExecFail(
                body["error"]?.GetValue<string>() ?? "bridge reported failure with no error message",
                body["stack"]?.GetValue<string>() ?? "",
                stdoutLog,
                inst.Version, inst.Pid, inst.Pid.ToString()).ToJsonString());
            return 0;  // exec dispatched cleanly; the *user* code failed → exit 0, ok:false.
        }

        Console.WriteLine(Receipts.ExecOk(
            body["result"],
            inst.Version,
            inst.Pid,
            inst.Pid.ToString(),
            stdoutLog).ToJsonString());
        return 0;
    }
}
