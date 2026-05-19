// TapirSchemaParser — parse Tapir's `common_schema_definitions.js` (the
// shared-type catalog referenced from individual command schemas via
// `{ "$ref": "#/<TypeName>" }`) into a dictionary keyed by type name.
//
// File shape:
//     var gSchemaDefinitions = {
//         "Elements": { "type": "array", "description": "...", "items": ... },
//         "ElementIdArrayItem": { "type": "object", ... },
//         ...
//     };
//
// Each shared type is a JSON Schema fragment. We keep the raw JsonElement around
// so the IR builder (Extractor.cs) can resolve $ref chains during type-derivation.
//
// Output: a Dictionary<string, JsonElement> keyed by the shared-type name.

using System.Text.Json;

namespace AwareSidecar.Ingest.Extractors.ArchiCAD;

public static class TapirSchemaParser
{
    /// <summary>
    /// Parse the JS-wrapped JSON of common_schema_definitions.js into a dictionary
    /// mapping shared-type name → schema JsonElement. The JsonElements are cloned
    /// out of the parsing JsonDocument so they outlive its disposal — callers can
    /// keep references after this function returns.
    /// </summary>
    public static Dictionary<string, JsonElement> Parse(string js)
    {
        var json = TapirCommandParser.StripJsWrapper(js, "gSchemaDefinitions");
        using var doc = JsonDocument.Parse(json);

        var result = new Dictionary<string, JsonElement>(StringComparer.Ordinal);
        foreach (var prop in doc.RootElement.EnumerateObject())
        {
            // Clone() is essential — the JsonDocument is disposed at scope exit
            // and JsonElement instances backed by a disposed document throw at
            // every access. Clone() detaches the element from the document.
            result[prop.Name] = prop.Value.Clone();
        }
        return result;
    }

    /// <summary>
    /// Render a JSON Schema fragment to a short human-readable type label (used for
    /// `params[].type` and `returns.type` in the IR). Handles the primitive shapes
    /// (string / integer / number / boolean / null), object / array, and `$ref`
    /// pointers to the shared schema. Falls back to "object" if the shape is too
    /// exotic to summarise.
    ///
    /// Examples:
    ///   { "type": "string" }                              → "string"
    ///   { "type": "integer" }                             → "integer"
    ///   { "type": "array", "items": { "$ref": "#/A" } }   → "A[]"
    ///   { "$ref": "#/ElementId" }                         → "ElementId"
    ///   { "$ref": "APITypes.json#/definitions/ExecResult" } → "ExecResult" (the fragment tail)
    ///   { "enum": ["A", "B"] }                            → "string-enum"
    ///   { "type": "object" }                              → "object" (in-line schema)
    /// </summary>
    public static string SummariseSchemaType(JsonElement schema)
    {
        if (schema.ValueKind != JsonValueKind.Object) return "object";

        // $ref takes precedence over type
        if (schema.TryGetProperty("$ref", out var refEl) && refEl.ValueKind == JsonValueKind.String)
        {
            var raw = refEl.GetString() ?? "";
            // Tapir uses "#/TypeName" form
            // Graphisoft uses "APITypes.json#/definitions/ExecutionResult" form
            var hash = raw.LastIndexOf('/');
            return hash >= 0 ? raw.Substring(hash + 1) : raw;
        }

        if (schema.TryGetProperty("type", out var typeEl))
        {
            var t = typeEl.ValueKind == JsonValueKind.String ? typeEl.GetString() ?? "object" : "object";
            if (t == "array")
            {
                if (schema.TryGetProperty("items", out var itemsEl))
                    return SummariseSchemaType(itemsEl) + "[]";
                return "array";
            }
            // primitive
            return t;
        }

        if (schema.TryGetProperty("enum", out var enumEl) && enumEl.ValueKind == JsonValueKind.Array)
        {
            // Determine enum value type from first member
            foreach (var v in enumEl.EnumerateArray())
            {
                if (v.ValueKind == JsonValueKind.String) return "string-enum";
                if (v.ValueKind == JsonValueKind.Number) return "number-enum";
                break;
            }
            return "enum";
        }

        if (schema.TryGetProperty("oneOf", out _)) return "oneOf";
        if (schema.TryGetProperty("anyOf", out _)) return "anyOf";

        return "object";
    }

    /// <summary>
    /// Extract one-line documentation prose from a JSON Schema fragment's `description`
    /// (preferred) or fall back to `title` if present. Returns empty string for fragments
    /// with neither. Strips newlines so the result is safe to embed in single-line YAML.
    /// </summary>
    public static string ExtractDescription(JsonElement schema)
    {
        if (schema.ValueKind != JsonValueKind.Object) return "";
        if (schema.TryGetProperty("description", out var dEl) && dEl.ValueKind == JsonValueKind.String)
            return (dEl.GetString() ?? "").Replace('\r', ' ').Replace('\n', ' ').Trim();
        if (schema.TryGetProperty("title", out var tEl) && tEl.ValueKind == JsonValueKind.String)
            return (tEl.GetString() ?? "").Replace('\r', ' ').Replace('\n', ' ').Trim();
        return "";
    }
}
