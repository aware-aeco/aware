//! Lockfile compiler — produces `<app>.lock` sidecar.
//!
//! Per `10-core/app-spec.md § Lockfile sidecar` (v0.24).
//!
//! The lockfile is the **deterministic, type-resolved, agent-version-pinned**
//! compiled plan that engineers read instead of the AI's prose source. It's a
//! YAML sidecar emitted next to the source app file by `aware app compile`.
//!
//! Distinct from the install-time `lockfile.yaml` (`cli/src/lockfile.rs`),
//! which pins agent versions inside `~/.aware/apps/<app>/`. This module is
//! the engineer-facing compile artifact.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::agent::Mode;
use crate::manifest::loader::{DiscoveredAgent, discover_agents};
use crate::paths::Paths;

/// The lockfile schema. Serialized as YAML to `<app>.lock`.
#[derive(Debug, Deserialize, Serialize)]
pub struct LockFile {
    /// SHA-256 of the source app file (UTF-8 bytes).
    #[serde(rename = "source-hash")]
    pub source_hash: String,

    /// ISO 8601 of when the lockfile was compiled.
    #[serde(rename = "compiled-at")]
    pub compiled_at: String,

    /// AWARE CLI version that produced this lockfile.
    #[serde(rename = "compiler-version")]
    pub compiler_version: String,

    /// App id (from the source).
    pub app: String,

    /// App version (from the source).
    pub version: String,

    /// Resolved agent id → pinned version. One entry per agent referenced
    /// by any node.
    #[serde(rename = "agent-pins")]
    pub agent_pins: BTreeMap<String, String>,

    /// Compiled nodes — every template that can be resolved at compile
    /// time IS resolved; runtime expressions are tagged with the
    /// `{{ runtime: ... }}` prefix to make their dynamic nature explicit.
    pub nodes: Vec<CompiledNode>,

    /// App-level `schedule:` block (if present), passed through verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<serde_yaml::Value>,

    /// App-level `engineering:` block (if present), passed through verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engineering: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompiledNode {
    pub id: String,

    /// One of: `agent`, `inline`, `assert`, `for-each`, `compare`,
    /// `sweep`, `approve`, `snapshot`, `model-lock`, `unknown`.
    pub kind: String,

    /// Agent id (for `agent` kind only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,

    /// Command name on the agent (for `agent` kind only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Read or write — for safety-contract enforcement at run time.
    /// Always populated for `agent` kind; `read` for inline-glue +
    /// pure-read substrate primitives; `write` for the rest.
    pub mode: String,

    /// Safety contract block, if declared on the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety: Option<serde_yaml::Value>,

    /// Compile-time-resolved inputs. Static values resolved; runtime
    /// references kept verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_yaml::Value>,

    /// Schema of the agent command's outputs (if `kind: agent`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output-schema")]
    pub output_schema: Option<serde_yaml::Value>,

    /// Notes captured at compile time, each carrying a machine-readable
    /// `kind` (info / warn / error) so consumers can render by severity
    /// without string-matching the prose (#170). Serialized as a list of
    /// `{ kind, text }` maps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<CompileNote>,

    /// RFC #223: `true` when this node resolves to a curated `model-extraction`
    /// command (`vision.extract`) — it calls a model at run time. Lets the lock be
    /// the single source of truth that a model runs here, so `aware app run`, the
    /// Glass Box, and a front door can render it honestly to the approver. Absent
    /// (false) for every ordinary deterministic node.
    #[serde(rename = "runtime-model", default, skip_serializing_if = "is_false")]
    pub runtime_model: bool,

    /// RFC #223 §5.3: the pinned model id the lock was approved against (from the
    /// node's `model` input). A model swap changes this, re-invalidating the
    /// approval the same way a source-hash change does. None for non-extraction nodes.
    #[serde(rename = "model-pin", default, skip_serializing_if = "Option::is_none")]
    pub model_pin: Option<String>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(b: &bool) -> bool {
    !*b
}

/// Severity of a compile-time [`CompileNote`]. Consumers (the CLI, the lock
/// audit, floless.app) render by `kind` — `info` quiet/collapsible, `warn` /
/// `error` prominent — and stay correct across note-wording changes (#170).
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NoteKind {
    /// Benign provenance / FYI — e.g. "the compiler trusted the node-level
    /// `mode:` for a command whose mode it can't infer" (the `exec` case, #165).
    Info,
    /// Something the author should look at — e.g. a silent write-mode
    /// fallback, an uninstalled agent, or a dangling input reference.
    Warn,
    /// A condition that should block the run.
    ///
    /// No compile path constructs this yet, but `10-core/app-spec.md` publishes
    /// `info | warn | error` as the lockfile's note-kind contract, so the
    /// variant stays until the spec drops it.
    #[allow(dead_code)]
    Error,
}

/// A single compile-time note: a severity [`kind`](NoteKind) plus its prose.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CompileNote {
    pub kind: NoteKind,
    pub text: String,
}

impl CompileNote {
    /// Benign provenance / FYI note (`kind: info`).
    pub fn info(text: impl Into<String>) -> Self {
        Self {
            kind: NoteKind::Info,
            text: text.into(),
        }
    }

    /// Actionable warning (`kind: warn`).
    pub fn warn(text: impl Into<String>) -> Self {
        Self {
            kind: NoteKind::Warn,
            text: text.into(),
        }
    }

    /// Run-blocking error (`kind: error`). Unused today — kept in step with
    /// [`NoteKind::Error`], which the app-spec still publishes.
    #[allow(dead_code)]
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            kind: NoteKind::Error,
            text: text.into(),
        }
    }
}

fn hash_source(source_path: &Path) -> Result<String, AwareError> {
    let source_bytes = std::fs::read(source_path)
        .map_err(|e| AwareError::Internal(format!("read {}: {e}", source_path.display())))?;
    Ok(hash_source_bytes(&source_bytes))
}

fn hash_source_bytes(source_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source_bytes);
    format!("sha256:{:x}", hasher.finalize())
}

/// Load an installed app and enforce its compiled approval over the same bytes.
///
/// The lock is named by the source app id and its `source-hash` covers the raw
/// source bytes. Missing, unreadable, malformed, or stale approval artifacts
/// are validation failures. Reading, parsing, and hashing one buffer ensures the
/// parsed app is exactly the artifact the lock approves even if the file is
/// replaced concurrently.
pub fn load_approved_app(source_path: &Path) -> Result<App, AwareError> {
    let source_text = std::fs::read_to_string(source_path).map_err(|error| {
        std::io::Error::new(error.kind(), format!("{}: {error}", source_path.display()))
    })?;
    let app: App = serde_yaml::from_str(&source_text)
        .map_err(|error| AwareError::Validation(format!("{}: {error}", source_path.display())))?;
    let source_dir = source_path
        .parent()
        .ok_or_else(|| AwareError::Internal("source path has no parent".into()))?;
    let lock_path = source_dir.join(format!("{}.lock", app.app));
    let lock_text = match std::fs::read_to_string(&lock_path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(AwareError::Validation(format!(
                "[E_APP_LOCK_MISSING] app {} has no compiled approval at {}; run `aware app compile {}` first",
                app.app,
                lock_path.display(),
                source_path.display()
            )));
        }
        Err(error) => {
            return Err(AwareError::Validation(format!(
                "[E_APP_LOCK_INVALID] cannot read compiled approval {}: {error}; run `aware app compile {}` again",
                lock_path.display(),
                source_path.display()
            )));
        }
    };
    let lock: LockFile = serde_yaml::from_str(&lock_text).map_err(|error| {
        AwareError::Validation(format!(
            "[E_APP_LOCK_INVALID] compiled approval {} is invalid: {error}; run `aware app compile {}` again",
            lock_path.display(),
            source_path.display()
        ))
    })?;
    let current_hash = hash_source_bytes(source_text.as_bytes());
    if lock.source_hash != current_hash {
        return Err(AwareError::Validation(format!(
            "[E_APP_LOCK_STALE] compiled approval {} does not match the installed source (approved {}, current {}); run `aware app compile {}` again",
            lock_path.display(),
            lock.source_hash,
            current_hash,
            source_path.display()
        )));
    }
    Ok(app)
}

/// Compile a parsed app + the installed agent catalogue into a lockfile.
///
/// The lockfile is *not* written to disk here — callers (typically
/// `aware app compile`) handle the write.
pub fn compile(
    app: &App,
    agents: &[DiscoveredAgent],
    source_path: &Path,
) -> Result<LockFile, AwareError> {
    let source_hash = hash_source(source_path)?;

    // Flatten the node tree: top-level nodes plus the bodies of `do:`-bearing
    // primitives (for-each / sweep), so inner nodes are pinned, compiled, and
    // ref-checked rather than silently ignored (#117 finding #3). Body nodes
    // carry a scoped id (`parent.child`) so the flat lockfile stays unambiguous.
    let mut flat: Vec<FlatNode> = Vec::new();
    flatten_nodes(&app.nodes, None, &[], &mut flat);

    // Pin every agent referenced by any node (incl. `do:` bodies).
    let mut agent_pins: BTreeMap<String, String> = BTreeMap::new();
    for (node, _, _, _) in &flat {
        if let Some(aid) = &node.agent
            && let Some(d) = agents.iter().find(|d| d.manifest.agent == *aid)
        {
            agent_pins.insert(aid.clone(), d.manifest.version.clone());
        }
    }

    // Compile each node (flattened — `do:` bodies included), labelling body
    // nodes with their scoped id in the lockfile.
    let mut nodes: Vec<CompiledNode> = Vec::new();
    for (node, scoped_id, _, _) in &flat {
        let mut cn = compile_node(node, agents)?;
        cn.id = scoped_id.clone();
        nodes.push(cn);
    }

    // Second pass — compile-time reference checking. Build scoped-id → known
    // output field set (None when the schema can't be resolved: inline glue,
    // primitives, or an uninstalled agent), then verify every
    // `{{ <node>.<field> }}` reference in a node's inputs points at a real
    // field on the referenced node. A miss becomes a compile note (same
    // channel as command-not-found), turning a silent runtime failure into a
    // fixable compile-time signal — the deterministic plan validation that
    // makes decalog #9 usable in practice.
    //
    // The field set is keyed by SCOPED id (`parent.child`) so every node — top
    // level and `do:`-body-local — is addressable. A reference is resolved with
    // lexical scoping: `resolve_scope` walks the enclosing `do:` scopes from the
    // referrer outward (innermost body → … → top level) and validates against the
    // first node that matches, so a body-local id correctly shadows a same-named
    // outer/top-level node at any nesting depth. `flat`/`nodes` are index-parallel.
    let mut field_sets: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();
    for compiled in nodes.iter() {
        field_sets.insert(
            compiled.id.clone(),
            output_field_set(compiled.output_schema.as_ref()),
        );
    }
    for (i, entry) in flat.iter().enumerate() {
        let (node, scoped_id, iter_vars) = (entry.0, &entry.1, &entry.3);
        // This node's own scope prefix (`a.b.c` → `a.b`; `None` for top level).
        // References inside it are resolved starting one level out from here.
        let scope_prefix = scoped_id.rsplit_once('.').map(|(p, _)| p);
        let local_id = node.id.as_str();
        let mut refs: Vec<(String, String)> = Vec::new();
        // Scan the COMPILED inputs (config: + inputs: already merged by
        // `Node::merged_params`), not the raw keys — so an `inputs:` value that
        // overrode a `config:` value on the same key isn't double-checked
        // against the discarded `config:` ref (Codex #117-3).
        if let Some(compiled_inputs) = nodes[i].inputs.as_ref() {
            collect_refs(compiled_inputs, &mut refs);
        }
        // The `for-each` source expression (e.g. `{{ drawings.rows }}`) is also a
        // reference into an upstream node's output — check it too.
        if let Some(expr) = &node.for_each {
            collect_refs(&serde_yaml::Value::String(expr.clone()), &mut refs);
        }
        refs.sort();
        refs.dedup();
        for (nid, field) in refs {
            if nid == local_id {
                continue; // self-reference — skip
            }
            // Any enclosing primitive's per-iteration variable (`item` for
            // for-each, the `var:` name for sweep) is a runtime prefix inside the
            // body, not a node ref. Skip it even if a node shares the name —
            // scoped to this body's ancestors only (Codex #117-3).
            if iter_vars.iter().any(|v| v == &nid) {
                continue;
            }
            // Resolve `nid` through the lexical scope chain (innermost enclosing
            // `do:` body → … → top level). The first matching node wins, so a
            // body-local id shadows a same-named outer/top-level node at any depth.
            // Unresolved => a runtime prefix (inputs/secrets/run/now/ctx) or
            // unknown id — neither is flagged.
            let Some(target) = resolve_scope(&nid, scope_prefix, &field_sets) else {
                continue;
            };
            // Only flag when the resolved node's output schema is known and the
            // field is genuinely absent. Schema-less targets (inline/primitive/
            // uninstalled) resolve but carry `None`, so they're skipped.
            if let Some(Some(fields)) = field_sets.get(&target)
                && !fields.contains(&field)
            {
                let available = if fields.is_empty() {
                    "none".to_string()
                } else {
                    fields.iter().cloned().collect::<Vec<_>>().join(", ")
                };
                nodes[i].notes.push(CompileNote::warn(format!(
                    "input references {{{{ {nid}.{field} }}}} but node {nid:?} has no output field {field:?} (available: {available})"
                )));
            }
        }
    }

    let schedule = app
        .schedule
        .as_ref()
        .and_then(|s| serde_yaml::to_value(s).ok());
    let engineering = app
        .engineering
        .as_ref()
        .and_then(|e| serde_yaml::to_value(e).ok());

    Ok(LockFile {
        source_hash,
        compiled_at: chrono::Utc::now().to_rfc3339(),
        compiler_version: env!("CARGO_PKG_VERSION").to_string(),
        app: app.app.clone(),
        version: app.version.clone(),
        agent_pins,
        nodes,
        schedule,
        engineering,
    })
}

/// One entry per node in a depth-first flatten of the node tree: the node, its
/// **scoped lockfile id** (parent ids joined by `.`, so a `do:`-body node
/// becomes `parent.child`), and whether it is a genuine top-level node. Body
/// ids are scoped + reusable, so the scoped id keeps the flat lockfile node
/// list unambiguous and lets ref-checking treat only top-level ids as global.
/// A node in the flattened tree: `(node, scoped_id, is_top, iter_var)`.
/// `iter_vars` is the set of per-step runtime prefixes reserved by ALL enclosing
/// `do:`-bearing primitives — the literal `item` for each `for-each`, the literal
/// `var` for each `sweep` (per app-spec § Substrate primitives) — accumulated
/// outermost-to-innermost. Empty for top-level nodes. Each is a runtime prefix,
/// not a node ref, so the ref checker skips it within scope. Nested bodies inherit
/// their ancestors' prefixes (a `sweep` inside a `for-each` keeps `item` reserved
/// alongside `var`).
type FlatNode<'a> = (&'a crate::manifest::app::Node, String, bool, Vec<String>);

fn flatten_nodes<'a>(
    nodes: &'a [crate::manifest::app::Node],
    prefix: Option<&str>,
    iter_vars: &[String],
    out: &mut Vec<FlatNode<'a>>,
) {
    for n in nodes {
        let scoped_id = match prefix {
            Some(p) => format!("{p}.{}", n.id),
            None => n.id.clone(),
        };
        out.push((n, scoped_id.clone(), prefix.is_none(), iter_vars.to_vec()));
        if let Some(body) = &n.do_ {
            // The body adds THIS node's per-step runtime prefix to the inherited
            // set. Per app-spec § Substrate primitives, `for-each` binds the
            // literal `{{ item }}` and `sweep` binds the literal `{{ var }}` (the
            // `var:` field only NAMES the swept value; the body still references
            // it as `{{ var }}`). Other `do:`-bearing nodes (e.g. schedule scopes)
            // add nothing. Ancestors' prefixes stay reserved so a nested body can
            // still reference an outer `{{ item.* }}` without it being mistaken
            // for a node ref.
            let mut body_vars = iter_vars.to_vec();
            if n.for_each.is_some() {
                body_vars.push("item".to_string());
            } else if n.sweep.is_some() {
                body_vars.push("var".to_string());
            }
            flatten_nodes(body, Some(&scoped_id), &body_vars, out);
        }
    }
}

/// Resolve a referenced node id through the lexical scope chain: try the
/// innermost enclosing `do:` scope first (`<scope_prefix>.<nid>`), then each
/// outer scope, then top level (`<nid>`). Returns the scoped id of the first
/// node present in `field_sets`, or `None` if it resolves to no node (a runtime
/// prefix like inputs/secrets/run/now/ctx, or an unknown id). This is what lets
/// a body-local node shadow a same-named outer/top-level node at any nesting
/// depth (Codex #117-3).
fn resolve_scope(
    nid: &str,
    scope_prefix: Option<&str>,
    field_sets: &BTreeMap<String, Option<BTreeSet<String>>>,
) -> Option<String> {
    let mut scope = scope_prefix.map(str::to_string);
    loop {
        let candidate = match &scope {
            Some(p) => format!("{p}.{nid}"),
            None => nid.to_string(),
        };
        if field_sets.contains_key(&candidate) {
            return Some(candidate);
        }
        match scope {
            None => return None,
            Some(p) => scope = p.rsplit_once('.').map(|(par, _)| par.to_string()),
        }
    }
}

fn compile_node(
    node: &crate::manifest::app::Node,
    agents: &[DiscoveredAgent],
) -> Result<CompiledNode, AwareError> {
    let kind = classify_node(node);

    let (mode, output_schema, command_notes) = if let Some(aid) = &node.agent {
        // Agent invocation — look up mode + output schema.
        let cmd_name = node.command.as_deref().unwrap_or("");
        if let Some(d) = agents.iter().find(|d| d.manifest.agent == *aid) {
            if let Some(cmd) = d.manifest.commands.get(cmd_name) {
                // Resolve mode against the manifest. For a `mode-overridable`
                // command (caller-determined behavior, e.g. `exec`) an explicit
                // node-level `mode:` wins; otherwise the manifest is
                // authoritative. (#165)
                let resolved = d.manifest.effective_mode(cmd_name, cmd, node.mode);
                let out = cmd
                    .outputs
                    .as_ref()
                    .and_then(|v| serde_yaml::to_value(v).ok());
                // Benign provenance: the compiler trusted the node-level
                // `mode:` for a command whose mode it can't infer (#165). Info.
                let notes = if resolved.overridden {
                    vec![CompileNote::info(format!(
                        "command {cmd_name} is mode-overridable; using author-declared mode: {}",
                        resolved.mode.as_str()
                    ))]
                } else {
                    Vec::new()
                };
                (resolved.mode.as_str().to_string(), out, notes)
            } else {
                // Command not in manifest — cannot infer mode. If the author
                // declared an explicit node-level `mode:`, honor it (info);
                // otherwise default to write-mode for safety (warn — a silent
                // write-mode fallback is worth surfacing).
                match node.mode {
                    Some(Mode::Read) => (
                        "read".to_string(),
                        None,
                        vec![CompileNote::info(format!(
                            "agent {aid} installed but command {cmd_name} not found; using author-declared mode: read"
                        ))],
                    ),
                    Some(Mode::Write) => (
                        "write".to_string(),
                        None,
                        vec![CompileNote::info(format!(
                            "agent {aid} installed but command {cmd_name} not found; using author-declared mode: write"
                        ))],
                    ),
                    None => (
                        "write".to_string(),
                        None,
                        vec![CompileNote::warn(format!(
                            "agent {aid} installed but command {cmd_name} not found; defaulting to write-mode for safety"
                        ))],
                    ),
                }
            }
        } else {
            // Agent not installed — the missing agent is the salient,
            // actionable fact (schema can't be resolved), so these are warnings
            // regardless of whether the author also declared a mode (#170).
            match node.mode {
                Some(Mode::Read) => (
                    "read".to_string(),
                    None,
                    vec![CompileNote::warn(format!(
                        "agent {aid} not installed; using author-declared mode: read"
                    ))],
                ),
                Some(Mode::Write) => (
                    "write".to_string(),
                    None,
                    vec![CompileNote::warn(format!(
                        "agent {aid} not installed; using author-declared mode: write"
                    ))],
                ),
                None => (
                    "write".to_string(),
                    None,
                    vec![CompileNote::warn(format!(
                        "agent {aid} not installed; defaulting to write-mode for safety"
                    ))],
                ),
            }
        }
    } else if kind == "inline" || kind == "assert" || kind == "compare" || kind == "snapshot" {
        ("read".to_string(), None, Vec::new())
    } else {
        ("write".to_string(), None, Vec::new())
    };

    let safety = node
        .safety
        .as_ref()
        .and_then(|s| serde_yaml::to_value(s).ok());

    // A node's invocation parameters may be written under `config:` or
    // `inputs:` (app-spec allows both; examples favor `inputs:`). `Node::merged_
    // params` collapses them into one map — the SAME rule the runtime uses to
    // invoke the node — so the lockfile can't show args the run drops (#117-3).
    let inputs = node.merged_params();

    // RFC #223: stamp the runtime-model marker + the pinned model id when this node
    // resolves to a curated `model-extraction` command (`vision.extract`). The lock
    // then carries the fact that a model runs here, and the model-pin makes a model
    // swap re-invalidate approval (the validator already fences which nodes may set it).
    let runtime_model = node
        .agent
        .as_ref()
        .zip(node.command.as_ref())
        .and_then(|(aid, cmd_name)| {
            let d = agents.iter().find(|d| d.manifest.agent == *aid)?;
            d.manifest
                .commands
                .get(cmd_name.as_str())
                .map(|c| c.model_extraction)
        })
        .unwrap_or(false);
    let model_pin = if runtime_model {
        inputs
            .as_ref()
            .and_then(|v| v.get("model"))
            .and_then(|m| m.as_str())
            .map(str::to_string)
    } else {
        None
    };

    Ok(CompiledNode {
        id: node.id.clone(),
        kind: kind.to_string(),
        agent: node.agent.clone(),
        command: node.command.clone(),
        mode,
        safety,
        inputs,
        output_schema,
        notes: command_notes,
        runtime_model,
        model_pin,
    })
}

fn classify_node(node: &crate::manifest::app::Node) -> &'static str {
    if node.agent.is_some() {
        "agent"
    } else if node.inline.is_some() {
        "inline"
    } else if node.assert.is_some() {
        "assert"
    } else if node.for_each.is_some() {
        "for-each"
    } else if node.compare.is_some() {
        "compare"
    } else if node.sweep.is_some() {
        "sweep"
    } else if node.approve.is_some() {
        "approve"
    } else if node.snapshot.is_some() {
        "snapshot"
    } else if node.model_lock.is_some() {
        "model-lock"
    } else {
        "unknown"
    }
}

/// Extract the set of top-level output field names from a resolved
/// `output-schema` (the `outputs:` block, shaped `{ type, schema: {...} }`).
/// Returns `None` when the schema is absent or has no `schema:` mapping — the
/// caller treats `None` as "unknown, don't check references into this node".
fn output_field_set(output_schema: Option<&serde_yaml::Value>) -> Option<BTreeSet<String>> {
    let inner = output_schema?.get("schema")?.as_mapping()?;
    let mut set = BTreeSet::new();
    for (k, _) in inner {
        if let Some(s) = k.as_str() {
            set.insert(s.to_string());
        }
    }
    Some(set)
}

/// Walk a config/inputs value and collect `(node-id, field)` pairs from every
/// `{{ <node>.<field>… }}` template reference. Only the leading dotted path is
/// parsed (the first two segments); function calls / operators terminate it,
/// so complex expressions like `{{ join(a, b) }}` yield nothing — keeping the
/// check to direct references, which is the high-signal case.
fn collect_refs(value: &serde_yaml::Value, out: &mut Vec<(String, String)>) {
    match value {
        serde_yaml::Value::String(s) => {
            let mut rest = s.as_str();
            while let Some(start) = rest.find("{{") {
                let after = &rest[start + 2..];
                let Some(end) = after.find("}}") else { break };
                let inner = after[..end].trim();
                let path_end = inner
                    .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.'))
                    .unwrap_or(inner.len());
                let parts: Vec<&str> = inner[..path_end]
                    .split('.')
                    .filter(|p| !p.is_empty())
                    .collect();
                if parts.len() >= 2 {
                    out.push((parts[0].to_string(), parts[1].to_string()));
                }
                rest = &after[end + 2..];
            }
        }
        serde_yaml::Value::Mapping(m) => {
            for (_, v) in m {
                collect_refs(v, out);
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            for v in seq {
                collect_refs(v, out);
            }
        }
        _ => {}
    }
}

/// Like [`collect_refs`] but captures the leading node-id HEAD of every `{{ <head>… }}`
/// reference — including a bare whole-node ref `{{ projects }}` (a single path segment),
/// which `collect_refs` skips (it records only two-segment `<node>.<field>` pairs). Edge
/// derivation (#208) needs the head alone: a whole-node reference reads the upstream
/// node's entire output and is just as much a data dependency as `{{ projects.body }}`.
fn collect_ref_heads(value: &serde_yaml::Value, out: &mut Vec<String>) {
    match value {
        serde_yaml::Value::String(s) => {
            let mut rest = s.as_str();
            while let Some(start) = rest.find("{{") {
                let after = &rest[start + 2..];
                let Some(end) = after.find("}}") else { break };
                let inner = after[..end].trim();
                let path_end = inner
                    .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.'))
                    .unwrap_or(inner.len());
                if let Some(head) = inner[..path_end].split('.').find(|p| !p.is_empty()) {
                    out.push(head.to_string());
                }
                rest = &after[end + 2..];
            }
        }
        serde_yaml::Value::Mapping(m) => {
            for (_, v) in m {
                collect_ref_heads(v, out);
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            for v in seq {
                collect_ref_heads(v, out);
            }
        }
        _ => {}
    }
}

/// Scheduling edges implied by `{{ <node>.<field> }}` data references (#208).
///
/// A node config that reads an upstream node's output (`data: '{{ projects.body }}'`)
/// is a data dependency, but the orchestrator orders execution from explicit
/// `connections` only. So a reference WITHOUT a matching connection used to race its
/// source — and since #205 (whole-value templates resolve structurally) the unresolved
/// ref is a hard `template render: undefined value` rather than a silent empty render.
/// Deriving the implied edges lets the scheduler order the referencing node after its
/// source; a genuinely circular data dependency then surfaces as a topo-sort cycle.
///
/// Every node (top-level or `do:`-body) is scanned; each reference is resolved through
/// the SAME lexical scope chain the compile-time ref-check uses ([`resolve_scope`]). A
/// reference that resolves to a TOP-LEVEL node becomes an edge `from: <referenced>, to:
/// <this node's top-level ancestor>` (connections are between top-level nodes). Excluded:
/// self-edges, references that resolve to a `do:`-body-local node, per-iteration vars
/// (`item` / `var`), namespace heads (`inputs` / `secrets` / `config` / `run` / … —
/// which resolve to no node), and edges already declared in `connections`. The result is
/// the deduped set of NEW edges, sorted for determinism.
pub(crate) fn derive_connections(app: &App) -> Vec<crate::manifest::app::Connection> {
    use crate::manifest::app::Connection;

    let mut flat: Vec<FlatNode> = Vec::new();
    flatten_nodes(&app.nodes, None, &[], &mut flat);

    // Every scoped id is a resolvable target. Values are unused — `resolve_scope` only
    // tests key presence — so they're all `None`. Both the raw id AND its underscore
    // alias are registered, because `record_output` exposes both at run time (a kebab
    // node `tekla-watch` is referenceable as `{{ tekla_watch.x }}`); `canonical` maps the
    // alias back to the raw id so the derived edge matches the node id used in
    // `connections` / topo (#208 Codex).
    let mut known: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();
    let mut canonical: BTreeMap<String, String> = BTreeMap::new();
    for (_, scoped_id, _, _) in &flat {
        known.insert(scoped_id.clone(), None);
        canonical.insert(scoped_id.clone(), scoped_id.clone());
        let aliased = scoped_id.replace('-', "_");
        if aliased != *scoped_id {
            known.entry(aliased.clone()).or_insert(None);
            canonical
                .entry(aliased)
                .or_insert_with(|| scoped_id.clone());
        }
    }

    let existing: BTreeSet<(String, String)> = app
        .connections
        .iter()
        .map(|c| (c.from.clone(), c.to.clone()))
        .collect();

    let mut derived: BTreeSet<(String, String)> = BTreeSet::new();
    for entry in &flat {
        let (node, scoped_id, iter_vars) = (entry.0, &entry.1, &entry.3);
        // Connections are between top-level nodes, so a `do:`-body ref is attributed to
        // its top-level ancestor (the first segment of the scoped id).
        let to_top = scoped_id
            .split('.')
            .next()
            .unwrap_or(scoped_id.as_str())
            .to_string();
        let scope_prefix = scoped_id.rsplit_once('.').map(|(p, _)| p);
        let local_id = node.id.as_str();

        // Collect the node-id HEAD of every reference — including a bare whole-node ref
        // (`{{ projects }}`, `for-each: '{{ projects }}'`) which reads the upstream
        // node's entire output and is just as much a dependency as `{{ projects.body }}`
        // (#208 Codex).
        let mut heads: Vec<String> = Vec::new();
        if let Some(params) = node.merged_params() {
            collect_ref_heads(&params, &mut heads);
        }
        if let Some(expr) = &node.for_each {
            collect_ref_heads(&serde_yaml::Value::String(expr.clone()), &mut heads);
        }
        // Substrate primitives carry cross-node refs OUTSIDE config/inputs, resolved at
        // run time (run_compare / assert / sweep) — scan them too so their sources are
        // ordered first (#208 Codex). compare sides + snapshots, the assert expression,
        // and sweep values can each be a `{{ <node>… }}` reference.
        if let Some(cmp) = &node.compare {
            for s in [&cmp.a, &cmp.b, &cmp.a_snapshot, &cmp.b_snapshot]
                .into_iter()
                .flatten()
            {
                collect_ref_heads(&serde_yaml::Value::String(s.clone()), &mut heads);
            }
        }
        if let Some(assert) = &node.assert {
            collect_ref_heads(&serde_yaml::Value::String(assert.expr.clone()), &mut heads);
        }
        if let Some(sweep) = &node.sweep {
            for v in &sweep.values {
                collect_ref_heads(v, &mut heads);
            }
        }
        for nid in heads {
            if nid == local_id || iter_vars.iter().any(|v| v == &nid) {
                continue;
            }
            let Some(matched) = resolve_scope(&nid, scope_prefix, &known) else {
                continue; // namespace / unknown head — resolves to no node, no edge
            };
            // Map an underscore-alias match back to the canonical (raw) id.
            let target = canonical.get(&matched).cloned().unwrap_or(matched);
            // Only a TOP-LEVEL target (bare id, no scope dot) is a connection endpoint;
            // and never a self-edge to this node's own top-level subtree.
            if target.contains('.') || target == to_top {
                continue;
            }
            let edge = (target, to_top.clone());
            if !existing.contains(&edge) {
                derived.insert(edge);
            }
        }
    }

    derived
        .into_iter()
        .map(|(from, to)| Connection {
            from,
            to,
            label: None,
            input: None,
        })
        .collect()
}

/// Write a lockfile to disk as YAML next to the source app file.
///
/// Output filename: `<app-name>.lock` (substrate-correct per
/// `10-core/app-spec.md § Lockfile sidecar`; NEVER `.flo.lock`).
pub fn write_lockfile(
    lock: &LockFile,
    source_path: &Path,
) -> Result<std::path::PathBuf, AwareError> {
    let dir = source_path
        .parent()
        .ok_or_else(|| AwareError::Internal("source path has no parent".into()))?;
    let lock_path = dir.join(format!("{}.lock", lock.app));
    let yaml = serde_yaml::to_string(lock)
        .map_err(|e| AwareError::Internal(format!("serialize lockfile: {e}")))?;
    let header = format!(
        "# {}.lock — compiled from {}\n# DO NOT EDIT — regenerated by `aware app compile`.\n\n",
        lock.app,
        source_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("(source)")
    );
    std::fs::write(&lock_path, format!("{header}{yaml}"))
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", lock_path.display())))?;
    Ok(lock_path)
}

/// Find the source app file (`.flo` / `.app` / `.flow` / `.aware`) at a path.
/// If `path` is a file, returned directly. If a directory, searched for the
/// first matching extension.
pub fn find_app_source(path: &Path) -> Option<std::path::PathBuf> {
    if path.is_file() {
        return Some(path.to_path_buf());
    }
    std::fs::read_dir(path).ok()?.flatten().find_map(|entry| {
        let p = entry.path();
        match p.extension().and_then(|e| e.to_str()) {
            Some("flo") | Some("app") | Some("flow") | Some("aware") => Some(p),
            _ => None,
        }
    })
}

/// End-to-end: load + compile + write. Called by `aware app compile`.
pub fn compile_to_disk(source: &Path, paths: &Paths) -> Result<std::path::PathBuf, AwareError> {
    let app = crate::manifest::loader::load_app(source)?;
    // Refuse to produce a lock for an app the runtime can't execute (e.g. an
    // inline kind the orchestrator rejects). Gating here covers every
    // lock-producing path — `app compile`, `app inspect`, … — so an unrunnable
    // construct fails before locking, not at run (#160).
    let issues = crate::validate::validate_app(&app);
    if let Some(err) = issues
        .iter()
        .find(|i| i.severity == crate::validate::Severity::Error)
    {
        return Err(AwareError::Validation(format!(
            "app failed validation: [{}] {}",
            err.code, err.message
        )));
    }
    let agents = discover_agents(paths)?;
    // Refuse to lock an app that references a not-yet-runnable agent (e.g.
    // html-report, whose transport binary isn't shipped) — fail here, not at run
    // with "program not found" (#161).
    if let Some(err) = crate::validate::validate_app_agents(&app, &agents)
        .into_iter()
        .find(|i| i.severity == crate::validate::Severity::Error)
    {
        return Err(AwareError::Validation(format!(
            "app failed validation: [{}] {}",
            err.code, err.message
        )));
    }
    // #349: refuse to lock an app against an agent whose installed version the
    // app's `requires:` pin does not admit. Unlike the missing-agent case below
    // this is not a "compile now, install later" gap — the agent IS installed
    // and is the wrong one, so the lock would record a version the author never
    // asked for, and the lock is the approved artifact.
    if let Some(err) =
        crate::validate::unsatisfied_pins(&app, &agents, crate::validate::Severity::Error).first()
    {
        return Err(AwareError::Validation(format!(
            "app failed validation: [{}] {}",
            err.code, err.message
        )));
    }
    // #308: warn (don't refuse) when a node's agent isn't installed. Compiling
    // before the agents are installed is supported — the node is locked with its
    // author-declared mode and no resolved schema (#170) — but that gap used to
    // be silent, and only showed up at run as a bare `os error 3`. `aware app
    // run` refuses it; here the user just gets told, with the remedy.
    for m in crate::validate::missing_agents(&app, &agents, crate::validate::Severity::Warning) {
        eprintln!("\u{26a0} [{}] {}", m.code, m.message);
    }
    let lock = compile(&app, &agents, source)?;
    write_lockfile(&lock, source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_lockfile_uses_substrate_correct_filename() {
        // The lockfile sidecar is named `<app-name>.lock`, NEVER `.flo.lock`.
        // This test guards against the FloLess-anchor regression that was
        // caught + corrected during the v0.24 design.
        let tmp = tempfile::tempdir().unwrap();
        let source = tmp.path().join("my-app.flo");
        std::fs::write(&source, "app: my-cool-app\n").unwrap();
        let lock = LockFile {
            source_hash: "sha256:test".into(),
            compiled_at: "2026-05-17T00:00:00Z".into(),
            compiler_version: "0.24.0".into(),
            app: "my-cool-app".into(),
            version: "0.1.0".into(),
            agent_pins: BTreeMap::new(),
            nodes: vec![],
            schedule: None,
            engineering: None,
        };
        let lock_path = write_lockfile(&lock, &source).unwrap();
        assert_eq!(lock_path.file_name().unwrap(), "my-cool-app.lock");
        // The .flo extension MUST NOT appear in the lockfile name.
        assert!(!lock_path.to_string_lossy().contains(".flo.lock"));
    }

    #[test]
    fn find_app_source_picks_first_matching_extension() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("README.md"), "x").unwrap();
        std::fs::write(tmp.path().join("my.app"), "x").unwrap();
        let found = find_app_source(tmp.path()).unwrap();
        assert_eq!(found.extension().unwrap(), "app");
    }

    #[test]
    fn find_app_source_accepts_extension_agnostic_inputs() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("x.flow"), "x").unwrap();
        let found = find_app_source(tmp.path()).unwrap();
        assert_eq!(found.extension().unwrap(), "flow");
    }

    #[test]
    fn output_field_set_reads_schema_keys() {
        let v: serde_yaml::Value =
            serde_yaml::from_str("type: single\nschema:\n  path: string\n  row-count: number\n")
                .unwrap();
        let fields = output_field_set(Some(&v)).unwrap();
        assert!(fields.contains("path") && fields.contains("row-count"));
        // No `schema:` mapping → None (treated as "unknown, don't check").
        let no_schema: serde_yaml::Value = serde_yaml::from_str("type: stream").unwrap();
        assert!(output_field_set(Some(&no_schema)).is_none());
    }

    #[test]
    fn collect_refs_extracts_direct_paths_only() {
        let v: serde_yaml::Value = serde_yaml::from_str(
            "a: '{{ src.path }}'\nb: '{{ join(x.y, z) }}'\nc: '{{ list.folders.*.id }}'\n",
        )
        .unwrap();
        let mut refs = Vec::new();
        collect_refs(&v, &mut refs);
        assert!(refs.contains(&("src".to_string(), "path".to_string())));
        assert!(refs.contains(&("list".to_string(), "folders".to_string())));
        // Function-call expression contributes no direct ref.
        assert!(!refs.iter().any(|(n, _)| n == "join"));
    }

    #[test]
    fn derive_connections_adds_edge_for_ref_without_connection() {
        // #208: `report` reads `{{ projects.body }}` but declares NO connection — the
        // implied data-dependency edge projects->report must be derived. A namespace
        // ref (`{{ inputs.title }}`) and a self-ref must NOT become edges.
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("pipe.flo");
        std::fs::write(
            &src,
            r#"app: pipe
version: 0.0.1
description: x
nodes:
  - id: projects
    agent: trimble-connect
    command: list-projects
  - id: report
    agent: html-report
    command: render
    config:
      data: '{{ projects.body }}'
      title: '{{ inputs.title }}'
      self: '{{ report.x }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let derived = derive_connections(&app);
        assert!(
            derived
                .iter()
                .any(|c| c.from == "projects" && c.to == "report"),
            "expected derived edge projects->report; got {:?}",
            derived
                .iter()
                .map(|c| (c.from.as_str(), c.to.as_str()))
                .collect::<Vec<_>>()
        );
        assert!(
            !derived.iter().any(|c| c.from == "inputs"),
            "namespace head must not become an edge: {derived:?}"
        );
        assert!(
            !derived
                .iter()
                .any(|c| c.from == "report" || c.to == "projects"),
            "self-ref / reverse edge must not be derived: {derived:?}"
        );
    }

    #[test]
    fn derive_connections_skips_already_declared_edges() {
        // An explicit connection a->b already covers the `{{ a.body }}` ref, so nothing
        // new is derived (no duplicate edge).
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("explicit.flo");
        std::fs::write(
            &src,
            r#"app: explicit
version: 0.0.1
description: x
nodes:
  - id: a
    agent: x
    command: emit
  - id: b
    agent: x
    command: consume
    config:
      v: '{{ a.body }}'
connections:
  - from: a
    to: b
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        assert!(
            derive_connections(&app).is_empty(),
            "explicit a->b edge must not be re-derived"
        );
    }

    #[test]
    fn derive_connections_covers_compare_sides() {
        // #208 Codex: a `compare` node's inline sides (`a:`/`b:`) carry cross-node refs
        // resolved at run time, outside config/inputs — derive_connections must still
        // order their sources first.
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("prims.flo");
        std::fs::write(
            &src,
            r#"app: prims
version: 0.0.1
description: x
nodes:
  - id: left
    agent: x
    command: emit
  - id: right
    agent: x
    command: emit
  - id: diff
    compare:
      a: '{{ left.rows }}'
      b: '{{ right.rows }}'
      by: id
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let derived = derive_connections(&app);
        assert!(
            derived.iter().any(|c| c.from == "left" && c.to == "diff"),
            "compare side `a` must derive left->diff: {derived:?}"
        );
        assert!(
            derived.iter().any(|c| c.from == "right" && c.to == "diff"),
            "compare side `b` must derive right->diff: {derived:?}"
        );
    }

    #[test]
    fn derive_connections_resolves_underscore_alias() {
        // #208 Codex: a kebab node `tekla-watch` referenced via its underscore alias
        // `{{ tekla_watch.mark }}` (which record_output exposes at run time) must still
        // derive an edge — keyed to the canonical raw id used in connections / topo.
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("alias.flo");
        std::fs::write(
            &src,
            r#"app: alias
version: 0.0.1
description: x
nodes:
  - id: tekla-watch
    agent: x
    command: emit
  - id: consumer
    agent: x
    command: consume
    config:
      v: '{{ tekla_watch.mark }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let derived = derive_connections(&app);
        assert!(
            derived
                .iter()
                .any(|c| c.from == "tekla-watch" && c.to == "consumer"),
            "underscore-alias ref must derive tekla-watch->consumer (canonical id); got {derived:?}"
        );
    }

    #[test]
    fn derive_connections_handles_bare_whole_node_ref() {
        // #208 Codex: a bare whole-node ref `{{ projects }}` (one path segment) reads the
        // upstream node's entire output and must derive an edge — `collect_refs` records
        // only two-segment `<node>.<field>` pairs, so the head-collector covers this.
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("whole.flo");
        std::fs::write(
            &src,
            r#"app: whole
version: 0.0.1
description: x
nodes:
  - id: projects
    agent: x
    command: emit
  - id: report
    agent: x
    command: consume
    config:
      data: '{{ projects }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let derived = derive_connections(&app);
        assert!(
            derived
                .iter()
                .any(|c| c.from == "projects" && c.to == "report"),
            "bare whole-node ref must derive projects->report; got {derived:?}"
        );
    }

    #[test]
    fn compile_flags_unknown_output_field_references() {
        use crate::manifest::loader::DiscoveredAgent;
        let agent_yaml = r#"
agent: testagent
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-test } }
commands:
  emit:
    lifecycle: single
    category: curated
    description: emits a path
    outputs:
      type: single
      schema:
        path: string
        row-count: number
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: consumes
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("app.flo");
        std::fs::write(
            &src,
            r#"app: refcheck
version: 0.0.1
description: x
nodes:
  - id: src
    agent: testagent
    command: emit
  - id: sink
    agent: testagent
    command: consume
    config:
      good: '{{ src.path }}'
      bad: '{{ src.nope }}'
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let sink = lock.nodes.iter().find(|n| n.id == "sink").unwrap();
        assert!(
            sink.notes.iter().any(|n| n.text.contains("src.nope")),
            "expected a note for the bad reference; notes: {:?}",
            sink.notes
        );
        assert!(
            !sink.notes.iter().any(|n| n.text.contains("{{ src.path }}")),
            "valid reference must NOT be flagged; notes: {:?}",
            sink.notes
        );
    }

    #[test]
    fn vision_extract_node_stamps_runtime_model_and_pin() {
        // RFC #223 §5.3: a curated model-extraction node compiles with
        // `runtime-model: true` + the pinned model id; an ordinary node carries neither.
        let agent_yaml = r#"agent: vision
version: 0.1.0
description: x
stateful: false
license: MIT
capabilities:
  runtime-model-extraction: true
transport:
  builtin: {}
commands:
  extract:
    lifecycle: single
    category: curated
    mode: read
    model-extraction: true
    description: x
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("app.flo");
        std::fs::write(
            &src,
            r#"app: vapp
version: 0.0.1
description: x
nodes:
  - id: extract
    agent: vision
    command: extract
    config:
      model: claude-sonnet-4-6
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let n = lock.nodes.iter().find(|n| n.id == "extract").unwrap();
        assert!(
            n.runtime_model,
            "vision.extract must stamp runtime-model: true"
        );
        assert_eq!(
            n.model_pin.as_deref(),
            Some("claude-sonnet-4-6"),
            "must pin the model id from the node's `model` input"
        );
    }

    #[test]
    fn compile_descends_into_for_each_do_body() {
        use crate::manifest::loader::DiscoveredAgent;
        let testagent: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: testagent
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-test } }
commands:
  emit:
    lifecycle: single
    category: curated
    description: emits
    outputs:
      type: single
      schema:
        path: string
        rows: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: consumes
"#,
        )
        .unwrap();
        // `writer` is referenced ONLY inside the for-each `do:` body — proving
        // the compiler descends (pins + compiles + ref-checks inner nodes).
        let writer: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: writer
version: 2.3.4
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-writer } }
commands:
  post:
    lifecycle: single
    category: curated
    mode: write
    description: writes
    outputs:
      type: single
      schema:
        file-id: string
"#,
        )
        .unwrap();
        let agents = vec![
            DiscoveredAgent {
                manifest: testagent,
                root: std::path::PathBuf::from("/dev/null"),
            },
            DiscoveredAgent {
                manifest: writer,
                root: std::path::PathBuf::from("/dev/null"),
            },
        ];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("feach.flo");
        std::fs::write(
            &src,
            r#"app: feach
version: 0.0.1
description: x
nodes:
  - id: src
    agent: testagent
    command: emit
  - id: sync
    for-each: '{{ src.rows }}'
    do:
      - id: upsert
        agent: writer
        command: post
        config:
          good: '{{ src.path }}'
          bad:  '{{ src.nope }}'
  - id: after
    agent: testagent
    command: consume
    config:
      x: '{{ upsert.nope }}'
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();

        // Inner do: agent is pinned.
        assert_eq!(
            lock.agent_pins.get("writer").map(String::as_str),
            Some("2.3.4"),
            "inner do: agent not pinned: {:?}",
            lock.agent_pins
        );
        // Inner node is compiled into the lock under its scoped id.
        let upsert = lock
            .nodes
            .iter()
            .find(|n| n.id == "sync.upsert")
            .expect("inner do: node 'sync.upsert' missing from lock");
        // Ref-check descended into the do: body (bad ref flagged, good one not).
        assert!(
            upsert.notes.iter().any(|n| n.text.contains("src.nope")),
            "bad ref inside do: not flagged; notes: {:?}",
            upsert.notes
        );
        assert!(
            !upsert
                .notes
                .iter()
                .any(|n| n.text.contains("{{ src.path }}")),
            "valid ref inside do: must not be flagged; notes: {:?}",
            upsert.notes
        );
        // Scope: a top-level node referencing a do:-body-local id (`upsert`)
        // must NOT resolve it — body ids aren't globally addressable, so the
        // ref is treated as an unknown prefix (skipped), not blessed or flagged.
        let after = lock.nodes.iter().find(|n| n.id == "after").unwrap();
        assert!(
            !after.notes.iter().any(|n| n.text.contains("upsert")),
            "body-local id leaked into global scope (should not resolve): {:?}",
            after.notes
        );
    }

    #[test]
    fn do_body_reusing_top_level_id_does_not_overwrite_field_set() {
        use crate::manifest::loader::DiscoveredAgent;
        // `dup` is BOTH a top-level node (outputs `rows`) and a reused do:-body
        // id (outputs `file-id`, no `rows`). The top-level schema must win in
        // the global field set; a string-keyed filter would let the body node
        // overwrite it and falsely flag `{{ dup.rows }}`.
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  hasrows:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        rows: array
  norows:
    lifecycle: single
    category: curated
    mode: write
    description: x
    outputs:
      type: single
      schema:
        file-id: string
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("reuse.flo");
        std::fs::write(
            &src,
            r#"app: reuse
version: 0.0.1
description: x
nodes:
  - id: dup
    agent: a
    command: hasrows
  - id: loop
    for-each: '{{ dup.rows }}'
    do:
      - id: dup
        agent: a
        command: norows
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        // `{{ dup.rows }}` (on `loop`) must validate against the TOP-LEVEL dup,
        // which has `rows` — not the body dup, which doesn't. So: no note.
        let lp = lock.nodes.iter().find(|n| n.id == "loop").unwrap();
        assert!(
            !lp.notes.iter().any(|n| n.text.contains("rows")),
            "top-level dup schema overwritten by the do:-body dup: {:?}",
            lp.notes
        );
    }

    #[test]
    fn do_body_ref_to_shadowing_sibling_not_validated_against_top_level() {
        use crate::manifest::loader::DiscoveredAgent;
        // A do:-body node `rfis` shadows a same-named top-level node. A sibling
        // body node references `{{ rfis.issues }}` — which lexically resolves to
        // the body `rfis`, so it must NOT be validated against (and falsely
        // flagged by) the top-level `rfis` schema (Codex #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  toprows:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        rows: array
  bodyissues:
    lifecycle: single
    category: curated
    mode: write
    description: x
    outputs:
      type: single
      schema:
        issues: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("shadow.flo");
        std::fs::write(
            &src,
            r#"app: shadow
version: 0.0.1
description: x
nodes:
  - id: rfis
    agent: a
    command: toprows
  - id: loop
    for-each: '{{ rfis.rows }}'
    do:
      - id: rfis
        agent: a
        command: bodyissues
      - id: consumer
        agent: a
        command: consume
        config:
          x: '{{ rfis.issues }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let consumer = lock.nodes.iter().find(|n| n.id == "loop.consumer").unwrap();
        assert!(
            !consumer.notes.iter().any(|n| n.text.contains("rfis")),
            "body ref to shadowing sibling wrongly validated against top-level: {:?}",
            consumer.notes
        );
    }

    #[test]
    fn do_body_item_ref_not_validated_against_same_named_top_level_node() {
        use crate::manifest::loader::DiscoveredAgent;
        // `item` is the for-each per-iteration variable inside a `do:` body. A
        // top-level node ALSO named `item` (outputs `bar`, not `foo`) must not
        // cause `{{ item.foo }}` in a body node to be flagged: inside the body,
        // `item` is the runtime per-iteration prefix, not the node (Codex #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  itemcmd:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        bar: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("itemvar.flo");
        std::fs::write(
            &src,
            r#"app: itemvar
version: 0.0.1
description: x
nodes:
  - id: item
    agent: a
    command: itemcmd
  - id: loop
    for-each: '{{ item.bar }}'
    do:
      - id: consumer
        agent: a
        command: consume
        config:
          x: '{{ item.foo }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        // The body `{{ item.foo }}` is the per-iteration var — no note. The
        // top-level `{{ item.bar }}` on `loop` is a real ref that resolves.
        let consumer = lock.nodes.iter().find(|n| n.id == "loop.consumer").unwrap();
        assert!(
            !consumer.notes.iter().any(|n| n.text.contains("item")),
            "body `item` per-iteration ref wrongly validated against top-level node: {:?}",
            consumer.notes
        );
    }

    #[test]
    fn do_body_node_inputs_key_is_preserved_in_lockfile() {
        use crate::manifest::loader::DiscoveredAgent;
        // A for-each `do:` body node that declares its parameters under the
        // `inputs:` key (the app-spec key the examples favor) must keep them in
        // the compiled lockfile — `compile_node` historically copied only
        // `config:`, dropping `inputs:` from newly-flattened body nodes (Codex
        // #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  rows:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        items: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("bodyinputs.flo");
        std::fs::write(
            &src,
            r#"app: bodyinputs
version: 0.0.1
description: x
nodes:
  - id: src
    agent: a
    command: rows
  - id: loop
    for-each: '{{ src.items }}'
    do:
      - id: worker
        agent: a
        command: consume
        inputs:
          target: '{{ item.id }}'
          mode: 'sync'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let worker = lock.nodes.iter().find(|n| n.id == "loop.worker").unwrap();
        let inputs = worker
            .inputs
            .as_ref()
            .expect("body node inputs: dropped from lockfile");
        let map = inputs.as_mapping().expect("inputs not a mapping");
        assert!(
            map.contains_key(serde_yaml::Value::String("target".into()))
                && map.contains_key(serde_yaml::Value::String("mode".into())),
            "body node `inputs:` keys missing from lockfile: {inputs:?}"
        );
    }

    #[test]
    fn sweep_body_checks_item_ref_but_skips_literal_var_prefix() {
        use crate::manifest::loader::DiscoveredAgent;
        // In a `sweep` body, the per-step prefix is the literal `{{ var }}` (app-
        // spec § Substrate primitives), NOT the configured `var:` name. So:
        // `{{ item.foo }}` is not reserved here (that's the for-each var) and must
        // be checked against a real top-level `item` node, while `{{ var.foo }}` is
        // the reserved runtime prefix and is skipped even when a top-level node is
        // literally named `var` (Codex #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  itemcmd:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        bar: array
  varcmd:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        qux: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("sweepitem.flo");
        std::fs::write(
            &src,
            r#"app: sweepitem
version: 0.0.1
description: x
nodes:
  - id: item
    agent: a
    command: itemcmd
  - id: var
    agent: a
    command: varcmd
  - id: study
    sweep:
      var: storeys
      values: [3, 4, 5]
    do:
      - id: worker
        agent: a
        command: consume
        config:
          a: '{{ item.foo }}'
          b: '{{ var.foo }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let worker = lock.nodes.iter().find(|n| n.id == "study.worker").unwrap();
        assert!(
            worker
                .notes
                .iter()
                .any(|n| n.text.contains("item") && n.text.contains("foo")),
            "sweep body `item` ref should be checked against top-level node: {:?}",
            worker.notes
        );
        assert!(
            !worker.notes.iter().any(|n| n.text.contains("\"var\"")),
            "literal sweep prefix `var` wrongly validated against top-level `var` node: {:?}",
            worker.notes
        );
    }

    #[test]
    fn nested_body_keeps_outer_iteration_var_in_scope() {
        use crate::manifest::loader::DiscoveredAgent;
        // A `sweep` nested inside a `for-each` body: the inner worker references
        // the OUTER for-each var `{{ item.target }}`. `item` must stay reserved
        // from the enclosing for-each — even with a top-level node named `item`
        // that lacks `target` — so no bogus note is produced. The inner sweep var
        // (`storeys`) is also reserved (Codex #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  itemcmd:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        bar: array
  produce:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        items: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("nested.flo");
        std::fs::write(
            &src,
            r#"app: nested
version: 0.0.1
description: x
nodes:
  - id: item
    agent: a
    command: itemcmd
  - id: src
    agent: a
    command: produce
  - id: outer
    for-each: '{{ src.items }}'
    do:
      - id: study
        sweep:
          var: storeys
          values: [1, 2]
        do:
          - id: worker
            agent: a
            command: consume
            config:
              a: '{{ item.target }}'
              b: '{{ var.label }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let worker = lock
            .nodes
            .iter()
            .find(|n| n.id == "outer.study.worker")
            .unwrap();
        assert!(
            !worker.notes.iter().any(|n| n.text.contains("item")),
            "outer for-each `item` var lost in nested sweep body: {:?}",
            worker.notes
        );
        assert!(
            !worker.notes.iter().any(|n| n.text.contains("\"var\"")),
            "inner sweep prefix `var` wrongly validated as a node ref: {:?}",
            worker.notes
        );
    }

    #[test]
    fn nested_body_ref_resolves_to_enclosing_body_local_shadow() {
        use crate::manifest::loader::DiscoveredAgent;
        // A body-local `rfis` (outputs `issues`) in the OUTER for-each body
        // shadows a top-level `rfis` (outputs `bar`). A worker in a NESTED sweep
        // body references `{{ rfis.issues }}`: lexical resolution must walk out to
        // the enclosing `outer.rfis` (which has `issues`), not fall through to the
        // top-level `rfis` (which doesn't) and emit a bogus note (Codex #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  topcmd:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        bar: array
  produce:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        items: array
  bodycmd:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        issues: array
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("shadownest.flo");
        std::fs::write(
            &src,
            r#"app: shadownest
version: 0.0.1
description: x
nodes:
  - id: rfis
    agent: a
    command: topcmd
  - id: src
    agent: a
    command: produce
  - id: outer
    for-each: '{{ src.items }}'
    do:
      - id: rfis
        agent: a
        command: bodycmd
      - id: study
        sweep:
          var: storeys
          values: [1, 2]
        do:
          - id: worker
            agent: a
            command: consume
            config:
              x: '{{ rfis.issues }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let worker = lock
            .nodes
            .iter()
            .find(|n| n.id == "outer.study.worker")
            .unwrap();
        assert!(
            !worker.notes.iter().any(|n| n.text.contains("rfis")),
            "nested ref resolved to top-level `rfis` instead of the enclosing body-local one: {:?}",
            worker.notes
        );
    }

    #[test]
    fn ref_check_uses_merged_params_not_overridden_config() {
        use crate::manifest::loader::DiscoveredAgent;
        // A node declares the same key under both `config:` and `inputs:`. The
        // merge keeps the `inputs:` value, so the ref checker must validate that
        // surviving value — not the discarded `config:` ref (Codex #117-3).
        let a: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: a
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-a } }
commands:
  produce:
    lifecycle: single
    category: curated
    description: x
    outputs:
      type: single
      schema:
        path: string
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: x
"#,
        )
        .unwrap();
        let agents = vec![DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }];
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("override.flo");
        std::fs::write(
            &src,
            r#"app: override
version: 0.0.1
description: x
nodes:
  - id: src
    agent: a
    command: produce
  - id: sink
    agent: a
    command: consume
    config:
      x: '{{ src.nope }}'
    inputs:
      x: '{{ src.path }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let sink = lock.nodes.iter().find(|n| n.id == "sink").unwrap();
        assert!(
            !sink.notes.iter().any(|n| n.text.contains("nope")),
            "ref check flagged the overridden config: ref instead of the merged inputs: value: {:?}",
            sink.notes
        );
    }

    #[test]
    fn compile_honors_author_declared_mode_on_unknown_command() {
        // An author-declared `mode: read` on a node whose command is not found
        // in the agent manifest (e.g. `exec`) must be honored — not silently
        // overridden to write-mode (#165).
        use crate::manifest::loader::DiscoveredAgent;
        let agent_yaml = r#"
agent: tekla
version: 0.1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  sheet.list:
    lifecycle: single
    category: curated
    description: lists sheets
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("exec-mode.flo");
        // `exec` is not in the agent manifest above — the compiler cannot infer
        // mode from the manifest. The author-declared `mode: read` must win.
        std::fs::write(
            &src,
            r#"app: exec-mode-repro
version: 0.1.0
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    mode: read
    config:
      code: return new { ok = true };
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let probe = lock.nodes.iter().find(|n| n.id == "probe").unwrap();

        assert_eq!(
            probe.mode, "read",
            "author-declared mode: read must be honored on exec node; got: {}",
            probe.mode
        );
        assert!(
            !probe.notes.iter().any(|n| n.text.contains("defaulting")),
            "must not claim we defaulted when author declared a mode; notes: {:?}",
            probe.notes
        );
        assert!(
            probe
                .notes
                .iter()
                .any(|n| n.text.contains("author-declared")),
            "note must mention that the author-declared mode was used; notes: {:?}",
            probe.notes
        );
    }

    #[test]
    fn compile_defaults_write_mode_on_unknown_command_without_declaration() {
        // When no `mode:` is declared on a node with an unknown command, the
        // original safe default (write-mode) must still apply (#165 regression guard).
        use crate::manifest::loader::DiscoveredAgent;
        let agent_yaml = r#"
agent: tekla
version: 0.1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  sheet.list:
    lifecycle: single
    category: curated
    description: lists sheets
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("exec-no-mode.flo");
        std::fs::write(
            &src,
            r#"app: exec-no-mode
version: 0.1.0
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    config:
      code: return new { ok = true };
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let probe = lock.nodes.iter().find(|n| n.id == "probe").unwrap();

        assert_eq!(
            probe.mode, "write",
            "must still default to write-mode when no mode is declared; got: {}",
            probe.mode
        );
        let defaulting_note = probe
            .notes
            .iter()
            .find(|n| n.text.contains("defaulting"))
            .expect("must include the defaulting-to-write note");
        // #170: a silent write-mode fallback is actionable — must be `warn`.
        assert_eq!(
            defaulting_note.kind,
            NoteKind::Warn,
            "defaulting-to-write note must be warn, got {:?}",
            defaulting_note.kind
        );
    }

    #[test]
    fn compile_honors_mode_read_on_mode_overridable_command() {
        // The real #165 repro: `exec` IS declared in the manifest with a
        // conservative `mode: write` default, but flagged `mode-overridable`.
        // An explicit node-level `mode: read` must win — the lock records
        // `read`, with an informational (non-"defaulting") note.
        use crate::manifest::loader::DiscoveredAgent;
        let agent_yaml = r#"
agent: tekla
version: 0.1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    category: curated
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("exec-read.flo");
        std::fs::write(
            &src,
            r#"app: exec-read
version: 0.1.0
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    mode: read
    config:
      code: return new { ok = true };
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let probe = lock.nodes.iter().find(|n| n.id == "probe").unwrap();

        assert_eq!(
            probe.mode, "read",
            "explicit mode: read must win on a mode-overridable command; got: {}",
            probe.mode
        );
        assert!(
            !probe.notes.iter().any(|n| n.text.contains("defaulting")),
            "must not emit a defaulting note when the author declared the mode; notes: {:?}",
            probe.notes
        );
        let override_note = probe
            .notes
            .iter()
            .find(|n| n.text.contains("mode-overridable") && n.text.contains("mode: read"))
            .expect("note must explain the override");
        // #170: the override note is benign provenance — must be `info` so
        // consumers don't render it as a warning.
        assert_eq!(
            override_note.kind,
            NoteKind::Info,
            "author-declared-mode provenance note must be info, got {:?}",
            override_note.kind
        );
    }

    #[test]
    fn compile_defaults_write_on_mode_overridable_command_without_declaration() {
        // An un-annotated node on a mode-overridable `exec` keeps the
        // conservative `mode: write` default (so the safety contract still
        // applies), with no override note.
        use crate::manifest::loader::DiscoveredAgent;
        let agent_yaml = r#"
agent: tekla
version: 0.1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    category: curated
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("exec-default.flo");
        std::fs::write(
            &src,
            r#"app: exec-default
version: 0.1.0
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    config:
      code: return new { ok = true };
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let probe = lock.nodes.iter().find(|n| n.id == "probe").unwrap();

        assert_eq!(
            probe.mode, "write",
            "un-annotated node on mode-overridable exec must default to write; got: {}",
            probe.mode
        );
        assert!(
            probe.notes.is_empty(),
            "manifest-authoritative default needs no note; notes: {:?}",
            probe.notes
        );
    }

    #[test]
    fn compile_note_serializes_as_kind_and_text() {
        // #170: the lock contract is `notes: [{ kind, text }]` with a
        // lowercase `kind`. Consumers (floless.app, the lock audit) rely on
        // this shape to render by severity without parsing prose.
        use crate::manifest::loader::DiscoveredAgent;
        let agent_yaml = r#"
agent: tekla
version: 0.1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    category: curated
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#;
        let manifest: crate::manifest::Agent = serde_yaml::from_str(agent_yaml).unwrap();
        let agents = vec![DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("exec-read.flo");
        std::fs::write(
            &src,
            r#"app: exec-read
version: 0.1.0
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    mode: read
    config:
      code: return new { ok = true };
requires: []
"#,
        )
        .unwrap();

        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let yaml = serde_yaml::to_string(&lock).unwrap();

        // The note must serialize as a `{ kind, text }` map with a lowercase
        // kind — not a bare string.
        assert!(
            yaml.contains("kind: info"),
            "note must serialize with a lowercase `kind:`; yaml:\n{yaml}"
        );
        assert!(
            yaml.contains("text: 'command exec is mode-overridable")
                || yaml.contains("text: \"command exec is mode-overridable")
                || yaml.contains("text: command exec is mode-overridable"),
            "note must serialize its prose under `text:`; yaml:\n{yaml}"
        );
    }

    // ── node classification and the kind → mode default ───────────────────────
    //
    // `kind` and `mode` are the two lockfile fields an approver actually reads.
    // `aware app glass-box` colours a card red and demands a `safety:` block on
    // `mode: write`, grey and silent on `mode: read` (`commands::app::
    // render_glass_box_html`). That renderer is tested against hand-built
    // `CompiledNode`s; nothing tested that `compile` puts the right two strings
    // into the lock in the first place, so a primitive that mutates a live model
    // could have compiled to `read` and shown up grey with no missing-safety
    // badge, on the one surface a human approves the run from.

    /// One agent whose two commands pin the ends of the mode axis, so an
    /// `agent` node's mode is unambiguously the manifest's and not a default.
    fn mode_axis_agents() -> Vec<crate::manifest::loader::DiscoveredAgent> {
        let manifest: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: testagent
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-test } }
commands:
  emit:
    lifecycle: single
    category: curated
    mode: read
    description: reads
    outputs:
      type: single
      schema:
        path: string
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: writes
"#,
        )
        .unwrap();
        vec![crate::manifest::loader::DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }]
    }

    /// One node per declarable shape, including a node that declares no
    /// primitive at all. Returns the compiled lock so a test can read the two
    /// published fields off it rather than off an internal.
    fn compile_one_node_of_every_kind(tmp: &Path) -> LockFile {
        let src = tmp.join("kinds.flo");
        std::fs::write(
            &src,
            r#"app: kinds
version: 0.0.1
description: x
nodes:
  - id: n-agent-read
    agent: testagent
    command: emit
  - id: n-agent-write
    agent: testagent
    command: consume
  - id: n-inline
    inline:
      kind: transform
      description: glue
      code: 'return 1;'
  - id: n-assert
    assert:
      expr: '1 == 1'
  - id: n-compare
    compare:
      a: '{{ n-agent-read.path }}'
      b: '{{ n-agent-read.path }}'
      by: id
  - id: n-snapshot
    snapshot:
      of:
        agent: testagent
        target: model
      name: before
  - id: n-for-each
    for-each: '[1, 2]'
    do:
      - id: inner
        inline:
          kind: transform
          description: inner glue
          code: 'return 2;'
  - id: n-sweep
    sweep:
      var: t
      values: [1, 2]
  - id: n-approve
    approve:
      channel: cli
      prompt: proceed?
  - id: n-model-lock
    model-lock:
      agent: testagent
      target: model
  - id: n-nothing
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        compile(&app, &mode_axis_agents(), &src).unwrap()
    }

    fn compiled<'a>(lock: &'a LockFile, id: &str) -> &'a CompiledNode {
        lock.nodes
            .iter()
            .find(|n| n.id == id)
            .unwrap_or_else(|| panic!("node {id} missing from lock"))
    }

    #[test]
    fn every_declared_primitive_is_named_by_its_own_kind_in_the_lock() {
        // `kind` is what the Glass Box prints for a node with no agent/command
        // pair (`<em>{kind}</em>`), and what a consumer switches on to render a
        // plan. Each arm is asserted separately: a classifier that answered
        // "for-each" for a `sweep:` would still satisfy a test that only counted
        // the kinds present.
        let tmp = tempfile::tempdir().unwrap();
        let lock = compile_one_node_of_every_kind(tmp.path());
        for (id, want) in [
            ("n-agent-read", "agent"),
            ("n-inline", "inline"),
            ("n-assert", "assert"),
            ("n-compare", "compare"),
            ("n-snapshot", "snapshot"),
            ("n-for-each", "for-each"),
            ("n-sweep", "sweep"),
            ("n-approve", "approve"),
            ("n-model-lock", "model-lock"),
            ("n-nothing", "unknown"),
        ] {
            let node = compiled(&lock, id);
            assert_eq!(
                node.kind, want,
                "node {id} compiled as kind {:?}, expected {want:?}",
                node.kind
            );
        }
    }

    #[test]
    fn only_the_observing_primitives_default_to_read_mode() {
        // The safety-relevant half. A primitive with no agent behind it takes
        // its mode from `kind` alone, and the split is not cosmetic: `read`
        // renders grey and is exempt from the "safety MISSING" badge, so any
        // model-touching primitive that landed on the read side would be
        // approved as an observer. `for-each` and `sweep` drive their `do:`
        // bodies, `approve` gates a write, `model-lock` takes a writer's lock on
        // a live model — none of them belongs there.
        let tmp = tempfile::tempdir().unwrap();
        let lock = compile_one_node_of_every_kind(tmp.path());
        for (id, want) in [
            ("n-inline", "read"),
            ("n-assert", "read"),
            ("n-compare", "read"),
            ("n-snapshot", "read"),
            ("n-for-each", "write"),
            ("n-sweep", "write"),
            ("n-approve", "write"),
            ("n-model-lock", "write"),
        ] {
            let node = compiled(&lock, id);
            assert_eq!(
                node.mode, want,
                "node {id} (kind {}) compiled as mode {:?}, expected {want:?}",
                node.kind, node.mode
            );
        }
        // An agent node is NOT decided by the kind table — `agent` sits on the
        // write side of it, yet a read-mode command must still compile to read.
        // Without this the two rules are indistinguishable on every agent node
        // whose command happens to be a writer.
        assert_eq!(compiled(&lock, "n-agent-read").mode, "read");
        assert_eq!(compiled(&lock, "n-agent-write").mode, "write");
    }

    #[test]
    fn a_node_that_declares_no_primitive_is_unknown_and_falls_to_write() {
        // The fail-safe, asserted on its own because it is the one case with no
        // author intent to read: a node the compiler cannot place must be
        // presented as a writer, so the approver sees a red card demanding a
        // safety block rather than a grey one that reads as harmless.
        let tmp = tempfile::tempdir().unwrap();
        let lock = compile_one_node_of_every_kind(tmp.path());
        let node = compiled(&lock, "n-nothing");
        assert_eq!(node.kind, "unknown");
        assert_eq!(
            node.mode, "write",
            "an unclassifiable node must default to write-mode for safety"
        );
        assert!(
            node.agent.is_none() && node.command.is_none(),
            "the fixture node declares neither, so the Glass Box labels it by kind"
        );
    }

    #[test]
    fn a_do_body_node_is_classified_by_its_own_declaration_not_its_parents() {
        // `do:` bodies are flattened into the same flat node list under a scoped
        // id and rendered as their own cards. The body of a write-mode
        // `for-each` is an inline transform and must compile as one — inheriting
        // the parent's kind would mislabel every body node in the lock.
        let tmp = tempfile::tempdir().unwrap();
        let lock = compile_one_node_of_every_kind(tmp.path());
        let inner = compiled(&lock, "n-for-each.inner");
        assert_eq!(inner.kind, "inline");
        assert_eq!(inner.mode, "read");
        assert_eq!(
            compiled(&lock, "n-for-each").kind,
            "for-each",
            "the parent keeps its own kind"
        );
    }

    #[test]
    fn an_uninstalled_agent_is_warned_about_whatever_mode_the_author_declared() {
        // #170 draws a line the note text alone does not: when the agent is
        // missing, the mode is the lesser fact and every note is a WARN — an
        // author-declared `mode: read` does not downgrade it to info, because
        // what is actually wrong is that no manifest was consulted at all and
        // the output schema is unknowable. Nothing exercised this branch, so a
        // compiler that emitted `info` here (or resolved the missing agent to a
        // silent read) would have compiled clean.
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("ghost.flo");
        std::fs::write(
            &src,
            r#"app: ghost
version: 0.0.1
description: x
nodes:
  - id: silent
    agent: ghost-agent
    command: whatever
  - id: says-read
    agent: ghost-agent
    command: whatever
    mode: read
  - id: says-write
    agent: ghost-agent
    command: whatever
    mode: write
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        // Deliberately compiled against an EMPTY agent set — that is the
        // "not installed" condition.
        let lock = compile(&app, &[], &src).unwrap();

        for (id, want_mode) in [
            ("silent", "write"),
            ("says-read", "read"),
            ("says-write", "write"),
        ] {
            let node = compiled(&lock, id);
            assert_eq!(
                node.mode, want_mode,
                "node {id} compiled as mode {:?}",
                node.mode
            );
            assert_eq!(
                node.notes.len(),
                1,
                "node {id} must carry exactly one note; got {:?}",
                node.notes
            );
            assert_eq!(
                node.notes[0].kind,
                NoteKind::Warn,
                "an uninstalled agent is a warning whatever the declared mode; node {id} note: {:?}",
                node.notes[0]
            );
            assert!(
                node.notes[0].text.contains("not installed"),
                "the note must name the missing agent as the cause; got {:?}",
                node.notes[0].text
            );
            assert!(
                node.output_schema.is_none(),
                "no manifest was read, so node {id} can carry no output schema"
            );
        }
    }

    #[test]
    fn a_command_missing_from_an_installed_manifest_informs_when_told_and_warns_when_guessing() {
        // The other half of the #170 severity split, and the discriminating one:
        // both branches land on `mode: write`, so mode alone cannot tell them
        // apart. An author who wrote `mode: write` has already made the call —
        // that is provenance (info). A compiler that had to guess is reporting
        // its own uncertainty (warn), and collapsing the two would either bury
        // the guess or nag about a decision already made.
        let manifest: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: testagent
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-test } }
commands:
  known:
    lifecycle: single
    category: curated
    mode: read
    description: the only command this agent has
"#,
        )
        .unwrap();
        let agents = vec![crate::manifest::loader::DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("missing-cmd.flo");
        std::fs::write(
            &src,
            r#"app: missing-cmd
version: 0.0.1
description: x
nodes:
  - id: declared
    agent: testagent
    command: absent
    mode: write
  - id: guessed
    agent: testagent
    command: absent
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();

        let declared = compiled(&lock, "declared");
        assert_eq!(declared.mode, "write");
        assert_eq!(
            declared.notes[0].kind,
            NoteKind::Info,
            "an author-declared write is provenance, not a warning: {:?}",
            declared.notes[0]
        );
        assert!(
            declared.notes[0]
                .text
                .contains("using author-declared mode: write"),
            "the note must say the declaration was honoured; got {:?}",
            declared.notes[0].text
        );

        let guessed = compiled(&lock, "guessed");
        assert_eq!(guessed.mode, "write");
        assert_eq!(
            guessed.notes[0].kind,
            NoteKind::Warn,
            "a silent write-mode fallback must be surfaced: {:?}",
            guessed.notes[0]
        );
        assert_ne!(
            declared.notes[0].kind, guessed.notes[0].kind,
            "the two branches must be distinguishable by severity, since both compile to write"
        );
    }

    #[test]
    fn a_reference_from_an_assert_or_a_sweep_still_orders_the_node_after_its_source() {
        // #208 scans the substrate primitives that carry cross-node references
        // OUTSIDE `config:`/`inputs:` — the assert expression and the sweep
        // values are resolved at run time, so without a derived edge the
        // scheduler is free to run them before the node they read. Both
        // scanners were untested: deleting either left every existing test
        // green while the assert raced its source.
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("prims.flo");
        std::fs::write(
            &src,
            r#"app: prims
version: 0.0.1
description: x
nodes:
  - id: src
    agent: testagent
    command: emit
  - id: gate
    assert:
      expr: '{{ src.rows }} > 0'
  - id: sweeper
    sweep:
      var: t
      values:
        - '{{ src.path }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let edges: Vec<(String, String)> = derive_connections(&app)
            .into_iter()
            .map(|c| (c.from, c.to))
            .collect();
        assert!(
            edges.contains(&("src".to_string(), "gate".to_string())),
            "an assert reading {{{{ src.rows }}}} must be ordered after src; edges: {edges:?}"
        );
        assert!(
            edges.contains(&("src".to_string(), "sweeper".to_string())),
            "a sweep value reading {{{{ src.path }}}} must be ordered after src; edges: {edges:?}"
        );
    }

    #[test]
    fn a_reference_buried_in_a_list_value_is_ref_checked_like_any_other() {
        // `collect_refs` recurses through mappings AND sequences, but only the
        // mapping arm was exercised. App authors routinely pass lists of
        // templated paths, and a broken reference inside one is exactly as fatal
        // at run time (`template render: undefined value`) as a scalar — it just
        // used to compile without a note.
        let manifest: crate::manifest::Agent = serde_yaml::from_str(
            r#"
agent: testagent
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-test } }
commands:
  emit:
    lifecycle: single
    category: curated
    mode: read
    description: emits
    outputs:
      type: single
      schema:
        path: string
  consume:
    lifecycle: single
    category: curated
    mode: write
    description: consumes
"#,
        )
        .unwrap();
        let agents = vec![crate::manifest::loader::DiscoveredAgent {
            manifest,
            root: std::path::PathBuf::from("/dev/null"),
        }];

        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("list-ref.flo");
        std::fs::write(
            &src,
            r#"app: list-ref
version: 0.0.1
description: x
nodes:
  - id: src
    agent: testagent
    command: emit
  - id: sink
    agent: testagent
    command: consume
    config:
      files:
        - '{{ src.path }}'
        - '{{ src.nope }}'
requires: []
"#,
        )
        .unwrap();
        let app = crate::manifest::loader::load_app(&src).unwrap();
        let lock = compile(&app, &agents, &src).unwrap();
        let sink = compiled(&lock, "sink");
        assert!(
            sink.notes.iter().any(|n| n.text.contains("src.nope")),
            "a bad reference inside a list must be flagged; notes: {:?}",
            sink.notes
        );
        assert!(
            !sink.notes.iter().any(|n| n.text.contains("src.path")),
            "the valid sibling in the same list must not be flagged; notes: {:?}",
            sink.notes
        );
    }
}
