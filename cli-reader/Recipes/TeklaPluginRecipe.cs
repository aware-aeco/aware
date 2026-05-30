using System.Text;

namespace AwareReader.Recipes;

/// <summary>
/// Recognises the Tekla Structures model plug-in idiom and emits a single Component-insert
/// command per plug-in instead of the useless per-method commands (#180).
///
/// A model plug-in is a public class whose base chain reaches
/// <c>Tekla.Structures.Plugins.PluginBase</c> and that carries <c>[Plugin("Name")]</c>. Its
/// parameter contract is a StructuresData class — the type of its public single-parameter
/// constructor (the classic <c>MyPlugin(MyData data) : PluginBase</c> pattern), which may live
/// in another assembly (resolved via the cross-assembly <see cref="ReflectedSet.TypeIndex"/>).
/// The data class's public <c>[StructuresField("attr")]</c> fields (double/int/string only, per
/// Tekla) become the command's typed inputs; the picked points become an <c>input-points</c> input.
///
/// The emitted command documents the in-model action the aware-tekla bridge already runs via its
/// <c>exec</c> verb: <c>new ComponentInput()</c> (+ points) → <c>Component { Name = "&lt;plugin&gt;" }</c>
/// → <c>SetAttribute("&lt;field&gt;", value)</c> per field → <c>Insert()</c> / <c>CommitComponentInput</c>.
/// No bridge change.
/// </summary>
public sealed class TeklaPluginRecipe : IRecipe
{
    // Default config = the real Tekla.Structures.Plugins full names. Kept as constants (rather
    // than hardcoded inline) so a future variant/test can target alternative names.
    private const string PluginBaseFullName = "Tekla.Structures.Plugins.PluginBase";
    private const string PluginAttr = "Tekla.Structures.Plugins.PluginAttribute";
    private const string StructuresFieldAttr = "Tekla.Structures.Plugins.StructuresFieldAttribute";

    public string Name => "tekla-plugin";

    public bool Matches(ReflectedSet set) => FindPlugins(set).Count > 0;

    public GeneratedAgent Apply(ReflectedSet set, GeneratedAgent baseline)
    {
        var plugins = FindPlugins(set);
        if (plugins.Count == 0) return baseline;

        var commands = baseline.Commands.ToList();
        foreach (var plugin in plugins)
        {
            // Drop the plug-in type's per-method commands (e.g. demo-plugin-run, demo-plugin-define-input).
            var perMethod = plugin.Methods
                .Where(m => !m.IsOperator)
                .Select(m => Kebab($"{plugin.Name}-{m.Name}"))
                .ToHashSet(StringComparer.Ordinal);
            commands.RemoveAll(c => perMethod.Contains(c.Name));

            var insert = BuildInsertCommand(plugin, set.TypeIndex);
            commands.RemoveAll(c => c.Name == insert.Name); // idempotent on re-runs / name clashes
            commands.Add(insert);
        }

        commands.Sort((a, b) => string.CompareOrdinal(a.Name, b.Name));

        return new GeneratedAgent
        {
            Id = baseline.Id,
            Version = baseline.Version,
            Description = baseline.Description,
            License = baseline.License,
            Stateful = baseline.Stateful,
            Commands = commands.ToArray(),
            Skills = baseline.Skills,
        };
    }

    // ── Matching ──────────────────────────────────────────────────────────────

    private static List<TypeRecord> FindPlugins(ReflectedSet set)
    {
        var plugins = new List<TypeRecord>();
        foreach (var asm in set.Assemblies)
            foreach (var t in asm.Types)
                if (IsModelPlugin(t, set.TypeIndex))
                    plugins.Add(t);
        return plugins;
    }

    private static bool IsModelPlugin(TypeRecord t, IReadOnlyDictionary<string, TypeRecord> index)
    {
        if (t.Kind != TypeKind.Class) return false;
        if (!HasAttribute(t.Attributes, PluginAttr)) return false;
        return BaseChainReaches(t, PluginBaseFullName, index);
    }

    /// <summary>
    /// Walks the base-type chain by full name. The direct base name is captured even when its
    /// definition lives in another (unreflected) assembly, so a direct <c>: PluginBase</c> matches
    /// without the Tekla assembly present; intermediate in-set bases are followed via the index and
    /// the walk stops cleanly at the first external (unresolved) base.
    /// </summary>
    private static bool BaseChainReaches(
        TypeRecord t, string target, IReadOnlyDictionary<string, TypeRecord> index)
    {
        var current = t.BaseTypeName;
        var visited = new HashSet<string>(StringComparer.Ordinal);
        while (!string.IsNullOrEmpty(current))
        {
            if (current == target) return true;
            if (!visited.Add(current)) return false;          // cycle guard
            if (!index.TryGetValue(current, out var bt)) return false; // external base — can't walk further
            current = bt.BaseTypeName;
        }
        return false;
    }

    // ── Command synthesis ───────────────────────────────────────────────────────

    private static GeneratedCommand BuildInsertCommand(
        TypeRecord plugin, IReadOnlyDictionary<string, TypeRecord> index)
    {
        var pluginName = PluginName(plugin) ?? plugin.Name;
        var dataType = FindDataContract(plugin, index);

        var inputs = new List<GeneratedInput>
        {
            new()
            {
                Name = "input-points",
                Type = "points",
                Optional = false,
                Default = null,
            },
        };

        if (dataType is not null)
        {
            foreach (var field in dataType.Fields)
            {
                var attrName = StructuresFieldName(field);
                if (attrName is null) continue; // not a [StructuresField]
                var awareType = MapFieldType(field.Type);
                if (awareType is null) continue; // not a double/int/string field
                inputs.Add(new GeneratedInput
                {
                    Name = Kebab(attrName),
                    Type = awareType,
                    Optional = true,
                    Default = null,
                });
            }
        }

        var desc =
            $"Insert the \"{pluginName}\" Tekla model plug-in as a component: build a ComponentInput "
            + "from the input points, set each StructuresField attribute, then Insert(). "
            + "(Recipe: Tekla model plug-in.)";

        return new GeneratedCommand
        {
            Name = Kebab($"insert-{pluginName}"),
            Lifecycle = "single",
            Description = desc,
            Mode = "write",
            Inputs = inputs.ToArray(),
        };
    }

    /// <summary>
    /// The StructuresData contract = the type of the plug-in's public single-parameter constructor,
    /// resolved via the cross-assembly index, whose type carries <c>[StructuresField]</c> fields.
    /// Falls back to the plug-in class itself when it carries the fields directly (older single-class
    /// pattern). When several single-param ctors qualify, the first (signature-sorted) wins.
    /// </summary>
    private static TypeRecord? FindDataContract(
        TypeRecord plugin, IReadOnlyDictionary<string, TypeRecord> index)
    {
        foreach (var ctor in plugin.Constructors)
        {
            if (ctor.Parameters.Count != 1) continue;
            var paramType = ctor.Parameters[0].Type;
            if (index.TryGetValue(paramType, out var dt) && HasStructuresField(dt))
                return dt;
        }
        return HasStructuresField(plugin) ? plugin : null;
    }

    // ── Attribute helpers ─────────────────────────────────────────────────────

    private static bool HasAttribute(IReadOnlyList<AttributeRecord> attrs, string fullName) =>
        attrs.Any(a => a.TypeName == fullName);

    private static bool HasStructuresField(TypeRecord t) =>
        t.Fields.Any(f => f.Attributes.Any(a => a.TypeName == StructuresFieldAttr));

    private static string? PluginName(TypeRecord t) =>
        t.Attributes.FirstOrDefault(a => a.TypeName == PluginAttr)?.FixedArguments.FirstOrDefault()?.Value;

    private static string? StructuresFieldName(FieldRecord f) =>
        f.Attributes.FirstOrDefault(a => a.TypeName == StructuresFieldAttr)?.FixedArguments.FirstOrDefault()?.Value;

    /// <summary>
    /// Maps a [StructuresField] field type to an AWARE input type. Accepts both the compiled
    /// reader's FQN encoding (System.Double) and any keyword form, since the two readers differ.
    /// Tekla restricts these fields to double / int / string.
    /// </summary>
    private static string? MapFieldType(string fieldType) => fieldType switch
    {
        "System.Double" or "double" => "number",
        "System.Int32" or "int" => "integer",
        "System.String" or "string" => "string",
        _ => null,
    };

    // ── Naming (kebab-case; mirrors AgentSynthesizer) ─────────────────────────────

    private static string Kebab(string s)
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
                if (sb.Length > 0 && sb[^1] != '-') sb.Append('-');
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
