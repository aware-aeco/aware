using System.Runtime.CompilerServices;
using System.Xml.Linq;
using AwareReader;
using Microsoft.Build.Locator;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.MSBuild;
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
///
/// A <c>.csproj</c>/<c>.sln</c> input (#185) is loaded via <see cref="MSBuildWorkspace"/> — the
/// <c>.cs</c> set + all <c>PackageReference</c>/<c>ProjectReference</c>s are resolved
/// automatically (no <c>--reference-dir</c> needed). Each project's <see cref="Compilation"/> is
/// fed through the SAME <see cref="ReflectCompilation"/> symbol→IR path, so the project path and
/// the bare-<c>.cs</c> path produce identical IR. This path needs the host .NET SDK installed
/// (<see cref="Microsoft.Build.Locator"/> loads its MSBuild); the bare-<c>.cs</c> path does not.
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
        // A .csproj/.sln input goes through MSBuildWorkspace (#185). Register the host SDK's
        // MSBuild FIRST — before the workspace loads any Microsoft.Build assembly — then call the
        // (non-inlined) graph reader so the MSBuild types it references are only JIT-loaded after
        // registration. The bare-.cs path below never touches MSBuild, so it needs no SDK.
        if (paths.Any(IsProjectOrSolution))
        {
            EnsureMSBuildLocator();
            return ReflectProjectGraph(paths);
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

    // ── Project/solution graph loading via MSBuildWorkspace (#185) ─────────────

    private static bool _msbuildRegistered;

    /// <summary>
    /// Register a host .NET SDK's MSBuild with <see cref="MSBuildLocator"/> exactly once, so
    /// <see cref="MSBuildWorkspace"/> can load <c>Microsoft.Build.*</c> at run time (our build pins
    /// those packages with <c>ExcludeAssets=runtime</c>, so the SDK's copy is authoritative). Must
    /// run before any MSBuild type loads — hence the call site precedes the non-inlined
    /// <see cref="ReflectProjectGraph"/>. Throws an actionable error when no SDK is present.
    /// </summary>
    private static void EnsureMSBuildLocator()
    {
        if (_msbuildRegistered) return;
        var instances = MSBuildLocator.QueryVisualStudioInstances().ToList();
        if (instances.Count == 0)
        {
            throw new InvalidOperationException(
                "--from-csproj/--from-sln needs the .NET SDK installed (its MSBuild is loaded at "
                + "run time); install it from https://dotnet.microsoft.com, or use --from-csharp "
                + "with --reference-dir pointing at the SDK's DLLs.");
        }
        // Highest version available — matches the newest installed SDK.
        MSBuildLocator.RegisterInstance(instances.OrderByDescending(i => i.Version).First());
        _msbuildRegistered = true;
    }

    /// <summary>
    /// Load each <c>.csproj</c> (or every C# project in a <c>.sln</c>) via MSBuildWorkspace,
    /// resolving its package/project references, and map every project's <see cref="Compilation"/>
    /// through <see cref="ReflectCompilation"/> — the SAME symbol→IR path the bare-<c>.cs</c> and
    /// compiled-DLL readers use. Non-C# projects in a mixed solution are skipped.
    /// <para>
    /// <c>NoInlining</c> keeps the MSBuild-referencing body out of <see cref="ReflectPaths"/>'s JIT
    /// frame, so MSBuild assemblies load only when this method is first called — after
    /// <see cref="EnsureMSBuildLocator"/> has registered the SDK.
    /// </para>
    /// </summary>
    [MethodImpl(MethodImplOptions.NoInlining)]
    private static SourceReflection ReflectProjectGraph(IReadOnlyList<string> paths)
    {
        using var workspace = MSBuildWorkspace.Create();
        var failures = new List<string>();
        workspace.WorkspaceFailed += (_, e) =>
        {
            // Record ALL hard FAILUREs; whether each is actually fatal is decided AFTER load
            // (below) by correlating it with the projects that produced a compilation. The
            // Kind/message can't be trusted on its own: MSBuildWorkspace surfaces benign MSBuild
            // *warnings* — notably NuGet advisory warnings NU1901-NU1904 — as `Failure`
            // diagnostics that name the project's `.csproj`, even though the project compiles
            // fine (open bug dotnet/roslyn#75182). Keying fatality on the message alone would
            // reject any otherwise-buildable project that has a flagged transitive dependency.
            if (e.Diagnostic.Kind == WorkspaceDiagnosticKind.Failure)
                failures.Add(e.Diagnostic.Message);
        };

        var projects = new List<Project>();
        foreach (var path in paths)
        {
            if (path.EndsWith(".sln", StringComparison.OrdinalIgnoreCase))
                projects.AddRange(workspace.OpenSolutionAsync(path).GetAwaiter().GetResult().Projects);
            else
                projects.Add(workspace.OpenProjectAsync(path).GetAwaiter().GetResult());
        }

        var docs = new Dictionary<string, string>(StringComparer.Ordinal);
        var asms = new List<AssemblyRecord>();
        var index = new Dictionary<string, TypeRecord>(StringComparer.Ordinal);
        // File paths of the C# projects that produced a compilation — used below to decide which
        // recorded failures are real (a project that compiled did NOT fail to load).
        var loaded = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        foreach (var project in projects)
        {
            if (project.Language != LanguageNames.CSharp) continue; // C#-shaped IR only
            var comp = project.GetCompilationAsync().GetAwaiter().GetResult();
            if (comp is null) continue;
            if (project.FilePath is { Length: > 0 } fp) loaded.Add(fp);
            var asm = ReflectCompilation(comp, docs);
            asms.Add(asm);
            foreach (var t in asm.Types) index.TryAdd(t.FullName, t);
        }

        // A failure is fatal ONLY when the project it is ABOUT did not produce a compilation.
        // MSBuildWorkspace names that project as the FIRST `'…'`-quoted path in the message
        // ("Msbuild failed when processing the file '<path>' with message: …"). Match ONLY that
        // token against `loaded` — never the free message body — so:
        //   • a project that compiled despite a warning-as-failure (an MSBuild/NuGet WARNING
        //     surfaced as Failure, dotnet/roslyn#75182) is in `loaded`, so its own diagnostic is
        //     filtered out and not treated as fatal; and
        //   • a genuine C# load failure (its project absent from `loaded`) still surfaces — no
        //     silent partial — even when its message body happens to mention a loaded sibling's
        //     path (e.g. a project-reference resolution error). A message with no quoted path
        //     can't be attributed, so it is treated as fatal (fail loud, never drop silently).
        static string? FailedProjectPath(string msg)
        {
            // Anchor to the diagnostic template ("…processing the file '{0}' with message: {1}")
            // rather than the first quote pair, so a project path that itself contains an
            // apostrophe (legal on Windows + Linux, e.g. C:\Users\O'Brien\App.csproj) isn't
            // truncated — the closing delimiter is `' with message:`, not just `'` (review).
            const string pre = "processing the file '";
            const string post = "' with message:";
            var start = msg.IndexOf(pre, StringComparison.Ordinal);
            if (start < 0) return null;
            start += pre.Length;
            var end = msg.IndexOf(post, start, StringComparison.Ordinal);
            return end > start ? msg[start..end] : null;
        }
        var fatal = failures
            .Where(msg => FailedProjectPath(msg) is not { } p || !loaded.Contains(p))
            .ToList();
        if (asms.Count == 0)
        {
            var detail = fatal.Count > 0 ? $" ({string.Join("; ", fatal.Take(3))})" : "";
            throw new InvalidOperationException(
                "no C# project compilation could be loaded from the given .csproj/.sln" + detail
                + "; ensure the .NET SDK is installed and the project restores.");
        }
        if (fatal.Count > 0)
        {
            throw new InvalidOperationException(
                "the .csproj/.sln did not load cleanly, so reflection would be incomplete: "
                + string.Join("; ", fatal.Take(5))
                + ". Ensure the .NET SDK is installed and every project restores, or pass a "
                + "specific project with --from-csproj.");
        }

        return new SourceReflection(new ReflectedSet(asms, index), docs);
    }

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

        // Parity with the compiled reader: surface the assembly version (→ sdk-target) and
        // [assembly: AssemblyDescription] when the source declares them.
        var version = comp.Assembly.Identity.Version?.ToString() ?? "0.1.0";
        var description = comp.Assembly.GetAttributes()
            .FirstOrDefault(a => a.AttributeClass?.ToDisplayString(Fqn)
                == "System.Reflection.AssemblyDescriptionAttribute")
            ?.ConstructorArguments.FirstOrDefault().Value as string;

        return new AssemblyRecord(comp.AssemblyName ?? "source", version, types) { Description = description };
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
        // Match the compiled reader's field order: enums by underlying value, else by name.
        if (kind == IrTypeKind.Enum)
            fields.Sort((a, b) => EnumOrdinalKey(a.ConstantValue).CompareTo(EnumOrdinalKey(b.ConstantValue)));
        else
            fields.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));

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
            DefaultValueLiteral: p is { HasExplicitDefaultValue: true } ? FormatConstLiteral(p.ExplicitDefaultValue) : null);

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
            EnumUnderlyingValueLiteral: f.IsConst ? FormatConstLiteral(f.ConstantValue) : null)
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
        _ => FormatAttrValue(tc.Value), // enums arrive as their underlying integer
    };

    // Attribute-argument values: UNquoted, matching AwareReader.AttributeReader.FormatValue.
    private static string? FormatAttrValue(object? v) => v switch
    {
        null => null,
        string s => s,
        bool b => b ? "true" : "false",
        char c => c.ToString(),
        _ => Convert.ToString(v, System.Globalization.CultureInfo.InvariantCulture),
    };

    // Const / default-parameter literals: QUOTED, matching MetadataReflector.FormatConstantLiteral.
    private static string? FormatConstLiteral(object? v) => v switch
    {
        null => "null",
        string s => "\"" + s.Replace("\\", "\\\\").Replace("\"", "\\\"") + "\"",
        bool b => b ? "true" : "false",
        char c => "'" + c + "'",
        _ => Convert.ToString(v, System.Globalization.CultureInfo.InvariantCulture),
    };

    // Sort key for enum members by underlying value (mirrors MetadataReflector's OrdinalKey).
    private static long EnumOrdinalKey(object? constantValue) => constantValue switch
    {
        sbyte sb => sb,
        byte b => b,
        short sh => sh,
        ushort us => us,
        int i => i,
        uint u => u,
        long l => l,
        ulong ul => unchecked((long)ul),
        _ => 0,
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
