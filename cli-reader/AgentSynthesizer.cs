using System.Text;

namespace AwareReader;

/// <summary>Knobs for <see cref="AgentSynthesizer.Synthesize"/>.</summary>
public sealed class SynthesisOptions
{
    /// <summary>Explicit agent id; overrides the primary-assembly heuristic when non-empty.</summary>
    public string? AgentIdOverride { get; init; }

    /// <summary>
    /// XML-doc-id → summary, merged across every sibling XML doc of the reflected set
    /// (T:Type, M:Type.Method(args)). Used to describe commands and types. Null = none.
    /// </summary>
    public IReadOnlyDictionary<string, string>? DocSummaries { get; init; }
}

/// <summary>
/// Projects a reflected set of assemblies (#180) into one coherent <see cref="GeneratedAgent"/>.
/// The default projection mirrors the historical <c>DllReflector</c> behaviour — one command per
/// public, non-operator method, one skill per namespace — but spans the WHOLE set rather than the
/// first assembly, so a product directory of DLLs yields a single agent (id/description chosen from
/// the assembly contributing the most public types). Recipes (a later step) post-process this.
/// </summary>
public static class AgentSynthesizer
{
    public static GeneratedAgent Synthesize(ReflectedSet set, SynthesisOptions? options = null)
    {
        options ??= new SynthesisOptions();
        var stemIndex = BuildMethodStemIndex(options.DocSummaries);

        // Primary assembly = the one contributing the most public types (no first-wins).
        AssemblyRecord? primary = null;
        foreach (var a in set.Assemblies)
            if (primary is null || a.Types.Count > primary.Types.Count) primary = a;

        var id = options.AgentIdOverride is { Length: > 0 } overrideId
            ? overrideId
            : primary is not null ? ToKebab(primary.Name) : "reflected-agent";
        if (string.IsNullOrEmpty(id)) id = "reflected-agent";

        // First-wins dedupe across the set; ordinal-sorted output for determinism.
        var commands = new SortedDictionary<string, GeneratedCommand>(StringComparer.Ordinal);
        var typesByNamespace = new SortedDictionary<string, List<TypeRecord>>(StringComparer.Ordinal);

        foreach (var asm in set.Assemblies)
        {
            foreach (var type in asm.Types)
            {
                var ns = string.IsNullOrEmpty(type.Namespace) ? "(global)" : type.Namespace;
                if (!typesByNamespace.TryGetValue(ns, out var list))
                    typesByNamespace[ns] = list = new List<TypeRecord>();
                list.Add(type);

                foreach (var method in type.Methods)
                {
                    if (method.IsOperator) continue; // operators aren't end-user commands
                    var cmdName = ToKebab($"{type.Name}-{method.Name}");
                    if (string.IsNullOrEmpty(cmdName) || commands.ContainsKey(cmdName)) continue;
                    commands[cmdName] = new GeneratedCommand
                    {
                        Name = cmdName,
                        Lifecycle = "single",
                        Description = ResolveSummary(options.DocSummaries, stemIndex, method.DocId)
                                      ?? $"{type.Name}.{method.Name}",
                    };
                }
            }
        }

        var skills = new List<GeneratedSkill>();
        var seenSkillFile = new HashSet<string>(StringComparer.Ordinal);
        foreach (var (ns, typeList) in typesByNamespace)
        {
            var safeNs = ToKebab(ns);
            var filename = $"{safeNs}.md";
            if (!seenSkillFile.Add(filename)) continue;
            skills.Add(BuildNamespaceSkill(id, ns, safeNs, filename, typeList, options.DocSummaries));
        }

        return new GeneratedAgent
        {
            Id = id,
            // The reflected SDK/package version (surfaced as the agent's sdk-target downstream).
            Version = primary?.Version ?? "0.1.0",
            Description = primary?.Description
                          ?? $"Generated from {set.Assemblies.Count} assembly file(s) via reflection.",
            License = "UNKNOWN",
            Stateful = false,
            Commands = commands.Values.ToArray(),
            Skills = skills.OrderBy(s => s.Filename, StringComparer.Ordinal).ToArray(),
        };
    }

    private static GeneratedSkill BuildNamespaceSkill(
        string agentId,
        string ns,
        string safeNs,
        string filename,
        List<TypeRecord> typeList,
        IReadOnlyDictionary<string, string>? docs)
    {
        var body = new StringBuilder();
        body.AppendLine("---");
        body.AppendLine($"name: {agentId}-{safeNs}");
        body.AppendLine($"description: API reference for namespace {ns}");
        body.AppendLine("---");
        body.AppendLine();
        body.AppendLine($"# {ns}");
        body.AppendLine();

        var seenType = new HashSet<string>(StringComparer.Ordinal);
        foreach (var type in typeList.OrderBy(t => t.Name, StringComparer.Ordinal))
        {
            if (!seenType.Add(type.Name)) continue;
            body.AppendLine($"- **{type.Name}**");
            if (docs is not null && docs.TryGetValue($"T:{type.FullName}", out var summary)
                && !string.IsNullOrWhiteSpace(summary))
            {
                body.AppendLine($"  - {summary}");
            }
        }
        return new GeneratedSkill { Filename = filename, Body = body.ToString() };
    }

    /// <summary>
    /// Pre-indexes method doc summaries by their <c>M:Type.Method</c> stem (params stripped),
    /// first-wins, so overload/encoding mismatches resolve in O(1) instead of scanning all docs
    /// per method — important for large SDKs (thousands of methods).
    /// </summary>
    private static Dictionary<string, string> BuildMethodStemIndex(IReadOnlyDictionary<string, string>? docs)
    {
        var index = new Dictionary<string, string>(StringComparer.Ordinal);
        if (docs is null) return index;
        foreach (var kv in docs)
        {
            if (!kv.Key.StartsWith("M:", StringComparison.Ordinal)) continue;
            var paren = kv.Key.IndexOf('(');
            var stem = paren >= 0 ? kv.Key[..paren] : kv.Key;
            index.TryAdd(stem, kv.Value);
        }
        return index;
    }

    private static string? ResolveSummary(
        IReadOnlyDictionary<string, string>? docs, Dictionary<string, string> stemIndex, string docId)
    {
        if (docs is not null && docs.TryGetValue(docId, out var exact)) return exact;
        var paren = docId.IndexOf('(');
        var stem = paren >= 0 ? docId[..paren] : docId;
        return stemIndex.TryGetValue(stem, out var byStem) ? byStem : null;
    }

    /// <summary>
    /// kebab-cases a CLR identifier (camelCase boundaries, separators collapsed). Ported verbatim
    /// from the historical DllReflector so generated command/skill names are unchanged.
    /// </summary>
    private static string ToKebab(string s)
    {
        var sb = new StringBuilder(s.Length + 4);
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
