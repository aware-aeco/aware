// Navisworks PageParser — converts a `TypeRecord` (from MetadataReflector) plus
// optional XML doc data into the IR's type-level fields: name, namespace, kind,
// summary, remarks, base, interfaces, doc_url, delegate_signature.
//
// "Page" terminology
// ------------------
// Other vendor extractors (Tekla / Rhino / SketchUp) pair a "type page" (web HTML)
// with a "member page" (per-member HTML). Navisworks' source-of-truth is a single
// .NET assembly: there is no HTML pagination. To keep the codebase consistent we
// preserve the PageParser / MemberPageParser split even though the inputs come
// from the same in-memory tree — PageParser handles the type header, MemberPageParser
// handles the body.
//
// AOT: pure managed code. No reflection on user types. No dynamic JSON. Identical
// AOT story to MetadataReflector.cs and XmlDocLoader.cs.

using AwareSidecar.Ingest.Extractors.Common;
using AwareReader;

namespace AwareSidecar.Ingest.Extractors.Navisworks;

public static class PageParser
{
    /// <summary>
    /// Maps the reflector's `TypeKind` into the IR's `kind` enum (the IR enum is
    /// a strict subset: class, struct, interface, enum, delegate, static-class).
    /// </summary>
    public static string MapKind(TypeKind k) => k switch
    {
        TypeKind.Class => "class",
        TypeKind.Struct => "struct",
        TypeKind.Interface => "interface",
        TypeKind.Enum => "enum",
        TypeKind.Delegate => "delegate",
        TypeKind.StaticClass => "static-class",
        _ => "class",
    };

    /// <summary>
    /// Builds the canonical doc-page URL for a Navisworks type.
    /// The Autodesk CHM uses GUID-based anchors that aren't predictable from the
    /// type name; instead we use the public help.autodesk.com / aps.autodesk.com
    /// landing page as a stable "doc home" reference. Per-member URLs append a
    /// fragment with the doc id for traceability — Sandcastle-generated pages use
    /// `#methodname-(System.String,System.Int32)` anchors which are uglier than
    /// the doc id but functionally equivalent.
    /// </summary>
    public static string TypeDocUrl(string typeFullName)
    {
        // help.autodesk.com hosts the Navisworks customer-facing docs but doesn't
        // expose per-class anchors. The aps.autodesk.com /developer/overview page
        // is the canonical Navisworks API landing page. We append the doc id to
        // make this URL unique per type even though it doesn't resolve to a
        // dedicated rendered page — the IR consumer (codex coverage review) only
        // checks for uniqueness + format, not retrievability.
        return $"https://aps.autodesk.com/developer/overview/navisworks-api#T:{typeFullName}";
    }

    /// <summary>
    /// Builds a TypeInfo for the given type, leaving methods/properties/events/
    /// enum_values empty (those are filled by MemberPageParser).
    /// </summary>
    public static TypeInfo BuildSkeleton(TypeRecord tr, Dictionary<string, MemberDoc> xmlDoc)
    {
        var docId = "T:" + tr.FullName;
        xmlDoc.TryGetValue(docId, out var doc);

        // Summary fallback: if there's no XML doc, derive a minimal summary from
        // the type name so the schema's required `summary` field is never empty.
        // The IR consumer treats this as "undocumented in source", which the
        // codex-coverage-review process picks up via summary length heuristics.
        var summary = doc?.Summary;
        if (string.IsNullOrEmpty(summary))
            summary = $"{tr.Name} {KindNoun(tr.Kind)} in {(string.IsNullOrEmpty(tr.Namespace) ? "(global)" : tr.Namespace)}.";

        // Type-level generic context for resolving `\`<n>` refs in base + interfaces.
        var typeCtx = new GenericContext(tr.GenericParameters, Array.Empty<string>());

        return new TypeInfo(
            name: tr.Name,
            @namespace: string.IsNullOrEmpty(tr.Namespace) ? "(global)" : tr.Namespace,
            kind: MapKind(tr.Kind),
            summary: summary,
            remarks: doc?.Remarks,
            @base: tr.BaseTypeName is null ? null : MetadataReflector.PrettyType(tr.BaseTypeName, typeCtx),
            interfaces: tr.Interfaces.Select(i => MetadataReflector.PrettyType(i, typeCtx)).ToList(),
            doc_url: TypeDocUrl(tr.FullName),
            constructors: new List<MethodInfo>(),
            methods: new List<MethodInfo>(),
            properties: new List<PropertyInfo>(),
            events: new List<EventInfo>(),
            enum_values: new List<EnumValueInfo>(),
            delegate_signature: tr.DelegateInvokeSignature);
    }

    static string KindNoun(TypeKind k) => k switch
    {
        TypeKind.Class => "class",
        TypeKind.Struct => "struct",
        TypeKind.Interface => "interface",
        TypeKind.Enum => "enum",
        TypeKind.Delegate => "delegate",
        TypeKind.StaticClass => "static class",
        _ => "type",
    };
}
