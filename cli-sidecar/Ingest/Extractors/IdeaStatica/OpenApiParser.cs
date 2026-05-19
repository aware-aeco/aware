// OpenApiParser — convert an openapi-generator YAML document (from the
// IDEA Statica REST clients repo) into a list of TypeInfo records for the
// host-coverage IR.
//
// Mapping from OpenAPI concepts to the IR:
// ---------------------------------------
//
//   1. components/schemas/<Name> → one TypeInfo per schema.
//      • Object schemas → kind="class". Each property becomes a PropertyInfo
//        with `type` derived from the property's $ref / type / format (see
//        ResolveTypeRef). Getter+setter both true (REST DTOs are POCOs).
//        readOnly properties drop the setter.
//      • String schemas with `enum` → kind="enum". Each enum value becomes
//        an EnumValueInfo. Per the IR schema, EnumValueInfo.value is a
//        JsonElement that's either an integer or a string. openapi-generator
//        emits all IDEA Statica enums as `type: string`, so value is always
//        the string label (which is what the API serializer expects on the
//        wire).
//
//   2. paths/<path>/<verb> → one MethodInfo on a synthetic *Api class.
//      • The *Api class is grouped by the operation's first `tags[0]` value
//        (e.g. tag "Calculation" → CalculationApi). This matches the SDK's
//        own naming convention (CalculationApi.cs in the auto-generated C#
//        client) and is what the SDK markdown docs use as the page slug.
//      • The operation's `operationId` is the method name. We append "Async"
//        to match the SDK markdown (which documents the async variant as
//        the canonical surface — that's the only public client method).
//      • Params: path parameters + the requestBody (if any) become IR ParamInfo
//        entries in the order the OpenAPI document declares them.
//      • Return type: from responses["200"].content["application/json"].schema —
//        either an inline schema or a $ref. The Tier A IR schema doesn't have a
//        formal HTTP-status concept; we collapse to the success type. Operations
//        with no 200 body (rare in this API) return null (treated as void).
//      • Method signature is synthesized: "Async<ReturnType> Name(Type1 p1, Type2 p2)".
//
//   3. Type-level summary / description fields come from:
//      • Schemas: `description` if present, else a placeholder of the form
//        "(No description provided in vendor docs for ...)" matching the
//        Tekla extractor's behaviour for schema-required-summary entries.
//      • API classes (the synthetic *Api groupers): a generic blurb plus the
//        owning OpenAPI title/version so readers can trace the surface back.
//
// What this parser does NOT do (deliberate scope):
//
//   • Discriminators / oneOf / allOf inheritance — IDEA Statica's spec uses
//     allOf in a few places for response-envelope inheritance; we collapse to
//     the most-specific schema and record a remarks note. Future enhancement
//     can deepen this.
//   • Polymorphic typing — the IR's `base` field is left null for all schemas
//     (the spec doesn't expose a class hierarchy reliably).
//   • Path / query parameters split — both go into params[] with `optional`
//     reflecting the OpenAPI `required` field.

using System.Text;

namespace AwareSidecar.Ingest.Extractors.IdeaStatica;

public static class OpenApiParser
{
    /// <summary>
    /// Per-spec context: the parsed OpenAPI doc plus its source URL (used as
    /// each TypeInfo's `doc_url`) and the canonical .NET namespace to prefix
    /// onto schemas + API classes. The <paramref name="DocBaseUrl"/> field is
    /// the openapi-generator's `docs/` folder base URL — used to compute the
    /// initial `doc_url` for each Type / Api class so the IR points at the
    /// markdown page even before the Pass-2 enrichment fetches it.
    /// </summary>
    public sealed record SpecContext(
        YamlNode.Mapping Document,
        string SourceUrl,
        string DocBaseUrl,            // e.g. "https://raw.../<tag>/src/api-sdks/connection-api/clients/csharp/docs/"
        string NamespaceForSchemas,   // e.g. "IdeaStatiCa.ConnectionApi.Model"
        string NamespaceForApis,      // e.g. "IdeaStatiCa.ConnectionApi.Api"
        string ApiTitle,              // e.g. "Connection Rest API 3.0"
        string ApiVersion             // e.g. "3.0"
    );

    /// <summary>
    /// Parse the OpenAPI document and return a list of TypeInfo records mirroring its
    /// schemas + path operations.
    /// </summary>
    public static List<TypeInfo> ParseAll(SpecContext ctx)
    {
        var types = new List<TypeInfo>();

        // Schemas (models).
        var components = ctx.Document.GetMapping("components");
        var schemas = components?.GetMapping("schemas");
        if (schemas is not null)
        {
            foreach (var key in schemas.KeyOrder)
            {
                if (schemas.Get(key) is not YamlNode.Mapping schema) continue;
                types.Add(ParseSchema(ctx, key, schema));
            }
        }

        // Paths → grouped into *Api classes by tag.
        var paths = ctx.Document.GetMapping("paths");
        if (paths is not null)
        {
            var byTag = new Dictionary<string, List<(string verb, string pathStr, YamlNode.Mapping operation)>>(
                StringComparer.Ordinal);
            foreach (var pathKey in paths.KeyOrder)
            {
                if (paths.Get(pathKey) is not YamlNode.Mapping verbsMap) continue;
                foreach (var verb in verbsMap.KeyOrder)
                {
                    if (verbsMap.Get(verb) is not YamlNode.Mapping op) continue;
                    string tag = FirstTag(op) ?? "Default";
                    if (!byTag.TryGetValue(tag, out var list))
                    {
                        list = new List<(string, string, YamlNode.Mapping)>();
                        byTag[tag] = list;
                    }
                    list.Add((verb, pathKey, op));
                }
            }
            foreach (var kvp in byTag)
            {
                types.Add(BuildApiClass(ctx, kvp.Key, kvp.Value));
            }
        }

        return types;
    }

    /// <summary>
    /// Build a TypeInfo for one schema. Object schemas become class types; enum
    /// schemas become enum types.
    /// </summary>
    static TypeInfo ParseSchema(SpecContext ctx, string name, YamlNode.Mapping schema)
    {
        var description = schema.GetScalar("description");
        var summary = NormaliseSummary(description, $"{ctx.NamespaceForSchemas}.{name}");

        var enumNode = schema.GetSequence("enum");
        if (enumNode is not null)
        {
            // Enum schema.
            var valueType = schema.GetScalar("type") ?? "string";
            var values = new List<EnumValueInfo>();
            foreach (var item in enumNode.Items)
            {
                if (item is not YamlNode.Scalar s) continue;
                var labelText = s.Value;
                System.Text.Json.JsonElement valueElem;
                if (valueType == "integer" || valueType == "number")
                {
                    // Coerce numeric enum labels into ints for the IR.
                    if (int.TryParse(labelText, System.Globalization.NumberStyles.Integer,
                                     System.Globalization.CultureInfo.InvariantCulture, out var ival))
                        valueElem = System.Text.Json.JsonSerializer.SerializeToElement(ival, IrJsonContext.Default.Int32);
                    else
                        valueElem = System.Text.Json.JsonSerializer.SerializeToElement(labelText, IrJsonContext.Default.String);
                }
                else
                {
                    valueElem = System.Text.Json.JsonSerializer.SerializeToElement(labelText, IrJsonContext.Default.String);
                }
                values.Add(new EnumValueInfo(labelText, valueElem, null));
            }
            return new TypeInfo(
                name: name,
                @namespace: ctx.NamespaceForSchemas,
                kind: "enum",
                summary: summary,
                remarks: null,
                @base: null,
                interfaces: new List<string>(),
                doc_url: $"{ctx.DocBaseUrl}{name}.md",
                constructors: new List<MethodInfo>(),
                methods: new List<MethodInfo>(),
                properties: new List<PropertyInfo>(),
                events: new List<EventInfo>(),
                enum_values: values,
                delegate_signature: null);
        }

        // Object schema → class type.
        var properties = new List<PropertyInfo>();
        var propsNode = schema.GetMapping("properties");
        if (propsNode is not null)
        {
            foreach (var propName in propsNode.KeyOrder)
            {
                if (propsNode.Get(propName) is not YamlNode.Mapping propSpec) continue;
                var propType = ResolveTypeRef(propSpec);
                bool readOnly = string.Equals(propSpec.GetScalar("readOnly"), "true", StringComparison.Ordinal);
                bool deprecated = string.Equals(propSpec.GetScalar("deprecated"), "true", StringComparison.Ordinal);
                var propDoc = propSpec.GetScalar("description");
                var propSummary = string.IsNullOrEmpty(propDoc)
                    ? $"(No description provided in vendor docs for {name}.{propName}.)"
                    : NormaliseInlineHtml(propDoc);

                properties.Add(new PropertyInfo(
                    name: PascalCase(propName),
                    type: propType,
                    getter: true,
                    setter: !readOnly,
                    summary: propSummary,
                    remarks: deprecated ? "Deprecated by vendor." : null,
                    doc_url: $"{ctx.DocBaseUrl}{name}.md"));
            }
        }

        // additionalProperties — when set to a $ref we expose it as a synthetic "Items" property
        // so the consumer can see the contained shape.
        var addlProps = schema.Get("additionalProperties");
        if (addlProps is YamlNode.Mapping addlMap && addlMap.Get("$ref") is YamlNode.Scalar refScalar)
        {
            var itemType = "Dictionary<string, " + RefToTypeName(refScalar.Value) + ">";
            properties.Add(new PropertyInfo(
                name: "Items",
                type: itemType,
                getter: true,
                setter: true,
                summary: $"Map of additional {name} entries keyed by string.",
                remarks: null,
                doc_url: $"{ctx.DocBaseUrl}{name}.md"));
        }

        return new TypeInfo(
            name: name,
            @namespace: ctx.NamespaceForSchemas,
            kind: "class",
            summary: summary,
            remarks: null,
            @base: null,
            interfaces: new List<string>(),
            doc_url: $"{ctx.DocBaseUrl}{name}.md",
            constructors: new List<MethodInfo>(),  // OpenAPI doesn't specify constructors
            methods: new List<MethodInfo>(),
            properties: properties,
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: null);
    }

    /// <summary>
    /// Build the synthetic *Api class for one OpenAPI tag (e.g. "Calculation" → CalculationApi).
    /// All operations sharing that tag become methods on the class.
    /// </summary>
    static TypeInfo BuildApiClass(
        SpecContext ctx,
        string tag,
        IReadOnlyList<(string verb, string pathStr, YamlNode.Mapping op)> ops)
    {
        var apiClassName = tag.EndsWith("Api", StringComparison.Ordinal) ? tag : tag + "Api";
        var summary = $"{ctx.ApiTitle} — {tag} endpoints. Auto-generated REST surface; one method per HTTP operation.";
        var apiDocUrl = $"{ctx.DocBaseUrl}{apiClassName}.md";
        var methods = new List<MethodInfo>();
        foreach (var (verb, pathStr, op) in ops)
        {
            methods.Add(BuildOperationMethod(ctx, verb, pathStr, op, apiDocUrl));
        }
        return new TypeInfo(
            name: apiClassName,
            @namespace: ctx.NamespaceForApis,
            kind: "class",
            summary: summary,
            remarks: $"REST root: {pathPrefix(ctx)}. Source: {ctx.SourceUrl}",
            @base: null,
            interfaces: new List<string>(),
            doc_url: $"{ctx.DocBaseUrl}{apiClassName}.md",
            constructors: new List<MethodInfo>(),
            methods: methods,
            properties: new List<PropertyInfo>(),
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: null);
    }

    static string pathPrefix(SpecContext ctx)
    {
        // Derive the rest path prefix from the first paths key (e.g. /api/3/...).
        var paths = ctx.Document.GetMapping("paths");
        if (paths is null || paths.KeyOrder.Count == 0) return "/";
        var first = paths.KeyOrder[0];
        // Trim trailing segment after the version prefix /api/N/ so reviewers see the version.
        // e.g. "/api/3/projects/{projectId}/connections/calculate" → "/api/3/"
        var parts = first.Split('/');
        if (parts.Length >= 3 && parts[1] == "api") return "/api/" + parts[2] + "/";
        return first;
    }

    static MethodInfo BuildOperationMethod(
        SpecContext ctx, string verb, string pathStr, YamlNode.Mapping op, string apiDocUrl)
    {
        var operationId = op.GetScalar("operationId") ?? "Unknown";
        var summary = op.GetScalar("summary") ?? "";
        if (string.IsNullOrEmpty(summary)) summary = $"{verb.ToUpperInvariant()} {pathStr}";
        else summary = NormaliseInlineHtml(summary);

        var description = op.GetScalar("description");
        var remarks = string.IsNullOrEmpty(description) ? null : NormaliseInlineHtml(description);

        var paramList = new List<ParamInfo>();

        // Path / query / header parameters.
        var parameters = op.GetSequence("parameters");
        if (parameters is not null)
        {
            foreach (var p in parameters.Items)
            {
                if (p is not YamlNode.Mapping pm) continue;
                var pname = pm.GetScalar("name") ?? "param";
                var pdesc = pm.GetScalar("description");
                var schema = pm.GetMapping("schema") ?? new YamlNode.Mapping(new(), new());
                var ptype = ResolveTypeRef(schema);
                bool required = string.Equals(pm.GetScalar("required"), "true", StringComparison.Ordinal);
                paramList.Add(new ParamInfo(
                    name: pname,
                    type: ptype,
                    doc: string.IsNullOrEmpty(pdesc) ? null : NormaliseInlineHtml(pdesc),
                    optional: !required,
                    @default: null));
            }
        }

        // Request body — comes after path/query params in the SDK signature.
        var requestBody = op.GetMapping("requestBody");
        if (requestBody is not null)
        {
            var bodyDesc = requestBody.GetScalar("description");
            bool bodyRequired = string.Equals(requestBody.GetScalar("required"), "true", StringComparison.Ordinal);
            var bodySchema = ExtractJsonSchema(requestBody);
            string bodyType = bodySchema is null ? "object" : ResolveTypeRef(bodySchema);
            string bodyName = DeriveBodyParamName(bodyType);
            paramList.Add(new ParamInfo(
                name: bodyName,
                type: bodyType,
                doc: string.IsNullOrEmpty(bodyDesc) ? null : NormaliseInlineHtml(bodyDesc),
                optional: !bodyRequired,
                @default: null));
        }

        // Return type — first 2xx response.
        var (returnType, returnDoc) = ResolveSuccessReturn(op);

        // Signature synthesised in the SDK's idiomatic shape:
        //   Task<List<ConResultSummary>> CalculateAsync(Guid projectId, List<int> requestBody)
        // Async-suffix the method name; SDK generated code always exposes the async variant.
        var methodName = operationId + "Async";
        var sig = SynthesiseSignature(methodName, returnType, paramList);

        // Throws — OpenAPI doesn't standardise this; the SDK clients consistently throw
        // ApiException for HTTP errors. Record that as the single canonical exception so the
        // IR exposes the failure mode.
        var throws = new List<ThrowsInfo>
        {
            new("ApiException", "Thrown by the SDK when the HTTP call returns a non-2xx status.")
        };

        return new MethodInfo(
            name: methodName,
            signature: sig,
            summary: summary,
            remarks: remarks,
            @params: paramList,
            returns: returnType is null
                ? null
                : new ReturnInfo(returnType, string.IsNullOrEmpty(returnDoc) ? "" : returnDoc),
            throws: throws,
            events_related: new List<string>(),
            doc_url: $"{apiDocUrl}#{operationId.ToLowerInvariant()}",
            since: null,
            deprecated: string.Equals(op.GetScalar("deprecated"), "true", StringComparison.Ordinal) ? "Deprecated by vendor." : null);
    }

    // ── helpers ────────────────────────────────────────────────────────────

    static string? FirstTag(YamlNode.Mapping op)
    {
        var tags = op.GetSequence("tags");
        if (tags is null || tags.Items.Count == 0) return null;
        if (tags.Items[0] is YamlNode.Scalar s) return s.Value;
        return null;
    }

    static YamlNode.Mapping? ExtractJsonSchema(YamlNode.Mapping requestBodyOrResponse)
    {
        var content = requestBodyOrResponse.GetMapping("content");
        if (content is null) return null;
        // Prefer application/json, fall back to the first content-type if not present.
        var json = content.GetMapping("application/json");
        json ??= content.KeyOrder.Count > 0 ? content.GetMapping(content.KeyOrder[0]) : null;
        return json?.GetMapping("schema");
    }

    static (string? type, string? doc) ResolveSuccessReturn(YamlNode.Mapping op)
    {
        var responses = op.GetMapping("responses");
        if (responses is null) return (null, null);

        // Prefer 200; otherwise fall back to the first 2xx response present.
        YamlNode.Mapping? winning = null;
        string? winningCode = null;
        foreach (var code in responses.KeyOrder)
        {
            if (!code.StartsWith('2')) continue;
            var resp = responses.GetMapping(code);
            if (resp is null) continue;
            winning = resp;
            winningCode = code;
            if (code == "200") break;
        }
        if (winning is null) return (null, null);

        var doc = winning.GetScalar("description");
        var schema = ExtractJsonSchema(winning);
        if (schema is null)
        {
            // No body → void.
            return (null, doc);
        }
        return (ResolveTypeRef(schema), doc);
    }

    /// <summary>
    /// Map an OpenAPI schema node to the closest .NET-style type string.
    /// </summary>
    public static string ResolveTypeRef(YamlNode.Mapping schema)
    {
        // $ref takes precedence.
        var refScalar = schema.GetScalar("$ref");
        if (!string.IsNullOrEmpty(refScalar)) return RefToTypeName(refScalar!);

        var typeStr = schema.GetScalar("type");
        var format = schema.GetScalar("format");
        bool nullable = string.Equals(schema.GetScalar("nullable"), "true", StringComparison.Ordinal);

        if (typeStr == "array")
        {
            var items = schema.GetMapping("items");
            string inner = items is null ? "object" : ResolveTypeRef(items);
            return $"List<{inner}>";
        }

        if (typeStr == "object")
        {
            // Unspecified object — produce `object` per IR convention.
            return "object";
        }

        if (typeStr == "string")
        {
            return format switch
            {
                "uuid" => "Guid",
                "date-time" => "DateTime",
                "byte" => "byte[]",
                "binary" => "Stream",
                _ => nullable ? "string" : "string"
            };
        }

        if (typeStr == "integer")
        {
            return format switch
            {
                "int64" => "long",
                "int32" => "int",
                _ => "int"
            };
        }

        if (typeStr == "number")
        {
            return format switch
            {
                "double" => "double",
                "float" => "float",
                "decimal" => "decimal",
                _ => "double"
            };
        }

        if (typeStr == "boolean") return "bool";

        return "object";
    }

    static string RefToTypeName(string refString)
    {
        // refString is e.g. "#/components/schemas/ConConnection".
        var slash = refString.LastIndexOf('/');
        return slash < 0 ? refString : refString.Substring(slash + 1);
    }

    /// <summary>
    /// Generate a parameter name for a body that doesn't carry one. We use a lowercase form
    /// of the body type's simple name. For "List&lt;int&gt;" we use "requestBody" to match
    /// the SDK's generated naming.
    /// </summary>
    static string DeriveBodyParamName(string bodyType)
    {
        if (bodyType.StartsWith("List<", StringComparison.Ordinal)) return "requestBody";
        if (bodyType == "object") return "body";
        // Take the simple type name and camelCase it.
        return CamelCase(bodyType);
    }

    static string SynthesiseSignature(string name, string? returnType, IReadOnlyList<ParamInfo> @params)
    {
        var sb = new StringBuilder();
        if (returnType is null)
            sb.Append("Task ");
        else
            sb.Append("Task<").Append(returnType).Append("> ");
        sb.Append(name).Append('(');
        for (int i = 0; i < @params.Count; i++)
        {
            if (i > 0) sb.Append(", ");
            sb.Append(@params[i].type).Append(' ').Append(@params[i].name);
        }
        sb.Append(')');
        return sb.ToString();
    }

    static string NormaliseSummary(string? rawDescription, string fqName)
    {
        if (string.IsNullOrWhiteSpace(rawDescription))
            return $"(No description provided in vendor docs for {fqName}.)";
        return NormaliseInlineHtml(rawDescription);
    }

    /// <summary>
    /// Strip the inline HTML that openapi-generator preserves verbatim from XML-doc comments in
    /// the source (e.g. <code>&lt;strong&gt;Bolt identifiers:&lt;/strong&gt;</code>) into compact
    /// readable text. We don't try to render rich HTML — the host-coverage consumer only needs
    /// readable strings.
    /// </summary>
    public static string NormaliseInlineHtml(string s)
    {
        // Cheap & cheerful: drop tags, collapse whitespace.
        var sb = new StringBuilder(s.Length);
        bool inTag = false;
        for (int i = 0; i < s.Length; i++)
        {
            char c = s[i];
            if (c == '<') { inTag = true; continue; }
            if (c == '>') { inTag = false; continue; }
            if (inTag) continue;
            sb.Append(c);
        }
        // HTML-entity unescape for the small set openapi-generator emits.
        var raw = sb.ToString()
            .Replace("&amp;", "&")
            .Replace("&lt;", "<")
            .Replace("&gt;", ">")
            .Replace("&quot;", "\"")
            .Replace("&#39;", "'")
            .Replace("&#x60;", "`")
            .Replace("\r", " ")
            .Replace("\n", " ")
            .Replace("\t", " ");
        // Collapse multiple whitespace runs into single spaces.
        var result = new StringBuilder(raw.Length);
        bool prevSpace = false;
        foreach (var ch in raw)
        {
            bool isSpace = char.IsWhiteSpace(ch);
            if (isSpace)
            {
                if (!prevSpace) result.Append(' ');
                prevSpace = true;
            }
            else
            {
                result.Append(ch);
                prevSpace = false;
            }
        }
        return result.ToString().Trim();
    }

    static string PascalCase(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        if (char.IsUpper(s[0])) return s;
        return char.ToUpperInvariant(s[0]) + s.Substring(1);
    }

    static string CamelCase(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        if (char.IsLower(s[0])) return s;
        return char.ToLowerInvariant(s[0]) + s.Substring(1);
    }
}
