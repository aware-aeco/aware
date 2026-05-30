// MetadataReflector — shared reflection helper used by vendor extractors whose
// source-of-truth is a .NET assembly (DLL) rather than an HTML doc site.
//
// Walks the PE metadata via System.Reflection.Metadata.PEReader +
// MetadataReader, returning a tree of structured TypeRecord / MethodRecord /
// PropertyRecord / EventRecord / FieldRecord values that downstream code can
// pair with an XmlDocLoader lookup to emit a CoverageIR.
//
// Why MetadataReader (not MetadataLoadContext): identical reasoning to
// `Reflection/DllReflector.cs` — MLC can StackOverflow on deep DevExpress-
// style type graphs, and reads metadata only (no IL), so it's safe to invoke
// from a NativeAOT-compiled host even though some PackageReference targets
// emit IL2026 warnings.
//
// Output canonicalisation
// -----------------------
// Type names match the .NET XML doc id format:
//
//   • Type ids:        T:Namespace.Outer.Nested
//   • Method ids:      M:Namespace.Outer.Method(System.String,System.Int32)
//   • Ctor ids:        M:Namespace.Outer.#ctor(System.String)
//   • Property ids:    P:Namespace.Outer.Name        / P:Namespace.Outer.Item(System.Int32)
//   • Event ids:       E:Namespace.Outer.Name
//   • Field/enum ids:  F:Namespace.Outer.Name
//
// Parameter type encoding follows the C# spec (ECMA-334 §D.3.3 / §D.4.2):
//   • Reference types and value types use their full-namespace name
//     (System.String, System.Int32).
//   • Arrays append `[]` (multidimensional: `[,]`, `[,,]`).
//   • By-ref params append `@`.
//   • Generic type instantiation uses braces: List{System.Int32}.
//   • Generic parameters use `T:` (single-backtick) for type-level and
//     `M:` (double-backtick) for method-level, e.g. ``0 (T) and ```0 (T at method level).
//
// The MemberDoc keys from XmlDocLoader follow exactly the same encoding, so
// MetadataReflector.MakeMethodDocId() and friends can be used directly as a
// dictionary lookup.
//
// AOT: pure System.Reflection.Metadata. No reflection on user types. No trim
// warnings from this file.

using System.Collections.Immutable;
using System.Reflection;
using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using System.Text;

namespace AwareReader;

public sealed record AssemblyRecord(
    string Name,
    string Version,
    IReadOnlyList<TypeRecord> Types);

public sealed record TypeRecord(
    string Namespace,
    string Name,
    string FullName,
    TypeKind Kind,
    bool IsPublic,
    bool IsStatic,
    bool IsSealed,
    bool IsAbstract,
    string? BaseTypeName,
    IReadOnlyList<string> Interfaces,
    IReadOnlyList<string> GenericParameters,
    IReadOnlyList<MethodRecord> Constructors,
    IReadOnlyList<MethodRecord> Methods,
    IReadOnlyList<PropertyRecord> Properties,
    IReadOnlyList<EventRecord> Events,
    IReadOnlyList<FieldRecord> Fields,
    string? DelegateInvokeSignature);

public enum TypeKind
{
    Class,
    Struct,
    Interface,
    Enum,
    Delegate,
    StaticClass,
}

public sealed record MethodRecord(
    string Name,
    string DocId,
    string Signature,
    bool IsStatic,
    bool IsConstructor,
    bool IsAbstract,
    bool IsVirtual,
    bool IsOperator,
    string ReturnType,
    IReadOnlyList<ParameterRecord> Parameters,
    IReadOnlyList<string> GenericParameters);

public sealed record ParameterRecord(
    string Name,
    string Type,
    bool IsByRef,
    bool IsIn,
    bool IsOut,
    bool IsOptional,
    string? DefaultValueLiteral);

public sealed record PropertyRecord(
    string Name,
    string DocId,
    string Type,
    bool HasGetter,
    bool HasSetter,
    bool IsStatic,
    IReadOnlyList<ParameterRecord> IndexParameters);

public sealed record EventRecord(
    string Name,
    string DocId,
    string DelegateType);

public sealed record FieldRecord(
    string Name,
    string DocId,
    string Type,
    bool IsStatic,
    bool IsLiteral,
    object? ConstantValue,
    string? EnumUnderlyingValueLiteral);

public static class MetadataReflector
{
    /// <summary>
    /// Reflects a single assembly and returns its full type tree.
    /// Throws if the file is unreadable or not a managed PE image.
    /// </summary>
    public static AssemblyRecord Reflect(string dllPath)
    {
        using var fs = new FileStream(dllPath, FileMode.Open, FileAccess.Read, FileShare.Read);
        using var peReader = new PEReader(fs);
        if (!peReader.HasMetadata)
            throw new InvalidOperationException($"{dllPath}: PE image has no CLR metadata");

        var mr = peReader.GetMetadataReader();
        if (!mr.IsAssembly)
            throw new InvalidOperationException($"{dllPath}: PE image is not an assembly");

        var asmDef = mr.GetAssemblyDefinition();
        var assemblyName = mr.GetString(asmDef.Name);
        var version = asmDef.Version.ToString();

        var sigDecoder = new SigTypeProvider(mr);
        var types = new List<TypeRecord>();

        foreach (var typeHandle in mr.TypeDefinitions)
        {
            var td = mr.GetTypeDefinition(typeHandle);

            // Filter:
            // - Skip non-public types (TypeAttributes.NestedPublic is acceptable; we
            //   handle nested types via their declaring type's loop, but for top-level
            //   types only TypeAttributes.Public matters).
            // - Skip <Module> (no namespace + name "<Module>") and compiler-generated
            //   types (name starts with '<').
            var attrs = td.Attributes;
            var visibility = attrs & TypeAttributes.VisibilityMask;
            if (visibility != TypeAttributes.Public && visibility != TypeAttributes.NestedPublic)
                continue;
            // Top-level only — nested types are emitted under their declaring type's
            // FullName so callers can walk them. We don't recurse into nested-private
            // members from outside.
            if (visibility == TypeAttributes.NestedPublic) continue;

            var name = mr.GetString(td.Name);
            if (name.Length > 0 && name[0] == '<') continue;
            if (name == "<Module>") continue;

            var ns = td.Namespace.IsNil ? "" : mr.GetString(td.Namespace);
            var fullName = string.IsNullOrEmpty(ns) ? name : $"{ns}.{name}";

            var rec = ReflectType(mr, td, fullName, ns, name, sigDecoder);
            types.Add(rec);
        }

        // Stable sort by FullName so repeated runs yield identical IR.
        types.Sort((a, b) => string.CompareOrdinal(a.FullName, b.FullName));

        return new AssemblyRecord(assemblyName, version, types);
    }

    static TypeRecord ReflectType(
        MetadataReader mr,
        TypeDefinition td,
        string fullName,
        string ns,
        string name,
        SigTypeProvider sigDecoder)
    {
        var attrs = td.Attributes;
        var isInterface = (attrs & TypeAttributes.Interface) != 0;
        var isAbstract = (attrs & TypeAttributes.Abstract) != 0;
        var isSealed = (attrs & TypeAttributes.Sealed) != 0;

        // Determine if the type is an enum or a delegate by looking at its base type.
        var baseTypeName = ResolveTypeHandle(mr, td.BaseType, sigDecoder);
        var isEnum = baseTypeName == "System.Enum";
        var isDelegate = baseTypeName == "System.MulticastDelegate" || baseTypeName == "System.Delegate";
        var isValueType = baseTypeName == "System.ValueType" || isEnum;

        TypeKind kind = isInterface ? TypeKind.Interface
            : isEnum ? TypeKind.Enum
            : isDelegate ? TypeKind.Delegate
            : isValueType ? TypeKind.Struct
            : (isAbstract && isSealed) ? TypeKind.StaticClass
            : TypeKind.Class;

        // Interfaces lose Object as their "base"; we report null for both interfaces
        // and types that inherit directly from Object. Enums also have an implicit
        // base (System.Enum) that's captured by `kind=enum`; we omit it from the
        // emitted IR for parity with the existing Tekla/Rhino extractors. Same for
        // delegates (System.MulticastDelegate is implicit in kind=delegate) and
        // structs (System.ValueType is implicit in kind=struct).
        var reportedBase = (kind == TypeKind.Interface
                            || kind == TypeKind.Enum
                            || kind == TypeKind.Delegate
                            || kind == TypeKind.Struct
                            || baseTypeName == "System.Object")
            ? null
            : baseTypeName;

        // Implemented interfaces.
        var interfaces = new List<string>();
        foreach (var iiHandle in td.GetInterfaceImplementations())
        {
            var ii = mr.GetInterfaceImplementation(iiHandle);
            var iname = ResolveTypeHandle(mr, ii.Interface, sigDecoder);
            if (!string.IsNullOrEmpty(iname))
                interfaces.Add(iname!);
        }
        interfaces.Sort(StringComparer.Ordinal);

        // Generic type parameters.
        var genericParams = new List<string>();
        foreach (var gpHandle in td.GetGenericParameters())
        {
            var gp = mr.GetGenericParameter(gpHandle);
            genericParams.Add(mr.GetString(gp.Name));
        }

        // Methods + constructors (split by name).
        var constructors = new List<MethodRecord>();
        var methods = new List<MethodRecord>();
        string? delegateInvokeSignature = null;

        foreach (var mh in td.GetMethods())
        {
            var md = mr.GetMethodDefinition(mh);
            var mAttrs = md.Attributes;
            // Public-only.
            if ((mAttrs & MethodAttributes.MemberAccessMask) != MethodAttributes.Public) continue;
            // Skip property accessors and event add/remove (they have SpecialName +
            // are paired with their owning property/event).
            if ((mAttrs & MethodAttributes.SpecialName) != 0)
            {
                var mname = mr.GetString(md.Name);
                // Keep operator overloads ('op_*'). Drop everything else with SpecialName
                // that isn't .ctor/.cctor (e.g. get_X, set_X, add_X, remove_X).
                if (!mname.StartsWith("op_", StringComparison.Ordinal)
                    && mname != ".ctor"
                    && mname != ".cctor")
                {
                    continue;
                }
            }

            var record = ReflectMethod(mr, md, fullName, genericParams, sigDecoder);
            if (record.IsConstructor)
            {
                if (record.Name == ".cctor") continue; // Static cctor not user-callable.
                constructors.Add(record);
            }
            else
            {
                if (isDelegate && record.Name == "Invoke")
                {
                    delegateInvokeSignature = record.Signature;
                    // Delegate Invoke methods aren't end-user methods; skip emit.
                    continue;
                }
                // Also drop other compiler-emitted delegate methods (BeginInvoke, EndInvoke).
                if (isDelegate && (record.Name == "BeginInvoke" || record.Name == "EndInvoke"))
                    continue;
                methods.Add(record);
            }
        }
        constructors.Sort((a, b) => string.CompareOrdinal(a.Signature, b.Signature));
        methods.Sort((a, b) => string.CompareOrdinal(a.Signature, b.Signature));

        // Properties.
        var properties = new List<PropertyRecord>();
        foreach (var ph in td.GetProperties())
        {
            var prop = mr.GetPropertyDefinition(ph);
            // Public if either accessor is public.
            var accs = prop.GetAccessors();
            var getter = !accs.Getter.IsNil ? mr.GetMethodDefinition(accs.Getter) : (MethodDefinition?)null;
            var setter = !accs.Setter.IsNil ? mr.GetMethodDefinition(accs.Setter) : (MethodDefinition?)null;
            bool hasPublicGetter = getter.HasValue
                && (getter.Value.Attributes & MethodAttributes.MemberAccessMask) == MethodAttributes.Public;
            bool hasPublicSetter = setter.HasValue
                && (setter.Value.Attributes & MethodAttributes.MemberAccessMask) == MethodAttributes.Public;
            if (!hasPublicGetter && !hasPublicSetter) continue;

            var pname = mr.GetString(prop.Name);
            var sig = prop.DecodeSignature(sigDecoder, new GenericContext(genericParams, Array.Empty<string>()));
            var propType = sig.ReturnType;

            // Indexer parameters (from the signature, not the accessor).
            var indexParams = new List<ParameterRecord>();
            if (sig.ParameterTypes.Length > 0)
            {
                // The accessor (getter preferred, else setter) carries parameter names.
                var accDef = getter ?? setter;
                if (accDef.HasValue)
                {
                    var pHandles = accDef.Value.GetParameters();
                    var names = new List<string>();
                    foreach (var pHandle in pHandles)
                    {
                        var p = mr.GetParameter(pHandle);
                        if (p.SequenceNumber == 0) continue; // return param
                        names.Add(mr.GetString(p.Name));
                    }
                    for (int i = 0; i < sig.ParameterTypes.Length; i++)
                    {
                        var nameP = i < names.Count ? names[i] : $"index{i}";
                        indexParams.Add(new ParameterRecord(
                            Name: nameP,
                            Type: sig.ParameterTypes[i],
                            IsByRef: false,
                            IsIn: false,
                            IsOut: false,
                            IsOptional: false,
                            DefaultValueLiteral: null));
                    }
                }
            }

            bool isStatic = (getter ?? setter)?.Attributes.HasFlag(MethodAttributes.Static) ?? false;

            properties.Add(new PropertyRecord(
                Name: pname,
                DocId: MakePropertyDocId(fullName, pname, indexParams),
                Type: propType,
                HasGetter: hasPublicGetter,
                HasSetter: hasPublicSetter,
                IsStatic: isStatic,
                IndexParameters: indexParams));
        }
        properties.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));

        // Events.
        var events = new List<EventRecord>();
        foreach (var eh in td.GetEvents())
        {
            var ev = mr.GetEventDefinition(eh);
            var ename = mr.GetString(ev.Name);
            // Public if the add accessor is public.
            var accs = ev.GetAccessors();
            if (accs.Adder.IsNil) continue;
            var adder = mr.GetMethodDefinition(accs.Adder);
            if ((adder.Attributes & MethodAttributes.MemberAccessMask) != MethodAttributes.Public) continue;

            var delegateType = ResolveTypeHandle(mr, ev.Type, sigDecoder) ?? "";
            events.Add(new EventRecord(
                Name: ename,
                DocId: $"E:{fullName}.{ename}",
                DelegateType: delegateType));
        }
        events.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));

        // Fields (incl. enum values).
        var fields = new List<FieldRecord>();
        foreach (var fh in td.GetFields())
        {
            var fd = mr.GetFieldDefinition(fh);
            var fAttrs = fd.Attributes;
            // Public + (instance | literal) only.
            if ((fAttrs & FieldAttributes.FieldAccessMask) != FieldAttributes.Public) continue;
            // Skip backing fields for events (they share names with the event and are SpecialName).
            if ((fAttrs & FieldAttributes.SpecialName) != 0) continue;

            var fname = mr.GetString(fd.Name);
            bool isLiteral = (fAttrs & FieldAttributes.Literal) != 0;
            bool isStatic = (fAttrs & FieldAttributes.Static) != 0;
            string typeStr;
            object? constValue = null;
            string? underlyingLit = null;
            try
            {
                typeStr = fd.DecodeSignature(sigDecoder, new GenericContext(genericParams, Array.Empty<string>()));
            }
            catch
            {
                typeStr = "";
            }
            if (isLiteral)
            {
                var constHandle = fd.GetDefaultValue();
                if (!constHandle.IsNil)
                {
                    var constant = mr.GetConstant(constHandle);
                    constValue = ReadConstantValue(mr, constant);
                    underlyingLit = FormatConstantLiteral(constValue);
                }
            }

            fields.Add(new FieldRecord(
                Name: fname,
                DocId: $"F:{fullName}.{fname}",
                Type: typeStr,
                IsStatic: isStatic,
                IsLiteral: isLiteral,
                ConstantValue: constValue,
                EnumUnderlyingValueLiteral: underlyingLit));
        }
        // Preserve enum value declaration order — sort by underlying value when an enum,
        // otherwise alphabetic.
        if (isEnum)
        {
            // Convert constant value to a comparable long for sort. Non-numeric values
            // (strings on string-keyed enums) fall back to alphabetic.
            int OrdinalKey(FieldRecord f) =>
                f.ConstantValue switch
                {
                    sbyte sb => sb,
                    byte b => b,
                    short sh => sh,
                    ushort us => us,
                    int i => i,
                    uint u => unchecked((int)u),
                    long l => unchecked((int)l),
                    ulong ul => unchecked((int)(long)ul),
                    _ => 0,
                };
            fields.Sort((a, b) => OrdinalKey(a).CompareTo(OrdinalKey(b)));
        }
        else
        {
            fields.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));
        }

        return new TypeRecord(
            Namespace: ns,
            Name: name,
            FullName: fullName,
            Kind: kind,
            IsPublic: true,
            IsStatic: kind == TypeKind.StaticClass,
            IsSealed: isSealed,
            IsAbstract: isAbstract,
            BaseTypeName: reportedBase,
            Interfaces: interfaces,
            GenericParameters: genericParams,
            Constructors: constructors,
            Methods: methods,
            Properties: properties,
            Events: events,
            Fields: fields,
            DelegateInvokeSignature: delegateInvokeSignature);
    }

    static MethodRecord ReflectMethod(
        MetadataReader mr,
        MethodDefinition md,
        string typeFullName,
        IReadOnlyList<string> typeGenericNames,
        SigTypeProvider sigDecoder)
    {
        var name = mr.GetString(md.Name);
        bool isCtor = name == ".ctor" || name == ".cctor";
        bool isStatic = (md.Attributes & MethodAttributes.Static) != 0;
        bool isAbstract = (md.Attributes & MethodAttributes.Abstract) != 0;
        bool isVirtual = (md.Attributes & MethodAttributes.Virtual) != 0;
        bool isOperator = name.StartsWith("op_", StringComparison.Ordinal);

        // Method generic parameters.
        var methodGenericNames = new List<string>();
        foreach (var gpHandle in md.GetGenericParameters())
        {
            var gp = mr.GetGenericParameter(gpHandle);
            methodGenericNames.Add(mr.GetString(gp.Name));
        }

        var ctx = new GenericContext(typeGenericNames, methodGenericNames);
        var sig = md.DecodeSignature(sigDecoder, ctx);

        // Parameter names + custom attributes for [out]/[in]/[Optional].
        var pHandles = md.GetParameters();
        var paramRecords = new List<ParameterRecord>();
        var nameByIndex = new Dictionary<int, (string Name, ParameterAttributes Attrs, ConstantHandle Default)>();
        foreach (var pHandle in pHandles)
        {
            var pd = mr.GetParameter(pHandle);
            // SequenceNumber 0 = return value, 1.. = real params.
            nameByIndex[pd.SequenceNumber] = (mr.GetString(pd.Name), pd.Attributes, pd.GetDefaultValue());
        }

        for (int i = 0; i < sig.ParameterTypes.Length; i++)
        {
            var seq = i + 1;
            string pname = nameByIndex.TryGetValue(seq, out var rec) ? rec.Name : $"arg{i}";
            var attrs = nameByIndex.TryGetValue(seq, out var rec2) ? rec2.Attrs : ParameterAttributes.None;
            var defHandle = nameByIndex.TryGetValue(seq, out var rec3) ? rec3.Default : default;

            bool isByRef = false;
            string ptype = sig.ParameterTypes[i];
            if (ptype.EndsWith("@", StringComparison.Ordinal))
            {
                isByRef = true;
                ptype = ptype[..^1];
            }

            bool isOptional = (attrs & ParameterAttributes.Optional) != 0;
            string? defLit = null;
            if (!defHandle.IsNil)
            {
                try
                {
                    var c = mr.GetConstant(defHandle);
                    defLit = FormatConstantLiteral(ReadConstantValue(mr, c));
                }
                catch { /* tolerate broken default values */ }
            }

            paramRecords.Add(new ParameterRecord(
                Name: pname,
                Type: ptype,
                IsByRef: isByRef,
                IsIn: (attrs & ParameterAttributes.In) != 0,
                IsOut: (attrs & ParameterAttributes.Out) != 0,
                IsOptional: isOptional,
                DefaultValueLiteral: defLit));
        }

        string docId = isCtor
            ? MakeMethodDocId(typeFullName, "#ctor", paramRecords, methodGenericNames)
            : MakeMethodDocId(typeFullName, name, paramRecords, methodGenericNames);

        // For operator overloads with conversion result discrimination (op_Implicit / op_Explicit),
        // the doc id appends `~ReturnType`.
        if (isOperator && (name == "op_Implicit" || name == "op_Explicit"))
        {
            docId = docId + "~" + sig.ReturnType;
        }

        // Generic context for pretty-printing — `\`<n>` and `\`\`<n>` references in
        // parameter types and return types resolve to the type/method generic parameter
        // names below. Without this, nested generic refs render with empty placeholders.
        var prettyCtx = new GenericContext(typeGenericNames, methodGenericNames);
        string displayName = isCtor ? CtorDisplayName(typeFullName, paramRecords, prettyCtx) : name;
        string signature = BuildSignature(
            name: isCtor ? DocId.ShortName(typeFullName) : name,
            isCtor: isCtor,
            isStatic: isStatic,
            returnType: sig.ReturnType,
            genericParams: methodGenericNames,
            parameters: paramRecords,
            ctx: prettyCtx);

        return new MethodRecord(
            Name: displayName,
            DocId: docId,
            Signature: signature,
            IsStatic: isStatic,
            IsConstructor: isCtor,
            IsAbstract: isAbstract,
            IsVirtual: isVirtual,
            IsOperator: isOperator,
            ReturnType: sig.ReturnType,
            Parameters: paramRecords,
            GenericParameters: methodGenericNames);
    }

    static string CtorDisplayName(string typeFullName, IReadOnlyList<ParameterRecord> ps, GenericContext? ctx = null)
    {
        // "Foo()" or "Foo(string title, ConfigOptions options)" — the catalog-friendly name
        // that disambiguates overloads. Mirrors the brief's "Constructor names carry overload
        // disambiguator" rule.
        var shortType = DocId.ShortName(typeFullName);
        var sb = new StringBuilder();
        sb.Append(shortType);
        sb.Append('(');
        for (int i = 0; i < ps.Count; i++)
        {
            if (i > 0) sb.Append(", ");
            sb.Append(PrettyType(ps[i].Type, ctx));
        }
        sb.Append(')');
        return sb.ToString();
    }

    /// <summary>
    /// Reconstructs a C#-ish display signature: "TReturn Name(TArg1 name1, TArg2 name2)".
    /// Used as IR `method.signature`. Does NOT include attribute modifiers; the schema's
    /// signature field is for human readability only.
    /// </summary>
    static string BuildSignature(
        string name,
        bool isCtor,
        bool isStatic,
        string returnType,
        IReadOnlyList<string> genericParams,
        IReadOnlyList<ParameterRecord> parameters,
        GenericContext? ctx = null)
    {
        var sb = new StringBuilder();
        if (isStatic && !isCtor) sb.Append("static ");
        if (!isCtor)
        {
            sb.Append(PrettyType(returnType, ctx));
            sb.Append(' ');
        }
        sb.Append(name);
        if (genericParams.Count > 0)
        {
            sb.Append('<');
            sb.Append(string.Join(", ", genericParams));
            sb.Append('>');
        }
        sb.Append('(');
        for (int i = 0; i < parameters.Count; i++)
        {
            if (i > 0) sb.Append(", ");
            var p = parameters[i];
            if (p.IsByRef) sb.Append(p.IsOut ? "out " : "ref ");
            else if (p.IsIn) sb.Append("in ");
            sb.Append(PrettyType(p.Type, ctx));
            sb.Append(' ');
            sb.Append(p.Name);
            if (p.IsOptional && p.DefaultValueLiteral is not null)
            {
                sb.Append(" = ");
                sb.Append(p.DefaultValueLiteral);
            }
        }
        sb.Append(')');
        return sb.ToString();
    }

    /// <summary>
    /// Converts the XML-doc-id-style type encoding back to a C#-readable form.
    ///   System.String          → string
    ///   System.Int32           → int
    ///   System.Boolean         → bool
    ///   System.Void            → void
    ///   System.Object          → object
    ///   List{System.Int32}     → List&lt;int&gt;
    ///   System.Nullable{T}     → T?
    ///   System.Collections.Generic.IEnumerable{T}  → IEnumerable&lt;T&gt;
    ///
    /// If a <see cref="GenericContext"/> is supplied, generic parameter references
    /// (encoded as `<i>0</i>, `<i>1</i> at type level and ``<i>0</i> at method level)
    /// are resolved to their declared names. Without a context, the backtick-encoded
    /// references are stripped to bare empty placeholders so the resulting string
    /// stays readable at the cost of fidelity (used only by tests that don't have a
    /// context to plumb in).
    /// </summary>
    public static string PrettyType(string typeIdSegment, GenericContext? ctx = null)
    {
        // Map primitives first.
        var trimmed = typeIdSegment;
        bool isByRef = false;
        if (trimmed.EndsWith("@", StringComparison.Ordinal))
        {
            isByRef = true;
            trimmed = trimmed[..^1];
        }
        // Array suffixes — keep as [], [,], etc.
        var arraySuffix = "";
        while (trimmed.EndsWith("]", StringComparison.Ordinal))
        {
            var openIdx = trimmed.LastIndexOf('[');
            if (openIdx < 0) break;
            arraySuffix = trimmed[openIdx..] + arraySuffix;
            trimmed = trimmed[..openIdx];
        }
        // Nullable shortcut.
        if (trimmed.StartsWith("System.Nullable{", StringComparison.Ordinal) && trimmed.EndsWith("}", StringComparison.Ordinal))
        {
            var inner = trimmed.Substring("System.Nullable{".Length, trimmed.Length - "System.Nullable{".Length - 1);
            return PrettyType(inner, ctx) + "?" + arraySuffix + (isByRef ? "&" : "");
        }
        // Generic instantiation.
        var braceIdx = trimmed.IndexOf('{');
        if (braceIdx >= 0 && trimmed.EndsWith("}", StringComparison.Ordinal))
        {
            var rawName = trimmed[..braceIdx];
            // Strip backtick arity off the raw name.
            var tickIdx = rawName.IndexOf('`');
            if (tickIdx >= 0) rawName = rawName[..tickIdx];
            var args = SplitGenericArgs(trimmed.Substring(braceIdx + 1, trimmed.Length - braceIdx - 2));
            var prettyArgs = string.Join(", ", args.Select(a => PrettyType(a, ctx)));
            return ShortenWellKnownName(rawName) + "<" + prettyArgs + ">" + arraySuffix + (isByRef ? "&" : "");
        }
        // Bare generic param reference: `0, `1 (type-level) or ``0, ``1 (method-level).
        // Resolve to declared name if available.
        if (trimmed.StartsWith("``", StringComparison.Ordinal))
        {
            var rest = trimmed.AsSpan(2);
            int n = 0;
            while (n < rest.Length && char.IsDigit(rest[n])) n++;
            if (n > 0 && int.TryParse(rest[..n], out var idx))
            {
                var name = ctx?.MethodParameterNames is { } mn && idx < mn.Count ? mn[idx] : "";
                return name + new string(rest[n..]) + arraySuffix + (isByRef ? "&" : "");
            }
        }
        if (trimmed.StartsWith("`", StringComparison.Ordinal))
        {
            var rest = trimmed.AsSpan(1);
            int n = 0;
            while (n < rest.Length && char.IsDigit(rest[n])) n++;
            if (n > 0 && int.TryParse(rest[..n], out var idx))
            {
                var name = ctx?.TypeParameterNames is { } tn && idx < tn.Count ? tn[idx] : "";
                return name + new string(rest[n..]) + arraySuffix + (isByRef ? "&" : "");
            }
        }
        // Strip backtick arity even when there are no braces (e.g. embedded inside
        // a non-generic-resolved type spec leftover).
        if (trimmed.Contains('`'))
        {
            var sb = new StringBuilder();
            int j = 0;
            while (j < trimmed.Length)
            {
                if (trimmed[j] == '`')
                {
                    // Skip digits.
                    j++;
                    while (j < trimmed.Length && char.IsDigit(trimmed[j])) j++;
                    continue;
                }
                sb.Append(trimmed[j]);
                j++;
            }
            trimmed = sb.ToString();
        }
        return ShortenWellKnownName(trimmed) + arraySuffix + (isByRef ? "&" : "");
    }

    static string ShortenWellKnownName(string fqName)
    {
        return fqName switch
        {
            "System.String" => "string",
            "System.Int32" => "int",
            "System.Int64" => "long",
            "System.Int16" => "short",
            "System.Byte" => "byte",
            "System.SByte" => "sbyte",
            "System.UInt16" => "ushort",
            "System.UInt32" => "uint",
            "System.UInt64" => "ulong",
            "System.Boolean" => "bool",
            "System.Char" => "char",
            "System.Double" => "double",
            "System.Single" => "float",
            "System.Decimal" => "decimal",
            "System.Void" => "void",
            "System.Object" => "object",
            "System.IntPtr" => "nint",
            "System.UIntPtr" => "nuint",
            _ => fqName,
        };
    }

    /// <summary>
    /// Splits a comma-separated generic argument list at top-level commas (i.e.
    /// commas not nested inside braces).
    /// </summary>
    public static List<string> SplitGenericArgs(string body)
    {
        var args = new List<string>();
        int depth = 0;
        int start = 0;
        for (int i = 0; i < body.Length; i++)
        {
            var c = body[i];
            if (c == '{') depth++;
            else if (c == '}') depth--;
            else if (c == ',' && depth == 0)
            {
                args.Add(body[start..i]);
                start = i + 1;
            }
        }
        args.Add(body[start..]);
        return args;
    }

    /// <summary>
    /// Builds an XML doc id for a method: `M:Type.Method(P1,P2,…)` or `M:Type.Method`
    /// (no parens when there are no params, per the C# spec).
    /// Generic method arity is encoded as `Method``N`.
    /// </summary>
    public static string MakeMethodDocId(
        string typeFullName,
        string methodName,
        IReadOnlyList<ParameterRecord> parameters,
        IReadOnlyList<string> methodGenericParams)
    {
        var sb = new StringBuilder();
        sb.Append("M:");
        sb.Append(typeFullName);
        sb.Append('.');
        sb.Append(methodName);
        if (methodGenericParams.Count > 0)
        {
            sb.Append("``");
            sb.Append(methodGenericParams.Count);
        }
        if (parameters.Count > 0)
        {
            sb.Append('(');
            for (int i = 0; i < parameters.Count; i++)
            {
                if (i > 0) sb.Append(',');
                var p = parameters[i];
                sb.Append(p.Type);
                if (p.IsByRef && !p.Type.EndsWith("@", StringComparison.Ordinal))
                    sb.Append('@');
            }
            sb.Append(')');
        }
        return sb.ToString();
    }

    /// <summary>
    /// Builds an XML doc id for a property: `P:Type.Name` or, for indexers,
    /// `P:Type.Item(P1,P2,…)`.
    /// </summary>
    public static string MakePropertyDocId(
        string typeFullName,
        string propertyName,
        IReadOnlyList<ParameterRecord> indexParams)
    {
        var sb = new StringBuilder();
        sb.Append("P:");
        sb.Append(typeFullName);
        sb.Append('.');
        sb.Append(propertyName);
        if (indexParams.Count > 0)
        {
            sb.Append('(');
            for (int i = 0; i < indexParams.Count; i++)
            {
                if (i > 0) sb.Append(',');
                sb.Append(indexParams[i].Type);
            }
            sb.Append(')');
        }
        return sb.ToString();
    }

    static string? ResolveTypeHandle(MetadataReader mr, EntityHandle handle, SigTypeProvider provider)
    {
        if (handle.IsNil) return null;
        try
        {
            return handle.Kind switch
            {
                HandleKind.TypeDefinition => provider.GetTypeFromDefinition(mr, (TypeDefinitionHandle)handle, 0),
                HandleKind.TypeReference => provider.GetTypeFromReference(mr, (TypeReferenceHandle)handle, 0),
                HandleKind.TypeSpecification => mr.GetTypeSpecification((TypeSpecificationHandle)handle)
                    .DecodeSignature(provider, new GenericContext(Array.Empty<string>(), Array.Empty<string>())),
                _ => null,
            };
        }
        catch { return null; }
    }

    static object? ReadConstantValue(MetadataReader mr, Constant constant)
    {
        var reader = mr.GetBlobReader(constant.Value);
        return constant.TypeCode switch
        {
            ConstantTypeCode.Boolean => reader.ReadBoolean(),
            ConstantTypeCode.Char => reader.ReadChar(),
            ConstantTypeCode.SByte => reader.ReadSByte(),
            ConstantTypeCode.Byte => reader.ReadByte(),
            ConstantTypeCode.Int16 => reader.ReadInt16(),
            ConstantTypeCode.UInt16 => reader.ReadUInt16(),
            ConstantTypeCode.Int32 => reader.ReadInt32(),
            ConstantTypeCode.UInt32 => reader.ReadUInt32(),
            ConstantTypeCode.Int64 => reader.ReadInt64(),
            ConstantTypeCode.UInt64 => reader.ReadUInt64(),
            ConstantTypeCode.Single => reader.ReadSingle(),
            ConstantTypeCode.Double => reader.ReadDouble(),
            ConstantTypeCode.String => reader.ReadUTF16(reader.RemainingBytes / 2),
            ConstantTypeCode.NullReference => null,
            _ => null,
        };
    }

    static string? FormatConstantLiteral(object? v)
    {
        if (v == null) return "null";
        return v switch
        {
            string s => "\"" + s.Replace("\\", "\\\\").Replace("\"", "\\\"") + "\"",
            bool b => b ? "true" : "false",
            char c => "'" + c + "'",
            _ => Convert.ToString(v, System.Globalization.CultureInfo.InvariantCulture),
        };
    }
}

/// <summary>
/// Tracks the type-parameter names available at the current decoding site.
/// The XML doc id format encodes type-parameter references positionally
/// (``0, ``1 for method generics; `0, `1 for type generics) — we keep the
/// names for nicer reproductions but the position is what matters.
/// </summary>
public readonly record struct GenericContext(
    IReadOnlyList<string> TypeParameterNames,
    IReadOnlyList<string> MethodParameterNames);

/// <summary>
/// ISignatureTypeProvider that emits XML-doc-id-style type strings (e.g.
/// "System.Collections.Generic.IEnumerable{System.Int32}"). The result is
/// directly usable as a key (or part of a key) into an XmlDocLoader dictionary.
/// </summary>
internal sealed class SigTypeProvider :
    ISignatureTypeProvider<string, GenericContext>,
    ICustomAttributeTypeProvider<string>
{
    readonly MetadataReader _mr;

    public SigTypeProvider(MetadataReader mr) { _mr = mr; }

    public string GetPrimitiveType(PrimitiveTypeCode typeCode) => typeCode switch
    {
        PrimitiveTypeCode.Boolean => "System.Boolean",
        PrimitiveTypeCode.Byte => "System.Byte",
        PrimitiveTypeCode.SByte => "System.SByte",
        PrimitiveTypeCode.Char => "System.Char",
        PrimitiveTypeCode.Int16 => "System.Int16",
        PrimitiveTypeCode.UInt16 => "System.UInt16",
        PrimitiveTypeCode.Int32 => "System.Int32",
        PrimitiveTypeCode.UInt32 => "System.UInt32",
        PrimitiveTypeCode.Int64 => "System.Int64",
        PrimitiveTypeCode.UInt64 => "System.UInt64",
        PrimitiveTypeCode.IntPtr => "System.IntPtr",
        PrimitiveTypeCode.UIntPtr => "System.UIntPtr",
        PrimitiveTypeCode.Single => "System.Single",
        PrimitiveTypeCode.Double => "System.Double",
        PrimitiveTypeCode.String => "System.String",
        PrimitiveTypeCode.Object => "System.Object",
        PrimitiveTypeCode.Void => "System.Void",
        PrimitiveTypeCode.TypedReference => "System.TypedReference",
        _ => typeCode.ToString(),
    };

    public string GetTypeFromDefinition(MetadataReader reader, TypeDefinitionHandle handle, byte rawTypeKind)
    {
        var td = reader.GetTypeDefinition(handle);
        var name = reader.GetString(td.Name);
        var ns = td.Namespace.IsNil ? "" : reader.GetString(td.Namespace);
        // Nested type: walk up the declaring chain joining with '+' (CLR canonical) then translate
        // to '.' for XML doc id form.
        if (td.IsNested)
        {
            var parts = new List<string> { name };
            var declHandle = td.GetDeclaringType();
            while (!declHandle.IsNil)
            {
                var parent = reader.GetTypeDefinition(declHandle);
                parts.Add(reader.GetString(parent.Name));
                if (!parent.IsNested)
                {
                    if (!parent.Namespace.IsNil)
                        ns = reader.GetString(parent.Namespace);
                    declHandle = default;
                }
                else
                {
                    declHandle = parent.GetDeclaringType();
                }
            }
            parts.Reverse();
            name = string.Join('.', parts);
        }
        return string.IsNullOrEmpty(ns) ? name : ns + "." + name;
    }

    public string GetTypeFromReference(MetadataReader reader, TypeReferenceHandle handle, byte rawTypeKind)
    {
        var tr = reader.GetTypeReference(handle);
        var name = reader.GetString(tr.Name);
        var ns = tr.Namespace.IsNil ? "" : reader.GetString(tr.Namespace);
        // Nested type ref: parent is another TypeReferenceHandle.
        if (tr.ResolutionScope.Kind == HandleKind.TypeReference)
        {
            var parent = GetTypeFromReference(reader, (TypeReferenceHandle)tr.ResolutionScope, rawTypeKind);
            return parent + "." + name;
        }
        return string.IsNullOrEmpty(ns) ? name : ns + "." + name;
    }

    public string GetTypeFromSpecification(MetadataReader reader, GenericContext genericContext, TypeSpecificationHandle handle, byte rawTypeKind)
    {
        return reader.GetTypeSpecification(handle).DecodeSignature(this, genericContext);
    }

    public string GetSZArrayType(string elementType) => elementType + "[]";

    public string GetArrayType(string elementType, ArrayShape shape)
    {
        if (shape.Rank == 1) return elementType + "[]";
        var sb = new StringBuilder();
        sb.Append(elementType);
        sb.Append('[');
        sb.Append(new string(',', shape.Rank - 1));
        sb.Append(']');
        return sb.ToString();
    }

    public string GetByReferenceType(string elementType) => elementType + "@";

    public string GetPointerType(string elementType) => elementType + "*";

    public string GetGenericInstantiation(string genericType, ImmutableArray<string> typeArguments)
    {
        // Strip the trailing `Tn arity marker the generic-type-def carries (List`1 → List).
        var name = genericType;
        var tickIdx = name.IndexOf('`');
        if (tickIdx >= 0) name = name[..tickIdx];
        return name + "{" + string.Join(",", typeArguments) + "}";
    }

    // Returns the XML-doc-id encoding for generic params — single backtick for
    // type-level (`0, `1) and double backtick for method-level (``0, ``1). The
    // XML doc id uses this verbatim, so we keep it for ParameterRecord.Type which
    // is what MakeMethodDocId consumes when building the doc-id lookup key.
    // Pretty-printing for human-readable signatures resolves these back to names
    // (see PrettyType + BuildSignature).
    public string GetGenericMethodParameter(GenericContext genericContext, int index) => "``" + index;
    public string GetGenericTypeParameter(GenericContext genericContext, int index) => "`" + index;

    public string GetModifiedType(string modifier, string unmodifiedType, bool isRequired) => unmodifiedType;

    public string GetPinnedType(string elementType) => elementType;

    public string GetFunctionPointerType(MethodSignature<string> signature)
    {
        var sb = new StringBuilder();
        sb.Append("function ");
        sb.Append(signature.ReturnType);
        sb.Append('(');
        sb.Append(string.Join(",", signature.ParameterTypes));
        sb.Append(')');
        return sb.ToString();
    }

    // ICustomAttributeTypeProvider — minimal impls so MetadataReader code paths that
    // accept either provider compile against this one. Not actually used by the
    // extractor (we don't decode attribute values), but the interface is required by
    // some overloads.
    public string GetSystemType() => "System.Type";
    public bool IsSystemType(string type) => type == "System.Type";
    public string GetTypeFromSerializedName(string name) => name;
    public PrimitiveTypeCode GetUnderlyingEnumType(string type) => PrimitiveTypeCode.Int32;
}

