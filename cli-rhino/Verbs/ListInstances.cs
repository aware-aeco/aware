// `list-instances` verb: shells out to `rhinocode list --json` and reshapes
// McNeel's keys (pipeId, processId, processVersion, activeDoc) into our
// snake_case schema (rhino_id, pid, version, active_doc).

using System.Text.Json.Nodes;

namespace AwareRhino.Verbs;

internal static class ListInstances
{
    public static int Run(RhinocodeClient client)
    {
        var (exit, stdout, stderr) = client.RunListJson();
        if (exit != 0)
        {
            Console.WriteLine(Receipts.GenericFail("list-instances",
                $"rhinocode list exited {exit}: {stderr.Trim()}").ToJsonString());
            return 2;
        }

        JsonArray? raw;
        try
        {
            raw = JsonNode.Parse(stdout) as JsonArray;
            if (raw is null) throw new InvalidOperationException("expected JSON array");
        }
        catch (Exception e)
        {
            Console.WriteLine(Receipts.GenericFail("list-instances",
                $"could not parse rhinocode list --json output: {e.Message}").ToJsonString());
            return 2;
        }

        // If rhinocode sees no sessions but Rhino.exe is running, the Script
        // Server was not started. Auto-start it and re-query so the caller gets
        // a useful result rather than an empty list.
        if (raw.Count == 0 && ScriptServerHelper.TryEnsureScriptServer(client))
        {
            var (exit2, stdout2, _) = client.RunListJson();
            if (exit2 == 0)
                try { raw = JsonNode.Parse(stdout2) as JsonArray ?? raw; } catch { }
        }

        var reshaped = new JsonArray();
        foreach (var item in raw)
        {
            if (item is not JsonObject jo) continue;
            reshaped.Add(new JsonObject
            {
                ["pid"]        = jo["processId"]?.GetValue<int?>(),
                ["version"]    = TrimVersion(jo["processVersion"]?.GetValue<string>()),
                ["rhino_id"]   = jo["pipeId"]?.GetValue<string>(),
                ["active_doc"] = jo["activeDoc"]?.GetValue<string>(),
            });
        }

        Console.WriteLine(Receipts.ListOk(reshaped).ToJsonString());
        return 0;
    }

    // "8.13.24296.13001" → "8.13" so it matches the version filter shape used in `exec`.
    static string? TrimVersion(string? full)
    {
        if (string.IsNullOrEmpty(full)) return null;
        var parts = full!.Split('.');
        return parts.Length >= 2 ? $"{parts[0]}.{parts[1]}" : full;
    }
}
