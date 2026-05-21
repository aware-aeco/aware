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
use crate::manifest::app::{CompareBlock, Node};
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
            let args = render_config(
                &yaml_to_json(node.merged_params().unwrap_or(serde_yaml::Value::Null))?,
                &self.ctx,
            )?;

            self.emit(RunEvent::NodeStart {
                ts: now_iso(),
                run_id: self.run_id.clone(),
                node: source_id.clone(),
                agent: node.agent.clone(),
                command: Some(command.to_string()),
            })
            .await?;

            if self.simulate {
                // Stub the stream under `--simulate`: emit ONE schema-shaped
                // placeholder event, then let the source close. Watcher
                // compositions (`lifecycle: start`) thus validate end-to-end
                // without `invoke_stream`, which isn't implemented yet (#117-2).
                // The placeholder propagates through the downstream chain once.
                let placeholder = self.synthesize_output(agent, command);
                let tx = event_tx.clone();
                let sid = source_id.clone();
                tokio::spawn(async move {
                    let _ = tx.send((sid, placeholder)).await;
                    // tx clone dropped here → contributes to channel closure.
                });
            } else {
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

        // ── for-each (substrate primitive) ────────────────────────────────────
        // A streaming source can feed a `for-each` node; its work lives in `do:`,
        // not an `agent`/`inline` block, so the agent path below would reject it.
        // Run the loop body per item (binding `{{ item }}`) and forward the
        // aggregate downstream like any other node output (#124).
        if let Some(expr) = node.for_each.clone() {
            if let NodeResult::Output(aggregate) = self.run_for_each(node, &expr).await? {
                self.ctx.record_output(&node.id, aggregate.clone());
                frontier.push((node.id.clone(), aggregate));
            }
            return Ok(());
        }

        // ── compare (substrate primitive) ─────────────────────────────────────
        // A streaming source can feed a `compare` node — diff the two lists its
        // event (or earlier upstream outputs) carry. Like for-each its work
        // isn't an `agent`/`inline` call, so run it here and forward the diff
        // downstream (#127).
        if let Some(cmp) = node.compare.clone() {
            // A fan-in `compare` (>1 incoming) receives the paired buffered event
            // as `current_event`, keyed by slot (the connection `input:` name,
            // defaulting to the source node id). Resolve `a`/`b` against a SCOPED
            // context that overlays each paired side under both its slot name and
            // its source node id, so either reference style sees the buffered
            // pair — not whatever each source last emitted globally (Codex P2:
            // out-of-order arrivals would otherwise mis-pair). The overlay is
            // discarded after, so the slot aliases never leak into later nodes
            // and the app `inputs` namespace is untouched.
            let incoming = self
                .app
                .connections
                .iter()
                .filter(|c| c.to == node.id)
                .count();
            let merged = if incoming > 1 {
                current_event.as_object()
            } else {
                None
            };
            let scoped = self.compare_resolve_ctx(node, merged);
            let saved = std::mem::replace(&mut self.ctx, scoped);
            let result = self.run_compare(node, &cmp).await;
            self.ctx = saved;
            if let NodeResult::Output(diff) = result? {
                self.ctx.record_output(&node.id, diff.clone());
                frontier.push((node.id.clone(), diff));
            }
            return Ok(());
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

        let args = render_config(
            &yaml_to_json(node.merged_params().unwrap_or(serde_yaml::Value::Null))?,
            &self.ctx,
        )?;

        // Dry-run / simulate short-circuit — mirrors `execute_node` so that
        // streaming (watcher) compositions validate end-to-end without touching
        // the host transport (#117-2). Write nodes emit `would-write:`; under
        // `--simulate` read nodes are stubbed with a schema-shaped placeholder.
        let out = if self.dry_run || self.simulate {
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
                // Downstream nodes may reference this write node's output, so in
                // simulate hand them a schema-shaped value; plain dry-run keeps the
                // lighter marker. A would-write node emits no NodeOutput.
                let placeholder = if self.simulate {
                    self.synthesize_output(agent_id, command)
                } else {
                    serde_json::json!({ "dry-run": true })
                };
                self.ctx.record_output(&node.id, placeholder.clone());
                frontier.push((node.id.clone(), placeholder));
                return Ok(());
            }
            // Read-mode node: stub under `--simulate`; under plain dry-run a read
            // is safe, so run it for real.
            if self.simulate {
                self.synthesize_output(agent_id, command)
            } else {
                self.invoker.invoke_single(agent_id, command, args).await?
            }
        } else {
            self.invoker.invoke_single(agent_id, command, args).await?
        };

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
            // Source params via the shared config:+inputs: merge rule, then
            // convert serde_yaml::Value → serde_json::Value and render templates.
            let config_json =
                yaml_to_json(node.merged_params().unwrap_or(serde_yaml::Value::Null))?;
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
                    // Inside a `for-each` `do:` body the predicate has no graph
                    // predecessor (the body topology is implicit), so it gates on
                    // the bound `{{ item }}` — a per-item filter (#124).
                    let event_to_test = most_recent_upstream(&self.ctx, &self.app, &node.id)
                        .or_else(|| self.ctx.upstream.get("item").cloned())
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
        } else if let Some(expr) = node.for_each.clone() {
            // `for-each` — iterate the resolved collection, binding `{{ item }}`
            // per element, and run the `do:` body once per item (#124).
            self.run_for_each(node, &expr).await
        } else if let Some(cmp) = node.compare.clone() {
            // `compare` — diff two collections by an identity key (#127). Resolve
            // `a`/`b` against a SCOPED context that overlays each incoming
            // connection's predecessor output under its declared `input:` slot
            // name, so a one-shot fan-in compare can reference sides by slot
            // (`{{ left.rows }}` for `input: left`) — parity with the streaming
            // path (Codex P2). The overlay is discarded after run_compare, so a
            // slot alias never leaks into (or clobbers) a real node's upstream
            // output for later nodes (Codex P3).
            let scoped = self.compare_resolve_ctx(node, None);
            let saved = std::mem::replace(&mut self.ctx, scoped);
            let result = self.run_compare(node, &cmp).await;
            self.ctx = saved;
            result
        } else if node.sweep.is_some()
            || node.approve.is_some()
            || node.snapshot.is_some()
            || node.model_lock.is_some()
        {
            // Substrate primitives parsed by v0.19 — runtime implementation
            // lands phase-by-phase in v0.19.x patches. For now, return a
            // clear "not yet implemented" error pointing at the spec section.
            let primitive = if node.sweep.is_some() {
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

    /// `for-each` runtime (#124). Resolve the iteration collection from the
    /// node's expression, bind `{{ item }}` per element, and run the `do:` body
    /// once per item. The aggregate output is the array of each iteration's
    /// final body-node output, recorded under the for-each node's id (by the
    /// caller) so downstream nodes can consume it.
    ///
    /// Under `--simulate` an empty or unresolved collection still runs the body
    /// once — the upstream source was itself stubbed and may legitimately yield
    /// nothing, yet the composition must validate end-to-end without a live host.
    async fn run_for_each(&mut self, node: &Node, expr: &str) -> Result<NodeResult, AwareError> {
        // Save any enclosing `item` binding before resolving the collection (the
        // collection expr may itself reference the outer `{{ item }}`) so the
        // outer per-iteration variable is restored when this loop returns. Nested
        // for-each loops share one `ctx`; without this, an inner loop's last
        // element would leak into a sibling body node's `{{ item }}` — a topology
        // the compiler explicitly scopes (app_lock `nested_body_keeps_outer_iteration_var_in_scope`).
        let prev_item = self.ctx.upstream.get("item").cloned();

        let mut collection = match template::resolve_value(expr, &self.ctx) {
            Value::Array(items) => items,
            Value::Null => Vec::new(),
            // A non-array, non-null value iterates once over itself.
            other => vec![other],
        };
        if collection.is_empty() && self.simulate {
            collection.push(serde_json::json!({ "simulated": true }));
        }

        let body = node.do_.clone().unwrap_or_default();
        let mut results: Vec<Value> = Vec::with_capacity(collection.len());
        for item in collection {
            // Bind the per-iteration variable `{{ item }}` — the reserved
            // for-each prefix the compiler scopes inside the body (#117-3).
            self.ctx.record_output("item", item);
            let mut iter_output = Value::Null;
            let mut gated = false;
            for body_node in &body {
                // Box the recursive call: `execute_node` → `run_for_each` →
                // `execute_node` is a cycle that needs a heap indirection to size.
                match Box::pin(self.execute_node(body_node)).await? {
                    NodeResult::Output(v) => {
                        self.ctx.record_output(&body_node.id, v.clone());
                        iter_output = v;
                    }
                    NodeResult::Gated => {
                        // An inline predicate in the body filters this item out:
                        // skip the rest of the body and drop the item from the
                        // aggregate (for-each + predicate = filter+map), mirroring
                        // top-level predicate gating which skips downstream nodes.
                        gated = true;
                        break;
                    }
                }
            }
            if !gated {
                results.push(iter_output);
            }
        }

        // Restore the enclosing `item` (or clear it at the top level) so a sibling
        // body node after a nested loop — or a connection-less downstream
        // predicate — never reads this loop's last element as `{{ item }}`.
        match prev_item {
            Some(v) => {
                self.ctx.upstream.insert("item".to_string(), v);
            }
            None => {
                self.ctx.upstream.remove("item");
            }
        }

        let aggregate = Value::Array(results);
        self.emit(RunEvent::NodeOutput {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            node: node.id.clone(),
            data: aggregate.clone(),
        })
        .await?;
        Ok(NodeResult::Output(aggregate))
    }

    /// Build the scoped context a `compare` node resolves `a`/`b` against. It is
    /// a clone of the live context with each incoming connection's paired side
    /// overlaid under BOTH its slot name (the connection `input:`, defaulting to
    /// the source id) and its source node id, so either reference style resolves
    /// to the right side. `merged` carries the paired fan-in event in the
    /// streaming path (value per slot); in the one-shot path it is `None` and the
    /// value is the predecessor's already-recorded output. The clone is discarded
    /// by the caller after resolving, so these aliases never leak into — or
    /// clobber — the live context that later nodes read (Codex P2/P3).
    fn compare_resolve_ctx(
        &self,
        node: &Node,
        merged: Option<&serde_json::Map<String, Value>>,
    ) -> RuntimeContext {
        let mut scoped = self.ctx.clone();
        for c in self.app.connections.iter().filter(|c| c.to == node.id) {
            let slot = c.input.clone().unwrap_or_else(|| c.from.clone());
            let value = match merged {
                Some(m) => m.get(&slot).cloned(),
                None => self.ctx.upstream.get(&c.from).cloned(),
            };
            if let Some(v) = value {
                scoped.record_output(&slot, v.clone());
                if c.from != slot {
                    scoped.record_output(&c.from, v);
                }
            }
        }
        scoped
    }

    /// `compare` runtime (#127). Resolve the `a` / `b` collections to structured
    /// arrays and diff them by the `by` identity key, emitting a
    /// `{ added, removed, changed }` NodeOutput (app-spec § Substrate
    /// primitives). `compare` is a pure, host-free computation, so it runs
    /// identically under `--dry-run` / `--simulate` — no transport, no
    /// `would-write:`. The caller records the output under the node id.
    ///
    /// Reference style is uniform across paths: `a`/`b` reference their sides by
    /// id/slot (`{{ today-export.rows }}`). The caller installs a scoped context
    /// (see [`Orchestrator::compare_resolve_ctx`]) so a fan-in compare reads the
    /// buffered pair under either the slot name or the source id, while the app
    /// `inputs` namespace is never shadowed — so either side may also reference a
    /// run-supplied `{{ inputs.<name> }}` baseline.
    async fn run_compare(
        &mut self,
        node: &Node,
        cmp: &CompareBlock,
    ) -> Result<NodeResult, AwareError> {
        // Snapshot-keyed compare reads named artifacts the `snapshot` primitive
        // produces, but that runtime isn't implemented yet — so reject ANY
        // snapshot key (alone, paired, or alongside an inline `a`/`b`) up front
        // rather than silently ignoring it in favor of an inline ref. Replace
        // this with real snapshot resolution when the `snapshot` primitive lands.
        if cmp.a_snapshot.is_some() || cmp.b_snapshot.is_some() {
            return Err(AwareError::Validation(format!(
                "compare node {} uses a-snapshot/b-snapshot — snapshot-backed compare needs the `snapshot` primitive runtime (spec § Substrate primitives), not yet implemented",
                node.id
            )));
        }

        // `compare` is a two-sided diff: with snapshots rejected above, both
        // inline sides must be present. A missing side is a malformed app — fail
        // rather than diff against an empty collection (which would report every
        // record on the present side as added/removed).
        let (Some(a_expr), Some(b_expr)) = (cmp.a.as_deref(), cmp.b.as_deref()) else {
            return Err(AwareError::Validation(format!(
                "compare node {} is missing an inline side — `compare` needs both `a` and `b` (spec § Substrate primitives)",
                node.id
            )));
        };
        let a_val = template::resolve_value(a_expr, &self.ctx);
        let b_val = template::resolve_value(b_expr, &self.ctx);
        // A side that doesn't resolve (missing node/field — e.g. a typo'd ref)
        // is malformed: compare refs are required and aren't compile-checked
        // like for-each's, so reject an unresolved side rather than coercing it
        // to an empty list and emitting a false diff (every record on the other
        // side reported as added/removed). Under `--simulate` the upstream is
        // stubbed and may legitimately yield nothing, so tolerate Null there
        // (mirrors run_for_each's simulate leniency). A resolved-but-empty array
        // is a valid input and is left alone.
        if !self.simulate {
            for (side, expr, val) in [("a", a_expr, &a_val), ("b", b_expr, &b_val)] {
                if val.is_null() {
                    return Err(AwareError::Validation(format!(
                        "compare node {} side `{side}` ({expr}) did not resolve — check the reference",
                        node.id
                    )));
                }
            }
        }
        let a = collection_of(a_val);
        let b = collection_of(b_val);

        let diff = diff_records(&a, &b, &cmp.by, &cmp.track);
        self.emit(RunEvent::NodeOutput {
            ts: now_iso(),
            run_id: self.run_id.clone(),
            node: node.id.clone(),
            data: diff.clone(),
        })
        .await?;
        Ok(NodeResult::Output(diff))
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

/// Coerce a resolved `{{ … }}` value into a list of records for `compare`:
/// an array is taken as-is, null (an unresolved ref) is an empty list, any
/// other scalar/object iterates once over itself.
fn collection_of(v: Value) -> Vec<Value> {
    match v {
        Value::Array(items) => items,
        Value::Null => Vec::new(),
        other => vec![other],
    }
}

/// Diff two record lists by an identity key, per `10-core/app-spec.md
/// § Substrate primitives` (`compare`). Returns `{ added, removed, changed }`:
/// `added` = records whose `by` value appears in `b` but not `a`; `removed` =
/// the reverse; `changed` = records present in both whose tracked fields
/// differ, each shaped `{ <by>: <id>, diffs: { <field>: { from, to } } }`.
/// Records lacking the `by` key are ignored; on a duplicate `by` value the
/// first record wins. Order is stable: added/removed follow first appearance
/// on their side, changed follows `b` order.
fn diff_records(a: &[Value], b: &[Value], by: &str, track: &[String]) -> Value {
    // Stable map-key for an identity value: its compact JSON form, so a string
    // "5" and a number 5 are distinct keys rather than colliding on "5".
    fn id_key(v: &Value) -> String {
        v.to_string()
    }
    // Ordered index of `by`-value → first record carrying it.
    fn index(records: &[Value], by: &str) -> (Vec<String>, HashMap<String, Value>) {
        let mut order = Vec::new();
        let mut map: HashMap<String, Value> = HashMap::new();
        for rec in records {
            let Some(id) = rec.get(by) else { continue };
            let key = id_key(id);
            if let std::collections::hash_map::Entry::Vacant(slot) = map.entry(key.clone()) {
                order.push(key);
                slot.insert(rec.clone());
            }
        }
        (order, map)
    }

    let (a_order, a_map) = index(a, by);
    let (b_order, b_map) = index(b, by);

    let added: Vec<Value> = b_order
        .iter()
        .filter(|k| !a_map.contains_key(*k))
        .map(|k| b_map[k].clone())
        .collect();
    let removed: Vec<Value> = a_order
        .iter()
        .filter(|k| !b_map.contains_key(*k))
        .map(|k| a_map[k].clone())
        .collect();

    let mut changed: Vec<Value> = Vec::new();
    for k in &b_order {
        let (Some(a_rec), Some(b_rec)) = (a_map.get(k), b_map.get(k)) else {
            continue;
        };
        // Which fields to diff: the explicit `track` list, or — when none is
        // given — every non-identity field across both records (union in
        // first-seen order, `a` then `b`).
        let fields: Vec<String> = if track.is_empty() {
            let mut seen: Vec<String> = Vec::new();
            for rec in [a_rec, b_rec] {
                if let Some(obj) = rec.as_object() {
                    for key in obj.keys() {
                        if key != by && !seen.contains(key) {
                            seen.push(key.clone());
                        }
                    }
                }
            }
            seen
        } else {
            track.to_vec()
        };

        let mut diffs = serde_json::Map::new();
        for field in &fields {
            let av = a_rec.get(field).cloned().unwrap_or(Value::Null);
            let bv = b_rec.get(field).cloned().unwrap_or(Value::Null);
            if av != bv {
                diffs.insert(field.clone(), serde_json::json!({ "from": av, "to": bv }));
            }
        }
        if !diffs.is_empty() {
            let mut entry = serde_json::Map::new();
            entry.insert(
                by.to_string(),
                b_rec.get(by).cloned().unwrap_or(Value::Null),
            );
            entry.insert("diffs".to_string(), Value::Object(diffs));
            changed.push(Value::Object(entry));
        }
    }

    serde_json::json!({ "added": added, "removed": removed, "changed": changed })
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

    #[tokio::test]
    async fn for_each_runs_body_once_per_item() {
        // src emits a 3-element array; the for-each `loop` runs its `do:` body
        // (`body`) once per element, binding `{{ item }}`. The aggregate output
        // recorded for `loop` is the array of each iteration's body output (#124).
        let app: App = serde_yaml::from_str(
            r#"
app: feach
version: 0.1.0
description: x
nodes:
  - id: src
    agent: ag-src
    command: list
  - id: loop
    for-each: '{{ src.items }}'
    do:
      - id: body
        agent: ag-body
        command: process
        inputs:
          val: '{{ item }}'
connections:
  - from: src
    to: loop
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-src",
                    "list",
                    serde_json::json!({ "items": [10, 20, 30] }),
                )
                .with_single("ag-body", "process", serde_json::json!({ "ok": true })),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        // The body node ran once per item → three NodeStart events for `body`.
        let body_starts = events
            .iter()
            .filter(|e| matches!(e, RunEvent::NodeStart { node, .. } if node == "body"))
            .count();
        assert_eq!(body_starts, 3, "body must run once per collection element");

        // The for-each node's aggregate output is a 3-element array.
        let loop_out = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "loop" => Some(data.clone()),
                _ => None,
            })
            .expect("for-each node must emit an aggregate NodeOutput");
        assert_eq!(loop_out.as_array().map(|a| a.len()), Some(3));
        assert_eq!(loop_out[0], serde_json::json!({ "ok": true }));
    }

    #[tokio::test]
    async fn for_each_under_simulate_runs_body_when_collection_empty() {
        // #124 repro: a one-shot `--simulate` app where the read source is
        // stubbed to an empty array. The for-each body must STILL run once (the
        // stubbed source legitimately yields nothing), and its write node must
        // emit a `would-write:` event rather than calling a host.
        let app: App = serde_yaml::from_str(
            r#"
app: feachsim
version: 0.1.0
description: x
nodes:
  - id: drawings
    agent: tekla
    command: drawing-list
  - id: sync
    for-each: '{{ drawings.drawings }}'
    do:
      - id: upsert
        agent: http
        command: post
        inputs:
          url: https://example.test/x
        safety:
          snapshot: false
connections:
  - from: drawings
    to: sync
requires: []
"#,
        )
        .unwrap();

        // Empty invoker: any real transport call surfaces as NotFound and fails.
        let inv = Arc::new(MockInvoker::new());
        let (mut orch, tmp, log_path) = make_orchestrator(app, inv).await;

        let agents = tmp.path().join("aware").join("agents");
        let tekla_dir = agents.join("tekla");
        std::fs::create_dir_all(&tekla_dir).unwrap();
        std::fs::write(
            tekla_dir.join("manifest.yaml"),
            r#"agent: tekla
version: 0.0.1
description: x
stateful: false
license: MIT
transport: { cli: { binary: this-binary-does-not-exist } }
commands:
  drawing-list:
    lifecycle: single
    category: curated
    mode: read
    description: lists drawings
    outputs:
      type: single
      schema:
        drawings: array
"#,
        )
        .unwrap();
        let http_dir = agents.join("http");
        std::fs::create_dir_all(&http_dir).unwrap();
        std::fs::write(
            http_dir.join("manifest.yaml"),
            r#"agent: http
version: 0.0.1
description: x
stateful: false
license: MIT
transport: { cli: { binary: this-binary-does-not-exist } }
commands:
  post:
    lifecycle: single
    category: curated
    mode: write
    description: posts a row
"#,
        )
        .unwrap();

        orch.simulate = true;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        // The write body node ran (once, for the synthetic iteration) and was
        // stubbed as a would-write rather than dispatched to the host.
        let would_write = events
            .iter()
            .any(|e| matches!(e, RunEvent::WouldWrite { node, .. } if node == "upsert"));
        assert!(
            would_write,
            "for-each body write node must emit a would-write under simulate"
        );
        if let RunEvent::RunEnd { status, .. } = events.last().unwrap() {
            assert_eq!(status, "ok");
        } else {
            panic!("run did not end cleanly");
        }
    }

    #[tokio::test]
    async fn for_each_body_predicate_filters_items() {
        // The for-each body is `[gate, body]`. `gate` keeps only items where
        // `item.keep == true`. With two items (one kept, one dropped), the write
        // node `body` runs once and the aggregate has a single element — proving
        // a gated body predicate stops the rest of that iteration AND drops the
        // item from the result (Codex P2-1).
        let app: App = serde_yaml::from_str(
            r#"
app: feachfilter
version: 0.1.0
description: x
nodes:
  - id: src
    agent: ag-src
    command: list
  - id: loop
    for-each: '{{ src.items }}'
    do:
      - id: gate
        inline:
          kind: predicate
          description: keep flagged items
          code: "e => e.keep == true"
      - id: body
        agent: ag-body
        command: process
connections:
  - from: src
    to: loop
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-src",
                    "list",
                    serde_json::json!({ "items": [{ "keep": true }, { "keep": false }] }),
                )
                .with_single("ag-body", "process", serde_json::json!({ "ok": true })),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let body_starts = events
            .iter()
            .filter(|e| matches!(e, RunEvent::NodeStart { node, .. } if node == "body"))
            .count();
        assert_eq!(body_starts, 1, "write body must run only for the kept item");

        let loop_out = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "loop" => Some(data.clone()),
                _ => None,
            })
            .expect("for-each node must emit an aggregate");
        assert_eq!(loop_out.as_array().map(|a| a.len()), Some(1));
    }

    #[tokio::test]
    async fn for_each_runs_in_long_running_stream() {
        // A watcher source emits one event carrying a 2-element array; the
        // downstream for-each node (reached via the streaming `execute_and_chain`
        // path) must run its body once per element (Codex P2-2).
        let app: App = serde_yaml::from_str(
            r#"
app: streamfeach
version: 0.1.0
description: x
nodes:
  - id: watch
    agent: ag-watch
    command: watch
  - id: loop
    for-each: '{{ watch.marks }}'
    do:
      - id: sink
        agent: ag-sink
        command: upload
        inputs:
          id: '{{ item.id }}'
connections:
  - from: watch
    to: loop
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_stream(
                    "ag-watch",
                    "watch",
                    vec![serde_json::json!({ "marks": [{ "id": 1 }, { "id": 2 }] })],
                )
                .with_single("ag-sink", "upload", serde_json::json!({ "received": "ok" })),
        );

        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        let (stop_tx, stop_rx) = stop_channel();
        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        let _ = stop_tx.send(true);
        run_handle.await.unwrap().unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let sink_outputs = events
            .iter()
            .filter(|e| matches!(e, RunEvent::NodeOutput { node, .. } if node == "sink"))
            .count();
        assert_eq!(
            sink_outputs, 2,
            "for-each body must run once per array element in the streaming path, events: {events:?}"
        );
    }

    #[tokio::test]
    async fn nested_for_each_restores_outer_item() {
        // Outer loop over regions; inner loop over each region's parts. After the
        // inner loop returns, a sibling body node (`mark`) must still resolve
        // `{{ item.name }}` to the OUTER region — not the last inner part. Run
        // under --dry-run so each write body node's `would-write:` event captures
        // the rendered args (#126 review: nested-loop item clobbering).
        let app: App = serde_yaml::from_str(
            r#"
app: nestfeach
version: 0.1.0
description: x
nodes:
  - id: src
    agent: ag-src
    command: list
  - id: outer
    for-each: '{{ src.regions }}'
    do:
      - id: inner
        for-each: '{{ item.parts }}'
        do:
          - id: tag
            agent: ag-part
            command: tag
            inputs:
              part: '{{ item }}'
            safety:
              snapshot: false
      - id: mark
        agent: ag-region
        command: mark
        inputs:
          region: '{{ item.name }}'
        safety:
          snapshot: false
connections:
  - from: src
    to: outer
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new().with_single(
            "ag-src",
            "list",
            serde_json::json!({
                "regions": [
                    { "name": "R1", "parts": ["p1", "p2"] },
                    { "name": "R2", "parts": ["p3"] }
                ]
            }),
        ));
        let (mut orch, tmp, log_path) = make_orchestrator(app, inv).await;

        // src is read (runs under dry-run via the mock); region/part are write
        // (emit would-write, capturing rendered args without a host).
        let agents = tmp.path().join("aware").join("agents");
        for (name, cmd, mode) in [
            ("ag-src", "list", "read"),
            ("ag-region", "mark", "write"),
            ("ag-part", "tag", "write"),
        ] {
            let dir = agents.join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(
                dir.join("manifest.yaml"),
                format!(
                    "agent: {name}\nversion: 0.0.1\ndescription: x\nstateful: false\nlicense: MIT\ntransport: {{ cli: {{ binary: nope }} }}\ncommands:\n  {cmd}:\n    lifecycle: single\n    category: curated\n    mode: {mode}\n    description: x\n    outputs:\n      type: single\n      schema:\n        regions: array\n"
                ),
            )
            .unwrap();
        }

        orch.dry_run = true;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        // The rendered `region` arg from every would-write on `mark`, in order.
        let regions: Vec<String> = events
            .iter()
            .filter_map(|e| match e {
                RunEvent::WouldWrite {
                    node,
                    proposed_inputs,
                    ..
                } if node == "mark" => proposed_inputs
                    .get("region")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                _ => None,
            })
            .collect();
        // Two outer iterations → the outer region name each time, never a part.
        assert_eq!(regions, vec!["R1".to_string(), "R2".to_string()]);
    }

    // ── #127: `compare` substrate primitive at runtime ──────────────────────

    #[tokio::test]
    async fn compare_one_shot_diffs_by_identity_key() {
        // `before` and `after` each emit a row list; the `compare` node diffs
        // them by the `mark` identity key, tracking the `profile` field. The
        // node output is `{ added, removed, changed }` per the app-spec
        // § Substrate primitives.
        let app: App = serde_yaml::from_str(
            r#"
app: cmp
version: 0.1.0
description: x
nodes:
  - id: before
    agent: ag-before
    command: list
  - id: after
    agent: ag-after
    command: list
  - id: delta
    compare:
      a: '{{ before.rows }}'
      b: '{{ after.rows }}'
      by: mark
      track: [profile]
connections:
  - from: before
    to: delta
  - from: after
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-before",
                    "list",
                    serde_json::json!({ "rows": [
                        { "mark": "A-1", "profile": "UB305" },
                        { "mark": "A-2", "profile": "UB406" }
                    ] }),
                )
                .with_single(
                    "ag-after",
                    "list",
                    serde_json::json!({ "rows": [
                        { "mark": "A-2", "profile": "UB457" },
                        { "mark": "A-3", "profile": "UB203" }
                    ] }),
                ),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let diff = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta" => Some(data.clone()),
                _ => None,
            })
            .expect("compare node must emit a NodeOutput");

        assert_eq!(
            diff["added"],
            serde_json::json!([{ "mark": "A-3", "profile": "UB203" }]),
            "A-3 is in b, not a"
        );
        assert_eq!(
            diff["removed"],
            serde_json::json!([{ "mark": "A-1", "profile": "UB305" }]),
            "A-1 is in a, not b"
        );
        assert_eq!(
            diff["changed"],
            serde_json::json!([{
                "mark": "A-2",
                "diffs": { "profile": { "from": "UB406", "to": "UB457" } }
            }]),
            "A-2's tracked profile field changed"
        );
    }

    #[tokio::test]
    async fn compare_empty_track_diffs_all_non_identity_fields() {
        // With `track` omitted, `changed` surfaces every differing non-identity
        // field. id=1 is in both: `status` changed, `note` did not — so the
        // diff lists only `status`.
        let app: App = serde_yaml::from_str(
            r#"
app: cmp2
version: 0.1.0
description: x
nodes:
  - id: before
    agent: ag-before
    command: list
  - id: after
    agent: ag-after
    command: list
  - id: delta
    compare:
      a: '{{ before.rows }}'
      b: '{{ after.rows }}'
      by: id
connections:
  - from: before
    to: delta
  - from: after
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-before",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 1, "status": "open", "note": "x" } ] }),
                )
                .with_single(
                    "ag-after",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 1, "status": "closed", "note": "x" } ] }),
                ),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let diff = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta" => Some(data.clone()),
                _ => None,
            })
            .expect("compare node must emit a NodeOutput");

        assert_eq!(diff["added"], serde_json::json!([]));
        assert_eq!(diff["removed"], serde_json::json!([]));
        assert_eq!(
            diff["changed"],
            serde_json::json!([{
                "id": 1,
                "diffs": { "status": { "from": "open", "to": "closed" } }
            }]),
            "empty track diffs all non-identity fields; only `status` changed"
        );
    }

    #[tokio::test]
    async fn compare_runs_in_long_running_stream() {
        // A watcher emits one event carrying `before` + `after` lists; the
        // downstream `compare` node, reached via the streaming
        // `execute_and_chain` path, must diff them and emit a NodeOutput
        // (mirrors for_each_runs_in_long_running_stream).
        let app: App = serde_yaml::from_str(
            r#"
app: streamcmp
version: 0.1.0
description: x
nodes:
  - id: src
    agent: ag-src
    command: watch
  - id: delta
    compare:
      a: '{{ src.before }}'
      b: '{{ src.after }}'
      by: id
connections:
  - from: src
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new().with_stream(
            "ag-src",
            "watch",
            vec![serde_json::json!({
                "before": [{ "id": 1 }],
                "after":  [{ "id": 1 }, { "id": 2 }]
            })],
        ));
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        let (stop_tx, stop_rx) = stop_channel();
        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        let _ = stop_tx.send(true);
        run_handle.await.unwrap().unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let diff = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta" => Some(data.clone()),
                _ => None,
            })
            .expect("compare node must emit a NodeOutput in the streaming path");
        assert_eq!(
            diff["added"],
            serde_json::json!([{ "id": 2 }]),
            "id=2 is new in `after`"
        );
        assert_eq!(diff["removed"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn compare_snapshot_keyed_errors_pending_snapshot_runtime() {
        // A snapshot-keyed compare (only `a-snapshot` / `b-snapshot`, no inline
        // `a`/`b`) depends on the `snapshot` primitive runtime, which is not yet
        // implemented. The run must fail with an error naming that dependency
        // rather than silently diffing two empty sets.
        let app: App = serde_yaml::from_str(
            r#"
app: cmpsnap
version: 0.1.0
description: x
nodes:
  - id: delta
    compare:
      a-snapshot: 'pre-tender-2026-05-10'
      b-snapshot: 'pre-tender-2026-05-21'
      by: element-id
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new());
        let (orch, _tmp, _log_path) = make_orchestrator(app, inv).await;
        let err = orch.run_one_shot().await.unwrap_err();
        assert!(
            err.to_string().contains("snapshot"),
            "snapshot-keyed compare must surface the snapshot-runtime dependency, got: {err}"
        );
    }

    #[tokio::test]
    async fn compare_mixed_inline_and_snapshot_errors() {
        // A compare mixing an inline `a` with a `b-snapshot` (no inline `b`)
        // must error on the snapshot dependency, not silently treat every `a`
        // record as removed by diffing against an empty `b` (reviewer Low-1).
        let app: App = serde_yaml::from_str(
            r#"
app: cmpmixed
version: 0.1.0
description: x
nodes:
  - id: src
    agent: ag-src
    command: list
  - id: delta
    compare:
      a: '{{ src.rows }}'
      b-snapshot: 'pre-tender-2026-05-10'
      by: mark
connections:
  - from: src
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new().with_single(
            "ag-src",
            "list",
            serde_json::json!({ "rows": [ { "mark": "A-1" } ] }),
        ));
        let (orch, _tmp, _log_path) = make_orchestrator(app, inv).await;
        let err = orch.run_one_shot().await.unwrap_err();
        assert!(
            err.to_string().contains("snapshot"),
            "mixed inline/snapshot compare must surface the snapshot dependency, got: {err}"
        );
    }

    #[test]
    fn diff_records_distinguishes_identity_value_types_and_first_wins() {
        // String "5" and numeric 5 are DISTINCT identities (no key collision);
        // on a duplicate `by` the first record wins; a record missing `by` is
        // skipped entirely (reviewer Low-2 + nits).
        let a = vec![
            serde_json::json!({ "id": "5", "v": "a-str" }),
            serde_json::json!({ "id": 7, "v": "first" }),
            serde_json::json!({ "id": 7, "v": "dupe-ignored" }),
            serde_json::json!({ "v": "no-id" }),
        ];
        let b = vec![
            serde_json::json!({ "id": 5, "v": "b-num" }),
            serde_json::json!({ "id": 7, "v": "first" }),
        ];
        let diff = diff_records(&a, &b, "id", &[]);

        assert_eq!(
            diff["removed"],
            serde_json::json!([{ "id": "5", "v": "a-str" }]),
            "string \"5\" is a distinct identity from numeric 5"
        );
        assert_eq!(
            diff["added"],
            serde_json::json!([{ "id": 5, "v": "b-num" }]),
            "numeric 5 is added, never matched to string \"5\""
        );
        assert_eq!(
            diff["changed"],
            serde_json::json!([]),
            "id 7 unchanged; the duplicate id-7 record in `a` is ignored (first wins)"
        );
    }

    #[tokio::test]
    async fn compare_streaming_fan_in_pairs_by_slot() {
        // Two watcher sources fan in to one `compare` node via explicit `input:`
        // slots. The streaming compare must resolve `a`/`b` from the PAIRED
        // fan-in event by slot name (`{{ left.rows }}`) — not from the global
        // latest upstream outputs (Codex P2). left carries an extra record
        // (id 2), so a correct paired diff reports it as removed.
        let app: App = serde_yaml::from_str(
            r#"
app: cmpfanin
version: 0.1.0
description: x
nodes:
  - id: left-feed
    agent: ag-left
    command: watch
  - id: right-feed
    agent: ag-right
    command: watch
  - id: delta
    compare:
      a: '{{ left.rows }}'
      b: '{{ right.rows }}'
      by: id
connections:
  - from: left-feed
    to: delta
    input: left
  - from: right-feed
    to: delta
    input: right
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_stream(
                    "ag-left",
                    "watch",
                    vec![serde_json::json!({ "rows": [ { "id": 1 }, { "id": 2 } ] })],
                )
                .with_stream(
                    "ag-right",
                    "watch",
                    vec![serde_json::json!({ "rows": [ { "id": 1 } ] })],
                ),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        let (stop_tx, stop_rx) = stop_channel();
        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let _ = stop_tx.send(true);
        let _ = run_handle.await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let diff = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta" => Some(data.clone()),
                _ => None,
            })
            .expect("fan-in compare must emit a NodeOutput");
        assert_eq!(
            diff["removed"],
            serde_json::json!([{ "id": 2 }]),
            "id 2 is in the paired left input, not the right — resolved by slot"
        );
        assert_eq!(diff["added"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn compare_streaming_preserves_app_inputs() {
        // A streaming compare diffs a run-supplied baseline (`{{ inputs.baseline }}`)
        // against the live event stream. Binding the event must NOT shadow the
        // `inputs` namespace (Codex P2), so side `a` still resolves to the app
        // input — id 2 is new in the stream, so it shows up as added.
        let app: App = serde_yaml::from_str(
            r#"
app: cmpbaseline
version: 0.1.0
description: x
nodes:
  - id: stream
    agent: ag-stream
    command: watch
  - id: delta
    compare:
      a: '{{ inputs.baseline }}'
      b: '{{ stream.rows }}'
      by: id
connections:
  - from: stream
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new().with_stream(
            "ag-stream",
            "watch",
            vec![serde_json::json!({ "rows": [ { "id": 1 }, { "id": 2 } ] })],
        ));
        let (mut orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        // Run-supplied baseline lives in the `inputs` namespace.
        orch.ctx.inputs = serde_json::json!({ "baseline": [ { "id": 1 } ] });
        let (stop_tx, stop_rx) = stop_channel();
        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let _ = stop_tx.send(true);
        let _ = run_handle.await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let diff = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta" => Some(data.clone()),
                _ => None,
            })
            .expect("compare must emit a NodeOutput");
        assert_eq!(
            diff["added"],
            serde_json::json!([{ "id": 2 }]),
            "id 2 is new in the stream vs the app-input baseline"
        );
        assert_eq!(diff["removed"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn compare_one_shot_fan_in_pairs_by_slot() {
        // A one-shot DAG where two predecessors fan into a `compare` node via
        // declared `input:` slots. The compare must resolve `a`/`b` by slot name
        // (`{{ left.rows }}` for `input: left`) — parity with the streaming path
        // (Codex P2). left carries an extra record (id 2) → reported as removed.
        let app: App = serde_yaml::from_str(
            r#"
app: cmposfanin
version: 0.1.0
description: x
nodes:
  - id: left-feed
    agent: ag-left
    command: list
  - id: right-feed
    agent: ag-right
    command: list
  - id: delta
    compare:
      a: '{{ left.rows }}'
      b: '{{ right.rows }}'
      by: id
connections:
  - from: left-feed
    to: delta
    input: left
  - from: right-feed
    to: delta
    input: right
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-left",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 1 }, { "id": 2 } ] }),
                )
                .with_single(
                    "ag-right",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 1 } ] }),
                ),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let diff = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta" => Some(data.clone()),
                _ => None,
            })
            .expect("one-shot fan-in compare must emit a NodeOutput");
        assert_eq!(
            diff["removed"],
            serde_json::json!([{ "id": 2 }]),
            "id 2 is in the `left` slot, not `right` — resolved by declared input slot"
        );
        assert_eq!(diff["added"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn compare_one_shot_slot_alias_does_not_leak() {
        // `delta1`'s incoming edge uses `input: bbb`, whose slot name collides
        // with the real node `bbb`. The compare's scoped overlay must NOT leak
        // that alias into the shared context: the later compare `delta2` reading
        // `{{ bbb.rows }}` must see the REAL `bbb` output ([id:2]), not delta1's
        // `a` side ([id:1]) (Codex P3).
        let app: App = serde_yaml::from_str(
            r#"
app: cmpleak
version: 0.1.0
description: x
nodes:
  - id: aaa
    agent: ag-aaa
    command: list
  - id: bbb
    agent: ag-bbb
    command: list
  - id: ccc
    agent: ag-ccc
    command: list
  - id: delta1
    compare:
      a: '{{ aaa.rows }}'
      b: '{{ ccc.rows }}'
      by: id
  - id: delta2
    compare:
      a: '{{ bbb.rows }}'
      b: '{{ ccc.rows }}'
      by: id
connections:
  - from: aaa
    to: delta1
    input: bbb
  - from: ccc
    to: delta1
  - from: bbb
    to: delta2
  - from: ccc
    to: delta2
  - from: delta1
    to: delta2
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-aaa",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 1 } ] }),
                )
                .with_single(
                    "ag-bbb",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 2 } ] }),
                )
                .with_single(
                    "ag-ccc",
                    "list",
                    serde_json::json!({ "rows": [ { "id": 3 } ] }),
                ),
        );
        let (orch, _tmp, log_path) = make_orchestrator(app, inv).await;
        orch.run_one_shot().await.unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let delta2 = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "delta2" => Some(data.clone()),
                _ => None,
            })
            .expect("delta2 must emit a NodeOutput");
        // delta2.a = bbb.rows = [{id:2}], delta2.b = ccc.rows = [{id:3}].
        // A leaked `input: bbb` alias would make bbb resolve to aaa.rows ([id:1]).
        assert_eq!(
            delta2["removed"],
            serde_json::json!([{ "id": 2 }]),
            "delta2 must read the REAL bbb ([id:2]); a leaked slot alias would show [id:1]"
        );
    }

    #[tokio::test]
    async fn compare_inline_plus_same_side_snapshot_errors() {
        // A side specifying BOTH an inline `a` AND `a-snapshot` must error —
        // snapshot-backed compare isn't implemented, so a snapshot key must
        // never be silently ignored in favor of the inline ref (Codex P2 #2).
        let app: App = serde_yaml::from_str(
            r#"
app: cmpboth
version: 0.1.0
description: x
nodes:
  - id: src
    agent: ag-src
    command: list
  - id: delta
    compare:
      a: '{{ src.rows }}'
      a-snapshot: 'pre-tender-2026-05-10'
      b: '{{ src.rows }}'
      by: mark
connections:
  - from: src
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new().with_single(
            "ag-src",
            "list",
            serde_json::json!({ "rows": [ { "mark": "A-1" } ] }),
        ));
        let (orch, _tmp, _log_path) = make_orchestrator(app, inv).await;
        let err = orch.run_one_shot().await.unwrap_err();
        assert!(
            err.to_string().contains("snapshot"),
            "any snapshot key must surface the snapshot dependency, got: {err}"
        );
    }

    #[tokio::test]
    async fn compare_missing_side_errors() {
        // An inline compare that omits one side (no snapshots) is malformed —
        // `compare` is a two-sided diff. It must error, not silently diff `a`
        // against an empty `b` and report every record as removed (Codex P2 #3).
        let app: App = serde_yaml::from_str(
            r#"
app: cmphalf
version: 0.1.0
description: x
nodes:
  - id: before
    agent: ag-before
    command: list
  - id: delta
    compare:
      a: '{{ before.rows }}'
      by: mark
connections:
  - from: before
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(MockInvoker::new().with_single(
            "ag-before",
            "list",
            serde_json::json!({ "rows": [ { "mark": "A-1" } ] }),
        ));
        let (orch, _tmp, _log_path) = make_orchestrator(app, inv).await;
        let err = orch.run_one_shot().await.unwrap_err();
        assert!(
            err.to_string().contains("missing"),
            "a compare missing an inline side must be rejected, got: {err}"
        );
    }

    #[tokio::test]
    async fn compare_unresolved_side_errors() {
        // A typo'd ref (`before.rowz` — the field is `rows`) resolves to Null.
        // In a real run that must error, not coerce to an empty list and report
        // every record on the other side as added (Codex P2 #4).
        let app: App = serde_yaml::from_str(
            r#"
app: cmptypo
version: 0.1.0
description: x
nodes:
  - id: before
    agent: ag-before
    command: list
  - id: after
    agent: ag-after
    command: list
  - id: delta
    compare:
      a: '{{ before.rowz }}'
      b: '{{ after.rows }}'
      by: mark
connections:
  - from: before
    to: delta
  - from: after
    to: delta
requires: []
"#,
        )
        .unwrap();

        let inv = Arc::new(
            MockInvoker::new()
                .with_single(
                    "ag-before",
                    "list",
                    serde_json::json!({ "rows": [ { "mark": "A-1" } ] }),
                )
                .with_single(
                    "ag-after",
                    "list",
                    serde_json::json!({ "rows": [ { "mark": "A-2" } ] }),
                ),
        );
        let (orch, _tmp, _log_path) = make_orchestrator(app, inv).await;
        let err = orch.run_one_shot().await.unwrap_err();
        assert!(
            err.to_string().contains("did not resolve"),
            "an unresolved compare side must be rejected, got: {err}"
        );
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
    async fn simulate_stubs_stream_source_and_downstream() {
        // A `lifecycle: start` watcher source → write sink. Under `--simulate`
        // the orchestrator must NOT call invoke_stream or invoke_single (an empty
        // MockInvoker errors if it does): the source emits one schema-shaped
        // placeholder event and the write sink emits `would-write:` (#117-2).
        let app: App = serde_yaml::from_str(
            r#"
app: sim-stream
version: 0.1.0
description: x
nodes:
  - id: watch
    agent: ag-watch
    command: watch
  - id: sink
    agent: ag-sink
    command: upload
connections:
  - from: watch
    to: sink
requires: []
"#,
        )
        .unwrap();

        // Empty invoker: any real invoke_stream/invoke_single fails the run.
        let inv = Arc::new(MockInvoker::new());
        let (mut orch, tmp, log_path) = make_orchestrator(app, inv).await;

        let agents = tmp.path().join("aware").join("agents");
        let watch_dir = agents.join("ag-watch");
        std::fs::create_dir_all(&watch_dir).unwrap();
        std::fs::write(
            watch_dir.join("manifest.yaml"),
            r#"agent: ag-watch
version: 0.0.1
description: x
stateful: true
license: MIT
transport: { cli: { binary: this-binary-does-not-exist } }
commands:
  watch:
    lifecycle: start
    category: curated
    mode: read
    description: stream of marks
    outputs:
      type: stream
      schema:
        mark: string
"#,
        )
        .unwrap();
        let sink_dir = agents.join("ag-sink");
        std::fs::create_dir_all(&sink_dir).unwrap();
        std::fs::write(
            sink_dir.join("manifest.yaml"),
            r#"agent: ag-sink
version: 0.0.1
description: x
stateful: false
license: MIT
transport: { cli: { binary: this-binary-does-not-exist } }
commands:
  upload:
    lifecycle: single
    category: curated
    mode: write
    description: upload a mark
    outputs:
      type: single
      schema:
        received: string
"#,
        )
        .unwrap();

        orch.simulate = true;
        let (stop_tx, stop_rx) = stop_channel();
        let run_handle = tokio::spawn(orch.run_long_running(stop_rx));
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let _ = stop_tx.send(true);
        // Must complete without ever touching the (empty) invoker.
        run_handle.await.unwrap().unwrap();

        let events = read_run_events(&log_path).await.unwrap();
        let watch_out = events
            .iter()
            .find_map(|e| match e {
                RunEvent::NodeOutput { node, data, .. } if node == "watch" => Some(data.clone()),
                _ => None,
            })
            .expect("watch source must emit a synthesized placeholder event");
        assert_eq!(watch_out["mark"], serde_json::json!("<simulated>"));
        let saw_would_write = events
            .iter()
            .any(|e| matches!(e, RunEvent::WouldWrite { node, .. } if node == "sink"));
        assert!(
            saw_would_write,
            "write sink must emit would-write under simulate; events: {events:?}"
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
