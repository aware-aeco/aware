// PageParser — convert parsed Tapir / Graphisoft command definitions into the
// IR's TypeInfo / MethodInfo shapes.
//
// The fundamental mapping is:
//   - JSON-RPC COMMAND CATEGORY    →  TypeInfo (kind = "static-class")
//   - INDIVIDUAL COMMAND           →  MethodInfo inside that TypeInfo
//   - inputScheme properties       →  MethodInfo.params[]
//   - outputScheme                 →  MethodInfo.returns (synthesised single-object return)
//   - schema definitions (shared)  →  TypeInfo (kind = "class") inside the Schema namespace
//
// Justification for "static-class":
//   - The category is not instantiable; it groups related commands
//   - Commands have no "this" semantics — they are top-level RPC verbs
//   - This matches the brief's instruction: "Each major command CATEGORY becomes a
//     TypeInfo (e.g. Elements, Properties, View, Window, Issue). Commands within
//     that category become methods[]. kind = static-class".
//
// Justification for routing shared schema types into the IR as "class" types:
//   - Tapir's common_schema_definitions.js defines ~200 reusable types that the
//     command schemas $ref into. Without these as first-class IR types, the
//     command MethodInfo.params would carry unresolved `$ref` strings like
//     `ElementIdArrayItem` with no body — opaque to AI agents composing apps.
//   - As IR types they appear in the catalog and skills, so the agent can read
//     "ElementId is an object with one property `guid` of type Guid" before
//     calling a command that takes an ElementId.
//
// Each MethodInfo's `doc_url` points to the Tapir docs page with a #command anchor
// (e.g. https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/#GetAddOnVersion)
// because Tapir's renderer uses each command's name as the anchor.

using System.Text;
using System.Text.Json;

namespace AwareSidecar.Ingest.Extractors.ArchiCAD;

public static class PageParser
{
    /// <summary>
    /// Convert a Tapir command CATEGORY into a TypeInfo. The category's name (e.g.
    /// "Application Commands") becomes the type name (no spaces stripped — Sandcastle-style
    /// "Application Commands" matches the visible group heading in the docs).
    ///
    /// `docsBaseUrl` is the public docs URL (typically https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/);
    /// each command's doc_url is `<docsBaseUrl>#<CommandName>`.
    /// </summary>
    public static TypeInfo BuildTapirCategoryType(
        CommandCategory category,
        string docsBaseUrl)
    {
        var typeName = SanitiseCategoryName(category.Name);
        var methods = new List<MethodInfo>();
        foreach (var cmd in category.Commands)
        {
            methods.Add(BuildTapirCommandMethod(cmd, docsBaseUrl));
        }

        var summary = $"Tapir Add-On '{category.Name}' command group — {category.Commands.Count} JSON-RPC commands.";

        return new TypeInfo(
            name: typeName,
            @namespace: "Tapir.AdditionalCommands",
            kind: "static-class",
            summary: summary,
            remarks: null,
            @base: null,
            interfaces: new List<string>(),
            doc_url: docsBaseUrl,
            constructors: new List<MethodInfo>(),
            methods: methods,
            properties: new List<PropertyInfo>(),
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: null);
    }

    /// <summary>
    /// Convert one Tapir CommandDefinition into a MethodInfo. The command's `inputScheme`
    /// (JSON Schema object) becomes the method's params; its `outputScheme` becomes the
    /// method's `returns` (as a single ReturnInfo whose type is the schema's name or
    /// synthesised name).
    ///
    /// The signature uses TypeScript-ish syntax for readability:
    ///   `GetAddOnVersion(): { version: string }`
    ///   `ChangeWindow(windowType: WindowType, databaseId?: DatabaseId): ExecutionResult`
    /// </summary>
    public static MethodInfo BuildTapirCommandMethod(CommandDefinition cmd, string docsBaseUrl)
    {
        var docUrl = $"{docsBaseUrl}#{cmd.Name}";

        var paramList = new List<ParamInfo>();
        if (cmd.InputScheme is JsonElement input)
        {
            paramList.AddRange(ExtractSchemaProperties(input));
        }

        ReturnInfo? returns = null;
        if (cmd.OutputScheme is JsonElement output)
        {
            var typeLabel = TapirSchemaParser.SummariseSchemaType(output);
            var doc = TapirSchemaParser.ExtractDescription(output);
            if (string.IsNullOrEmpty(doc))
            {
                // Fall back to a synthesised description that names the wrapped properties
                doc = SynthesiseOutputDoc(output);
            }
            returns = new ReturnInfo(type: typeLabel, doc: doc);
        }

        var signature = RenderSignature(cmd.Name, paramList, returns);

        return new MethodInfo(
            name: cmd.Name,
            signature: signature,
            summary: cmd.Description,
            remarks: null,
            @params: paramList,
            returns: returns,
            throws: new List<ThrowsInfo>(),
            events_related: new List<string>(),
            doc_url: docUrl,
            since: string.IsNullOrEmpty(cmd.Version) ? null : cmd.Version,
            deprecated: null);
    }

    /// <summary>
    /// Convert one Tapir shared-schema type entry into a TypeInfo. Each is mapped to an
    /// "object" type with `properties[]` derived from the schema's `properties`.
    /// Pure JSON-Schema-to-IR translation; no $ref resolution (we record the $ref tail
    /// as the property type, matching the Tapir docs viewer's rendering).
    ///
    /// Enums become `kind: "enum"` with the enum-values populated.
    /// Object schemas become `kind: "class"` with the properties populated.
    /// Array schemas become `kind: "class"` with a single `Items` property — same shape
    /// the docs viewer presents (array types are first-class in the catalog).
    /// </summary>
    public static TypeInfo BuildSharedSchemaType(
        string typeName,
        JsonElement schema,
        string docsBaseUrl)
    {
        var docUrl = $"{docsBaseUrl}#{typeName}";
        var summary = TapirSchemaParser.ExtractDescription(schema);
        if (string.IsNullOrEmpty(summary))
            summary = $"Shared schema type `{typeName}` defined in the Tapir / Graphisoft JSON Interface.";

        // Determine kind from schema shape
        string kind = "class";
        var enumValues = new List<EnumValueInfo>();
        var properties = new List<PropertyInfo>();

        if (schema.TryGetProperty("enum", out var enumEl) && enumEl.ValueKind == JsonValueKind.Array)
        {
            kind = "enum";
            foreach (var v in enumEl.EnumerateArray())
            {
                // Enum values can be either strings or integers — both round-trip through
                // JsonElement cleanly (the IR schema declares oneOf:[integer, string]).
                // We source-gen-serialize using the correct typed JsonTypeInfo for each
                // variant — SerializeToElement with a mismatched JsonTypeInfo<T> throws
                // InvalidCast under NativeAOT, so the branch MUST match the runtime type.
                JsonElement valueEl;
                string label;
                if (v.ValueKind == JsonValueKind.String)
                {
                    var s = v.GetString() ?? "";
                    valueEl = JsonSerializer.SerializeToElement(s, IrJsonContext.Default.String);
                    label = s;
                }
                else if (v.ValueKind == JsonValueKind.Number && v.TryGetInt32(out var iv))
                {
                    valueEl = JsonSerializer.SerializeToElement(iv, IrJsonContext.Default.Int32);
                    label = iv.ToString(System.Globalization.CultureInfo.InvariantCulture);
                }
                else
                {
                    // Fallback: stringify whatever shape the schema author chose. Tapir's
                    // enums are uniformly string-typed; this branch is here defensively.
                    var s = v.ToString();
                    valueEl = JsonSerializer.SerializeToElement(s, IrJsonContext.Default.String);
                    label = s;
                }
                enumValues.Add(new EnumValueInfo(label, valueEl, null));
            }
        }
        else if (schema.TryGetProperty("type", out var typeEl)
                 && typeEl.ValueKind == JsonValueKind.String
                 && typeEl.GetString() == "array")
        {
            // Array-of-X: render as a single "Items" property documenting the element type.
            string itemType = "object";
            if (schema.TryGetProperty("items", out var itemsEl))
                itemType = TapirSchemaParser.SummariseSchemaType(itemsEl);
            properties.Add(new PropertyInfo(
                name: "Items",
                type: $"{itemType}[]",
                getter: true,
                setter: false,
                summary: "Array element type.",
                remarks: null,
                doc_url: docUrl));
        }
        else
        {
            // Object schema (or untyped fallback)
            if (schema.TryGetProperty("properties", out var propsEl) && propsEl.ValueKind == JsonValueKind.Object)
            {
                var required = new HashSet<string>(StringComparer.Ordinal);
                if (schema.TryGetProperty("required", out var rqEl) && rqEl.ValueKind == JsonValueKind.Array)
                {
                    foreach (var r in rqEl.EnumerateArray())
                        if (r.ValueKind == JsonValueKind.String) required.Add(r.GetString() ?? "");
                }
                foreach (var prop in propsEl.EnumerateObject())
                {
                    var propType = TapirSchemaParser.SummariseSchemaType(prop.Value);
                    var propDoc = TapirSchemaParser.ExtractDescription(prop.Value);
                    if (string.IsNullOrEmpty(propDoc))
                        propDoc = $"Property `{prop.Name}` of type `{propType}`{(required.Contains(prop.Name) ? " (required)." : " (optional).")}";
                    properties.Add(new PropertyInfo(
                        name: prop.Name,
                        type: propType,
                        getter: true,
                        setter: !required.Contains(prop.Name),
                        summary: propDoc,
                        remarks: required.Contains(prop.Name) ? "Required." : null,
                        doc_url: docUrl));
                }
            }
        }

        return new TypeInfo(
            name: typeName,
            @namespace: "Tapir.Schema",
            kind: kind,
            summary: summary,
            remarks: null,
            @base: null,
            interfaces: new List<string>(),
            doc_url: docUrl,
            constructors: new List<MethodInfo>(),
            methods: new List<MethodInfo>(),
            properties: properties,
            events: new List<EventInfo>(),
            enum_values: enumValues,
            delegate_signature: null);
    }

    /// <summary>
    /// Pull `params[]` out of an inputScheme JSON Schema object. The top-level shape
    /// is always `{ "type": "object", "properties": {...}, "required": [...] }` — we
    /// flatten each top-level property into one ParamInfo. Nested object properties
    /// stay as their $ref-resolved type label; the body of the IR catalog row for the
    /// referenced type carries the structural details (see BuildSharedSchemaType).
    /// </summary>
    static List<ParamInfo> ExtractSchemaProperties(JsonElement schema)
    {
        var result = new List<ParamInfo>();
        if (schema.ValueKind != JsonValueKind.Object) return result;
        if (!schema.TryGetProperty("properties", out var propsEl) || propsEl.ValueKind != JsonValueKind.Object)
            return result;

        var required = new HashSet<string>(StringComparer.Ordinal);
        if (schema.TryGetProperty("required", out var rqEl) && rqEl.ValueKind == JsonValueKind.Array)
        {
            foreach (var r in rqEl.EnumerateArray())
                if (r.ValueKind == JsonValueKind.String) required.Add(r.GetString() ?? "");
        }

        foreach (var prop in propsEl.EnumerateObject())
        {
            var t = TapirSchemaParser.SummariseSchemaType(prop.Value);
            var doc = TapirSchemaParser.ExtractDescription(prop.Value);
            var isReq = required.Contains(prop.Name);
            result.Add(new ParamInfo(
                name: prop.Name,
                type: t,
                doc: string.IsNullOrEmpty(doc) ? null : doc,
                optional: !isReq,
                @default: null));
        }
        return result;
    }

    /// <summary>
    /// Synthesise a one-line description of an output schema by listing its top-level
    /// property names. Used when the schema itself has no `description`.
    /// </summary>
    static string SynthesiseOutputDoc(JsonElement schema)
    {
        if (schema.ValueKind != JsonValueKind.Object) return "Result.";
        if (schema.TryGetProperty("$ref", out _))
        {
            var t = TapirSchemaParser.SummariseSchemaType(schema);
            return $"Returns a `{t}`.";
        }
        if (schema.TryGetProperty("properties", out var propsEl) && propsEl.ValueKind == JsonValueKind.Object)
        {
            var names = new List<string>();
            foreach (var p in propsEl.EnumerateObject())
            {
                names.Add(p.Name);
                if (names.Count >= 5) break;
            }
            if (names.Count == 0) return "Empty result object.";
            return $"Returns an object with properties: {string.Join(", ", names)}.";
        }
        if (schema.TryGetProperty("type", out var tEl) && tEl.ValueKind == JsonValueKind.String)
        {
            var t = tEl.GetString();
            if (t == "array") return "Returns an array.";
            return $"Returns `{t}`.";
        }
        return "Result.";
    }

    /// <summary>
    /// Render a TypeScript-ish signature for an RPC command. The aim is human readability;
    /// downstream agents read `params[]` and `returns` for structured access.
    /// </summary>
    static string RenderSignature(string commandName, IReadOnlyList<ParamInfo> ps, ReturnInfo? ret)
    {
        var sb = new StringBuilder();
        sb.Append(commandName);
        sb.Append('(');
        for (int i = 0; i < ps.Count; i++)
        {
            var p = ps[i];
            if (i > 0) sb.Append(", ");
            sb.Append(p.name);
            if (p.optional) sb.Append('?');
            sb.Append(": ");
            sb.Append(p.type);
        }
        sb.Append("): ");
        sb.Append(ret?.type ?? "void");
        return sb.ToString();
    }

    /// <summary>
    /// Convert a category display name (e.g. "Application Commands", "3D Cut Plane Commands")
    /// into a stable identifier ("ApplicationCommands", "3DCutPlaneCommands"). The IR's
    /// TypeInfo.name is used both for the catalog filename and the human-readable docs;
    /// preserving "Commands" suffix matches Tapir's docs page heading exactly.
    /// </summary>
    static string SanitiseCategoryName(string name)
    {
        // Strip whitespace but preserve word casing.
        var sb = new StringBuilder(name.Length);
        foreach (var c in name)
        {
            if (char.IsLetterOrDigit(c) || c == '_') sb.Append(c);
        }
        return sb.ToString();
    }
}
