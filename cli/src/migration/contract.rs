//! The executable-contract diff (#628 plan §2): between two stored packages of
//! one agent, what changes in what a RUN of a given workflow would hand the
//! executor — and only that.
//!
//! An agent package is mostly not executable code: a CLI agent's bridge binary
//! belongs to the CLI version, and REST/builtin agents are executed by AWARE's
//! own code from manifest fields. So a pin's executable contract is
//!
//! 1. the **run-relevant projection of the raw manifest** — compared on the raw
//!    YAML value, so a key this CLI does not know is compared too (fail safe),
//!    minus an ignore-list of keys a run never reads ([`IGNORED_AGENT_KEYS`]);
//! 2. the **called commands** — each command a non-frozen node of the workflow
//!    calls, compared whole minus its top-level `description`;
//! 3. the **executable files** — every package file outside a documentation
//!    allowlist ([`is_doc_file`]), by sha256;
//! 4. the **executor identity** — the resolved bridge program and its sha256
//!    for a CLI agent, `aware-cli <version>` for REST/builtin, n/a for an
//!    app-backed agent;
//! 5. the **plan changes** — compiled-node fields that differ between the base
//!    and candidate locks (`mode`, `output-schema`, `runtime-model`, `model-pin`).
//!
//! `unchanged` ⇔ (1), (2), (3), (4) and (5) are all unchanged. Ignored
//! differences are reported, never hidden: `probe-changed` and `ignored`.
//!
//! The ignore-list is only sound while the run path does not read those keys;
//! `run_path_never_reads_a_contract_ignored_field` scans `runtime/` to keep it so.

// Wired into `aware app migrate plan` by #628 PR2.
#![cfg_attr(not(test), allow(dead_code))]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::app_lock::LockFile;
use crate::error::AwareError;
use crate::manifest::{Agent, App};

/// The output format tag of one diff.
pub const CONTRACT_DIFF_FORMAT: &str = "aware.contract-diff/v1";

/// Top-level manifest keys `aware app run` never reads, so a change to them
/// cannot change what a run does. `probe` is run only by `aware agent probe`
/// (`runtime/probe.rs`); its change is reported as `probe-changed` and not
/// counted. `commands` is compared through the called-commands projection.
pub const IGNORED_AGENT_KEYS: &[&str] = &[
    "version",
    "display-name",
    "description",
    "keywords",
    "homepage",
    "vendor",
    "license",
    "provenance",
    "skills",
    "probe",
];

/// Documentation files: never read by a run. Everything else in the package is
/// treated as executable (fail safe), `atoms/` included. `manifest.yaml` is
/// covered by the projections above and skipped here.
pub fn is_doc_file(relative: &str) -> bool {
    // `relative` is `/`-separated from the package root, so `starts_with`
    // already confines README*/LICENSE* to the root level.
    relative.starts_with("skills/")
        || (relative.starts_with("commands/") && relative.ends_with(".md"))
        || relative == "CHANGELOG.md"
        || relative.starts_with("README")
        || relative.starts_with("LICENSE")
        || crate::install::integrity::is_install_metadata(relative)
}

/// One side of a diff: a verified store package.
pub struct PackageSide<'a> {
    pub root: &'a Path,
    pub version: &'a str,
    pub digest: &'a str,
}

/// What executes a pin's commands at evaluation time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Executor {
    /// A CLI agent's bridge program, as `aware app run` would resolve it now.
    Cli {
        binary: String,
        program: String,
        /// The program was a relative path, resolved against the working
        /// directory this identity was computed in — a run started elsewhere
        /// would execute a different file.
        #[serde(rename = "relative-to-cwd", skip_serializing_if = "std::ops::Not::not")]
        relative_to_cwd: bool,
        /// `None` when the program cannot be found or read; `detail` says why.
        sha256: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        detail: Option<String>,
    },
    /// REST / builtin: AWARE's own code executes the manifest.
    AwareCli { version: String },
    /// App-backed: the backing app's agents are the executors (its own pins).
    App { backed_by: String },
    /// No dispatchable transport.
    None,
}

/// The executor of `manifest`, resolved exactly as dispatch would now.
pub fn executor_identity(manifest: &Agent) -> Executor {
    use crate::runtime::invoker::{TransportKind, dispatch_transport};
    match dispatch_transport(&manifest.transport) {
        Some(TransportKind::Cli) => {
            let binary = manifest
                .transport
                .cli
                .as_ref()
                .map(|c| c.binary.clone())
                .unwrap_or_default();
            // The same two steps dispatch takes: the program `spawn_cli` passes
            // to `Command::new`, then the file `Command::new` finds for it.
            let program = crate::runtime::invoker::cli_program(&binary);
            match crate::runtime::invoker::spawn_target(&program) {
                Some(target) => {
                    let (sha256, detail) = match std::fs::read(&target.path) {
                        Ok(bytes) => (Some(format!("sha256:{:x}", Sha256::digest(&bytes))), None),
                        Err(error) => (None, Some(format!("cannot read the program: {error}"))),
                    };
                    Executor::Cli {
                        binary,
                        program: target.path.display().to_string(),
                        relative_to_cwd: target.relative_to_cwd,
                        sha256,
                        detail,
                    }
                }
                None => Executor::Cli {
                    binary,
                    program: program.display().to_string(),
                    relative_to_cwd: false,
                    sha256: None,
                    detail: Some("the program is not installed".into()),
                },
            }
        }
        Some(TransportKind::Rest | TransportKind::Builtin) => Executor::AwareCli {
            version: crate::validate::CURRENT_CLI_VERSION.to_string(),
        },
        Some(TransportKind::App) => Executor::App {
            backed_by: manifest
                .transport
                .app
                .as_ref()
                .map(|t| t.backed_by.clone())
                .unwrap_or_default(),
        },
        None => Executor::None,
    }
}

/// A pin as the diff names it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PinRef {
    pub version: String,
    pub digest: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct AgentLevel {
    /// Top-level manifest keys (outside the ignore-list) that were added,
    /// removed or changed; plus `executor` when the executor identity differs.
    pub changed: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandDiff {
    pub command: String,
    /// The workflow nodes that call it.
    pub nodes: Vec<String>,
    pub unchanged: bool,
    /// The command-entry keys that differ (`added` / `removed` for a command
    /// present on one side only).
    pub changes: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct FileDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
}

impl FileDiff {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}

/// One compiled-node field that differs between the base and candidate locks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlanChange {
    pub node: String,
    /// `mode` | `output-schema` | `runtime-model` | `model-pin` | `node`.
    pub field: String,
    pub from: serde_json::Value,
    pub to: serde_json::Value,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ignored {
    pub doc_files: Vec<String>,
    pub commands_not_called: Vec<String>,
}

/// `aware.contract-diff/v1` for one moved agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractDiff {
    pub format: &'static str,
    pub agent: String,
    pub from: PinRef,
    pub to: PinRef,
    pub unchanged: bool,
    pub agent_level: AgentLevel,
    pub commands: Vec<CommandDiff>,
    pub executable_files: FileDiff,
    pub probe_changed: bool,
    /// The executor at evaluation time (the new pin's). A different executor
    /// for the old pin is recorded as agent-level `executor`.
    pub executor: Executor,
    pub plan_changes: Vec<PlanChange>,
    pub ignored: Ignored,
}

/// Everything [`diff`] compares.
pub struct DiffInput<'a> {
    pub agent: &'a str,
    pub old: PackageSide<'a>,
    pub new: PackageSide<'a>,
    /// Called command → the node ids calling it ([`called_commands`]).
    pub called: &'a BTreeMap<String, Vec<String>>,
    /// From [`plan_changes`] (empty when no candidate lock exists yet).
    pub plan_changes: Vec<PlanChange>,
}

/// The contract diff, with the executor resolved as dispatch would now.
pub fn diff(input: DiffInput<'_>) -> Result<ContractDiff, AwareError> {
    diff_with(input, executor_identity)
}

/// [`diff`] with an injected executor resolver (tests pin the identity).
pub fn diff_with(
    input: DiffInput<'_>,
    executor_of: impl Fn(&Agent) -> Executor,
) -> Result<ContractDiff, AwareError> {
    let old_raw = raw_manifest(input.old.root)?;
    let new_raw = raw_manifest(input.new.root)?;
    let old_typed = crate::manifest::loader::load_agent(&input.old.root.join("manifest.yaml"))?;
    let new_typed = crate::manifest::loader::load_agent(&input.new.root.join("manifest.yaml"))?;

    // (1) agent level, on the raw value.
    let mut agent_level = AgentLevel::default();
    let old_map = top_level(&old_raw);
    let new_map = top_level(&new_raw);
    let keys: BTreeSet<&String> = old_map.keys().chain(new_map.keys()).collect();
    for key in keys {
        if key == "commands" || IGNORED_AGENT_KEYS.contains(&key.as_str()) {
            continue;
        }
        if old_map.get(key) != new_map.get(key) {
            agent_level.changed.push(key.clone());
        }
    }
    let probe_changed = old_map.get("probe") != new_map.get("probe");

    // (4) executor identity, evaluated now for both pins.
    let executor = executor_of(&new_typed);
    if executor_of(&old_typed) != executor {
        agent_level.changed.push("executor".into());
    }

    // (2) called commands; uncalled differences are reported as ignored.
    let old_cmds = commands_of(&old_raw);
    let new_cmds = commands_of(&new_raw);
    let mut commands = Vec::new();
    for (command, nodes) in input.called {
        let changes = command_changes(old_cmds.get(command), new_cmds.get(command));
        commands.push(CommandDiff {
            command: command.clone(),
            nodes: nodes.clone(),
            unchanged: changes.is_empty(),
            changes,
        });
    }
    let all_cmds: BTreeSet<&String> = old_cmds.keys().chain(new_cmds.keys()).collect();
    let mut ignored = Ignored::default();
    for command in all_cmds {
        if input.called.contains_key(command) {
            continue;
        }
        if !command_changes(old_cmds.get(command), new_cmds.get(command)).is_empty() {
            ignored.commands_not_called.push(command.clone());
        }
    }

    // (3) executable files.
    let old_files = file_hashes(input.old.root)?;
    let new_files = file_hashes(input.new.root)?;
    let mut executable_files = FileDiff::default();
    let names: BTreeSet<&String> = old_files.keys().chain(new_files.keys()).collect();
    for name in names {
        let slot = match (old_files.get(name), new_files.get(name)) {
            (Some(a), Some(b)) if a == b => continue,
            (Some(_), Some(_)) => &mut executable_files.changed,
            (None, Some(_)) => &mut executable_files.added,
            (Some(_), None) => &mut executable_files.removed,
            (None, None) => continue,
        };
        if is_doc_file(name) {
            ignored.doc_files.push(name.clone());
        } else {
            slot.push(name.clone());
        }
    }

    let unchanged = agent_level.changed.is_empty()
        && commands.iter().all(|c| c.unchanged)
        && executable_files.is_empty()
        && input.plan_changes.is_empty();
    Ok(ContractDiff {
        format: CONTRACT_DIFF_FORMAT,
        agent: input.agent.to_string(),
        from: PinRef {
            version: input.old.version.to_string(),
            digest: input.old.digest.to_string(),
        },
        to: PinRef {
            version: input.new.version.to_string(),
            digest: input.new.digest.to_string(),
        },
        unchanged,
        agent_level,
        commands,
        executable_files,
        probe_changed,
        executor,
        plan_changes: input.plan_changes,
        ignored,
    })
}

fn raw_manifest(root: &Path) -> Result<serde_yaml::Value, AwareError> {
    let path = root.join("manifest.yaml");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    Ok(serde_yaml::from_str(&text)?)
}

/// The top-level mapping as canonical JSON per key. A key that is not a string
/// is kept under its YAML spelling, so it is still compared.
fn top_level(raw: &serde_yaml::Value) -> BTreeMap<String, String> {
    raw.as_mapping()
        .map(|m| m.iter().map(|(k, v)| (key_name(k), canonical(v))).collect())
        .unwrap_or_default()
}

fn commands_of(raw: &serde_yaml::Value) -> BTreeMap<String, serde_yaml::Value> {
    raw.get("commands")
        .and_then(serde_yaml::Value::as_mapping)
        .map(|m| m.iter().map(|(k, v)| (key_name(k), v.clone())).collect())
        .unwrap_or_default()
}

/// The keys of one command entry that differ, ignoring its top-level
/// `description`. Non-mapping entries compare whole.
fn command_changes(
    old: Option<&serde_yaml::Value>,
    new: Option<&serde_yaml::Value>,
) -> Vec<String> {
    match (old, new) {
        (None, None) => Vec::new(),
        (None, Some(_)) => vec!["added".into()],
        (Some(_), None) => vec!["removed".into()],
        (Some(a), Some(b)) => match (a.as_mapping(), b.as_mapping()) {
            (Some(a), Some(b)) => {
                let a: BTreeMap<String, String> =
                    a.iter().map(|(k, v)| (key_name(k), canonical(v))).collect();
                let b: BTreeMap<String, String> =
                    b.iter().map(|(k, v)| (key_name(k), canonical(v))).collect();
                let keys: BTreeSet<&String> = a.keys().chain(b.keys()).collect();
                keys.into_iter()
                    .filter(|k| *k != "description" && a.get(*k) != b.get(*k))
                    .cloned()
                    .collect()
            }
            _ if canonical(a) == canonical(b) => Vec::new(),
            _ => vec!["(entry)".into()],
        },
    }
}

fn key_name(key: &serde_yaml::Value) -> String {
    match key {
        serde_yaml::Value::String(s) => s.clone(),
        other => canonical(other),
    }
}

/// Canonical JSON text: object keys sorted at every depth (`serde_json` here
/// preserves insertion order, so the sort is explicit). A value JSON cannot
/// express falls back to its YAML text — still a stable, comparable spelling.
pub fn canonical(value: &serde_yaml::Value) -> String {
    match serde_json::to_value(value) {
        Ok(json) => sorted(json).to_string(),
        Err(_) => serde_yaml::to_string(value).unwrap_or_default(),
    }
}

fn sorted(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let ordered: BTreeMap<String, serde_json::Value> =
                map.into_iter().map(|(k, v)| (k, sorted(v))).collect();
            serde_json::Value::Object(ordered.into_iter().collect())
        }
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.into_iter().map(sorted).collect())
        }
        other => other,
    }
}

/// sha256 of every package file except `manifest.yaml` (doc files included —
/// the caller sorts them into `ignored`).
fn file_hashes(root: &Path) -> Result<BTreeMap<String, String>, AwareError> {
    let mut out = BTreeMap::new();
    for (relative, path) in crate::fs::plain_files_under(root, "agent package")? {
        if relative == "manifest.yaml" {
            continue;
        }
        let bytes = std::fs::read(&path)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
        out.insert(relative, format!("{:x}", Sha256::digest(&bytes)));
    }
    Ok(out)
}

/// Command → the scoped ids of the non-frozen nodes of `app` that call it on
/// `agent`. Frozen subtrees never dispatch, so their commands are not called.
pub fn called_commands(app: &App, agent: &str) -> BTreeMap<String, Vec<String>> {
    fn walk(
        nodes: &[crate::manifest::app::Node],
        prefix: Option<&str>,
        agent: &str,
        out: &mut BTreeMap<String, Vec<String>>,
    ) {
        for node in nodes {
            if node.frozen.is_some() {
                continue;
            }
            let scoped = match prefix {
                Some(p) => format!("{p}.{}", node.id),
                None => node.id.clone(),
            };
            if node.agent.as_deref() == Some(agent) {
                out.entry(node.command.clone().unwrap_or_default())
                    .or_default()
                    .push(scoped.clone());
            }
            if let Some(body) = &node.do_ {
                walk(body, Some(&scoped), agent, out);
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(&app.nodes, None, agent, &mut out);
    out
}

/// Compiled-node differences on `agent`'s nodes between the base lock and a
/// candidate lock, matched by node id.
pub fn plan_changes(base: &LockFile, candidate: &LockFile, agent: &str) -> Vec<PlanChange> {
    let pick = |lock: &LockFile| -> BTreeMap<String, serde_json::Value> {
        lock.nodes
            .iter()
            .filter(|n| n.agent.as_deref() == Some(agent))
            .map(|n| {
                let fields = serde_json::json!({
                    "mode": n.mode,
                    "output-schema": serde_json::to_value(&n.output_schema)
                        .map(sorted)
                        .unwrap_or_else(|_| serde_json::Value::String(
                            n.output_schema.as_ref().map(canonical).unwrap_or_default()
                        )),
                    "runtime-model": n.runtime_model,
                    "model-pin": n.model_pin,
                });
                (n.id.clone(), fields)
            })
            .collect()
    };
    let before = pick(base);
    let after = pick(candidate);
    let ids: BTreeSet<&String> = before.keys().chain(after.keys()).collect();
    let mut out = Vec::new();
    for id in ids {
        match (before.get(id), after.get(id)) {
            (Some(a), Some(b)) => {
                for field in ["mode", "output-schema", "runtime-model", "model-pin"] {
                    if a[field] != b[field] {
                        out.push(PlanChange {
                            node: id.clone(),
                            field: field.into(),
                            from: a[field].clone(),
                            to: b[field].clone(),
                        });
                    }
                }
            }
            (a, b) => out.push(PlanChange {
                node: id.clone(),
                field: "node".into(),
                from: serde_json::json!(a.is_some()),
                to: serde_json::json!(b.is_some()),
            }),
        }
    }
    out
}

#[cfg(test)]
mod tests;
