//! Agent builder framework — shared types + write_agent helper.
//!
//! Per-source builders land in tasks 3-7 (openapi / cli_help / nuget / python / stubs).

#![allow(dead_code)]

pub mod cli_help;
pub mod npm;
pub mod nuget;
pub mod openapi;
pub mod python;
pub mod ruby;
pub mod stubs;
pub mod yard;

use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::error::AwareError;

#[derive(Debug)]
pub struct GeneratedAgent {
    pub id: String,
    pub version: String,
    pub description: String,
    pub commands: BTreeMap<String, GeneratedCommand>,
    pub skills: Vec<GeneratedSkill>,
    pub provenance: Provenance,
    pub stateful: bool,
    pub license: String,
}

#[derive(Debug)]
pub struct GeneratedCommand {
    pub lifecycle: String,
    pub description: String,
    pub inputs_yaml: String,
    pub outputs_yaml: String,
}

#[derive(Debug)]
pub struct GeneratedSkill {
    pub filename: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct Provenance {
    #[serde(rename = "generated-by")]
    pub generated_by: String,
    #[serde(rename = "generator-version")]
    pub generator_version: String,
    pub source: serde_json::Value,
    #[serde(rename = "generated-at")]
    pub generated_at: String,
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Write the generated agent into `<output_dir>/<agent-id>/`. Returns the new agent's root path.
pub fn write_agent(
    agent: &GeneratedAgent,
    output_dir: &Path,
) -> Result<std::path::PathBuf, AwareError> {
    let dst = output_dir.join(&agent.id);
    if dst.exists() {
        return Err(AwareError::Conflict(format!(
            "agent {} already exists at {}",
            agent.id,
            dst.display()
        )));
    }
    std::fs::create_dir_all(dst.join("skills"))?;
    std::fs::create_dir_all(dst.join("commands"))?;

    let manifest = build_manifest_yaml(agent)?;
    std::fs::write(dst.join("manifest.yaml"), manifest)?;

    for s in &agent.skills {
        std::fs::write(dst.join("skills").join(&s.filename), &s.body)?;
    }

    for (name, cmd) in &agent.commands {
        let body = format!(
            "# {name}\n\nLifecycle: {}\n\n{}\n",
            cmd.lifecycle, cmd.description
        );
        std::fs::write(dst.join("commands").join(format!("{name}.md")), body)?;
    }

    Ok(dst)
}

fn build_manifest_yaml(agent: &GeneratedAgent) -> Result<String, AwareError> {
    let mut out = String::new();
    out.push_str(&format!("agent:        {}\n", agent.id));
    out.push_str(&format!("version:      {}\n", agent.version));
    out.push_str(&format!(
        "description: |\n  {}\n",
        agent.description.replace('\n', "\n  ")
    ));
    out.push_str(&format!("stateful:     {}\n", agent.stateful));
    out.push_str(&format!("license:      {}\n", agent.license));

    out.push_str("provenance:\n");
    let prov_yaml = serde_yaml::to_string(&agent.provenance)
        .map_err(|e| AwareError::Internal(format!("provenance yaml: {e}")))?;
    for line in prov_yaml.lines() {
        if line.trim().is_empty() {
            continue;
        }
        out.push_str("  ");
        out.push_str(line);
        out.push('\n');
    }

    out.push_str("transport:\n");
    out.push_str(&format!("  cli:\n    binary: aware-{}\n", agent.id));

    if !agent.commands.is_empty() {
        out.push_str("commands:\n");
        for (name, cmd) in &agent.commands {
            out.push_str(&format!("  {name}:\n"));
            out.push_str(&format!("    lifecycle: {}\n", cmd.lifecycle));
            // Single-line description (YAML-safe)
            let desc_one_line = cmd.description.replace('\n', " ");
            out.push_str(&format!(
                "    description: {}\n",
                quote_yaml_scalar(&desc_one_line)
            ));
            if !cmd.inputs_yaml.trim().is_empty() {
                out.push_str("    inputs:\n");
                for line in cmd.inputs_yaml.lines() {
                    out.push_str("      ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
            if !cmd.outputs_yaml.trim().is_empty() {
                out.push_str("    outputs:\n");
                for line in cmd.outputs_yaml.lines() {
                    out.push_str("      ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
    } else {
        // serde_yaml chokes on empty map — write `commands: {}` explicitly
        out.push_str("commands: {}\n");
    }

    if !agent.skills.is_empty() {
        let mut names: Vec<String> = agent.skills.iter().map(|s| s.filename.clone()).collect();
        names.sort();
        out.push_str("skills:\n");
        for n in names {
            out.push_str(&format!("  - {n}\n"));
        }
    }

    Ok(out)
}

/// Quote a YAML scalar if it contains characters that would make it ambiguous (colons, special chars).
fn quote_yaml_scalar(s: &str) -> String {
    // YAML "plain" scalars (no quotes) can't start with control characters
    // that have special meaning. Vendor docstrings sometimes do — e.g.
    // "**** This method is for proxies …" from RhinoCommon. Quote on any of
    // these triggers to stay parseable.
    let starts_special = s.chars().next().is_some_and(|c| {
        matches!(
            c,
            '-' | '?' | '*' | '&' | '!' | '|' | '>' | '@' | '`' | '%' | '['
        )
    });
    if s.contains(':') || s.contains('#') || starts_special || s.contains('"') {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_agent() -> GeneratedAgent {
        let mut cmds = BTreeMap::new();
        cmds.insert(
            "do-thing".into(),
            GeneratedCommand {
                lifecycle: "single".into(),
                description: "Does a thing.".into(),
                inputs_yaml: String::new(),
                outputs_yaml: String::new(),
            },
        );
        GeneratedAgent {
            id: "test-agent".into(),
            version: "0.1.0".into(),
            description: "Test agent generated for unit testing.".into(),
            commands: cmds,
            skills: vec![GeneratedSkill {
                filename: "intro.md".into(),
                body: "# intro\n\nIntro content.\n".into(),
            }],
            provenance: Provenance {
                generated_by: "aware-agent-builder".into(),
                generator_version: "test".into(),
                source: serde_json::json!({"type": "test"}),
                generated_at: "2026-05-16T00:00:00Z".into(),
            },
            stateful: false,
            license: "MIT".into(),
        }
    }

    #[test]
    fn write_agent_creates_full_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let agent = sample_agent();
        let dst = write_agent(&agent, tmp.path()).unwrap();
        assert!(dst.join("manifest.yaml").is_file());
        assert!(dst.join("skills/intro.md").is_file());
        assert!(dst.join("commands/do-thing.md").is_file());
    }

    #[test]
    fn write_agent_rejects_existing_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let agent = sample_agent();
        write_agent(&agent, tmp.path()).unwrap();
        let err = write_agent(&agent, tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }

    #[test]
    fn generated_manifest_parses_via_loader() {
        let tmp = tempfile::tempdir().unwrap();
        let agent = sample_agent();
        let dst = write_agent(&agent, tmp.path()).unwrap();
        // Round-trip: load the generated manifest via the existing loader
        let loaded = crate::manifest::loader::load_agent(&dst.join("manifest.yaml")).unwrap();
        assert_eq!(loaded.agent, "test-agent");
        assert_eq!(loaded.version, "0.1.0");
        assert_eq!(loaded.commands.len(), 1);
        assert_eq!(loaded.skills.len(), 1);
    }
}
