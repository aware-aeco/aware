// PageParser — turn the Bluebeam Studio API Postman v2.1 collection into a list of
// TypeInfo records for the host-coverage IR.
//
// Why Postman, not Swagger / OpenAPI:
// -----------------------------------
// Bluebeam's developer portal at https://developers.bluebeam.com/studio-api/ is
// auth-walled (the Salesforce-hosted Developer Network requires sign-in to view
// the API references). The vendor ALSO publishes a fully-public Postman v2.1
// collection at https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json
// (referenced verbatim from the "Getting Started" guide at
// https://support.bluebeam.com/developer/getting-started-dev-portal.html). That
// JSON file is the canonical machine-readable description of the Studio API and
// is what we ingest here.
//
// The collection has 123 operations grouped into 4 top-level folders (Sessions,
// Projects, Jobs, Misc) with a further level of subfolders. Each request carries:
//
//   • method (GET / POST / PUT / DELETE)
//   • url.path[] — the path segments incl. `{sessionId}`, `{id}` etc. placeholders
//   • name — a short imperative description ("Create metadata for a new Studio
//     Session file.")
//   • body.raw — optional JSON example illustrating the request body shape
//
// There are NO query-string parameters, NO response-body shapes, NO header
// declarations, NO schemas. The Studio API is documented as a flat list of
// path+verb tuples plus example JSON bodies — and the Postman collection is the
// only public machine-readable artifact that lists them.
//
// Mapping from Postman → IR:
// --------------------------
//   1. Folder structure → namespace + Api class. Each (top, sub) folder pair maps
//      to one synthetic *Api class. Example:
//        • Sessions / Files       → namespace "Bluebeam.StudioApi.Sessions",
//                                    class "SessionsFilesApi"
//        • Sessions / (root)      → "Bluebeam.StudioApi.Sessions", "SessionsApi"
//        • Sessions / Single Session  → folded into "SessionsApi" (the "Single Session"
//                                    subfolder is a UX-only split; both top-level
//                                    Sessions ops and Single Session ops act on
//                                    /publicapi/v1/sessions and /publicapi/v1/sessions/{id})
//        • Projects / Project File Jobs of "Jobs"  → "Bluebeam.StudioApi.Jobs",
//                                    class "ProjectFileJobsApi"
//        • Jobs / Job             → "Bluebeam.StudioApi.Jobs", "JobsApi"
//        • Misc                   → "Bluebeam.StudioApi", "MiscApi"
//
//   2. Each request → one MethodInfo. Method name is synthesized from the
//      request's `name` field via Title-casing + alpha-numeric stripping (see
//      ToMethodName below). Each path token like `{sessionId}` becomes one IR
//      ParamInfo with type `string` (or `Guid` for known UUID params, `int` for
//      numeric ids — Studio uses GUIDs for session/project IDs).
//
//   3. Each request body → one synthetic class TypeInfo named `<MethodName>Request`
//      with one PropertyInfo per top-level JSON field. The property `type` is
//      inferred from the example value's JSON type (`string` → string, `0` → int,
//      `true` → bool, `0.0` → double, nested object → object, array → List<object>).
//      The Request DTO is added as the last ParamInfo of the method (named
//      `request`) with type `<MethodName>Request`.
//
//   4. Return types are NOT documented anywhere in the collection (Bluebeam's
//      docs page describes them in prose; e.g. "Response: 204 status code" or
//      "Response: Session ID"). We can't infer them reliably so we record
//      return=null (treated as void) on every method. The Studio Session Guide at
//      support.bluebeam.com names a few — SessionId / DownloadUrl / Status — but
//      with no consistent schema, capturing this in the IR would require
//      hand-curation which contradicts the "automated extraction" contract.
//
//   5. Doc URLs:
//        • The TypeInfo's doc_url points at the Postman documentation page
//          (https://www.postman.com/bluebeam-developers/bluebeam-api/...) where
//          the operations actually render. We use the deeper Studio Session
//          Guide URL for the Sessions classes since that's the human-facing
//          rendered reference.
//        • Each MethodInfo gets the same doc_url for now — the Postman web
//          renderer doesn't expose stable per-operation anchors.
//
// Method-name uniqueness:
// ----------------------
// Some Postman folders contain multiple ops whose short names collide after
// PascalCase stripping. Example: SessionsUsersApi has BOTH `GET /users/{id}` and
// `GET /users/{userId}/permissions`, with Postman names "Get a single user in a
// Studio Session" and "Get Studio Session user permissions" — they don't collide
// in this case, but the synthesis logic must guarantee uniqueness across the
// class. We track seen method names per-class and disambiguate by appending the
// HTTP verb if a collision occurs.

using System.Collections.Generic;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest.Extractors.Bluebeam;

public static class PageParser
{
    /// <summary>
    /// One MethodInfo plus its associated Request-DTO TypeInfo (or null if the
    /// operation has no body). Returned alongside the parent class so the
    /// extractor can collate types from multiple ops without re-walking the JSON.
    /// </summary>
    public sealed record ParsedOperation(
        MethodInfo Method,
        TypeInfo? RequestDto);

    /// <summary>
    /// One synthetic *Api class with all its operations.
    /// </summary>
    public sealed record ParsedApiClass(
        string ClassName,
        string Namespace,
        string Summary,
        string DocUrl,
        List<MethodInfo> Methods,
        List<TypeInfo> RequestDtos);

    /// <summary>
    /// Constants pinned to vendor-published URLs as of 2026-05-19. The Postman web renderer
    /// re-mirrors the same JSON we ingested, so we use it as the type-level doc_url.
    /// </summary>
    public const string CollectionJsonUrl = "https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json";
    public const string PostmanDocsBase = "https://www.postman.com/bluebeam-developers/bluebeam-api/documentation/ngtxoq6/studio-api";
    public const string DevPortalRoot = "https://support.bluebeam.com/developer/";
    public const string StudioSessionGuideUrl = "https://support.bluebeam.com/developer/studio-session-guide.html";

    /// <summary>
    /// Path-token names → IR ParamInfo.type. Bluebeam uses GUIDs for top-level
    /// resource ids (sessionId, projectId, fileId in Projects context, etc.) and
    /// int for jobs / job-results / job step ids. Path tokens not in this map
    /// fall back to `string`.
    /// </summary>
    static readonly Dictionary<string, string> PathTokenTypeMap = new(StringComparer.Ordinal)
    {
        // GUID-typed identifiers (Studio Sessions + Projects all use GUIDs)
        ["sessionId"]   = "Guid",
        ["projectId"]   = "Guid",
        ["fileId"]      = "int",    // fileId is project-scoped int (the integer key in /projects/.../files)
        ["folderId"]    = "int",
        ["userId"]      = "Guid",
        ["jobId"]       = "Guid",
        ["resultId"]    = "int",
        ["id"]          = "Guid",   // top-level resource id, contextually a GUID for sessions/projects/users
    };

    /// <summary>
    /// Parse the entire Postman v2.1 collection JSON into a list of synthetic *Api class
    /// types (with methods) plus all their generated Request DTO types.
    /// </summary>
    public static List<TypeInfo> ParseCollection(string collectionJson)
    {
        var doc = JsonDocument.Parse(collectionJson);
        var root = doc.RootElement;

        // The Postman wrapper looks like {"collection": {"info": ..., "item": [...]}}
        if (!root.TryGetProperty("collection", out var coll))
            throw new InvalidDataException("Postman JSON missing 'collection' root property.");

        if (!coll.TryGetProperty("item", out var topItems) || topItems.ValueKind != JsonValueKind.Array)
            throw new InvalidDataException("Postman JSON missing 'collection.item' array.");

        var parsedClasses = new List<ParsedApiClass>();
        foreach (var top in topItems.EnumerateArray())
        {
            ProcessTopLevel(top, parsedClasses);
        }

        var allTypes = new List<TypeInfo>();
        var seenDtos = new HashSet<string>(StringComparer.Ordinal);
        foreach (var cls in parsedClasses)
        {
            allTypes.Add(new TypeInfo(
                name: cls.ClassName,
                @namespace: cls.Namespace,
                kind: "class",
                summary: cls.Summary,
                remarks: $"REST root: /publicapi/v1/. Source: {CollectionJsonUrl}",
                @base: null,
                interfaces: new List<string>(),
                doc_url: cls.DocUrl,
                constructors: new List<MethodInfo>(),
                methods: cls.Methods,
                properties: new List<PropertyInfo>(),
                events: new List<EventInfo>(),
                enum_values: new List<EnumValueInfo>(),
                delegate_signature: null));
            foreach (var dto in cls.RequestDtos)
            {
                // De-duplicate by full namespace.Name in case two ops produce semantically
                // identical body schemas — keep the first occurrence to preserve provenance.
                var key = dto.@namespace + "." + dto.name;
                if (seenDtos.Add(key))
                    allTypes.Add(dto);
            }
        }
        return allTypes;
    }

    static void ProcessTopLevel(JsonElement top, List<ParsedApiClass> output)
    {
        var topName = top.TryGetProperty("name", out var n) ? n.GetString() ?? "" : "";
        topName = topName.Trim();
        if (string.IsNullOrEmpty(topName)) topName = "Misc";

        // Map folder name → namespace + api-class-name prefix. Single-word folders ("Sessions",
        // "Projects", "Jobs") become both the namespace tail and the class prefix.
        var ns = $"Bluebeam.StudioApi.{topName}";

        // The top-level folder's direct children can be EITHER subfolders (with their own
        // items) OR direct request items belonging to the top-level Api class. We accumulate
        // both: direct requests roll into the top-level *Api class (e.g. SessionsApi), while
        // subfolders produce their own Api class (e.g. SessionsFilesApi).
        var topOps = new List<(string folderPath, JsonElement request, string requestName)>();
        var bySubfolder = new Dictionary<string, List<(string folderPath, JsonElement request, string requestName)>>(StringComparer.Ordinal);

        if (top.TryGetProperty("item", out var items) && items.ValueKind == JsonValueKind.Array)
        {
            foreach (var child in items.EnumerateArray())
            {
                CollectOpsRecursive(child, topName, topOps, bySubfolder);
            }
        }

        // 1) Emit the top-level *Api class for direct ops (and "Single X" rollup folders).
        if (topOps.Count > 0)
        {
            var topClassName = $"{topName}Api";
            var className = NormaliseClassName(topClassName);
            output.Add(BuildClass(className, ns, topName, topOps, parentSubfolder: null));
        }

        // 2) Emit one *Api class per subfolder that has its own ops. Class-name compounding:
        //    if the subfolder name already contains the top-level name (e.g. "Project File Jobs"
        //    under "Jobs"), use just the subfolder PascalCased + "Api". Otherwise concatenate
        //    top + sub for unambiguity (Sessions/Files → SessionsFilesApi).
        foreach (var kvp in bySubfolder)
        {
            if (kvp.Value.Count == 0) continue;
            var subName = kvp.Key;
            string classBase;
            if (subName.Contains(topName, StringComparison.OrdinalIgnoreCase)
                || subName.Contains(topName.TrimEnd('s'), StringComparison.OrdinalIgnoreCase))
            {
                classBase = subName;
            }
            else
            {
                classBase = $"{topName}{subName}";
            }
            var compoundName = NormaliseClassName($"{classBase}Api");
            output.Add(BuildClass(compoundName, ns, $"{topName} / {subName}", kvp.Value, parentSubfolder: subName));
        }
    }

    /// <summary>
    /// Walk a tree under a top-level folder. Direct-request children land in the
    /// <paramref name="topOps"/> list. Subfolder children recurse — their requests land in
    /// <paramref name="bySubfolder"/> keyed by the subfolder name. The "Single Session" /
    /// "Single Project" / "Job" UI-grouping subfolders fold INTO the top-level list rather
    /// than spawning their own *Api class (their ops act on the SAME root path as the parent
    /// folder's own ops).
    /// </summary>
    static void CollectOpsRecursive(
        JsonElement node,
        string topName,
        List<(string folderPath, JsonElement request, string requestName)> topOps,
        Dictionary<string, List<(string folderPath, JsonElement request, string requestName)>> bySubfolder)
    {
        var name = node.TryGetProperty("name", out var n) ? n.GetString() ?? "" : "";
        name = name.Trim();

        if (node.TryGetProperty("request", out var req) && req.ValueKind == JsonValueKind.Object)
        {
            // Direct request: belongs to top-level Api class.
            topOps.Add((topName, req, name));
            return;
        }

        // Subfolder. The "Single <X>" UI rollups fold INTO the parent. So does an
        // empty-named subfolder (Postman includes anonymous spacer folders in some
        // collections — Bluebeam has one in Sessions which holds the two root POST/GET
        // operations).
        bool foldUp = string.IsNullOrEmpty(name)
            || name.StartsWith("Single ", StringComparison.OrdinalIgnoreCase)
            || name.Equals("Job", StringComparison.OrdinalIgnoreCase);  // Jobs/Job rolls into JobsApi

        if (node.TryGetProperty("item", out var sub) && sub.ValueKind == JsonValueKind.Array)
        {
            foreach (var child in sub.EnumerateArray())
            {
                if (foldUp)
                {
                    CollectOpsRecursive(child, topName, topOps, bySubfolder);
                }
                else
                {
                    if (!bySubfolder.TryGetValue(name, out var bucket))
                    {
                        bucket = new List<(string folderPath, JsonElement request, string requestName)>();
                        bySubfolder[name] = bucket;
                    }
                    var nested = new Dictionary<string, List<(string, JsonElement, string)>>(StringComparer.Ordinal);
                    CollectOpsRecursive(child, topName, bucket, nested);
                    // Nested subfolders inside a subfolder (rare for Bluebeam) — we still
                    // fold them up to the immediate parent so the class structure stays flat.
                    foreach (var inner in nested)
                        bucket.AddRange(inner.Value);
                }
            }
        }
    }

    static ParsedApiClass BuildClass(
        string className,
        string ns,
        string displayGroup,
        IReadOnlyList<(string folderPath, JsonElement request, string requestName)> ops,
        string? parentSubfolder)
    {
        var methods = new List<MethodInfo>();
        var dtos = new List<TypeInfo>();
        var seenMethodNames = new HashSet<string>(StringComparer.OrdinalIgnoreCase);

        foreach (var (_, req, opName) in ops)
        {
            var parsed = ParseOperation(req, opName, className, ns, seenMethodNames);
            methods.Add(parsed.Method);
            if (parsed.RequestDto is not null) dtos.Add(parsed.RequestDto);
        }

        var summary = $"Bluebeam Studio API — {displayGroup} endpoints. Auto-generated REST surface; one method per HTTP operation.";
        return new ParsedApiClass(
            ClassName: className,
            Namespace: ns,
            Summary: summary,
            DocUrl: PickClassDocUrl(displayGroup),
            Methods: methods,
            RequestDtos: dtos);
    }

    /// <summary>
    /// Build one MethodInfo from a Postman request. Synthesizes a Request DTO TypeInfo for
    /// the body shape if the request has a `body.raw` JSON example.
    /// </summary>
    static ParsedOperation ParseOperation(
        JsonElement request,
        string requestName,
        string className,
        string classNamespace,
        HashSet<string> seenMethodNames)
    {
        string verb = request.TryGetProperty("method", out var m) ? (m.GetString() ?? "GET").ToUpperInvariant() : "GET";

        // Path reconstruction. Postman's url is sometimes a string, sometimes an object with
        // .path being an array of segments. We accept both.
        var (pathSegments, host) = ExtractUrl(request);
        var fullPath = "/" + string.Join("/", pathSegments);

        // Method name: derived from (verb, last meaningful path segment, optional action suffix).
        // The Postman request names are human prose ("Create metadata for a new Studio Session
        // file.") which produces ugly identifiers. We synthesize idiomatic SDK names instead by
        // looking at the path. Path-based naming is bounded in length, consistent across the
        // surface, and matches what a hand-written client would expose.
        var methodName = SynthesizeMethodNameFromPath(verb, pathSegments);
        // Ensure uniqueness inside the class.
        var finalName = methodName;
        int dedupe = 2;
        while (!seenMethodNames.Add(finalName))
        {
            finalName = $"{methodName}{dedupe}";
            dedupe++;
            if (dedupe > 99) break;  // belt and braces
        }

        // Path parameters. Anything inside `{...}` is one ParamInfo.
        var paramList = new List<ParamInfo>();
        foreach (var seg in pathSegments)
        {
            if (seg.Length >= 3 && seg[0] == '{' && seg[^1] == '}')
            {
                var pname = seg[1..^1];
                paramList.Add(new ParamInfo(
                    name: pname,
                    type: PathTokenTypeMap.TryGetValue(pname, out var pt) ? pt : "string",
                    doc: $"Path parameter `{pname}` from `{fullPath}`.",
                    optional: false,
                    @default: null));
            }
        }

        // Request body: if present, synthesize a DTO type AND add a body param to the method.
        // The DTO name combines the parent class's resource prefix + the method name to
        // disambiguate across classes (e.g. SessionsApi.UpdateSession's body becomes
        // `UpdateSessionRequest` rather than just `UpdateRequest`).
        TypeInfo? bodyDto = null;
        if (request.TryGetProperty("body", out var bodyElem)
            && bodyElem.ValueKind == JsonValueKind.Object
            && bodyElem.TryGetProperty("raw", out var rawElem)
            && rawElem.ValueKind == JsonValueKind.String)
        {
            var rawBody = rawElem.GetString() ?? "";
            if (!string.IsNullOrWhiteSpace(rawBody))
            {
                var dtoName = $"{finalName}Request";
                var (dtoType, dtoProps) = TryParseBodySchema(rawBody, dtoName, classNamespace);
                if (dtoType is not null)
                {
                    bodyDto = dtoType;
                    paramList.Add(new ParamInfo(
                        name: "request",
                        type: dtoName,
                        doc: $"Request body. JSON object with fields: {string.Join(", ", dtoProps)}.",
                        optional: false,
                        @default: null));
                }
                else
                {
                    // Body present but unparseable — fall back to a generic body param.
                    paramList.Add(new ParamInfo(
                        name: "request",
                        type: "object",
                        doc: $"Raw JSON request body (could not auto-derive schema; see vendor docs).",
                        optional: false,
                        @default: null));
                }
            }
        }

        // Summary: the Postman request's "name" field carries a human description that's
        // genuinely useful even though it's prose ("Create metadata for a new Studio Session
        // file." — verbose but accurate). We keep it as the IR's `summary` because it's the
        // only piece of vendor-authored documentation we have for each operation.
        var rawName = CleanString(requestName);
        var summary = string.IsNullOrEmpty(rawName)
            ? $"{verb} {fullPath}"
            : rawName;
        // Return type: undocumented by Bluebeam; treat as void. The collection JSON carries
        // no response shapes, and the prose docs use status-code-only descriptions that don't
        // map cleanly to types.
        ReturnInfo? returns = null;

        var sig = SynthesiseSignature(finalName, returns?.type, paramList);

        // Throws — REST clients commonly raise an HTTP exception on non-2xx. Capture that as
        // the single canonical exception so the IR exposes the failure mode.
        var throws = new List<ThrowsInfo>
        {
            new("HttpRequestException", "Thrown when the HTTP call returns a non-2xx status.")
        };

        // Per-method doc URL: point at the Postman documentation page. The Postman web
        // renderer doesn't carry stable per-operation anchors so we use the class-level URL
        // (one anchor per *Api class is the highest fidelity we can achieve).
        var doc_url = PickClassDocUrl(className);

        var method = new MethodInfo(
            name: finalName,
            signature: sig,
            summary: summary,
            remarks: $"HTTP {verb} `{fullPath}`. Vendor docs page: {DevPortalRoot}",
            @params: paramList,
            returns: returns,
            throws: throws,
            events_related: new List<string>(),
            doc_url: doc_url,
            since: null,
            deprecated: null);

        return new ParsedOperation(method, bodyDto);
    }

    static (IReadOnlyList<string> segments, string host) ExtractUrl(JsonElement request)
    {
        if (!request.TryGetProperty("url", out var url))
            return (Array.Empty<string>(), "");
        if (url.ValueKind == JsonValueKind.String)
        {
            var s = url.GetString() ?? "";
            // Parse loosely; we only need the path portion for type reasoning.
            try
            {
                var u = new Uri(s, UriKind.Absolute);
                return (u.AbsolutePath.TrimStart('/').Split('/'), u.Host);
            }
            catch
            {
                return (s.TrimStart('/').Split('/'), "");
            }
        }
        if (url.ValueKind == JsonValueKind.Object)
        {
            var segs = new List<string>();
            if (url.TryGetProperty("path", out var p) && p.ValueKind == JsonValueKind.Array)
            {
                foreach (var seg in p.EnumerateArray())
                {
                    if (seg.ValueKind == JsonValueKind.String) segs.Add(seg.GetString() ?? "");
                }
            }
            string host = "";
            if (url.TryGetProperty("host", out var h) && h.ValueKind == JsonValueKind.Array)
            {
                var hostParts = new List<string>();
                foreach (var hp in h.EnumerateArray())
                {
                    if (hp.ValueKind == JsonValueKind.String) hostParts.Add(hp.GetString() ?? "");
                }
                host = string.Join(".", hostParts);
            }
            return (segs, host);
        }
        return (Array.Empty<string>(), "");
    }

    /// <summary>
    /// Try to parse a raw JSON body example into a synthesized DTO TypeInfo. Returns
    /// (null, []) on parse failure or empty body. Each top-level property becomes one
    /// PropertyInfo with type inferred from the example value.
    /// </summary>
    public static (TypeInfo? dto, List<string> propNames) TryParseBodySchema(string rawBody, string dtoName, string classNamespace)
    {
        try
        {
            // The Postman JSON body is wrapped as an escaped string — parse it as a JSON
            // document independently. Empty objects are common (`{}`) and produce a DTO with
            // 0 properties; we still emit it so the type referenced by ParamInfo exists.
            using var bodyDoc = JsonDocument.Parse(rawBody);
            var bodyRoot = bodyDoc.RootElement;
            if (bodyRoot.ValueKind != JsonValueKind.Object)
            {
                return (null, new List<string>());
            }
            var props = new List<PropertyInfo>();
            var propNames = new List<string>();
            foreach (var prop in bodyRoot.EnumerateObject())
            {
                var pname = prop.Name;
                var ptype = InferType(prop.Value);
                propNames.Add(pname);
                props.Add(new PropertyInfo(
                    name: pname,
                    type: ptype,
                    getter: true,
                    setter: true,
                    summary: $"Request-body field `{pname}` (type inferred from example).",
                    remarks: null,
                    doc_url: CollectionJsonUrl));
            }
            // Place the synthesized DTO in a sub-namespace `.Models` so it doesn't collide
            // with the *Api class names.
            var ns = $"{classNamespace}.Models";
            var dto = new TypeInfo(
                name: dtoName,
                @namespace: ns,
                kind: "class",
                summary: $"Request body DTO for the corresponding `{dtoName.Substring(0, dtoName.Length - "Request".Length)}` operation. Synthesized from the Postman example schema.",
                remarks: null,
                @base: null,
                interfaces: new List<string>(),
                doc_url: CollectionJsonUrl,
                constructors: new List<MethodInfo>(),
                methods: new List<MethodInfo>(),
                properties: props,
                events: new List<EventInfo>(),
                enum_values: new List<EnumValueInfo>(),
                delegate_signature: null);
            return (dto, propNames);
        }
        catch (JsonException)
        {
            return (null, new List<string>());
        }
    }

    /// <summary>
    /// Map a JSON value to a .NET-style type string. The Postman example values are
    /// indicative (`""`, `0`, `0.0`, `true`, `[]`, `{}`) so the inference is broad-strokes
    /// but matches what a hand-written SDK would emit.
    /// </summary>
    static string InferType(JsonElement v) => v.ValueKind switch
    {
        JsonValueKind.String => "string",
        JsonValueKind.Number => v.TryGetInt64(out _) ? "int" : "double",
        JsonValueKind.True or JsonValueKind.False => "bool",
        JsonValueKind.Array => InferArrayType(v),
        JsonValueKind.Object => "object",
        JsonValueKind.Null => "object",
        _ => "object",
    };

    static string InferArrayType(JsonElement arr)
    {
        // Look at the first non-null element to guess T in List<T>.
        foreach (var el in arr.EnumerateArray())
        {
            if (el.ValueKind == JsonValueKind.Null) continue;
            return $"List<{InferType(el)}>";
        }
        return "List<object>";
    }

    /// <summary>
    /// Synthesize an idiomatic SDK method name from a REST verb + path. Names follow the
    /// pattern verb-action + last-meaningful-resource. Examples:
    ///   POST   /sessions/{sessionId}/files           → CreateFile
    ///   GET    /sessions/{sessionId}/files           → ListFiles
    ///   GET    /sessions/{sessionId}/files/{id}      → GetFile
    ///   POST   /sessions/{sessionId}/files/{id}/snapshot          → CreateFileSnapshot
    ///   GET    /sessions/{sessionId}/files/{id}/snapshot          → GetFileSnapshot
    ///   POST   /sessions/{sessionId}/files/{id}/confirm-upload    → ConfirmFileUpload
    ///   POST   /sessions/{sessionId}/files/{id}/checkin           → CheckinFile
    ///   POST   /sessions/{sessionId}/invite                       → InviteSession
    ///   POST   /projects/{projectId}/files/{id}/checkout-to-session → CheckoutFileToSession
    ///   GET    /publicapi/api/version                              → GetVersion
    ///   GET    /publicapi/v1/users/me                              → GetMe (action token "me")
    ///   PUT    /publicapi/v1/users/me                              → UpdateMe
    /// </summary>
    static string SynthesizeMethodNameFromPath(string verb, IReadOnlyList<string> pathSegments)
    {
        // Skip the path prefix segments (publicapi/v1 or publicapi/api). What's left is the
        // resource path itself.
        var meaningful = new List<string>();
        bool sawV1 = false;
        foreach (var seg in pathSegments)
        {
            if (string.IsNullOrEmpty(seg)) continue;
            if (!sawV1)
            {
                // Skip "publicapi", "v1", "api" prefix tokens — they're not resource segments.
                if (seg == "publicapi" || seg == "v1" || seg == "api") { sawV1 = sawV1 || seg == "v1" || seg == "api"; continue; }
                // Resource segment encountered before v1/api — keep it.
                sawV1 = true;
            }
            meaningful.Add(seg);
        }
        if (meaningful.Count == 0) meaningful.Add("Root");

        // Identify the trailing "action" segment (a non-path-param tail like "snapshot",
        // "confirm-upload", "invite"). If the last segment is a path param like {id} we use
        // the segment before it as the "resource" and recognise there's no extra action.
        bool isParam(string s) => s.Length >= 3 && s[0] == '{' && s[^1] == '}';

        // Trailing action: tail segment that's NOT a path param. If the segment immediately
        // before is also a non-param (e.g. .../files/{id}/snapshot), we treat that as the
        // action verb. If the tail IS a param (e.g. .../files/{id}), there's no extra action.
        string? action = null;
        // Scan from the tail backwards for the first non-param.
        int tailIdx = meaningful.Count - 1;
        if (!isParam(meaningful[tailIdx]))
        {
            // Look for a resource segment immediately before the action.
            // .../sessions/{sessionId}/files/{id}/snapshot  → resource=files, action=snapshot
            // .../sessions/{sessionId}/files                → resource=files, action=null
            // ...projects/healthcheck/details               → resource=healthcheck, action=details
            // ...users/me                                   → resource=users, action=me
            action = meaningful[tailIdx];
        }

        // Resource: the last non-param segment that ISN'T the action.
        string? resource = null;
        for (int i = meaningful.Count - 1; i >= 0; i--)
        {
            var seg = meaningful[i];
            if (isParam(seg)) continue;
            if (action != null && seg == action && i == meaningful.Count - 1) continue;
            resource = seg;
            break;
        }
        resource ??= meaningful[0];

        // Generate the verb prefix from the HTTP verb + the path morphology.
        // GET: List (collection — tail is a non-param resource) or Get (item — tail is a param)
        // POST: Create (or specialised name if action token present)
        // PUT: Update (or specialised name if action token present)
        // DELETE: Delete
        bool tailIsParam = isParam(meaningful[^1]);
        bool hasActionTail = action != null && action != resource;

        string verbPrefix;
        bool useResourcePluralAsIs = false;
        if (verb == "GET")
        {
            if (tailIsParam)
            {
                verbPrefix = "Get";
            }
            else if (hasActionTail)
            {
                verbPrefix = "Get";   // GET /...files/{id}/snapshot → GetFileSnapshot
            }
            else
            {
                verbPrefix = "List";   // GET /...files → ListFiles
                useResourcePluralAsIs = true;
            }
        }
        else if (verb == "POST")
        {
            if (hasActionTail)
            {
                // POST /...files/{id}/snapshot → CreateFileSnapshot (the action IS the new resource)
                // POST /...sessions/{sessionId}/invite → InviteSession (the action IS a verb)
                // POST /...sessions/{sessionId}/files → CreateSessionFile (sub-collection create)
                // Treat action as a verb if it's a known imperative; otherwise check if it's a
                // sub-collection plural and use "Create<Parent><Singular>"; else fall back to
                // a "Create..." synth treating the action as the new resource.
                var actionAsVerb = ActionAsImperative(action!);
                if (actionAsVerb != null)
                {
                    // "Invite" / "Checkin" / etc. — action becomes the verb prefix.
                    return CombineMethodName(actionAsVerb, ToPascal(SingularizeResource(resource)));
                }
                if (IsCollectionPlural(action!))
                {
                    // POST .../sessions/{sessionId}/files → CreateSessionFile
                    return CombineMethodName(
                        "Create",
                        ToPascal(SingularizeResource(resource)) + ToPascal(SingularizeResource(action!)));
                }
                verbPrefix = "Create";   // POST .../snapshot → CreateSnapshot (resource=files, action=snapshot)
            }
            else
            {
                verbPrefix = "Create";
            }
        }
        else if (verb == "PUT")
        {
            if (hasActionTail)
            {
                var actionAsVerb = ActionAsImperative(action!);
                if (actionAsVerb != null)
                {
                    return CombineMethodName(actionAsVerb, ToPascal(SingularizeResource(resource)));
                }
                verbPrefix = "Update";
            }
            else
            {
                verbPrefix = "Update";
            }
        }
        else if (verb == "DELETE")
        {
            verbPrefix = "Delete";
        }
        else
        {
            verbPrefix = ToPascal(verb.ToLowerInvariant());
        }

        // Synthesise the final method name. Strategy:
        //   - If the tail is an action different from the resource: <Verb><Resource><Action>
        //   - If the tail is a param: <Verb><Resource> (singularised)
        //   - If the tail is the resource (no action): <Verb><Resource>
        string resourcePart;
        if (hasActionTail)
        {
            // POST .../files/{id}/snapshot → CreateFileSnapshot
            // GET  .../files/{id}/snapshot → GetFileSnapshot
            // POST .../files/{id}/confirm-upload → ConfirmFileUpload (action treated as imperative? No — `confirm-upload` is a verb-phrase, leave to specialised mapping)
            // Heuristic: prepend the resource (singular) then the action (PascalCase'd).
            // For verbs we already returned earlier (Invite/Checkin) — those bypass this code path.
            // For POST/PUT/DELETE the action token becomes the suffix on a singular resource.
            // For GET on a sub-collection (action token is a plural noun like "files" / "users"
            // / "permissions" — the path's true tail), we treat that as the resource itself and
            // use `List` semantics: GET /sessions/{sessionId}/files → ListSessionFiles.
            if (verb == "GET" && tailIsParam == false && IsCollectionPlural(action!))
            {
                // The action is actually the collection — rewrite to "List<Parent><Collection>".
                return CombineMethodName(
                    "List",
                    ToPascal(SingularizeResource(resource)) + ToPascal(action!));
            }
            resourcePart = ToPascal(SingularizeResource(resource)) + ToPascal(action!);
        }
        else if (tailIsParam)
        {
            // singular: /files/{id} → File
            resourcePart = ToPascal(SingularizeResource(resource));
        }
        else
        {
            // tail IS the resource (collection level): use plural verbatim for List, singular for Create/Update/Delete
            resourcePart = useResourcePluralAsIs
                ? ToPascal(resource)
                : ToPascal(SingularizeResource(resource));
        }

        return CombineMethodName(verbPrefix, resourcePart);
    }

    /// <summary>
    /// Recognise common English plural collection tokens in Bluebeam paths. Used to detect
    /// when a "trailing action" segment is actually a sub-collection name rather than a
    /// resource-level action verb. Used during method-name synthesis to pick `List...` over
    /// `Get...Files` for GET collection endpoints.
    /// </summary>
    static bool IsCollectionPlural(string token) =>
        token.Equals("files", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("folders", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("users", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("activities", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("permissions", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("revisions", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("sharedlinks", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("items", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("results", StringComparison.OrdinalIgnoreCase) ||
        token.Equals("jobs", StringComparison.OrdinalIgnoreCase);

    static string CombineMethodName(string verb, string resource)
    {
        // The verb part is already PascalCase; the resource may start with a non-letter (rare).
        if (string.IsNullOrEmpty(resource)) return verb;
        return verb + resource;
    }

    /// <summary>
    /// Map an action token to a verb if it's an imperative. The trailing path segment for
    /// some Bluebeam ops carries the verb (e.g. `invite`, `checkin`). Returning a non-null
    /// value means "use this as the method's verb prefix instead of the HTTP verb".
    /// </summary>
    static string? ActionAsImperative(string action)
    {
        var s = action.ToLowerInvariant();
        return s switch
        {
            "invite"               => "Invite",
            "checkin"              => "Checkin",
            "checkout"             => "Checkout",
            "copy"                 => "Copy",
            "cancel"               => "Cancel",
            "restore"              => "Restore",
            "confirm-upload"       => "ConfirmUpload",
            "confirm-checkin"      => "ConfirmCheckin",
            "checkout-to-session"  => "CheckoutToSession",
            "snapshot"             => null,    // snapshot is a NOUN here (the result), not a verb — bypass mapping
            _ => null
        };
    }

    /// <summary>
    /// Convert a plural-or-already-singular noun to its singular form. The Bluebeam path
    /// vocabulary uses simple English plurals (sessions, projects, files, folders, users,
    /// permissions, activities, jobs, results, revisions, sharedlinks); a regex-light
    /// approach handles these.
    /// </summary>
    static string SingularizeResource(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        var lower = s.ToLowerInvariant();
        return lower switch
        {
            "sessions"      => "Session",
            "projects"      => "Project",
            "files"         => "File",
            "folders"       => "Folder",
            "users"         => "User",
            "permissions"   => "Permissions",   // permissions is collective; keep plural
            "activities"    => "Activity",
            "jobs"          => "Job",
            "results"       => "Result",
            "revisions"     => "Revision",
            "sharedlinks"   => "SharedLink",
            "items"         => "Item",
            "by-path"       => "ByPath",
            "healthcheck"   => "Healthcheck",
            "me"            => "Me",
            "version"       => "Version",
            "details"       => "Details",
            _ => ToPascal(s)
        };
    }

    static string ToPascal(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        // Hyphenated tokens become PascalCase per word: "confirm-upload" → "ConfirmUpload"
        var parts = s.Split('-');
        var sb = new System.Text.StringBuilder();
        foreach (var p in parts)
        {
            if (string.IsNullOrEmpty(p)) continue;
            sb.Append(char.ToUpperInvariant(p[0]));
            if (p.Length > 1) sb.Append(p.Substring(1));
        }
        return sb.ToString();
    }

    /// <summary>
    /// Map a Postman request `name` (human prose) into a PascalCase C#-style method name.
    /// Strips trailing punctuation, splits on whitespace, capitalises each word, and drops
    /// stop-words that don't add identifier value ("a", "the", "in", "for", "of", "to").
    /// Prepends the verb if the resulting name would be empty.
    ///
    /// NOTE: this is kept as a fallback used only if the path-based synthesis is empty
    /// (extremely unlikely). The main entry point is SynthesizeMethodNameFromPath.
    /// </summary>
    static string ToMethodName(string rawName, string verb)
    {
        if (string.IsNullOrWhiteSpace(rawName)) return $"{ToTitle(verb.ToLowerInvariant())}Request";
        var stripped = WordSplit(rawName);
        // Filter stop-words.
        var stop = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { "a", "an", "the", "in", "for", "of", "to", "from", "by", "and", "or", "via" };
        var words = stripped.Where(w => !stop.Contains(w)).ToList();
        if (words.Count == 0) return $"{ToTitle(verb.ToLowerInvariant())}Request";
        // Map "Studio" + "Session"/"Project"/"Sessions" combinations: the request names verbose them
        // out ("Create a Studio Session") but the SDK shape should be terse ("CreateSession"). We
        // keep the prose verbatim though — uniqueness within the class is what matters.
        var sb = new System.Text.StringBuilder();
        foreach (var w in words)
        {
            sb.Append(ToTitle(w));
        }
        var name = sb.ToString();
        // Special-case identifier-illegal leading characters.
        if (name.Length > 0 && char.IsDigit(name[0])) name = "_" + name;
        return name;
    }

    static List<string> WordSplit(string s)
    {
        var words = new List<string>();
        var current = new System.Text.StringBuilder();
        foreach (var ch in s)
        {
            if (char.IsLetterOrDigit(ch)) current.Append(ch);
            else
            {
                if (current.Length > 0) { words.Add(current.ToString()); current.Clear(); }
            }
        }
        if (current.Length > 0) words.Add(current.ToString());
        return words;
    }

    static string ToTitle(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        if (s.Length == 1) return s.ToUpperInvariant();
        return char.ToUpperInvariant(s[0]) + s.Substring(1).ToLowerInvariant();
    }

    static string CleanString(string s)
    {
        if (string.IsNullOrEmpty(s)) return s;
        return s.Replace("\r", " ").Replace("\n", " ").Replace("\t", " ").Trim().TrimEnd('.', ' ');
    }

    /// <summary>
    /// Build the canonical synthesised method signature in the SDK's idiomatic shape:
    ///   Task[&lt;ReturnType&gt;] Name(Type1 p1, Type2 p2)
    /// Bluebeam's docs only describe async-friendly REST calls; the SDK we're modelling is
    /// notional but follows the same convention as IDEA Statica / typical .NET REST clients.
    /// </summary>
    static string SynthesiseSignature(string name, string? returnType, IReadOnlyList<ParamInfo> @params)
    {
        var sb = new System.Text.StringBuilder();
        if (returnType is null) sb.Append("Task ");
        else sb.Append("Task<").Append(returnType).Append("> ");
        sb.Append(name).Append('(');
        for (int i = 0; i < @params.Count; i++)
        {
            if (i > 0) sb.Append(", ");
            sb.Append(@params[i].type).Append(' ').Append(@params[i].name);
        }
        sb.Append(')');
        return sb.ToString();
    }

    static string NormaliseClassName(string s)
    {
        // Strip spaces / punctuation, PascalCase the result.
        var sb = new System.Text.StringBuilder();
        bool upperNext = true;
        foreach (var ch in s)
        {
            if (char.IsLetterOrDigit(ch))
            {
                sb.Append(upperNext ? char.ToUpperInvariant(ch) : ch);
                upperNext = false;
            }
            else
            {
                upperNext = true;
            }
        }
        return sb.ToString();
    }

    static string PickClassDocUrl(string classOrGroupName)
    {
        // We anchor every type's `doc_url` on the canonical Postman v2.1 JSON URL itself.
        // The collection JSON is the only public machine-readable source for the Studio
        // API (the developer portal is auth-walled). Two reasons this is the right choice:
        //
        //   1. Provenance — readers can verify what we extracted by fetching the same URL.
        //   2. Verification — codex-rescue's strict-verify can dispatch on `.json` URLs
        //      and structurally check the catalog against the JSON (paths, body keys).
        //
        // The Postman web renderer at postman.com/.../studio-api gives a friendlier human
        // view of the same content, but its URL doesn't carry stable per-folder anchors,
        // so deep-linking to it would just duplicate the JSON URL with a worse fragment.
        // The Bluebeam Studio Session Guide at support.bluebeam.com/.../studio-session-guide
        // is the closest thing to a human-readable spec for the Sessions surface, but it
        // covers only a fraction of the 123 operations. We surface it in `remarks` and
        // EXTRACTION-NOTES.md rather than overloading doc_url with it.
        return CollectionJsonUrl;
    }
}
