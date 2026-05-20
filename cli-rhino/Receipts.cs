// Receipt envelope used by every aware-rhino verb. Shape is identical to
// cli-tekla's receipts so downstream orchestrators see the same JSON across
// vendors. Compact (no indentation) for transport efficiency.

using System.Text.Json.Nodes;

namespace AwareRhino;

internal static class Receipts
{
    public static JsonObject ExecOk(object? result, string? hostVersion, int? hostPid,
                                    string? rhinoId, string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]            = true,
            ["result"]        = SerializeForReceipt(result),
            ["host"]          = "rhino",
            ["host_version"]  = hostVersion,
            ["host_pid"]      = hostPid,
            ["rhino_id"]      = rhinoId,
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    // exitCode is the rhinocode process exit code when the failure is an infra
    // fault (rhinocode itself errored / never wrote the file). It's null for a
    // script-reported failure ({ok:false} from a script that ran fine), so a
    // caller can tell infra-fault from script-fault: exit_code != null && != 0
    // ⇒ rhinocode/infra problem; exit_code == 0 or null ⇒ the script ran.
    public static JsonObject ExecFail(string error, string stack, string stdoutLog,
                                      int? exitCode = null, string? stderrLog = null)
    {
        return new JsonObject
        {
            ["ok"]            = false,
            ["error"]         = error,
            ["stack"]         = stack,
            ["host"]          = "rhino",
            ["verb"]          = "exec",
            ["exit_code"]     = exitCode,
            ["stdout_log"]    = stdoutLog,
            ["stderr_log"]    = stderrLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ListOk(JsonArray instances)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["instances"]    = instances,
            ["host"]         = "rhino",
            ["verb"]         = "list-instances",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject SendStatusOk(string message, string? hostVersion, int? hostPid)
    {
        return new JsonObject
        {
            ["status"]          = "ok",
            ["host"]            = "rhino",
            ["host_version"]    = hostVersion,
            ["host_pid"]        = hostPid,
            ["host_session_id"] = hostPid is { } pid ? $"rhino-{pid}" : null,
            ["verb"]            = "send-status",
            ["verb_result"]     = new JsonObject { ["message"] = message },
            ["delivered_at"]    = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject GenericFail(string verb, string error, string? stack = null)
    {
        var o = new JsonObject
        {
            ["status"]       = "err",
            ["host"]         = "rhino",
            ["verb"]         = verb,
            ["error"]        = error,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        if (stack is not null) o["stack"] = stack;
        return o;
    }

    // Pre-parsed result (read from the script's result file) is already a
    // JsonNode. For other paths the caller may hand us a raw primitive — keep it.
    static JsonNode? SerializeForReceipt(object? result)
    {
        return result switch
        {
            null         => null,
            JsonNode jn  => jn,
            string s     => JsonValue.Create(s),
            bool b       => JsonValue.Create(b),
            int i        => JsonValue.Create(i),
            long l       => JsonValue.Create(l),
            double d     => JsonValue.Create(d),
            _            => JsonValue.Create(result.ToString()),
        };
    }
}
