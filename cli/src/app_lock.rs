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

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::agent::Mode;
use crate::manifest::loader::{DiscoveredAgent, discover_agents};
use crate::paths::Paths;

/// The lockfile schema. Serialized as YAML to `<app>.lock`.
#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

    /// Free-form notes captured at compile time (e.g. "agent not
    /// installed — schema deferred to first install").
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
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
    let source_bytes = std::fs::read(source_path)
        .map_err(|e| AwareError::Internal(format!("read {}: {e}", source_path.display())))?;
    let mut hasher = Sha256::new();
    hasher.update(&source_bytes);
    let source_hash = format!("sha256:{:x}", hasher.finalize());

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
                nodes[i].notes.push(format!(
                    "input references {{{{ {nid}.{field} }}}} but node {nid:?} has no output field {field:?} (available: {available})"
                ));
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
/// `iter_vars` is the set of per-iteration variables reserved by ALL enclosing
/// `do:`-bearing primitives — `item` for each `for-each`, the `var:` name for
/// each `sweep` — accumulated outermost-to-innermost. Empty for top-level nodes.
/// Each is a runtime prefix, not a node ref, so the ref checker skips it within
/// scope. Nested bodies inherit their ancestors' vars (a `sweep` inside a
/// `for-each` keeps `item` reserved alongside the sweep var).
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
            // The body adds THIS node's per-iteration variable to the inherited
            // set: `for-each` binds `item`, `sweep` binds its `var:` name. Other
            // `do:`-bearing nodes (e.g. schedule scopes) add nothing. Ancestors'
            // vars stay reserved so a nested body can still reference an outer
            // `{{ item.* }}` without it being mistaken for a node ref.
            let mut body_vars = iter_vars.to_vec();
            if n.for_each.is_some() {
                body_vars.push("item".to_string());
            } else if let Some(s) = &n.sweep {
                body_vars.push(s.var.clone());
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
                let m = d.manifest.mode_of(cmd_name, cmd);
                let mode_str = match m {
                    Mode::Read => "read",
                    Mode::Write => "write",
                };
                let out = cmd
                    .outputs
                    .as_ref()
                    .and_then(|v| serde_yaml::to_value(v).ok());
                (mode_str.to_string(), out, Vec::new())
            } else {
                (
                    "write".to_string(),
                    None,
                    vec![format!(
                        "agent {aid} installed but command {cmd_name} not found; defaulting to write-mode for safety"
                    )],
                )
            }
        } else {
            (
                "write".to_string(),
                None,
                vec![format!(
                    "agent {aid} not installed; defaulting to write-mode for safety"
                )],
            )
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
    let agents = discover_agents(paths)?;
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
            sink.notes.iter().any(|n| n.contains("src.nope")),
            "expected a note for the bad reference; notes: {:?}",
            sink.notes
        );
        assert!(
            !sink.notes.iter().any(|n| n.contains("{{ src.path }}")),
            "valid reference must NOT be flagged; notes: {:?}",
            sink.notes
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
            upsert.notes.iter().any(|n| n.contains("src.nope")),
            "bad ref inside do: not flagged; notes: {:?}",
            upsert.notes
        );
        assert!(
            !upsert.notes.iter().any(|n| n.contains("{{ src.path }}")),
            "valid ref inside do: must not be flagged; notes: {:?}",
            upsert.notes
        );
        // Scope: a top-level node referencing a do:-body-local id (`upsert`)
        // must NOT resolve it — body ids aren't globally addressable, so the
        // ref is treated as an unknown prefix (skipped), not blessed or flagged.
        let after = lock.nodes.iter().find(|n| n.id == "after").unwrap();
        assert!(
            !after.notes.iter().any(|n| n.contains("upsert")),
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
            !lp.notes.iter().any(|n| n.contains("rows")),
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
            !consumer.notes.iter().any(|n| n.contains("rfis")),
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
            !consumer.notes.iter().any(|n| n.contains("item")),
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
    fn sweep_body_checks_item_ref_but_skips_sweep_var() {
        use crate::manifest::loader::DiscoveredAgent;
        // In a `sweep` body, `item` is NOT the per-iteration variable (that's the
        // sweep's `var:` name). `{{ item.foo }}` must still be validated against a
        // real top-level `item` node, while the sweep var (`storeys`) is the
        // reserved runtime prefix and is skipped (Codex #117-3).
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
          b: '{{ storeys.label }}'
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
                .any(|n| n.contains("item") && n.contains("foo")),
            "sweep body `item` ref should be checked against top-level node: {:?}",
            worker.notes
        );
        assert!(
            !worker.notes.iter().any(|n| n.contains("storeys")),
            "sweep var `storeys` wrongly validated as a node ref: {:?}",
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
              b: '{{ storeys.label }}'
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
            !worker.notes.iter().any(|n| n.contains("item")),
            "outer for-each `item` var lost in nested sweep body: {:?}",
            worker.notes
        );
        assert!(
            !worker.notes.iter().any(|n| n.contains("storeys")),
            "inner sweep var `storeys` wrongly validated as a node ref: {:?}",
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
            !worker.notes.iter().any(|n| n.contains("rfis")),
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
            !sink.notes.iter().any(|n| n.contains("nope")),
            "ref check flagged the overridden config: ref instead of the merged inputs: value: {:?}",
            sink.notes
        );
    }
}
