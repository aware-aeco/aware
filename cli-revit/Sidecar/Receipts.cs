// Receipt envelope used by every aware-revit verb. Shape mirrors cli-rhino:
// {ok, result, host, host_pid, host_version, verb, stdout_log, delivered_at}
// for exec; analogous shapes for the other verbs. Compact (no indentation)
// for transport efficiency.

using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar;

internal static class Receipts
{
    /// <param name="warnings">What Revit objected to while committing. An unattended
    /// exec suppresses Revit's failure dialog, so without carrying these the objection
    /// reaches nobody (#337). Omitted from the receipt when Revit raised nothing.</param>
    public static JsonObject ExecOk(JsonNode? result, string? hostVersion, int? hostPid,
                                    string stdoutLog, IReadOnlyList<string>? warnings = null)
    {
        var receipt = new JsonObject
        {
            ["ok"]            = true,
            ["result"]        = result,
            ["host"]          = "revit",
            ["host_version"]  = hostVersion,
            ["host_pid"]      = hostPid,
            ["verb"]          = "exec",
            ["stdout_log"]    = stdoutLog,
            ["delivered_at"]  = DateTime.UtcNow.ToString("o"),
        };
        if (warnings is { Count: > 0 })
        {
            var array = new JsonArray();
            foreach (var warning in warnings) array.Add(warning);
            receipt["warnings"] = array;
        }
        return receipt;
    }

    public static JsonObject ExecFail(string error, string stack, string stdoutLog)
    {
        return new JsonObject
        {
            ["ok"]           = false,
            ["error"]        = error,
            ["stack"]        = stack,
            ["host"]         = "revit",
            ["verb"]         = "exec",
            ["stdout_log"]   = stdoutLog,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject ListOk(JsonArray instances)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["instances"]    = instances,
            ["host"]         = "revit",
            ["verb"]         = "list-instances",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject SendStatusOk(string message, string? hostVersion, int? hostPid)
    {
        return new JsonObject
        {
            ["status"]           = "ok",
            ["host"]             = "revit",
            ["host_version"]     = hostVersion,
            ["host_pid"]         = hostPid,
            ["host_session_id"]  = hostPid is { } pid ? $"revit-{pid}" : null,
            ["verb"]             = "send-status",
            ["verb_result"]      = new JsonObject { ["message"] = message },
            ["delivered_at"]     = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject LaunchOk(int hostPid, string hostVersion, JsonObject verbResult)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = "revit",
            ["host_version"] = hostVersion,
            ["host_pid"]     = hostPid,
            ["verb"]         = "launch",
            ["verb_result"]  = verbResult,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject CloseOk(int hostPid, string hostVersion, string mode)
    {
        return new JsonObject
        {
            ["status"]       = "ok",
            ["host"]         = "revit",
            ["host_version"] = hostVersion,
            ["host_pid"]     = hostPid,
            ["mode"]         = mode,
            ["verb"]         = "close",
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
    }

    public static JsonObject GenericFail(string verb, string error, string? stack = null)
    {
        var o = new JsonObject
        {
            ["status"]       = "err",
            ["host"]         = "revit",
            ["verb"]         = verb,
            ["error"]        = error,
            ["delivered_at"] = DateTime.UtcNow.ToString("o"),
        };
        if (stack is not null) o["stack"] = stack;
        return o;
    }
}
