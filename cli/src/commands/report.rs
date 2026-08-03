//! `aware report substrate` — self-contained interactive HTML report of
//! the installed substrate. Single file, inlined CSS + JS, openable in
//! any browser.
//!
//! Output structure:
//!  - Header: substrate-level stats
//!  - Per-vertical stats strip
//!  - Search box (client-side substring filter — auto-expands matching
//!    sections, hides non-matching command lines)
//!  - Per-vertical collapsible sections, with per-agent + per-class
//!    nested `<details>` elements
//!  - Each command is one `<li>` line with name + description

use std::collections::BTreeMap;
use std::time::Instant;

use clap::{Args, Subcommand};

use crate::context::Context;
use crate::error::AwareError;
use crate::manifest::loader::{DiscoveredAgent, discover_agents};

#[derive(Subcommand, Debug)]
pub enum ReportCommand {
    /// Render an interactive HTML report of the installed substrate.
    Substrate(SubstrateArgs),
}

#[derive(Args, Debug)]
pub struct SubstrateArgs {
    /// Output file path. Default: prints to stdout.
    #[arg(long)]
    pub output: Option<String>,
}

pub fn dispatch(cmd: ReportCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        ReportCommand::Substrate(args) => substrate(ctx, &args),
    }
}

fn substrate(ctx: &Context, args: &SubstrateArgs) -> Result<(), AwareError> {
    let started = Instant::now();
    let agents = discover_agents(&ctx.paths)?;
    let html = render_substrate_html(&agents);

    if let Some(target) = &args.output {
        std::fs::write(target, &html)
            .map_err(|e| AwareError::Internal(format!("write {target}: {e}")))?;
        println!(
            "\u{2713} wrote substrate report to {target} ({} agents, {:.1} KB, {:?})",
            agents.len(),
            html.len() as f64 / 1024.0,
            started.elapsed()
        );
    } else {
        print!("{html}");
    }
    Ok(())
}

fn render_substrate_html(agents: &[DiscoveredAgent]) -> String {
    let total_agents = agents.len();
    let total_skills: usize = agents.iter().map(|a| a.manifest.skill_count()).sum();
    let total_commands: usize = agents.iter().map(|a| a.manifest.command_count()).sum();

    // Group agents by vertical (keywords-based, same heuristic as aware diagram)
    let buckets = group_by_vertical(agents);

    let mut html = String::with_capacity(1024 * 1024);
    html.push_str(HEAD);
    html.push_str(&format!(
        r#"<header><div class="title">AWARE substrate report</div><div class="stats">
        <span class="stat"><b>{total_agents}</b> agents</span>
        <span class="stat"><b>{total_skills}</b> skills</span>
        <span class="stat"><b>{total_commands}</b> commands</span>
        <span class="stat">{verticals} verticals</span>
        </div></header>
        <input id="search" type="search" placeholder="Search command names + descriptions..." autocomplete="off" />
        <main>
        "#,
        verticals = buckets.iter().filter(|(_, v)| !v.is_empty()).count(),
    ));

    for (vertical, agents_in) in &buckets {
        if agents_in.is_empty() {
            continue;
        }
        let v_skills: usize = agents_in.iter().map(|a| a.manifest.skill_count()).sum();
        let v_cmds: usize = agents_in.iter().map(|a| a.manifest.command_count()).sum();
        html.push_str(&format!(
            r#"<details class="vertical" open><summary><span class="vname">{vertical}</span>
            <span class="meta">{n} agents · {v_skills} skills · {v_cmds} cmds</span></summary>"#,
            n = agents_in.len()
        ));
        for agent in agents_in {
            render_agent_section(&mut html, agent);
        }
        html.push_str("</details>");
    }

    html.push_str("</main>");
    html.push_str(SCRIPT);
    html.push_str("</body></html>");
    html
}

fn render_agent_section(html: &mut String, agent: &DiscoveredAgent) {
    let display = agent
        .manifest
        .display_name
        .as_deref()
        .unwrap_or(agent.manifest.agent.as_str());
    let vendor = agent.manifest.vendor.as_deref().unwrap_or("");
    let version = agent.manifest.version.as_str();
    let sdk_target = agent.manifest.sdk_target.as_deref().unwrap_or("");
    let cmd_count = agent.manifest.command_count();
    let skill_count = agent.manifest.skill_count();

    // The substrate semver (agent.version) lives behind a small `v` prefix
    // so it can't be confused with the SDK pin. SDK pin (when present) is
    // rendered in its own labelled chip.
    let sdk_chip = if sdk_target.is_empty() {
        String::new()
    } else {
        format!(
            r#" · <span class="sdk">SDK {sdk}</span>"#,
            sdk = html_escape(sdk_target)
        )
    };
    html.push_str(&format!(
        r#"<details class="agent" data-vendor="{vendor}"><summary><span class="aname">{aid}</span>
        <span class="adisplay">{display}</span>
        <span class="meta">v{version} · {vendor}{sdk_chip} · {skill_count} skills · {cmd_count} cmds</span></summary>"#,
        aid = html_escape(&agent.manifest.agent),
        display = html_escape(display),
        vendor = html_escape(vendor),
        version = html_escape(version),
    ));

    // Group commands by class (same heuristic as `aware tree`)
    let mut by_class: BTreeMap<String, Vec<(&String, &str)>> = BTreeMap::new();
    for (name, cmd) in &agent.manifest.commands {
        let group = extract_group(&cmd.description);
        by_class
            .entry(group)
            .or_default()
            .push((name, cmd.description.as_str()));
    }

    for (class_name, cmds) in &by_class {
        html.push_str(&format!(
            r#"<details class="class"><summary><span class="cname">{class}</span>
            <span class="meta">{n}</span></summary><ul>"#,
            class = html_escape(class_name),
            n = cmds.len()
        ));
        for (name, desc) in cmds {
            html.push_str(&format!(
                r#"<li class="cmd"><span class="cmdname">{n}</span><span class="cmddesc">{d}</span></li>"#,
                n = html_escape(name),
                d = html_escape(desc),
            ));
        }
        html.push_str("</ul></details>");
    }

    html.push_str("</details>");
}

fn group_by_vertical(agents: &[DiscoveredAgent]) -> Vec<(&'static str, Vec<&DiscoveredAgent>)> {
    // Preserve the order we want in the report.
    let order = [
        "Engineering",
        "Architecture",
        "Construction",
        "Visualization",
        "Cross-cutting",
        "Operations",
        "Meta + Utility",
    ];
    let mut buckets: BTreeMap<&'static str, Vec<&DiscoveredAgent>> = BTreeMap::new();
    for label in order.iter() {
        buckets.insert(*label, Vec::new());
    }
    for a in agents {
        let kws: Vec<&str> = a.manifest.keywords.iter().map(|s| s.as_str()).collect();
        let bucket = if kws.contains(&"engineering") {
            "Engineering"
        } else if kws.contains(&"architecture") {
            "Architecture"
        } else if kws.contains(&"construction") {
            "Construction"
        } else if kws.contains(&"visualization") {
            "Visualization"
        } else if kws.contains(&"cross-cutting") {
            "Cross-cutting"
        } else if kws.contains(&"operations") {
            "Operations"
        } else {
            "Meta + Utility"
        };
        buckets.entry(bucket).or_default().push(a);
    }
    // Stable order
    order
        .iter()
        .map(|l| (*l, buckets.remove(l).unwrap_or_default()))
        .collect()
}

// TODO: byte-identical to `commands::tree::extract_group`, and both copies are
// now pinned by their own tests. Hoist to one shared module rather than letting
// the two drift.
fn extract_group(description: &str) -> String {
    let trimmed = description.trim_start();
    let head: String = trimmed
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$')
        .collect();
    if head.is_empty() {
        return "Top-level".into();
    }
    if let Some(rest) = trimmed.strip_prefix(&head)
        && rest.starts_with('.')
    {
        return head;
    }
    "Top-level".into()
}

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

const HEAD: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>AWARE substrate report</title>
<style>
* { box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; margin: 0; padding: 0; background: #fafafa; color: #1a1a1a; line-height: 1.5; }
header { padding: 16px 24px; background: #1a1a1a; color: #f5f5f5; position: sticky; top: 0; z-index: 10; box-shadow: 0 2px 4px rgba(0,0,0,0.2); }
.title { font-size: 18px; font-weight: 600; }
.stats { font-size: 13px; opacity: 0.9; margin-top: 4px; }
.stat { margin-right: 16px; }
.stat b { color: #4ade80; }
#search { display: block; width: calc(100% - 48px); margin: 16px 24px; padding: 10px 14px; font-size: 14px; border: 1px solid #ddd; border-radius: 4px; }
#search:focus { outline: none; border-color: #2563eb; box-shadow: 0 0 0 2px rgba(37,99,235,0.2); }
main { padding: 0 24px 48px; }
details { margin: 4px 0; }
summary { cursor: pointer; padding: 4px 0; user-select: none; }
summary:hover { color: #2563eb; }
.vertical > summary { padding: 8px 0; font-size: 15px; font-weight: 600; border-bottom: 1px solid #ddd; margin-top: 16px; }
.vertical > summary .vname { color: #2563eb; }
.agent { margin-left: 16px; padding: 4px 0; }
.agent > summary { font-size: 14px; }
.agent > summary .aname { font-family: monospace; color: #be185d; background: #fce7f3; padding: 1px 6px; border-radius: 3px; font-size: 13px; }
.agent > summary .adisplay { margin-left: 6px; font-weight: 500; }
.class { margin-left: 32px; padding: 2px 0; }
.class > summary { font-size: 13px; }
.class > summary .cname { font-family: monospace; color: #6b7280; }
.meta { color: #6b7280; font-size: 12px; margin-left: 8px; font-weight: normal; }
.sdk  { background: #eef2ff; color: #3730a3; padding: 1px 6px; border-radius: 3px; font-family: monospace; font-size: 11px; }
ul { margin: 4px 0 4px 24px; padding: 0; list-style: none; }
.cmd { font-size: 12px; padding: 2px 0; display: flex; gap: 12px; }
.cmdname { font-family: monospace; color: #1a1a1a; min-width: 240px; flex-shrink: 0; }
.cmddesc { color: #4b5563; }
.cmd.hidden { display: none; }
</style>
</head>
<body>
"#;

const SCRIPT: &str = r#"<script>
(function() {
  const input = document.getElementById('search');
  const allCmds = document.querySelectorAll('.cmd');
  const allDetails = document.querySelectorAll('details');
  let debounce;
  input.addEventListener('input', () => {
    clearTimeout(debounce);
    debounce = setTimeout(() => {
      const term = input.value.trim().toLowerCase();
      if (!term) {
        // Reset: show everything, close non-vertical details
        allCmds.forEach(c => c.classList.remove('hidden'));
        allDetails.forEach(d => {
          d.open = d.classList.contains('vertical');
        });
        return;
      }
      // Hide non-matching commands
      const matchedDetails = new Set();
      allCmds.forEach(c => {
        const text = c.textContent.toLowerCase();
        if (text.includes(term)) {
          c.classList.remove('hidden');
          // Walk up the DOM and mark each <details> as needing-open
          let p = c.parentElement;
          while (p) {
            if (p.tagName === 'DETAILS') matchedDetails.add(p);
            p = p.parentElement;
          }
        } else {
          c.classList.add('hidden');
        }
      });
      // Open all ancestor details of matches, close others
      allDetails.forEach(d => {
        d.open = matchedDetails.has(d);
      });
    }, 120);
  });
})();
</script>
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    /// Build a `DiscoveredAgent` by deserializing a manifest, so the fixture
    /// travels the same serde path the real report does. `extra` is spliced in
    /// as raw YAML so a test can add `keywords:` / `sdk-target:` / `skills:`
    /// without a builder that would drift from the manifest schema.
    fn agent(id: &str, extra: &str, commands: &[(&str, &str)]) -> DiscoveredAgent {
        let mut yaml = format!(
            "agent: {id}\nversion: 1.2.3\ndescription: an agent\nstateful: false\n\
             license: MIT\ntransport:\n  cli:\n    binary: aware-{id}\n{extra}"
        );
        if commands.is_empty() {
            yaml.push_str("commands: {}\n");
        } else {
            yaml.push_str("commands:\n");
            for (name, desc) in commands {
                yaml.push_str(&format!(
                    "  {name}:\n    lifecycle: single\n    description: {desc:?}\n"
                ));
            }
        }
        let manifest: Agent =
            serde_yaml::from_str(&yaml).unwrap_or_else(|e| panic!("fixture yaml: {e}\n{yaml}"));
        DiscoveredAgent {
            manifest,
            root: PathBuf::from("."),
        }
    }

    fn keywords(list: &[&str]) -> String {
        let mut s = String::from("keywords:\n");
        for k in list {
            s.push_str(&format!("  - {k}\n"));
        }
        s
    }

    #[test]
    fn html_escape_replaces_every_markup_significant_character() {
        assert_eq!(
            html_escape(r#"<script>a && b "q" 'p'</script>"#),
            "&lt;script&gt;a &amp;&amp; b &quot;q&quot; &#39;p&#39;&lt;/script&gt;"
        );
    }

    #[test]
    fn html_escape_leaves_ordinary_and_non_ascii_text_alone() {
        assert_eq!(
            html_escape("Ø 45 × 2 — poutre béton"),
            "Ø 45 × 2 — poutre béton"
        );
    }

    #[test]
    fn rendered_agent_section_escapes_every_manifest_field_it_interpolates() {
        // A manifest is data from disk, not a trusted template. Command names
        // come from vendor reflection and versions come from a file someone
        // else wrote, so every one of the six interpolated fields carries
        // markup here — a field that stops being escaped must show up.
        let manifest: Agent = serde_yaml::from_str(
            "agent: \"evil<one>\"\nversion: \"1.0<v>\"\ndescription: an agent\n\
             stateful: false\nlicense: MIT\nsdk-target: \"2026<s>\"\n\
             display-name: 'Bad \"Name\"'\nvendor: \"A & B\"\n\
             transport:\n  cli:\n    binary: aware-evil\n\
             commands:\n  \"wipe<n>\":\n    lifecycle: single\n    \
             description: \"<script>alert(1)</script>\"\n",
        )
        .expect("fixture yaml");
        let a = DiscoveredAgent {
            manifest,
            root: PathBuf::from("."),
        };
        let mut html = String::new();
        render_agent_section(&mut html, &a);

        assert!(
            !html.contains("<script>"),
            "raw <script> reached the page: {html}"
        );
        for (field, escaped) in [
            ("agent id", "evil&lt;one&gt;"),
            ("display name", "Bad &quot;Name&quot;"),
            ("vendor", "A &amp; B"),
            ("version", "v1.0&lt;v&gt;"),
            ("sdk-target", "SDK 2026&lt;s&gt;"),
            ("command name", "wipe&lt;n&gt;"),
            ("description", "&lt;script&gt;alert(1)&lt;/script&gt;"),
        ] {
            assert!(html.contains(escaped), "unescaped {field} in: {html}");
        }
        // Each escaped value must also land in its own slot — a `contains`
        // sweep alone cannot tell the id span from the display-name span.
        assert!(html.contains(r#"<span class="aname">evil&lt;one&gt;</span>"#));
        assert!(html.contains(r#"<span class="adisplay">Bad &quot;Name&quot;</span>"#));
    }

    #[test]
    fn agent_section_omits_the_sdk_chip_when_no_sdk_target_is_declared() {
        let without = {
            let mut h = String::new();
            render_agent_section(&mut h, &agent("plain", "", &[]));
            h
        };
        let with = {
            let mut h = String::new();
            render_agent_section(&mut h, &agent("pinned", "sdk-target: \"2026.1\"\n", &[]));
            h
        };
        assert!(
            !without.contains("class=\"sdk\""),
            "chip rendered with no sdk-target: {without}"
        );
        assert!(
            with.contains(r#"<span class="sdk">SDK 2026.1</span>"#),
            "sdk chip missing or malformed: {with}"
        );
    }

    #[test]
    fn agent_section_falls_back_to_the_agent_id_when_display_name_is_absent() {
        let mut html = String::new();
        render_agent_section(&mut html, &agent("tekla", "", &[]));
        assert!(
            html.contains(r#"<span class="adisplay">tekla</span>"#),
            "id not used as display fallback: {html}"
        );
    }

    #[test]
    fn agent_section_groups_commands_into_one_class_section_per_type_prefix() {
        let a = agent(
            "grouped",
            "",
            &[
                ("load", "Viewer.LoadModel loads a model"),
                ("save", "Viewer.SaveModel saves a model"),
                ("ping", "Checks the connection is alive"),
            ],
        );
        let mut html = String::new();
        render_agent_section(&mut html, &a);

        assert_eq!(
            html.matches(r#"<details class="class">"#).count(),
            2,
            "expected exactly Viewer + Top-level sections: {html}"
        );
        assert!(html.contains(r#"<span class="cname">Viewer</span>"#));
        assert!(html.contains(r#"<span class="cname">Top-level</span>"#));
        // The Viewer section holds both of its commands, the prose one neither.
        let (_, after) = html
            .split_once(r#"<span class="cname">Viewer</span>"#)
            .expect("Viewer section");
        let (viewer, _) = after.split_once("</ul>").expect("Viewer command list");
        assert!(
            viewer.contains(">load<") && viewer.contains(">save<"),
            "{viewer}"
        );
        assert!(
            !viewer.contains(">ping<"),
            "prose command misfiled: {viewer}"
        );
    }

    #[test]
    fn verticals_come_back_in_the_declared_report_order_not_alphabetically() {
        // Buckets are accumulated in a BTreeMap, which sorts its keys —
        // "Architecture" would lead if the map's own order leaked out. The
        // report wants Engineering first.
        let labels: Vec<&str> = group_by_vertical(&[]).into_iter().map(|(l, _)| l).collect();
        assert_eq!(
            labels,
            [
                "Engineering",
                "Architecture",
                "Construction",
                "Visualization",
                "Cross-cutting",
                "Operations",
                "Meta + Utility",
            ]
        );
    }

    #[test]
    fn each_keyword_routes_its_agent_to_the_matching_vertical() {
        for (keyword, expected) in [
            ("engineering", "Engineering"),
            ("architecture", "Architecture"),
            ("construction", "Construction"),
            ("visualization", "Visualization"),
            ("cross-cutting", "Cross-cutting"),
            ("operations", "Operations"),
        ] {
            let agents = vec![agent("a", &keywords(&[keyword]), &[])];
            let placed: Vec<&str> = group_by_vertical(&agents)
                .into_iter()
                .filter(|(_, v)| !v.is_empty())
                .map(|(l, _)| l)
                .collect();
            assert_eq!(placed, [expected], "keyword {keyword} misrouted");
        }
    }

    #[test]
    fn an_agent_with_no_recognised_keyword_lands_in_meta_and_utility() {
        let agents = vec![
            agent("bare", "", &[]),
            agent("odd", &keywords(&["surveying", "gis"]), &[]),
        ];
        let buckets = group_by_vertical(&agents);
        let meta = buckets
            .iter()
            .find(|(l, _)| *l == "Meta + Utility")
            .expect("Meta + Utility bucket");
        assert_eq!(meta.1.len(), 2, "unrecognised agents did not fall through");
    }

    #[test]
    fn the_first_matching_keyword_in_precedence_order_wins() {
        // An agent may carry several vertical keywords. The if-chain's own
        // order — not the manifest's list order — decides where it is filed.
        // Every adjacent pair in that chain is checked, so reordering any two
        // arms is caught; testing keywords one at a time is order-blind.
        let chain = [
            "engineering",
            "architecture",
            "construction",
            "visualization",
            "cross-cutting",
            "operations",
        ];
        let labels = [
            "Engineering",
            "Architecture",
            "Construction",
            "Visualization",
            "Cross-cutting",
            "Operations",
        ];
        for i in 0..chain.len() - 1 {
            // Declared in reverse, so a chain that honoured list order would
            // file the agent under the *lower*-precedence keyword.
            let agents = vec![agent("multi", &keywords(&[chain[i + 1], chain[i]]), &[])];
            let placed: Vec<&str> = group_by_vertical(&agents)
                .into_iter()
                .filter(|(_, v)| !v.is_empty())
                .map(|(l, _)| l)
                .collect();
            assert_eq!(
                placed,
                [labels[i]],
                "`{}` must outrank `{}`",
                chain[i],
                chain[i + 1]
            );
        }
    }

    #[test]
    fn empty_verticals_are_neither_rendered_nor_counted_in_the_header() {
        let agents = vec![
            agent("a", &keywords(&["engineering"]), &[]),
            agent("b", &keywords(&["engineering"]), &[]),
        ];
        let html = render_substrate_html(&agents);
        assert_eq!(
            html.matches(r#"<details class="vertical"#).count(),
            1,
            "an empty vertical was rendered: {html}"
        );
        assert!(
            html.contains(r#"<span class="stat">1 verticals</span>"#),
            "header vertical count disagrees with what was rendered"
        );
        assert!(!html.contains("Meta + Utility"), "empty bucket emitted");
    }

    #[test]
    fn header_totals_sum_skills_and_commands_across_every_agent() {
        // Agent, skill and command totals are all different numbers, so a
        // header that sums the wrong field — or renders the right two totals
        // in each other's slot — cannot pass by coincidence.
        let agents = vec![
            agent(
                "a",
                "skills:\n  - one\n  - two\n",
                &[("v", "does v"), ("w", "does w"), ("x", "does x")],
            ),
            agent(
                "b",
                "skills:\n  - three\n",
                &[("y", "does y"), ("z", "does z")],
            ),
        ];
        let html = render_substrate_html(&agents);
        assert!(
            html.contains("<b>2</b> agents"),
            "agent total wrong: {html}"
        );
        assert!(
            html.contains("<b>3</b> skills"),
            "skill total wrong: {html}"
        );
        assert!(
            html.contains("<b>5</b> commands"),
            "command total wrong: {html}"
        );
    }

    #[test]
    fn per_vertical_meta_counts_only_the_agents_in_that_vertical() {
        let agents = vec![
            agent(
                "eng",
                &format!("{}skills:\n  - s1\n", keywords(&["engineering"])),
                &[("a", "does a")],
            ),
            agent(
                "ops",
                &format!("{}skills:\n  - s2\n  - s3\n", keywords(&["operations"])),
                &[("b", "does b"), ("c", "does c"), ("d", "does d")],
            ),
        ];
        let html = render_substrate_html(&agents);
        // Skill and command counts are deliberately unequal so swapping the two
        // fields, or summing over every agent instead of the bucket, both show up.
        assert!(
            html.contains("1 agents · 1 skills · 1 cmds"),
            "Engineering strip wrong: {html}"
        );
        assert!(
            html.contains("1 agents · 2 skills · 3 cmds"),
            "Operations strip wrong: {html}"
        );
    }

    #[test]
    fn the_report_is_one_self_contained_document() {
        // The command's contract is a single file openable offline: doctype,
        // inlined style + script, closed body. A partial document is the
        // failure mode a caller can't see until they open it in a browser.
        let html = render_substrate_html(&[agent("a", &keywords(&["engineering"]), &[])]);
        assert!(html.starts_with("<!DOCTYPE html>"), "no doctype");
        assert!(html.contains("<style>") && html.contains("</style>"));
        assert!(html.contains("<script>") && html.contains("</script>"));
        assert!(html.contains("<main>") && html.contains("</main>"));
        assert!(html.trim_end().ends_with("</body></html>"), "unclosed body");

        // "Self-contained" means one file: nothing the browser resolves
        // against the network or the filesystem when the page opens. The
        // template emits no URL-bearing attribute at all, so the correct rule
        // is the strict one — any external reference is a violation.
        let refs = external_refs(&html);
        assert!(
            refs.is_empty(),
            "report is not self-contained; it references {refs:?}"
        );
    }

    /// Pins the detector, so the assertion above cannot quietly go back to
    /// passing on a page that fetches. Each group below is a spelling that got
    /// past an earlier hand-rolled version of this check.
    #[test]
    fn every_spelling_of_an_external_reference_is_caught() {
        for spelling in [
            // Quote styles, and none at all.
            r#"<script src="//cdn.example/x.js"></script>"#,
            r#"<script src='//cdn.example/x.js'></script>"#,
            r#"<script src=//cdn.example/x.js></script>"#,
            r#"<link href='//cdn.example/a.css'>"#,
            // Whitespace around `=`, which HTML permits.
            r#"<script src = "//cdn.example/x.js"></script>"#,
            r#"<script src=  //cdn.example/x.js></script>"#,
            "<script src\n=\n'//cdn.example/x.js'></script>",
            // Attribute names are ASCII-case-insensitive.
            r#"<script SRC="//cdn.example/x.js"></script>"#,
            r#"<link HREF='//cdn.example/a.css'>"#,
            // A network-path reference needs no dot in its authority.
            r#"<script src="//localhost/x.js"></script>"#,
            r#"<script src="//assets/bundle.js"></script>"#,
            // Carriers that are not `src`/`href`.
            r#"<video poster="//cdn.example/p.jpg"></video>"#,
            r#"<body background="//cdn.example/b.png">"#,
            r#"<form action="//cdn.example/submit"></form>"#,
            r#"<blockquote cite="//cdn.example/q"></blockquote>"#,
            // `<object data>` fetches; `data-vendor` below does not. Only a
            // real parser can tell those two apart by name.
            r#"<object data="//cdn.example/x.svg"></object>"#,
            // Every `srcset` candidate, not just the first.
            r#"<img srcset="//cdn.example/x.png 2x">"#,
            r#"<img srcset="data:image/gif;base64,R0lGOD 1x, //cdn.example/x.png 2x">"#,
            // A `>` inside a quoted value does not end the tag.
            r#"<script data-note="a > b" src="//cdn/x.js"></script>"#,
            // CSS, in every quoting and spacing style.
            r#"<style>@font-face{src:url(//cdn.example/x.woff)}</style>"#,
            r#"<style>@font-face{src:url("//cdn.example/x.woff")}</style>"#,
            r#"<style>@font-face{src:url('//cdn.example/x.woff')}</style>"#,
            r#"<style>@font-face{src:url( //cdn.example/x.woff )}</style>"#,
            r#"<style>@font-face{src:URL(//fonts/x.woff)}</style>"#,
            r#"<style>@import "//cdn.example/a.css";</style>"#,
            // CSS unescapes identifiers, so this is the `url()` function
            // spelled around a substring scan.
            r#"<style>a{background:u\72 l(//cdn.example/x.png)}</style>"#,
            r#"<style>a{background:\75 rl("//cdn.example/y.png")}</style>"#,
            // The walk descends into every block, and a truncated stylesheet
            // must not end it early and quietly drop what follows.
            r#"<style>@media screen{a{background:url(//cdn.example/m.png)}}</style>"#,
            r#"<style>a{background:image-set(url(//cdn.example/n.png) 1x)}</style>"#,
            "<style>a{background:url(//cdn.example/unclosed.png</style>",
            // CSS in a `style` attribute fetches exactly as a stylesheet does.
            r#"<div style="background-image:url(//cdn.example/x.png)"></div>"#,
            r#"<div style="background:URL('//assets/y.png')"></div>"#,
            // `srcdoc` is a whole nested document, escaped into an attribute.
            r#"<iframe srcdoc="&lt;img src='//cdn.example/x.png'&gt;"></iframe>"#,
            r#"<iframe srcdoc="&lt;link href=&quot;//cdn/a.css&quot;&gt;"></iframe>"#,
            // Numeric character references spell the same nested document.
            r#"<iframe srcdoc="&#60;img src='//cdn/x.png'&#62;"></iframe>"#,
            r#"<iframe srcdoc="&#x3C;img src='//cdn/x.png'&#x3E;"></iframe>"#,
            // A `srcset` whose remote candidate follows a comma-bearing data URI.
            r#"<img srcset="data:image/gif;base64,R0lGOD 1x, //cdn.example/x.png 2x">"#,
            // A second file is not self-contained either, whatever the scheme.
            r#"<link href="https://cdn.example/a.css">"#,
            r#"<link href="theme.css">"#,
        ] {
            assert!(
                !external_refs(spelling).is_empty(),
                "an external reference spelled `{spelling}` slipped past the detector"
            );
        }

        // The other direction. A detector that fires on the report's own
        // inlined script, on manifest text, or on in-page anchors gets switched
        // off rather than fixed.
        for benign in [
            // `//` opens a JS comment and occurs inside ordinary strings.
            "<script>\n// a comment\nlet x = 1;\n</script>",
            r#"<script>const u = "a//b";</script>"#,
            "<script>let src = // why not\nlet x = 1;</script>",
            "<script>let href =\n// note\n2;</script>",
            "<script>let src = //localhost is fine here\n1;</script>",
            // Markup-shaped text inside a script is raw text, not markup.
            r#"<script>const s = '<img src="//cdn/x">';</script>"#,
            // `data-vendor` is interpolated from the manifest and `/` is not
            // escaped, so a vendor opening with `//` reaches the page.
            r#"<details class="agent" data-vendor="//odd"><summary>x</summary></details>"#,
            // A URL in *text* fetches nothing — the old substring probes for
            // `https://` would have failed the report over a description.
            "<p>see https://example.com for details</p>",
            // In-page anchors and inline data are self-contained.
            r##"<a href="#top">top</a>"##,
            r#"<img src="data:image/gif;base64,R0lGOD">"#,
            "<style>a{color:#fff}</style>",
            // The comma belongs to the base64 payload, not to the candidate
            // list — splitting on commas reports `R0lGOD` as a relative path.
            r#"<img srcset="data:image/gif;base64,R0lGOD 1x">"#,
            r#"<img srcset="data:image/gif;base64,AAA 1x, data:image/gif;base64,BBB 2x">"#,
            // A `style` attribute carrying no URL at all.
            r#"<div style="color:#fff;background:#000"></div>"#,
            // `url(` inside a CSS comment or string fetches nothing. Flagging
            // either would fail the report over a note someone left in STYLE.
            "<style>/* see url(//cdn/x.png) for why */a{color:#fff}</style>",
            r#"<style>a::after{content:"url(//cdn/x.png)"}</style>"#,
            r#"<style>a::after{content:'@import "//cdn/a.css"'}</style>"#,
            // A quoted argument to a real url() is still read, so the string
            // handling must locate strings rather than strip them.
            r#"<style>a{background:url("data:image/gif;base64,AAA")}</style>"#,
            // A nested document that is itself self-contained.
            r#"<iframe srcdoc="&lt;p&gt;hello&lt;/p&gt;"></iframe>"#,
        ] {
            assert!(
                external_refs(benign).is_empty(),
                "detector false-positived on `{benign}`: {:?}",
                external_refs(benign)
            );
        }
    }

    /// The check above is a blacklist: it knows the ways a browser can be made
    /// to fetch. That list is open-ended — `srcset` candidate lists, `<object
    /// data>`, a `style` attribute and `<iframe srcdoc>` were each found
    /// missing from it in turn, one review round at a time.
    ///
    /// This bounds the same property from the other side, where the set *is*
    /// finite: the report emits a small, fixed surface, and none of it can
    /// fetch. Anything new fails here until someone adds it deliberately —
    /// which is exactly the moment to ask whether it reaches the network.
    #[test]
    fn the_report_emits_only_its_known_element_and_attribute_surface() {
        /// Every element the renderer emits. None fetches a subresource:
        /// there is no `img`, `iframe`, `link`, `object`, `embed` or `a`.
        const ELEMENTS: [&str; 17] = [
            "b", "body", "details", "div", "head", "header", "html", "input", "li", "main", "meta",
            "script", "span", "style", "summary", "title", "ul",
        ];
        /// Every attribute the renderer emits. None is URL-bearing: no `src`,
        /// `href`, `style` or `srcset`. `data-vendor` carries manifest text.
        const ATTRIBUTES: [&str; 9] = [
            "autocomplete",
            "charset",
            "class",
            "data-vendor",
            "id",
            "lang",
            "open",
            "placeholder",
            "type",
        ];

        // Exercise every render branch, so a tag emitted only for, say, an
        // agent with an sdk-target is still covered.
        let html = render_substrate_html(&[
            agent(
                "eng",
                &format!(
                    "{}sdk-target: \"2026.1\"\ndisplay-name: Eng\nvendor: ACME\nskills:\n  - s\n",
                    keywords(&["engineering"])
                ),
                &[("load", "Viewer.LoadModel loads"), ("ping", "checks it")],
            ),
            agent("bare", "", &[]),
        ]);

        let (elements, attributes) = document_surface(&html);
        let unexpected_elements: Vec<_> = elements
            .iter()
            .filter(|e| !ELEMENTS.contains(&e.as_str()))
            .collect();
        let unexpected_attributes: Vec<_> = attributes
            .iter()
            .filter(|a| !ATTRIBUTES.contains(&a.as_str()))
            .collect();
        assert!(
            unexpected_elements.is_empty(),
            "report emits unlisted elements {unexpected_elements:?}; if one of these fetches a \
             subresource the document is no longer self-contained"
        );
        assert!(
            unexpected_attributes.is_empty(),
            "report emits unlisted attributes {unexpected_attributes:?}; if one of these is \
             URL-bearing the document is no longer self-contained"
        );

        // And the converse. A subset check alone is only as good as the
        // fixture: a branch this render never took could emit anything and the
        // list would still look satisfied. Requiring every listed name to
        // actually appear makes an inadequate fixture a failure here rather
        // than a silent hole, and stops the list going stale as the renderer
        // drops elements.
        let missing_elements: Vec<_> = ELEMENTS
            .iter()
            .filter(|e| !elements.contains(**e))
            .collect();
        let missing_attributes: Vec<_> = ATTRIBUTES
            .iter()
            .filter(|a| !attributes.contains(**a))
            .collect();
        assert!(
            missing_elements.is_empty() && missing_attributes.is_empty(),
            "these are listed but never rendered: {missing_elements:?} {missing_attributes:?} — \
             either the fixture stopped exercising a branch, or the list is stale"
        );
    }

    /// The set of element and attribute names `html` actually contains.
    fn document_surface(html: &str) -> (BTreeSet<String>, BTreeSet<String>) {
        use lol_html::{RewriteStrSettings, element, rewrite_str};
        use std::cell::RefCell;
        use std::rc::Rc;

        let seen = Rc::new(RefCell::new((BTreeSet::new(), BTreeSet::new())));
        let sink = Rc::clone(&seen);
        rewrite_str(
            html,
            RewriteStrSettings::new().append_element_content_handler(element!("*", move |el| {
                let mut s = sink.borrow_mut();
                s.0.insert(el.tag_name().to_ascii_lowercase());
                for attr in el.attributes() {
                    s.1.insert(attr.name().to_ascii_lowercase());
                }
                Ok(())
            })),
        )
        .expect("fixture html tokenizes");
        let surface = seen.borrow();
        (surface.0.clone(), surface.1.clone())
    }

    /// Every reference in `html` that the browser would resolve against the
    /// network or the filesystem.
    ///
    /// Tokenized with `lol_html` rather than pattern-matched. Six rounds of
    /// review on a hand-rolled scanner turned up nine distinct spellings it
    /// missed or misfired on — quote styles, whitespace around `=`, attribute
    /// case, single-label hosts, non-`src` carriers, `srcset` candidate lists,
    /// `<object data>`, a `>` inside a quoted value, and a `<` inside a script
    /// string. Every one of those is a tokenizer's job, and a spec-compliant
    /// tokenizer answers them by construction. What is left here is the only
    /// part that is genuinely this project's judgement: which attributes fetch,
    /// and what counts as self-contained.
    fn external_refs(html: &str) -> Vec<String> {
        use lol_html::{RewriteStrSettings, element, rewrite_str, text};
        use std::cell::RefCell;
        use std::rc::Rc;

        /// Attribute names whose value a browser resolves. Exact names, not
        /// suffixes — the parser hands us the real name, so `data` (which
        /// `<object>` fetches) no longer risks colliding with `data-vendor`.
        const URL_ATTRS: [&str; 8] = [
            "src",
            "href",
            "poster",
            "background",
            "action",
            "formaction",
            "cite",
            "data",
        ];

        /// A value is self-contained only if it stays inside this document: an
        /// in-page anchor, or an inline `data:` URI. Anything else — remote,
        /// protocol-relative, or a relative path to a second file — is not.
        fn is_external(value: &str) -> bool {
            let v = value.trim();
            !v.is_empty() && !v.starts_with('#') && !v.to_ascii_lowercase().starts_with("data:")
        }

        /// Candidate URLs in a `srcset`, parsed the way the HTML spec does: a
        /// candidate's URL is a run of non-whitespace characters, so a comma
        /// inside a `data:` URI belongs to that URL rather than separating
        /// candidates. Splitting on commas instead reports the tail of a base64
        /// payload as a relative reference.
        fn srcset_urls(value: &str) -> Vec<String> {
            let mut urls = Vec::new();
            let mut rest = value;
            loop {
                rest = rest.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == ',');
                if rest.is_empty() {
                    break;
                }
                let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
                let (candidate, tail) = rest.split_at(end);
                let url = candidate.trim_end_matches(',');
                if !url.is_empty() {
                    urls.push(url.to_string());
                }
                if candidate.ends_with(',') {
                    // Empty descriptor; the next token is another URL.
                    rest = tail;
                } else if let Some(comma) = tail.find(',') {
                    rest = &tail[comma + 1..];
                } else {
                    break;
                }
            }
            urls
        }

        /// `url(…)` and `@import` targets in CSS — a stylesheet's text, or a
        /// `style` attribute, which carries the same CSS and fetches the same
        /// way.
        ///
        /// Tokenized with `cssparser`, for the same reason the HTML is
        /// tokenized: a substring scan cannot see CSS token boundaries. It
        /// reports `content: "url(//cdn/x)"` and `/* url(//cdn/x) */`, neither
        /// of which fetches, and misses `u\72l(//cdn/x)`, which does — CSS
        /// unescapes identifiers, so that is the `url()` function by another
        /// spelling.
        fn css_refs(css: &str) -> Vec<String> {
            use cssparser::{ParseError, Parser, ParserInput, Token};

            fn collect(parser: &mut Parser, out: &mut Vec<String>) {
                loop {
                    // Cloned so the borrow of `parser` ends before descending.
                    let token = match parser.next_including_whitespace_and_comments() {
                        Ok(token) => token.clone(),
                        Err(_) => break,
                    };
                    match token {
                        Token::UnquotedUrl(value) if is_external(&value) => {
                            out.push(value.to_string());
                        }
                        Token::AtKeyword(name) if name.eq_ignore_ascii_case("import") => {
                            out.push("@import".to_string());
                        }
                        // `url("…")` tokenizes as a function whose block holds
                        // the quoted target. Any other function may nest one,
                        // so every block is walked.
                        Token::Function(name) => {
                            let is_url = name.eq_ignore_ascii_case("url");
                            let sink = &mut *out;
                            let _ = parser.parse_nested_block(|nested| {
                                if is_url
                                    && let Ok(Token::QuotedString(value)) = nested.next()
                                    && is_external(value)
                                {
                                    sink.push(value.to_string());
                                }
                                collect(nested, sink);
                                Ok::<(), ParseError<'_, ()>>(())
                            });
                        }
                        Token::ParenthesisBlock
                        | Token::SquareBracketBlock
                        | Token::CurlyBracketBlock => {
                            let sink = &mut *out;
                            let _ = parser.parse_nested_block(|nested| {
                                collect(nested, sink);
                                Ok::<(), ParseError<'_, ()>>(())
                            });
                        }
                        _ => {}
                    }
                }
            }

            let mut input = ParserInput::new(css);
            let mut parser = Parser::new(&mut input);
            let mut out = Vec::new();
            collect(&mut parser, &mut out);
            out
        }

        /// `lol_html` hands back attribute values with character references
        /// intact. `srcdoc` carries a whole nested document escaped into an
        /// attribute, so it must be decoded before it can be tokenized.
        fn decode_entities(value: &str) -> String {
            let mut out = String::with_capacity(value.len());
            let mut rest = value;
            while let Some(amp) = rest.find('&') {
                out.push_str(&rest[..amp]);
                rest = &rest[amp..];
                let Some(semi) = rest.find(';') else { break };
                let decoded = match &rest[1..semi] {
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    "amp" => Some('&'),
                    // Numeric references spell the same characters: `&#60;`
                    // and `&#x3C;` are both `<`, and a nested document escaped
                    // that way must decode too.
                    other => other.strip_prefix('#').and_then(|n| {
                        match n.strip_prefix(['x', 'X']) {
                            Some(hex) => u32::from_str_radix(hex, 16).ok(),
                            None => n.parse::<u32>().ok(),
                        }
                        .and_then(char::from_u32)
                    }),
                };
                match decoded {
                    Some(c) => {
                        out.push(c);
                        rest = &rest[semi + 1..];
                    }
                    // Not a reference we know — keep the `&` and move on.
                    None => {
                        out.push('&');
                        rest = &rest[1..];
                    }
                }
            }
            out.push_str(rest);
            out
        }

        let refs = Rc::new(RefCell::new(Vec::<String>::new()));
        let css = Rc::new(RefCell::new(String::new()));
        let (attr_sink, css_sink) = (Rc::clone(&refs), Rc::clone(&css));

        rewrite_str(
            html,
            RewriteStrSettings::new()
                .append_element_content_handler(element!("*", move |el| {
                    for attr in el.attributes() {
                        let name = attr.name().to_ascii_lowercase();
                        let value = attr.value();
                        let mut sink = attr_sink.borrow_mut();
                        match name.as_str() {
                            "srcset" | "imagesrcset" => sink
                                .extend(srcset_urls(&value).into_iter().filter(|u| is_external(u))),
                            "style" => sink.extend(css_refs(&value)),
                            // A nested document, escaped into an attribute. The
                            // browser parses it and fetches what it references.
                            "srcdoc" => sink.extend(external_refs(&decode_entities(&value))),
                            _ if URL_ATTRS.contains(&name.as_str()) && is_external(&value) => {
                                sink.push(value.trim().to_string());
                            }
                            _ => {}
                        }
                    }
                    Ok(())
                }))
                .append_element_content_handler(text!("style", move |chunk| {
                    css_sink.borrow_mut().push_str(chunk.as_str());
                    Ok(())
                })),
        )
        .expect("fixture html tokenizes");

        let collected = css.borrow().clone();
        refs.borrow_mut().extend(css_refs(&collected));
        refs.borrow().clone()
    }
}
