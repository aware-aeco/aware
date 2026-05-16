//! Typed deserialization for AWARE app manifests (`.flo` files).
//!
//! Shapes verified against the 2 reference apps:
//! - `30-apps/_examples/welded-to-tc.flo`   — linear layout, 3 nodes, 2 connections
//! - `30-apps/_examples/qa-drawings-to-tekla.flo` — DAG layout, 7 nodes, 6 connections

use serde::Deserialize;
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
