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
    // IL2026 / IL2065 / IL2075 / IL3000: GetTypes() and GetMethods() are called on assemblies
    // loaded via MetadataLoadContext, which only loads metadata and never executes managed code.
    // Assembly.Location is guarded by a null/empty check with RuntimeEnvironment fallback.
    // The trimmer / AOT warnings are not applicable — no live types are reflected.
    [UnconditionalSuppressMessage("AOT", "IL2026", Justification = "MLC loads metadata only — no live code is reflected")]
    [UnconditionalSuppressMessage("AOT", "IL2065", Justification = "MLC loads metadata only — no live code is reflected")]
    [UnconditionalSuppressMessage("AOT", "IL2075", Justification = "MLC loads metadata only — no live code is reflected")]
    [UnconditionalSuppressMessage("AOT", "IL3000", Justification = "Assembly.Location guarded by empty-check + RuntimeEnvironment fallback")]
    public static GeneratedAgent Reflect(string[] globs, string? agentIdOverride)
    {
        var dllPaths = ResolveGlobs(globs);
        if (dllPaths.Count == 0)
        {
            throw new InvalidOperationException($"no dlls matched globs: {string.Join(", ", globs)}");
        }

        // Locate the .NET runtime/reference DLLs for MetadataLoadContext.
        // In a NativeAOT binary Assembly.Location is empty (IL3000 suppressed on this method).
        // RuntimeEnvironment.GetRuntimeDirectory() is also not reliable in NativeAOT — it may
        // return the binary's directory or a system path with no managed DLLs.
        // Strategy: try Assembly.Location → RuntimeEnvironment.GetRuntimeDirectory() → dotnet
        // shared runtime → dotnet reference packs. Use the first that contains *.dll files.
        var runtimeDir = FindReferenceAssemblyDir();

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

    /// <summary>
    /// Finds a directory containing .NET reference/runtime DLLs suitable for
    /// MetadataLoadContext. Works under both regular .NET and NativeAOT.
    /// </summary>
    // IL3000 on typeof(object).Assembly.Location: we explicitly guard the empty-string case.
    [UnconditionalSuppressMessage("AOT", "IL3000", Justification = "Assembly.Location guarded by empty-check; NativeAOT falls through to dotnet path probing")]
    private static string FindReferenceAssemblyDir()
    {
        // 1. Assembly.Location — works in regular .NET, empty in NativeAOT.
        var loc = typeof(object).Assembly.Location;
        if (!string.IsNullOrEmpty(loc))
        {
            var dir = Path.GetDirectoryName(loc);
            if (!string.IsNullOrEmpty(dir) && HasDotNetDlls(dir))
                return dir;
        }

        // 2. RuntimeEnvironment.GetRuntimeDirectory() — may work in NativeAOT on some hosts.
        var rtDir = RuntimeEnvironment.GetRuntimeDirectory();
        if (!string.IsNullOrEmpty(rtDir) && HasDotNetDlls(rtDir))
            return rtDir;

        // 3. Dotnet shared runtime directory (e.g. C:\Program Files\dotnet\shared\Microsoft.NETCore.App\X.Y.Z\).
        //    Preferred over reference packs: shared contains implementation DLLs that MLC
        //    can actually resolve cross-assembly dependencies against.
        var sharedDir = FindDotnetSharedDir();
        if (sharedDir != null)
            return sharedDir;

        // 4. Dotnet reference pack (C:\Program Files\dotnet\packs\Microsoft.NETCore.App.Ref\X.Y.Z\ref\netX.Y\).
        var refPackDir = FindDotnetRefPackDir();
        if (refPackDir != null)
            return refPackDir;

        // 5. Fall back to AppContext.BaseDirectory (the binary's directory in NativeAOT).
        return AppContext.BaseDirectory;
    }

    private static bool HasDotNetDlls(string dir) =>
        Directory.Exists(dir) && Directory.EnumerateFiles(dir, "System.Runtime.dll").Any();

    private static string? FindDotnetSharedDir()
    {
        // Try to discover via DOTNET_ROOT env var or the standard program files path.
        var dotnetRoot = Environment.GetEnvironmentVariable("DOTNET_ROOT")
                         ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles), "dotnet");
        var sharedBase = Path.Combine(dotnetRoot, "shared", "Microsoft.NETCore.App");
        if (!Directory.Exists(sharedBase)) return null;
        // Pick the highest version that exists.
        var best = Directory.EnumerateDirectories(sharedBase)
            .Where(d => HasDotNetDlls(d))
            .OrderByDescending(d => d, StringComparer.Ordinal)
            .FirstOrDefault();
        return best;
    }

    private static string? FindDotnetRefPackDir()
    {
        var dotnetRoot = Environment.GetEnvironmentVariable("DOTNET_ROOT")
                         ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ProgramFiles), "dotnet");
        var packsBase = Path.Combine(dotnetRoot, "packs", "Microsoft.NETCore.App.Ref");
        if (!Directory.Exists(packsBase)) return null;
        // Each subdir is a version; each has ref/netX.Y/ inside.
        foreach (var versionDir in Directory.EnumerateDirectories(packsBase)
                     .OrderByDescending(d => d, StringComparer.Ordinal))
        {
            var refBase = Path.Combine(versionDir, "ref");
            if (!Directory.Exists(refBase)) continue;
            var tfm = Directory.EnumerateDirectories(refBase)
                .OrderByDescending(d => d, StringComparer.Ordinal)
                .FirstOrDefault();
            if (tfm != null && HasDotNetDlls(tfm))
                return tfm;
        }
        return null;
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
