// launch verb: spawn Revit.exe (and optionally open a model). Mirrors
// cli-tekla's launch -- returns immediately after Process.Start; the caller
// polls list-instances for addin_loaded=true to confirm IPC readiness.

using System.Diagnostics;
using System.IO;
using System.Text.Json.Nodes;

namespace AwareRevit.Sidecar.Verbs;

internal static class Launch
{
    public static int Run(JsonNode? input)
    {
        var version = input?["version"]?.GetValue<string>();
        var modelPath = input?["model_path"]?.GetValue<string>();
        var language = input?["language"]?.GetValue<string>();

        if (string.IsNullOrEmpty(version))
        {
            EmitGuide("missing required field: version (e.g. \"2026\")");
            return 2;
        }

        var revitExe = $@"C:\Program Files\Autodesk\Revit {version}\Revit.exe";
        if (!File.Exists(revitExe))
        {
            EmitGuide($"Revit {version} is not installed at {revitExe}. " +
                      "Install it or pick a version that is. " +
                      "Run `aware-revit list-instances` to see installed versions.");
            return 3;
        }

        var argList = "";
        if (!string.IsNullOrEmpty(language))
            argList += $"/language {language} ";
        if (!string.IsNullOrEmpty(modelPath))
            argList += $"\"{modelPath}\"";

        var psi = new ProcessStartInfo
        {
            FileName        = revitExe,
            Arguments       = argList.Trim(),
            UseShellExecute = true,
            WindowStyle     = ProcessWindowStyle.Maximized,
        };
        var p = Process.Start(psi);
        if (p == null)
        {
            Console.Error.WriteLine("aware-revit: Process.Start returned null");
            return 2;
        }

        var verbResult = new JsonObject
        {
            ["model_path"] = modelPath ?? "",
            ["language"]   = language ?? "",
            ["note"]       = "Revit is starting; poll list-instances until addin_loaded=true (~20s)",
        };
        Console.WriteLine(Receipts.LaunchOk(p.Id, version!, verbResult).ToJsonString());
        return 0;
    }

    static void EmitGuide(string message)
    {
        Console.Error.WriteLine($"aware-revit launch: {message}");
        Console.Error.WriteLine();
        Console.Error.WriteLine("Required stdin JSON shape:");
        Console.Error.WriteLine("  { \"version\": \"2026\",");
        Console.Error.WriteLine("    \"model_path\": \"C:/Models/Project.rvt\" }");
    }
}
