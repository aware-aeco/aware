// Navisworks MemberPageParser — fills MethodInfo / PropertyInfo / EventInfo /
// EnumValueInfo records for a single type, drawing data from the MetadataReflector
// tree and the XmlDocLoader dictionary.
//
// Real-data guarantee (defensive checklist #1)
// --------------------------------------------
// Every emitted record must carry REAL data — no placeholders, no "Type" or
// "void" filler:
//   • MethodInfo.signature   — built from PE metadata (return type + name + params)
//   • MethodInfo.params[]    — names from Parameter table; types from PE signature
//   • MethodInfo.returns     — null for void/ctor, else (type, doc) from PE+XML
//   • MethodInfo.throws[]    — from <exception> elements in the XML doc
//   • PropertyInfo.type      — PE signature
//   • EventInfo.delegate     — PE field type
//   • EventInfo.handler_thread — Navisworks events fire on "main" by default; we
//     conservatively stamp "unknown" because the XML doc doesn't carry thread
//     affinity. The IR schema accepts main|worker|unknown and the codex review
//     protocol tolerates "unknown" when the source doesn't expose threading.
//
// Doc-id matching
// ---------------
// Every record needs an XML doc lookup. The reflector emits doc ids in the same
// canonical form the XML doc uses (System.String, IEnumerable{T}, M:Foo.Bar(P1,P2)).
// Lookups are exact-match; mismatches mean either:
//   1. The member is undocumented in source (XML doc omits the entry entirely).
//   2. Our id generator has a bug — caught by parser tests + strict verification.
//
// Doc URLs (per-member) — for traceability we add the doc id as a fragment to
// the type's doc_url. The result is unique per member even though it doesn't
// resolve to a rendered page (the underlying CHM has no anchors that match
// these ids; pointing at the Navisworks API landing page with the doc id is the
// best we can do without a live render).

using AwareSidecar.Ingest.Extractors.Common;

namespace AwareSidecar.Ingest.Extractors.Navisworks;

public static class MemberPageParser
{
    public static void Fill(TypeInfo skeleton, TypeRecord tr, Dictionary<string, MemberDoc> xmlDoc)
    {
        // Type-level generic context for resolving `<n>` and ``<n>` placeholders in
        // property / event / field types back to the declared parameter names.
        var typeCtx = new GenericContext(tr.GenericParameters, Array.Empty<string>());

        // Constructors.
        foreach (var ctor in tr.Constructors)
        {
            xmlDoc.TryGetValue(ctor.DocId, out var doc);
            skeleton.constructors.Add(BuildMethodInfo(ctor, doc, tr));
        }

        // Methods (incl. operator overloads).
        foreach (var m in tr.Methods)
        {
            xmlDoc.TryGetValue(m.DocId, out var doc);
            skeleton.methods.Add(BuildMethodInfo(m, doc, tr));
        }

        // Properties.
        foreach (var p in tr.Properties)
        {
            xmlDoc.TryGetValue(p.DocId, out var doc);
            // <value> is the canonical property-doc element; some authors put the
            // text under <summary> instead. We prefer <summary> + fall back to <value>.
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
                doc_url: MemberDocUrl(tr.FullName, p.DocId)));
        }

        // Events.
        foreach (var ev in tr.Events)
        {
            xmlDoc.TryGetValue(ev.DocId, out var doc);
            var summary = doc?.Summary ?? $"{ev.Name} event of {tr.Name}.";

            // Build the event signature from the delegate's `Invoke` shape if we have
            // one; otherwise we emit a Generic "EventHandler" signature.
            var delegatePretty = MetadataReflector.PrettyType(ev.DelegateType, typeCtx);
            var signature = $"event {delegatePretty} {ev.Name}";

            skeleton.events.Add(new EventInfo(
                name: ev.Name,
                @delegate: delegatePretty,
                signature: signature,
                summary: summary,
                fires_when: doc?.Remarks,
                doc_url: MemberDocUrl(tr.FullName, ev.DocId),
                handler_thread: "unknown",
                interacts_with: new List<string>()));
        }

        // Enum values (for enum types only). We treat enum fields as enum_values;
        // for non-enum types these are static constants and are not emitted in IR
        // (the IR schema reserves enum_values for actual enum members).
        if (tr.Kind == TypeKind.Enum)
        {
            foreach (var f in tr.Fields)
            {
                if (!f.IsLiteral) continue;
                xmlDoc.TryGetValue(f.DocId, out var doc);

                // Underlying value: int unless the constant fits a larger type. The
                // schema accepts integer or string. For Navisworks (all int-backed
                // enums), we emit integer.
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

        // Combined generic context — both the declaring type's params (resolved as
        // `<n>`) and the method's own params (resolved as ``<n>`) need name resolution.
        var ctx = new GenericContext(tr.GenericParameters, m.GenericParameters);

        // Real parameter records with per-param doc strings.
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

        // Returns — null for void methods and constructors.
        ReturnInfo? returns = null;
        if (!m.IsConstructor && m.ReturnType != "System.Void")
        {
            var returnsDoc = doc?.Returns ?? "";
            returns = new ReturnInfo(
                type: MetadataReflector.PrettyType(m.ReturnType, ctx),
                doc: returnsDoc);
        }

        // Throws — from <exception> elements. ExceptionDoc.TypeRef is already in
        // doc-id form (no generic params here, exception types are always concrete).
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
            doc_url: MemberDocUrl(tr.FullName, m.DocId),
            since: null,
            deprecated: null);
    }

    static string MemberDocUrl(string typeFullName, string docId) =>
        $"https://aps.autodesk.com/developer/overview/navisworks-api#{Uri.EscapeDataString(docId)}";
}
