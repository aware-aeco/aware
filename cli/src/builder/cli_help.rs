//! Run `<binary> --help`, parse for command listings, emit an agent.
//!
//! Heuristic: scan for a line containing "Commands:" or "Subcommands:";
//! treat subsequent indented lines as `<name>  <description>` pairs.

#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::builder::{GeneratedAgent, GeneratedCommand, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_cli(binary: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    let output = std::process::Command::new(binary)
        .arg("--help")
        .output()
        .map_err(|e| AwareError::Network(format!("spawn {binary} --help: {e}")))?;
    if !output.status.success() {
        return Err(AwareError::Network(format!(
            "{binary} --help failed (exit {:?})",
            output.status.code()
        )));
    }
    let help = String::from_utf8_lossy(&output.stdout);
    let commands = parse_help(&help);

    let id = agent_id.map(String::from).unwrap_or_else(|| {
        // Strip path separators; just use the binary basename
        std::path::Path::new(binary)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(binary)
            .to_string()
    });

    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "cli", "binary": binary }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: None,
        description: format!("Agent wrapping {binary} CLI"),
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license: "UNKNOWN".into(),
        rest: None,
    })
}

pub(crate) fn parse_help(help: &str) -> BTreeMap<String, GeneratedCommand> {
    let mut commands = BTreeMap::new();
    let mut in_commands = false;
    for line in help.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            // Don't reset in_commands on blank lines — many help outputs blank-line-separate
            // the section header from its entries
            continue;
        }
        let lower = trimmed.to_lowercase();
        if lower.starts_with("commands:") || lower.starts_with("subcommands:") {
            in_commands = true;
            continue;
        }
        if !in_commands {
            continue;
        }
        // Indented entry?
        if line.starts_with(' ') || line.starts_with('\t') {
            let mut parts = trimmed.splitn(2, char::is_whitespace);
            let name = parts.next().unwrap_or("").trim().to_string();
            let desc = parts.next().unwrap_or("").trim().to_string();
            if name.is_empty() || name.starts_with('-') {
                continue;
            }
            // Reject obvious section subheaders (e.g. "Options:")
            if name.ends_with(':') {
                in_commands = false;
                continue;
            }
            commands.insert(
                name.clone(),
                GeneratedCommand {
                    lifecycle: "single".into(),
                    description: if desc.is_empty() {
                        format!("subcommand {name}")
                    } else {
                        desc
                    },
                    inputs_yaml: String::new(),
                    outputs_yaml: String::new(),
                },
            );
        } else {
            // Top-level non-indented line; either a new section or end of commands
            in_commands = false;
        }
    }
    commands
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAKE_HELP: &str = "myapp 1.2.3 \u{2014} a tool\n\nUsage: myapp [OPTIONS] <COMMAND>\n\nCommands:\n  init       Initialize a project\n  build      Build the project\n  deploy     Deploy somewhere\n  -h, --help Print help\n\nOptions:\n  --verbose  Be verbose\n";

    #[test]
    fn parses_three_commands_from_fake_help() {
        let cmds = parse_help(FAKE_HELP);
        assert_eq!(cmds.len(), 3);
        assert!(cmds.contains_key("init"));
        assert!(cmds.contains_key("build"));
        assert!(cmds.contains_key("deploy"));
        // -h is filtered (starts with -)
        assert!(!cmds.contains_key("-h"));
    }

    #[test]
    fn stops_at_options_section() {
        let help = "Commands:\n  one        First\n\nOptions:\n  --verbose  Be verbose\n";
        let cmds = parse_help(help);
        assert_eq!(cmds.len(), 1);
        assert!(cmds.contains_key("one"));
    }

    #[test]
    fn handles_no_commands_section_gracefully() {
        let help = "Usage: foo [args]\n\nNo subcommands.\n";
        let cmds = parse_help(help);
        assert!(cmds.is_empty());
    }
}
