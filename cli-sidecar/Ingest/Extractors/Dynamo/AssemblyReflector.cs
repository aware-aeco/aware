// AssemblyReflector — walks the public surface of a managed assembly via
// PEReader+MetadataReader and turns it into a stream of TypeInfo records ready to
// drop into the host-coverage IR.
//
// Why MetadataReader over MetadataLoadContext or Assembly.Load?
//   - Single-file, AOT-clean. We never resolve types across assembly boundaries,
//     so the CLR type loader is never invoked. No StackOverflowException on huge
//     dependency graphs.
//   - Same approach proven in cli-sidecar/Reflection/DllReflector.cs (used by
//     `aware build agent --from-nuget`).
//
// Two-pass design:
//   Pass 1 — index all type definitions by name so that base-type / interface
//            references that resolve to types inside the SAME assembly can be
//            rendered using the canonical Namespace.Type form.
//   Pass 2 — emit TypeInfo per public type:
//            • kind: class / static-class / interface / struct / enum / delegate
//            • base + interfaces (string-rendered)
//            • constructors (M:#ctor entries)
//            • methods (regular M: entries, skipping special-name accessors)
//            • properties (P:) — getter/setter visibility computed from accessor presence + visibility
//            • events (E:) — delegate type from the EventDefinition's underlying type
//            • fields (F:) — only for enums (rendered as enum_values)
//            • generic type parameters in the type name (Foo<T>)
//
// Pairing with XmlDocReader:
//   We don't need every member to have a docstring (real-world XML doc coverage
//   is usually ~60-80% of public surface). For members with no XML doc match, the
//   summary defaults to `${TypeName}.${MemberName}` (same approach as DllReflector).
//
// Property indexer / operator naming:
//   Indexers expose as "Item" (the metadata name). We surface them with name
//   "this[...]" so the IR consumer can disambiguate from regular properties.
//   Operators expose as "op_Addition" etc.; we keep the metadata name verbatim
//   because that's how the XML doc IDs reference them.
//
// Visibility filter:
//   We match the public surface that Roslyn would document — TypeAttributes.Public
//   for top-level types, NestedPublic for nested. Methods, properties, events need
//   at least one public accessor. Fields: only public + non-special for non-enum,
//   for enums we emit ALL non-special fields (the underlying-value field is
//   filtered separately via FieldAttributes.RTSpecialName).

using System.Collections.Immutable;
using System.Reflection;
using System.Reflection.Metadata;
using System.Reflection.Metadata.Ecma335;
using System.Reflection.PortableExecutable;

namespace AwareSidecar.Ingest.Extractors.Dynamo;

/// <summary>
/// Reflects a single .NET managed assembly, paired with its XML doc file, into a
/// list of TypeInfo records.
/// </summary>
public sealed class AssemblyReflector
{
    readonly string _assemblyName;
    readonly XmlDocReader _docs;
    readonly Func<string, string, string> _typeDocUrlFn;     // (assemblyName, typeFullName) → URL
    readonly Func<string, string, string, string> _memberDocUrlFn; // (assemblyName, typeFullName, memberId) → URL

    public AssemblyReflector(
        string assemblyName,
        XmlDocReader docs,
        Func<string, string, string> typeDocUrlFn,
        Func<string, string, string, string> memberDocUrlFn)
    {
        _assemblyName = assemblyName;
        _docs = docs;
        _typeDocUrlFn = typeDocUrlFn;
        _memberDocUrlFn = memberDocUrlFn;
    }

    /// <summary>
    /// Emit a TypeInfo per public type in the given assembly file.
    /// </summary>
    public List<TypeInfo> Reflect(string dllPath)
    {
        var output = new List<TypeInfo>();
        using var fileStream = new FileStream(dllPath, FileMode.Open, FileAccess.Read, FileShare.Read);
        using var pe = new PEReader(fileStream);
        if (!pe.HasMetadata)
            throw new InvalidDataException($"{dllPath} has no managed metadata");
        var mr = pe.GetMetadataReader();

        // Pre-pass: build a set of fully-qualified type names defined in this assembly.
        // Useful for future logic (e.g. internal-vs-cross-asm rendering) but not strictly
        // required here — the signature provider strips all assembly identity.

        foreach (var th in mr.TypeDefinitions)
        {
            var td = mr.GetTypeDefinition(th);
            var attrs = td.Attributes;
            var visibility = attrs & TypeAttributes.VisibilityMask;
            // Public top-level OR public nested. Nested types are surfaced as `Outer.Inner`
            // entries in the IR (same as Sandcastle / DocFX).
            if (visibility != TypeAttributes.Public && visibility != TypeAttributes.NestedPublic) continue;

            // Compiler-generated types (`<>c__DisplayClass0_0`) have RTSpecialName + names starting
            // with `<` — skip.
            var rawName = mr.GetString(td.Name);
            if (rawName.StartsWith('<')) continue;

            var ns = td.Namespace.IsNil ? "" : mr.GetString(td.Namespace);
            var typeFullName = ComputeFullName(mr, td, ns);
            var typeName = CleanGenericArity(rawName);
            // For nested types, qualify the IR `@namespace` with the declaring chain so
            // `(namespace, name)` is unique — Allplan convention. Without this, multiple
            // outer types each holding a nested `State` enum all serialise as
            // `namespace="", name="State"` and collide in the catalog (silent-overwrite
            // bug surfaced by dynamo codex-coverage FAIL 2026-05-19; 6 distinct vendor
            // types were lost). Example: a `State` enum nested in
            // `Dynamo.Models.AnnotationModel` becomes
            // namespace="Dynamo.Models.AnnotationModel", name="State".
            string irNamespace = td.IsNested
                ? ComputeContainerNamespace(mr, td)
                : ns;

            // Generic type parameters → list of names like "T", "TKey", "TValue"
            var typeGenericParams = td.GetGenericParameters()
                .Select(gph => mr.GetString(mr.GetGenericParameter(gph).Name))
                .ToImmutableArray();
            var displayName = typeGenericParams.Length == 0
                ? typeName
                : $"{typeName}<{string.Join(", ", typeGenericParams)}>";

            var ctx = new GenericContext(typeGenericParams, ImmutableArray<string>.Empty);

            // Kind classification.
            var isInterface = (attrs & TypeAttributes.Interface) != 0;
            var isEnum = IsEnumType(mr, td);
            var isValueType = IsValueType(mr, td) && !isEnum;
            var isDelegate = IsDelegateType(mr, td) && !isInterface;
            var isStaticClass = IsStaticClass(attrs);
            string kind;
            if (isInterface) kind = "interface";
            else if (isEnum) kind = "enum";
            else if (isValueType) kind = "struct";
            else if (isDelegate) kind = "delegate";
            else if (isStaticClass) kind = "static-class";
            else kind = "class";

            // Type-level docstring via T: ID. The XML doc ID never uses arity for plain types
            // (it does for generic ones: `Foo``1`), so we use the bare metadata name.
            var typeXmlId = string.IsNullOrEmpty(ns)
                ? "T:" + rawName.Replace('+', '.')
                : "T:" + ns + "." + ComputeXmlNestedName(mr, td);
            _docs.Members.TryGetValue(typeXmlId, out var typeDoc);
            var summary = typeDoc?.Summary ?? $"Type {displayName}";
            var remarks = typeDoc?.Remarks;

            // Base + interfaces.
            string? baseName = null;
            if (!td.BaseType.IsNil)
            {
                try { baseName = RenderTypeReferenceOrSpec(mr, td.BaseType, ctx); }
                catch { baseName = null; }
                // Don't bother rendering object/ValueType/Enum as base — those are implied by kind.
                if (baseName == "System.Object" || baseName == "System.ValueType" || baseName == "System.Enum"
                    || baseName == "System.MulticastDelegate" || baseName == "System.Delegate")
                {
                    baseName = null;
                }
            }
            var interfaces = new List<string>();
            foreach (var ih in td.GetInterfaceImplementations())
            {
                try
                {
                    var ii = mr.GetInterfaceImplementation(ih);
                    var name = RenderTypeReferenceOrSpec(mr, ii.Interface, ctx);
                    interfaces.Add(name);
                }
                catch { /* tolerated — exotic interface refs */ }
            }

            // Delegate signature (Invoke method) for kind=delegate.
            string? delegateSig = null;
            if (kind == "delegate")
            {
                foreach (var mh in td.GetMethods())
                {
                    var md = mr.GetMethodDefinition(mh);
                    if (mr.GetString(md.Name) != "Invoke") continue;
                    var sig = md.DecodeSignature(SignatureTypeNameProvider.Instance, ctx);
                    delegateSig = FormatMethodSignature("Invoke", sig, md, mr, ctx);
                    break;
                }
            }

            // Members.
            var ctors = new List<MethodInfo>();
            var methods = new List<MethodInfo>();
            var properties = new List<PropertyInfo>();
            var events = new List<EventInfo>();
            var enumValues = new List<EnumValueInfo>();

            // Bookkeeping: methods that ARE accessors (property/event under-the-hood) must be
            // skipped from `methods`. We collect their handles in a set first.
            var accessorHandles = new HashSet<int>();
            foreach (var ph in td.GetProperties())
            {
                var pacc = mr.GetPropertyDefinition(ph).GetAccessors();
                if (!pacc.Getter.IsNil) accessorHandles.Add(MetadataTokens.GetToken(pacc.Getter));
                if (!pacc.Setter.IsNil) accessorHandles.Add(MetadataTokens.GetToken(pacc.Setter));
            }
            foreach (var eh in td.GetEvents())
            {
                var eacc = mr.GetEventDefinition(eh).GetAccessors();
                if (!eacc.Adder.IsNil) accessorHandles.Add(MetadataTokens.GetToken(eacc.Adder));
                if (!eacc.Remover.IsNil) accessorHandles.Add(MetadataTokens.GetToken(eacc.Remover));
                if (!eacc.Raiser.IsNil) accessorHandles.Add(MetadataTokens.GetToken(eacc.Raiser));
            }

            foreach (var mh in td.GetMethods())
            {
                var md = mr.GetMethodDefinition(mh);
                var methodAttrs = md.Attributes;
                // Public methods only — protected/private/internal are not part of the public API.
                var methodVis = methodAttrs & MethodAttributes.MemberAccessMask;
                if (methodVis != MethodAttributes.Public && methodVis != MethodAttributes.Family
                    && methodVis != MethodAttributes.FamORAssem)
                {
                    continue;
                }
                // Filter out compiler-generated and accessor methods.
                if (accessorHandles.Contains(MetadataTokens.GetToken(mh))) continue;

                var name = mr.GetString(md.Name);
                if (name.StartsWith('<')) continue; // compiler-generated

                // ctor naming: metadata name is `.ctor` or `.cctor`. We skip `.cctor` (static type
                // initializer — not a public construction path). `.ctor` becomes the type name.
                if (name == ".cctor") continue;
                bool isCtor = name == ".ctor";

                // Method-level generic parameters.
                var methodGenericNames = md.GetGenericParameters()
                    .Select(gph => mr.GetString(mr.GetGenericParameter(gph).Name))
                    .ToImmutableArray();
                var methodCtx = new GenericContext(typeGenericParams, methodGenericNames);

                MethodSignature<string> sig;
                try { sig = md.DecodeSignature(SignatureTypeNameProvider.Instance, methodCtx); }
                catch { continue; }

                // Parameter names — these live in the Parameter table, indexed by sequence.
                var paramNames = new Dictionary<int, string>();
                foreach (var paramHandle in md.GetParameters())
                {
                    var p = mr.GetParameter(paramHandle);
                    if (p.SequenceNumber == 0) continue; // sequence 0 is the return parameter
                    paramNames[p.SequenceNumber - 1] = mr.GetString(p.Name);
                }

                // Compose method XML-doc ID.
                var methodXmlId = MakeMethodXmlId(typeFullName, isCtor ? "#ctor" : EscapeOperatorName(name),
                    typeGenericParams.Length, methodGenericNames.Length, sig.ParameterTypes);

                if (!_docs.Members.TryGetValue(methodXmlId, out var methodDoc))
                {
                    // Exact-match miss. The XML doc id round-trip from display strings is best-effort
                    // (generic parameter rendering, modreq attribute stripping, etc., aren't fully
                    // reversible). Fall back to a prefix scan: match the same name on the same type,
                    // taking the first overload. For most public methods Dynamo has zero or one
                    // overload, so this picks up the right entry.
                    var prefix = isCtor ? $"M:{typeFullName}.#ctor" : $"M:{typeFullName}.{name}";
                    // Generic types carry the arity backtick: M:Type`1.Method(...)
                    if (typeGenericParams.Length > 0)
                    {
                        prefix = isCtor
                            ? $"M:{typeFullName}`{typeGenericParams.Length}.#ctor"
                            : $"M:{typeFullName}`{typeGenericParams.Length}.{name}";
                    }
                    methodDoc = FindByPrefix(prefix);
                }
                var methodSummary = methodDoc?.Summary
                                    ?? $"{typeName}.{(isCtor ? typeName : name)}";

                // Render the method's display name (with overload signature shape).
                var displayMethodName = isCtor ? typeName : name;
                var paramsList = BuildParams(sig.ParameterTypes, paramNames, methodDoc, md, mr);
                var returnInfo = BuildReturn(sig.ReturnType, methodDoc, isCtor);
                var throws = (methodDoc?.Exceptions ?? new())
                    .Select(e => new ThrowsInfo(e.Cref, e.Doc))
                    .ToList();

                var signature = FormatMethodSignature(displayMethodName, sig, md, mr, methodCtx);
                var memberDocUrl = _memberDocUrlFn(_assemblyName, typeFullName, methodXmlId);

                var info = new MethodInfo(
                    name: isCtor ? FormatCtorName(typeName, paramsList) : (paramsList.Count == 0 ? name : $"{name}({string.Join(", ", paramsList.Select(p => p.type))})"),
                    signature: signature,
                    summary: methodSummary,
                    remarks: methodDoc?.Remarks,
                    @params: paramsList,
                    returns: returnInfo,
                    throws: throws,
                    events_related: new List<string>(),
                    doc_url: memberDocUrl,
                    since: null,
                    deprecated: null);
                if (isCtor) ctors.Add(info);
                else methods.Add(info);
            }

            foreach (var ph in td.GetProperties())
            {
                var pd = mr.GetPropertyDefinition(ph);
                var acc = pd.GetAccessors();
                bool getterPub = !acc.Getter.IsNil && IsPublicMethod(mr, acc.Getter);
                bool setterPub = !acc.Setter.IsNil && IsPublicMethod(mr, acc.Setter);
                if (!getterPub && !setterPub) continue;

                var pname = mr.GetString(pd.Name);
                var psig = pd.DecodeSignature(SignatureTypeNameProvider.Instance, ctx);
                var ptype = psig.ReturnType;
                var displayPname = pname;
                if (pname == "Item")
                {
                    // Indexer — render as this[...] with parameter types.
                    displayPname = $"this[{string.Join(", ", psig.ParameterTypes)}]";
                }

                // XML doc ID: P: prefix. Generic types use ``N suffix on the type.
                var typeIdPrefix = typeGenericParams.Length > 0
                    ? $"{typeFullName}`{typeGenericParams.Length}"
                    : typeFullName;
                var propXmlId = "P:" + typeIdPrefix + "." + pname;
                if (psig.ParameterTypes.Length > 0)
                {
                    // Indexers carry their index-type list in parentheses.
                    propXmlId += "(" + string.Join(",", psig.ParameterTypes.Select(EscapeIdParam)) + ")";
                }
                _docs.Members.TryGetValue(propXmlId, out var pDoc);
                var pSummary = pDoc?.Summary ?? $"{typeName}.{pname}";

                properties.Add(new PropertyInfo(
                    name: displayPname,
                    type: ptype,
                    getter: getterPub,
                    setter: setterPub,
                    summary: pSummary,
                    remarks: pDoc?.Remarks,
                    doc_url: _memberDocUrlFn(_assemblyName, typeFullName, propXmlId)));
            }

            foreach (var eh in td.GetEvents())
            {
                var ed = mr.GetEventDefinition(eh);
                var ename = mr.GetString(ed.Name);
                var eacc = ed.GetAccessors();
                bool publicAdd = !eacc.Adder.IsNil && IsPublicMethod(mr, eacc.Adder);
                if (!publicAdd) continue;

                // Delegate type — Event.Type is a TypeReferenceHandle / TypeDefinitionHandle / TypeSpecHandle.
                string delegateType;
                try { delegateType = RenderTypeReferenceOrSpec(mr, ed.Type, ctx); }
                catch { delegateType = "EventHandler"; }

                var evtTypeIdPrefix = typeGenericParams.Length > 0
                    ? $"{typeFullName}`{typeGenericParams.Length}"
                    : typeFullName;
                var evtXmlId = "E:" + evtTypeIdPrefix + "." + ename;
                _docs.Members.TryGetValue(evtXmlId, out var eDoc);

                events.Add(new EventInfo(
                    name: ename,
                    @delegate: delegateType,
                    signature: $"public event {delegateType} {ename}",
                    summary: eDoc?.Summary ?? $"{typeName}.{ename}",
                    fires_when: eDoc?.Remarks,
                    doc_url: _memberDocUrlFn(_assemblyName, typeFullName, evtXmlId),
                    handler_thread: "unknown",
                    interacts_with: new List<string>()));
            }

            if (isEnum)
            {
                foreach (var fh in td.GetFields())
                {
                    var fd = mr.GetFieldDefinition(fh);
                    var fattrs = fd.Attributes;
                    // RTSpecialName = the synthesized underlying-value field (`value__`). Skip.
                    if ((fattrs & FieldAttributes.RTSpecialName) != 0) continue;
                    if ((fattrs & FieldAttributes.FieldAccessMask) != FieldAttributes.Public) continue;
                    if ((fattrs & FieldAttributes.Static) == 0) continue;
                    if ((fattrs & FieldAttributes.Literal) == 0) continue;

                    var fname = mr.GetString(fd.Name);
                    var fTypeIdPrefix = typeGenericParams.Length > 0
                        ? $"{typeFullName}`{typeGenericParams.Length}"
                        : typeFullName;
                    var fxmlId = "F:" + fTypeIdPrefix + "." + fname;
                    _docs.Members.TryGetValue(fxmlId, out var fDoc);

                    // Decode the constant value.
                    System.Text.Json.JsonElement valueEl = System.Text.Json.JsonSerializer.SerializeToElement(0, IrJsonContext.Default.Int32);
                    if (!fd.GetDefaultValue().IsNil)
                    {
                        var c = mr.GetConstant(fd.GetDefaultValue());
                        var blob = mr.GetBlobReader(c.Value);
                        object? boxed = c.TypeCode switch
                        {
                            ConstantTypeCode.SByte    => (object)blob.ReadSByte(),
                            ConstantTypeCode.Int16    => (object)blob.ReadInt16(),
                            ConstantTypeCode.Int32    => (object)blob.ReadInt32(),
                            ConstantTypeCode.Int64    => (object)blob.ReadInt64(),
                            ConstantTypeCode.Byte     => (object)blob.ReadByte(),
                            ConstantTypeCode.UInt16   => (object)blob.ReadUInt16(),
                            ConstantTypeCode.UInt32   => (object)blob.ReadUInt32(),
                            ConstantTypeCode.UInt64   => (object)(long)blob.ReadUInt64(), // long fits IR schema's integer
                            ConstantTypeCode.String   => blob.ReadUTF16(blob.RemainingBytes),
                            _ => null
                        };
                        if (boxed is string strVal)
                            valueEl = System.Text.Json.JsonSerializer.SerializeToElement(strVal, IrJsonContext.Default.String);
                        else if (boxed is null)
                            valueEl = System.Text.Json.JsonSerializer.SerializeToElement(0, IrJsonContext.Default.Int32);
                        else
                            valueEl = System.Text.Json.JsonSerializer.SerializeToElement(Convert.ToInt32(boxed), IrJsonContext.Default.Int32);
                    }

                    enumValues.Add(new EnumValueInfo(fname, valueEl, fDoc?.Summary));
                }
            }

            // Sort members for deterministic IR output.
            ctors = ctors.OrderBy(c => c.name, StringComparer.Ordinal).ToList();
            methods = methods.OrderBy(m => m.name, StringComparer.Ordinal).ToList();
            properties = properties.OrderBy(p => p.name, StringComparer.Ordinal).ToList();
            events = events.OrderBy(e => e.name, StringComparer.Ordinal).ToList();
            enumValues = enumValues.OrderBy(e => e.name, StringComparer.Ordinal).ToList();

            output.Add(new TypeInfo(
                name: displayName,
                @namespace: irNamespace,
                kind: kind,
                summary: summary,
                remarks: remarks,
                @base: baseName,
                interfaces: interfaces,
                doc_url: _typeDocUrlFn(_assemblyName, typeFullName),
                constructors: ctors,
                methods: methods,
                properties: properties,
                events: events,
                enum_values: enumValues,
                delegate_signature: delegateSig));
        }

        return output;
    }

    // ── helpers ─────────────────────────────────────────────────────────────

    /// <summary>Format `(Type1 p1, Type2 p2)` for a ctor or operator. Display-only.</summary>
    static string FormatCtorName(string typeName, List<ParamInfo> paramsList)
    {
        if (paramsList.Count == 0) return $"{typeName}()";
        return $"{typeName}({string.Join(", ", paramsList.Select(p => p.type))})";
    }

    static string FormatMethodSignature(string methodName, MethodSignature<string> sig, MethodDefinition md, MetadataReader mr, GenericContext ctx)
    {
        var generic = sig.GenericParameterCount > 0
            ? "<" + string.Join(", ", ctx.MethodGenericParameters) + ">"
            : "";
        var paramsCsv = string.Join(", ", sig.ParameterTypes.Zip(
            md.GetParameters()
              .Select(ph => mr.GetParameter(ph))
              .Where(p => p.SequenceNumber > 0)
              .Select(p => mr.GetString(p.Name))
              .Concat(Enumerable.Repeat("", sig.ParameterTypes.Length)),
            (t, n) => string.IsNullOrEmpty(n) ? t : $"{t} {n}"));
        return $"{sig.ReturnType} {methodName}{generic}({paramsCsv})";
    }

    static List<ParamInfo> BuildParams(ImmutableArray<string> paramTypes, Dictionary<int, string> paramNames,
        XmlDocReader.MemberDoc? doc, MethodDefinition md, MetadataReader mr)
    {
        var list = new List<ParamInfo>();
        for (int i = 0; i < paramTypes.Length; i++)
        {
            paramNames.TryGetValue(i, out var pname);
            var paramName = string.IsNullOrEmpty(pname) ? $"arg{i}" : pname;
            string? pdoc = null;
            doc?.Params.TryGetValue(paramName, out pdoc);
            // Optional / default? The Parameter table carries FieldAttributes.HasDefault.
            // We don't extract default constant values here — most Dynamo public methods are
            // non-defaulted. Keep optional=false to match the schema's default.
            list.Add(new ParamInfo(paramName, paramTypes[i], pdoc, optional: false, @default: null));
        }
        return list;
    }

    static ReturnInfo? BuildReturn(string returnType, XmlDocReader.MemberDoc? doc, bool isCtor)
    {
        if (isCtor) return null;       // ctors are void from the schema's POV (returns=null)
        if (returnType == "void") return null;
        return new ReturnInfo(returnType, doc?.ReturnsDoc ?? "");
    }

    static string ComputeFullName(MetadataReader mr, TypeDefinition td, string ns)
    {
        var name = CleanGenericArity(mr.GetString(td.Name));
        if (td.IsNested)
        {
            var parent = mr.GetTypeDefinition(td.GetDeclaringType());
            var parentNs = parent.Namespace.IsNil ? "" : mr.GetString(parent.Namespace);
            var parentFull = ComputeFullName(mr, parent, parentNs);
            return parentFull + "." + name;
        }
        return string.IsNullOrEmpty(ns) ? name : ns + "." + name;
    }

    /// <summary>
    /// For a nested type, returns the fully qualified path of its enclosing types so the
    /// IR `@namespace` field can hold `<outer-namespace>.<outer-name>` (Allplan convention).
    /// For `State` nested in `Dynamo.Models.AnnotationModel` this returns
    /// `Dynamo.Models.AnnotationModel`. For a deeply nested `Inner` inside
    /// `Outer.Mid.Inner` it returns `Ns.Outer.Mid`. The result is empty when the immediate
    /// parent has no namespace and no enclosing types (root-level type that's still
    /// IsNested by metadata) — caller treats that as a plain top-level namespace.
    /// Precondition: `td.IsNested` is true; otherwise caller should pass `ns` directly.
    /// </summary>
    static string ComputeContainerNamespace(MetadataReader mr, TypeDefinition td)
    {
        var parent = mr.GetTypeDefinition(td.GetDeclaringType());
        var parentNs = parent.Namespace.IsNil ? "" : mr.GetString(parent.Namespace);
        return ComputeFullName(mr, parent, parentNs);
    }

    static string ComputeXmlNestedName(MetadataReader mr, TypeDefinition td)
    {
        var raw = mr.GetString(td.Name);
        if (!td.IsNested) return raw;
        var parent = mr.GetTypeDefinition(td.GetDeclaringType());
        return ComputeXmlNestedName(mr, parent) + "." + raw;
    }

    static bool IsEnumType(MetadataReader mr, TypeDefinition td)
    {
        if (td.BaseType.IsNil) return false;
        if (td.BaseType.Kind != HandleKind.TypeReference) return false;
        var tr = mr.GetTypeReference((TypeReferenceHandle)td.BaseType);
        var name = mr.GetString(tr.Name);
        var ns = tr.Namespace.IsNil ? "" : mr.GetString(tr.Namespace);
        return ns == "System" && name == "Enum";
    }

    static bool IsValueType(MetadataReader mr, TypeDefinition td)
    {
        if (td.BaseType.IsNil) return false;
        if (td.BaseType.Kind != HandleKind.TypeReference) return false;
        var tr = mr.GetTypeReference((TypeReferenceHandle)td.BaseType);
        var name = mr.GetString(tr.Name);
        var ns = tr.Namespace.IsNil ? "" : mr.GetString(tr.Namespace);
        return ns == "System" && (name == "ValueType" || name == "Enum");
    }

    static bool IsDelegateType(MetadataReader mr, TypeDefinition td)
    {
        if (td.BaseType.IsNil) return false;
        if (td.BaseType.Kind != HandleKind.TypeReference) return false;
        var tr = mr.GetTypeReference((TypeReferenceHandle)td.BaseType);
        var name = mr.GetString(tr.Name);
        var ns = tr.Namespace.IsNil ? "" : mr.GetString(tr.Namespace);
        return ns == "System" && (name == "MulticastDelegate" || name == "Delegate");
    }

    static bool IsStaticClass(TypeAttributes attrs)
    {
        // Static classes are encoded as abstract + sealed (Roslyn / csc).
        return (attrs & TypeAttributes.Abstract) != 0
               && (attrs & TypeAttributes.Sealed) != 0;
    }

    static bool IsPublicMethod(MetadataReader mr, MethodDefinitionHandle h)
    {
        var md = mr.GetMethodDefinition(h);
        var v = md.Attributes & MethodAttributes.MemberAccessMask;
        return v == MethodAttributes.Public;
    }

    static string RenderTypeReferenceOrSpec(MetadataReader mr, EntityHandle handle, GenericContext ctx)
    {
        switch (handle.Kind)
        {
            case HandleKind.TypeDefinition:
                return SignatureTypeNameProvider.Instance.GetTypeFromDefinition(mr, (TypeDefinitionHandle)handle, 0);
            case HandleKind.TypeReference:
                return SignatureTypeNameProvider.Instance.GetTypeFromReference(mr, (TypeReferenceHandle)handle, 0);
            case HandleKind.TypeSpecification:
                var ts = mr.GetTypeSpecification((TypeSpecificationHandle)handle);
                return ts.DecodeSignature(SignatureTypeNameProvider.Instance, ctx);
            default:
                return "object";
        }
    }

    static string CleanGenericArity(string name)
    {
        var tick = name.IndexOf('`');
        return tick < 0 ? name : name.Substring(0, tick);
    }

    /// <summary>
    /// Find the first XML-doc entry whose ID matches `<prefix>(` or `<prefix>` exactly.
    /// Used as a fallback when the exact-form lookup misses because of round-trip
    /// imperfections in our display→canonical type-name conversion. Iterates the doc
    /// dictionary once per fallback — O(N) per miss but acceptable: a hot type has
    /// dozens of methods at most, and the entire DynamoCore XML has ~7000 entries.
    /// </summary>
    XmlDocReader.MemberDoc? FindByPrefix(string idPrefix)
    {
        // Exact zero-arg form: `M:Foo.Bar`
        if (_docs.Members.TryGetValue(idPrefix, out var hit)) return hit;
        // Parameterized form: `M:Foo.Bar(...)`
        var parenPrefix = idPrefix + "(";
        foreach (var (key, doc) in _docs.Members)
        {
            if (key.StartsWith(parenPrefix, StringComparison.Ordinal)) return doc;
        }
        // Generic-method form: `M:Foo.Bar``N(...)`
        var genericPrefix = idPrefix + "``";
        foreach (var (key, doc) in _docs.Members)
        {
            if (key.StartsWith(genericPrefix, StringComparison.Ordinal)) return doc;
        }
        return null;
    }

    /// <summary>
    /// Build a Roslyn-style XML doc ID for a method: M:Namespace.Type.Method(P1,P2).
    /// Generic methods include the ``N suffix; generic types include the `N suffix on the
    /// declaring type. Parameters are emitted with their fully-qualified ELEMENT_TYPE form,
    /// using ``0/``1 placeholders for method generics and `0/`1 for type generics.
    /// </summary>
    static string MakeMethodXmlId(string typeFullName, string methodName, int typeGenericCount, int methodGenericCount,
        ImmutableArray<string> paramTypes)
    {
        // Convert the displayed param-type strings back to XML-doc-ID compatible form.
        // The SignatureTypeNameProvider renders type generics as `T` (the name) — XML doc IDs
        // use `0, `1, etc. Without re-decoding the blob, we can't roundtrip perfectly. The
        // simplest reliable trick: when our display picked up the type generic param names
        // (T/TKey/TValue), we substitute them back at concat time. For non-generic methods
        // this just produces "M:Foo.Bar.Baz(System.String,System.Int32)".
        //
        // In practice we won't always get a perfect XML-doc-id roundtrip from these strings.
        // Looking up the XML-doc summary becomes "best-effort": if the exact-form key isn't
        // found, we fall back to a prefix-scan on `M:Foo.Bar.Baz(`.
        var sb = new System.Text.StringBuilder();
        sb.Append("M:");
        // The doc-id type prefix carries the arity backtick for generic types.
        if (typeGenericCount > 0)
        {
            sb.Append(typeFullName);
            sb.Append('`');
            sb.Append(typeGenericCount);
        }
        else
        {
            sb.Append(typeFullName);
        }
        sb.Append('.');
        sb.Append(methodName);
        if (methodGenericCount > 0) sb.Append("``").Append(methodGenericCount);
        if (paramTypes.Length > 0)
        {
            sb.Append('(');
            for (int i = 0; i < paramTypes.Length; i++)
            {
                if (i > 0) sb.Append(',');
                sb.Append(EscapeIdParam(paramTypes[i]));
            }
            sb.Append(')');
        }
        return sb.ToString();
    }

    /// <summary>
    /// Translate a display type name (C# alias form: `string`, `int`, `List&lt;T&gt;`) back to the
    /// XML-doc-id form (canonical: `System.String`, `System.Int32`, `System.Collections.Generic.List{T}`).
    /// </summary>
    /// <remarks>
    /// Roslyn's XML doc ID format uses curly braces for generic instantiations (the cref form), not
    /// angle brackets. Primitives use their `System.&lt;X&gt;` canonical names. The transformations are
    /// stable enough to round-trip the most common Dynamo public API signatures so XmlDocReader
    /// lookups by `M:` ID find their docstring match.
    /// </remarks>
    static string EscapeIdParam(string typeName)
    {
        // ref T → T@ (XML doc id puts the @ AFTER the type)
        if (typeName.StartsWith("ref ", StringComparison.Ordinal))
        {
            return EscapeIdParam(typeName[4..]) + "@";
        }
        // out T → T@ as well (XML doc id doesn't distinguish in/out from ref).
        if (typeName.StartsWith("out ", StringComparison.Ordinal))
        {
            return EscapeIdParam(typeName[4..]) + "@";
        }
        // Array suffix is preserved.
        var arraySuffix = "";
        while (typeName.EndsWith("]", StringComparison.Ordinal))
        {
            var bracketStart = typeName.LastIndexOf('[');
            if (bracketStart < 0) break;
            arraySuffix = typeName[bracketStart..] + arraySuffix;
            typeName = typeName[..bracketStart];
        }
        // Generic instantiation: peel off the type-arg list, recurse on the base name + each arg.
        if (typeName.EndsWith('>'))
        {
            var open = typeName.IndexOf('<');
            if (open > 0)
            {
                var baseName = typeName[..open];
                var argsCsv = typeName[(open + 1)..^1];
                var args = SplitTopLevelCommas(argsCsv);
                var canonicalBase = AliasToCanonical(baseName);
                var canonicalArgs = string.Join(",", args.Select(EscapeIdParam));
                return canonicalBase + "{" + canonicalArgs + "}" + arraySuffix;
            }
        }
        return AliasToCanonical(typeName) + arraySuffix;
    }

    static string AliasToCanonical(string typeName) => typeName switch
    {
        "bool"      => "System.Boolean",
        "byte"      => "System.Byte",
        "sbyte"     => "System.SByte",
        "char"      => "System.Char",
        "short"     => "System.Int16",
        "ushort"    => "System.UInt16",
        "int"       => "System.Int32",
        "uint"      => "System.UInt32",
        "long"      => "System.Int64",
        "ulong"     => "System.UInt64",
        "float"     => "System.Single",
        "double"    => "System.Double",
        "string"    => "System.String",
        "object"    => "System.Object",
        "void"      => "System.Void",
        "decimal"   => "System.Decimal",
        _           => typeName
    };

    /// <summary>Split a comma-separated list at TOP-LEVEL only, respecting &lt; &gt; nesting.</summary>
    static List<string> SplitTopLevelCommas(string csv)
    {
        var result = new List<string>();
        int depth = 0, start = 0;
        for (int i = 0; i < csv.Length; i++)
        {
            char c = csv[i];
            if (c == '<') depth++;
            else if (c == '>') depth--;
            else if (c == ',' && depth == 0)
            {
                result.Add(csv[start..i].Trim());
                start = i + 1;
            }
        }
        if (start < csv.Length) result.Add(csv[start..].Trim());
        return result;
    }

    static string EscapeOperatorName(string name) => name; // already in metadata form (op_Addition etc.)
}
