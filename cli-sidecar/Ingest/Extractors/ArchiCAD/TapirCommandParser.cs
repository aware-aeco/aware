// TapirCommandParser — parse Tapir's `command_definitions.js` (the JS-wrapped JSON
// file that drives https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)
// into a flat list of typed `CommandDefinition` records.
//
// Tapir is fundamentally different from a class-library vendor (Tekla, Rhino):
//   - It exposes a JSON-RPC command catalog, not a typed surface area
//   - Each command has an `inputScheme` and `outputScheme` (both JSON Schema fragments)
//   - Commands are grouped into categories ("Application Commands", "Element Commands", ...)
//
// File shape (literally the JS source served by the docs page):
//
//     var gCommands = [
//         { "name": "Application Commands", "commands": [
//             { "name": "GetAddOnVersion", "version": "0.1.0",
//               "description": "Retrieves the version...",
//               "inputScheme": null,
//               "outputScheme": { ...JSON Schema... }
//             }, ...
//         ]}, ...
//     ];
//
// Parsing: strip the `var gCommands = ` prefix and trailing `;`, then JsonDocument.Parse.
// No reflection, no AOT hazard — JsonDocument is source-gen-free.
//
// Output: one CommandCategory per top-level group, each with a list of CommandDefinition
// records. The Extractor.cs orchestrator turns each category into a TypeInfo with the
// commands as MethodInfo entries.

using System.Text.Json;

namespace AwareSidecar.Ingest.Extractors.ArchiCAD;

public sealed record CommandCategory(string Name, List<CommandDefinition> Commands);

public sealed record CommandDefinition(
    string Name,
    string Version,
    string Description,
    JsonElement? InputScheme,
    JsonElement? OutputScheme);

public static class TapirCommandParser
{
    /// <summary>
    /// Parse the JS-wrapped JSON of command_definitions.js into a list of CommandCategory.
    /// The file's literal shape is `var gCommands = [...];`; we strip the prefix + trailing
    /// semicolon before handing the body to JsonDocument.
    /// </summary>
    public static List<CommandCategory> Parse(string js)
    {
        var json = StripJsWrapper(js, "gCommands");
        using var doc = JsonDocument.Parse(json);

        var categories = new List<CommandCategory>();
        foreach (var groupEl in doc.RootElement.EnumerateArray())
        {
            var groupName = groupEl.GetProperty("name").GetString() ?? "Unknown";
            var commands = new List<CommandDefinition>();
            if (groupEl.TryGetProperty("commands", out var cmdsEl))
            {
                foreach (var c in cmdsEl.EnumerateArray())
                {
                    var name = c.GetProperty("name").GetString() ?? "Unknown";
                    var version = c.TryGetProperty("version", out var vEl) && vEl.ValueKind == JsonValueKind.String
                        ? vEl.GetString() ?? ""
                        : "";
                    var description = c.TryGetProperty("description", out var dEl) && dEl.ValueKind == JsonValueKind.String
                        ? dEl.GetString() ?? ""
                        : "";
                    JsonElement? input = c.TryGetProperty("inputScheme", out var iEl) && iEl.ValueKind != JsonValueKind.Null
                        ? iEl.Clone()
                        : null;
                    JsonElement? output = c.TryGetProperty("outputScheme", out var oEl) && oEl.ValueKind != JsonValueKind.Null
                        ? oEl.Clone()
                        : null;
                    commands.Add(new CommandDefinition(name, version, description, input, output));
                }
            }
            categories.Add(new CommandCategory(groupName, commands));
        }
        return categories;
    }

    /// <summary>
    /// Strip the `var <name> = ` prefix and trailing `;` from a JS-wrapped JSON file. The
    /// remaining body is valid JSON. Throws if the wrapper is missing — by design, we want
    /// the extraction to fail loudly if Tapir restructures the file.
    /// </summary>
    internal static string StripJsWrapper(string js, string varName)
    {
        var prefix = $"var {varName} = ";
        var startIdx = js.IndexOf(prefix, StringComparison.Ordinal);
        if (startIdx < 0)
            throw new InvalidDataException(
                $"TapirCommandParser: expected `{prefix}` in JS file but did not find it. " +
                $"Tapir restructured the file? Head: {js.Substring(0, Math.Min(120, js.Length))}");

        var bodyStart = startIdx + prefix.Length;
        var body = js.Substring(bodyStart).TrimEnd();
        // Trailing semicolon is the only non-JSON character.
        if (body.EndsWith(";", StringComparison.Ordinal))
            body = body.Substring(0, body.Length - 1).TrimEnd();
        return body;
    }
}
