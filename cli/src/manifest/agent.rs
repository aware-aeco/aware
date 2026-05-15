//! Typed deserialization for AWARE agent manifests.
//!
//! Shapes verified against all 7 reference agents:
//! - `20-agents/aeco/engineering/tekla/manifest.yaml`
//! - `20-agents/aeco/construction/trimble-connect/manifest.yaml`
//! - `20-agents/aeco/cross-cutting/microsoft-365/manifest.yaml`
//! - `20-agents/aeco/cross-cutting/google-workspace/manifest.yaml`
//! - `20-agents/_core/aware-agent-builder/manifest.yaml`
//! - `20-agents/_core/aware-skill-builder/manifest.yaml`
//! - `20-agents/_core/html-report/manifest.yaml`
//!
//! `#![allow(dead_code)]` covers fields used by `agent describe` (Task 10+):
//! `homepage`, `keywords`, `provenance`, `requires`, `transport` details,
//! and nested structs (`Provenance`, `Requires`, `TransportCli`, `Command`,
//! `Lifecycle`). The `agent`, `version`, `skills`, `commands`, `kind()`,
//! `skill_count()`, and `command_count()` members are consumed by Task 9
//! (`agent list`). Keep the file-level allow until all describe/run tasks land.
#![allow(dead_code)]

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_yaml::Value;

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub agent: String,
    pub version: String,
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    pub description: String,
    pub stateful: bool,
    pub vendor: Option<String>,
    pub license: String,
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub provenance: Option<Provenance>,
    pub requires: Option<Requires>,
    pub transport: Transport,
    #[serde(default)]
    pub commands: BTreeMap<String, Command>,
    #[serde(default)]
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Provenance {
    #[serde(rename = "generated-by")]
    pub generated_by: Option<String>,
    #[serde(rename = "generator-version")]
    pub generator_version: Option<String>,
    pub source: Option<Value>,
    #[serde(rename = "refined-by", default)]
    pub refined_by: Vec<String>,
    #[serde(rename = "generated-at")]
    pub generated_at: Option<String>,
    /// Present in some ported agents (e.g. microsoft-365, google-workspace)
    /// to record the FloLess production path the skills were ported from.
    #[serde(rename = "ported-from")]
    pub ported_from: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Requires {
    #[serde(default)]
    pub filesystem: Vec<Value>,
    #[serde(default)]
    pub network: Vec<String>,
    #[serde(default)]
    pub software: Vec<String>,
    #[serde(default)]
    pub secrets: Vec<String>,
    /// Present in `aware-skill-builder` to declare a dependency on an
    /// Anthropic built-in skill (e.g. `anthropic:skill-creator`).
    #[serde(default)]
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Transport {
    pub cli: Option<TransportCli>,
    pub mcp: Option<Value>,
    pub rest: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct TransportCli {
    pub binary: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub lifecycle: Lifecycle,
    pub description: String,
    #[serde(default)]
    pub inputs: Value,
    pub outputs: Option<Value>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    Start,
    Stop,
    Single,
}

impl Agent {
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Human-readable kind label for the `agent list` table: lowercased
    /// `display-name`, falling back to `agent` id, truncated to 17 chars.
    pub fn kind(&self) -> String {
        let raw = self
            .display_name
            .as_ref()
            .map(|d| d.to_lowercase())
            .unwrap_or_else(|| self.agent.clone());
        if raw.chars().count() > 17 {
            let mut s: String = raw.chars().take(14).collect();
            s.push_str("...");
            s
        } else {
            raw
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEKLA_MIN: &str = r#"
agent: tekla
version: 2025.0.1
display-name: Tekla Structures
description: |
  Watches the active Tekla model.
stateful: true
license: Apache-2.0
transport:
  cli:
    binary: aware-tekla
commands:
  watch:
    lifecycle: start
    description: Subscribe.
    outputs:
      type: stream
  insert:
    lifecycle: single
    description: Create.
skills:
  - drawing-identity.md
  - event-threading.md
"#;

    #[test]
    fn parses_minimal_manifest() {
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert_eq!(a.agent, "tekla");
        assert_eq!(a.version, "2025.0.1");
        assert!(a.stateful);
        assert_eq!(a.license, "Apache-2.0");
        assert_eq!(a.skill_count(), 2);
        assert_eq!(a.command_count(), 2);
        assert_eq!(a.transport.cli.unwrap().binary, "aware-tekla");
    }

    #[test]
    fn kind_uses_lowercased_display_name() {
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert_eq!(a.kind(), "tekla structures");
    }

    #[test]
    fn kind_truncates_long_names() {
        let yaml = TEKLA_MIN.replace("Tekla Structures", "Some Extremely Long Name For The Agent");
        let a: Agent = serde_yaml::from_str(&yaml).unwrap();
        let k = a.kind();
        assert!(k.chars().count() <= 17, "kind too long: {k:?}");
        assert!(k.ends_with("..."));
    }

    #[test]
    fn kind_falls_back_to_agent_id_when_no_display_name() {
        let yaml = r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: {}
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(a.kind(), "tekla");
    }

    #[test]
    fn missing_required_field_errors() {
        let bad = "agent: tekla\nversion: 1.0";
        assert!(serde_yaml::from_str::<Agent>(bad).is_err());
    }

    #[test]
    fn parses_real_tekla_manifest() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents/aeco/engineering/tekla/manifest.yaml");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: Agent = serde_yaml::from_str(&text).unwrap();
        assert_eq!(a.agent, "tekla");
        assert_eq!(a.skill_count(), 31);
        assert_eq!(a.command_count(), 3);
        assert!(a.stateful);
    }
}
