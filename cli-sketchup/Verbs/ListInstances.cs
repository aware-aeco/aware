// `list-instances` verb: enumerates running SketchUp bridges via the
// discovery-file folder. Output shape mirrors cli-rhino's list-instances:
//   {"status":"ok","instances":[{pid, version, port, model_path}], ...}

using System.Text.Json.Nodes;

namespace AwareSketchup.Verbs;

internal static class ListInstances
{
    public static int Run(SketchupClient? clientOverride = null)
    {
        var client = clientOverride ?? new SketchupClient();
        var all = client.ListInstances();
        var reshaped = new JsonArray();
        foreach (var inst in all)
        {
            reshaped.Add(new JsonObject
            {
                ["pid"]            = inst.Pid,
                ["version"]        = inst.Version,
                ["port"]           = inst.Port,
                ["model_path"]     = inst.ModelPath,
                ["sketchup_id"]    = inst.Pid.ToString(),
                // The bridge RUNNING in that session — null when it predates 0.35.0.
                // A session keeps the bridge it loaded at startup, so this is the only
                // way to tell a stale session from one running the installed bridge.
                ["bridge_version"] = inst.BridgeVersion,
            });
        }
        Console.WriteLine(Receipts.ListOk(reshaped).ToJsonString());
        return 0;
    }
}
