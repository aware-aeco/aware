//! `aware diagram regenerate` — auto-build the master Mermaid diagram from
//! installed agents.
//!
//! Two modes:
//!  - **Standalone** (default): emit a complete Mermaid `.mmd` snapshot of
//!    the current substrate state to stdout. Useful for end users who want
//!    to visualize what's installed locally.
//!  - **In-place** (`--in-place <file>`): replace the content between
//!    `%% AWARE-AUTOGEN-AGENTS-BEGIN` and `%% AWARE-AUTOGEN-AGENTS-END`
//!    markers in the target file. Used by AWARE maintainers to keep
//!    `40-diagrams/aware-master.mmd` in sync with the registry as
//!    new agents land.
//!
//! Idempotent: same set of installed agents → byte-identical output.

use clap::{Args, Subcommand};

use crate::context::Context;
use crate::error::AwareError;
use crate::manifest::loader::{DiscoveredAgent, discover_agents};

#[derive(Subcommand, Debug)]
pub enum DiagramCommand {
    /// Regenerate the master diagram from installed agents.
    Regenerate(RegenerateArgs),
}

#[derive(Args, Debug)]
pub struct RegenerateArgs {
    /// Replace the AGENTS block (between AWARE-AUTOGEN markers) in this file.
    /// Without this flag, prints the standalone diagram to stdout.
    #[arg(long = "in-place")]
    pub in_place: Option<String>,
    /// Render only one AECO vertical's diagram (engineering / architecture /
    /// construction / visualization / cross-cutting / operations / meta).
    /// Output is a focused per-vertical mermaid graph with the agents in
    /// that bucket as nodes + a per-agent skill+command count.
    #[arg(long)]
    pub vertical: Option<String>,
}

pub fn dispatch(cmd: DiagramCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        DiagramCommand::Regenerate(args) => regenerate(ctx, &args),
    }
}

fn regenerate(ctx: &Context, args: &RegenerateArgs) -> Result<(), AwareError> {
    let agents = discover_agents(&ctx.paths)?;

    // --vertical takes precedence over --in-place — they're separate modes.
    if let Some(v) = &args.vertical {
        let body = render_vertical(&agents, v)?;
        println!("{body}");
        return Ok(());
    }

    let agents_block = render_agents_block(&agents);
    if let Some(target) = &args.in_place {
        let path = std::path::PathBuf::from(target);
        let original = std::fs::read_to_string(&path)
            .map_err(|e| AwareError::NotFound(format!("read {}: {e}", path.display())))?;
        let updated = replace_markers(&original, &agents_block)?;
        std::fs::write(&path, &updated)
            .map_err(|e| AwareError::Internal(format!("write {}: {e}", path.display())))?;
        println!(
            "\u{2713} regenerated AGENTS block in {} ({} agents across {} verticals)",
            path.display(),
            agents.len(),
            count_verticals(&agents),
        );
    } else {
        println!("{}", render_standalone(&agents, &agents_block));
    }
    Ok(())
}

/// Render a single-vertical diagram. Each agent becomes its own node with
/// skill + command counts. Useful for a focused view when the master
/// diagram's per-vertical bucket gets crowded.
fn render_vertical(agents: &[DiscoveredAgent], vertical: &str) -> Result<String, AwareError> {
    let normalized = vertical.to_lowercase();
    let allowed = [
        "engineering",
        "architecture",
        "construction",
        "visualization",
        "cross-cutting",
        "operations",
        "meta",
    ];
    if !allowed.contains(&normalized.as_str()) {
        return Err(AwareError::Validation(format!(
            "unknown vertical '{vertical}' — expected one of: {}",
            allowed.join(", ")
        )));
    }

    let in_vertical: Vec<&DiscoveredAgent> = agents
        .iter()
        .filter(|a| {
            let kws: Vec<&str> = a.manifest.keywords.iter().map(|s| s.as_str()).collect();
            kws.contains(&normalized.as_str())
        })
        .collect();

    let label = match normalized.as_str() {
        "engineering" => "Engineering",
        "architecture" => "Architecture",
        "construction" => "Construction",
        "visualization" => "Visualization",
        "cross-cutting" => "Cross-cutting",
        "operations" => "Operations",
        _ => "Meta",
    };

    let mut out = String::new();
    out.push_str("%%{init: {'theme':'neutral'}}%%\n");
    out.push_str("graph TB\n\n");
    out.push_str(
        "  classDef poc        fill:#4ade80,stroke:#16a34a,color:#0f172a,stroke-width:2px\n",
    );
    out.push_str("  classDef future     fill:#f1f5f9,stroke:#94a3b8,color:#475569\n");
    out.push_str(
        "  classDef substrate  fill:#dbeafe,stroke:#2563eb,color:#0f172a,stroke-width:2px\n\n",
    );

    let total_skills: usize = in_vertical.iter().map(|a| a.manifest.skill_count()).sum();
    let total_cmds: usize = in_vertical.iter().map(|a| a.manifest.command_count()).sum();
    out.push_str(&format!(
        "  Header[\"AWARE {label}<br/>{n} agents · {total_skills} skills · {total_cmds} cmds\"]:::substrate\n\n",
        n = in_vertical.len()
    ));

    if in_vertical.is_empty() {
        out.push_str(&format!(
            "  Empty[\"(no installed agents in {label})\"]:::future\n  Header --> Empty\n"
        ));
        return Ok(out);
    }

    // Group agents by vendor for color/cluster
    let mut by_vendor: std::collections::BTreeMap<String, Vec<&&DiscoveredAgent>> =
        std::collections::BTreeMap::new();
    for a in &in_vertical {
        let vendor = a
            .manifest
            .vendor
            .clone()
            .unwrap_or_else(|| "unknown".into());
        by_vendor.entry(vendor).or_default().push(a);
    }

    for (vendor, agents_v) in &by_vendor {
        let safe_vendor = vendor.replace('-', "_");
        out.push_str(&format!(
            "  subgraph V_{safe_vendor}[\"{vendor}\"]\n    direction TB\n"
        ));
        for a in agents_v {
            let id = a.manifest.agent.replace('-', "_");
            let display = a
                .manifest
                .display_name
                .as_deref()
                .unwrap_or(a.manifest.agent.as_str());
            out.push_str(&format!(
                "    {id}[\"{display}<br/>{skills} skills · {cmds} cmds\"]:::poc\n",
                skills = a.manifest.skill_count(),
                cmds = a.manifest.command_count()
            ));
        }
        out.push_str("  end\n");
        out.push_str(&format!("  Header --> V_{safe_vendor}\n"));
    }
    Ok(out)
}

const MARKER_BEGIN: &str = "%% AWARE-AUTOGEN-AGENTS-BEGIN";
const MARKER_END: &str = "%% AWARE-AUTOGEN-AGENTS-END";

/// Replace content between markers in `original`. Both markers must exist
/// and `BEGIN` must precede `END`; otherwise we return a validation error
/// rather than silently appending — making the failure mode loud.
fn replace_markers(original: &str, new_block: &str) -> Result<String, AwareError> {
    let begin = original.find(MARKER_BEGIN).ok_or_else(|| {
        AwareError::Validation(format!("marker {MARKER_BEGIN} not found in target file"))
    })?;
    let end = original.find(MARKER_END).ok_or_else(|| {
        AwareError::Validation(format!("marker {MARKER_END} not found in target file"))
    })?;
    if end < begin {
        return Err(AwareError::Validation(format!(
            "marker order wrong: {MARKER_END} appears before {MARKER_BEGIN}"
        )));
    }

    // Find the newline after BEGIN marker (we keep the marker line itself).
    let after_begin = original[begin..]
        .find('\n')
        .map(|n| begin + n + 1)
        .ok_or_else(|| {
            AwareError::Validation(format!("{MARKER_BEGIN} marker has no trailing newline"))
        })?;

    let mut out = String::with_capacity(original.len() + new_block.len());
    out.push_str(&original[..after_begin]);
    out.push_str(new_block);
    if !new_block.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&original[end..]);
    Ok(out)
}

/// Auto-regenerate `<aware_home>/diagrams/installed.mmd` after install /
/// uninstall / update. Best-effort — failures don't tear down the caller.
pub fn auto_regenerate(ctx: &Context) -> Result<(), AwareError> {
    let agents = discover_agents(&ctx.paths)?;
    let diagrams_dir = ctx.paths.diagrams_dir();
    std::fs::create_dir_all(&diagrams_dir)
        .map_err(|e| AwareError::Internal(format!("mkdir {}: {e}", diagrams_dir.display())))?;
    let agents_block = render_agents_block(&agents);
    let body = render_standalone(&agents, &agents_block);
    let path = diagrams_dir.join("installed.mmd");
    std::fs::write(&path, body)
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", path.display())))?;
    Ok(())
}

/// Render the standalone Mermaid diagram — the full picture, not just the
/// agent block. Useful for end users wanting to see "what's in my AWARE
/// install" without needing the repo's hand-curated `aware-master.mmd`.
fn render_standalone(agents: &[DiscoveredAgent], agents_block: &str) -> String {
    let mut out = String::new();
    out.push_str("%%{init: {'theme':'neutral'}}%%\n");
    out.push_str("graph TB\n\n");
    out.push_str(
        "  classDef poc        fill:#4ade80,stroke:#16a34a,color:#0f172a,stroke-width:2px\n",
    );
    out.push_str("  classDef future     fill:#f1f5f9,stroke:#94a3b8,color:#475569\n");
    out.push_str(
        "  classDef substrate  fill:#dbeafe,stroke:#2563eb,color:#0f172a,stroke-width:2px\n",
    );
    out.push_str(
        "  classDef meta       fill:#fef3c7,stroke:#d97706,color:#0f172a,stroke-width:2px\n\n",
    );

    out.push_str(&format!(
        "  CLI[\"aware CLI<br/>{} installed agents\"]:::substrate\n\n",
        agents.len()
    ));

    out.push_str("  subgraph AGENTS[\"Installed agents\"]\n");
    out.push_str("    direction TB\n");
    out.push_str(agents_block);
    out.push_str("  end\n\n");

    out.push_str("  CLI --> AGENTS\n");
    out
}

/// Render the lines that go INSIDE the AGENTS subgraph. One node per
/// AECO vertical, using the canonical IDs the rest of `aware-master.mmd`
/// references (`Eng`/`Arch`/`Cons`/`Cross`/`Ops`). Meta primitives + utility
/// agents (html-report) are NOT emitted here — they live in their own
/// subgraphs (META) higher up in the diagram.
fn render_agents_block(agents: &[DiscoveredAgent]) -> String {
    let buckets = bucket_by_vertical(agents);

    // Per-vertical static "future:" hints — the products we haven't shipped
    // agents for yet. Hand-curated; the substrate doesn't track future state.
    // Update this list in code when shipping (or retiring) a future placeholder.
    let verticals = [
        (
            "Eng",
            "Engineering",
            &buckets.engineering,
            "ETABS · STAAD · IDEA · Robot · Civil 3D",
            "poc",
        ),
        (
            "Arch",
            "Architecture",
            &buckets.architecture,
            "ArchiCAD · Allplan",
            "poc",
        ),
        (
            "Cons",
            "Construction",
            &buckets.construction,
            "Slack · Procore · ACC · Navisworks",
            "poc",
        ),
        (
            "Viz",
            "Visualization",
            &buckets.visualization,
            "Lumion · Enscape · Twinmotion · V-Ray",
            "poc",
        ),
        (
            "Cross",
            "Cross-cutting",
            &buckets.cross_cutting,
            "file · email · excel · think-node",
            "poc",
        ),
        (
            "Ops",
            "Operations",
            &buckets.ops,
            "IBM Maximo · Autodesk Tandem · Planon · ServiceNow",
            "future",
        ),
    ];

    let mut out = String::new();
    for (id, label, agents_in, future_hint, fallback_class) in verticals {
        let class = if agents_in.is_empty() {
            fallback_class
        } else {
            "poc"
        };
        let star = if class == "poc" { " \u{2605}" } else { "" };

        let mut lines: Vec<String> = agents_in
            .iter()
            .map(|a| {
                let display = a
                    .manifest
                    .display_name
                    .clone()
                    .unwrap_or_else(|| a.manifest.agent.clone());
                format!(
                    "{display} \u{2605} ({sk} skills · {cmd} cmds)",
                    display = display,
                    sk = a.manifest.skill_count(),
                    cmd = a.manifest.commands.len()
                )
            })
            .collect();
        if !future_hint.is_empty() {
            lines.push(format!("future: {future_hint}"));
        }

        out.push_str(&format!(
            "    {id}[\"{label}{star}<br/>{lines}\"]:::{class}\n",
            id = id,
            label = label,
            star = star,
            lines = lines.join("<br/>"),
            class = class,
        ));
    }
    out
}

#[derive(Default)]
struct Buckets<'a> {
    engineering: Vec<&'a DiscoveredAgent>,
    architecture: Vec<&'a DiscoveredAgent>,
    construction: Vec<&'a DiscoveredAgent>,
    visualization: Vec<&'a DiscoveredAgent>,
    cross_cutting: Vec<&'a DiscoveredAgent>,
    ops: Vec<&'a DiscoveredAgent>,
}

/// Bucket agents by AECO vertical based on the `keywords:` field. Heuristic
/// but matches how the substrate's own agents are authored. Anything not
/// matching an AECO vertical (meta primitives, utilities like html-report,
/// non-AECO agents) is silently dropped from this view — those agents live
/// in their own subgraphs higher up in the master diagram.
fn bucket_by_vertical(agents: &[DiscoveredAgent]) -> Buckets<'_> {
    let mut buckets = Buckets::default();
    for a in agents {
        let kws: Vec<&str> = a.manifest.keywords.iter().map(|s| s.as_str()).collect();
        if kws.contains(&"engineering") {
            buckets.engineering.push(a);
        } else if kws.contains(&"architecture") {
            buckets.architecture.push(a);
        } else if kws.contains(&"construction") {
            buckets.construction.push(a);
        } else if kws.contains(&"visualization") {
            buckets.visualization.push(a);
        } else if kws.contains(&"cross-cutting") {
            buckets.cross_cutting.push(a);
        } else if kws.contains(&"operations") {
            buckets.ops.push(a);
        }
        // else: meta primitives + utilities + everything else — not in AGENTS
    }
    buckets
}

fn count_verticals(agents: &[DiscoveredAgent]) -> usize {
    let b = bucket_by_vertical(agents);
    [
        !b.engineering.is_empty(),
        !b.architecture.is_empty(),
        !b.construction.is_empty(),
        !b.visualization.is_empty(),
        !b.cross_cutting.is_empty(),
        !b.ops.is_empty(),
    ]
    .iter()
    .filter(|x| **x)
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_markers_inserts_between_markers() {
        let original = "line 1\n%% AWARE-AUTOGEN-AGENTS-BEGIN\n  old content\n%% AWARE-AUTOGEN-AGENTS-END\nline 5\n";
        let new_block = "  new content\n";
        let updated = replace_markers(original, new_block).unwrap();
        assert!(updated.contains("line 1"));
        assert!(updated.contains("new content"));
        assert!(updated.contains("line 5"));
        assert!(!updated.contains("old content"));
    }

    #[test]
    fn replace_markers_idempotent() {
        let original = "%% AWARE-AUTOGEN-AGENTS-BEGIN\n  old\n%% AWARE-AUTOGEN-AGENTS-END\n";
        let new_block = "  same content\n";
        let once = replace_markers(original, new_block).unwrap();
        let twice = replace_markers(&once, new_block).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn replace_markers_missing_begin_errors() {
        let original = "%% AWARE-AUTOGEN-AGENTS-END\n";
        let err = replace_markers(original, "").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn replace_markers_missing_end_errors() {
        let original = "%% AWARE-AUTOGEN-AGENTS-BEGIN\n";
        let err = replace_markers(original, "").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn replace_markers_wrong_order_errors() {
        let original = "%% AWARE-AUTOGEN-AGENTS-END\n%% AWARE-AUTOGEN-AGENTS-BEGIN\n";
        let err = replace_markers(original, "").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }
}
