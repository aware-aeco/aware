// `send-status` verb: synthesizes a tiny C# snippet that calls
// Rhino.UI.StatusBar.ShowMessage(args["message"]) and dispatches it through
// the same Exec pipeline. Validates the wrapper end-to-end on every call.

using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class SendStatus
{
    public static int Run(RhinocodeClient client, JsonNode? input)
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
            ["verb"]     = "exec",
            ["version"]  = input?["version"]?.GetValue<string>(),
            ["rhino_id"] = input?["rhino_id"]?.GetValue<string>(),
            ["code"]     =
                """
                var msg = args.TryGetValue("message", out var m) ? (string?)m?.ToString() : null;
                if (!string.IsNullOrEmpty(msg))
                {
                    Rhino.UI.StatusBar.ShowMessage(msg);
                }
                return new { delivered = !string.IsNullOrEmpty(msg), message = msg };
                """,
            ["args"]     = new JsonObject { ["message"] = message },
        };

        // Capture the exec receipt and re-shape into a send-status receipt.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = Exec.Run(client, execInput); }
        finally { Console.SetOut(originalOut); }

        var execReceipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        if (execReceipt is null || execReceipt["ok"]?.GetValue<bool>() != true)
        {
            // Forward exec's error envelope but mark verb as send-status.
            var err = execReceipt?["error"]?.GetValue<string>() ?? "exec failed";
            Console.WriteLine(Receipts.GenericFail("send-status", err,
                execReceipt?["stack"]?.GetValue<string>()).ToJsonString());
            return execExit;
        }

        var version = execReceipt["host_version"]?.GetValue<string>();
        var pid     = execReceipt["host_pid"]?.GetValue<int?>();
        Console.WriteLine(Receipts.SendStatusOk(message!, version, pid).ToJsonString());
        return 0;
    }
}
