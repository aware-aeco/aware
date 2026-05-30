// TeklaSdk MemberPageParser — fills MethodInfo / PropertyInfo / EventInfo /
// EnumValueInfo records for a single type from the MetadataReflector tree + the
// XmlDocLoader dictionary. Structurally identical to Navisworks/MemberPageParser;
// the only vendor-specific bit is the per-member doc-URL (namespace-aware, via
// PageParser.MemberDocUrl).
//
// XML-doc availability differs per package:
//   • DM (Tekla.Structures.Plugins.DirectManipulation) ships an XML doc → rich
//     summaries/remarks/params/returns.
//   • TeklaFusion ships NO XML doc → the xmlDoc dictionary is empty and every
//     member falls back to a generated "<name> <kind> of <Type>" summary. That is
//     the correct, honest signatures-only behaviour (decompile-summarisation of
//     the proprietary Fusion assembly is disallowed by license-aware-extraction).
//
// AOT: pure managed code, identical story to Navisworks/MemberPageParser.cs.

using AwareSidecar.Ingest.Extractors.Common;
using AwareReader;

namespace AwareSidecar.Ingest.Extractors.TeklaSdk;

public static class MemberPageParser
{
    public static void Fill(TypeInfo skeleton, TypeRecord tr, Dictionary<string, MemberDoc> xmlDoc)
    {
        var typeCtx = new GenericContext(tr.GenericParameters, Array.Empty<string>());

        foreach (var ctor in tr.Constructors)
        {
            xmlDoc.TryGetValue(ctor.DocId, out var doc);
            skeleton.constructors.Add(BuildMethodInfo(ctor, doc, tr));
        }

        foreach (var m in tr.Methods)
        {
            xmlDoc.TryGetValue(m.DocId, out var doc);
            skeleton.methods.Add(BuildMethodInfo(m, doc, tr));
        }

        foreach (var p in tr.Properties)
        {
            xmlDoc.TryGetValue(p.DocId, out var doc);
            var summary = doc?.Summary;
            if (string.IsNullOrEmpty(summary)) summary = doc?.Value;
            if (string.IsNullOrEmpty(summary)) summary = $"{p.Name} property of {tr.Name}.";

            skeleton.properties.Add(new PropertyInfo(
                name: p.Name,
                type: MetadataReflector.PrettyType(p.Type, typeCtx),
                getter: p.HasGetter,
                setter: p.HasSetter,
                summary: summary,
                remarks: doc?.Remarks,
                doc_url: PageParser.MemberDocUrl(tr.Namespace, p.DocId)));
        }

        foreach (var ev in tr.Events)
        {
            xmlDoc.TryGetValue(ev.DocId, out var doc);
            var summary = doc?.Summary ?? $"{ev.Name} event of {tr.Name}.";

            var delegatePretty = MetadataReflector.PrettyType(ev.DelegateType, typeCtx);
            var signature = $"event {delegatePretty} {ev.Name}";

            skeleton.events.Add(new EventInfo(
                name: ev.Name,
                @delegate: delegatePretty,
                signature: signature,
                summary: summary,
                fires_when: doc?.Remarks,
                doc_url: PageParser.MemberDocUrl(tr.Namespace, ev.DocId),
                handler_thread: "unknown",
                interacts_with: new List<string>()));
        }

        if (tr.Kind == TypeKind.Enum)
        {
            foreach (var f in tr.Fields)
            {
                if (!f.IsLiteral) continue;
                xmlDoc.TryGetValue(f.DocId, out var doc);

                System.Text.Json.JsonElement valueElem;
                if (f.ConstantValue is int i)
                    valueElem = System.Text.Json.JsonSerializer.SerializeToElement(i, IrJsonContext.Default.Int32);
                else if (f.ConstantValue is long l && l >= int.MinValue && l <= int.MaxValue)
                    valueElem = System.Text.Json.JsonSerializer.SerializeToElement((int)l, IrJsonContext.Default.Int32);
                else if (f.ConstantValue is uint u && u <= int.MaxValue)
                    valueElem = System.Text.Json.JsonSerializer.SerializeToElement((int)u, IrJsonContext.Default.Int32);
                else
                    valueElem = System.Text.Json.JsonSerializer.SerializeToElement(
                        f.ConstantValue?.ToString() ?? "0",
                        IrJsonContext.Default.String);

                skeleton.enum_values.Add(new EnumValueInfo(
                    name: f.Name,
                    value: valueElem,
                    summary: doc?.Summary));
            }
        }
    }

    static MethodInfo BuildMethodInfo(MethodRecord m, MemberDoc? doc, TypeRecord tr)
    {
        var summary = doc?.Summary;
        if (string.IsNullOrEmpty(summary))
            summary = m.IsConstructor
                ? $"Constructs a new {tr.Name}."
                : $"{m.Name} method of {tr.Name}.";

        var ctx = new GenericContext(tr.GenericParameters, m.GenericParameters);

        var paramInfos = new List<ParamInfo>();
        foreach (var p in m.Parameters)
        {
            string? pdocVal = null;
            if (doc != null && doc.Params.TryGetValue(p.Name, out var pdoc2))
                pdocVal = pdoc2;

            paramInfos.Add(new ParamInfo(
                name: p.Name,
                type: MetadataReflector.PrettyType(p.Type, ctx),
                doc: pdocVal,
                optional: p.IsOptional,
                @default: p.DefaultValueLiteral));
        }

        ReturnInfo? returns = null;
        if (!m.IsConstructor && m.ReturnType != "System.Void")
        {
            var returnsDoc = doc?.Returns ?? "";
            returns = new ReturnInfo(
                type: MetadataReflector.PrettyType(m.ReturnType, ctx),
                doc: returnsDoc);
        }

        var throwsList = new List<ThrowsInfo>();
        if (doc != null)
        {
            foreach (var ex in doc.Exceptions)
            {
                throwsList.Add(new ThrowsInfo(
                    type: MetadataReflector.PrettyType(ex.TypeRef, ctx),
                    when: ex.Description));
            }
        }

        return new MethodInfo(
            name: m.Name,
            signature: m.Signature,
            summary: summary,
            remarks: doc?.Remarks,
            @params: paramInfos,
            returns: returns,
            throws: throwsList,
            events_related: new List<string>(),
            doc_url: PageParser.MemberDocUrl(tr.Namespace, m.DocId),
            since: null,
            deprecated: null);
    }
}
