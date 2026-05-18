// SkillWriter — render a CoverageIR into one narrative skill .md per namespace.
//
// One markdown file per namespace, ordered alphabetically; within each file
// types are listed alphabetically by name, methods alphabetically by name,
// and constructors / properties / events / enum-values in declared order.
// Each file is a self-contained skill (YAML frontmatter + body) ready to drop
// under skills/ in the generated agent.
//
// The skill is dual-use: AI reads it to compose apps; humans read it to
// understand what the agent can do. Frontmatter mirrors the Anthropic
// skill format so the same file works under skill-creator without edits.
//
// This writer is pure / stateless / synchronous; the caller is responsible
// for any I/O.

using System.Text;

namespace AwareSidecar.Ingest;

public static class SkillWriter
{
    public static Dictionary<string, string> RenderAll(CoverageIR ir)
    {
        var result = new Dictionary<string, string>();
        var byNamespace = ir.types.GroupBy(t => t.@namespace).OrderBy(g => g.Key);
        foreach (var group in byNamespace)
        {
            var slug = NormaliseToSlug(group.Key);
            result[$"{slug}.md"] = RenderNamespace(group.Key, group.ToList(), ir.host, ir.host_version);
        }
        return result;
    }

    static string RenderNamespace(string ns, List<TypeInfo> types, string host, string hostVersion)
    {
        var sb = new StringBuilder();
        sb.AppendLine("---");
        sb.AppendLine($"name: {host}-{NormaliseToSlug(ns)}");
        sb.AppendLine($"description: This skill encodes the {host} {hostVersion} surface of the {ns} namespace — {types.Count} types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: {string.Join(", ", types.Take(8).Select(t => t.name))}{(types.Count > 8 ? ", and " + (types.Count - 8) + " more types" : "")}.");
        sb.AppendLine("---");
        sb.AppendLine();
        sb.AppendLine($"# {ns}");
        sb.AppendLine();
        sb.AppendLine($"Auto-generated from vendor docs for {host} {hostVersion}. {types.Count} types in this namespace.");
        sb.AppendLine();
        foreach (var t in types.OrderBy(t => t.name))
        {
            sb.AppendLine($"## {t.name} ({t.kind})");
            sb.AppendLine();
            sb.AppendLine(t.summary);
            sb.AppendLine();
            if (!string.IsNullOrEmpty(t.remarks))
            {
                sb.AppendLine("**Remarks:** " + t.remarks.Replace("\n", " "));
                sb.AppendLine();
            }
            sb.AppendLine($"[Vendor docs]({t.doc_url})");
            sb.AppendLine();

            if (t.constructors.Count > 0)
            {
                sb.AppendLine("### Constructors");
                foreach (var c in t.constructors)
                    sb.AppendLine($"- `{c.signature}` — {c.summary}");
                sb.AppendLine();
            }
            if (t.methods.Count > 0)
            {
                sb.AppendLine("### Methods");
                foreach (var m in t.methods.OrderBy(m => m.name))
                {
                    sb.AppendLine($"#### `{m.signature}`");
                    sb.AppendLine();
                    sb.AppendLine(m.summary);
                    if (!string.IsNullOrEmpty(m.remarks))
                    {
                        sb.AppendLine();
                        sb.AppendLine($"**Remarks:** {m.remarks}");
                    }
                    if (m.@params.Count > 0)
                    {
                        sb.AppendLine();
                        sb.AppendLine("**Parameters:**");
                        foreach (var p in m.@params)
                            sb.AppendLine($"- `{p.name}` ({p.type}){(p.doc != null ? " — " + p.doc : "")}");
                    }
                    if (m.returns != null)
                    {
                        sb.AppendLine();
                        sb.AppendLine($"**Returns:** `{m.returns.type}` — {m.returns.doc}");
                    }
                    if (m.events_related.Count > 0)
                    {
                        sb.AppendLine();
                        sb.AppendLine($"**Related events:** {string.Join(", ", m.events_related)}");
                    }
                    sb.AppendLine();
                    sb.AppendLine($"[Docs]({m.doc_url})");
                    sb.AppendLine();
                }
            }
            if (t.properties.Count > 0)
            {
                sb.AppendLine("### Properties");
                foreach (var p in t.properties)
                {
                    var access = p.getter && p.setter ? "get/set" : p.getter ? "get" : "set";
                    sb.AppendLine($"- `{p.name}` ({p.type}, {access}) — {p.summary}");
                }
                sb.AppendLine();
            }
            if (t.events.Count > 0)
            {
                sb.AppendLine("### Events");
                foreach (var e in t.events)
                {
                    sb.AppendLine($"#### `{e.name}` (`{e.@delegate}`)");
                    sb.AppendLine();
                    sb.AppendLine($"**Signature:** `{e.signature}`");
                    sb.AppendLine();
                    sb.AppendLine(e.summary);
                    if (!string.IsNullOrEmpty(e.fires_when))
                    {
                        sb.AppendLine();
                        sb.AppendLine($"**Fires when:** {e.fires_when}");
                    }
                    sb.AppendLine();
                    sb.AppendLine($"[Docs]({e.doc_url})");
                    sb.AppendLine();
                }
            }
            if (t.enum_values.Count > 0)
            {
                sb.AppendLine("### Values");
                foreach (var v in t.enum_values)
                    sb.AppendLine($"- `{v.name}` = `{v.value}`{(v.summary != null ? " — " + v.summary : "")}");
                sb.AppendLine();
            }
        }
        return sb.ToString();
    }

    static string NormaliseToSlug(string s) => s.Replace('.', '-').ToLowerInvariant();
}
