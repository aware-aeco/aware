// ManifestWriter — render a CoverageIR into a manifest.yaml document.
//
// The manifest is the compact verb catalog the AWARE runtime reads on
// `aware agent install`. One verb per method, one-line description each,
// plus a header block (agent / version / vendor / provenance / transport)
// and a tail referencing the per-namespace skill files generated alongside.
//
// This writer is pure / stateless / synchronous; the caller is responsible
// for any I/O. The output is plain text (a YAML 1.2 document); we do not
// round-trip through a YAML serializer because the layout — comments,
// blank lines, key alignment — is part of the spec.
//
// Inputs assumed valid: A2's IRReader returns a CoverageIR with non-null
// collections, so we iterate without null guards.

using System.Text;
using System.Text.RegularExpressions;

namespace AwareSidecar.Ingest;

public static class ManifestWriter
{
    public static string Render(CoverageIR ir, string agentId, string vendor, string vertical)
    {
        var sb = new StringBuilder();
        sb.AppendLine($"agent:        {agentId}");
        sb.AppendLine($"version:      0.30.0");
        sb.AppendLine($"sdk-target:   {ir.host_version}");
        sb.AppendLine($"display-name: {agentId} (raw — {ir.types.Count} types, full coverage)");
        sb.AppendLine();
        sb.AppendLine("description: |");
        sb.AppendLine($"  Auto-generated from vendor docs ({ir.source.kind}: {string.Join(", ", ir.source.urls)})");
        sb.AppendLine($"  on {ir.source.extracted_at:yyyy-MM-dd}.");
        sb.AppendLine();
        sb.AppendLine($"  Coverage: {ir.metadata.type_count} types, {ir.metadata.method_count} methods,");
        sb.AppendLine($"  {ir.metadata.event_count} events, {ir.metadata.property_count} properties.");
        sb.AppendLine();
        sb.AppendLine($"  Behavioral notes from vendor remarks/summary preserved per member.");
        sb.AppendLine($"  See skills/ for narrative, catalog/ for structured per-type data.");
        sb.AppendLine();
        sb.AppendLine("stateful: false");
        sb.AppendLine();
        sb.AppendLine($"vendor:   {vendor}");
        sb.AppendLine("license:  Apache-2.0");
        sb.AppendLine($"homepage: https://github.com/aware-aeco/aware/tree/main/20-agents/aeco/{vertical}/{agentId}");
        sb.AppendLine($"keywords: [aeco, {vertical}, {ir.host}, {vendor}, generated, raw, v0.30, full-coverage]");
        sb.AppendLine();
        sb.AppendLine("provenance:");
        sb.AppendLine("  generated-by:     aware-agent-builder");
        sb.AppendLine("  generator-version: 0.30.0");
        sb.AppendLine("  source:");
        sb.AppendLine($"    type: {ir.source.kind}");
        sb.AppendLine($"    urls: {YamlList(ir.source.urls)}");
        sb.AppendLine($"  generated-at: {ir.source.extracted_at:o}");
        sb.AppendLine();
        sb.AppendLine("transport:");
        sb.AppendLine("  cli:");
        sb.AppendLine($"    binary: aware-{ir.host}");
        sb.AppendLine();
        sb.AppendLine("commands:");
        foreach (var t in ir.types.OrderBy(t => t.@namespace).ThenBy(t => t.name))
        {
            foreach (var m in t.methods.OrderBy(m => m.name))
            {
                var verbId = $"{NormaliseToVerb(t.name)}-{NormaliseToVerb(m.name)}";
                var oneLineDesc = (m.summary ?? "").Replace("\n", " ").Replace("\r", " ").Trim();
                if (oneLineDesc.Length > 200) oneLineDesc = oneLineDesc.Substring(0, 197) + "...";
                sb.AppendLine($"  {verbId}:");
                sb.AppendLine($"    lifecycle: single");
                sb.AppendLine($"    description: {QuoteYamlScalar(oneLineDesc)}");
            }
        }
        sb.AppendLine();
        sb.AppendLine("skills:");
        var namespaces = ir.types.Select(t => t.@namespace).Distinct().OrderBy(n => n);
        foreach (var ns in namespaces)
        {
            // Ruby's root namespace is empty; the SkillWriter emits its skill as `_root.md`.
            // Keep the two in sync so `aware agent install` can resolve every entry.
            var slug = string.IsNullOrEmpty(ns) ? "_root" : NormaliseToSlug(ns);
            sb.AppendLine($"  - {slug}.md");
        }
        return sb.ToString();
    }

    /// <summary>
    /// Normalise a type / method name into a verb-id token. Handles both .NET CamelCase
    /// (`GetActiveModel` → `get-active-model`) and Ruby snake_case (`get_active_model` →
    /// `get-active-model`) so verb IDs collide cleanly across the two language idioms.
    /// Trailing Ruby suffixes `?` `!` `=` are mapped to readable tokens:
    ///   `?` → `-q`     predicate methods
    ///   `!` → `-bang`  mutator methods
    ///   `=` → `-set`   setter methods
    /// </summary>
    static string NormaliseToVerb(string s)
    {
        var t = s;
        // Ruby suffix tokens — replace BEFORE the CamelCase split so we don't smash them.
        if (t.EndsWith("?", StringComparison.Ordinal)) t = t.Substring(0, t.Length - 1) + "_q";
        else if (t.EndsWith("!", StringComparison.Ordinal)) t = t.Substring(0, t.Length - 1) + "_bang";
        else if (t.EndsWith("=", StringComparison.Ordinal)) t = t.Substring(0, t.Length - 1) + "_set";
        // CamelCase split: `GetActive` → `Get-Active`.
        t = Regex.Replace(t, "([a-z])([A-Z])", "$1-$2");
        // Snake-case underscores become dashes.
        t = t.Replace('_', '-');
        // Ruby-namespace separator (defensive — verb names are bare method names so this
        // rarely fires, but if a vendor emits `Foo::Bar` we get `foo-bar`).
        t = t.Replace("::", "-", StringComparison.Ordinal);
        return t.ToLowerInvariant();
    }

    /// <summary>
    /// Convert a vendor namespace into a filename-safe slug. .NET vendors use `.` as the
    /// namespace separator; Ruby vendors use `::`. Both collapse into dash-delimited
    /// lowercase so the skills/ filenames work cross-platform.
    /// </summary>
    static string NormaliseToSlug(string s) =>
        s.Replace("::", "-").Replace('.', '-').ToLowerInvariant();

    static string YamlList(IEnumerable<string> items) =>
        $"[{string.Join(", ", items.Select(s => $"\"{s}\""))}]";

    static string QuoteYamlScalar(string s)
    {
        if (string.IsNullOrEmpty(s)) return "\"\"";
        if (s.Contains(":") || s.Contains("#") || s.StartsWith("-") || s.StartsWith("&") || s.StartsWith("*") || s.StartsWith("!") || s.StartsWith("|") || s.StartsWith(">"))
            return $"\"{s.Replace("\"", "\\\"")}\"";
        return s;
    }
}
