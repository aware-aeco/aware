using System.Xml.Linq;
using AwareReader;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.Text;
using RoslynTypeKind = Microsoft.CodeAnalysis.TypeKind;
using IrTypeKind = AwareReader.TypeKind;

namespace AwareRoslyn;

/// <summary>The reflected set plus the doc summaries harvested from the source's XML comments.</summary>
public sealed record SourceReflection(ReflectedSet Set, IReadOnlyDictionary<string, string> DocSummaries);

/// <summary>
/// Reads C# *source* via Roslyn and maps it onto the SAME <see cref="AwareReader"/> IR the
/// compiled-DLL reader produces, so <see cref="AgentSynthesizer"/> and the recipe layer treat
/// source and compiled inputs identically. Bare <c>.cs</c> files/dirs/globs compile standalone
/// with framework references plus any caller-supplied reference-dir DLLs; pointing a reference
/// dir at an SDK's bin resolves base types/attributes so the recipe fires on source.
/// (Project/solution graph loading via MSBuildWorkspace is intentionally out of scope — see the
/// csproj note — so a <c>.csproj</c>/<c>.sln</c> input returns a clear, actionable error.)
/// </summary>
public static class RoslynReader
{
    // Fully-qualified, no special-type keywords (System.Int32, not int) and no global:: — matches
    // the compiled reader's encoding closely so the recipe's type checks behave the same for both.
    private static readonly SymbolDisplayFormat Fqn = new(
        globalNamespaceStyle: SymbolDisplayGlobalNamespaceStyle.Omitted,
        typeQualificationStyle: SymbolDisplayTypeQualificationStyle.NameAndContainingTypesAndNamespaces,
        genericsOptions: SymbolDisplayGenericsOptions.IncludeTypeParameters,
        miscellaneousOptions: SymbolDisplayMiscellaneousOptions.ExpandNullable);

    public static SourceReflection ReflectPaths(IReadOnlyList<string> paths, IReadOnlyList<string> referenceDirs)
    {
        if (paths.Any(IsProjectOrSolution))
        {
            throw new InvalidOperationException(
                "project/solution (.csproj/.sln) graph loading is not supported in this build; "
                + "pass .cs files, a directory, or a glob to --from-csharp, with --reference-dir "
                + "pointing at the SDK's DLLs so base types and attributes resolve.");
        }

        var comp = BuildSourceCompilation(paths, referenceDirs);

        var docs = new Dictionary<string, string>(StringComparer.Ordinal);
        var asm = ReflectCompilation(comp, docs);
        var index = new Dictionary<string, TypeRecord>(StringComparer.Ordinal);
        foreach (var t in asm.Types) index.TryAdd(t.FullName, t);

        return new SourceReflection(new ReflectedSet(new[] { asm }, index), docs);
    }

    private static bool IsProjectOrSolution(string p) =>
        p.EndsWith(".csproj", StringComparison.OrdinalIgnoreCase)
        || p.EndsWith(".sln", StringComparison.OrdinalIgnoreCase);

    // ── Compilation construction ──────────────────────────────────────────────

    private static Compilation BuildSourceCompilation(
        IReadOnlyList<string> sourceInputs, IReadOnlyList<string> referenceDirs)
    {
        var files = new List<string>();
        foreach (var input in sourceInputs)
        {
            if (Directory.Exists(input))
                files.AddRange(Directory.EnumerateFiles(input, "*.cs", SearchOption.AllDirectories));
            else if (File.Exists(input))
                files.Add(input);
            else
            {
                var dir = Path.GetDirectoryName(input);
                var pattern = Path.GetFileName(input);
                if (string.IsNullOrEmpty(dir)) dir = ".";
                if (Directory.Exists(dir))
                    files.AddRange(Directory.EnumerateFiles(dir, pattern));
            }
        }

        var trees = files
            .Distinct(StringComparer.OrdinalIgnoreCase)
            .Select(f => CSharpSyntaxTree.ParseText(
                SourceText.From(File.ReadAllText(f)),
                new CSharpParseOptions(documentationMode: DocumentationMode.Parse),
                path: f))
            .ToList();
        if (trees.Count == 0)
            throw new InvalidOperationException("no .cs files matched the given source inputs");

        // Dedup references by file name (framework refs + any --reference-dir DLLs).
        var seen = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        var refs = new List<MetadataReference>();
        void AddRef(string dll)
        {
            if (seen.Add(Path.GetFileName(dll)) && File.Exists(dll))
                refs.Add(MetadataReference.CreateFromFile(dll));
        }

        var tpa = (AppContext.GetData("TRUSTED_PLATFORM_ASSEMBLIES") as string ?? "")
            .Split(Path.PathSeparator);
        foreach (var p in tpa)
            if (p.EndsWith(".dll", StringComparison.OrdinalIgnoreCase)) AddRef(p);
        foreach (var dir in referenceDirs)
            if (Directory.Exists(dir))
                foreach (var dll in Directory.EnumerateFiles(dir, "*.dll"))
                    AddRef(dll);

        return CSharpCompilation.Create(
            "source", trees, refs,
            new CSharpCompilationOptions(OutputKind.DynamicallyLinkedLibrary));
    }

    // ── Symbol → IR mapping ───────────────────────────────────────────────────

    private static AssemblyRecord ReflectCompilation(Compilation comp, Dictionary<string, string> docs)
    {
        var types = new List<TypeRecord>();
        foreach (var t in EnumerateNamespaceTypes(comp.GlobalNamespace))
        {
            // Only public, top-level types declared in THIS compilation (not referenced ones).
            if (t.DeclaredAccessibility != Accessibility.Public) continue;
            if (t.ContainingType is not null) continue;
            if (!SymbolEqualityComparer.Default.Equals(t.ContainingAssembly, comp.Assembly)) continue;
            if (t.TypeKind is not (RoslynTypeKind.Class or RoslynTypeKind.Struct or RoslynTypeKind.Interface
                or RoslynTypeKind.Enum or RoslynTypeKind.Delegate)) continue;

            types.Add(MapType(t, docs));
        }
        types.Sort((a, b) => string.CompareOrdinal(a.FullName, b.FullName));
        return new AssemblyRecord(comp.AssemblyName ?? "source", "0.1.0", types);
    }

    private static IEnumerable<INamedTypeSymbol> EnumerateNamespaceTypes(INamespaceSymbol ns)
    {
        foreach (var t in ns.GetTypeMembers()) yield return t;
        foreach (var child in ns.GetNamespaceMembers())
            foreach (var t in EnumerateNamespaceTypes(child))
                yield return t;
    }

    private static TypeRecord MapType(INamedTypeSymbol t, Dictionary<string, string> docs)
    {
        CollectDoc(t, docs);

        var ns = t.ContainingNamespace.IsGlobalNamespace ? "" : t.ContainingNamespace.ToDisplayString();
        var fullName = string.IsNullOrEmpty(ns) ? t.Name : $"{ns}.{t.Name}";

        var kind = t.TypeKind switch
        {
            RoslynTypeKind.Interface => IrTypeKind.Interface,
            RoslynTypeKind.Enum => IrTypeKind.Enum,
            RoslynTypeKind.Delegate => IrTypeKind.Delegate,
            RoslynTypeKind.Struct => IrTypeKind.Struct,
            _ => t.IsStatic ? IrTypeKind.StaticClass : IrTypeKind.Class,
        };

        // Mirror the compiled reader: implicit bases (Object/ValueType/Enum/MulticastDelegate)
        // and interface/enum/delegate/struct kinds report no base.
        string? baseName = null;
        if (kind == IrTypeKind.Class || kind == IrTypeKind.StaticClass)
        {
            var b = t.BaseType?.ToDisplayString(Fqn);
            if (!string.IsNullOrEmpty(b) && b != "System.Object") baseName = b;
        }

        var constructors = new List<MethodRecord>();
        var methods = new List<MethodRecord>();
        var properties = new List<PropertyRecord>();
        var events = new List<EventRecord>();
        var fields = new List<FieldRecord>();

        foreach (var member in t.GetMembers())
        {
            if (member.DeclaredAccessibility != Accessibility.Public) continue;
            switch (member)
            {
                case IMethodSymbol m when m.MethodKind == MethodKind.Constructor:
                    CollectDoc(m, docs);
                    constructors.Add(MapMethod(m));
                    break;
                case IMethodSymbol m when m.MethodKind is MethodKind.Ordinary
                        or MethodKind.UserDefinedOperator or MethodKind.Conversion:
                    CollectDoc(m, docs);
                    methods.Add(MapMethod(m));
                    break;
                case IPropertySymbol p:
                    CollectDoc(p, docs);
                    properties.Add(MapProperty(p));
                    break;
                case IEventSymbol e:
                    CollectDoc(e, docs);
                    events.Add(new EventRecord(e.Name, e.GetDocumentationCommentId() ?? $"E:{fullName}.{e.Name}",
                        e.Type.ToDisplayString(Fqn)));
                    break;
                case IFieldSymbol f when !f.IsImplicitlyDeclared:
                    CollectDoc(f, docs);
                    fields.Add(MapField(f));
                    break;
            }
        }

        constructors.Sort((a, b) => string.CompareOrdinal(a.Signature, b.Signature));
        methods.Sort((a, b) => string.CompareOrdinal(a.Signature, b.Signature));
        properties.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));
        events.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));

        return new TypeRecord(
            Namespace: ns,
            Name: t.Name,
            FullName: fullName,
            Kind: kind,
            IsPublic: true,
            IsStatic: t.IsStatic,
            IsSealed: t.IsSealed,
            IsAbstract: t.IsAbstract,
            BaseTypeName: baseName,
            Interfaces: t.Interfaces.Select(i => i.ToDisplayString(Fqn)).OrderBy(x => x, StringComparer.Ordinal).ToList(),
            GenericParameters: t.TypeParameters.Select(p => p.Name).ToList(),
            Constructors: constructors,
            Methods: methods,
            Properties: properties,
            Events: events,
            Fields: fields,
            DelegateInvokeSignature: null)
        {
            Attributes = MapAttributes(t.GetAttributes()),
        };
    }

    private static MethodRecord MapMethod(IMethodSymbol m)
    {
        bool isCtor = m.MethodKind == MethodKind.Constructor;
        bool isOperator = m.MethodKind is MethodKind.UserDefinedOperator or MethodKind.Conversion;
        return new MethodRecord(
            Name: isCtor ? m.ContainingType.Name : m.Name,
            DocId: m.GetDocumentationCommentId() ?? "",
            Signature: m.ToDisplayString(),
            IsStatic: m.IsStatic,
            IsConstructor: isCtor,
            IsAbstract: m.IsAbstract,
            IsVirtual: m.IsVirtual,
            IsOperator: isOperator,
            ReturnType: m.ReturnsVoid ? "System.Void" : m.ReturnType.ToDisplayString(Fqn),
            Parameters: m.Parameters.Select(MapParam).ToList(),
            GenericParameters: m.TypeParameters.Select(p => p.Name).ToList())
        {
            Attributes = MapAttributes(m.GetAttributes()),
        };
    }

    private static ParameterRecord MapParam(IParameterSymbol p) =>
        new(Name: p.Name,
            Type: p.Type.ToDisplayString(Fqn),
            IsByRef: p.RefKind != RefKind.None,
            IsIn: p.RefKind == RefKind.In,
            IsOut: p.RefKind == RefKind.Out,
            IsOptional: p.IsOptional,
            DefaultValueLiteral: p is { HasExplicitDefaultValue: true } ? FormatLiteral(p.ExplicitDefaultValue) : null);

    private static PropertyRecord MapProperty(IPropertySymbol p) =>
        new(Name: p.Name,
            DocId: p.GetDocumentationCommentId() ?? "",
            Type: p.Type.ToDisplayString(Fqn),
            HasGetter: p.GetMethod is { DeclaredAccessibility: Accessibility.Public },
            HasSetter: p.SetMethod is { DeclaredAccessibility: Accessibility.Public },
            IsStatic: p.IsStatic,
            IndexParameters: p.Parameters.Select(MapParam).ToList())
        {
            Attributes = MapAttributes(p.GetAttributes()),
        };

    private static FieldRecord MapField(IFieldSymbol f) =>
        new(Name: f.Name,
            DocId: f.GetDocumentationCommentId() ?? "",
            Type: f.Type.ToDisplayString(Fqn),
            IsStatic: f.IsStatic,
            IsLiteral: f.IsConst,
            ConstantValue: f.ConstantValue,
            EnumUnderlyingValueLiteral: f.IsConst ? FormatLiteral(f.ConstantValue) : null)
        {
            Attributes = MapAttributes(f.GetAttributes()),
        };

    private static IReadOnlyList<AttributeRecord> MapAttributes(IEnumerable<AttributeData> attrs)
    {
        var result = new List<AttributeRecord>();
        foreach (var a in attrs)
        {
            var typeName = a.AttributeClass?.ToDisplayString(Fqn);
            if (string.IsNullOrEmpty(typeName)) continue;
            var fixedArgs = a.ConstructorArguments
                .Select(c => new AttributeArg(c.Type?.ToDisplayString(Fqn) ?? "", FormatTyped(c)))
                .ToList();
            var namedArgs = a.NamedArguments
                .Select(n => new NamedArg(n.Key, n.Value.Type?.ToDisplayString(Fqn) ?? "", FormatTyped(n.Value)))
                .ToList();
            result.Add(new AttributeRecord(typeName!, fixedArgs, namedArgs));
        }
        result.Sort((a, b) => string.CompareOrdinal(a.TypeName, b.TypeName));
        return result;
    }

    private static string? FormatTyped(TypedConstant tc) => tc.Kind switch
    {
        TypedConstantKind.Array => "[" + string.Join(", ", tc.Values.Select(FormatTyped)) + "]",
        TypedConstantKind.Type => (tc.Value as ITypeSymbol)?.ToDisplayString(Fqn),
        _ => FormatLiteral(tc.Value), // enums arrive as their underlying integer
    };

    private static string? FormatLiteral(object? v) => v switch
    {
        null => null,
        string s => s,
        bool b => b ? "true" : "false",
        char c => c.ToString(),
        _ => Convert.ToString(v, System.Globalization.CultureInfo.InvariantCulture),
    };

    private static void CollectDoc(ISymbol sym, Dictionary<string, string> docs)
    {
        var id = sym.GetDocumentationCommentId();
        if (string.IsNullOrEmpty(id)) return;
        var xml = sym.GetDocumentationCommentXml();
        if (string.IsNullOrEmpty(xml)) return;
        try
        {
            var doc = XDocument.Parse(xml);
            var summary = doc.Descendants("summary").FirstOrDefault()?.Value;
            if (!string.IsNullOrWhiteSpace(summary))
                docs[id] = string.Join(" ", summary.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries));
        }
        catch { /* malformed doc xml — skip */ }
    }
}
