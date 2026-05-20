// send-status verb: synthesises a tiny C# snippet that pops a transient
// TaskDialog (Revit has no status-bar equivalent) and routes through exec.
// Same validation-of-the-whole-pipeline approach as cli-rhino.

using System.IO;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class SendStatus
{
    public static async Task<int> RunAsync(JsonNode? input)
    {
        var message = input?["message"]?.GetValue<string>();
        if (string.IsNullOrEmpty(message))
        {
            Console.WriteLine(Receipts.GenericFail("send-status",
                "missing required field: message").ToJsonString());
            return 2;
        }

        // Build a synthetic exec request and reuse the Exec pipeline.
        var execInput = new JsonObject
        {
            ["verb"]    = "exec",
            ["version"] = input?["version"]?.GetValue<string>(),
            ["code"]    =
                """
                var msg = args.TryGetValue("message", out var m) ? m?.ToString() : null;
                if (!string.IsNullOrEmpty(msg))
                {
                    Autodesk.Revit.UI.TaskDialog.Show("AWARE", msg);
                }
                return new { delivered = !string.IsNullOrEmpty(msg), message = msg };
                """,
            ["args"]    = new JsonObject { ["message"] = message },
        };

        // Capture exec's stdout, then re-shape into a send-status receipt.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = await Exec.RunAsync(execInput); }
        finally { Console.SetOut(originalOut); }

        var execReceipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        if (execReceipt is null || execReceipt["ok"]?.GetValue<bool>() != true)
        {
            var err = execReceipt?["error"]?.GetValue<string>() ?? "exec failed";
            Console.WriteLine(Receipts.GenericFail("send-status", err,
                execReceipt?["stack"]?.GetValue<string>()).ToJsonString());
            return execExit;
        }

        var version = execReceipt["host_version"]?.GetValue<string>();
        int? pid = null;
        if (execReceipt["host_pid"] is JsonNode pidNode && pidNode.GetValueKind() == System.Text.Json.JsonValueKind.Number)
            pid = pidNode.GetValue<int>();
        Console.WriteLine(Receipts.SendStatusOk(message!, version, pid).ToJsonString());
        return 0;
    }
}
