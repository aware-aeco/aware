// `send-status` verb: synthesizes a Ruby snippet that calls
// Sketchup.set_status_text(msg, SB_PROMPT) and dispatches it through the
// same Exec pipeline. Validates the bridge end-to-end on every call.

using System.Text.Json.Nodes;

namespace AwareSketchup.Verbs;

internal static class SendStatus
{
    public static int Run(JsonNode? input, SketchupClient? clientOverride = null)
    {
        var message = Exec.TryString(input, "message");
        if (string.IsNullOrEmpty(message))
        {
            Console.WriteLine(Receipts.GenericFail("send-status",
                "missing required field: message").ToJsonString());
            return 2;
        }

        // SB_PROMPT = 0x100, the constant for the prompt area of SketchUp's status bar.
        // We pass it as a literal rather than relying on the constant name in case
        // a SketchUp version flips its namespacing.
        var rubyCode =
            "msg = args['message'] || ''\n" +
            "Sketchup.set_status_text(msg, 256)\n" +
            "{ 'delivered' => !msg.empty?, 'message' => msg }";

        var execInput = new JsonObject
        {
            ["verb"]        = "exec",
            ["version"]     = Exec.TryString(input, "version"),
            ["sketchup_id"] = Exec.TryStringOrNumber(input, "sketchup_id"),
            ["code"]        = rubyCode,
            ["args"]        = new JsonObject { ["message"] = message },
        };

        // Capture Exec.Run's stdout so we can re-shape into a send-status receipt.
        using var sw = new StringWriter();
        var originalOut = Console.Out;
        Console.SetOut(sw);
        int execExit;
        try { execExit = Exec.Run(execInput, clientOverride); }
        finally { Console.SetOut(originalOut); }

        var execReceipt = JsonNode.Parse(sw.ToString().Trim()) as JsonObject;
        if (execReceipt is null || execReceipt["ok"]?.GetValue<bool>() != true)
        {
            var err = execReceipt?["error"]?.GetValue<string>() ?? "exec failed";
            var stack = execReceipt?["stack"]?.GetValue<string>();
            Console.WriteLine(Receipts.GenericFail("send-status", err, stack).ToJsonString());
            return execExit == 0 ? 1 : execExit;
        }

        var version = execReceipt["host_version"]?.GetValue<string>();
        var pid     = execReceipt["host_pid"]?.GetValue<int?>();
        Console.WriteLine(Receipts.SendStatusOk(message!, version, pid).ToJsonString());
        return 0;
    }
}
