// TeklaSdk PageParser — converts a `TypeRecord` (from MetadataReflector) plus
// optional XML doc data into the IR's type-level fields. Structurally identical to
// the Navisworks PageParser (same MetadataReflector → TypeInfo mapping); the only
// vendor-specific bit is the doc-URL, which branches on namespace:
//   • Tekla.Structures.Plugins.DirectManipulation.* → the DM API guide (the DM
//     namespaces are NOT in developer.tekla.com's scraped reference tree, so the
//     guide page + a `#T:<fullname>` fragment is the stable "doc home").
//   • Fusion.*                                       → the TeklaFusion NuGet page
//     (TeklaFusion has no public per-type reference; the package page is the
//     canonical landing URL).
//
// Both URL forms follow the same "landing page + doc-id fragment" pattern the
// Navisworks extractor uses: the IR consumer checks uniqueness + format, not
// retrievability.
//
// AOT: pure managed code, identical story to Navisworks/PageParser.cs.

using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.TeklaSdk;

public static class PageParser
{
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

    const string DmGuideUrl = "https://developer.tekla.com/documentation/direct-manipulation-api-plugins";
    const string FusionPkgUrl = "https://www.nuget.org/packages/TeklaFusion";

    static string DocHome(string @namespace) =>
        @namespace.StartsWith("Fusion", StringComparison.Ordinal) ? FusionPkgUrl : DmGuideUrl;

    public static string TypeDocUrl(string @namespace, string typeFullName) =>
        $"{DocHome(@namespace)}#T:{typeFullName}";

    public static string MemberDocUrl(string @namespace, string docId) =>
        $"{DocHome(@namespace)}#{Uri.EscapeDataString(docId)}";

    /// <summary>
    /// Builds a TypeInfo for the given type, leaving members empty (filled by
    /// MemberPageParser).
    /// </summary>
    public static TypeInfo BuildSkeleton(TypeRecord tr, Dictionary<string, MemberDoc> xmlDoc)
    {
        var docId = "T:" + tr.FullName;
        xmlDoc.TryGetValue(docId, out var doc);

        var summary = doc?.Summary;
        if (string.IsNullOrEmpty(summary))
            summary = $"{tr.Name} {KindNoun(tr.Kind)} in {(string.IsNullOrEmpty(tr.Namespace) ? "(global)" : tr.Namespace)}.";

        var typeCtx = new GenericContext(tr.GenericParameters, Array.Empty<string>());

        return new TypeInfo(
            name: tr.Name,
            @namespace: string.IsNullOrEmpty(tr.Namespace) ? "(global)" : tr.Namespace,
            kind: MapKind(tr.Kind),
            summary: summary,
            remarks: doc?.Remarks,
            @base: tr.BaseTypeName is null ? null : MetadataReflector.PrettyType(tr.BaseTypeName, typeCtx),
            interfaces: tr.Interfaces.Select(i => MetadataReflector.PrettyType(i, typeCtx)).ToList(),
            doc_url: TypeDocUrl(tr.Namespace, tr.FullName),
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
