// GraphisoftMenuParser — parse Graphisoft's official Archicad JSON Interface
// documentation, served from https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/.
//
// Structure (verified 2026-05-19):
//
//   content/menutree.json          — the directory: nested groups + commands +
//                                    HTML docs (free-form prose pages we skip)
//   content/<CommandFile>.json     — one file per command, body shape:
//
//     {
//       "$schema": "http://json-schema.org/draft-04/schema",
//       "description": "Executes a command registered in an Add-On.",
//       "definitions": {
//         "command_parameters":  { ...JSON Schema... },
//         "response_parameters": { ...JSON Schema... },
//         "examples": [...]
//       }
//     }
//
//   menutree.json contains:
//     - top-level groups (with `menuitems[]`) — e.g. "AddOn Commands", "Attribute Commands"
//     - each group's menuitems[] is either:
//         { "name": "CmdName", "commanddocumentation": "API.CmdName.json" }   — a command
//         { "name": "Sub", "htmldocumentation": "foo.html" }                  — free prose
//         { "name": "Sub", "menuitems": [...] }                               — nested
//
// We walk the tree depth-first; every leaf with `commanddocumentation` becomes a
// downloadable command. Categories nest as "ParentName_ChildName" if needed (rare —
// the public surface is one level deep).
//
// Compared with Tapir's JS-wrapped catalog, this site requires N+1 fetches: one for
// the menutree, then one per command JSON. We use the existing HttpScraper.

using System.Text.Json;
using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.ArchiCAD;

public static class GraphisoftMenuParser
{
    public const string DocsBaseUrl = "https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/";
    public const string ContentBaseUrl = DocsBaseUrl + "content/";

    public sealed record GraphisoftCommand(
        string GroupName,
        string CommandName,
        string FileName,
        string FetchUrl,
        string DocPageUrl);

    /// <summary>
    /// Parse the menutree JSON into a flat list of (group, command, file, url) tuples.
    /// Skips html-only entries (Introduction, Command Format, General Information sub-pages)
    /// since they have no command schema to extract.
    /// </summary>
    public static List<GraphisoftCommand> ParseMenuTree(string menuJson)
    {
        using var doc = JsonDocument.Parse(menuJson);
        var result = new List<GraphisoftCommand>();
        if (!doc.RootElement.TryGetProperty("menuitems", out var topItemsEl)) return result;
        foreach (var topEl in topItemsEl.EnumerateArray())
        {
            WalkMenuItem(topEl, parentGroup: null, result);
        }
        return result;
    }

    static void WalkMenuItem(JsonElement item, string? parentGroup, List<GraphisoftCommand> sink)
    {
        var name = item.TryGetProperty("name", out var nEl) && nEl.ValueKind == JsonValueKind.String
            ? nEl.GetString() ?? ""
            : "";

        // Leaf: a command
        if (item.TryGetProperty("commanddocumentation", out var cdEl) && cdEl.ValueKind == JsonValueKind.String)
        {
            var fileName = cdEl.GetString() ?? "";
            sink.Add(new GraphisoftCommand(
                GroupName: parentGroup ?? "Misc",
                CommandName: name,
                FileName: fileName,
                FetchUrl: ContentBaseUrl + fileName,
                DocPageUrl: DocsBaseUrl + "#" + name));
            return;
        }

        // Nested group: recurse
        if (item.TryGetProperty("menuitems", out var childEl) && childEl.ValueKind == JsonValueKind.Array)
        {
            foreach (var c in childEl.EnumerateArray())
            {
                WalkMenuItem(c, parentGroup: name, sink);
            }
        }
        // else: leaf htmldocumentation — skip (prose, no schema)
    }

    /// <summary>
    /// Convert one Graphisoft command JSON (content/API.X.json) into a CommandDefinition
    /// that can be fed to PageParser.BuildTapirCommandMethod. The wrap-around fields
    /// are: top-level `description` → CommandDefinition.Description; `definitions.command_parameters`
    /// → InputScheme; `definitions.response_parameters` → OutputScheme.
    /// </summary>
    public static CommandDefinition ParseCommandJson(string commandName, string json)
    {
        using var doc = JsonDocument.Parse(json);
        var root = doc.RootElement;
        var description = root.TryGetProperty("description", out var dEl) && dEl.ValueKind == JsonValueKind.String
            ? dEl.GetString() ?? ""
            : "";

        JsonElement? input = null;
        JsonElement? output = null;
        if (root.TryGetProperty("definitions", out var defsEl) && defsEl.ValueKind == JsonValueKind.Object)
        {
            if (defsEl.TryGetProperty("command_parameters", out var iEl) && iEl.ValueKind != JsonValueKind.Null)
                input = iEl.Clone();
            if (defsEl.TryGetProperty("response_parameters", out var oEl) && oEl.ValueKind != JsonValueKind.Null)
                output = oEl.Clone();
        }

        return new CommandDefinition(
            Name: commandName,
            Version: "",  // Graphisoft official site does not stamp per-command versions
            Description: description,
            InputScheme: input,
            OutputScheme: output);
    }
}
