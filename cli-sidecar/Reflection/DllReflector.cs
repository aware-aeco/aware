using System.Diagnostics.CodeAnalysis;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Xml.Linq;
using AwareSidecar.Protocol;

namespace AwareSidecar.Reflection;

/// <summary>
/// Reflects .NET assemblies via MetadataLoadContext — loads metadata without executing code.
/// Emits a GeneratedAgent: one command per public method on each public type.
/// XML doc summaries are picked up from sibling .xml files when present.
/// </summary>
public static class DllReflector
{
    // IL2026 / IL2075: GetTypes() and GetMethods() are called on assemblies loaded via
    // MetadataLoadContext, which only loads metadata and never executes managed code.
    // The trimmer warnings are not applicable — no live types are reflected.
    [UnconditionalSuppressMessage("AOT", "IL2026", Justification = "MLC loads metadata only — no live code is reflected")]
    [UnconditionalSuppressMessage("AOT", "IL2075", Justification = "MLC loads metadata only — no live code is reflected")]
    public static GeneratedAgent Reflect(string[] globs, string? agentIdOverride)
    {
        var dllPaths = ResolveGlobs(globs);
        if (dllPaths.Count == 0)
        {
            throw new InvalidOperationException($"no dlls matched globs: {string.Join(", ", globs)}");
        }

        // Locate the .NET runtime directory for MetadataLoadContext reference assemblies.
        // typeof(object).Assembly.Location is empty in single-file/NativeAOT builds (IL3000),
        // so we suppress and fall back to RuntimeEnvironment.GetRuntimeDirectory() when empty.
#pragma warning disable IL3000
        var runtimeDir = Path.GetDirectoryName(typeof(object).Assembly.Location);
#pragma warning restore IL3000
        if (string.IsNullOrEmpty(runtimeDir))
            runtimeDir = RuntimeEnvironment.GetRuntimeDirectory();

        var resolver = new PathAssemblyResolver(
            dllPaths.Concat(Directory.GetFiles(runtimeDir, "*.dll"))
                    .Distinct(StringComparer.OrdinalIgnoreCase)
                    .ToList()
        );

        var commands = new List<GeneratedCommand>();
        var skills = new List<GeneratedSkill>();
        string? assemblyName = null;
        string? assemblyDescription = null;

        using (var mlc = new MetadataLoadContext(resolver))
        {
            foreach (var dllPath in dllPaths)
            {
                Assembly asm;
                try { asm = mlc.LoadFromAssemblyPath(dllPath); }
                catch (Exception ex)
                {
                    // Skip dlls that fail to load (native DLLs, non-managed PE files, etc.)
                    Console.Error.WriteLine($"skip {dllPath}: {ex.Message}");
                    continue;
                }

                assemblyName ??= asm.GetName().Name;
                assemblyDescription ??= GetAssemblyDescription(asm);

                // Load sibling XML doc file if present
                var xmlDocPath = Path.ChangeExtension(dllPath, ".xml");
                var xmlDoc = File.Exists(xmlDocPath) ? LoadXmlDoc(xmlDocPath) : new Dictionary<string, string>();

                // Per-namespace skill aggregating types
                var typesByNamespace = new SortedDictionary<string, List<Type>>(StringComparer.Ordinal);

                foreach (var t in asm.GetTypes())
                {
                    if (!t.IsPublic || !t.IsClass && !t.IsInterface) continue;
                    var ns = t.Namespace ?? "(global)";
                    if (!typesByNamespace.ContainsKey(ns))
                        typesByNamespace[ns] = new List<Type>();
                    typesByNamespace[ns].Add(t);

                    // Emit one command per declared public instance/static method
                    foreach (var m in t.GetMethods(BindingFlags.Public | BindingFlags.Instance | BindingFlags.Static | BindingFlags.DeclaredOnly))
                    {
                        if (m.IsSpecialName) continue; // skip property/event accessors
                        var memberKey = $"M:{t.FullName}.{m.Name}({string.Join(",", m.GetParameters().Select(p => p.ParameterType.FullName ?? p.ParameterType.Name))})";
                        var summary = xmlDoc.GetValueOrDefault(memberKey)
                                      ?? xmlDoc.GetValueOrDefault($"M:{t.FullName}.{m.Name}")
                                      ?? $"{t.Name}.{m.Name}";
                        var cmdName = ToKebab($"{t.Name}-{m.Name}");
                        commands.Add(new GeneratedCommand
                        {
                            Name = cmdName,
                            Lifecycle = "single",
                            Description = summary,
                        });
                    }
                }

                // One skill per namespace summarizing the types
                foreach (var (ns, types) in typesByNamespace)
                {
                    var safeNs = ToKebab(ns);
                    var skillFilename = $"{safeNs}.md";
                    if (skills.Any(s => s.Filename == skillFilename)) continue; // dedupe
                    var body = new System.Text.StringBuilder();
                    body.AppendLine($"---");
                    body.AppendLine($"name: {ToKebab(assemblyName ?? "assembly")}-{safeNs}");
                    body.AppendLine($"description: API reference for namespace {ns} from {Path.GetFileName(dllPath)}");
                    body.AppendLine($"---");
                    body.AppendLine();
                    body.AppendLine($"# {ns}");
                    body.AppendLine();
                    foreach (var t in types.OrderBy(t => t.Name, StringComparer.Ordinal))
                    {
                        body.AppendLine($"- **{t.Name}** ({(t.IsInterface ? "interface" : "class")})");
                        var typeSummary = xmlDoc.GetValueOrDefault($"T:{t.FullName}");
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

    private static List<string> ResolveGlobs(string[] globs)
    {
        var out_ = new List<string>();
        foreach (var g in globs)
        {
            if (File.Exists(g))
            {
                out_.Add(Path.GetFullPath(g));
                continue;
            }
            // Simple glob: split directory + pattern
            var dir = Path.GetDirectoryName(g);
            var pattern = Path.GetFileName(g);
            if (string.IsNullOrEmpty(dir)) dir = ".";
            if (!Directory.Exists(dir)) continue;
            foreach (var hit in Directory.EnumerateFiles(dir, pattern))
                out_.Add(Path.GetFullPath(hit));
        }
        return out_.Distinct(StringComparer.OrdinalIgnoreCase).ToList();
    }

    private static Dictionary<string, string> LoadXmlDoc(string path)
    {
        var dict = new Dictionary<string, string>(StringComparer.Ordinal);
        var doc = XDocument.Load(path);
        foreach (var member in doc.Descendants("member"))
        {
            var name = (string?)member.Attribute("name");
            if (string.IsNullOrEmpty(name)) continue;
            var summary = member.Element("summary")?.Value.Trim().Replace('\n', ' ').Replace('\r', ' ');
            if (!string.IsNullOrEmpty(summary)) dict[name] = summary;
        }
        return dict;
    }

    private static string? GetAssemblyDescription(Assembly asm)
    {
        var attrs = asm.GetCustomAttributesData()
            .FirstOrDefault(a => a.AttributeType.FullName == "System.Reflection.AssemblyDescriptionAttribute");
        if (attrs != null && attrs.ConstructorArguments.Count > 0)
            return attrs.ConstructorArguments[0].Value as string;
        return null;
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
}
