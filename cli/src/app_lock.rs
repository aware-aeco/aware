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

use std::collections::BTreeMap;
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

    // Pin every agent referenced by any node.
    let mut agent_pins: BTreeMap<String, String> = BTreeMap::new();
    for node in &app.nodes {
        if let Some(aid) = &node.agent
            && let Some(d) = agents.iter().find(|d| d.manifest.agent == *aid)
        {
            agent_pins.insert(aid.clone(), d.manifest.version.clone());
        }
    }

    // Compile each node.
    let mut nodes: Vec<CompiledNode> = Vec::new();
    for node in &app.nodes {
        nodes.push(compile_node(node, agents)?);
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

    let inputs = if node.config.is_null() {
        None
    } else {
        Some(node.config.clone())
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

/// Write a lockfile to disk as YAML next to the source app file.
///
/// Output filename: `<app-name>.lock` (substrate-correct per
/// `10-core/app-spec.md § Lockfile sidecar`; NEVER `.flo.lock`).
pub fn write_lockfile(lock: &LockFile, source_path: &Path) -> Result<std::path::PathBuf, AwareError> {
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
}
