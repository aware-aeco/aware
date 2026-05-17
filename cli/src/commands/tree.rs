//! `aware tree <agent>` — text-tree view of an agent's commands grouped by
//! their owning type (extracted from the `description` field, which for
//! reflected agents starts with `TypeName.MethodName`).
//!
//! For curated agents whose descriptions are vendor prose rather than
//! `Type.Method`, commands fall under a single `Top-level` bucket.

use std::collections::BTreeMap;
use std::time::Instant;

use clap::Args;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::manifest::loader::discover_agents;

#[derive(Args, Debug)]
pub struct TreeArgs {
    /// Agent id, optionally followed by `/Class` to drill into a single group.
    /// Examples: `tekla`, `revit-2026/Document`, `itwin-5-9/Viewport`.
    pub agent: String,
}

pub fn run(ctx: &Context, args: &TreeArgs) -> Result<(), AwareError> {
    let started = Instant::now();
    let (agent_id, class_filter) = match args.agent.split_once('/') {
        Some((a, c)) => (a.to_string(), Some(c.to_string())),
        None => (args.agent.clone(), None),
    };

    let discovered = discover_agents(&ctx.paths)?;
    let agent = discovered
        .iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| AwareError::NotFound(format!("agent: {agent_id}")))?;

    let mut groups: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for (name, cmd) in &agent.manifest.commands {
        let group = extract_group(&cmd.description);
        groups
            .entry(group)
            .or_default()
            .push((name.clone(), cmd.description.clone()));
    }

    if ctx.json {
        let data = TreeData {
            agent: &agent.manifest.agent,
            display_name: agent.manifest.display_name.as_deref(),
            skill_count: agent.manifest.skill_count(),
            command_count: agent.manifest.command_count(),
            class_filter: class_filter.as_deref(),
            groups: groups
                .iter()
                .filter(|(g, _)| {
                    class_filter
                        .as_ref()
                        .is_none_or(|c| g.as_str() == c.as_str())
                })
                .map(|(g, cmds)| TreeGroup {
                    name: g,
                    commands: cmds
                        .iter()
                        .map(|(n, d)| TreeCommand {
                            name: n,
                            description: d,
                        })
                        .collect(),
                })
                .collect(),
        };
        envelope::print_ok("tree", data, started).ok();
        return Ok(());
    }

    let header = match agent.manifest.display_name.as_deref() {
        Some(d) => format!(
            "{} ({d} · {} skills · {} cmds)",
            agent.manifest.agent,
            agent.manifest.skill_count(),
            agent.manifest.command_count()
        ),
        None => format!(
            "{} ({} skills · {} cmds)",
            agent.manifest.agent,
            agent.manifest.skill_count(),
            agent.manifest.command_count()
        ),
    };
    println!("{header}");

    let visible: Vec<(&String, &Vec<(String, String)>)> = groups
        .iter()
        .filter(|(g, _)| class_filter.as_ref().is_none_or(|c| *g == c))
        .collect();

    if visible.is_empty() {
        if let Some(c) = &class_filter {
            println!("(no commands found for {c})");
        } else {
            println!("(no commands)");
        }
        return Ok(());
    }

    for (gi, (group, cmds)) in visible.iter().enumerate() {
        let is_last_group = gi == visible.len() - 1;
        let connector = if is_last_group {
            "└──"
        } else {
            "├──"
        };
        println!("{connector} {group}");
        let indent = if is_last_group { "    " } else { "│   " };
        for (ci, (name, _desc)) in cmds.iter().enumerate() {
            let is_last_cmd = ci == cmds.len() - 1;
            let cmd_conn = if is_last_cmd {
                "└──"
            } else {
                "├──"
            };
            println!("{indent}{cmd_conn} {name}");
        }
    }
    Ok(())
}

/// Extract the group bucket for a command from its `description`.
///
/// Heuristic: if the description starts with `Word.AnotherWord` (no spaces
/// between Word and `.`), that's the owning class — return the leading
/// `Word`. Otherwise return `Top-level`.
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

#[derive(Serialize)]
struct TreeData<'a> {
    agent: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<&'a str>,
    skill_count: usize,
    command_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_filter: Option<&'a str>,
    groups: Vec<TreeGroup<'a>>,
}

#[derive(Serialize)]
struct TreeGroup<'a> {
    name: &'a str,
    commands: Vec<TreeCommand<'a>>,
}

#[derive(Serialize)]
struct TreeCommand<'a> {
    name: &'a str,
    description: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_group_picks_leading_class_from_type_method() {
        assert_eq!(extract_group("Viewer.LoadModel"), "Viewer");
        assert_eq!(extract_group("Camera.GetFlight()"), "Camera");
        assert_eq!(extract_group("IFC4.Wall"), "IFC4");
    }

    #[test]
    fn extract_group_falls_back_to_top_level_for_prose() {
        assert_eq!(
            extract_group("Subscribe to ModelObjectChanged events on the active Tekla model."),
            "Top-level"
        );
        assert_eq!(
            extract_group("Adds the given assemblable instance to the assembly."),
            "Top-level"
        );
    }

    #[test]
    fn extract_group_handles_leading_whitespace() {
        assert_eq!(extract_group("  Viewer.LoadModel"), "Viewer");
    }

    #[test]
    fn extract_group_treats_word_then_space_as_prose() {
        // "Foo bar" is prose; "Foo.bar" is Type.Method
        assert_eq!(
            extract_group("Returns true if a property is overridden"),
            "Top-level"
        );
    }
}
