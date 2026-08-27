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

    // `linear_topology_lists_nodes_in_flow_order` stood here. It asserted that
    // `welded-to-tc.app` renders tekla-watch before filter-welded before
    // tc-upload — but that fixture DECLARES its three nodes in exactly that
    // order (`30-apps/_examples/welded-to-tc.app`), and `topological_order`
    // falls back to declaration order for everything the edge walk does not
    // reach. Replace `current = next_by_id.get(id).copied()` with
    // `current = None` — deleting the edge walk outright — and the test still
    // passed. It could not fail for the reason its name gave, and the residue
    // it did check (the fixture parses, renders `linear`, names all three
    // nodes) is asserted end-to-end by
    // `tests/app_show.rs::shows_welded_to_tc_linear_topology` through the same
    // code path. `linear_flow_follows_the_edges_rather_than_declaration_order`
    // below makes the claim testable by declaring the nodes in the opposite
    // order to the flow; it is the test that catches that mutation.

    #[test]
    fn dag_topology_lists_nodes_and_connections() {
        let app = load_fixture("30-apps/_examples/qa-drawings-to-tekla.app");
        let s = format_topology(&app);
        assert!(s.contains("dag"));
        assert!(s.contains("match-build"));
        assert!(s.contains("→"));
        // fan-in: both pdf-extract and excel-lookup point at match-build
        assert!(s.contains("pdf-extract → match-build"));
        assert!(s.contains("excel-lookup → match-build"));
    }

    /// The fixture test above reads a real substrate app, and those are
    /// well-behaved by construction: one source, one edge out of each node, no
    /// orphans, no loops, nodes declared in flow order.
    /// Everything below feeds `format_topology` the shapes a hand-written `.app`
    /// can have and a fixture never does. `App` is `Deserialize`-only, so the
    /// graph is spelled as YAML rather than built field-by-field — which also
    /// keeps the assertions pointed at the renderer instead of at a struct the
    /// test just filled in.
    fn app_from(body: &str) -> App {
        serde_yaml::from_str(&format!("app: t\nversion: 0.1.0\ndescription: t\n{body}")).unwrap()
    }

    /// How many times `[<id>]` — the label prefix every node gets — appears.
    fn listed(rendered: &str, id: &str) -> usize {
        rendered.matches(&format!("[{id}]")).count()
    }

    /// `topological_order` walks the edge chain and then sweeps up whatever the
    /// walk missed. Both halves have to hold for the header's node count to be
    /// true, and neither is exercised by a linear fixture: this graph forks
    /// (`a` has two outgoing edges, and `next_by_id` keeps only one of them) and
    /// carries a node no edge mentions at all.
    #[test]
    fn every_node_is_listed_exactly_once_however_the_edges_run() {
        let app = app_from(
            "layout: linear
nodes:
  - id: a
    agent: x
    command: go
  - id: b
    agent: x
    command: go
  - id: c
    agent: x
    command: go
  - id: orphan
    agent: x
    command: go
connections:
  - from: a
    to: b
  - from: a
    to: c
",
        );
        let s = format_topology(&app);
        for id in ["a", "b", "c", "orphan"] {
            assert_eq!(listed(&s, id), 1, "node {id} not listed exactly once: {s}");
        }
        // The header promises a count; the body has to honour it.
        assert!(s.contains("4 nodes"), "header lost the node count: {s}");
    }

    /// The point of ordering at all: the flow the edges describe, not the order
    /// the author happened to declare the nodes in. Declared c, b, a — rendered
    /// a, b, c.
    #[test]
    fn linear_flow_follows_the_edges_rather_than_declaration_order() {
        let app = app_from(
            "layout: linear
nodes:
  - id: c
    agent: x
    command: go
  - id: b
    agent: x
    command: go
  - id: a
    agent: x
    command: go
connections:
  - from: a
    to: b
  - from: b
    to: c
",
        );
        let s = format_topology(&app);
        let (pa, pb, pc) = (
            s.find("[a]").unwrap(),
            s.find("[b]").unwrap(),
            s.find("[c]").unwrap(),
        );
        assert!(pa < pb && pb < pc, "edges did not drive the order: {s}");
    }

    /// One arrow BETWEEN each pair, so a three-node chain gets two — and the
    /// first node is never preceded by one.
    #[test]
    fn arrows_join_consecutive_nodes_and_never_lead_the_first() {
        let app = app_from(
            "layout: linear
nodes:
  - id: a
    agent: x
    command: go
  - id: b
    agent: x
    command: go
  - id: c
    agent: x
    command: go
connections:
  - from: a
    to: b
  - from: b
    to: c
",
        );
        let s = format_topology(&app);
        assert_eq!(s.matches('▼').count(), 2, "wrong arrow count: {s}");
        let first_arrow = s.find('▼').unwrap();
        assert!(
            s.find("[a]").unwrap() < first_arrow,
            "an arrow leads the first node: {s}"
        );
    }

    /// A closed loop has no node that is never a destination, so the walk has no
    /// source to start from and contributes nothing. The sweep is then the only
    /// thing standing between a cyclic app and a `Topology (3 nodes, linear):`
    /// header with no nodes under it.
    #[test]
    fn a_closed_loop_of_edges_still_lists_every_node() {
        let app = app_from(
            "layout: linear
nodes:
  - id: a
    agent: x
    command: go
  - id: b
    agent: x
    command: go
  - id: c
    agent: x
    command: go
connections:
  - from: a
    to: b
  - from: b
    to: c
  - from: c
    to: a
",
        );
        let s = format_topology(&app);
        for id in ["a", "b", "c"] {
            assert_eq!(listed(&s, id), 1, "node {id} lost to the cycle: {s}");
        }
    }

    /// A chain that re-enters itself (`a → b → c → b`) DOES have a source, so the
    /// walk runs and reaches `b` twice. It has to stop there: the guard is what
    /// makes `app show` terminate on a hand-written app with a loop in it, and
    /// what keeps `b` off the list twice.
    #[test]
    fn a_path_that_re_enters_itself_terminates_without_repeating_a_node() {
        let app = app_from(
            "layout: linear
nodes:
  - id: a
    agent: x
    command: go
  - id: b
    agent: x
    command: go
  - id: c
    agent: x
    command: go
connections:
  - from: a
    to: b
  - from: b
    to: c
  - from: c
    to: b
",
        );
        let s = format_topology(&app);
        for id in ["a", "b", "c"] {
            assert_eq!(listed(&s, id), 1, "node {id} not listed once: {s}");
        }
    }

    /// The three label shapes. A node can name an agent (with or without a
    /// command), carry inline code instead, or be a bare primitive node —
    /// `for-each`, `approve`, `assert` and friends all render through this last
    /// branch, and the substrate fixtures above exercise only the first.
    #[test]
    fn a_node_label_names_its_agent_command_its_inline_kind_or_neither() {
        let app = app_from(
            "layout: linear
nodes:
  - id: with-command
    agent: tekla
    command: watch
  - id: without-command
    agent: tekla
  - id: inline-node
    inline:
      kind: python
      description: filter the rows
  - id: bare
",
        );
        let s = format_topology(&app);
        assert!(s.contains("[with-command] (tekla/watch)"), "{s}");
        assert!(s.contains("[without-command] (tekla/?)"), "{s}");
        assert!(s.contains("[inline-node] (inline/python)"), "{s}");
        // A bare node gets the id and nothing else — no empty parens, no `/?`.
        assert!(
            s.lines().any(|l| l.trim() == "[bare]"),
            "bare node did not render as a naked label: {s}"
        );
    }

    /// The dag view annotates a node with its grid slot, and a slot needs both
    /// halves to mean anything. A node given only one of the two is rendered as
    /// unpositioned rather than as half a coordinate.
    #[test]
    fn dag_shows_a_grid_position_only_when_both_row_and_col_are_given() {
        let app = app_from(
            "layout: dag
nodes:
  - id: placed
    agent: x
    command: go
    row: 1
    col: 2
  - id: half-placed
    agent: x
    command: go
    row: 3
  - id: unplaced
    agent: x
    command: go
",
        );
        let s = format_topology(&app);
        // Row and col in that order — a swap here silently mislabels the grid.
        assert!(s.contains("[placed] (x/go)  (row 1, col 2)"), "{s}");
        for line in s.lines() {
            if line.contains("[half-placed]") || line.contains("[unplaced]") {
                assert!(
                    !line.contains("row") && !line.contains("col"),
                    "partial coordinate rendered as a position: {line}"
                );
            }
        }
    }

    /// A dag connection line carries an optional label and an optional target
    /// input. Both are `Option`, and the fixture app declares labels on every
    /// edge — so the degraded form (empty brackets, no input suffix) has never
    /// been rendered under test.
    #[test]
    fn dag_connections_carry_label_and_input_and_degrade_to_empty_brackets() {
        let app = app_from(
            "layout: dag
nodes:
  - id: a
    agent: x
    command: go
  - id: b
    agent: x
    command: go
connections:
  - from: a
    to: b
    label: welded parts
    input: parts
  - from: b
    to: a
",
        );
        let s = format_topology(&app);
        assert!(
            s.contains("a → b  [welded parts] → input:parts"),
            "labelled edge lost its label or input: {s}"
        );
        assert!(
            s.contains("b → a  []\n"),
            "bare edge did not degrade to empty brackets: {s}"
        );
    }
}
