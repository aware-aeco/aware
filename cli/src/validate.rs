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

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
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
        && agent.transport.app.is_none()
        && agent.transport.builtin.is_none()
    {
        out.push(ValidationIssue::error(
            "E_AGENT_NO_TRANSPORT",
            "agent must declare at least one transport (cli / mcp / rest / app / builtin)",
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

    // An `exposes-as-agent: true` app must declare at least one exposed command —
    // otherwise the synthesized agent has zero commands and is uncallable
    // (app-spec § exposes-as-agent / exposed-commands).
    if app.exposes_as_agent && app.exposed_commands.as_ref().is_none_or(|m| m.is_empty()) {
        out.push(ValidationIssue::error(
            "E_APP_EXPOSES_NO_COMMANDS",
            "app sets `exposes-as-agent: true` but declares no `exposed-commands:` \
             (the synthesized agent would have zero callable commands)",
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

    // #208: a `{{ <node>.<field> }}` data reference is an implicit scheduling edge
    // (see `app_lock::derive_connections` + the orchestrator's `topo_order`). A circular
    // DATA dependency is therefore just as unschedulable as an explicit connection cycle,
    // so include the derived edges here — catching it at `validate` / `compile` instead
    // of only at run, matching how explicit cycles are rejected.
    let derived = crate::app_lock::derive_connections(app);
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for c in app.connections.iter().chain(derived.iter()) {
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

    check_inline_nodes(&app.nodes, &mut out);
    out
}

/// Recursively validate inline-glue nodes, descending into `for-each` `do:` bodies
/// (which the compiler flattens and the runtime executes), so unsupported inline
/// kinds are caught wherever they appear, not just at the top level (#160).
fn check_inline_nodes(nodes: &[crate::manifest::app::Node], out: &mut Vec<ValidationIssue>) {
    for n in nodes {
        if let Some(inline) = &n.inline {
            if inline.description.trim().is_empty() {
                out.push(ValidationIssue::error(
                    "E_APP_INLINE_NO_DESC",
                    format!("inline node {:?} missing description", n.id),
                ));
            }
            // The runtime executes only `predicate` inline glue today (see
            // orchestrator). Reject other kinds so an unrunnable app fails at
            // validate/compile — not after the author validated + locked it.
            if inline.kind != "predicate" {
                out.push(ValidationIssue::error(
                    "E_APP_INLINE_KIND",
                    format!(
                        "inline node {:?}: kind {:?} is not runnable yet (only 'predicate' is supported)",
                        n.id, inline.kind
                    ),
                ));
            }
        }
        if let Some(body) = &n.do_ {
            check_inline_nodes(body, out);
        }
    }
}

/// Validate the safety contract for write-mode nodes against the installed
/// agent catalogue.
///
/// Per `10-core/app-spec.md § Safety contract (write-mode nodes)`:
/// any node whose command resolves to `Mode::Write` MUST declare a `safety:`
/// block. Apps missing `safety:` on a write-mode node are rejected by
/// `aware app validate` and refused by `aware app run`.
///
/// Agents referenced by the app but not installed are skipped (not an
/// error here — caught by lockfile resolution earlier).
#[allow(dead_code)] // wired by `aware app validate` + `aware app run` in v0.11
pub fn validate_app_safety(
    app: &App,
    agents: &[crate::manifest::loader::DiscoveredAgent],
) -> Vec<ValidationIssue> {
    use crate::manifest::agent::Mode;
    let mut out = Vec::new();
    for node in &app.nodes {
        // A frozen node never invokes its agent (the orchestrator emits its pinned output and
        // skips dispatch), so it never writes — the write-mode safety contract does not apply.
        if node.frozen.is_some() {
            continue;
        }
        let (Some(agent_id), Some(cmd_name)) = (node.agent.as_ref(), node.command.as_ref()) else {
            continue;
        };
        let Some(agent) = agents.iter().find(|d| d.manifest.agent == *agent_id) else {
            continue;
        };
        let Some(cmd) = agent.manifest.commands.get(cmd_name.as_str()) else {
            continue;
        };

        // Note: a node-level `mode:` that *conflicts* with a non-overridable
        // command is rejected by `validate_app_agents` (#165) — the agent-aware
        // validator that runs on every lock-producing + run path. Here we only
        // resolve the effective mode for the write-mode safety gate; on a
        // mode-overridable command an explicit `mode: read` legitimately makes
        // the node read-mode (so no `safety:` block is required).
        let effective = agent.manifest.effective_mode(cmd_name, cmd, node.mode);
        if effective.mode == Mode::Write && node.safety.is_none() {
            out.push(ValidationIssue::error(
                "E_APP_WRITE_WITHOUT_SAFETY",
                format!(
                    "node {:?} calls write-mode command {}.{} without a `safety:` block (required per app-spec § Safety contract)",
                    node.id, agent_id, cmd_name
                ),
            ));
        }
    }
    out
}

/// Reject nodes that reference an agent the runtime can't dispatch to — one
/// declared `status: planned` (no shipped/installable transport binary). Fails at
/// validate/compile rather than at run with "program not found" (#161). Recurses
/// into `for-each` `do:` bodies. Agents not in the catalogue are skipped here
/// (lockfile resolution / run handle the missing-agent case).
#[allow(dead_code)] // wired by `aware app validate` + `compile_to_disk` + `app run`
pub fn validate_app_agents(
    app: &App,
    agents: &[crate::manifest::loader::DiscoveredAgent],
) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    check_node_agents(&app.nodes, agents, app.exposes_as_agent, &app.app, &mut out);
    out
}

fn check_node_agents(
    nodes: &[crate::manifest::app::Node],
    agents: &[crate::manifest::loader::DiscoveredAgent],
    app_exposes: bool,
    app_id: &str,
    out: &mut Vec<ValidationIssue>,
) {
    use crate::manifest::agent::AgentStatus;
    for n in nodes {
        // A frozen node never dispatches to its agent (it emits a pinned output and short-circuits),
        // so agent/command availability does not gate it — and it needn't have its agent installed
        // at all. Skip it (and its `do:` body, which likewise never runs while frozen).
        if n.frozen.is_some() {
            continue;
        }

        // v0 no-recursion rule (self-reference): an `exposes-as-agent` app that
        // composes its own id recurses into itself. Catch it regardless of
        // catalogue state — on first install the synthesized agent does not
        // exist yet, so the discovered-agent check below would miss it.
        if app_exposes
            && let Some(agent_id) = &n.agent
            && agent_id == app_id
        {
            out.push(ValidationIssue::error(
                "E_APP_EXPOSED_COMPOSES_EXPOSED",
                format!(
                    "node {:?} composes agent {:?}, which is this app itself; an exposes-as-agent \
                     app cannot compose itself (v0 constraint, app-spec § exposes-as-agent \
                     constraints)",
                    n.id, agent_id
                ),
            ));
        }

        if let Some(agent_id) = &n.agent
            && let Some(d) = agents.iter().find(|d| d.manifest.agent == *agent_id)
        {
            if d.manifest.status == AgentStatus::Planned {
                out.push(ValidationIssue::error(
                    "E_APP_AGENT_UNAVAILABLE",
                    format!(
                        "node {:?} references agent {:?}, which is declared but not yet runnable \
                         (no shipped/installable transport binary)",
                        n.id, agent_id
                    ),
                ));
            } else if let Some(cmd_name) = &n.command
                && d.manifest
                    .commands
                    .get(cmd_name)
                    .is_some_and(|c| c.status == AgentStatus::Planned)
            {
                // Per-command availability (#199): the agent is runnable but this
                // specific command is declared `status: planned` (not yet wired —
                // e.g. a multi-step/binary REST op). Fail fast at validate/compile
                // rather than at run.
                out.push(ValidationIssue::error(
                    "E_APP_COMMAND_UNAVAILABLE",
                    format!(
                        "node {:?} references command {:?} of agent {:?}, which is declared but \
                         not yet runnable (planned)",
                        n.id, cmd_name, agent_id
                    ),
                ));
            }

            // v0 no-recursion rule: an `exposes-as-agent` app cannot itself
            // compose another exposes-as-agent app (an app-backed synthesized
            // agent). Caught here so it fails at install/validate/compile rather
            // than only at dispatch (app-spec § exposes-as-agent constraints).
            // (The self-reference case above also covers first-install, before
            // this app's own synthesized agent exists in the catalogue.)
            if app_exposes && agent_id != app_id && d.manifest.transport.app.is_some() {
                out.push(ValidationIssue::error(
                    "E_APP_EXPOSED_COMPOSES_EXPOSED",
                    format!(
                        "node {:?} composes agent {:?}, which is itself an exposes-as-agent app; \
                         an exposed app cannot compose another exposed app (v0 constraint, \
                         app-spec § exposes-as-agent constraints)",
                        n.id, agent_id
                    ),
                ));
            }

            // #165: a node-level `mode:` is only meaningful on a
            // `mode-overridable` command (caller-determined behavior, e.g.
            // `exec`). On any other command the manifest mode is authoritative,
            // so a *conflicting* node-level declaration would be silently
            // dropped. Reject it here — in the agent-aware validator that runs
            // on every lock-producing + run path (install, compile, inspect,
            // validate, run, dry-run), not just safety pre-flight — so the
            // authoring field is never quietly ignored on any surface.
            if let Some(cmd_name) = &n.command
                && let Some(cmd) = d.manifest.commands.get(cmd_name.as_str())
                && !cmd.mode_overridable
                && let Some(declared) = n.mode
                && declared != d.manifest.mode_of(cmd_name, cmd)
            {
                out.push(ValidationIssue::error(
                    "E_APP_NODE_MODE_NOT_OVERRIDABLE",
                    format!(
                        "node {:?} declares `mode: {}` but command {}.{} is not `mode-overridable`; its manifest mode ({}) is authoritative. Remove the node-level `mode:` (or have the agent declare the command `mode-overridable` if its mode is caller-determined).",
                        n.id,
                        declared.as_str(),
                        agent_id,
                        cmd_name,
                        d.manifest.mode_of(cmd_name, cmd).as_str(),
                    ),
                ));
            }

            // RFC #223 — the fenced runtime model-extraction carve-out. A command that
            // calls a model at run time (`model-extraction: true`) is admitted into the
            // deterministic run path ONLY when it is `category: curated` AND its agent
            // declares `capabilities.runtime-model-extraction: true`. Every other path to
            // the flag — a reflected/auto-generated command, or a curated command on an
            // agent that lacks the capability — is rejected here, so no node can mint a
            // model-reader except the single curated `vision.extract`. Keyed to the flag +
            // curated + capability (structural), never to the literal agent/command name,
            // so the exception to decalog #9 stays narrow and cannot widen by accident.
            if let Some(cmd_name) = &n.command
                && let Some(cmd) = d.manifest.commands.get(cmd_name.as_str())
                && cmd.model_extraction
            {
                let curated =
                    d.manifest.category_of(cmd) == crate::manifest::agent::Category::Curated;
                let capable = d
                    .manifest
                    .capabilities
                    .as_ref()
                    .is_some_and(|c| c.runtime_model_extraction);
                if !(curated && capable) {
                    out.push(ValidationIssue::error(
                        "E_APP_RUNTIME_MODEL_FORBIDDEN",
                        format!(
                            "node {:?} resolves to {}.{}, which declares `model-extraction: true` (it \
                             calls a model at run time) but is not an admitted carve-out: a runtime model \
                             extraction is permitted ONLY on a `category: curated` command whose agent \
                             declares `capabilities.runtime-model-extraction: true` ({}). The run path is \
                             deterministic — see RFC #223 + app-spec § Runtime model extraction (the one \
                             carve-out).",
                            n.id,
                            agent_id,
                            cmd_name,
                            if !curated {
                                "this command is not category: curated"
                            } else {
                                "this agent does not declare the runtime-model-extraction capability"
                            },
                        ),
                    ));
                }
            }
        }
        if let Some(body) = &n.do_ {
            check_node_agents(body, agents, app_exposes, app_id, out);
        }
    }
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
    fn implicit_ref_cycle_is_rejected() {
        // #208: `a` reads `{{ b.x }}` and `b` reads `{{ a.x }}` with NO explicit
        // connections. The derived edges form a cycle, which must be rejected at
        // validate (like an explicit cycle) — not silently pass and fail at run.
        let app: App = serde_yaml::from_str(
            r#"app: cyc
version: 0.0.1
description: x
nodes:
  - id: a
    agent: x
    command: emit
    config:
      v: '{{ b.x }}'
  - id: b
    agent: x
    command: emit
    config:
      v: '{{ a.x }}'
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app(&app);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_CYCLE"),
            "implicit ref cycle must be flagged E_APP_CYCLE; issues: {issues:?}"
        );
    }

    #[test]
    fn real_welded_to_tc_passes_validation() {
        let a = load_app("30-apps/_examples/welded-to-tc.app");
        let issues = validate_app(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    fn agent_with_status(status_line: &str) -> crate::manifest::loader::DiscoveredAgent {
        let yaml = format!(
            "agent: html-report\nversion: 0.1.0\ndescription: x\nstateful: false\n{status_line}\
             license: MIT\ntransport:\n  cli:\n    binary: aware-html-report\ncommands:\n  \
             render:\n    lifecycle: single\n    description: x\n"
        );
        crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(&yaml).unwrap(),
            root: std::path::PathBuf::from("."),
        }
    }

    #[test]
    fn rejects_node_referencing_planned_agent() {
        let agents = vec![agent_with_status("status: planned\n")];
        let app: App = serde_yaml::from_str(
            "app: uses-planned\nversion: 0.0.1\ndescription: |\n  uses planned agent\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n",
        )
        .unwrap();
        let issues = validate_app_agents(&app, &agents);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_AGENT_UNAVAILABLE"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn available_agent_passes_agent_validation() {
        // No `status:` → defaults to available → no E_APP_AGENT_UNAVAILABLE.
        let agents = vec![agent_with_status("")];
        let app: App = serde_yaml::from_str(
            "app: uses-avail\nversion: 0.0.1\ndescription: |\n  uses available agent\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n",
        )
        .unwrap();
        let issues = validate_app_agents(&app, &agents);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_AGENT_UNAVAILABLE"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn frozen_node_skips_planned_agent_gate() {
        // A frozen node never dispatches to its agent, so a planned (not-yet-runnable) agent must
        // NOT gate it — `frozen:` short-circuits the availability preflight (it needn't be installed).
        let agents = vec![agent_with_status("status: planned\n")];
        let app: App = serde_yaml::from_str(
            "app: frz\nversion: 0.0.1\ndescription: |\n  frozen planned\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n    frozen:\n      ok: true\n",
        )
        .unwrap();
        let issues = validate_app_agents(&app, &agents);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_AGENT_UNAVAILABLE"),
            "frozen node must skip the agent-availability gate; issues: {issues:?}"
        );
    }

    #[test]
    fn frozen_write_node_skips_safety_gate() {
        // A frozen node never writes (it emits a pinned value), so the write-mode safety contract
        // must not apply — a frozen write-mode node needs no `safety:` block.
        let writer = crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(
                "agent: writer\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
                 transport:\n  cli:\n    binary: aware-writer\ncommands:\n  apply:\n    lifecycle: single\n    mode: write\n    description: x\n",
            )
            .unwrap(),
            root: std::path::PathBuf::from("."),
        };
        let app: App = serde_yaml::from_str(
            "app: frzw\nversion: 0.0.1\ndescription: |\n  frozen write\n\
             requires: []\nnodes:\n  - id: w\n    agent: writer\n    command: apply\n    frozen:\n      done: true\n",
        )
        .unwrap();
        let issues = validate_app_safety(&app, &[writer]);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_WRITE_WITHOUT_SAFETY"),
            "frozen write-mode node must skip the safety gate; issues: {issues:?}"
        );
    }

    #[test]
    fn rejects_node_referencing_planned_command() {
        // The agent is available, but one command is `status: planned` (#199) — an
        // app using it must fail validate, while a sibling available command passes.
        let yaml = "agent: part\nversion: 0.1.0\ndescription: x\nstateful: false\n\
                    license: MIT\ntransport:\n  rest:\n    base: https://x\ncommands:\n  \
                    read-thing:\n    lifecycle: single\n    description: x\n  \
                    write-thing:\n    lifecycle: single\n    status: planned\n    description: x\n";
        let agents = vec![crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(yaml).unwrap(),
            root: std::path::PathBuf::from("."),
        }];

        let bad: App = serde_yaml::from_str(
            "app: uses-planned-cmd\nversion: 0.0.1\ndescription: x\nrequires: []\n\
             nodes:\n  - id: w\n    agent: part\n    command: write-thing\n",
        )
        .unwrap();
        let issues = validate_app_agents(&bad, &agents);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_COMMAND_UNAVAILABLE"),
            "planned command must be rejected: {issues:?}"
        );

        let ok: App = serde_yaml::from_str(
            "app: uses-avail-cmd\nversion: 0.0.1\ndescription: x\nrequires: []\n\
             nodes:\n  - id: r\n    agent: part\n    command: read-thing\n",
        )
        .unwrap();
        let issues = validate_app_agents(&ok, &agents);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_COMMAND_UNAVAILABLE"),
            "available command must pass: {issues:?}"
        );
    }

    fn one_node_app(agent: &str, command: &str) -> App {
        serde_yaml::from_str(&format!(
            "app: a\nversion: 0.0.1\ndescription: x\nrequires: []\nnodes:\n  - id: e\n    agent: {agent}\n    command: {command}\n"
        ))
        .unwrap()
    }

    #[test]
    fn admits_curated_capable_vision_extract() {
        // RFC #223: a curated `model-extraction` command whose agent declares the
        // capability is ADMITTED into the run path (no E_APP_RUNTIME_MODEL_FORBIDDEN).
        let agents = vec![discovered(
            "agent: vision\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             capabilities:\n  runtime-model-extraction: true\ntransport:\n  builtin: {}\n\
             commands:\n  extract:\n    lifecycle: single\n    category: curated\n    mode: read\n    \
             model-extraction: true\n    description: x\n",
        )];
        let issues = validate_app_agents(&one_node_app("vision", "extract"), &agents);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "curated+capable vision.extract must be admitted: {issues:?}"
        );
    }

    #[test]
    fn rejects_model_extraction_without_capability() {
        // Same curated command, but the agent does NOT declare the capability → forbidden.
        let agents = vec![discovered(
            "agent: vision\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  builtin: {}\ncommands:\n  extract:\n    lifecycle: single\n    \
             category: curated\n    mode: read\n    model-extraction: true\n    description: x\n",
        )];
        let issues = validate_app_agents(&one_node_app("vision", "extract"), &agents);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "model-extraction without the capability flag must be forbidden: {issues:?}"
        );
    }

    #[test]
    fn rejects_reflected_model_extraction_even_with_capability() {
        // A model-extraction command that is NOT curated (reflected) → forbidden even
        // with the capability flag, so no `aware build --from-*` output mints a reader.
        let agents = vec![discovered(
            "agent: rogue\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             provenance:\n  generated-by: reflected\ncapabilities:\n  runtime-model-extraction: true\n\
             transport:\n  builtin: {}\ncommands:\n  extract:\n    lifecycle: single\n    \
             category: reflected\n    mode: read\n    model-extraction: true\n    description: x\n",
        )];
        let issues = validate_app_agents(&one_node_app("rogue", "extract"), &agents);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "reflected model-extraction must be forbidden: {issues:?}"
        );
    }

    #[test]
    fn real_vision_manifest_loads_and_is_admitted() {
        // The shipped 20-agents/_core/vision manifest deserializes AND its extract node
        // passes the carve-out (curated + capability) — guards the manifest against drift.
        let a = load_agent("20-agents/_core/vision/manifest.yaml");
        let agents = vec![crate::manifest::loader::DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("."),
        }];
        let issues = validate_app_agents(&one_node_app("vision", "extract"), &agents);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "the real vision manifest must be admitted: {issues:?}"
        );
    }

    #[test]
    fn rejects_unrunnable_inline_kind_at_validate() {
        // kind: shape passes parse but the runtime only runs `predicate`; validate
        // must reject it so the failure surfaces before compile/lock (#160).
        let yaml = r#"
app: inline-shape
version: 0.0.1
description: |
  Inline shape node.
requires: []
nodes:
  - id: passthrough
    inline:
      kind: shape
      description: reshape a value
      code: "() => ({ ok: true })"
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(has_errors(&issues), "issues: {issues:?}");
        assert!(issues.iter().any(|i| i.code == "E_APP_INLINE_KIND"));
    }

    #[test]
    fn rejects_unrunnable_inline_kind_nested_in_for_each_body() {
        // A shape node inside a for-each `do:` body is flattened + run, so it must
        // be rejected at validate too — not just top-level nodes (#160 review).
        let yaml = r#"
app: inline-shape-body
version: 0.0.1
description: |
  for-each with an inline shape in its body.
requires: []
nodes:
  - id: loop
    for-each: '{{ items }}'
    do:
      - id: reshape
        inline:
          kind: shape
          description: reshape each item
          code: "() => ({ ok: true })"
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_INLINE_KIND"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn predicate_inline_kind_passes_validate() {
        let yaml = r#"
app: inline-pred
version: 0.0.1
description: |
  Inline predicate node.
requires: []
nodes:
  - id: gate
    inline:
      kind: predicate
      description: gate on type
      code: 'e.type == "Welded"'
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_INLINE_KIND"),
            "issues: {issues:?}"
        );
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

    /// Build a `DiscoveredAgent` for the safety-contract tests below.
    fn discovered(agent_yaml: &str) -> crate::manifest::loader::DiscoveredAgent {
        let a: Agent = serde_yaml::from_str(agent_yaml).unwrap();
        crate::manifest::loader::DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }
    }

    #[test]
    fn safety_check_rejects_write_node_without_safety_block() {
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: Apache-2.0
transport: { cli: { binary: aware-tekla } }
commands:
  insert:
    lifecycle: single
    category: curated
    description: Insert a connection.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: write-no-safety
version: 0.0.1
description: x
nodes:
  - id: t
    agent: tekla
    command: insert
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E_APP_WRITE_WITHOUT_SAFETY");
    }

    #[test]
    fn safety_check_passes_when_write_node_has_safety_block() {
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: Apache-2.0
transport: { cli: { binary: aware-tekla } }
commands:
  insert:
    lifecycle: single
    category: curated
    description: Insert a connection.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: write-with-safety
version: 0.0.1
description: x
nodes:
  - id: t
    agent: tekla
    command: insert
    safety:
      transaction-group: my-tx
      snapshot: true
      worksharing:
        check: true
        fail-if-other-user: true
      audit-stamp:
        uda-prefix: AWARE_
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
    }

    #[test]
    fn safety_check_skips_read_mode_nodes() {
        let agent = discovered(
            r#"
agent: ex
version: 1.0
description: x
stateful: false
license: Apache-2.0
transport: { cli: { binary: aware-ex } }
commands:
  list:
    lifecycle: single
    category: curated
    description: List items.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: read-only
version: 0.0.1
description: x
nodes:
  - id: l
    agent: ex
    command: list
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
    }

    #[test]
    fn safety_check_honors_explicit_mode_write_field() {
        let agent = discovered(
            r#"
agent: x
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-x } }
commands:
  unusual-name-not-matching-convention:
    lifecycle: single
    mode: write
    description: Writes despite the name not matching the convention.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: explicit-write
version: 0.0.1
description: x
nodes:
  - id: n
    agent: x
    command: unusual-name-not-matching-convention
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E_APP_WRITE_WITHOUT_SAFETY");
    }

    #[test]
    fn safety_check_honors_explicit_mode_read_override() {
        // A command that names itself like a write (.insert suffix) but
        // declares mode: read explicitly is treated as read-mode. Lets
        // curators override the naming-convention default.
        let agent = discovered(
            r#"
agent: x
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-x } }
commands:
  thing.insert:
    lifecycle: single
    mode: read
    description: Looks like a write but actually a read.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: misleading-name
version: 0.0.1
description: x
nodes:
  - id: n
    agent: x
    command: thing.insert
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
    }

    #[test]
    fn safety_check_passes_read_override_on_mode_overridable_exec() {
        // The #165 install repro: `exec` is declared write-mode but
        // `mode-overridable`. A node-level `mode: read` makes it read-mode, so
        // no `safety:` block is required and install is NOT refused.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: exec-mode-repro
version: 0.0.1
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    mode: read
    config:
      code: return new { ok = true };
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(
            issues.is_empty(),
            "read-only exec must install without safety: ; issues: {issues:?}"
        );
    }

    #[test]
    fn safety_check_still_requires_safety_on_unannotated_mode_overridable_exec() {
        // Without a node-level mode, mode-overridable exec keeps its
        // conservative write default — safety: is still required.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: exec-unannotated
version: 0.0.1
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    config:
      code: Model.CommitChanges();
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E_APP_WRITE_WITHOUT_SAFETY");
    }

    #[test]
    fn agents_check_rejects_conflicting_node_mode_on_non_overridable_command() {
        // A node-level `mode: read` on a genuinely-writing, non-overridable
        // command must be rejected loudly — never silently dropped (#165).
        // The check lives in `validate_app_agents` so it fires on every
        // agent-aware path (install, compile, inspect, validate, run, dry-run),
        // not just safety pre-flight.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  uda-set:
    lifecycle: single
    mode: write
    description: writes a UDA
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: sneaky-write
version: 0.0.1
description: x
nodes:
  - id: n
    agent: tekla
    command: uda-set
    mode: read
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_agents(&app, &[agent]);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_NODE_MODE_NOT_OVERRIDABLE"),
            "conflicting node mode on non-overridable command must be rejected; issues: {issues:?}"
        );
    }

    #[test]
    fn agents_check_rejects_conflicting_node_mode_in_nested_body() {
        // The conflict check recurses into `for-each`/`sweep` `do:` bodies —
        // a nested node cannot smuggle a dropped `mode:` past validation.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  uda-set:
    lifecycle: single
    mode: write
    description: writes a UDA
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: nested-sneaky
version: 0.0.1
description: x
nodes:
  - id: loop
    for-each: '{{ inputs.items }}'
    do:
      - id: inner
        agent: tekla
        command: uda-set
        mode: read
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_agents(&app, &[agent]);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_NODE_MODE_NOT_OVERRIDABLE"),
            "nested conflicting node mode must be rejected; issues: {issues:?}"
        );
    }

    #[test]
    fn agents_check_allows_redundant_matching_node_mode_on_non_overridable_command() {
        // Restating the manifest mode (no conflict) is redundant but allowed —
        // it must not error in either the agent-aware or safety validators.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  bolt-list:
    lifecycle: single
    mode: read
    description: reads a bolt schedule
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: redundant-mode
version: 0.0.1
description: x
nodes:
  - id: n
    agent: tekla
    command: bolt-list
    mode: read
connections: []
requires: []
"#,
        )
        .unwrap();

        assert!(
            validate_app_agents(&app, std::slice::from_ref(&agent)).is_empty(),
            "redundant matching mode must not error in validate_app_agents"
        );
        assert!(
            validate_app_safety(&app, &[agent]).is_empty(),
            "redundant matching read mode must not error in validate_app_safety"
        );
    }

    // ── #178: exposes-as-agent validation ──────────────────────────────────

    #[test]
    fn exposes_as_agent_without_commands_is_error() {
        let app: App = serde_yaml::from_str(
            r#"
app: empty-expose
version: 0.0.1
description: x
exposes-as-agent: true
nodes:
  - id: n
    inline:
      kind: predicate
      description: pass
      code: 'true'
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app(&app);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_EXPOSES_NO_COMMANDS"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn exposes_as_agent_with_commands_passes() {
        let app: App = serde_yaml::from_str(
            r#"
app: ok-expose
version: 0.0.1
description: x
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: n
    inline:
      kind: predicate
      description: pass
      code: 'true'
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app(&app);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_EXPOSES_NO_COMMANDS"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn exposed_app_composing_another_exposed_app_is_rejected() {
        // `inner` is an app-backed (synthesized) agent — it declares an `app`
        // transport. A `mid` app that is ITSELF exposes-as-agent and composes
        // `inner` violates the v0 no-recursion rule.
        let inner = discovered(
            r#"
agent: inner
version: 1.0
description: x
stateful: false
license: app-exposed
transport:
  app:
    backed-by: inner
commands:
  run:
    lifecycle: single
    description: x
"#,
        );
        let mid: App = serde_yaml::from_str(
            r#"
app: mid
version: 0.0.1
description: x
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: call-inner
    agent: inner
    command: run
connections: []
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app_agents(&mid, std::slice::from_ref(&inner));
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_EXPOSED_COMPOSES_EXPOSED"),
            "an exposed app composing another exposed app must be rejected; issues: {issues:?}"
        );

        // A NON-exposed app composing the same app-backed agent is allowed
        // (one-level composition by a normal consumer).
        let outer: App = serde_yaml::from_str(
            r#"
app: outer
version: 0.0.1
description: x
nodes:
  - id: call-inner
    agent: inner
    command: run
connections: []
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app_agents(&outer, &[inner]);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_EXPOSED_COMPOSES_EXPOSED"),
            "a normal consumer composing an exposed app must be allowed; issues: {issues:?}"
        );
    }

    #[test]
    fn exposed_app_referencing_itself_is_rejected_even_on_first_install() {
        // An exposes-as-agent app whose node references its OWN id recurses into
        // itself. On first install the synthesized agent does not exist yet, so
        // this must be caught from the app id alone — not only via the catalogue.
        let selfie: App = serde_yaml::from_str(
            r#"
app: selfie
version: 0.0.1
description: x
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: loop-self
    agent: selfie
    command: run
connections: []
requires: []
"#,
        )
        .unwrap();
        // Empty catalogue — the synth agent for `selfie` is not yet installed.
        let issues = validate_app_agents(&selfie, &[]);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_EXPOSED_COMPOSES_EXPOSED"),
            "self-referential exposed app must be rejected at first install; issues: {issues:?}"
        );
    }
}
