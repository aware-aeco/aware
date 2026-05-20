//! Orchestrator: load app → build topology → execute.
//!
//! Task 10 ships the **one-shot path**: all nodes stateless, run in topo order.
//! Task 11 adds long-running (streaming sources + per-event downstream propagation).
//! Task 12 adds DAG fan-in.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::agent::Lifecycle;
use crate::manifest::app::Node;
use crate::runtime::context::RuntimeContext;
use crate::runtime::inline::eval_predicate;
use crate::runtime::invoker::AgentInvoker;
use crate::runtime::lifecycle::StopReceiver;
use crate::runtime::provenance::{ProvenanceWriter, RunEvent, now_iso};
use crate::runtime::template;

/// Per-slot input queues for fan-in nodes.
/// Outer key: node-id of the fan-in node.
/// Inner key: slot name (from `connection.input`, or the source node-id as fallback).
/// Value: FIFO queue of pending events.
type SlotMap = HashMap<String, HashMap<String, VecDeque<Value>>>;

#[derive(Default)]
pub struct FanInState {
    slots: SlotMap,
}

pub struct Orchestrator {
    pub app: App,
    pub agents_dir: PathBuf,
    pub run_id: String,
    pub instance: String,
    pub invoker: Arc<dyn AgentInvoker>,
    pub provenance: ProvenanceWriter,
    pub ctx: RuntimeContext,
    pub inputs: Value,
    pub fan_in: FanInState,
    /// When `true`, write-mode nodes skip the agent transport and emit a
    /// `would-write:` provenance event instead of mutating state. Read-mode
    /// nodes execute normally. See `10-core/app-spec.md § Safety contract`
    /// for the full semantics.
    pub dry_run: bool,
    /// When `true`, *every* node is stubbed — read-mode nodes too. Instead of
    /// contacting any host sidecar, each node yields a schema-shaped placeholder
    /// synthesized from its command's declared `output-schema`, so a whole
    /// composition can be validated end-to-end on a machine with no host app
    /// installed. Implies `dry_run` semantics for write nodes (they still emit
    /// `would-write:`). See issue #103 finding #1.
    pub simulate: bool,
}

impl Orchestrator {
    /// Run an all-stateless app in topo order. Returns on the last node's completion
    /// or on first error. Writes provenance throughout.
    pub async fn run_one_shot(mut self) -> Result<(), AwareError> {
        self.emit(RunEvent::RunStart {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            app: self.app.app.clone(),
            instance: self.instance.clone(),
            config: self.inputs.clone(),
        })
        .await?;

        let order = topo_order(&self.app)?;
        // Track which nodes were "gated out" by inline predicates so downstream nodes can skip.
        let mut gated_out: HashSet<String> = HashSet::new();
        // Track inbound predecessors so we can detect upstream-gated propagation.
        let predecessors = build_predecessors(&self.app);

        for node_id in order {
            // If any predecessor was gated out AND the predicate gate sits between this
            // node and the original predecessor, also skip. Simplest model: skip a node
            // if any direct predecessor is in `gated_out`.
            if predecessors
                .get(&node_id)
                .map(|ps| ps.iter().any(|p| gated_out.contains(p)))
                .unwrap_or(false)
            {
                gated_out.insert(node_id.clone());
                continue;
            }

            let node = self
                .app
                .nodes
                .iter()
                .find(|n| n.id == node_id)
                .unwrap()
                .clone();
            let result = self.execute_node(&node).await;
            match result {
                Ok(NodeResult::Output(v)) => {
                    self.ctx.record_output(&node.id, v.clone());
                }
                Ok(NodeResult::Gated) => {
                    gated_out.insert(node.id.clone());
                }
                Err(e) => {
                    self.emit(RunEvent::NodeError {
                        ts: now_iso(),
                        run_id: self.run_id.clone(),
                        node: node.id.clone(),
                        error: e.to_string(),
                    })
                    .await?;
                    self.emit(RunEvent::RunEnd {
                        ts: now_iso(),
                        run_id: self.run_id.clone(),
                        status: "error".into(),
                    })
                    .await?;
                    return Err(e);
                }
            }
        }

        self.emit(RunEvent::RunEnd {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            status: "ok".into(),
        })
        .await?;
        Ok(())
    }

    /// Run a long-running app: identify streaming source nodes, fan their events through
    /// downstream nodes per-event. Returns when `stop_rx` flips to `true` or all source
    /// streams have naturally closed.
    pub async fn run_long_running(mut self, mut stop_rx: StopReceiver) -> Result<(), AwareError> {
        self.emit(RunEvent::RunStart {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            app: self.app.app.clone(),
            instance: self.instance.clone(),
            config: self.inputs.clone(),
        })
        .await?;

        // 1. Identify source nodes — stateful `start` lifecycle (or `command == "watch"`
        //    when the agent manifest isn't installed, which covers the test-mock case).
        let source_ids: Vec<String> = {
            let mut ids = Vec::new();
            for node in &self.app.nodes {
                if node.agent.is_some()
                    && (node.command.as_deref() == Some("watch")
                        || self.is_stateful_start(node).await)
                {
                    ids.push(node.id.clone());
                }
            }
            ids
        };

        if source_ids.is_empty() {
            // No streaming sources — nothing to do for the long-running path.
            self.emit(RunEvent::RunEnd {
                ts: now_iso(),
                run_id: self.run_id.clone(),
                status: "ok".into(),
            })
            .await?;
            return Ok(());
        }

        // 2. Start each source stream. Collect stop senders for graceful shutdown.
        //    Move each receiver into a forwarding tokio task that pushes into a shared channel.
        let (event_tx, mut event_rx) = mpsc::channel::<(String, Value)>(64);
        let mut stop_senders: Vec<oneshot::Sender<()>> = Vec::new();

        for source_id in &source_ids {
            let node = self
                .app
                .nodes
                .iter()
                .find(|n| &n.id == source_id)
                .unwrap()
                .clone();
            let agent = node.agent.as_ref().unwrap();
            let command = node.command.as_deref().unwrap_or("");
            let args = render_config(&yaml_to_json(node.config.clone())?, &self.ctx)?;

            self.emit(RunEvent::NodeStart {
                ts: now_iso(),
                run_id: self.run_id.clone(),
                node: source_id.clone(),
                agent: node.agent.clone(),
                command: Some(command.to_string()),
            })
            .await?;

            let handle = self.invoker.invoke_stream(agent, command, args).await?;
            // Destructure — take rx into the forwarding task, keep stop for shutdown.
            let (rx, stop) = (handle.rx, handle.stop);
            stop_senders.push(stop);

            let tx = event_tx.clone();
            let sid = source_id.clone();
            tokio::spawn(async move {
                let mut rx = rx;
                while let Some(res) = rx.recv().await {
                    if let Ok(v) = res
                        && tx.send((sid.clone(), v)).await.is_err()
                    {
                        break;
                    }
                }
                // When the loop exits the tx clone is dropped, contributing to channel closure.
            });
        }

        // Drop our own clone so the channel closes when all forwarder tasks finish.
        drop(event_tx);

        // 3. Main event loop — drive until stop signal or all sources drain.
        let mut status = "ok";
        loop {
            tokio::select! {
                changed = stop_rx.changed() => {
                    if changed.is_ok() && *stop_rx.borrow() {
                        status = "interrupted";
                        break;
                    }
                }
                maybe_evt = event_rx.recv() => {
                    let Some((source_id, event)) = maybe_evt else {
                        // All forwarding tasks have exited — streams naturally ended.
                        break;
                    };

                    self.ctx.record_output(&source_id, event.clone());
                    self.emit(RunEvent::NodeOutput {
                        ts: now_iso(),
                        run_id: self.run_id.clone(),
                        node: source_id.clone(),
                        data: event.clone(),
                    })
                    .await?;

                    // Propagate through downstream chain synchronously per event.
                    if let Err(e) = self.propagate_from(&source_id, &event).await {
                        self.emit(RunEvent::NodeError {
                            ts: now_iso(),
                            run_id: self.run_id.clone(),
                            node: source_id.clone(),
                            error: e.to_string(),
                        })
                        .await?;
                        // Keep the run alive — a single downstream error doesn't tear down the watcher.
                    }
                }
            }
        }

        // 4. Send stop to all source streams (they may have already closed naturally).
        for stop in stop_senders {
            let _ = stop.send(());
            self.emit(RunEvent::NodeStop {
                ts: now_iso(),
                run_id: self.run_id.clone(),
                node: "(stream)".into(),
                reason: status.to_string(),
            })
            .await?;
        }

        self.emit(RunEvent::RunEnd {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            status: status.to_string(),
        })
        .await?;
        Ok(())
    }

    /// Propagate one event through downstream nodes starting from `source_id`.
    ///
    /// Uses a frontier queue (BFS-style) so a single source event can fan out
    /// to multiple downstream branches. Fan-in nodes (>1 incoming connection)
    /// buffer per-slot and only fire once every declared slot has at least one
    /// pending value; the oldest entry from each slot is merged into a single
    /// `{ slot: value, … }` object and passed to the fan-in node.
    async fn propagate_from(&mut self, source_id: &str, event: &Value) -> Result<(), AwareError> {
        let mut frontier: Vec<(String, Value)> = vec![(source_id.to_string(), event.clone())];

        while let Some((from_id, current_event)) = frontier.pop() {
            // All outgoing connections from this frontier node.
            let outgoing: Vec<(String, Option<String>)> = self
                .app
                .connections
                .iter()
                .filter(|c| c.from == from_id)
                .map(|c| (c.to.clone(), c.input.clone()))
                .collect();

            for (next_id, input_slot) in outgoing {
                let incoming_count = self
                    .app
                    .connections
                    .iter()
                    .filter(|c| c.to == next_id)
                    .count();
                let next_node = self
                    .app
                    .nodes
                    .iter()
                    .find(|n| n.id == next_id)
                    .unwrap()
                    .clone();

                // ── Fan-in case ──────────────────────────────────────────────
                if incoming_count > 1 {
                    let slot_name = input_slot.unwrap_or_else(|| from_id.clone());

                    // Enqueue into the appropriate slot.
                    self.fan_in
                        .slots
                        .entry(next_id.clone())
                        .or_default()
                        .entry(slot_name)
                        .or_default()
                        .push_back(current_event.clone());

                    // Collect every expected slot name (one per incoming connection).
                    let expected_slots: Vec<String> = self
                        .app
                        .connections
                        .iter()
                        .filter(|c| c.to == next_id)
                        .map(|c| c.input.clone().unwrap_or_else(|| c.from.clone()))
                        .collect();

                    // Check whether every slot has at least one pending value.
                    let slots_for_node =
                        self.fan_in.slots.get(&next_id).cloned().unwrap_or_default();
                    let all_filled = expected_slots
                        .iter()
                        .all(|s| slots_for_node.get(s).is_some_and(|q| !q.is_empty()));

                    if !all_filled {
                        continue; // Still waiting for other slots — buffer and move on.
                    }

                    // Pop the oldest entry from each slot and merge into one event.
                    let mut merged = serde_json::Map::new();
                    let node_slots = self.fan_in.slots.get_mut(&next_id).unwrap();
                    for s in &expected_slots {
                        if let Some(q) = node_slots.get_mut(s)
                            && let Some(v) = q.pop_front()
                        {
                            merged.insert(s.clone(), v);
                        }
                    }
                    let merged_event = Value::Object(merged);

                    self.execute_and_chain(&next_node, &merged_event, &mut frontier)
                        .await?;
                    continue;
                }

                // ── Linear case (single incoming connection) ─────────────────
                self.execute_and_chain(&next_node, &current_event, &mut frontier)
                    .await?;
            }
        }
        Ok(())
    }

    /// Emit NodeStart, run the node (inline predicate or agent call), emit NodeOutput,
    /// and push the result onto `frontier` for further downstream propagation.
    async fn execute_and_chain(
        &mut self,
        node: &Node,
        current_event: &Value,
        frontier: &mut Vec<(String, Value)>,
    ) -> Result<(), AwareError> {
        self.emit(RunEvent::NodeStart {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            node: node.id.clone(),
            agent: node.agent.clone(),
            command: node.command.clone(),
        })
        .await?;

        // ── Inline predicate ─────────────────────────────────────────────────
        if let Some(inline) = &node.inline {
            if inline.kind == "predicate" {
                let code = inline.code.as_deref().unwrap_or("true");
                let pass = eval_predicate(code, current_event)?;
                self.emit(RunEvent::NodeOutput {
                    ts: now_iso(),
                    run_id: self.run_id.clone(),
                    node: node.id.clone(),
                    data: serde_json::json!({ "pass": pass }),
                })
                .await?;
                if pass {
                    frontier.push((node.id.clone(), current_event.clone()));
                }
                return Ok(());
            }
            return Err(AwareError::Validation(format!(
                "inline kind {:?} not supported in propagate_from",
                inline.kind
            )));
        }

        // ── Agent invocation ──────────────────────────────────────────────────
        let agent_id = node.agent.as_ref().ok_or_else(|| {
            AwareError::Validation(format!(
                "node {} has neither agent nor inline block",
                node.id
            ))
        })?;
        let command = node.command.as_deref().unwrap_or("");

        // Record the incoming event so templates can access it via `{{ inputs.<slot> }}`.
        if let Value::Object(_) = current_event {
            self.ctx
                .upstream
                .insert("inputs".into(), current_event.clone());
        }
        self.ctx.record_output(&node.id, current_event.clone());

        let args = render_config(&yaml_to_json(node.config.clone())?, &self.ctx)?;
        let out = self.invoker.invoke_single(agent_id, command, args).await?;

        self.emit(RunEvent::NodeOutput {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            node: node.id.clone(),
            data: out.clone(),
        })
        .await?;

        self.ctx.record_output(&node.id, out.clone());
        frontier.push((node.id.clone(), out));
        Ok(())
    }

    /// Returns true when the node's agent manifest declares `stateful: true` and
    /// the referenced command has `lifecycle: start`. Agents not yet installed
    /// (missing manifest) return false — callers fall back to the `command == "watch"`
    /// heuristic for test mocks.
    async fn is_stateful_start(&self, node: &Node) -> bool {
        let agent_id = match &node.agent {
            Some(a) => a,
            None => return false,
        };
        let cmd_name = match &node.command {
            Some(c) => c,
            None => return false,
        };
        let manifest_path = self.agents_dir.join(agent_id).join("manifest.yaml");
        let Ok(m) = crate::manifest::loader::load_agent(&manifest_path) else {
            return false;
        };
        if !m.stateful {
            return false;
        }
        matches!(
            m.commands.get(cmd_name).map(|c| c.lifecycle),
            Some(Lifecycle::Start)
        )
    }

    async fn emit(&mut self, event: RunEvent) -> Result<(), AwareError> {
        self.provenance.write(&event).await
    }

    /// Resolve the read/write mode of a command by loading the agent's
    /// manifest. Falls back to `Mode::Write` for unknown agents/commands in
    /// dry-run contexts — i.e. err on the side of caution: an unknown
    /// command is treated as write-mode, so dry-run will short-circuit it
    /// rather than calling an unknown transport.
    fn lookup_command_mode(&self, agent_id: &str, command: &str) -> crate::manifest::agent::Mode {
        let mp = self.agents_dir.join(agent_id).join("manifest.yaml");
        match crate::manifest::loader::load_agent(&mp) {
            Ok(agent) => agent
                .commands
                .get(command)
                .map(|c| agent.mode_of(command, c))
                .unwrap_or(crate::manifest::agent::Mode::Write),
            Err(_) => crate::manifest::agent::Mode::Write,
        }
    }

    /// Build a schema-shaped placeholder output for a command, used by
    /// `--simulate` to stand in for a node that would otherwise call the host.
    /// Reads the command's declared `outputs` block (`{ type, schema: {...} }`)
    /// and maps each top-level field to a typed zero value. When the schema
    /// can't be resolved (uninstalled agent, untyped command), falls back to a
    /// `{ "simulated": true }` marker so the DAG still flows.
    fn synthesize_output(&self, agent_id: &str, command: &str) -> Value {
        let fallback = || serde_json::json!({ "simulated": true });
        let mp = self.agents_dir.join(agent_id).join("manifest.yaml");
        let Ok(agent) = crate::manifest::loader::load_agent(&mp) else {
            return fallback();
        };
        let Some(cmd) = agent.commands.get(command) else {
            return fallback();
        };
        let Some(schema) = cmd
            .outputs
            .as_ref()
            .and_then(|o| o.get("schema"))
            .and_then(|s| s.as_mapping())
        else {
            return fallback();
        };
        let mut obj = serde_json::Map::new();
        for (k, v) in schema {
            let Some(field) = k.as_str() else { continue };
            obj.insert(field.to_string(), placeholder_for_type(v));
        }
        if obj.is_empty() {
            fallback()
        } else {
            Value::Object(obj)
        }
    }

    async fn execute_node(&mut self, node: &Node) -> Result<NodeResult, AwareError> {
        self.emit(RunEvent::NodeStart {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            node: node.id.clone(),
            agent: node.agent.clone(),
            command: node.command.clone(),
        })
        .await?;

        if let Some(agent_id) = &node.agent {
            let command = node.command.as_deref().unwrap_or("");
            // Convert serde_yaml::Value config to serde_json::Value, then render templates.
            let config_json = yaml_to_json(node.config.clone())?;
            let args = render_config(&config_json, &self.ctx)?;

            // Dry-run / simulate short-circuit. Write-mode nodes never touch
            // the transport in either mode: they emit a `would-write:` event
            // and return a placeholder. In `--simulate`, read-mode nodes are
            // ALSO stubbed (schema-shaped placeholder, no host sidecar) so the
            // whole composition can be validated without a live app.
            if self.dry_run || self.simulate {
                let mode = self.lookup_command_mode(agent_id, command);
                if mode == crate::manifest::agent::Mode::Write {
                    let safety_block = node
                        .safety
                        .as_ref()
                        .and_then(|s| serde_json::to_value(s).ok())
                        .unwrap_or(serde_json::json!(null));
                    self.emit(RunEvent::WouldWrite {
                        ts: now_iso(),
                        run_id: self.run_id.clone(),
                        node: node.id.clone(),
                        agent: agent_id.clone(),
                        command: command.to_string(),
                        proposed_inputs: args.clone(),
                        safety: safety_block,
                    })
                    .await?;
                    // In simulate, downstream nodes may reference this write
                    // node's output, so hand them a schema-shaped value; plain
                    // dry-run keeps the lighter `{ "dry-run": true }` marker.
                    let placeholder = if self.simulate {
                        self.synthesize_output(agent_id, command)
                    } else {
                        serde_json::json!({ "dry-run": true })
                    };
                    return Ok(NodeResult::Output(placeholder));
                }

                // Read-mode node under `--simulate`: stub it rather than calling
                // the host. Emit a NodeOutput carrying the synthesized shape so
                // `aware app logs --replay` shows what each node produces.
                if self.simulate {
                    let synth = self.synthesize_output(agent_id, command);
                    self.emit(RunEvent::NodeOutput {
                        ts: now_iso(),
                        run_id: self.run_id.clone(),
                        node: node.id.clone(),
                        data: synth.clone(),
                    })
                    .await?;
                    return Ok(NodeResult::Output(synth));
                }
            }

            let output = self.invoker.invoke_single(agent_id, command, args).await?;
            self.emit(RunEvent::NodeOutput {
                ts: now_iso(),
                run_id: self.run_id.clone(),
                node: node.id.clone(),
                data: output.clone(),
            })
            .await?;
            Ok(NodeResult::Output(output))
        } else if let Some(inline) = &node.inline {
            match inline.kind.as_str() {
                "predicate" => {
                    let code = inline.code.as_deref().unwrap_or("true");
                    // Predicate gates against the most recent upstream output.
                    // For a linear topology, the immediate predecessor's output is in ctx.upstream.
                    let event_to_test = most_recent_upstream(&self.ctx, &self.app, &node.id)
                        .unwrap_or(serde_json::json!({}));
                    let pass = eval_predicate(code, &event_to_test)?;
                    self.emit(RunEvent::NodeOutput {
                        ts: now_iso(),
                        run_id: self.run_id.clone(),
                        node: node.id.clone(),
                        data: serde_json::json!({ "pass": pass }),
                    })
                    .await?;
                    if pass {
                        // Forward the upstream output through (no transformation).
                        Ok(NodeResult::Output(event_to_test))
                    } else {
                        Ok(NodeResult::Gated)
                    }
                }
                other => Err(AwareError::Validation(format!(
                    "inline kind {other:?} not supported in v0.3 (only 'predicate')"
                ))),
            }
        } else if let Some(assert) = &node.assert {
            // v0.19 — `assert:` primitive. Evaluate the expr; abort the run if false.
            let pass = eval_predicate(&assert.expr, &serde_json::json!({}))?;
            self.emit(RunEvent::NodeOutput {
                ts: now_iso(),
                run_id: self.run_id.clone(),
                node: node.id.clone(),
                data: serde_json::json!({ "asserted": pass }),
            })
            .await?;
            if pass {
                Ok(NodeResult::Output(serde_json::json!({ "asserted": true })))
            } else {
                let msg = assert
                    .on_fail
                    .clone()
                    .unwrap_or_else(|| format!("assertion failed: {}", assert.expr));
                Err(AwareError::Validation(msg))
            }
        } else if node.for_each.is_some()
            || node.compare.is_some()
            || node.sweep.is_some()
            || node.approve.is_some()
            || node.snapshot.is_some()
            || node.model_lock.is_some()
        {
            // Substrate primitives parsed by v0.19 — runtime implementation
            // lands phase-by-phase in v0.19.x patches. For now, return a
            // clear "not yet implemented" error pointing at the spec section.
            let primitive = if node.for_each.is_some() {
                "for-each"
            } else if node.compare.is_some() {
                "compare"
            } else if node.sweep.is_some() {
                "sweep"
            } else if node.approve.is_some() {
                "approve"
            } else if node.snapshot.is_some() {
                "snapshot"
            } else {
                "model-lock"
            };
            Err(AwareError::Validation(format!(
                "substrate primitive `{primitive}` in node {} — parsing works (spec § Substrate primitives); runtime execution lands in v0.19.x patches",
                node.id
            )))
        } else {
            Err(AwareError::Validation(format!(
                "node {} has no agent, no inline block, and no substrate primitive",
                node.id
            )))
        }
    }
}

enum NodeResult {
    Output(Value),
    Gated,
}

fn topo_order(app: &App) -> Result<Vec<String>, AwareError> {
    // Kahn's algorithm.
    let mut indegree: HashMap<&str, usize> = app.nodes.iter().map(|n| (n.id.as_str(), 0)).collect();
    for c in &app.connections {
        *indegree.entry(c.to.as_str()).or_default() += 1;
    }
    let mut queue: std::collections::VecDeque<&str> = indegree
        .iter()
        .filter_map(|(id, deg)| if *deg == 0 { Some(*id) } else { None })
        .collect();
    let mut out = Vec::new();
    while let Some(id) = queue.pop_front() {
        out.push(id.to_string());
        for c in app.connections.iter().filter(|c| c.from == id) {
            let entry = indegree.get_mut(c.to.as_str()).unwrap();
            *entry -= 1;
            if *entry == 0 {
                queue.push_back(c.to.as_str());
            }
        }
    }
    if out.len() != app.nodes.len() {
        return Err(AwareError::Validation(
            "cycle in connections (caught by orchestrator topo sort)".into(),
        ));
    }
    Ok(out)
}

fn build_predecessors(app: &App) -> HashMap<String, Vec<String>> {
    let mut preds: HashMap<String, Vec<String>> = HashMap::new();
    for c in &app.connections {
        preds.entry(c.to.clone()).or_default().push(c.from.clone());
    }
    preds
}

/// Convert a `serde_yaml::Value` to a `serde_json::Value` by round-tripping
/// through JSON serialisation. `serde_yaml::Value` implements `Serialize`, so
/// this is always lossless for the subset of YAML we use (no tags, no anchors).
fn yaml_to_json(v: serde_yaml::Value) -> Result<Value, AwareError> {
    let s = serde_json::to_string(&v)?;
    let j: Value = serde_json::from_str(&s)?;
    Ok(j)
}

fn render_config(config: &Value, ctx: &RuntimeContext) -> Result<Value, AwareError> {
    // Recursively walk the config object. For each string leaf, run it through the templater.
    // Non-string leaves pass through unchanged.
    fn walk(v: &Value, ctx: &RuntimeContext) -> Result<Value, AwareError> {
        match v {
            Value::String(s) => {
                let rendered = template::render(s, ctx)?;
                Ok(Value::String(rendered))
            }
            Value::Object(map) => {
                let mut out = serde_json::Map::new();
                for (k, sub) in map {
                    out.insert(k.clone(), walk(sub, ctx)?);
                }
                Ok(Value::Object(out))
            }
            Value::Array(arr) => {
                let mut out = Vec::new();
                for sub in arr {
                    out.push(walk(sub, ctx)?);
                }
                Ok(Value::Array(out))
            }
            other => Ok(other.clone()),
        }
    }
    walk(config, ctx)
}

/// Map an `output-schema` field's declared type to a typed zero placeholder
/// for `--simulate`. The type can be a bare string (`field: string`) or a
/// mapping carrying a `type:` key (`field: { type: string, ... }`). Unknown or
/// missing types fall back to a string placeholder.
fn placeholder_for_type(decl: &serde_yaml::Value) -> Value {
    let type_name = match decl {
        serde_yaml::Value::String(s) => s.as_str(),
        serde_yaml::Value::Mapping(_) => decl.get("type").and_then(|t| t.as_str()).unwrap_or(""),
        _ => "",
    };
    match type_name {
        "number" | "integer" | "int" | "float" => serde_json::json!(0),
        "boolean" | "bool" => serde_json::json!(false),
        "array" | "list" => serde_json::json!([]),
        "object" | "map" | "mapping" => serde_json::json!({}),
        _ => Value::String("<simulated>".to_string()),
    }
}

fn most_recent_upstream(ctx: &RuntimeContext, app: &App, node_id: &str) -> Option<Value> {
    // Find this node's predecessors; return the first one with output in ctx.
    for c in &app.connections {
        if c.to == node_id
            && let Some(out) = ctx.upstream.get(&c.from)
        {
            return Some(out.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;
    use crate::runtime::invoker::MockInvoker;
    use crate::runtime::lifecycle::stop_channel;
    use crate::runtime::provenance::{log_path_for, read_run_events};

    async fn make_orchestrator(
        app: App,
        invoker: Arc<dyn AgentInvoker>,
    ) -> (Orchestrator, tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(&aware).unwrap();
        let paths = Paths {
            aware_home: aware.clone(),
        };
        let run_id = "r_test".to_string();
        let log_path = log_path_for(&paths.logs_dir(), &app.app, "default", &run_id);
        let prov = ProvenanceWriter::open(&log_path).await.unwrap();
        let orch = Orchestrator {
            app,
            agents_dir: paths.agents_dir(),
            run_id,
            instance: "default".into(),
            invoker,
            provenance: prov,
            ctx: RuntimeContext::default(),
            inputs: serde_json::json!({}),
            fan_in: FanInState::default(),
            dry_run: false,
            simulate: false,
        };
        (orch, tmp, log_path)
    }

    #[tokio::test]
    async fn one_shot_two_stateless_agents() {
        // Synthetic 2-node linear app: a→b. Both stateless agent calls.
        let app: App = serde_yaml::from_str(
            r#"
app: tiny
version: 0.1.0
description: x
nodes:
  - id: a
    agent: ag-a
    command: do-a
  - id: b
    agent: ag-b
    command: do-b
connections:
  - from: a
    to: b
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single("ag-a", "do-a", serde_json::json!({"value":"from-a"}))
                .with_single("ag-b", "do-b", serde_json::json!({"value":"from-b"})),
        );

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        // RunStart, NodeStart a, NodeOutput a, NodeStart b, NodeOutput b, RunEnd
        assert_eq!(events.len(), 6);
        if let RunEvent::RunEnd { status, .. } = events.last().unwrap() {
            assert_eq!(status, "ok");
        } else {
            panic!();
        }
    }

    #[tokio::test]
    async fn simulate_stubs_read_node_with_schema_shape() {
        // A single read-mode node. In `--simulate` the orchestrator must NOT
        // call the invoker (an empty MockInvoker would error if it did) and
        // must emit a NodeOutput shaped from the command's output-schema.
        let app: App = serde_yaml::from_str(
            r#"
app: sim
version: 0.1.0
description: x
nodes:
  - id: r
    agent: ag-read
    command: report
connections: []
requires: []
"#,
        )
        .unwrap();

        // Empty invoker: any real call surfaces as NotFound and fails the run.
        let inv = Arc::new(MockInvoker::new());
        let (mut orch, tmp, log_path) = make_orchestrator(app, inv).await;

        // Install a manifest declaring `report` as a read command with a typed
        // output-schema, so mode-lookup + synthesis resolve from disk.
        let agent_dir = tmp.path().join("aware").join("agents").join("ag-read");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            r#"agent: ag-read
version: 0.0.1
description: x
stateful: false
license: MIT
transport: { cli: { binary: this-binary-does-not-exist } }
commands:
  report:
    lifecycle: single
    category: curated
    mode: read
    description: emits a report path
    outputs:
      type: single
      schema:
        path: string
        row-count: number
"#,
        )
        .unwrap();

        orch.simulate = true;
        // Must succeed despite no host binary and an empty invoker.
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let output = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "r" => Some(data.clone()),
                _ => None,
            })
            .expect("read node must emit a synthesized NodeOutput");
        assert_eq!(output["path"], serde_json::json!("<simulated>"));
        assert_eq!(output["row-count"], serde_json::json!(0));
        if let RunEvent::RunEnd { status, .. } = events.last().unwrap() {
            assert_eq!(status, "ok");
        } else {
            panic!("run did not end cleanly");
        }
    }

    #[test]
    fn placeholder_for_type_maps_each_kind() {
        let s = |y: &str| serde_yaml::from_str::<serde_yaml::Value>(y).unwrap();
        assert_eq!(placeholder_for_type(&s("number")), serde_json::json!(0));
        assert_eq!(
            placeholder_for_type(&s("boolean")),
            serde_json::json!(false)
        );
        assert_eq!(placeholder_for_type(&s("array")), serde_json::json!([]));
        assert_eq!(placeholder_for_type(&s("object")), serde_json::json!({}));
        assert_eq!(
            placeholder_for_type(&s("string")),
            serde_json::json!("<simulated>")
        );
        // Unknown type → string placeholder.
        assert_eq!(
            placeholder_for_type(&s("guid")),
            serde_json::json!("<simulated>")
        );
        // Mapping form `{ type: number, description: ... }` reads `type`.
        assert_eq!(
            placeholder_for_type(&s("type: number\ndescription: a count")),
            serde_json::json!(0)
        );
    }

    #[tokio::test]
    async fn predicate_gating_blocks_downstream() {
        // a → predicate (gates false) → b. b must NOT run.
        let app: App = serde_yaml::from_str(
            r#"
app: gated
version: 0.1.0
description: x
nodes:
  - id: a
    agent: ag-a
    command: emit
  - id: gate
    inline:
      kind: predicate
      description: drop everything
      code: "e => e.type == \"NeverMatches\""
  - id: b
    agent: ag-b
    command: do-b
connections:
  - from: a
    to: gate
  - from: gate
    to: b
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single("ag-a", "emit", serde_json::json!({"type":"Other"}))
                .with_single(
                    "ag-b",
                    "do-b",
                    serde_json::json!({"value":"should-not-appear"}),
                ),
        );

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let saw_b_start = events
            .iter()
            .any(|e| matches!(e, RunEvent::NodeStart { node, .. } if node == "b"));
        assert!(
            !saw_b_start,
            "b should not have started; events: {events:?}"
        );
    }

    #[tokio::test]
    async fn long_running_streams_three_events_through_filter() {
        let app: App = serde_yaml::from_str(
            r#"
app: stream-test
version: 0.1.0
description: x
nodes:
  - id: watch
    agent: ag-watch
    command: watch
  - id: gate
    inline:
      kind: predicate
      description: pass only Welded
      code: "e => e.type == \"Welded\""
  - id: sink
    agent: ag-sink
    command: upload
connections:
  - from: watch
    to: gate
  - from: gate
    to: sink
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_stream(
                    "ag-watch",
                    "watch",
                    vec![
                        serde_json::json!({"type":"Welded","mark":"A"}),
                        serde_json::json!({"type":"Bolted","mark":"B"}),
                        serde_json::json!({"type":"Welded","mark":"C"}),
                    ],
                )
                .with_single("ag-sink", "upload", serde_json::json!({"received":"ok"})),
        );

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        let (stop_tx, stop_rx) = stop_channel();

        // Spawn the orchestrator; it naturally ends when the 3-event stream drains.
        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        // Give it time to drain all 3 events (each mock event has a 5 ms delay).
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let _ = stop_tx.send(true);
        run_handle.await.unwrap().unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        // Sink must fire exactly twice — once per Welded event; Bolted is gated out.
        let sink_outputs = events
            .iter()
            .filter(|e| matches!(e, RunEvent::NodeOutput { node, .. } if node == "sink"))
            .count();
        assert_eq!(
            sink_outputs, 2,
            "expected 2 sink outputs (Bolted filtered), events: {events:?}"
        );
    }

    #[tokio::test]
    async fn long_running_ctrl_c_interrupts_mid_stream() {
        let app: App = serde_yaml::from_str(
            r#"
app: interruptable
version: 0.1.0
description: x
nodes:
  - id: watch
    agent: ag-watch
    command: watch
connections: []
requires: []
"#,
        )
        .unwrap();

        // 100-event stream — far more than we'll drain before the stop signal.
        let outputs: Vec<_> = (0..100).map(|i| serde_json::json!({"i": i})).collect();
        let inv = Arc::new(MockInvoker::new().with_stream("ag-watch", "watch", outputs));

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        let (stop_tx, stop_rx) = stop_channel();

        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        stop_tx.send(true).unwrap();
        run_handle.await.unwrap().unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let run_end = events
            .iter()
            .rev()
            .find(|e| matches!(e, RunEvent::RunEnd { .. }))
            .expect("RunEnd event missing");
        if let RunEvent::RunEnd { status, .. } = run_end {
            assert_eq!(status, "interrupted");
        }
    }

    #[tokio::test]
    async fn templating_substitutes_upstream_output() {
        // a emits {"mark":"A-104"}; b consumes config { tag: "{{ a.mark }}" }
        let app: App = serde_yaml::from_str(
            r#"
app: tmpl
version: 0.1.0
description: x
nodes:
  - id: a
    agent: ag-a
    command: emit
  - id: b
    agent: ag-b
    command: consume
    config:
      tag: "{{ a.mark }}"
connections:
  - from: a
    to: b
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single("ag-a", "emit", serde_json::json!({"mark":"A-104"}))
                .with_single("ag-b", "consume", serde_json::json!({"received":"ok"})),
        );

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        // The mock invoker doesn't inspect args — we have to verify via the trace that
        // the orchestrator at least RAN b. For a deeper "did the template render" check,
        // future tests can use a custom invoker that captures args.
        let events = read_run_events(&log_path).await.unwrap();
        let saw_b_output = events
            .iter()
            .any(|e| matches!(e, RunEvent::NodeOutput { node, .. } if node == "b"));
        assert!(saw_b_output);
    }

    #[tokio::test]
    async fn dag_fan_in_pairs_oldest_entries() {
        // pdf-watch + excel-watch → match-build → sink
        // match-build is a fan-in node: it needs one value from each upstream.
        // pdf-watch emits 2 events; excel-watch emits 1.
        // match-build should fire exactly once (paired with the first pdf event).
        let app: App = serde_yaml::from_str(
            r#"
app: fan-in-test
version: 0.1.0
description: x
nodes:
  - id: pdf-watch
    agent: ag-pdf
    command: watch
  - id: excel-watch
    agent: ag-excel
    command: watch
  - id: match-build
    agent: ag-match
    command: pair
  - id: sink
    agent: ag-sink
    command: store
connections:
  - from: pdf-watch
    to: match-build
    input: drawing
  - from: excel-watch
    to: match-build
    input: spec
  - from: match-build
    to: sink
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_stream(
                    "ag-pdf",
                    "watch",
                    vec![
                        serde_json::json!({"mark": "A", "kind": "pdf"}),
                        serde_json::json!({"mark": "B", "kind": "pdf"}),
                    ],
                )
                .with_stream(
                    "ag-excel",
                    "watch",
                    vec![serde_json::json!({"mark": "A", "kind": "spec"})],
                )
                .with_single("ag-match", "pair", serde_json::json!({"paired": "ok"}))
                .with_single("ag-sink", "store", serde_json::json!({"stored": "ok"})),
        );

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        let (stop_tx, stop_rx) = stop_channel();

        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        // Give streams time to drain (each mock event has a 5 ms delay; 2+1 events = ~30 ms).
        // The run may end naturally before 200 ms; ignore send error if the receiver is gone.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let _ = stop_tx.send(true);
        let _ = run_handle.await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        // match-build should fire exactly once (only 1 spec available; pairs with first pdf).
        let match_outputs = events
            .iter()
            .filter(|e| matches!(e, RunEvent::NodeOutput { node, .. } if node == "match-build"))
            .count();
        assert_eq!(
            match_outputs, 1,
            "expected 1 match-build output; events: {events:?}"
        );
    }
}
