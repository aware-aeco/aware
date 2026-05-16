//! Schema + cycle + capability validation for agents and apps.
//!
//! Pure functions — no I/O except optional `skills/` directory probe when
//! the caller passes a path (used by `validate_agent_on_disk`).
//!
//! Public API consumed by:
//! - Task 6  (`agent validate` command)
//! - Task 14 (`app validate` command)
//! - install pre-flight (Task 10+)

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::manifest::{Agent, App};

#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
}

impl ValidationIssue {
    fn error(code: &'static str, msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: msg.into(),
        }
    }
    fn warning(code: &'static str, msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: msg.into(),
        }
    }
}

pub fn has_errors(issues: &[ValidationIssue]) -> bool {
    issues.iter().any(|i| i.severity == Severity::Error)
}

/// Validate an `Agent` manifest without touching disk. Catches issues
/// `serde` can't (empty command map, missing transport, stateful-without-start, etc.).
pub fn validate_agent(agent: &Agent) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    if agent.agent.trim().is_empty() {
        out.push(ValidationIssue::error(
            "E_AGENT_EMPTY_ID",
            "agent id is empty",
        ));
    }
    if agent.commands.is_empty() {
        out.push(ValidationIssue::error(
            "E_AGENT_NO_COMMANDS",
            "agent declares zero commands",
        ));
    }
    if agent.transport.cli.is_none()
        && agent.transport.mcp.is_none()
        && agent.transport.rest.is_none()
    {
        out.push(ValidationIssue::error(
            "E_AGENT_NO_TRANSPORT",
            "agent must declare at least one transport (cli / mcp / rest)",
        ));
    }
    if agent.stateful {
        let has_start = agent
            .commands
            .values()
            .any(|c| matches!(c.lifecycle, crate::manifest::agent::Lifecycle::Start));
        if !has_start {
            out.push(ValidationIssue::error(
                "E_STATEFUL_NO_START",
                "stateful agent must have at least one command with lifecycle: start",
            ));
        }
    }
    out
}

/// Disk-aware: confirms every manifest-listed skill file exists, warns on orphans.
pub fn validate_agent_on_disk(agent: &Agent, agent_root: &Path) -> Vec<ValidationIssue> {
    let mut out = validate_agent(agent);
    let skills_dir = agent_root.join("skills");
    for skill in &agent.skills {
        if !skills_dir.join(skill).is_file() {
            out.push(ValidationIssue::error(
                "E_SKILL_FILE_MISSING",
                format!("manifest lists skill {skill:?} but file is missing under skills/"),
            ));
        }
    }
    if skills_dir.is_dir() {
        let listed: HashSet<_> = agent.skills.iter().cloned().collect();
        if let Ok(read) = std::fs::read_dir(&skills_dir) {
            for entry in read.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".md") && !listed.contains(&name) {
                    out.push(ValidationIssue::warning(
                        "W_SKILL_ORPHAN",
                        format!("skill file {name:?} on disk but not listed in manifest skills:"),
                    ));
                }
            }
        }
    }
    out
}

/// Validate an `App` manifest: cycles, dangling refs, unique node ids, inline-glue descriptions.
#[allow(dead_code)] // consumed by Task 14 (`aware app validate`)
pub fn validate_app(app: &App) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    if app.app.trim().is_empty() {
        out.push(ValidationIssue::error("E_APP_EMPTY_ID", "app id is empty"));
    }
    if app.nodes.is_empty() {
        out.push(ValidationIssue::error(
            "E_APP_NO_NODES",
            "app declares zero nodes",
        ));
    }

    let mut seen = HashSet::new();
    for n in &app.nodes {
        if !seen.insert(n.id.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DUPLICATE_NODE_ID",
                format!("duplicate node id: {}", n.id),
            ));
        }
    }

    let node_ids: HashSet<&str> = app.nodes.iter().map(|n| n.id.as_str()).collect();
    for c in &app.connections {
        if !node_ids.contains(c.from.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DANGLING_FROM",
                format!("connection from unknown node: {}", c.from),
            ));
        }
        if !node_ids.contains(c.to.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DANGLING_TO",
                format!("connection to unknown node: {}", c.to),
            ));
        }
    }

    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for c in &app.connections {
        graph
            .entry(c.from.as_str())
            .or_default()
            .push(c.to.as_str());
    }
    let mut visited: HashSet<&str> = HashSet::new();
    let mut on_stack: HashSet<&str> = HashSet::new();
    for n in &app.nodes {
        if !visited.contains(n.id.as_str())
            && has_cycle(n.id.as_str(), &graph, &mut visited, &mut on_stack)
        {
            out.push(ValidationIssue::error(
                "E_APP_CYCLE",
                format!("cycle detected in connections starting from node: {}", n.id),
            ));
            break;
        }
    }

    for n in &app.nodes {
        if let Some(inline) = &n.inline
            && inline.description.trim().is_empty()
        {
            out.push(ValidationIssue::error(
                "E_APP_INLINE_NO_DESC",
                format!("inline node {:?} missing description", n.id),
            ));
        }
    }
    out
}

#[allow(dead_code)] // called by validate_app above
fn has_cycle<'a>(
    node: &'a str,
    graph: &BTreeMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    on_stack: &mut HashSet<&'a str>,
) -> bool {
    visited.insert(node);
    on_stack.insert(node);
    if let Some(neighbors) = graph.get(node) {
        for &next in neighbors {
            if !visited.contains(next) {
                if has_cycle(next, graph, visited, on_stack) {
                    return true;
                }
            } else if on_stack.contains(next) {
                return true;
            }
        }
    }
    on_stack.remove(node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;

    fn load_agent(rel: &str) -> Agent {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel);
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
    fn load_app(rel: &str) -> App {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel);
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn real_tekla_passes_validation() {
        let a = load_agent("20-agents/aeco/engineering/tekla/manifest.yaml");
        let issues = validate_agent(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    #[test]
    fn real_welded_to_tc_passes_validation() {
        let a = load_app("30-apps/_examples/welded-to-tc.flo");
        let issues = validate_app(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    #[test]
    fn detects_cycle_in_connections() {
        let yaml = r#"
app: cyc
version: 0.0.1
description: |
  Cyclic app.
nodes:
  - id: a
  - id: b
  - id: c
connections:
  - { from: a, to: b }
  - { from: b, to: c }
  - { from: c, to: a }
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(has_errors(&issues));
        assert!(issues.iter().any(|i| i.code == "E_APP_CYCLE"));
    }

    #[test]
    fn detects_dangling_connection() {
        let yaml = r#"
app: dangle
version: 0.0.1
description: x
nodes:
  - id: a
connections:
  - { from: a, to: nope }
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(issues.iter().any(|i| i.code == "E_APP_DANGLING_TO"));
    }

    #[test]
    fn detects_duplicate_node_ids() {
        let yaml = r#"
app: dup
version: 0.0.1
description: x
nodes:
  - id: a
  - id: a
connections: []
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(issues.iter().any(|i| i.code == "E_APP_DUPLICATE_NODE_ID"));
    }

    #[test]
    fn stateful_without_start_is_error() {
        let yaml = r#"
agent: x
version: 1.0
description: y
stateful: true
license: MIT
transport: { cli: { binary: x } }
commands:
  do:
    lifecycle: single
    description: z
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_agent(&a);
        assert!(issues.iter().any(|i| i.code == "E_STATEFUL_NO_START"));
    }

    #[test]
    fn skill_file_missing_is_error() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("skills")).unwrap();
        let manifest = r#"
agent: x
version: 1.0
description: y
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: { do: { lifecycle: single, description: z } }
skills:
  - foo.md
"#;
        let a: Agent = serde_yaml::from_str(manifest).unwrap();
        let issues = validate_agent_on_disk(&a, tmp.path());
        assert!(issues.iter().any(|i| i.code == "E_SKILL_FILE_MISSING"));
    }
}
