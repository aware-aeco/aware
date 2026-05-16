using System.Reflection;
using System.Reflection.Metadata;
using System.Reflection.PortableExecutable;
using System.Runtime.InteropServices;
using System.Xml.Linq;
using AwareSidecar.Protocol;

namespace AwareSidecar.Reflection;

/// <summary>
/// Reflects .NET assemblies via System.Reflection.Metadata.PEReader and MetadataReader —
/// reads PE metadata directly from the file without loading or executing any managed code.
/// Emits a GeneratedAgent: one command per public method on each public type.
/// XML doc summaries are picked up from sibling .xml files when present.
/// </summary>
/// <remarks>
/// Using MetadataReader (instead of MetadataLoadContext) eliminates StackOverflowException
/// on large type graphs (e.g. DevExpress WPF assemblies in Tekla 2025, Revit 2026) because
/// MetadataReader is a purely iterative PE token table reader — it never resolves types
/// across assembly boundaries and therefore never recurses into the CLR type loader.
/// </remarks>
public static class DllReflector
{
    public static GeneratedAgent Reflect(string[] globs, string? agentIdOverride)
    {
        var dllPaths = ResolveGlobs(globs);
        if (dllPaths.Count == 0)
        {
            throw new InvalidOperationException($"no dlls matched globs: {string.Join(", ", globs)}");
        }

        // Run on a dedicated thread with a large stack as a defence-in-depth measure.
        // MetadataReader is non-recursive by design, but reading and processing thousands
        // of type/method tokens from 473 DLLs still benefits from extra stack headroom.
        GeneratedAgent result = default!;
        Exception? threadEx = null;

        var workerThread = new System.Threading.Thread(
            () =>
            {
                try { result = ReflectCore(dllPaths, agentIdOverride); }
                catch (Exception ex) { threadEx = ex; }
            },
            64 * 1024 * 1024); // 64 MB — defence in depth for large product directories

        workerThread.Start();
        workerThread.Join();

        if (threadEx != null)
            System.Runtime.ExceptionServices.ExceptionDispatchInfo.Capture(threadEx).Throw();

        return result;
    }

    /// <summary>
    /// Inner reflection loop.  Uses System.Reflection.Metadata.PEReader + MetadataReader to
    /// enumerate types and methods directly from PE token tables — no type resolution, no SOE.
    /// </summary>
    private static GeneratedAgent ReflectCore(List<string> dllPaths, string? agentIdOverride)
    {
        var commands = new List<GeneratedCommand>();
        var skills = new List<GeneratedSkill>();
        string? assemblyName = null;
        string? assemblyDescription = null;

        foreach (var dllPath in dllPaths)
        {
            using var fileStream = TryOpenFile(dllPath);
            if (fileStream == null) continue;

            PEReader peReader;
            try { peReader = new PEReader(fileStream); }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"skip {dllPath}: {ex.Message}");
                continue;
            }

            using (peReader)
            {
                if (!peReader.HasMetadata)
                {
                    Console.Error.WriteLine($"skip {dllPath}: This PE image is not a managed executable.");
                    continue;
                }

                MetadataReader mr;
                try { mr = peReader.GetMetadataReader(); }
                catch (Exception ex)
                {
                    Console.Error.WriteLine($"skip {dllPath}: {ex.Message}");
                    continue;
                }

                // Read assembly identity and description from the Assembly table.
                if (assemblyName == null && mr.IsAssembly)
                {
                    var asmDef = mr.GetAssemblyDefinition();
                    assemblyName = mr.GetString(asmDef.Name);
                    assemblyDescription = ReadAssemblyDescription(mr, asmDef);
                }

                // Load sibling XML doc file if present.
                var xmlDocPath = Path.ChangeExtension(dllPath, ".xml");
                var xmlDoc = File.Exists(xmlDocPath) ? LoadXmlDoc(xmlDocPath) : new Dictionary<string, string>();

                // Per-namespace skill aggregating type names (string, not Type — no resolution).
                var typesByNamespace = new SortedDictionary<string, List<string>>(StringComparer.Ordinal);

                foreach (var typeHandle in mr.TypeDefinitions)
                {
                    TypeDefinition typeDef;
                    try { typeDef = mr.GetTypeDefinition(typeHandle); }
                    catch { continue; }

                    // Only public types: TypeAttributes.Public or TypeAttributes.NestedPublic.
                    // We skip nested types for simplicity (same as MLC DeclaredOnly behaviour).
                    var attrs = typeDef.Attributes;
                    var visibility = attrs & TypeAttributes.VisibilityMask;
                    if (visibility != TypeAttributes.Public) continue;

                    // Only classes and interfaces (mirrors the old IsClass/IsInterface filter).
                    var isClass = (attrs & TypeAttributes.ClassSemanticsMask) == TypeAttributes.Class
                                  && (attrs & TypeAttributes.Interface) == 0;
                    var isInterface = (attrs & TypeAttributes.Interface) != 0;
                    if (!isClass && !isInterface) continue;

                    var typeName = mr.GetString(typeDef.Name);
                    var ns = typeDef.Namespace.IsNil ? "(global)" : mr.GetString(typeDef.Namespace);

                    if (!typesByNamespace.ContainsKey(ns))
                        typesByNamespace[ns] = new List<string>();
                    typesByNamespace[ns].Add(typeName);

                    // Emit one command per declared public instance/static method.
                    foreach (var methodHandle in typeDef.GetMethods())
                    {
                        MethodDefinition methodDef;
                        try { methodDef = mr.GetMethodDefinition(methodHandle); }
                        catch { continue; }

                        var methodAttrs = methodDef.Attributes;

                        // Public only.
                        if ((methodAttrs & MethodAttributes.MemberAccessMask) != MethodAttributes.Public)
                            continue;

                        // Skip special names (property getters/setters, event add/remove, etc.).
                        if ((methodAttrs & MethodAttributes.SpecialName) != 0) continue;

                        var methodName = mr.GetString(methodDef.Name);

                        // XML doc lookup. The XML doc format uses
                        //   M:Type.Method                      (no-args methods)
                        //   M:Type.Method(Param1,Param2,…)     (methods with parameters)
                        // So we first try the exact no-args key, then fall back to a prefix
                        // scan for any signature variant — picking the first match for
                        // overloaded methods.
                        var typeFullName = ns == "(global)" ? typeName : $"{ns}.{typeName}";
                        var memberKey = $"M:{typeFullName}.{methodName}";
                        var summary = xmlDoc.GetValueOrDefault(memberKey)
                                      ?? FindXmlSummaryByPrefix(xmlDoc, memberKey)
                                      ?? $"{typeName}.{methodName}";

                        var cmdName = ToKebab($"{typeName}-{methodName}");
                        commands.Add(new GeneratedCommand
                        {
                            Name = cmdName,
                            Lifecycle = "single",
                            Description = summary,
                        });
                    }
                }

                // One skill per namespace summarizing the types.
                foreach (var (ns, typeNames) in typesByNamespace)
                {
                    var safeNs = ToKebab(ns);
                    var skillFilename = $"{safeNs}.md";
                    if (skills.Any(s => s.Filename == skillFilename)) continue; // dedupe

                    var body = new System.Text.StringBuilder();
                    body.AppendLine("---");
                    body.AppendLine($"name: {ToKebab(assemblyName ?? "assembly")}-{safeNs}");
                    body.AppendLine($"description: API reference for namespace {ns} from {Path.GetFileName(dllPath)}");
                    body.AppendLine("---");
                    body.AppendLine();
                    body.AppendLine($"# {ns}");
                    body.AppendLine();
                    foreach (var typeName in typeNames.OrderBy(n => n, StringComparer.Ordinal))
                    {
                        body.AppendLine($"- **{typeName}**");
                        var typeFullName = ns == "(global)" ? typeName : $"{ns}.{typeName}";
                        var typeSummary = xmlDoc.GetValueOrDefault($"T:{typeFullName}");
                        if (!string.IsNullOrWhiteSpace(typeSummary))
                            body.AppendLine($"  - {typeSummary}");
                    }
                    skills.Add(new GeneratedSkill { Filename = skillFilename, Body = body.ToString() });
                }
            }
        }

        commands = commands.OrderBy(c => c.Name, StringComparer.Ordinal).ToList();
        skills = skills.OrderBy(s => s.Filename, StringComparer.Ordinal).ToList();

        var id = agentIdOverride ?? ToKebab(assemblyName ?? "reflected-agent");
        return new GeneratedAgent
        {
            Id = id,
            Version = "0.1.0",
            Description = assemblyDescription ?? $"Generated from {dllPaths.Count} assembly file(s) via reflection.",
            License = "UNKNOWN",
            Stateful = false,
            Commands = commands.ToArray(),
            Skills = skills.ToArray(),
        };
    }

    /// <summary>
    /// Opens a file for reading; returns null and prints a skip message on failure.
    /// </summary>
    private static FileStream? TryOpenFile(string path)
    {
        try { return new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.Read); }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"skip {path}: {ex.Message}");
            return null;
        }
    }

    /// <summary>
    /// Reads the AssemblyDescriptionAttribute value from assembly-level custom attributes
    /// using MetadataReader (no type loading, no recursion).
    /// </summary>
    private static string? ReadAssemblyDescription(MetadataReader mr, AssemblyDefinition asmDef)
    {
        foreach (var caHandle in asmDef.GetCustomAttributes())
        {
            try
            {
                var ca = mr.GetCustomAttribute(caHandle);
                // Resolve the constructor to its declaring type name.
                string? ctorTypeName = null;
                if (ca.Constructor.Kind == HandleKind.MemberReference)
                {
                    var memberRef = mr.GetMemberReference((MemberReferenceHandle)ca.Constructor);
                    if (memberRef.Parent.Kind == HandleKind.TypeReference)
                    {
                        var typeRef = mr.GetTypeReference((TypeReferenceHandle)memberRef.Parent);
                        ctorTypeName = mr.GetString(typeRef.Name);
                    }
                }
                else if (ca.Constructor.Kind == HandleKind.MethodDefinition)
                {
                    var methodDef = mr.GetMethodDefinition((MethodDefinitionHandle)ca.Constructor);
                    var typeDef = mr.GetTypeDefinition(methodDef.GetDeclaringType());
                    ctorTypeName = mr.GetString(typeDef.Name);
                }

                if (ctorTypeName != "AssemblyDescriptionAttribute") continue;

                // The attribute blob: prolog (2 bytes 0x01 0x00) + UTF-8 length-prefixed string.
                var value = ca.DecodeValue(new SimpleAttributeDecoder());
                if (value.FixedArguments.Length > 0 && value.FixedArguments[0].Value is string s)
                    return s;
            }
            catch { /* skip unreadable attribute */ }
        }
        return null;
    }

    private static List<string> ResolveGlobs(string[] globs)
    {
        var out_ = new List<string>();
        foreach (var g in globs)
        {
            // Case 1: literal file path.
            if (File.Exists(g))
            {
                out_.Add(Path.GetFullPath(g));
                continue;
            }
            // Case 2: bare directory path — auto-expand to *.dll so callers do not
            // need to append \*.dll manually (e.g. --from-dlls "C:\Tekla\bin").
            if (Directory.Exists(g))
            {
                foreach (var hit in Directory.EnumerateFiles(g, "*.dll"))
                    out_.Add(Path.GetFullPath(hit));
                continue;
            }
            // Case 3: glob pattern — split directory + pattern and enumerate.
            var dir = Path.GetDirectoryName(g);
            var pattern = Path.GetFileName(g);
            if (string.IsNullOrEmpty(dir)) dir = ".";
            if (!Directory.Exists(dir)) continue;
            foreach (var hit in Directory.EnumerateFiles(dir, pattern))
                out_.Add(Path.GetFullPath(hit));
        }
        return out_.Distinct(StringComparer.OrdinalIgnoreCase).ToList();
    }

    /// <summary>
    /// Scan the XML doc dictionary for any key starting with `{prefix}(` —
    /// matches overloaded methods or methods with parameters where the
    /// XML doc key includes the typed parameter list. Returns the first
    /// match (overload selection is arbitrary; future enhancement could
    /// match arity).
    /// </summary>
    private static string? FindXmlSummaryByPrefix(Dictionary<string, string> xmlDoc, string prefix)
    {
        var paren = prefix + "(";
        foreach (var (key, value) in xmlDoc)
        {
            if (key.StartsWith(paren, StringComparison.Ordinal))
                return value;
        }
        return null;
    }

    private static Dictionary<string, string> LoadXmlDoc(string path)
    {
        var dict = new Dictionary<string, string>(StringComparer.Ordinal);
        try
        {
            var doc = XDocument.Load(path);
            foreach (var member in doc.Descendants("member"))
            {
                var name = (string?)member.Attribute("name");
                if (string.IsNullOrEmpty(name)) continue;
                var summary = member.Element("summary")?.Value.Trim().Replace('\n', ' ').Replace('\r', ' ');
                if (!string.IsNullOrEmpty(summary)) dict[name] = summary;
            }
        }
        catch { /* skip malformed XML docs */ }
        return dict;
    }

    private static string ToKebab(string s)
    {
        var sb = new System.Text.StringBuilder(s.Length + 4);
        var prevLower = false;
        foreach (var c in s)
        {
            if (char.IsUpper(c))
            {
                if (prevLower && sb.Length > 0) sb.Append('-');
                sb.Append(char.ToLowerInvariant(c));
                prevLower = false;
            }
            else if (char.IsWhiteSpace(c) || c == '_' || c == '.' || c == '-')
            {
                if (sb.Length > 0 && sb[sb.Length - 1] != '-') sb.Append('-');
                prevLower = false;
            }
            else if (char.IsLetterOrDigit(c))
            {
                sb.Append(char.ToLowerInvariant(c));
                prevLower = char.IsLower(c) || char.IsDigit(c);
            }
        }
        return sb.ToString().Trim('-');
    }

    /// <summary>
    /// Minimal ICustomAttributeTypeProvider implementation required by
    /// MetadataReader.CustomAttribute.DecodeValue() to decode fixed string arguments.
    /// </summary>
    private sealed class SimpleAttributeDecoder : ICustomAttributeTypeProvider<object>
    {
        public object GetPrimitiveType(PrimitiveTypeCode typeCode) => typeCode;
        public object GetSystemType() => typeof(Type);
        public object GetSZArrayType(object elementType) => Array.Empty<object>();
        public object GetTypeFromDefinition(MetadataReader reader, TypeDefinitionHandle handle, byte rawTypeKind) => handle;
        public object GetTypeFromReference(MetadataReader reader, TypeReferenceHandle handle, byte rawTypeKind) => handle;
        public object GetTypeFromSerializedName(string name) => name;
        public PrimitiveTypeCode GetUnderlyingEnumType(object type) => PrimitiveTypeCode.Int32;
        public bool IsSystemType(object type) => type is Type;
    }
}
