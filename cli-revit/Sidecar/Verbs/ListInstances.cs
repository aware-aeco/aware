// list-instances verb — enumerate running Revit.exe processes, probe each
// for the AwareRevit add-in pipe, emit a JSON receipt. Does not invoke any
// Revit API (process listing + cheap pipe probe only).

using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class ListInstances
{
    public static async Task<int> RunAsync()
    {
        var raw = RevitProcessFinder.Enumerate();
        var probed = new List<RevitInstance>();
        foreach (var inst in raw)
            probed.Add(await RevitProcessFinder.ProbeAddinAsync(inst));

        var arr = new JsonArray();
        foreach (var i in probed)
        {
            arr.Add(new JsonObject
            {
                ["pid"]          = i.Pid,
                ["version"]      = i.Version,
                ["exe_path"]     = i.ExePath,
                ["addin_loaded"] = i.AddinLoaded,
            });
        }
        Console.WriteLine(Receipts.ListOk(arr).ToJsonString());
        return 0;
    }
}
