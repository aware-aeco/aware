//! Typed deserialization for AWARE app manifests (`.flo` files).
//!
//! Shapes verified against the 2 reference apps:
//! - `30-apps/_examples/welded-to-tc.flo`   — linear layout, 3 nodes, 2 connections
//! - `30-apps/_examples/qa-drawings-to-tekla.flo` — DAG layout, 7 nodes, 6 connections

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

pub use super::agent::Mode;

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
    /// App-level `schedule:` block — registers a cron-style trigger
    /// with the runtime. See `10-core/app-spec.md § Substrate primitives`.
    #[serde(default)]
    pub schedule: Option<ScheduleBlock>,
    /// App-level `engineering:` block — pins code revisions / catalogues /
    /// solver builds for reproducible engineer-signed deliverables (v0.21).
    /// See `10-core/agent-spec.md § Engineering envelope`.
    #[serde(default)]
    pub engineering: Option<EngineeringBinding>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EngineeringBinding {
    /// Map of pin-id → pin-value (e.g. `code-of-practice: "eurocode-3@2022+uk-na"`).
    /// Validated at install time against the engineering agent's declared
    /// pinnable set.
    #[serde(default)]
    pub pins: std::collections::BTreeMap<String, String>,
    /// Optional output-seal block — produces an `.aware-receipt.json`
    /// chain-of-custody record next to the named artifact.
    #[serde(rename = "output-seal", default)]
    pub output_seal: Option<OutputSeal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OutputSeal {
    /// Template expression resolving to the artifact path
    /// (e.g. `'{{ calc-pack.path }}'`).
    pub artifact: String,
    /// Template expression resolving to the operator's id
    /// (e.g. CEng/PE reference).
    pub operator: String,
    /// Optional template expression resolving to a digital-signing
    /// credential (e.g. `'{{ secrets.ceng-seal }}'`).
    pub credential: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScheduleBlock {
    pub cron: String,
    pub timezone: String,
    #[serde(rename = "start-date", default)]
    pub start_date: Option<String>,
    #[serde(rename = "end-date", default)]
    pub end_date: Option<String>,
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
    /// Author-declared read/write intent. Honored by the compiler when the
    /// command cannot be inferred from the agent manifest (e.g. `exec`, or
    /// agent not yet installed). When present and the command is unknown,
    /// this resolves the ambiguity instead of defaulting to write-mode.
    #[serde(default)]
    pub mode: Option<Mode>,
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

    // ---- v0.19 substrate primitives (10-core/app-spec.md § Substrate primitives) ----
    /// `for-each` — iterate over a static list, run `do:` per item.
    #[serde(rename = "for-each", default)]
    pub for_each: Option<String>,

    /// `compare` — diff two snapshots / lists by an identity key.
    #[serde(default)]
    pub compare: Option<CompareBlock>,

    /// `sweep` — parametric sweep over a variable.
    #[serde(default)]
    pub sweep: Option<SweepBlock>,

    /// `approve` — human-in-the-loop pause.
    #[serde(default)]
    pub approve: Option<ApproveBlock>,

    /// `assert` — pre-flight gate; halt on false.
    #[serde(default)]
    pub assert: Option<AssertBlock>,

    /// `snapshot` — freeze model state to an immutable artifact.
    #[serde(default)]
    pub snapshot: Option<SnapshotBlock>,

    /// `model-lock` — single-writer guarantee on a host model.
    #[serde(rename = "model-lock", default)]
    pub model_lock: Option<ModelLockBlock>,

    /// `panel:` — multi-voice review on write-mode nodes (v0.25).
    /// N voices in N domain hats must agree before the write commits.
    /// See `10-core/app-spec.md § Panel review`.
    #[allow(dead_code)]
    #[serde(default)]
    pub panel: Option<PanelBlock>,

    /// Nested topology for `for-each`, `sweep`, `schedule`-scoped nodes.
    /// `do:` is the body that runs per-iteration.
    #[serde(default, rename = "do")]
    pub do_: Option<Vec<Node>>,
}

impl Node {
    /// A node's invocation parameters: its `config:` and `inputs:` maps merged
    /// into one, with `inputs:` winning on a key collision. The app-spec allows
    /// either key (examples favor `inputs:`, esp. for-each `do:` bodies). The
    /// lockfile compiler and the runtime orchestrator BOTH source params through
    /// this single rule, so the compiled plan can't diverge from what actually
    /// executes. Returns `None` when the node declares neither (the lockfile then
    /// omits the field; the runtime treats it as empty).
    pub fn merged_params(&self) -> Option<Value> {
        match (self.config.is_null(), self.inputs.is_null()) {
            (true, true) => None,
            (false, true) => Some(self.config.clone()),
            (true, false) => Some(self.inputs.clone()),
            (false, false) => match (&self.config, &self.inputs) {
                (Value::Mapping(c), Value::Mapping(i)) => {
                    let mut merged = c.clone();
                    for (k, v) in i {
                        merged.insert(k.clone(), v.clone());
                    }
                    Some(Value::Mapping(merged))
                }
                // Non-mapping (unusual) — `inputs:` takes precedence.
                _ => Some(self.inputs.clone()),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PanelBlock {
    /// Quorum policy: `unanimous`, `majority`, or `quorum-N` (e.g. `quorum-3`).
    #[serde(default = "default_panel_require")]
    pub require: String,

    /// What to do when the quorum isn't met: `halt` (abort the run),
    /// `log-only` (continue + record), `warn` (continue with stderr warn).
    #[serde(rename = "on-dissent", default = "default_on_dissent")]
    pub on_dissent: String,

    /// Voices on the panel. Each is a model + voice-pack reference +
    /// optional jurisdiction tag.
    #[serde(default)]
    pub voices: Vec<Voice>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Voice {
    /// Voice id — appears in the receipt next to verdicts.
    pub id: String,
    /// LLM model name (e.g. `claude-opus-4-7`, `gpt-5`, `gemini-2-5-pro`).
    pub model: String,
    /// Reference to a voice pack — `atom://voices/<scope>/<id>@<version>`
    /// or shorthand `@<scope>/<id>@<version>`.
    #[serde(rename = "voice-pack", default)]
    pub voice_pack: Option<String>,
    /// Optional jurisdiction tag (e.g. `UK`, `US-CA`, `AU-NSW`).
    /// Surfaces in receipts; useful for filtering audits.
    #[serde(default)]
    pub jurisdiction: Option<String>,
}

fn default_panel_require() -> String {
    "unanimous".to_string()
}

fn default_on_dissent() -> String {
    "halt".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompareBlock {
    pub a: Option<String>,
    pub b: Option<String>,
    #[serde(rename = "a-snapshot")]
    pub a_snapshot: Option<String>,
    #[serde(rename = "b-snapshot")]
    pub b_snapshot: Option<String>,
    pub by: String,
    #[serde(default)]
    pub track: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SweepBlock {
    pub var: String,
    pub values: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApproveBlock {
    pub channel: String,
    pub prompt: String,
    #[serde(rename = "timeout-minutes", default)]
    pub timeout_minutes: Option<u64>,
    #[serde(default)]
    pub approvers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AssertBlock {
    pub expr: String,
    #[serde(rename = "on-fail", default)]
    pub on_fail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SnapshotBlock {
    pub of: SnapshotTarget,
    pub name: String,
    #[serde(rename = "keep-days", default)]
    pub keep_days: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SnapshotTarget {
    pub agent: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelLockBlock {
    pub agent: String,
    pub target: String,
    #[serde(rename = "acquire-timeout-seconds", default)]
    pub acquire_timeout_seconds: Option<u64>,
    #[serde(rename = "write-budget-per-second", default)]
    pub write_budget_per_second: Option<u64>,
    #[serde(rename = "on-conflict", default)]
    pub on_conflict: Option<String>,
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
    /// `atom://` URI referencing a reusable atom in `atoms/`,
    /// `20-agents/<agent>/atoms/`, or app-local atoms (v0.20).
    /// Mutually exclusive with `code:`.
    #[allow(dead_code)]
    #[serde(default)]
    pub atom: Option<String>,
    /// When `atom:` is set, this map binds the atom's named inputs to
    /// app-context expressions.
    #[allow(dead_code)]
    #[serde(default, rename = "inputs")]
    pub inline_inputs: Value,
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
    fn merged_params_unions_config_and_inputs_inputs_wins() {
        // A node declaring both keys merges them; `inputs:` overrides `config:`
        // on a shared key. Absent on both → None (lockfile omits, runtime empty).
        let node: Node = serde_yaml::from_str(
            "id: n\nconfig: { a: 1, shared: from-config }\ninputs: { b: 2, shared: from-inputs }",
        )
        .unwrap();
        let m = node.merged_params().unwrap();
        let map = m.as_mapping().unwrap();
        assert_eq!(map.get(Value::String("a".into())).unwrap(), 1);
        assert_eq!(map.get(Value::String("b".into())).unwrap(), 2);
        assert_eq!(
            map.get(Value::String("shared".into())).unwrap(),
            "from-inputs"
        );

        let only_inputs: Node = serde_yaml::from_str("id: n\ninputs: { x: 1 }").unwrap();
        assert!(only_inputs.merged_params().unwrap().as_mapping().is_some());

        let neither: Node = serde_yaml::from_str("id: n").unwrap();
        assert!(neither.merged_params().is_none());
    }

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

    // ---- v0.19 substrate-primitive parsing tests ----

    #[test]
    fn parses_assert_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: gate
    assert:
      expr: '{{ x > 0 }}'
      on-fail: 'x must be positive'
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let n = &app.nodes[0];
        assert!(n.assert.is_some());
        let a = n.assert.as_ref().unwrap();
        assert_eq!(a.expr, "{{ x > 0 }}");
        assert_eq!(a.on_fail.as_deref(), Some("x must be positive"));
    }

    #[test]
    fn parses_for_each_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: each
    for-each: '{{ projects.items }}'
    do:
      - id: inner
        agent: revit-2026
        command: sheet.list
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let n = &app.nodes[0];
        assert_eq!(n.for_each.as_deref(), Some("{{ projects.items }}"));
        let inner = n.do_.as_ref().unwrap();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].id, "inner");
    }

    #[test]
    fn parses_compare_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: diff
    compare:
      a: '{{ snap-a.rows }}'
      b: '{{ snap-b.rows }}'
      by: mark
      track: [profile, length-mm]
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let c = app.nodes[0].compare.as_ref().unwrap();
        assert_eq!(c.by, "mark");
        assert_eq!(c.track, vec!["profile", "length-mm"]);
    }

    #[test]
    fn parses_sweep_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: storeys
    sweep:
      var: n
      values: [4, 5, 6, 7, 8]
    do:
      - id: inner
        agent: tsd-26
        command: analysis.run
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let s = app.nodes[0].sweep.as_ref().unwrap();
        assert_eq!(s.var, "n");
        assert_eq!(s.values.len(), 5);
    }

    #[test]
    fn parses_approve_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: gate
    approve:
      channel: 'teams://Acme/coord'
      prompt: 'Ship?'
      timeout-minutes: 60
      approvers: [pm@acme.com]
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let a = app.nodes[0].approve.as_ref().unwrap();
        assert_eq!(a.channel, "teams://Acme/coord");
        assert_eq!(a.timeout_minutes, Some(60));
        assert_eq!(a.approvers, vec!["pm@acme.com"]);
    }

    #[test]
    fn parses_snapshot_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: snap
    snapshot:
      of:
        agent: revit-2026
        target: '{{ project.path }}'
      name: 'pre-tender-2026-05-17'
      keep-days: 90
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let s = app.nodes[0].snapshot.as_ref().unwrap();
        assert_eq!(s.of.agent, "revit-2026");
        assert_eq!(s.name, "pre-tender-2026-05-17");
        assert_eq!(s.keep_days, Some(90));
    }

    #[test]
    fn parses_model_lock_primitive() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
nodes:
  - id: lock
    model-lock:
      agent: revit-2026
      target: '{{ project.path }}'
      acquire-timeout-seconds: 60
      write-budget-per-second: 10
      on-conflict: abort
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let l = app.nodes[0].model_lock.as_ref().unwrap();
        assert_eq!(l.agent, "revit-2026");
        assert_eq!(l.acquire_timeout_seconds, Some(60));
        assert_eq!(l.on_conflict.as_deref(), Some("abort"));
    }

    #[test]
    fn parses_app_level_schedule() {
        let yaml = r#"
app: x
version: 0.0.1
description: x
schedule:
  cron: '0 7 * * MON'
  timezone: 'Europe/London'
  start-date: '2026-06-01'
nodes: []
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let s = app.schedule.as_ref().unwrap();
        assert_eq!(s.cron, "0 7 * * MON");
        assert_eq!(s.timezone, "Europe/London");
        assert_eq!(s.start_date.as_deref(), Some("2026-06-01"));
    }
}
