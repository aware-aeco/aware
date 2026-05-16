# AWARE CLI v0.3 (Runtime) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax.

**Goal:** AWARE apps actually run. `aware app run <app>` executes the topology — invokes agents, marshals events between nodes, fans-in synchronized inputs, writes a full provenance trace, handles Ctrl+C cleanly. Stateful and stateless apps both. v0.3 turns the substrate from "documentation" into "operational."

**Architecture:** Async runtime atop tokio. An `AgentInvoker` trait abstracts how a node actually runs — production impl spawns subprocesses and communicates via JSON envelope over stdin/stdout; test impl runs in-process Rust closures so the orchestrator can be tested without real agent binaries. A topology graph derived from `connections:` schedules nodes: source-first for one-shot apps, broadcast loop for long-running apps with stateful nodes. Templating via minijinja resolves `{{ inputs.X }}` / `{{ <node>.<output> }}` / `{{ secrets.<id> }}` against the runtime context. Provenance writes to `~/.aware/logs/<app>/<instance>/<run-id>.jsonl` with typed events. Ctrl+C broadcasts stop through tokio channels; stateful nodes get a chance to clean up.

**Tech Stack additions:**
- `tokio` 1.x — full features (async, signal, sync, fs, process, time, macros)
- `minijinja` 2.x — Jinja2-style templating (lightweight, no Jinja2 Python dep)
- `uuid` 1.x — for run-id generation
- (Already present from v0.1/v0.2: serde, serde_yaml, serde_json, ureq, flate2, tar, chrono, thiserror, anyhow, tempfile, assert_cmd, predicates)

---

## Open design decisions (resolved here — flag if you want changes)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **Agent invocation abstraction** | `AgentInvoker` trait with `CliInvoker` (subprocess + JSON envelope) and `MockInvoker` (test-only, in-process closures) | Decouples orchestrator from transport reality; tests run without external binaries; production path is real-but-needs-binaries-from-v0.5 | Hardcoded subprocess spawn — tests would need actual binaries on disk (impossible until v0.5) |
| **Real agent invocation in v0.3** | `CliInvoker` exists and works for any binary that's actually on `$PATH`; for the 7 reference agents (which have no built binaries yet), the orchestrator surfaces a clear "agent transport binary not found" error at run-time. Tests use `MockInvoker` to verify end-to-end orchestration logic | Defers v0.5 (the binary builders) without blocking v0.3 def-of-done | Build a fake `aware-tekla` etc. as part of v0.3 — premature, locks shape |
| **Templating engine** | `minijinja` 2.x | Drop-in Jinja2 syntax matches the spec examples; native Rust; no Python | Hand-rolled regex substitution (loses loops/conditionals); `tera` (similar but bigger surface) |
| **Inline glue evaluation** | Minimal hand-rolled predicate evaluator: `==`, `!=`, `&&`, `||`, parenthesization, field access (`e.foo.bar`), literal strings/numbers/null/bool. No general scripting | Covers the two examples in the reference apps (`e.type == "Welded" && e.mark != null`); no script-engine dep | Embed `rhai` or `boa` JS engine — heavy for one feature; runtime safety surface |
| **Async runtime** | `tokio` with `full` features | Required for `signal::ctrl_c`, channels, async fs, async process spawning | `smol`/`async-std` — smaller but less ergonomic for this use case |
| **One-shot vs long-running** | Inferred: if any node's agent is `stateful: true`, app is long-running and blocks until `stop` / Ctrl+C; otherwise one-shot (run-to-completion) | Matches `app-spec.md` §Lifecycle | User flag `--mode oneshot|long` — confusing redundancy with manifest |
| **Provenance shape** | JSONL, one event per line, typed `kind` field: `run-start`, `node-start`, `node-output`, `node-error`, `node-stop`, `run-end` | Greppable, line-buffered, simple to replay | Binary log (faster but opaque); SQLite (over-engineered) |
| **Run-id generation** | UUID v4, written as the trace file's basename (`<uuid>.jsonl`) | Globally unique without coordination | Monotonic counter (collisions across machines); ISO-timestamp (collisions within ms) |
| **Ctrl+C handling** | `tokio::signal::ctrl_c` future raced against the run; on signal, broadcasts "stop" via a tokio `watch` channel; stateful nodes get a chance to call their `stop` lifecycle | Clean, idiomatic, cross-platform | OS-specific signal handlers — fragile on Windows |
| **`aware app stop`** | Reads `~/.aware/apps/<app>/instances/<instance>/pidfile`; sends OS signal (SIGTERM / Ctrl+C on Windows) | Standard daemon-stop pattern; works without an IPC channel | Custom socket protocol for stop — premature; complicates running app |
| **`aware app logs --replay`** | Reads the most recent `.jsonl` (or `--run-id <id>` for a specific one); pretty-prints each event with timestamps and color-coded node labels | Replay = "show me what happened" not "re-execute" | Re-execute from logs — requires state replay machinery, way out of scope |
| **Per-instance state dir** | Created at `~/.aware/apps/<app>/instances/<instance>/state/`; v0.3 leaves it empty (stateful agents may write to it; the runtime itself doesn't persist orchestrator state across runs in v0.3) | Spec calls for it; v0.3 doesn't yet need rehydration | Persist event log + checkpoints — that's v0.4+ if pause/resume becomes a goal |
| **Pidfile** | YAML at `~/.aware/apps/<app>/instances/<instance>/pidfile.yaml` — `{pid: N, started-at: ISO}` | Simple, debuggable | Just a `.pid` text file — fine too; YAML is consistent with rest of `~/.aware/` |
| **Fan-in synchronization** | Each fan-in node has a small in-memory buffer per input slot (declared via `input:` in connections). The node fires when every slot has at least one value; oldest unfired buffer entries are paired up | Matches the qa-drawings-to-tekla example (paired drawing + spec) | Strict time-windowed sync (more complex); user-defined sync function (later) |
| **Secrets in templating** | `{{ secrets.<id> }}` reads `~/.aware/credentials/<id>.json` (or `.<alias>.json`); v0.3 reads plain JSON; v0.4 will replace this with the OS-keyring-encrypted store | Unblocks testing; v0.4 transparent swap | Stub the secrets table — breaks the templating end-to-end test |
| **Testing strategy** | Integration tests use `MockInvoker` configured with canned outputs per (agent-id, command). One full trace test against `welded-to-tc` (linear, stateful watcher streams 3 events, predicate filters, upload-mock receives them). One DAG fan-in test against `qa-drawings-to-tekla` (drawing + spec arrive in different orders; pair-and-validate fires correctly) | Sufficient to lock def-of-done | Real-agent integration tests — impossible until v0.5 |

---

## Scope discipline

**v0.3 user-facing surfaces** (3 new):
- `aware app run <app> [--instance <id>] [--config k=v ...]`
- `aware app stop <app> [--instance <id>]`
- `aware app logs <app> [--instance <id>] [--tail | --replay [--run-id <id>]]`

**v0.3 internal surfaces** (built but not user-facing):
- `cli::runtime::orchestrator` — load `.flo` → graph → schedule
- `cli::runtime::template` — minijinja substitution against a runtime context
- `cli::runtime::inline` — predicate evaluator for inline glue nodes
- `cli::runtime::invoker` — `AgentInvoker` trait + `CliInvoker` + `MockInvoker` + `StreamingMockInvoker`
- `cli::runtime::lifecycle` — start/stop hooks for stateful agents
- `cli::runtime::provenance` — `RunEvent` type + JSONL writer + reader/replay
- `cli::runtime::pidfile` — write/read instance pidfile for `app stop`

**Out of scope** — stay `NotYetImplemented`:
- `aware agent publish` — v0.2+ (registry write)
- `aware connect / disconnect` — v0.4
- `aware build agent`, `aware skill ...` — v0.5
- The 7 reference agents' actual runtime binaries — v0.5 generates these from DLLs / NuGet / OpenAPI; v0.3 just calls into whatever binary exists

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli/src/runtime/mod.rs` | Re-exports. |
| `cli/src/runtime/orchestrator.rs` | `Orchestrator` — owns the topology graph, scheduling, run loop. |
| `cli/src/runtime/template.rs` | `render_template(template, &Context) -> String`. minijinja-backed. |
| `cli/src/runtime/inline.rs` | `eval_predicate(code, &Value) -> bool`. Minimal evaluator. |
| `cli/src/runtime/invoker.rs` | `AgentInvoker` trait, `CliInvoker`, `MockInvoker`, `StreamingMockInvoker`. |
| `cli/src/runtime/lifecycle.rs` | `start_stateful_node`, `stop_stateful_node`, stop-broadcast channel helpers. |
| `cli/src/runtime/provenance.rs` | `RunEvent` enum + JSONL writer + reader for replay. |
| `cli/src/runtime/pidfile.rs` | Read/write `instances/<id>/pidfile.yaml`. |
| `cli/src/runtime/context.rs` | `RuntimeContext` — collected inputs, upstream outputs, secrets, config. |
| `cli/tests/app_run.rs` | Integration tests for `aware app run` (linear + DAG cases). |
| `cli/tests/app_stop.rs` | Integration tests for `aware app stop`. |
| `cli/tests/app_logs.rs` | Integration tests for `aware app logs --tail` and `--replay`. |
| `cli/tests/runtime_helpers/mod.rs` | Test-only helpers: build a `MockInvoker` setup, populate AWARE_HOME with fixtures, sandbox `~/.aware/logs/` and `instances/`. |

### Modified files

| Path | Change |
|---|---|
| `cli/Cargo.toml` | Add `tokio = { version = "1", features = ["full"] }`, `minijinja = "2"`, `uuid = { version = "1", features = ["v4"] }`. |
| `cli/src/main.rs` | Add `mod runtime;`. `fn main` becomes `#[tokio::main] async fn main` to support async dispatchers. |
| `cli/src/commands/app.rs` | Implement `Run`, `Stop`, `Logs` arms. They call into `runtime::orchestrator`, `runtime::pidfile`, `runtime::provenance`. |
| `cli/src/commands/doctor.rs` | Extend with a `Running:` block — count active instances by reading pidfiles. |
| `cli/src/paths.rs` | Add `logs_dir()` → `<aware_home>/logs/`; `app_instance_dir(app, instance)` → `<aware_home>/apps/<app>/instances/<instance>/`. |

### Key data shapes

**`RunEvent`** (serialized as JSONL):

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RunEvent {
    RunStart   { ts: String, run_id: String, app: String, instance: String, config: serde_json::Value },
    NodeStart  { ts: String, run_id: String, node: String, agent: Option<String>, command: Option<String> },
    NodeOutput { ts: String, run_id: String, node: String, data: serde_json::Value },
    NodeError  { ts: String, run_id: String, node: String, error: String },
    NodeStop   { ts: String, run_id: String, node: String, reason: String },
    RunEnd     { ts: String, run_id: String, status: String }, // "ok" | "error" | "interrupted"
}
```

**`pidfile.yaml`**:

```yaml
app: welded-to-tc
instance: default
pid: 12345
started-at: 2026-05-16T14:23:00Z
run-id: r_8c1d…
```

**`AgentInvoker` trait**:

```rust
#[async_trait::async_trait]
pub trait AgentInvoker: Send + Sync {
    /// Run a single command (single-lifecycle agents).
    async fn invoke_single(&self, agent: &str, command: &str, args: serde_json::Value) -> Result<serde_json::Value, AwareError>;
    /// Start a streaming command (start-lifecycle agents).
    /// Returns a receiver that yields outputs until the streaming task ends or `stop()` is called.
    async fn invoke_stream(&self, agent: &str, command: &str, args: serde_json::Value) -> Result<StreamingHandle, AwareError>;
}
```

(We'll need `async_trait` 0.1.x as an additional dep. Add in Task 1.)

---

## Task 0: Branch off main + baseline

- [ ] `git checkout main && git pull --ff-only && git checkout -b feat/cli-v03-runtime` (already done)
- [ ] `cargo test` baseline — 91 tests still passing (the v0.2 surface)

No commit.

---

## Task 1: Add `tokio`, `minijinja`, `uuid`, `async-trait` dependencies

**Files:** `cli/Cargo.toml`

Append to `[dependencies]`:

```toml
tokio       = { version = "1", features = ["full"] }
minijinja   = "2"
uuid        = { version = "1", features = ["v4"] }
async-trait = "0.1"
```

Build, ensure 91-test baseline still green, commit:

```
chore(cli): add tokio + minijinja + uuid + async-trait for v0.3
```

---

## Task 2: `cli::runtime::provenance` — RunEvent + JSONL writer

**Files:** Create `cli/src/runtime/mod.rs`, `cli/src/runtime/provenance.rs`. Modify `cli/src/main.rs` (add `mod runtime;`). Modify `cli/src/paths.rs` (add `logs_dir()`).

`runtime/mod.rs`:

```rust
//! Async orchestrator + provenance + agent invocation.
//! v0.3 phase. See docs/superpowers/plans/2026-05-16-aware-cli-v03-runtime.md.

#![allow(dead_code)]

pub mod provenance;
// other submodules added in subsequent tasks
```

`runtime/provenance.rs`:

```rust
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::error::AwareError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RunEvent {
    RunStart   { ts: String, run_id: String, app: String, instance: String, config: serde_json::Value },
    NodeStart  { ts: String, run_id: String, node: String, agent: Option<String>, command: Option<String> },
    NodeOutput { ts: String, run_id: String, node: String, data: serde_json::Value },
    NodeError  { ts: String, run_id: String, node: String, error: String },
    NodeStop   { ts: String, run_id: String, node: String, reason: String },
    RunEnd     { ts: String, run_id: String, status: String },
}

pub fn run_id_now() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn log_dir_for(logs_dir: &Path, app: &str, instance: &str) -> PathBuf {
    logs_dir.join(app).join(instance)
}

pub fn log_path_for(logs_dir: &Path, app: &str, instance: &str, run_id: &str) -> PathBuf {
    log_dir_for(logs_dir, app, instance).join(format!("{run_id}.jsonl"))
}

pub struct ProvenanceWriter {
    file: tokio::fs::File,
}

impl ProvenanceWriter {
    pub async fn open(path: &Path) -> Result<Self, AwareError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let file = OpenOptions::new().create(true).append(true).open(path).await?;
        Ok(Self { file })
    }

    pub async fn write(&mut self, event: &RunEvent) -> Result<(), AwareError> {
        let mut line = serde_json::to_string(event)
            .map_err(|e| AwareError::Internal(format!("serialize event: {e}")))?;
        line.push('\n');
        self.file.write_all(line.as_bytes()).await?;
        Ok(())
    }
}

pub async fn read_run_events(path: &Path) -> Result<Vec<RunEvent>, AwareError> {
    let file = tokio::fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut out = Vec::new();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() { continue; }
        let evt: RunEvent = serde_json::from_str(&line)
            .map_err(|e| AwareError::Validation(format!("parse run event: {e}")))?;
        out.push(evt);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn writes_and_reads_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("test.jsonl");

        let mut w = ProvenanceWriter::open(&path).await.unwrap();
        w.write(&RunEvent::RunStart {
            ts: "2026-05-16T00:00:00Z".into(),
            run_id: "r1".into(),
            app: "welded-to-tc".into(),
            instance: "default".into(),
            config: serde_json::json!({}),
        }).await.unwrap();
        w.write(&RunEvent::NodeStart {
            ts: "2026-05-16T00:00:01Z".into(),
            run_id: "r1".into(),
            node: "tekla-watch".into(),
            agent: Some("tekla".into()),
            command: Some("watch".into()),
        }).await.unwrap();
        drop(w); // close

        let events = read_run_events(&path).await.unwrap();
        assert_eq!(events.len(), 2);
        if let RunEvent::RunStart { app, .. } = &events[0] {
            assert_eq!(app, "welded-to-tc");
        } else {
            panic!("expected RunStart");
        }
    }
}
```

`paths.rs` additions inside `impl Paths`:

```rust
pub fn logs_dir(&self) -> PathBuf {
    self.aware_home.join("logs")
}

pub fn app_instance_dir(&self, app: &str, instance: &str) -> PathBuf {
    self.apps_dir().join(app).join("instances").join(instance)
}
```

Test, commit: `feat(cli): runtime provenance (RunEvent + JSONL writer)`.

---

## Task 3: `cli::runtime::template` — minijinja substitution

**Files:** Create `cli/src/runtime/template.rs`. Modify `cli/src/runtime/mod.rs`.

```rust
use minijinja::{Environment, context};
use serde_json::Value;

use crate::error::AwareError;

pub struct RenderContext {
    pub inputs: Value,
    /// upstream node outputs keyed by node id.
    pub upstream: serde_json::Map<String, Value>,
    /// resolved secrets keyed by credential id.
    pub secrets: serde_json::Map<String, Value>,
    /// app-level config kv.
    pub config: serde_json::Map<String, Value>,
}

pub fn render(template: &str, ctx: &RenderContext) -> Result<String, AwareError> {
    let mut env = Environment::new();
    env.add_template("t", template)
        .map_err(|e| AwareError::Validation(format!("template parse: {e}")))?;
    let tmpl = env.get_template("t").unwrap();

    // Build a flat context map: top-level keys are `inputs`, `secrets`, `config`,
    // plus each upstream-node-id maps to its outputs object.
    let mut top = serde_json::Map::new();
    top.insert("inputs".into(), ctx.inputs.clone());
    top.insert("secrets".into(), Value::Object(ctx.secrets.clone()));
    top.insert("config".into(), Value::Object(ctx.config.clone()));
    for (node_id, outputs) in &ctx.upstream {
        top.insert(node_id.clone(), outputs.clone());
    }

    let rendered = tmpl.render(context! { ..serde_json::Value::Object(top) })
        .map_err(|e| AwareError::Validation(format!("template render: {e}")))?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitutes_inputs() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "tc-project-id": "proj-123" }),
            upstream: serde_json::Map::new(),
            secrets: serde_json::Map::new(),
            config: serde_json::Map::new(),
        };
        let out = render("{{ inputs['tc-project-id'] }}", &ctx).unwrap();
        assert_eq!(out, "proj-123");
    }

    #[test]
    fn substitutes_upstream_outputs() {
        let mut upstream = serde_json::Map::new();
        upstream.insert("tekla-watch".into(), serde_json::json!({ "mark": "A-104" }));
        let ctx = RenderContext {
            inputs: serde_json::json!({}),
            upstream,
            secrets: serde_json::Map::new(),
            config: serde_json::Map::new(),
        };
        let out = render("file:{{ tekla-watch.mark }}.pdf", &ctx).unwrap();
        // Note: kebab-case identifiers need bracket notation in Jinja
        // We adjust template authoring to handle this; for now test with underscore-safe id.
        // Actually minijinja allows the dot syntax but parses `tekla-watch` as `tekla - watch` (subtract).
        // We'll need to special-case kebab-case node ids in the template author or use bracket access.
        // For now, document that kebab-case requires `{{ outputs['tekla-watch'].mark }}`.
        let _ = out; // smoke; the assertion above may fail depending on minijinja's identifier rules
    }
}
```

**Important note on kebab-case node ids in templates:** minijinja parses `tekla-watch` as `tekla minus watch`. The reference apps use kebab-case node ids like `tekla-watch`. Two options:

1. Convert kebab → underscore when building the context: `tekla-watch` becomes `tekla_watch` in templates. Document in README.
2. Require bracket access in templates: `{{ ['tekla-watch'].mark }}` — uglier but unambiguous.

**Decision:** Option 1 for v0.3 — auto-convert kebab to underscore in upstream-output identifiers. So `{{ tekla_watch.mark }}` resolves to `tekla-watch`'s output's `mark` field. The runtime warns if a node id has both forms (impossible since serde dedupe, but defensive).

Update the implementation accordingly. The render-test above is then:

```rust
let out = render("{{ tekla_watch.mark }}", &ctx).unwrap();
assert_eq!(out, "A-104");
```

`runtime/mod.rs`: add `pub mod template;`.

Test, commit: `feat(cli): runtime template (minijinja + kebab/underscore translation)`.

---

## Task 4: `cli::runtime::inline` — predicate evaluator

**Files:** Create `cli/src/runtime/inline.rs`. Modify `runtime/mod.rs`.

A minimal evaluator supporting `==`, `!=`, `&&`, `||`, parens, `e.field.path`, literal strings/numbers/booleans/null.

```rust
//! Hand-rolled predicate evaluator for inline glue nodes.
//!
//! Supports the subset needed by the v0.3 reference apps:
//!   e.field == "literal" && e.other != null
//!
//! Tokens: identifiers, dotted paths, ==, !=, &&, ||, (, ), "strings", numbers, true/false/null.

use serde_json::Value;

use crate::error::AwareError;

pub fn eval_predicate(code: &str, event: &Value) -> Result<bool, AwareError> {
    // Pre-process: many of the reference predicates use the form `e => <expr>`.
    // Strip the arrow prefix if present.
    let expr_src = code.trim().split_once("=>").map(|(_, r)| r.trim()).unwrap_or(code);
    let tokens = tokenize(expr_src)?;
    let (val, rest) = parse_or(&tokens)?;
    if !rest.is_empty() {
        return Err(AwareError::Validation(format!(
            "predicate: unexpected trailing tokens: {rest:?}"
        )));
    }
    let evaluated = eval_node(&val, event)?;
    match evaluated {
        Value::Bool(b) => Ok(b),
        other => Err(AwareError::Validation(format!(
            "predicate did not evaluate to bool: {other:?}"
        ))),
    }
}

// ... (tokenizer, parser, evaluator — full implementation in the task subagent prompt below)
```

The full evaluator code is ~200 lines. I'll embed it in the Task 4 implementer prompt rather than the plan body, to keep the plan readable. Key behaviors:
- `e.foo.bar` → JSON-path lookup into the event
- `e == ...` is an error (`e` alone is a Value, not a comparable literal)
- `null` literal matches JSON null
- Short-circuit `&&` / `||`
- All other operators error out clearly

Tests cover:
- `e.type == "Welded"` against `{"type":"Welded"}` → true
- `e.mark != null` against `{"mark":"A-104"}` → true
- `e.type == "Welded" && e.mark != null` against the above → true
- Arrow form: `e => e.type == "Welded"` → same
- Parens: `(e.x == 1) || (e.y == 2)` → correct
- Unknown field defaults to null

`runtime/mod.rs`: add `pub mod inline;`.

Test, commit: `feat(cli): inline predicate evaluator`.

---

## Task 5: `cli::runtime::invoker` — AgentInvoker trait + MockInvoker

**Files:** Create `cli/src/runtime/invoker.rs`. Modify `runtime/mod.rs`.

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc;

use crate::error::AwareError;

/// A streaming handle: a receiver yielding outputs and a stop signaler.
pub struct StreamingHandle {
    pub rx: mpsc::Receiver<Result<Value, AwareError>>,
    pub stop: tokio::sync::oneshot::Sender<()>,
}

#[async_trait]
pub trait AgentInvoker: Send + Sync {
    async fn invoke_single(&self, agent: &str, command: &str, args: Value) -> Result<Value, AwareError>;
    async fn invoke_stream(&self, agent: &str, command: &str, args: Value) -> Result<StreamingHandle, AwareError>;
}

/// Test-only invoker: pre-baked outputs per (agent, command) call.
#[derive(Default, Clone)]
pub struct MockInvoker {
    /// (agent, command) -> single output
    single: Arc<Mutex<HashMap<(String, String), Value>>>,
    /// (agent, command) -> stream of outputs
    streams: Arc<Mutex<HashMap<(String, String), Vec<Value>>>>,
}

impl MockInvoker {
    pub fn new() -> Self { Self::default() }

    pub fn with_single(mut self, agent: &str, command: &str, output: Value) -> Self {
        self.single.lock().unwrap().insert((agent.into(), command.into()), output);
        self
    }

    pub fn with_stream(mut self, agent: &str, command: &str, outputs: Vec<Value>) -> Self {
        self.streams.lock().unwrap().insert((agent.into(), command.into()), outputs);
        self
    }
}

#[async_trait]
impl AgentInvoker for MockInvoker {
    async fn invoke_single(&self, agent: &str, command: &str, _args: Value) -> Result<Value, AwareError> {
        let key = (agent.to_string(), command.to_string());
        self.single.lock().unwrap().get(&key).cloned()
            .ok_or_else(|| AwareError::NotFound(format!("mock invoker has no canned output for {agent}/{command}")))
    }

    async fn invoke_stream(&self, agent: &str, command: &str, _args: Value) -> Result<StreamingHandle, AwareError> {
        let key = (agent.to_string(), command.to_string());
        let outputs = self.streams.lock().unwrap().get(&key).cloned()
            .ok_or_else(|| AwareError::NotFound(format!("mock invoker has no canned stream for {agent}/{command}")))?;
        let (tx, rx) = mpsc::channel(16);
        let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            for o in outputs {
                if stop_rx.try_recv().is_ok() { break; }
                if tx.send(Ok(o)).await.is_err() { break; }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        });
        Ok(StreamingHandle { rx, stop: stop_tx })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_single_returns_canned_output() {
        let inv = MockInvoker::new().with_single("tc", "upload", serde_json::json!({"file-id":"f_1"}));
        let out = inv.invoke_single("tc", "upload", serde_json::json!({})).await.unwrap();
        assert_eq!(out["file-id"], "f_1");
    }

    #[tokio::test]
    async fn mock_stream_yields_outputs_in_order() {
        let inv = MockInvoker::new().with_stream("tk", "watch", vec![
            serde_json::json!({"mark":"A"}),
            serde_json::json!({"mark":"B"}),
        ]);
        let mut handle = inv.invoke_stream("tk", "watch", serde_json::json!({})).await.unwrap();
        let a = handle.rx.recv().await.unwrap().unwrap();
        let b = handle.rx.recv().await.unwrap().unwrap();
        assert_eq!(a["mark"], "A");
        assert_eq!(b["mark"], "B");
    }
}
```

`runtime/mod.rs`: add `pub mod invoker;`.

Test, commit: `feat(cli): AgentInvoker trait + MockInvoker for tests`.

---

## Task 6: `CliInvoker` — spawn subprocess + JSON envelope

**Files:** Modify `cli/src/runtime/invoker.rs` (add `CliInvoker`).

```rust
/// Production invoker: spawn the agent's CLI transport binary, talk JSON over stdin/stdout.
pub struct CliInvoker {
    pub agents_dir: std::path::PathBuf,
}

#[async_trait]
impl AgentInvoker for CliInvoker {
    async fn invoke_single(&self, agent: &str, command: &str, args: Value) -> Result<Value, AwareError> {
        // 1. Read agent manifest to find transport.cli.binary
        let manifest_path = self.agents_dir.join(agent).join("manifest.yaml");
        let m = crate::manifest::loader::load_agent(&manifest_path)?;
        let cli = m.transport.cli.as_ref()
            .ok_or_else(|| AwareError::Validation(format!("agent {agent} has no cli transport")))?;
        let binary = &cli.binary;

        // 2. Spawn: <binary> <command> --json-stdin
        let mut child = tokio::process::Command::new(binary)
            .arg(command)
            .arg("--json-stdin")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| AwareError::Network(format!("spawn {binary}: {e}")))?;

        // 3. Write args JSON to stdin
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(serde_json::to_string(&args).unwrap().as_bytes()).await
                .map_err(|e| AwareError::Network(format!("stdin write: {e}")))?;
        }

        // 4. Wait for completion, parse stdout
        let output = child.wait_with_output().await
            .map_err(|e| AwareError::Network(format!("wait: {e}")))?;
        if !output.status.success() {
            return Err(AwareError::Network(format!("agent {agent}/{command} failed: {}", String::from_utf8_lossy(&output.stderr))));
        }
        let body: Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| AwareError::Validation(format!("agent {agent}/{command} stdout not JSON: {e}")))?;
        Ok(body)
    }

    async fn invoke_stream(&self, agent: &str, command: &str, args: Value) -> Result<StreamingHandle, AwareError> {
        // Similar to invoke_single but: keep stdin open, parse stdout line-by-line as it arrives.
        // Each line is a JSON event, forwarded to the receiver.
        // Stop signal closes stdin and waits for child to exit.
        // Full implementation in the task subagent prompt.
        Err(AwareError::NotYetImplemented("CliInvoker::invoke_stream (waiting on real agent binaries in v0.5)"))
    }
}
```

For v0.3, `CliInvoker::invoke_stream` is stubbed because there are no agent binaries that produce streaming output yet. Tests exercise `MockInvoker::invoke_stream`. When v0.5 lands binaries, this gets filled in.

Test note: `CliInvoker::invoke_single` can be tested by pointing it at the `aware` binary itself (which has `--json` mode and can `agent list` etc.). But that's a stretch — better to leave it untested until v0.5 has real agents. For v0.3, we add ONE smoke test that asserts `CliInvoker::invoke_single` fails clearly when the binary doesn't exist.

Test, commit: `feat(cli): CliInvoker subprocess + JSON envelope`.

---

## Task 7: `cli::runtime::lifecycle` — stop broadcast + cleanup

**Files:** Create `cli/src/runtime/lifecycle.rs`. Modify `runtime/mod.rs`.

```rust
use tokio::sync::watch;

/// Sender end of the broadcast: orchestrator owns this, fires once on Ctrl+C or `app stop`.
pub type StopSender = watch::Sender<bool>;
/// Receiver end: each spawned task holds one; awaits `.changed()` to know when to stop.
pub type StopReceiver = watch::Receiver<bool>;

pub fn stop_channel() -> (StopSender, StopReceiver) {
    watch::channel(false)
}

/// Register Ctrl+C: when the signal arrives, set `stop` to true.
pub fn install_ctrl_c_handler(stop: StopSender) {
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            let _ = stop.send(true);
        }
    });
}
```

Test, commit: `feat(cli): runtime lifecycle stop channel + ctrl-c handler`.

---

## Task 8: `cli::runtime::pidfile`

**Files:** Create `cli/src/runtime/pidfile.rs`. Modify `runtime/mod.rs`.

```rust
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::error::AwareError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pidfile {
    pub app: String,
    pub instance: String,
    pub pid: u32,
    #[serde(rename = "started-at")]
    pub started_at: String,
    #[serde(rename = "run-id")]
    pub run_id: String,
}

pub fn write(pid: &Pidfile, instance_dir: &Path) -> Result<(), AwareError> {
    std::fs::create_dir_all(instance_dir)?;
    let body = serde_yaml::to_string(pid).map_err(|e| AwareError::Internal(format!("pidfile: {e}")))?;
    std::fs::write(instance_dir.join("pidfile.yaml"), body)?;
    Ok(())
}

pub fn read(instance_dir: &Path) -> Result<Pidfile, AwareError> {
    let body = std::fs::read_to_string(instance_dir.join("pidfile.yaml"))?;
    serde_yaml::from_str(&body).map_err(|e| AwareError::Validation(format!("pidfile parse: {e}")))
}

pub fn remove(instance_dir: &Path) {
    let _ = std::fs::remove_file(instance_dir.join("pidfile.yaml"));
}
```

Test, commit: `feat(cli): runtime pidfile (write/read for app stop)`.

---

## Task 9: `cli::runtime::context` — RuntimeContext for templating

**Files:** Create `cli/src/runtime/context.rs`. Modify `runtime/mod.rs`.

```rust
use std::collections::HashMap;
use std::path::Path;

use serde_json::{Map, Value};

use crate::error::AwareError;

#[derive(Default)]
pub struct RuntimeContext {
    pub inputs: Value,
    pub upstream: Map<String, Value>,   // raw — uses node id (may be kebab-case)
    pub secrets: Map<String, Value>,
    pub config: Map<String, Value>,
}

impl RuntimeContext {
    /// Insert an upstream output. Stored both under the raw id and an underscore-translated
    /// form so templates can use `{{ tekla_watch.mark }}` for a `tekla-watch` node.
    pub fn record_output(&mut self, node_id: &str, output: Value) {
        self.upstream.insert(node_id.to_string(), output.clone());
        let translated = node_id.replace('-', "_");
        if translated != node_id {
            self.upstream.insert(translated, output);
        }
    }

    /// Load a secret from `<creds_dir>/<id>.json` if present.
    pub fn load_secret(&mut self, creds_dir: &Path, id: &str) -> Result<(), AwareError> {
        let path = creds_dir.join(format!("{id}.json"));
        if !path.is_file() { return Ok(()); } // soft: missing secrets aren't fatal yet
        let body = std::fs::read_to_string(&path)?;
        let v: Value = serde_json::from_str(&body)
            .map_err(|e| AwareError::Validation(format!("secret {id}: {e}")))?;
        self.secrets.insert(id.to_string(), v);
        Ok(())
    }
}
```

Test (basic record/translate), commit: `feat(cli): runtime context (templating inputs collector)`.

---

## Task 10: `cli::runtime::orchestrator` — one-shot mode

**Files:** Create `cli/src/runtime/orchestrator.rs`. Modify `runtime/mod.rs`.

Handles all-stateless apps. Build the topology, execute nodes in topo order, write provenance, return when the last node completes.

Pseudocode (full code in task subagent prompt):

```
Orchestrator::run_one_shot(app, ctx, invoker, provenance_writer):
  emit RunStart
  topo = topo_order(app)
  for node in topo:
    emit NodeStart
    render config templates → args
    match node:
      agent invocation → invoker.invoke_single(agent, command, args) → output
      inline predicate → eval_predicate(code, last_upstream_event) → bool gate
      inline map → render template against event → new value
    if success: ctx.record_output(node.id, output); emit NodeOutput
    if fail: emit NodeError, abort or continue based on policy (v0.3 = abort)
  emit RunEnd ok
```

Add tests for a linear synthetic app (one stateless agent followed by another) using `MockInvoker`.

Test, commit: `feat(cli): orchestrator one-shot mode`.

---

## Task 11: Orchestrator long-running mode (single stateful watcher → stateless sink)

The hardest task. Welded-to-tc shape: `tekla-watch` (streams events) → `filter-welded` (inline predicate gates) → `tc-upload` (single per event).

Pseudocode:

```
Orchestrator::run_long_running(app, ctx, invoker, provenance_writer, stop_rx):
  emit RunStart
  // Identify stateful start nodes
  for node in app.nodes where node.agent.stateful == true && command.lifecycle == start:
    handle = invoker.invoke_stream(agent, command, args).await?
    spawn task: forward handle.rx events into a tokio mpsc::Sender keyed by node-id
  for stateless node: register as downstream-ready
  loop {
    select! {
      _ = stop_rx.changed() => break
      Some(event) = events_channel.recv() => {
        propagate event through downstream nodes:
          for each connection from this node:
            run downstream node with event as input
            if downstream is also stateful, output goes back through the channel
            if downstream is inline predicate, gate; if pass, propagate to NEXT downstream
            if downstream is single agent, invoker.invoke_single(...)
            emit NodeOutput / NodeError appropriately
      }
    }
  }
  // Clean shutdown
  for handle in stateful handles: handle.stop.send(())
  emit RunEnd interrupted (or ok if all sources naturally ended)
```

This is the substantive engineering. Tests:
- A 3-event mock watcher: each event passes the predicate, all 3 reach the upload mock
- An event that fails the predicate is dropped
- Ctrl+C mid-stream: stop event is emitted, RunEnd is "interrupted"

Test, commit: `feat(cli): orchestrator long-running mode (stateful → stateless)`.

---

## Task 12: Orchestrator fan-in synchronization (DAG)

qa-drawings-to-tekla shape: `pdf-extract` + `excel-lookup` → `match-build`. `match-build` declares `inputs: { drawing: { from: pdf-extract.drawing }, spec: { from: excel-lookup.row } }`. The node fires once both slots have a value; oldest pair fires first.

Implementation:
- Maintain per-input-slot queues for fan-in nodes
- On any upstream emit, push to the corresponding queue
- After every push, check: does every slot have at least one entry? If yes, pop oldest from each, call the downstream node, propagate output

Tests:
- Drawing arrives, then spec → fires once with both
- Two drawings, then one spec → fires once (first drawing + spec), one drawing still queued
- Spec arrives without ever pairing → never fires

Test, commit: `feat(cli): orchestrator DAG fan-in synchronization`.

---

## Task 13: Wire `aware app run` (one-shot path)

**Files:** Modify `cli/src/commands/app.rs`.

```rust
AppCommand::Run { app, instance, config } => {
    use crate::runtime::*;
    let instance = instance.unwrap_or_else(|| "default".into());
    let run_id = provenance::run_id_now();
    let instance_dir = ctx.paths.app_instance_dir(&app, &instance);
    let log_path = provenance::log_path_for(&ctx.paths.logs_dir(), &app, &instance, &run_id);

    let inv = invoker::CliInvoker { agents_dir: ctx.paths.agents_dir() };
    let orchestrator = orchestrator::Orchestrator::new(/* loaded app, ctx, inv, log path, run id */);
    orchestrator.run().await?;
    Ok(())
}
```

Tests:
- `aware app run welded-to-tc` with no MockInvoker hook fails with clear "agent binary not found" (CliInvoker invocation fails for tekla-watch since aware-tekla binary doesn't exist on PATH)
- Tests that exercise the orchestrator hooks live in unit tests against `MockInvoker`; the integration test for `aware app run` just verifies the dispatcher wiring + error pathway

Test, commit: `feat(cli): wire aware app run (one-shot path)`.

---

## Task 14: Wire `aware app run` long-running + write pidfile

Extend `app run` to:
- Detect long-running app (any stateful node)
- Write pidfile before starting
- Install Ctrl+C handler
- Block until stop
- Remove pidfile on exit

Test, commit: `feat(cli): wire aware app run (long-running + pidfile)`.

---

## Task 15: Wire `aware app stop`

**Files:** Modify `cli/src/commands/app.rs`.

```rust
AppCommand::Stop { app, instance } => {
    let instance = instance.unwrap_or_else(|| "default".into());
    let instance_dir = ctx.paths.app_instance_dir(&app, &instance);
    let pid = crate::runtime::pidfile::read(&instance_dir)?;
    // Send SIGTERM on unix, GenerateConsoleCtrlEvent on windows
    #[cfg(unix)]
    unsafe { libc::kill(pid.pid as i32, libc::SIGTERM); }
    #[cfg(windows)]
    {
        use std::process::Command;
        Command::new("taskkill").args(["/PID", &pid.pid.to_string(), "/T", "/F"]).status().ok();
    }
    crate::runtime::pidfile::remove(&instance_dir);
    println!("✓ stopped {app} (instance {instance})");
    Ok(())
}
```

Windows uses `taskkill /T /F` since clean Ctrl+C signaling across processes is fraught. For graceful stop the orchestrator also writes a sentinel file the running process polls — out of scope for v0.3.

Test, commit: `feat(cli): wire aware app stop`.

---

## Task 16: Wire `aware app logs` (default, --tail)

```rust
AppCommand::Logs { app, instance, tail } => logs(ctx, &app, instance.as_deref(), tail).await,
```

Default mode: read the most recent `.jsonl` and print it line-by-line (raw JSON).

`--tail`: open the most recent file, seek to end, poll for new lines (every 200ms), print as they arrive.

Test, commit: `feat(cli): wire aware app logs (default + --tail)`.

---

## Task 17: `aware app logs --replay --run-id <id>`

Read the named JSONL, pretty-print events with color-coded node labels and human timestamps.

Format example:
```
[14:03:42] ▶ run-start  welded-to-tc/default (run r_8c1d…)
[14:03:42] ▶ tekla-watch  (tekla/watch)
[14:03:43] → tekla-watch  output {mark: A-104, type: Welded}
[14:03:43] → filter-welded  pass
[14:03:43] → tc-upload  output {file-id: f_8u2k1}
[14:03:55] ✓ run-end  ok
```

Update the `AppCommand::Logs` arm to handle a new `--replay` flag (need to amend the AppCommand enum in commands/app.rs to add it).

Test, commit: `feat(cli): aware app logs --replay`.

---

## Task 18: Doctor `Running:` block

After the Integrity block, list active instances (any pidfile present anywhere under `<aware-home>/apps/<*>/instances/<*>/pidfile.yaml`). Show app name, instance, run-id, started-at.

Test, commit: `feat(cli): doctor Running block (active instances)`.

---

## Task 19: README + final verification

Update `cli/README.md`:
- Status section bumps to v0.3 (19 surfaces)
- Run section adds `aware app run <app>`, `aware app stop <app>`, `aware app logs <app>` examples
- New "Templating" subsection: explain kebab-case → underscore translation, `{{ inputs.X }}`, `{{ secrets.X }}`

Final checks:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `cargo build --release`
- Manual smoke: run `aware app run welded-to-tc` (will fail clear-cut at first agent invocation since binaries don't exist) and verify the error message is informative

Commit: `docs(cli): README quickstart for v0.3`.

---

## Self-review

**Spec coverage** — v0.3 def-of-done:
- `aware app run welded-to-tc` against mocked Tekla + TC → orchestrator + MockInvoker covered by Task 11 + 14
- Stateful start lifecycle + Ctrl+C stop → Task 7 + 11 + 14
- Fan-in synchronization (qa-drawings-to-tekla) → Task 12
- Templating substitutes upstream outputs + secrets → Task 3 + 9
- `aware app logs --replay` shows replayable trace → Task 17

**Realistic scope honesty:** 19 tasks is a lot. Each one is non-trivial. The plan is correct in shape; execution will surface issues (especially around async lifecycle + tokio cancellation semantics in Tasks 11/12). Build-test-iterate per task. Don't merge until all 19 commits land + tests are green.

**Things NOT covered in v0.3** (intentionally):
- Real OAuth-flow secret loading — v0.4 replaces the `load_secret` plain-JSON read
- Real agent binaries — v0.5 builds them; v0.3 just orchestrates whatever exists
- Pause/resume via persisted state — out of scope, will revisit if needed
- Distributed/cloud execution — out of scope forever (decalog #2: local-first)

---

## Execution handoff

Subagent-driven, same as v0.1 + v0.2. Each task: implementer → spec/quality check on big ones → commit → next.
