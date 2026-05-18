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
        buckets.get_mut(bucket).unwrap().push(a);
    }
    // Stable order
    order
        .iter()
        .map(|l| (*l, buckets.remove(l).unwrap_or_default()))
        .collect()
}

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
