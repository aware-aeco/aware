//! Typed deserialization for AWARE app manifests (`.flo` files).
//!
//! Shapes verified against the 2 reference apps:
//! - `30-apps/_examples/welded-to-tc.flo`   — linear layout, 3 nodes, 2 connections
//! - `30-apps/_examples/qa-drawings-to-tekla.flo` — DAG layout, 7 nodes, 6 connections

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Deserialize)]
pub struct App {
    pub app: String,
    pub version: String,
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    pub description: String,
    #[serde(rename = "exposes-as-agent", default)]
    pub exposes_as_agent: bool,
    #[allow(dead_code)]
    #[serde(rename = "exposed-commands", default)]
    pub exposed_commands: Option<Value>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[allow(dead_code)]
    #[serde(rename = "requires-permissions")]
    pub requires_permissions: Option<Value>,
    #[serde(default = "default_layout")]
    pub layout: Layout,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[allow(dead_code)]
    #[serde(default)]
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Linear,
    Dag,
}

fn default_layout() -> Layout {
    Layout::Linear
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub agent: Option<String>,
    pub command: Option<String>,
    pub inline: Option<Inline>,
    pub row: Option<u32>,
    pub col: Option<u32>,
    #[allow(dead_code)]
    #[serde(default)]
    pub config: Value,
    #[allow(dead_code)]
    #[serde(default)]
    pub inputs: Value,
    /// Safety contract — required on write-mode nodes per app-spec § Safety contract.
    /// `aware app validate` rejects a write-mode node without this block;
    /// `aware app run` refuses to execute one.
    #[serde(default)]
    pub safety: Option<Safety>,
}

/// Safety contract block — declared on write-mode nodes.
///
/// See `10-core/app-spec.md § Safety contract (write-mode nodes)` for the
/// full semantics. Each field guards a class of failure mode the 2026-05-17
/// persona audit identified as a blocker for live-model adoption.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Safety {
    /// Names the rollback boundary. Multiple write nodes sharing a
    /// `transaction-group` rollback together.
    #[serde(rename = "transaction-group")]
    pub transaction_group: Option<String>,

    /// If `true`, the agent's transport must save a snapshot before mutating.
    /// If explicitly `false`, the app author has acknowledged no-undo risk.
    #[serde(default)]
    pub snapshot: bool,

    /// Override the default snapshot path
    /// (`~/.aware/snapshots/<app>/<run-id>/<node-id>/`).
    #[serde(rename = "snapshot-path")]
    pub snapshot_path: Option<String>,

    /// Worksharing pre-flight (for Revit / Tekla / Allplan).
    #[serde(default)]
    pub worksharing: Worksharing,

    /// Audit stamp — write per-object UDAs/properties recording the run.
    #[serde(rename = "audit-stamp", default)]
    pub audit_stamp: Option<AuditStamp>,

    /// Templating expression resolving to the rollback token returned by
    /// the write. Used by `aware app rollback`.
    #[serde(rename = "rollback-token")]
    pub rollback_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Worksharing {
    /// Pre-flight check that the document is in a writable state by the
    /// current user. Default `true` for Revit / Tekla / Allplan.
    #[serde(default)]
    pub check: Option<bool>,

    /// Abort if any touched element is owned by someone else.
    #[serde(rename = "fail-if-other-user", default)]
    pub fail_if_other_user: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuditStamp {
    /// Per-object UDA / property prefix. Default `AWARE_`.
    #[serde(rename = "uda-prefix", default = "default_uda_prefix")]
    pub uda_prefix: String,

    /// Subset of `{run-id, app, operator, timestamp}` to write.
    /// Default: all four.
    #[serde(default = "default_audit_fields")]
    pub fields: Vec<String>,
}

fn default_uda_prefix() -> String {
    "AWARE_".to_string()
}

fn default_audit_fields() -> Vec<String> {
    vec![
        "run-id".to_string(),
        "app".to_string(),
        "operator".to_string(),
        "timestamp".to_string(),
    ]
}

#[derive(Debug, Deserialize, Clone)]
pub struct Inline {
    pub kind: String,
    #[allow(dead_code)]
    pub description: String,
    #[allow(dead_code)]
    pub code: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub input: Option<String>,
}

impl App {
    #[allow(dead_code)]
    pub fn is_dag(&self) -> bool {
        self.layout == Layout::Dag
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_real_welded_to_tc() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("30-apps/_examples/welded-to-tc.flo");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: App = serde_yaml::from_str(&text).unwrap();
        assert_eq!(a.app, "welded-to-tc");
        assert_eq!(a.version, "0.3.1");
        assert!(a.exposes_as_agent);
        assert_eq!(a.layout, Layout::Linear);
        assert_eq!(a.node_count(), 3);
        assert_eq!(a.connection_count(), 2);
    }

    #[test]
    fn parses_real_qa_drawings_to_tekla() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("30-apps/_examples/qa-drawings-to-tekla.flo");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: App = serde_yaml::from_str(&text).unwrap();
        assert_eq!(a.app, "qa-drawings-to-tekla");
        assert_eq!(a.layout, Layout::Dag);
        assert!(a.is_dag());
        assert_eq!(a.node_count(), 7);
        assert_eq!(a.connection_count(), 6);
    }

    #[test]
    fn parses_inline_glue_nodes() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("30-apps/_examples/welded-to-tc.flo");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: App = serde_yaml::from_str(&text).unwrap();
        let inline = a.nodes.iter().find(|n| n.id == "filter-welded").unwrap();
        assert!(inline.inline.is_some());
        assert_eq!(inline.inline.as_ref().unwrap().kind, "predicate");
    }
}
