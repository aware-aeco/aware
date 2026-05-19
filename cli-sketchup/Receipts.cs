// Receipt envelope used by every aware-sketchup verb. Shape mirrors cli-tekla
// and cli-rhino so downstream orchestrators see the same JSON across hosts —
// only the `host` field changes ("sketchup" vs "tekla" vs "rhino"). Compact
// (no indentation) for transport efficiency.

using System.Text.Json.Nodes;

namespace AwareSketchup;

internal static class Receipts
{
    public const string Host = "sketchup";

    public static JsonObject ExecOk(JsonNode? result, string? hostVersion, int? hostPid,
                                    string? sketchupId, string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]            = true,
            ["result"]        = result?.DeepClone(),
            ["host"]          = Host,
            ["host_version"]  = hostVersion,
            ["host_pid"]      = hostPid,
            ["sketchup_id"]   = sketchupId,
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ExecFail(string error, string stack, string stdoutLog,
                                      string? hostVersion = null, int? hostPid = null,
                                      string? sketchupId = null)
    {
        return new JsonObject
        {
            ["ok"]            = false,
            ["error"]         = error,
            ["stack"]         = stack,
            ["host"]          = Host,
            ["host_version"]  = hostVersion,
            ["host_pid"]      = hostPid,
            ["sketchup_id"]   = sketchupId,
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ListOk(JsonArray instances)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["instances"]    = instances,
            ["host"]         = Host,
            ["verb"]         = "list-instances",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject SendStatusOk(string message, string? hostVersion, int? hostPid)
    {
        return new JsonObject
        {
            ["status"]          = "ok",
            ["host"]            = Host,
            ["host_version"]    = hostVersion,
            ["host_pid"]        = hostPid,
            ["host_session_id"] = hostPid is { } pid ? $"sketchup-{pid}" : null,
            ["verb"]            = "send-status",
            ["verb_result"]     = new JsonObject { ["message"] = message },
            ["delivered_at"]    = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject LaunchOk(int hostPid, JsonObject verbResult)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = Host,
            ["host_pid"]     = hostPid,
            ["verb"]         = "launch",
            ["verb_result"]  = verbResult,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject CloseOk(JsonArray verbResult)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = Host,
            ["verb"]         = "close",
            ["verb_result"]  = verbResult,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject GenericFail(string verb, string error, string? stack = null)
    {
        var o = new JsonObject
        {
            ["status"]       = "err",
            ["host"]         = Host,
            ["verb"]         = verb,
            ["error"]        = error,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        if (stack is not null) o["stack"] = stack;
        return o;
    }
}
