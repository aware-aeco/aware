//! Declared effect (#628 plan §3): what each node of a workflow is declared to
//! do, and whether a whole workflow is **declared read-only**.
//!
//! "Declared" is deliberate wording. Every judgement here comes from manifest
//! declarations — an explicit, non-overridable `mode:` — never from anything
//! observed at run time. A label built on it must say "declared read-only",
//! never "read-only" (plan §12, R1-8).
//!
//! The rule (plan §3):
//!
//! * a node is eligible as read iff its mode is `read` on basis
//!   [`ModeBasis::Declared`] under BOTH the old and the new pins; or it is an
//!   `inline` / `assert` / `compare` primitive; or it calls an app-backed agent
//!   whose inherited effect is declared read-only and whose pin does not move —
//!   AND it does not call a model at run time (`runtime-model`) AND it is not a
//!   `sweep` / `approve` / `snapshot` / `model-lock` primitive;
//! * a workflow is read-only iff EVERY dispatchable node is eligible. That is
//!   stricter than "every command on that tool is read": one `exec` defaulted to
//!   write, or one verb whose `read` was only inferred from its name, makes the
//!   whole workflow ask a person.

// Wired into `aware app migrate plan` by #628 PR2; `aware agent describe` uses
// the wrapper half today.
#![cfg_attr(not(test), allow(dead_code))]

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::manifest::App;
use crate::manifest::agent::{Mode, ModeBasis};
use crate::manifest::loader::DiscoveredAgent;

/// Why a node has the mode it has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeBasis {
    /// An agent node: the manifest's basis for the command it calls.
    Agent(ModeBasis),
    /// The node names an agent the resolved catalogue does not hold, so its
    /// command could not be judged at all.
    AgentNotResolved,
    /// `inline` / `assert` / `compare` — glue that reads values only.
    ReadPrimitive,
    /// `sweep` / `approve` / `snapshot` / `model-lock` / an unrecognised node —
    /// never eligible as read for a migration, whatever they record in a lock.
    EffectPrimitive,
    /// `for-each` with no agent: a container whose body nodes are judged on
    /// their own; it has no effect of its own.
    Container,
}

impl NodeBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            NodeBasis::Agent(basis) => basis.as_str(),
            NodeBasis::AgentNotResolved => "agent-not-resolved",
            NodeBasis::ReadPrimitive => "read-primitive",
            NodeBasis::EffectPrimitive => "effect-primitive",
            NodeBasis::Container => "container",
        }
    }
}

impl Serialize for NodeBasis {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// The declared effect of one dispatchable node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NodeEffect {
    /// Scoped node id (`parent.child` inside a `do:` body).
    pub node: String,
    pub kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    pub mode: Mode,
    pub basis: NodeBasis,
    /// The command calls a model at run time (RFC #223 `model-extraction`).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub runtime_model: bool,
    /// For an app-backed agent: the backing app whose effect this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    /// For an app-backed agent: whether the backing app is itself declared
    /// read-only (plan §3, judged against its own resolved pins).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_read_only: Option<bool>,
}

/// The effect of one app-backed agent command: the backing app's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WrapperEffect {
    pub backed_by: String,
    pub command: String,
    /// The max mode over every dispatchable node of the backing app, or `write`
    /// when its author wrote `mode: write` on the exposed command.
    pub mode: Mode,
    /// Every dispatchable node of the backing app is eligible as read.
    pub read_only: bool,
    /// Why it is not read-only, one sentence each; empty when it is.
    pub reasons: Vec<String>,
    pub nodes: Vec<NodeEffect>,
}

/// The effect of every dispatchable node of `app`, judged against `agents` —
/// the catalogue the caller resolved for ONE set of pins. Frozen subtrees are
/// skipped (a frozen node emits its pinned output and never dispatches), and
/// `do:` bodies of live nodes are descended into, exactly as
/// `validate::dispatchable_agents` walks.
///
/// `wrappers` supplies the backing-app effect of each app-backed agent the app
/// calls, keyed by agent id and command (`agent>command`); an app-backed node
/// with no entry is reported without one and is never eligible.
pub fn node_effects(
    app: &App,
    agents: &[DiscoveredAgent],
    wrappers: &BTreeMap<String, WrapperEffect>,
) -> Vec<NodeEffect> {
    let mut out = Vec::new();
    collect(&app.nodes, None, agents, wrappers, &mut out);
    out
}

/// The key [`node_effects`] looks a wrapper effect up by.
pub fn wrapper_key(agent: &str, command: &str) -> String {
    format!("{agent}>{command}")
}

fn collect(
    nodes: &[crate::manifest::app::Node],
    prefix: Option<&str>,
    agents: &[DiscoveredAgent],
    wrappers: &BTreeMap<String, WrapperEffect>,
    out: &mut Vec<NodeEffect>,
) {
    for node in nodes {
        if node.frozen.is_some() {
            continue;
        }
        let scoped = match prefix {
            Some(p) => format!("{p}.{}", node.id),
            None => node.id.clone(),
        };
        out.push(judge(node, &scoped, agents, wrappers));
        if let Some(body) = &node.do_ {
            collect(body, Some(&scoped), agents, wrappers, out);
        }
    }
}

fn judge(
    node: &crate::manifest::app::Node,
    scoped: &str,
    agents: &[DiscoveredAgent],
    wrappers: &BTreeMap<String, WrapperEffect>,
) -> NodeEffect {
    let kind = crate::app_lock::classify_node(node);
    let mut effect = NodeEffect {
        node: scoped.to_string(),
        kind,
        agent: node.agent.clone(),
        command: node.command.clone(),
        mode: Mode::Write,
        basis: NodeBasis::EffectPrimitive,
        runtime_model: false,
        inherited_from: None,
        inherited_read_only: None,
    };
    let Some(agent_id) = &node.agent else {
        (effect.mode, effect.basis) = match kind {
            "inline" | "assert" | "compare" => (Mode::Read, NodeBasis::ReadPrimitive),
            "for-each" => (Mode::Read, NodeBasis::Container),
            // Compile records `snapshot` as read; for a migration it is an
            // effect primitive all the same (plan §3).
            _ => (Mode::Write, NodeBasis::EffectPrimitive),
        };
        return effect;
    };
    let command = node.command.as_deref().unwrap_or("");
    let Some(discovered) = agents.iter().find(|d| d.manifest.agent == *agent_id) else {
        effect.basis = NodeBasis::AgentNotResolved;
        effect.mode = node.mode.unwrap_or(Mode::Write);
        return effect;
    };
    let manifest = &discovered.manifest;
    let cmd = manifest.commands.get(command);
    let resolved = manifest.mode_basis(command, cmd, node.mode);
    effect.mode = resolved.mode;
    effect.basis = NodeBasis::Agent(resolved.basis);
    effect.runtime_model = cmd.is_some_and(|c| c.model_extraction);
    if resolved.basis == ModeBasis::Inherited {
        effect.inherited_from = manifest.transport.app.as_ref().map(|t| t.backed_by.clone());
        if let Some(wrapper) = wrappers.get(&wrapper_key(agent_id, command)) {
            effect.mode = wrapper.mode;
            effect.inherited_read_only = Some(wrapper.read_only);
        }
    }
    effect
}

/// Whether ONE node, judged against one set of pins, is eligible as read.
/// `Err` carries the plain-English reason it is not.
pub fn eligible_read(effect: &NodeEffect) -> Result<(), String> {
    let who = match (&effect.agent, &effect.command) {
        (Some(agent), Some(command)) => format!("node {} ({agent} {command})", effect.node),
        _ => format!("node {} ({})", effect.node, effect.kind),
    };
    if effect.runtime_model {
        return Err(format!("{who} calls a model at run time"));
    }
    match effect.basis {
        NodeBasis::ReadPrimitive | NodeBasis::Container => Ok(()),
        NodeBasis::EffectPrimitive => Err(format!(
            "{who} is a {} step, which is never treated as read-only",
            effect.kind
        )),
        NodeBasis::AgentNotResolved => Err(format!(
            "{who}: the agent is not among the resolved pins, so its effect is unknown"
        )),
        NodeBasis::Agent(basis) => match (effect.mode, basis) {
            (Mode::Write, _) => Err(format!("{who} is write ({})", basis_phrase(basis))),
            (Mode::Read, ModeBasis::Declared) => Ok(()),
            (Mode::Read, ModeBasis::Inherited) => match effect.inherited_read_only {
                Some(true) => Ok(()),
                Some(false) => Err(format!(
                    "{who} runs app {} whose own nodes are not all declared read-only",
                    effect.inherited_from.as_deref().unwrap_or("?")
                )),
                None => Err(format!("{who} runs an app whose effect was not evaluated")),
            },
            (Mode::Read, other) => Err(format!(
                "{who} is read only by {}, not by the agent's declaration",
                basis_phrase(other)
            )),
        },
    }
}

fn basis_phrase(basis: ModeBasis) -> &'static str {
    match basis {
        ModeBasis::Declared => "declared by the agent",
        ModeBasis::OverridableDefault => "the command's overridable default",
        ModeBasis::NodeOverride => "the workflow author's own mode: on the node",
        ModeBasis::InferredName => "inference from the command name",
        ModeBasis::FallbackWrite => "the safety fallback for an unknown command",
        ModeBasis::Inherited => "the backing app's effect",
    }
}

/// The verdict of [`workflow_read_only`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReadOnlyVerdict {
    /// The workflow is declared read-only under both pins.
    pub read_only: bool,
    /// Why not, one sentence each; empty iff `read_only`.
    pub reasons: Vec<String>,
}

/// Plan §3: a workflow is declared read-only iff EVERY dispatchable node is
/// eligible as read under BOTH the old pins (`old`) and the new pins (`new`),
/// and no app-backed agent it calls moves (`moved` = the agent ids whose pin
/// differs between the two).
///
/// A node present under one set of pins and not the other cannot be judged and
/// counts against the verdict.
pub fn workflow_read_only(
    old: &[NodeEffect],
    new: &[NodeEffect],
    moved: &BTreeSet<String>,
) -> ReadOnlyVerdict {
    let mut reasons = Vec::new();
    let old_by_id: BTreeMap<&str, &NodeEffect> = old.iter().map(|e| (e.node.as_str(), e)).collect();
    let new_ids: BTreeSet<&str> = new.iter().map(|e| e.node.as_str()).collect();
    for effect in new {
        let Some(before) = old_by_id.get(effect.node.as_str()) else {
            reasons.push(format!(
                "node {} exists only under the new pins",
                effect.node
            ));
            continue;
        };
        for (side, judged) in [("old", *before), ("new", effect)] {
            if let Err(reason) = eligible_read(judged) {
                reasons.push(format!("{reason} (under the {side} pins)"));
            }
        }
        if effect.basis == NodeBasis::Agent(ModeBasis::Inherited)
            && let Some(agent) = &effect.agent
            && moved.contains(agent)
        {
            reasons.push(format!(
                "node {} runs app-backed agent {agent}, whose pin moves",
                effect.node
            ));
        }
    }
    for effect in old {
        if !new_ids.contains(effect.node.as_str()) {
            reasons.push(format!(
                "node {} exists only under the old pins",
                effect.node
            ));
        }
    }
    reasons.dedup();
    ReadOnlyVerdict {
        read_only: reasons.is_empty(),
        reasons,
    }
}

/// The inherited effect of command `command` of an app-backed agent (plan §3):
/// the max effect over EVERY dispatchable node of the backing app, each judged
/// against the backing app's own resolved pins (`backing_agents`). An author
/// who wrote `mode: write` on the exposed command makes it write outright.
///
/// The backing app runs all of its nodes whichever exposed command was called,
/// so the command only matters for that author-written mode. A backing node
/// that itself calls an app-backed agent exceeds the v0 one-hop limit and is
/// never read-only.
pub fn wrapper_effect(
    backing: &App,
    command: &str,
    backing_agents: &[DiscoveredAgent],
) -> WrapperEffect {
    let nodes = node_effects(backing, backing_agents, &BTreeMap::new());
    let mut reasons = Vec::new();
    let mut mode = nodes.iter().map(|n| n.mode).max().unwrap_or(Mode::Read);
    if backing
        .exposed_command(command)
        .and_then(|c| c.mode)
        .is_some_and(|m| m == Mode::Write)
    {
        mode = Mode::Write;
        reasons.push(format!(
            "app {} declares its exposed command {command} as mode: write",
            backing.app
        ));
    }
    for node in &nodes {
        if node.basis == NodeBasis::Agent(ModeBasis::Inherited) {
            reasons.push(format!(
                "node {} calls another app-backed agent, beyond the one-hop limit",
                node.node
            ));
            continue;
        }
        if let Err(reason) = eligible_read(node) {
            reasons.push(reason);
        }
    }
    WrapperEffect {
        backed_by: backing.app.clone(),
        command: command.to_string(),
        mode,
        read_only: reasons.is_empty() && mode == Mode::Read,
        reasons,
        nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;
    use std::path::PathBuf;

    const TOOL: &str = r#"
agent: tool
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands:
  read-declared: { lifecycle: single, description: x, mode: read }
  write-declared: { lifecycle: single, description: x, mode: write }
  exec: { lifecycle: single, description: x, mode: write, mode-overridable: true }
  list-things: { lifecycle: single, description: x }
"#;

    const WRAP: &str = r#"
agent: wrap
version: 1.0.0
description: x
stateful: false
license: app-exposed
transport: { app: { backed-by: inner } }
commands:
  run: { lifecycle: single, description: x, mode: read }
"#;

    fn agent(yaml: &str) -> DiscoveredAgent {
        DiscoveredAgent {
            manifest: serde_yaml::from_str::<Agent>(yaml).unwrap(),
            root: PathBuf::from("unused"),
        }
    }

    fn app(nodes: &str) -> App {
        serde_yaml::from_str(&format!(
            "app: a\nversion: 1.0.0\ndescription: x\nrequires: []\nnodes:\n{nodes}"
        ))
        .unwrap()
    }

    fn effects(app: &App, agents: &[DiscoveredAgent]) -> Vec<NodeEffect> {
        node_effects(app, agents, &BTreeMap::new())
    }

    #[test]
    fn only_a_declared_read_or_a_read_primitive_is_eligible() {
        let tools = [agent(TOOL)];
        let a = app("  - { id: r, agent: tool, command: read-declared }\n\
             \x20 - { id: w, agent: tool, command: write-declared }\n\
             \x20 - { id: e, agent: tool, command: exec }\n\
             \x20 - { id: eo, agent: tool, command: exec, mode: read }\n\
             \x20 - { id: i, agent: tool, command: list-things }\n\
             \x20 - { id: u, agent: tool, command: missing }\n\
             \x20 - { id: g, agent: ghost, command: x }\n\
             \x20 - id: glue\n    inline: { kind: predicate, description: pass, code: 'true' }\n");
        let all = effects(&a, &tools);
        let verdict = |id: &str| eligible_read(all.iter().find(|e| e.node == id).unwrap());
        assert!(verdict("r").is_ok());
        assert!(verdict("glue").is_ok());
        assert!(verdict("w").unwrap_err().contains("is write"));
        assert!(verdict("e").unwrap_err().contains("overridable default"));
        // The workflow author's own `mode: read` on exec is not the agent's word.
        assert!(verdict("eo").unwrap_err().contains("workflow author"));
        assert!(verdict("i").unwrap_err().contains("command name"));
        assert!(verdict("u").unwrap_err().contains("safety fallback"));
        assert!(
            verdict("g")
                .unwrap_err()
                .contains("not among the resolved pins")
        );
    }

    #[test]
    fn frozen_subtrees_are_not_dispatchable_and_do_bodies_are() {
        let tools = [agent(TOOL)];
        let a = app(
            "  - id: f\n    agent: tool\n    command: write-declared\n    frozen: { ok: true }\n\
             \x20 - id: loop\n    for-each: '{{ inputs.xs }}'\n    do:\n      - { id: inner, agent: tool, command: read-declared }\n",
        );
        let all = effects(&a, &tools);
        let ids: Vec<&str> = all.iter().map(|e| e.node.as_str()).collect();
        assert_eq!(ids, ["loop", "loop.inner"]);
        assert_eq!(all[0].basis, NodeBasis::Container);
    }

    #[test]
    fn effect_primitives_and_runtime_models_are_never_read_only() {
        let mut tool = agent(TOOL);
        tool.manifest
            .commands
            .get_mut("read-declared")
            .unwrap()
            .model_extraction = true;
        let a = app("  - { id: m, agent: tool, command: read-declared }\n\
             \x20 - id: s\n    snapshot: { of: { agent: tool, target: model }, name: snap }\n");
        let all = effects(&a, &[tool]);
        assert!(
            eligible_read(&all[0])
                .unwrap_err()
                .contains("calls a model")
        );
        assert_eq!(all[1].kind, "snapshot");
        assert!(
            eligible_read(&all[1])
                .unwrap_err()
                .contains("snapshot step")
        );
    }

    #[test]
    fn a_workflow_is_read_only_only_when_every_node_is_under_both_pins() {
        let old_tool = agent(TOOL);
        // The new version stops declaring `read-declared` — it is now inferred.
        let new_tool = agent(&TOOL.replace(
            "read-declared: { lifecycle: single, description: x, mode: read }",
            "read-declared: { lifecycle: single, description: x }",
        ));
        let a = app("  - { id: r, agent: tool, command: read-declared }\n");
        let old = effects(&a, std::slice::from_ref(&old_tool));
        let same = workflow_read_only(&old, &old, &BTreeSet::new());
        assert!(same.read_only, "{:?}", same.reasons);
        let new = effects(&a, &[new_tool]);
        let moved = workflow_read_only(&old, &new, &BTreeSet::new());
        assert!(!moved.read_only);
        assert!(
            moved.reasons[0].contains("under the new pins"),
            "{:?}",
            moved.reasons
        );
        // ...and the other way round: a read declared only by the NEW version
        // was not a declaration when the workflow was approved.
        let promoted = workflow_read_only(&new, &old, &BTreeSet::new());
        assert!(!promoted.read_only);
        assert!(
            promoted.reasons[0].contains("under the old pins"),
            "{:?}",
            promoted.reasons
        );

        // One non-eligible node anywhere flips the whole workflow.
        let b = app("  - { id: r, agent: tool, command: read-declared }\n\
             \x20 - { id: e, agent: tool, command: exec }\n");
        let both = effects(&b, &[old_tool]);
        assert!(!workflow_read_only(&both, &both, &BTreeSet::new()).read_only);
    }

    #[test]
    fn a_node_on_one_side_only_counts_against_the_verdict() {
        let tools = [agent(TOOL)];
        let one = effects(
            &app("  - { id: r, agent: tool, command: read-declared }\n"),
            &tools,
        );
        let verdict = workflow_read_only(&one, &[], &BTreeSet::new());
        assert!(!verdict.read_only);
        assert!(verdict.reasons[0].contains("only under the old pins"));
    }

    fn inner_app(nodes: &str, exposed_mode: &str) -> App {
        serde_yaml::from_str(&format!(
            "app: inner\nversion: 1.0.0\ndescription: x\nrequires: []\nexposes-as-agent: true\n\
             exposed-commands:\n  run:\n    lifecycle: single\n{exposed_mode}nodes:\n{nodes}"
        ))
        .unwrap()
    }

    #[test]
    fn a_wrapper_inherits_the_max_effect_of_its_backing_app() {
        let tools = [agent(TOOL)];
        let reads = inner_app("  - { id: r, agent: tool, command: read-declared }\n", "");
        let read = wrapper_effect(&reads, "run", &tools);
        assert_eq!(read.mode, Mode::Read);
        assert!(read.read_only, "{:?}", read.reasons);

        let mixed = inner_app(
            "  - { id: r, agent: tool, command: read-declared }\n\
             \x20 - { id: w, agent: tool, command: write-declared }\n",
            "",
        );
        let write = wrapper_effect(&mixed, "run", &tools);
        assert_eq!(write.mode, Mode::Write);
        assert!(!write.read_only);

        // An author-written `mode: write` on the exposed command wins outright.
        let authored = inner_app(
            "  - { id: r, agent: tool, command: read-declared }\n",
            "    mode: write\n",
        );
        let forced = wrapper_effect(&authored, "run", &tools);
        assert_eq!(forced.mode, Mode::Write);
        assert!(!forced.read_only);

        // Judged against the backing app's OWN pins: the same app over a tool
        // whose read is only inferred is not read-only.
        let inferred = [agent(&TOOL.replace(
            "read-declared: { lifecycle: single, description: x, mode: read }",
            "read-declared: { lifecycle: single, description: x }",
        ))];
        let weak = wrapper_effect(&reads, "run", &inferred);
        assert_eq!(weak.mode, Mode::Read);
        assert!(!weak.read_only);
    }

    #[test]
    fn a_caller_node_takes_the_wrapper_effect_and_is_refused_when_the_wrapper_moves() {
        let tools = [agent(TOOL), agent(WRAP)];
        let caller = app("  - { id: c, agent: wrap, command: run }\n");
        // No wrapper effect supplied: never eligible.
        let bare = node_effects(&caller, &tools, &BTreeMap::new());
        assert_eq!(bare[0].basis, NodeBasis::Agent(ModeBasis::Inherited));
        assert_eq!(bare[0].inherited_from.as_deref(), Some("inner"));
        assert!(
            eligible_read(&bare[0])
                .unwrap_err()
                .contains("not evaluated")
        );

        let inner = inner_app("  - { id: r, agent: tool, command: read-declared }\n", "");
        let mut wrappers = BTreeMap::new();
        wrappers.insert(
            wrapper_key("wrap", "run"),
            wrapper_effect(&inner, "run", &tools[..1]),
        );
        let judged = node_effects(&caller, &tools, &wrappers);
        assert!(eligible_read(&judged[0]).is_ok());
        assert!(workflow_read_only(&judged, &judged, &BTreeSet::new()).read_only);
        let moved: BTreeSet<String> = ["wrap".to_string()].into();
        let verdict = workflow_read_only(&judged, &judged, &moved);
        assert!(!verdict.read_only);
        assert!(verdict.reasons[0].contains("whose pin moves"));

        // A write backing app makes the caller node write.
        let writer = inner_app("  - { id: w, agent: tool, command: write-declared }\n", "");
        wrappers.insert(
            wrapper_key("wrap", "run"),
            wrapper_effect(&writer, "run", &tools[..1]),
        );
        let judged = node_effects(&caller, &tools, &wrappers);
        assert_eq!(judged[0].mode, Mode::Write);
    }

    #[test]
    fn a_backing_app_calling_another_wrapper_exceeds_one_hop() {
        let tools = [agent(TOOL), agent(WRAP)];
        let nested = inner_app("  - { id: n, agent: wrap, command: run }\n", "");
        let effect = wrapper_effect(&nested, "run", &tools);
        assert!(!effect.read_only);
        assert!(effect.reasons.iter().any(|r| r.contains("one-hop")));
    }
}
