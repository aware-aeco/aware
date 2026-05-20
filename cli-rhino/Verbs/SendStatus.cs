// `send-status` verb: synthesizes a tiny Python snippet that calls
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
                msg = args.get("message")
                if msg:
                    Rhino.UI.StatusBar.ShowMessage(str(msg))
                return {"delivered": bool(msg), "message": msg}
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
        // Read `ok` tolerantly — a non-bool (or missing) field must not throw a
        // hard cast into the top-level handler. Anything other than a true bool
        // is treated as failure.
        var execOk = execReceipt?["ok"] is JsonValue okv && okv.TryGetValue<bool>(out var okb) && okb;
        if (execReceipt is null || !execOk)
        {
            // Forward exec's error envelope but mark verb as send-status.
            var err = AsString(execReceipt?["error"]) ?? "exec failed";
            Console.WriteLine(Receipts.GenericFail("send-status", err,
                AsString(execReceipt?["stack"])).ToJsonString());
            return execExit;
        }

        var version = AsString(execReceipt["host_version"]);
        var pid     = execReceipt["host_pid"] is JsonValue pv && pv.TryGetValue<int>(out var pi) ? pi : (int?)null;
        Console.WriteLine(Receipts.SendStatusOk(message!, version, pid).ToJsonString());
        return 0;
    }

    // Reads a node as a string only when it actually is one (null otherwise).
    static string? AsString(JsonNode? node)
        => node is JsonValue jv && jv.TryGetValue<string>(out var s) ? s : null;
}
