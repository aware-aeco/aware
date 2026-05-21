//! Agent builder framework — shared types + write_agent helper.
//!
//! Per-source builders land in tasks 3-7 (openapi / cli_help / nuget / python / stubs).

#![allow(dead_code)]

pub mod cli_help;
pub mod coverage;
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
    /// Substrate semver. Starts at "0.1.0" for newly-generated agents.
    pub version: String,
    /// Optional vendor SDK / package version this agent reflects. Distinct
    /// from `version` (which tracks substrate revisions of the agent).
    pub sdk_target: Option<String>,
    pub description: String,
    pub commands: BTreeMap<String, GeneratedCommand>,
    pub skills: Vec<GeneratedSkill>,
    pub provenance: Provenance,
    pub stateful: bool,
    pub license: String,
    /// When set, the agent uses the `rest` transport (HTTP API) with an
    /// optional base URL and declarative auth, instead of the default `cli`
    /// transport (a wrapped binary). Set by the OpenAPI builder; `None` for
    /// SDK/CLI-derived agents.
    pub rest: Option<RestBlock>,
}

/// REST transport + declarative auth for a generated HTTP-API agent.
#[derive(Debug, Default)]
pub struct RestBlock {
    /// Base URL (from the OpenAPI `servers`); commands pass relative paths.
    pub base: Option<String>,
    /// Declarative auth derived from the spec's `securitySchemes`.
    pub auth: Option<AuthBlock>,
}

/// Declarative auth emitted into the manifest's `auth:` block. The credential
/// `secret` is also added to `requires.secrets`.
#[derive(Debug)]
pub struct AuthBlock {
    /// `api-key` | `bearer` | `oauth2`.
    pub scheme: String,
    /// For `api-key`: `header` | `query`.
    pub location: Option<String>,
    /// For `api-key`: the header / query-param name.
    pub name: Option<String>,
    /// Credential handle (also added to `requires.secrets`).
    pub secret: String,
}

#[derive(Debug, Default)]
pub struct GeneratedCommand {
    pub lifecycle: String,
    pub description: String,
    pub inputs_yaml: String,
    pub outputs_yaml: String,
    /// REST operation mapping (OpenAPI builder): HTTP method + path template,
    /// emitted into the manifest so the REST transport can execute the command.
    /// `None` for SDK/CLI-derived commands.
    pub method: Option<String>,
    pub path: Option<String>,
    /// Explicit read/write mode (e.g. `write` for mutating HTTP methods), so the
    /// safety contract applies regardless of the command name. `None` lets the
    /// loader infer mode from the name convention.
    pub mode: Option<String>,
    /// Public endpoint that does not use the agent's declared auth (OpenAPI
    /// operation with empty effective security). Emitted as `no-auth: true`.
    pub no_auth: bool,
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
        let mut body = format!(
            "# {name}\n\nLifecycle: {}\n\n{}\n",
            cmd.lifecycle, cmd.description
        );
        if !cmd.inputs_yaml.trim().is_empty() {
            body.push_str(&format!("\n## Inputs\n\n```yaml\n{}```\n", cmd.inputs_yaml));
        }
        if !cmd.outputs_yaml.trim().is_empty() {
            body.push_str(&format!(
                "\n## Output\n\n```yaml\n{}```\n",
                cmd.outputs_yaml
            ));
        }
        std::fs::write(dst.join("commands").join(format!("{name}.md")), body)?;
    }

    Ok(dst)
}

fn build_manifest_yaml(agent: &GeneratedAgent) -> Result<String, AwareError> {
    let mut out = String::new();
    out.push_str(&format!("agent:        {}\n", agent.id));
    out.push_str(&format!("version:      {}\n", agent.version));
    if let Some(sdk) = &agent.sdk_target {
        out.push_str(&format!("sdk-target:   {sdk}\n"));
    }
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
    match &agent.rest {
        Some(rest) => {
            out.push_str("  rest:");
            match &rest.base {
                Some(base) => out.push_str(&format!("\n    base: {}\n", quote_yaml_scalar(base))),
                None => out.push_str(" {}\n"),
            }
        }
        None => {
            out.push_str(&format!("  cli:\n    binary: aware-{}\n", agent.id));
        }
    }

    // Declarative auth + the credential it requires (REST agents only).
    if let Some(auth) = agent.rest.as_ref().and_then(|r| r.auth.as_ref()) {
        out.push_str("auth:\n");
        out.push_str(&format!("  scheme: {}\n", auth.scheme));
        if let Some(loc) = &auth.location {
            out.push_str(&format!("  in: {loc}\n"));
        }
        if let Some(name) = &auth.name {
            out.push_str(&format!("  name: {}\n", quote_yaml_scalar(name)));
        }
        out.push_str(&format!("  secret: {}\n", quote_yaml_scalar(&auth.secret)));
        out.push_str("requires:\n  secrets:\n");
        out.push_str(&format!("    - {}\n", quote_yaml_scalar(&auth.secret)));
    }

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
            if let Some(mode) = &cmd.mode {
                out.push_str(&format!("    mode: {mode}\n"));
            }
            if let Some(method) = &cmd.method {
                out.push_str(&format!("    method: {method}\n"));
            }
            if let Some(path) = &cmd.path {
                out.push_str(&format!("    path: {}\n", quote_yaml_scalar(path)));
            }
            if cmd.no_auth {
                out.push_str("    no-auth: true\n");
            }
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
                ..Default::default()
            },
        );
        GeneratedAgent {
            id: "test-agent".into(),
            version: "0.1.0".into(),
            sdk_target: Some("1.2.3".into()),
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
            rest: None,
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
