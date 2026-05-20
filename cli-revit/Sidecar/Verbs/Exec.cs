// exec verb — picks a Revit target, opens a pipe to its AwareRevit.dll add-in,
// ships the request, reshapes the add-in's ExecResponse into the AWARE-shaped
// outer receipt envelope (adds host="revit", verb, delivered_at).

using System.Text.Json;
using System.Text.Json.Nodes;
using AwareRevit.Shared;

namespace AwareRevit.Sidecar.Verbs;

internal static class Exec
{
    static readonly JsonSerializerOptions JsonOpts = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull,
    };

    public static async Task<int> RunAsync(JsonNode? input, string? pipeNameOverride = null)
    {
        var code = input?["code"]?.GetValue<string>();
        if (string.IsNullOrEmpty(code))
        {
            Console.WriteLine(Receipts.ExecFail(
                "missing required field: code (C# script to compile + run)", "", "")
                .ToJsonString());
            return 2;
        }

        var version = input?["version"]?.GetValue<string>();
        int? hostPid = null;
        if (input?["host_pid"] is JsonNode pidNode && pidNode.GetValueKind() == JsonValueKind.Number)
            hostPid = pidNode.GetValue<int>();
        var transaction = input?["transaction"]?.GetValue<string>() ?? "none";
        var argsObj = input?["args"] as JsonObject ?? new JsonObject();

        // Resolve target. If pipeNameOverride is set (tests), bypass discovery.
        string pipeName;
        string? targetVersion = null;
        int? targetPid = null;
        if (pipeNameOverride is not null)
        {
            pipeName = pipeNameOverride;
        }
        else
        {
            var all = RevitProcessFinder.Enumerate();
            var matches = RevitProcessFinder.Filter(all, version, hostPid);
            if (matches.Count == 0)
            {
                Console.WriteLine(Receipts.ExecFail(
                    all.Count == 0
                        ? "no Revit instance running"
                        : $"requested target not found (running: {string.Join(", ", all.Select(i => i.Version))})",
                    "", "").ToJsonString());
                return 1;
            }
            if (matches.Count > 1)
            {
                Console.WriteLine(Receipts.ExecFail(
                    $"ambiguous target ({matches.Count} matches). Pass `version` or `host_pid` to disambiguate.",
                    "", "").ToJsonString());
                return 4;
            }
            var target = matches[0];
            targetPid = target.Pid;
            targetVersion = target.Version;
            pipeName = RevitProcessFinder.PipeNameFor(target.Pid);
        }

        // Build the inner request DTO.
        var req = new ExecRequest
        {
            Id = Guid.NewGuid().ToString("N"),
            Code = code!,
            Args = JsonObjectToDictionary(argsObj),
            Transaction = transaction,
            TimeoutSeconds = 60,
        };
        var reqJson = JsonSerializer.Serialize(req, JsonOpts);

        // Send over pipe.
        string respJson;
        try
        {
            var client = new PipeClient(pipeName);
            respJson = await client.SendRequestAsync(reqJson, TimeSpan.FromSeconds(30));
        }
        catch (TimeoutException te)
        {
            Console.WriteLine(Receipts.ExecFail(
                te.Message, "", "").ToJsonString());
            return 6;
        }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"pipe transport failed: {e.Message}", e.StackTrace ?? "", "").ToJsonString());
            return 2;
        }

        // Parse + reshape.
        ExecResponse? resp;
        try { resp = JsonSerializer.Deserialize<ExecResponse>(respJson, JsonOpts); }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.ExecFail(
                $"response JSON parse failed: {e.Message}", "", respJson).ToJsonString());
            return 2;
        }
        if (resp is null)
        {
            Console.WriteLine(Receipts.ExecFail(
                "response deserialised to null", "", respJson).ToJsonString());
            return 2;
        }

        if (resp.Ok)
        {
            // resp.Result was deserialised as a generic object — re-serialise then
            // parse as a JsonNode so the envelope carries proper JSON structure.
            JsonNode? resultNode = null;
            if (resp.Result is not null)
            {
                var resultJson = JsonSerializer.Serialize(resp.Result);
                resultNode = JsonNode.Parse(resultJson);
            }
            Console.WriteLine(Receipts.ExecOk(
                resultNode,
                resp.HostVersion ?? targetVersion,
                resp.HostPid ?? targetPid,
                resp.StdoutLog).ToJsonString());
            return 0;
        }

        Console.WriteLine(Receipts.ExecFail(
            resp.Error ?? "unknown error",
            resp.Stack ?? "",
            resp.StdoutLog).ToJsonString());
        return 2;
    }

    static Dictionary<string, object?> JsonObjectToDictionary(JsonObject obj)
    {
        var d = new Dictionary<string, object?>(StringComparer.Ordinal);
        foreach (var kvp in obj) d[kvp.Key] = JsonNodeToObject(kvp.Value);
        return d;
    }

    static object? JsonNodeToObject(JsonNode? node)
    {
        if (node is null) return null;
        if (node is JsonValue v)
        {
            if (v.TryGetValue<bool>(out var b)) return b;
            if (v.TryGetValue<int>(out var i)) return i;
            if (v.TryGetValue<long>(out var l)) return l;
            if (v.TryGetValue<double>(out var d)) return d;
            if (v.TryGetValue<string>(out var s)) return s;
            return v.ToString();
        }
        if (node is JsonArray arr)
        {
            var list = new List<object?>();
            foreach (var item in arr) list.Add(JsonNodeToObject(item));
            return list;
        }
        if (node is JsonObject jo) return JsonObjectToDictionary(jo);
        return null;
    }
}
