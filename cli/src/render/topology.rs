//! ASCII topology rendering for `aware app show`.
//!
//! Consumed by Task 13 (`app show`). The helpers below are intentionally
//! internal — only `format_topology` is public API.

use crate::manifest::App;
use crate::manifest::app::{Layout, Node};

pub fn format_topology(app: &App) -> String {
    match app.layout {
        Layout::Linear => format_linear(app),
        Layout::Dag => format_dag(app),
    }
}

fn node_label(node: &Node) -> String {
    if let Some(agent) = &node.agent {
        let cmd = node.command.as_deref().unwrap_or("?");
        format!("[{}] ({}/{cmd})", node.id, agent)
    } else if let Some(inline) = &node.inline {
        format!("[{}] (inline/{})", node.id, inline.kind)
    } else {
        format!("[{}]", node.id)
    }
}

fn format_linear(app: &App) -> String {
    let mut out = String::new();
    out.push_str(&format!("Topology ({} nodes, linear):\n", app.nodes.len()));

    let order = topological_order(app);
    for (i, node) in order.iter().enumerate() {
        if i == 0 {
            out.push_str(&format!("  {}\n", node_label(node)));
        } else {
            out.push_str(&format!("    │\n    ▼\n  {}\n", node_label(node)));
        }
    }
    out
}

fn format_dag(app: &App) -> String {
    let mut out = String::new();
    out.push_str(&format!("Topology ({} nodes, dag):\n", app.nodes.len()));
    out.push_str("\nNodes:\n");
    for node in &app.nodes {
        let pos = match (node.row, node.col) {
            (Some(r), Some(c)) => format!("  (row {r}, col {c})"),
            _ => String::new(),
        };
        out.push_str(&format!("  {}{pos}\n", node_label(node)));
    }
    out.push_str("\nConnections:\n");
    for c in &app.connections {
        let label = c.label.as_deref().unwrap_or("");
        let input = c
            .input
            .as_ref()
            .map(|i| format!(" → input:{i}"))
            .unwrap_or_default();
        out.push_str(&format!("  {} → {}  [{label}]{input}\n", c.from, c.to));
    }
    out
}

/// Heuristic topo sort for the linear case: source first, walk by edges.
fn topological_order(app: &App) -> Vec<&Node> {
    use std::collections::{HashMap, HashSet};

    let mut by_id: HashMap<&str, &Node> = app.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let mut next_by_id: HashMap<&str, &str> = HashMap::new();
    for c in &app.connections {
        next_by_id.insert(c.from.as_str(), c.to.as_str());
    }
    let mut destinations: HashSet<&str> = HashSet::new();
    for c in &app.connections {
        destinations.insert(c.to.as_str());
    }

    // Find the node that is never a destination — the source.
    let source = app
        .nodes
        .iter()
        .find(|n| !destinations.contains(n.id.as_str()))
        .map(|n| n.id.as_str());

    let mut out: Vec<&Node> = Vec::new();
    let mut current = source;
    let mut seen: HashSet<&str> = HashSet::new();
    while let Some(id) = current {
        if !seen.insert(id) {
            break;
        }
        if let Some(node) = by_id.remove(id) {
            out.push(node);
        }
        current = next_by_id.get(id).copied();
    }
    // Any nodes not reached (disconnected) get appended in original order.
    for n in &app.nodes {
        if !seen.contains(n.id.as_str()) {
            out.push(n);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_fixture(rel: &str) -> App {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel);
        let text = std::fs::read_to_string(&path).unwrap();
        serde_yaml::from_str(&text).unwrap()
    }

    #[test]
    fn linear_topology_lists_nodes_in_flow_order() {
        let app = load_fixture("30-apps/_examples/welded-to-tc.flo");
        let s = format_topology(&app);
        let pos_watch = s.find("tekla-watch").unwrap();
        let pos_filter = s.find("filter-welded").unwrap();
        let pos_upload = s.find("tc-upload").unwrap();
        assert!(
            pos_watch < pos_filter && pos_filter < pos_upload,
            "flow order broken: {s}"
        );
        assert!(s.contains("linear"));
    }

    #[test]
    fn dag_topology_lists_nodes_and_connections() {
        let app = load_fixture("30-apps/_examples/qa-drawings-to-tekla.flo");
        let s = format_topology(&app);
        assert!(s.contains("dag"));
        assert!(s.contains("match-build"));
        assert!(s.contains("→"));
        // fan-in: both pdf-extract and excel-lookup point at match-build
        assert!(s.contains("pdf-extract → match-build"));
        assert!(s.contains("excel-lookup → match-build"));
    }
}
