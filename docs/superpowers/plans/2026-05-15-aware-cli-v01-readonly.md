# AWARE CLI v0.1 (Read-Only Foundation) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship the v0.1 read-only foundation of the `aware` CLI binary — list, describe, and inspect installed AWARE agents and apps without any network calls, installs, or runtime execution.

**Architecture:** Rust 2024 / clap-derive routing. A `Paths` struct (driven by the `AWARE_HOME` env var, defaulting to `~/.aware/`) and a `Context` struct (carrying `Paths` + the global `--json` flag) thread through every dispatcher. `serde` + `serde_yaml` deserialize agent + app manifests into typed structs. A small `envelope` module formats `--json` output per the spec. A `render` module handles human-readable tables and ASCII topology. `doctor` is a filesystem health check.

**Tech Stack:** Rust 2024, clap 4.5 (derive + wrap_help), serde 1.0 (derive), serde_yaml 0.9, serde_json 1.0, thiserror 2.0, anyhow 1.0, dirs 6.0. Tests via assert_cmd 2.0 + predicates 3.1 + tempfile 3.14 (already in `Cargo.toml`).

**Scope discipline.** This plan delivers exactly the eight v0.1 surfaces from `10-core/cli-roadmap.md`:

- `aware --version` (already works via clap — verify only)
- `aware --help` (already works via clap — verify only)
- `aware doctor` (filesystem + config check only)
- `aware agent list`
- `aware agent describe <agent>`
- `aware agent skill <agent> <skill-name>`
- `aware app list`
- `aware app show <app>`

Everything else (`install`, `validate`, `run`, `connect`, `build`, `skill ...`, etc.) **remains `NotYetImplemented`** until its phase. Do not implement them in this plan.

**Forward note (do not act on in this plan).** The Rust-validation pass run during planning concluded: stick with Rust for v0.1–v0.4, but plan v0.5's DLL/NuGet/COM builders as a separate C# NativeAOT sidecar the Rust CLI invokes — PowerShell-reflection-by-subprocess is the one place Rust will bleed. Out of scope for v0.1.

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli/src/paths.rs` | `Paths` struct: `aware_home`, `agents_dir()`, `apps_dir()`, `config_path()`, `credentials_dir()`. Resolves `AWARE_HOME` env or defaults to `~/.aware/`. |
| `cli/src/context.rs` | `Context` struct passed to every dispatcher: `paths: Paths`, `json: bool`. |
| `cli/src/manifest/mod.rs` | Re-exports. |
| `cli/src/manifest/agent.rs` | `Agent` struct + nested types. `skill_count()`, `command_count()`, `kind()` helpers. |
| `cli/src/manifest/app.rs` | `App` struct + nested `Node`, `Connection`, `Inline`. `is_dag()`, `is_stateful()` helpers. |
| `cli/src/manifest/loader.rs` | `discover_agents(&Paths)`, `discover_apps(&Paths)`, `load_agent(&Path)`, `load_app(&Path)`. |
| `cli/src/envelope.rs` | `Envelope<T>` matching the spec JSON shape + `emit(ctx, command, payload, started)` helper. |
| `cli/src/render/mod.rs` | Re-exports. |
| `cli/src/render/table.rs` | `Table` builder for column-aligned text output. |
| `cli/src/render/topology.rs` | `format_topology(&App) -> String` — ASCII topology for `app show`. |
| `cli/tests/common/mod.rs` | Shared test fixture helper (one-time copy of `20-agents/` + `30-apps/_examples/` into a TempDir, exposed via `OnceLock`). |
| `cli/tests/agent_list.rs` | Integration tests for `aware agent list`. |
| `cli/tests/agent_describe.rs` | Integration tests for `aware agent describe`. |
| `cli/tests/agent_skill.rs` | Integration tests for `aware agent skill`. |
| `cli/tests/app_list.rs` | Integration tests for `aware app list`. |
| `cli/tests/app_show.rs` | Integration tests for `aware app show`. |
| `cli/tests/doctor.rs` | Integration tests for `aware doctor`. |

### Modified files

| Path | Change |
|---|---|
| `cli/src/main.rs` | Add module declarations. Build `Context` from `Cli`. Pass `&Context` to every dispatcher. |
| `cli/src/commands/agent.rs` | Replace `NotYetImplemented` for `List`, `Describe`, `Skill`. New signature `dispatch(cmd, ctx)`. |
| `cli/src/commands/app.rs` | Replace `NotYetImplemented` for `List`, `Show`. New signature. |
| `cli/src/commands/doctor.rs` | Replace `NotYetImplemented` with filesystem health check. New signature `run(ctx)`. |
| `cli/src/commands/connect.rs`, `cli/src/commands/skill.rs`, `cli/src/commands/build.rs` | Signature only — accept `&Context`, still return `NotYetImplemented`. |
| `cli/tests/basic.rs` | Update `unimplemented_subcommand_exits_nonzero_with_message` to target a still-stubbed command (e.g. `agent install`). |
| `cli/README.md` | Quickstart update: `AWARE_HOME` env var, the four working surfaces. |

### Key data shapes (verified against fixtures)

Agent manifest required fields: `agent`, `version`, `description`, `stateful`, `license`, `transport`, `commands`. Optional: `display-name`, `vendor`, `homepage`, `keywords`, `provenance`, `requires`, `skills`. Fixture sample: `20-agents/aeco/engineering/tekla/manifest.yaml` (31 skills, 3 commands, stateful: true).

App manifest required fields: `app`, `version`, `description`, `nodes`, `connections`, `requires`. Optional: `display-name`, `exposes-as-agent`, `exposed-commands`, `requires-permissions`, `layout`, `skills`. Fixture samples: `30-apps/_examples/welded-to-tc.flo` (3 nodes, 2 connections, linear), `30-apps/_examples/qa-drawings-to-tekla.flo` (7 nodes, 6 connections, DAG).

`agent list` "KIND" column: lowercased `display-name`, fallback to `agent` id, truncated at 17 chars. This deviates slightly from the spec example output (which has a curated taxonomy not derivable from any single manifest field). Document the deviation in the README.

---

## Task 0: Prerequisites — Rust toolchain + scaffold builds

No file changes. This is environment verification.

**Files:** none.

- [ ] **Step 1: Check toolchain present**

```powershell
rustc --version
cargo --version
```

Expected: both print a version. If `rustc` is missing:

```powershell
winget install --id Rustlang.Rustup -e
rustup default stable
```

Then re-open the shell and re-run `rustc --version`. Confirm `rustup` reports `default toolchain: stable-x86_64-pc-windows-msvc` (or platform equivalent).

- [ ] **Step 2: Build the scaffold (debug)**

```powershell
cd cli
cargo build
```

Expected: build succeeds, `target/debug/aware.exe` created. First build pulls ~30 crates — slow once, fast afterwards.

- [ ] **Step 3: Run the scaffold tests**

```powershell
cargo test
```

Expected: all 6 tests from `cli/tests/basic.rs` pass:
- `top_level_help_works`
- `version_prints_dev_version`
- `agent_subcommand_help_works`
- `app_subcommand_help_works`
- `unimplemented_subcommand_exits_nonzero_with_message`
- `missing_subcommand_shows_help`

If any fail, **stop and fix before continuing**. The scaffold is the baseline.

- [ ] **Step 4: Smoke the binary manually**

```powershell
cargo run -- --version
cargo run -- --help
cargo run -- agent list
```

Expected:
- `--version` prints `aware 0.1.0-dev`
- `--help` prints subcommand list
- `agent list` exits non-zero, stderr contains `not yet implemented: agent list`

No commit. This task is environment readiness only.

---

## Task 1: `cli::paths` module

The `Paths` struct resolves `~/.aware/` paths. `AWARE_HOME` env var overrides the default (critical for integration tests that must not pollute the real user home).

**Files:**
- Create: `cli/src/paths.rs`
- Modify: `cli/src/main.rs` (add `mod paths;`)

- [ ] **Step 1: Write the failing test**

Add at the bottom of `cli/src/paths.rs` (create the file):

```rust
//! Filesystem path resolution for the AWARE CLI.
//!
//! `AWARE_HOME` env var overrides the default `~/.aware/` location.
//! Tests rely on this override to avoid polluting the real home dir.

use std::path::PathBuf;

use crate::error::AwareError;

#[derive(Debug, Clone)]
pub struct Paths {
    pub aware_home: PathBuf,
}

impl Paths {
    pub fn from_env() -> Result<Self, AwareError> {
        let aware_home = match std::env::var_os("AWARE_HOME") {
            Some(p) => PathBuf::from(p),
            None => dirs::home_dir()
                .ok_or_else(|| {
                    AwareError::Internal("could not determine home directory".into())
                })?
                .join(".aware"),
        };
        Ok(Self { aware_home })
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.aware_home.join("agents")
    }

    pub fn apps_dir(&self) -> PathBuf {
        self.aware_home.join("apps")
    }

    pub fn config_path(&self) -> PathBuf {
        self.aware_home.join("config.yaml")
    }

    pub fn credentials_dir(&self) -> PathBuf {
        self.aware_home.join("credentials")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(home: &str) -> Paths {
        Paths { aware_home: PathBuf::from(home) }
    }

    #[test]
    fn agents_dir_appends_agents() {
        assert_eq!(p("/x").agents_dir(), PathBuf::from("/x/agents"));
    }

    #[test]
    fn apps_dir_appends_apps() {
        assert_eq!(p("/x").apps_dir(), PathBuf::from("/x/apps"));
    }

    #[test]
    fn config_path_appends_config_yaml() {
        assert_eq!(p("/x").config_path(), PathBuf::from("/x/config.yaml"));
    }

    #[test]
    fn credentials_dir_appends_credentials() {
        assert_eq!(p("/x").credentials_dir(), PathBuf::from("/x/credentials"));
    }
}
```

Add to `cli/src/main.rs` just below `mod commands;`:

```rust
mod paths;
```

- [ ] **Step 2: Run tests**

```powershell
cargo test --lib paths
```

Expected: 4 tests pass.

- [ ] **Step 3: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/paths.rs cli/src/main.rs
git commit -m "feat(cli): add paths module with AWARE_HOME resolution"
```

---

## Task 2: `cli::context` + thread through dispatchers

Every dispatcher needs the `Paths` + `--json` flag. A `Context` struct keeps the signatures uniform.

**Files:**
- Create: `cli/src/context.rs`
- Modify: `cli/src/main.rs`, `cli/src/commands/agent.rs`, `cli/src/commands/app.rs`, `cli/src/commands/connect.rs`, `cli/src/commands/skill.rs`, `cli/src/commands/build.rs`, `cli/src/commands/doctor.rs`

- [ ] **Step 1: Create the Context struct**

`cli/src/context.rs`:

```rust
//! Per-invocation context handed to every dispatcher.

use crate::paths::Paths;

#[derive(Debug, Clone)]
pub struct Context {
    pub paths: Paths,
    pub json: bool,
}
```

Add to `cli/src/main.rs` near other module declarations:

```rust
mod context;
```

- [ ] **Step 2: Rewrite main() to build and pass Context**

Replace the body of `fn main()` in `cli/src/main.rs`:

```rust
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let paths = match crate::paths::Paths::from_env() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(err.exit_code());
        }
    };
    let ctx = crate::context::Context { paths, json: cli.json };

    let result: Result<(), AwareError> = match cli.command {
        Command::Agent { action } => commands::agent::dispatch(action, &ctx),
        Command::App { action } => commands::app::dispatch(action, &ctx),
        Command::Connect(args) => commands::connect::run_connect(args, &ctx),
        Command::Disconnect(args) => commands::connect::run_disconnect(args, &ctx),
        Command::Skill { action } => commands::skill::dispatch(action, &ctx),
        Command::Build { action } => commands::build::dispatch(action, &ctx),
        Command::Doctor => commands::doctor::run(&ctx),
    };

    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(err.exit_code());
        }
    }
}
```

- [ ] **Step 3: Update every dispatcher signature**

Each `dispatch(cmd) -> Result<(), AwareError>` becomes `dispatch(cmd, ctx: &Context) -> Result<(), AwareError>`. Stub bodies keep returning `NotYetImplemented`.

`cli/src/commands/agent.rs` — change the `dispatch` signature and body:

```rust
use crate::context::Context;

pub fn dispatch(cmd: AgentCommand, _ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AgentCommand::List => Err(AwareError::NotYetImplemented("agent list")),
        AgentCommand::Describe { .. } => Err(AwareError::NotYetImplemented("agent describe")),
        AgentCommand::Skill { .. } => Err(AwareError::NotYetImplemented("agent skill")),
        AgentCommand::Install { .. } => Err(AwareError::NotYetImplemented("agent install")),
        AgentCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("agent uninstall")),
        AgentCommand::Update { .. } => Err(AwareError::NotYetImplemented("agent update")),
        AgentCommand::Validate { .. } => Err(AwareError::NotYetImplemented("agent validate")),
        AgentCommand::Publish { .. } => Err(AwareError::NotYetImplemented("agent publish")),
    }
}
```

Repeat the same `_ctx: &Context` addition for `app.rs`, `connect.rs` (both `run_connect` and `run_disconnect`), `skill.rs`, `build.rs`, and `doctor.rs` (where it becomes `pub fn run(_ctx: &Context)`).

For `connect.rs`, the existing signatures take `ConnectArgs` / `DisconnectArgs` — add `&Context` as the second parameter:

```rust
pub fn run_connect(_args: ConnectArgs, _ctx: &Context) -> Result<(), AwareError> { ... }
pub fn run_disconnect(_args: DisconnectArgs, _ctx: &Context) -> Result<(), AwareError> { ... }
```

- [ ] **Step 4: Verify the scaffold still passes**

```powershell
cargo build
cargo test
```

Expected: same 6 tests still pass. `agent list` still returns `NotYetImplemented` — the routing is unchanged, only the signatures shifted.

- [ ] **Step 5: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/context.rs cli/src/main.rs cli/src/commands/
git commit -m "feat(cli): thread Context through every dispatcher"
```

---

## Task 3: `cli::manifest::agent` — typed Agent struct

Deserialize agent manifests with serde. Cover all fields seen across the 7 fixtures.

**Files:**
- Create: `cli/src/manifest/mod.rs`, `cli/src/manifest/agent.rs`
- Modify: `cli/src/main.rs` (add `mod manifest;`)

- [ ] **Step 1: Create the module skeleton**

`cli/src/manifest/mod.rs`:

```rust
//! Typed deserialization of AWARE agent + app manifests.
//!
//! Shapes verified against the 7 reference agents under `20-agents/` and
//! the 2 reference apps under `30-apps/_examples/`.

pub mod agent;
pub mod app;
pub mod loader;

pub use agent::Agent;
pub use app::App;
```

Add to `cli/src/main.rs`:

```rust
mod manifest;
```

- [ ] **Step 2: Define the Agent struct**

`cli/src/manifest/agent.rs`:

```rust
use std::collections::BTreeMap;

use serde::Deserialize;
use serde_yaml::Value;

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub agent: String,
    pub version: String,
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    pub description: String,
    pub stateful: bool,
    pub vendor: Option<String>,
    pub license: String,
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub provenance: Option<Provenance>,
    pub requires: Option<Requires>,
    pub transport: Transport,
    #[serde(default)]
    pub commands: BTreeMap<String, Command>,
    #[serde(default)]
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Provenance {
    #[serde(rename = "generated-by")]
    pub generated_by: Option<String>,
    #[serde(rename = "generator-version")]
    pub generator_version: Option<String>,
    pub source: Option<Value>,
    #[serde(rename = "refined-by", default)]
    pub refined_by: Vec<String>,
    #[serde(rename = "generated-at")]
    pub generated_at: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Requires {
    #[serde(default)]
    pub filesystem: Vec<Value>,
    #[serde(default)]
    pub network: Vec<String>,
    #[serde(default)]
    pub software: Vec<String>,
    #[serde(default)]
    pub secrets: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Transport {
    pub cli: Option<TransportCli>,
    pub mcp: Option<Value>,
    pub rest: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct TransportCli {
    pub binary: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub lifecycle: Lifecycle,
    pub description: String,
    #[serde(default)]
    pub inputs: Value,
    pub outputs: Option<Value>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    Start,
    Stop,
    Single,
}

impl Agent {
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }
    /// Human-readable kind label for the `agent list` table: lowercased
    /// `display-name`, falling back to `agent` id, truncated to 17 chars.
    pub fn kind(&self) -> String {
        let raw = self
            .display_name
            .as_ref()
            .map(|d| d.to_lowercase())
            .unwrap_or_else(|| self.agent.clone());
        if raw.chars().count() > 17 {
            let mut s: String = raw.chars().take(14).collect();
            s.push_str("...");
            s
        } else {
            raw
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEKLA_MIN: &str = r#"
agent: tekla
version: 2025.0.1
display-name: Tekla Structures
description: |
  Watches the active Tekla model.
stateful: true
license: Apache-2.0
transport:
  cli:
    binary: aware-tekla
commands:
  watch:
    lifecycle: start
    description: Subscribe.
    outputs:
      type: stream
  insert:
    lifecycle: single
    description: Create.
skills:
  - drawing-identity.md
  - event-threading.md
"#;

    #[test]
    fn parses_minimal_manifest() {
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert_eq!(a.agent, "tekla");
        assert_eq!(a.version, "2025.0.1");
        assert!(a.stateful);
        assert_eq!(a.license, "Apache-2.0");
        assert_eq!(a.skill_count(), 2);
        assert_eq!(a.command_count(), 2);
        assert_eq!(a.transport.cli.unwrap().binary, "aware-tekla");
    }

    #[test]
    fn kind_uses_lowercased_display_name() {
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert_eq!(a.kind(), "tekla structures");
    }

    #[test]
    fn kind_truncates_long_names() {
        let yaml = TEKLA_MIN.replace("Tekla Structures", "Some Extremely Long Name For The Agent");
        let a: Agent = serde_yaml::from_str(&yaml).unwrap();
        let k = a.kind();
        assert!(k.chars().count() <= 17, "kind too long: {k:?}");
        assert!(k.ends_with("..."));
    }

    #[test]
    fn kind_falls_back_to_agent_id_when_no_display_name() {
        let yaml = r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: {}
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(a.kind(), "tekla");
    }

    #[test]
    fn missing_required_field_errors() {
        let bad = "agent: tekla\nversion: 1.0";
        assert!(serde_yaml::from_str::<Agent>(bad).is_err());
    }

    #[test]
    fn parses_real_tekla_manifest() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("20-agents/aeco/engineering/tekla/manifest.yaml");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: Agent = serde_yaml::from_str(&text).unwrap();
        assert_eq!(a.agent, "tekla");
        assert_eq!(a.skill_count(), 31);
        assert_eq!(a.command_count(), 3);
        assert!(a.stateful);
    }
}
```

- [ ] **Step 3: Run tests**

```powershell
cargo test --lib manifest::agent
```

Expected: 6 tests pass, including `parses_real_tekla_manifest` which loads the actual fixture.

- [ ] **Step 4: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/manifest/ cli/src/main.rs
git commit -m "feat(cli): add Agent manifest deserialization"
```

---

## Task 4: `cli::manifest::app` — typed App struct

Same approach for app manifests. Covers both `linear` and `dag` layouts and both kinds of nodes (agent invocations and inline glue).

**Files:**
- Create: `cli/src/manifest/app.rs`

- [ ] **Step 1: Define the App struct**

`cli/src/manifest/app.rs`:

```rust
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
    #[serde(rename = "exposed-commands", default)]
    pub exposed_commands: Option<Value>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(rename = "requires-permissions")]
    pub requires_permissions: Option<Value>,
    #[serde(default = "default_layout")]
    pub layout: Layout,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub connections: Vec<Connection>,
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

#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: String,
    pub agent: Option<String>,
    pub command: Option<String>,
    pub inline: Option<Inline>,
    pub row: Option<u32>,
    pub col: Option<u32>,
    #[serde(default)]
    pub config: Value,
    #[serde(default)]
    pub inputs: Value,
}

#[derive(Debug, Deserialize)]
pub struct Inline {
    pub kind: String,
    pub description: String,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub input: Option<String>,
}

impl App {
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
            .parent().unwrap()
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
            .parent().unwrap()
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
            .parent().unwrap()
            .join("30-apps/_examples/welded-to-tc.flo");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: App = serde_yaml::from_str(&text).unwrap();
        let inline = a.nodes.iter().find(|n| n.id == "filter-welded").unwrap();
        assert!(inline.inline.is_some());
        assert_eq!(inline.inline.as_ref().unwrap().kind, "predicate");
    }
}
```

- [ ] **Step 2: Run tests**

```powershell
cargo test --lib manifest::app
```

Expected: 3 tests pass.

- [ ] **Step 3: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/manifest/app.rs
git commit -m "feat(cli): add App manifest deserialization"
```

---

## Task 5: `cli::manifest::loader` — discovery + load helpers

Walk `~/.aware/agents/` and `~/.aware/apps/` and return parsed manifests, ignoring (with a clear error) entries that fail to parse.

**Files:**
- Create: `cli/src/manifest/loader.rs`

- [ ] **Step 1: Define the loader API**

`cli/src/manifest/loader.rs`:

```rust
use std::path::{Path, PathBuf};

use crate::error::AwareError;
use crate::manifest::{Agent, App};
use crate::paths::Paths;

/// A discovered agent on disk, with its source path retained for `describe`/`skill` commands.
#[derive(Debug)]
pub struct DiscoveredAgent {
    pub manifest: Agent,
    pub root: PathBuf,
}

/// A discovered app on disk, with its source path retained.
#[derive(Debug)]
pub struct DiscoveredApp {
    pub manifest: App,
    pub root: PathBuf,
    pub manifest_path: PathBuf,
}

/// Walk `<aware_home>/agents/` one level deep. Each subdir containing a
/// `manifest.yaml` is an installed agent. Returns discovered agents sorted
/// by id. Missing `agents/` directory returns an empty Vec (not an error).
pub fn discover_agents(paths: &Paths) -> Result<Vec<DiscoveredAgent>, AwareError> {
    let agents_dir = paths.agents_dir();
    if !agents_dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&agents_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let root = entry.path();
        let manifest_path = root.join("manifest.yaml");
        if !manifest_path.is_file() {
            continue;
        }
        let manifest = load_agent(&manifest_path)?;
        out.push(DiscoveredAgent { manifest, root });
    }
    out.sort_by(|a, b| a.manifest.agent.cmp(&b.manifest.agent));
    Ok(out)
}

/// Walk `<aware_home>/apps/` one level deep. Each subdir containing a
/// `.flo` or `.app` file is an installed app.
pub fn discover_apps(paths: &Paths) -> Result<Vec<DiscoveredApp>, AwareError> {
    let apps_dir = paths.apps_dir();
    if !apps_dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&apps_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let root = entry.path();
        let manifest_path = match find_app_manifest(&root) {
            Some(p) => p,
            None => continue,
        };
        let manifest = load_app(&manifest_path)?;
        out.push(DiscoveredApp { manifest, root, manifest_path });
    }
    out.sort_by(|a, b| a.manifest.app.cmp(&b.manifest.app));
    Ok(out)
}

fn find_app_manifest(root: &Path) -> Option<PathBuf> {
    // Preferred: <root>/<dir-name>.flo, then any *.flo, then any *.app.
    let dir_name = root.file_name()?.to_string_lossy().to_string();
    let canonical = root.join(format!("{dir_name}.flo"));
    if canonical.is_file() {
        return Some(canonical);
    }
    for ext in ["flo", "app"] {
        for entry in std::fs::read_dir(root).ok()?.flatten() {
            let p = entry.path();
            if p.extension().is_some_and(|e| e == ext) {
                return Some(p);
            }
        }
    }
    None
}

pub fn load_agent(manifest_path: &Path) -> Result<Agent, AwareError> {
    let text = std::fs::read_to_string(manifest_path)?;
    let parsed: Agent = serde_yaml::from_str(&text).map_err(|e| {
        AwareError::Validation(format!("{}: {e}", manifest_path.display()))
    })?;
    Ok(parsed)
}

pub fn load_app(manifest_path: &Path) -> Result<App, AwareError> {
    let text = std::fs::read_to_string(manifest_path)?;
    let parsed: App = serde_yaml::from_str(&text).map_err(|e| {
        AwareError::Validation(format!("{}: {e}", manifest_path.display()))
    })?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_paths() -> Paths {
        // Use the repo's 20-agents/30-apps directories via a synthetic
        // aware_home that already points at the layout we expect.
        // We rebuild a temp layout matching ~/.aware structure.
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(aware.join("agents/tekla")).unwrap();
        std::fs::create_dir_all(aware.join("apps/welded-to-tc")).unwrap();

        let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
        let src_manifest = repo_root.join("20-agents/aeco/engineering/tekla/manifest.yaml");
        std::fs::copy(&src_manifest, aware.join("agents/tekla/manifest.yaml")).unwrap();

        let src_flo = repo_root.join("30-apps/_examples/welded-to-tc.flo");
        std::fs::copy(&src_flo, aware.join("apps/welded-to-tc/welded-to-tc.flo")).unwrap();

        // Leak the TempDir intentionally so files stay alive for the test.
        let aware = aware.clone();
        std::mem::forget(tmp);
        Paths { aware_home: aware }
    }

    #[test]
    fn discovers_agents() {
        let paths = fixtures_paths();
        let agents = discover_agents(&paths).unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].manifest.agent, "tekla");
    }

    #[test]
    fn discovers_apps() {
        let paths = fixtures_paths();
        let apps = discover_apps(&paths).unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].manifest.app, "welded-to-tc");
    }

    #[test]
    fn missing_agents_dir_returns_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths { aware_home: tmp.path().join("nope") };
        let agents = discover_agents(&paths).unwrap();
        assert!(agents.is_empty());
    }
}
```

- [ ] **Step 2: Run tests**

```powershell
cargo test --lib manifest::loader
```

Expected: 3 tests pass.

- [ ] **Step 3: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/manifest/loader.rs
git commit -m "feat(cli): add manifest discovery + load helpers"
```

---

## Task 6: `cli::envelope` — JSON output shape

Wraps any payload in the `{ok, data, error, meta}` envelope from the spec.

**Files:**
- Create: `cli/src/envelope.rs`
- Modify: `cli/src/main.rs` (add `mod envelope;`)

- [ ] **Step 1: Create the module**

`cli/src/envelope.rs`:

```rust
//! JSON envelope per cli-spec.md § Response envelope.

use std::time::Instant;

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Envelope<T: Serialize> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<EnvelopeError>,
    pub meta: Meta,
}

#[derive(Serialize)]
pub struct EnvelopeError {
    pub code: String,
    pub message: String,
    pub details: Value,
}

#[derive(Serialize)]
pub struct Meta {
    #[serde(rename = "cli-version")]
    pub cli_version: &'static str,
    pub command: String,
    #[serde(rename = "duration-ms")]
    pub duration_ms: u128,
}

pub fn meta_for(command: &str, started: Instant) -> Meta {
    Meta {
        cli_version: env!("CARGO_PKG_VERSION"),
        command: command.to_string(),
        duration_ms: started.elapsed().as_millis(),
    }
}

/// Print a successful envelope to stdout.
pub fn print_ok<T: Serialize>(command: &str, data: T, started: Instant) -> std::io::Result<()> {
    let env = Envelope {
        ok: true,
        data: Some(data),
        error: None,
        meta: meta_for(command, started),
    };
    println!("{}", serde_json::to_string(&env).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_envelope_round_trips() {
        let env = Envelope {
            ok: true,
            data: Some(serde_json::json!({"n": 7})),
            error: None,
            meta: Meta {
                cli_version: "0.1.0",
                command: "agent list".into(),
                duration_ms: 1,
            },
        };
        let s = serde_json::to_string(&env).unwrap();
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["data"]["n"], 7);
        assert!(v["error"].is_null());
        assert_eq!(v["meta"]["command"], "agent list");
    }
}
```

Add to `cli/src/main.rs`:

```rust
mod envelope;
```

- [ ] **Step 2: Run tests**

```powershell
cargo test --lib envelope
```

Expected: 1 test passes.

- [ ] **Step 3: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/envelope.rs cli/src/main.rs
git commit -m "feat(cli): add JSON envelope module"
```

---

## Task 7: `cli::render::table` — column-aligned text output

Used by `agent list`, `app list`, and `doctor` counts.

**Files:**
- Create: `cli/src/render/mod.rs`, `cli/src/render/table.rs`
- Modify: `cli/src/main.rs` (add `mod render;`)

- [ ] **Step 1: Create the table module**

`cli/src/render/mod.rs`:

```rust
pub mod table;
pub mod topology;
```

`cli/src/render/table.rs`:

```rust
//! Minimal column-aligned text table. Pads each column to the max width of
//! its values plus 2 spaces. No external dep needed for v0.1.

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            headers: headers.into_iter().map(Into::into).collect(),
            rows: Vec::new(),
        }
    }

    pub fn row(&mut self, row: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    pub fn render(&self) -> String {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.chars().count()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.chars().count());
                }
            }
        }

        let mut out = String::new();
        let format_row = |row: &[String]| -> String {
            let mut line = String::new();
            for (i, cell) in row.iter().enumerate() {
                let w = widths[i];
                let pad = w.saturating_sub(cell.chars().count());
                line.push_str(cell);
                line.push_str(&" ".repeat(pad));
                if i + 1 < row.len() {
                    line.push_str("  ");
                }
            }
            line.trim_end().to_string()
        };

        out.push_str(&format_row(&self.headers));
        out.push('\n');
        for row in &self.rows {
            out.push_str(&format_row(row));
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_aligned_table() {
        let mut t = Table::new(["ID", "VERSION"]);
        t.row(["tekla", "2025.0.1"]).row(["html-report", "1.0.0"]);
        let s = t.render();
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines[0], "ID           VERSION");
        assert_eq!(lines[1], "tekla        2025.0.1");
        assert_eq!(lines[2], "html-report  1.0.0");
    }

    #[test]
    fn empty_table_renders_just_header() {
        let t = Table::new(["A", "B"]);
        let s = t.render();
        assert_eq!(s, "A  B\n");
    }
}
```

Add to `cli/src/main.rs`:

```rust
mod render;
```

- [ ] **Step 2: Run tests**

```powershell
cargo test --lib render::table
```

Expected: 2 tests pass.

- [ ] **Step 3: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/render/ cli/src/main.rs
git commit -m "feat(cli): add table renderer"
```

---

## Task 8: `cli::render::topology` — ASCII topology for `app show`

Linear apps render as `node1 → node2 → node3`. DAG apps render as a node list plus an edge list. Inline glue nodes are tagged `[inline]`.

**Files:**
- Create: `cli/src/render/topology.rs`

- [ ] **Step 1: Create the topology renderer**

`cli/src/render/topology.rs`:

```rust
//! ASCII topology rendering for `aware app show`.

use crate::manifest::App;
use crate::manifest::app::Layout;

pub fn format_topology(app: &App) -> String {
    match app.layout {
        Layout::Linear => format_linear(app),
        Layout::Dag => format_dag(app),
    }
}

fn node_label(node: &crate::manifest::app::Node) -> String {
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
    // Follow `connections:` in order: pick the source with no incoming edge,
    // then walk forward. Fall back to the natural node order if the graph
    // doesn't look linear.
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
        let input = c.input.as_ref().map(|i| format!(" → input:{i}")).unwrap_or_default();
        out.push_str(&format!("  {} → {}  [{label}]{input}\n", c.from, c.to));
    }
    out
}

/// Heuristic topo sort for the linear case: source first, walk by edges.
fn topological_order<'a>(app: &'a App) -> Vec<&'a crate::manifest::app::Node> {
    use std::collections::{HashMap, HashSet};
    let mut by_id: HashMap<&str, &crate::manifest::app::Node> =
        app.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
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

    let mut out = Vec::new();
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
            .parent().unwrap()
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
        assert!(pos_watch < pos_filter && pos_filter < pos_upload, "flow order broken: {s}");
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
```

- [ ] **Step 2: Run tests**

```powershell
cargo test --lib render::topology
```

Expected: 2 tests pass.

- [ ] **Step 3: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/render/topology.rs
git commit -m "feat(cli): add ASCII topology renderer"
```

---

## Task 9: Implement `aware agent list`

Now the foundation is in place. From here on, each task is: integration test → run → fail → implement → run → pass → commit.

**Files:**
- Create: `cli/tests/common/mod.rs`, `cli/tests/agent_list.rs`
- Modify: `cli/src/commands/agent.rs`

- [ ] **Step 1: Add the shared test fixture helper**

`cli/tests/common/mod.rs`:

```rust
//! Shared integration-test fixtures.
//!
//! Each test process gets one TempDir populated from <repo>/20-agents/
//! and <repo>/30-apps/_examples/, mirrored into `<tmp>/agents/<id>/` and
//! `<tmp>/apps/<id>/` (flat — the install layout, not the repo layout).
//!
//! Tests set AWARE_HOME=<that tmp dir> and run the binary read-only.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use tempfile::TempDir;

static FIXTURE: OnceLock<TempDir> = OnceLock::new();

pub fn aware_home() -> &'static Path {
    FIXTURE.get_or_init(populate).path()
}

fn populate() -> TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().expect("repo root").to_path_buf();

    std::fs::create_dir_all(tmp.path().join("agents")).unwrap();
    std::fs::create_dir_all(tmp.path().join("apps")).unwrap();

    // Walk 20-agents/ looking for manifest.yaml; for each, copy the agent
    // folder (manifest + skills + commands subdirs) into <tmp>/agents/<id>/.
    for manifest_path in find_manifests(&repo_root.join("20-agents"), "manifest.yaml") {
        let src_dir = manifest_path.parent().unwrap();
        let agent_id = src_dir.file_name().unwrap();
        let dst_dir = tmp.path().join("agents").join(agent_id);
        copy_dir_recursive(src_dir, &dst_dir).unwrap();
    }

    // Apps: each .flo in 30-apps/_examples/ becomes <tmp>/apps/<stem>/<stem>.flo
    let apps_src = repo_root.join("30-apps/_examples");
    for entry in std::fs::read_dir(&apps_src).unwrap().flatten() {
        let p = entry.path();
        if p.extension().is_some_and(|e| e == "flo" || e == "app") {
            let stem = p.file_stem().unwrap().to_string_lossy().to_string();
            let dst_dir = tmp.path().join("apps").join(&stem);
            std::fs::create_dir_all(&dst_dir).unwrap();
            std::fs::copy(&p, dst_dir.join(p.file_name().unwrap())).unwrap();
        }
    }

    tmp
}

fn find_manifests(root: &Path, name: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        let read = match std::fs::read_dir(&d) { Ok(r) => r, Err(_) => continue };
        for entry in read.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n == name) {
                out.push(p);
            }
        }
    }
    out
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
```

- [ ] **Step 2: Write the failing integration test**

`cli/tests/agent_list.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn lists_all_seven_fixture_agents() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"))
        .stdout(predicate::str::contains("VERSION"))
        .stdout(predicate::str::contains("KIND"))
        .stdout(predicate::str::contains("SKILLS"))
        .stdout(predicate::str::contains("COMMANDS"))
        .stdout(predicate::str::contains("tekla"))
        .stdout(predicate::str::contains("trimble-connect"))
        .stdout(predicate::str::contains("microsoft-365"))
        .stdout(predicate::str::contains("google-workspace"))
        .stdout(predicate::str::contains("html-report"))
        .stdout(predicate::str::contains("aware-agent-builder"))
        .stdout(predicate::str::contains("aware-skill-builder"));
}

#[test]
fn empty_home_lists_nothing_but_header() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"));
}

#[test]
fn json_output_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let body = std::str::from_utf8(&output).unwrap();
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["meta"]["command"], "agent list");
    let agents = v["data"]["agents"].as_array().unwrap();
    assert_eq!(agents.len(), 7);
    assert!(agents.iter().any(|a| a["id"] == "tekla"));
}
```

- [ ] **Step 3: Run — expect failure**

```powershell
cargo test --test agent_list
```

Expected: tests fail. `agent list` still returns `NotYetImplemented`.

- [ ] **Step 4: Implement `agent list`**

Replace the `AgentCommand::List` branch in `cli/src/commands/agent.rs::dispatch` and add the implementation:

```rust
use std::time::Instant;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::manifest::loader::discover_agents;
use crate::render::table::Table;

// ... existing AgentCommand enum unchanged ...

pub fn dispatch(cmd: AgentCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AgentCommand::List => list(ctx),
        AgentCommand::Describe { .. } => Err(AwareError::NotYetImplemented("agent describe")),
        AgentCommand::Skill { .. } => Err(AwareError::NotYetImplemented("agent skill")),
        AgentCommand::Install { .. } => Err(AwareError::NotYetImplemented("agent install")),
        AgentCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("agent uninstall")),
        AgentCommand::Update { .. } => Err(AwareError::NotYetImplemented("agent update")),
        AgentCommand::Validate { .. } => Err(AwareError::NotYetImplemented("agent validate")),
        AgentCommand::Publish { .. } => Err(AwareError::NotYetImplemented("agent publish")),
    }
}

#[derive(Serialize)]
struct AgentListRow {
    id: String,
    version: String,
    kind: String,
    skills: usize,
    commands: usize,
}

#[derive(Serialize)]
struct AgentListData {
    agents: Vec<AgentListRow>,
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_agents(&ctx.paths)?;

    if ctx.json {
        let data = AgentListData {
            agents: discovered.iter().map(|d| AgentListRow {
                id: d.manifest.agent.clone(),
                version: d.manifest.version.clone(),
                kind: d.manifest.kind(),
                skills: d.manifest.skill_count(),
                commands: d.manifest.command_count(),
            }).collect(),
        };
        envelope::print_ok("agent list", data, started).ok();
        return Ok(());
    }

    let mut t = Table::new(["ID", "VERSION", "KIND", "SKILLS", "COMMANDS"]);
    for d in &discovered {
        t.row([
            d.manifest.agent.clone(),
            d.manifest.version.clone(),
            d.manifest.kind(),
            d.manifest.skill_count().to_string(),
            d.manifest.command_count().to_string(),
        ]);
    }
    print!("{}", t.render());
    Ok(())
}
```

- [ ] **Step 5: Run — expect pass**

```powershell
cargo test --test agent_list
```

Expected: all 3 tests pass.

- [ ] **Step 6: Smoke manually**

```powershell
$env:AWARE_HOME = "C:\Users\bimst\source\repos\aware\target\debug-fixtures"
# (skip if no fixtures populated; manual sanity instead:)
cargo run -- agent list
```

Confirm the output shape looks right.

- [ ] **Step 7: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/commands/agent.rs cli/tests/common cli/tests/agent_list.rs
git commit -m "feat(cli): implement aware agent list"
```

---

## Task 10: Implement `aware agent describe <agent>`

Prints the agent's manifest summary + command list + skill index.

**Files:**
- Create: `cli/tests/agent_describe.rs`
- Modify: `cli/src/commands/agent.rs`

- [ ] **Step 1: Write the failing test**

`cli/tests/agent_describe.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn describes_tekla_agent() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("agent:"))
        .stdout(predicate::str::contains("tekla"))
        .stdout(predicate::str::contains("2025.0.1"))
        .stdout(predicate::str::contains("stateful:"))
        .stdout(predicate::str::contains("watch"))
        .stdout(predicate::str::contains("insert"))
        .stdout(predicate::str::contains("save-attributes"))
        .stdout(predicate::str::contains("drawing-identity.md"));
}

#[test]
fn missing_agent_returns_not_found_exit_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "does-not-exist"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn json_describe_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "describe", "tekla"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["agent"], "tekla");
    assert_eq!(v["data"]["skill_count"], 31);
    assert_eq!(v["data"]["command_count"], 3);
}
```

- [ ] **Step 2: Run — expect failure**

```powershell
cargo test --test agent_describe
```

- [ ] **Step 3: Implement `describe`**

In `cli/src/commands/agent.rs`, add to the dispatch match:

```rust
AgentCommand::Describe { agent } => describe(ctx, &agent),
```

And add the function:

```rust
fn describe(ctx: &Context, agent_id: &str) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_agents(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| AwareError::NotFound(format!("agent: {agent_id}")))?;

    if ctx.json {
        #[derive(Serialize)]
        struct CommandRow {
            name: String,
            lifecycle: String,
            description: String,
        }
        #[derive(Serialize)]
        struct DescribeData<'a> {
            agent: &'a str,
            version: &'a str,
            display_name: Option<&'a str>,
            description: &'a str,
            stateful: bool,
            license: &'a str,
            vendor: Option<&'a str>,
            commands: Vec<CommandRow>,
            skills: &'a [String],
            skill_count: usize,
            command_count: usize,
        }

        let cmds: Vec<CommandRow> = d.manifest.commands.iter().map(|(n, c)| CommandRow {
            name: n.clone(),
            lifecycle: format!("{:?}", c.lifecycle).to_lowercase(),
            description: c.description.clone(),
        }).collect();

        let data = DescribeData {
            agent: &d.manifest.agent,
            version: &d.manifest.version,
            display_name: d.manifest.display_name.as_deref(),
            description: &d.manifest.description,
            stateful: d.manifest.stateful,
            license: &d.manifest.license,
            vendor: d.manifest.vendor.as_deref(),
            command_count: d.manifest.command_count(),
            skill_count: d.manifest.skill_count(),
            commands: cmds,
            skills: &d.manifest.skills,
        };
        envelope::print_ok("agent describe", data, started).ok();
        return Ok(());
    }

    let m = &d.manifest;
    println!("agent:        {}", m.agent);
    println!("version:      {}", m.version);
    if let Some(dn) = &m.display_name {
        println!("display-name: {dn}");
    }
    println!("description:  {}", m.description.lines().next().unwrap_or("").trim());
    println!("stateful:     {}", m.stateful);
    if let Some(v) = &m.vendor { println!("vendor:       {v}"); }
    println!("license:      {}", m.license);
    if let Some(t) = &m.transport.cli {
        println!("transport:    cli ({})", t.binary);
    }
    println!();
    println!("commands:");
    for (name, c) in &m.commands {
        let lc = format!("{:?}", c.lifecycle).to_lowercase();
        let desc = c.description.lines().next().unwrap_or("").trim();
        println!("  {name:<18} {lc:<8} {desc}");
    }
    println!();
    println!("skills ({}):", m.skill_count());
    for s in &m.skills {
        println!("  - {s}");
    }
    Ok(())
}
```

- [ ] **Step 4: Run — expect pass**

```powershell
cargo test --test agent_describe
```

- [ ] **Step 5: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/commands/agent.rs cli/tests/agent_describe.rs
git commit -m "feat(cli): implement aware agent describe"
```

---

## Task 11: Implement `aware agent skill <agent> <skill-name>`

Prints the raw markdown of a specific skill. Accepts either the filename (`drawing-identity.md`) or the stem (`drawing-identity`).

**Files:**
- Create: `cli/tests/agent_skill.rs`
- Modify: `cli/src/commands/agent.rs`

- [ ] **Step 1: Write the failing test**

`cli/tests/agent_skill.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_drawing_identity_skill() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "tekla", "drawing-identity"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Mark"));
}

#[test]
fn accepts_filename_with_extension() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "tekla", "drawing-identity.md"])
        .assert()
        .success();
}

#[test]
fn missing_agent_exits_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "missing-agent", "anything"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn missing_skill_exits_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "tekla", "nope-not-real"])
        .assert()
        .failure()
        .code(7);
}
```

- [ ] **Step 2: Run — expect failure**

```powershell
cargo test --test agent_skill
```

- [ ] **Step 3: Implement `skill`**

Add to dispatch:

```rust
AgentCommand::Skill { agent, skill } => skill_cmd(ctx, &agent, &skill),
```

And the function:

```rust
fn skill_cmd(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let discovered = discover_agents(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| AwareError::NotFound(format!("agent: {agent_id}")))?;

    let filename = if skill_name.ends_with(".md") {
        skill_name.to_string()
    } else {
        format!("{skill_name}.md")
    };
    let path = d.root.join("skills").join(&filename);
    if !path.is_file() {
        return Err(AwareError::NotFound(format!("skill: {agent_id}/{filename}")));
    }
    let body = std::fs::read_to_string(&path)?;
    // Raw print — markdown is human-readable and AI-readable as-is.
    print!("{body}");
    Ok(())
}
```

- [ ] **Step 4: Run — expect pass**

```powershell
cargo test --test agent_skill
```

- [ ] **Step 5: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/commands/agent.rs cli/tests/agent_skill.rs
git commit -m "feat(cli): implement aware agent skill"
```

---

## Task 12: Implement `aware app list`

Table with id / version / nodes / connections / layout.

**Files:**
- Create: `cli/tests/app_list.rs`
- Modify: `cli/src/commands/app.rs`

- [ ] **Step 1: Write the failing test**

`cli/tests/app_list.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn lists_two_fixture_apps() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"))
        .stdout(predicate::str::contains("VERSION"))
        .stdout(predicate::str::contains("NODES"))
        .stdout(predicate::str::contains("CONNS"))
        .stdout(predicate::str::contains("LAYOUT"))
        .stdout(predicate::str::contains("welded-to-tc"))
        .stdout(predicate::str::contains("qa-drawings-to-tekla"))
        .stdout(predicate::str::contains("linear"))
        .stdout(predicate::str::contains("dag"));
}

#[test]
fn json_output_has_two_apps() {
    let home = common::aware_home();
    let out = Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "app", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["apps"].as_array().unwrap().len(), 2);
}
```

- [ ] **Step 2: Run — expect failure**

```powershell
cargo test --test app_list
```

- [ ] **Step 3: Implement `app list`**

In `cli/src/commands/app.rs`:

```rust
use std::time::Instant;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::manifest::loader::discover_apps;
use crate::render::table::Table;

// ... existing AppCommand enum unchanged ...

pub fn dispatch(cmd: AppCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AppCommand::List => list(ctx),
        AppCommand::Show { .. } => Err(AwareError::NotYetImplemented("app show")),
        AppCommand::Install { .. } => Err(AwareError::NotYetImplemented("app install")),
        AppCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("app uninstall")),
        AppCommand::Validate { .. } => Err(AwareError::NotYetImplemented("app validate")),
        AppCommand::Export { .. } => Err(AwareError::NotYetImplemented("app export")),
        AppCommand::Run { .. } => Err(AwareError::NotYetImplemented("app run")),
        AppCommand::Stop { .. } => Err(AwareError::NotYetImplemented("app stop")),
        AppCommand::Logs { .. } => Err(AwareError::NotYetImplemented("app logs")),
    }
}

#[derive(Serialize)]
struct AppListRow {
    id: String,
    version: String,
    nodes: usize,
    connections: usize,
    layout: String,
}

#[derive(Serialize)]
struct AppListData {
    apps: Vec<AppListRow>,
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_apps(&ctx.paths)?;

    if ctx.json {
        let data = AppListData {
            apps: discovered.iter().map(|d| AppListRow {
                id: d.manifest.app.clone(),
                version: d.manifest.version.clone(),
                nodes: d.manifest.node_count(),
                connections: d.manifest.connection_count(),
                layout: format!("{:?}", d.manifest.layout).to_lowercase(),
            }).collect(),
        };
        envelope::print_ok("app list", data, started).ok();
        return Ok(());
    }

    let mut t = Table::new(["ID", "VERSION", "NODES", "CONNS", "LAYOUT"]);
    for d in &discovered {
        t.row([
            d.manifest.app.clone(),
            d.manifest.version.clone(),
            d.manifest.node_count().to_string(),
            d.manifest.connection_count().to_string(),
            format!("{:?}", d.manifest.layout).to_lowercase(),
        ]);
    }
    print!("{}", t.render());
    Ok(())
}
```

- [ ] **Step 4: Run — expect pass**

```powershell
cargo test --test app_list
```

- [ ] **Step 5: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/commands/app.rs cli/tests/app_list.rs
git commit -m "feat(cli): implement aware app list"
```

---

## Task 13: Implement `aware app show <app>`

Renders the ASCII topology + provenance fields.

**Files:**
- Create: `cli/tests/app_show.rs`
- Modify: `cli/src/commands/app.rs`

- [ ] **Step 1: Write the failing test**

`cli/tests/app_show.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn shows_welded_to_tc_linear_topology() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "show", "welded-to-tc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app:"))
        .stdout(predicate::str::contains("welded-to-tc"))
        .stdout(predicate::str::contains("0.3.1"))
        .stdout(predicate::str::contains("Topology"))
        .stdout(predicate::str::contains("linear"))
        .stdout(predicate::str::contains("tekla-watch"))
        .stdout(predicate::str::contains("filter-welded"))
        .stdout(predicate::str::contains("tc-upload"))
        .stdout(predicate::str::contains("Requires:"));
}

#[test]
fn shows_qa_drawings_dag_topology() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "show", "qa-drawings-to-tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dag"))
        .stdout(predicate::str::contains("match-build"))
        .stdout(predicate::str::contains("pdf-extract → match-build"));
}

#[test]
fn missing_app_exits_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "show", "nope"])
        .assert()
        .failure()
        .code(7);
}
```

- [ ] **Step 2: Run — expect failure**

```powershell
cargo test --test app_show
```

- [ ] **Step 3: Implement `app show`**

Add to dispatch:

```rust
AppCommand::Show { app } => show(ctx, &app),
```

Implement:

```rust
fn show(ctx: &Context, app_id: &str) -> Result<(), AwareError> {
    use crate::render::topology::format_topology;

    let discovered = discover_apps(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.app == app_id)
        .ok_or_else(|| AwareError::NotFound(format!("app: {app_id}")))?;

    let m = &d.manifest;
    println!("app:           {}", m.app);
    println!("version:       {}", m.version);
    if let Some(dn) = &m.display_name {
        println!("display-name:  {dn}");
    }
    println!("description:   {}", m.description.lines().next().unwrap_or("").trim());
    println!("exposes-as-agent: {}", m.exposes_as_agent);
    println!("layout:        {}", format!("{:?}", m.layout).to_lowercase());
    println!();

    println!("Requires:");
    for r in &m.requires {
        println!("  - {r}");
    }
    println!();

    print!("{}", format_topology(m));
    Ok(())
}
```

- [ ] **Step 4: Run — expect pass**

```powershell
cargo test --test app_show
```

- [ ] **Step 5: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/commands/app.rs cli/tests/app_show.rs
git commit -m "feat(cli): implement aware app show"
```

---

## Task 14: Implement `aware doctor` (limited)

Per v0.1 roadmap: config + filesystem only. No credentials, no plugins, no registry.

**Files:**
- Create: `cli/tests/doctor.rs`
- Modify: `cli/src/commands/doctor.rs`

- [ ] **Step 1: Write the failing test**

`cli/tests/doctor.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_reports_seven_agents_and_two_apps() {
    let home = common::aware_home();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", home)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI:"))
        .stdout(predicate::str::contains("aware v0.1"))
        .stdout(predicate::str::contains("Filesystem:"))
        .stdout(predicate::str::contains("7 installed"))
        .stdout(predicate::str::contains("2 installed"));
}

#[test]
fn doctor_empty_home_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("0 installed"));
}
```

- [ ] **Step 2: Run — expect failure**

```powershell
cargo test --test doctor
```

- [ ] **Step 3: Implement**

`cli/src/commands/doctor.rs`:

```rust
//! `aware doctor` — health check.
//!
//! Phase v0.1 (limited): config + filesystem only.

use crate::context::Context;
use crate::error::AwareError;
use crate::manifest::loader::{discover_agents, discover_apps};

pub fn run(ctx: &Context) -> Result<(), AwareError> {
    let aware_home = &ctx.paths.aware_home;
    let agents_dir = ctx.paths.agents_dir();
    let apps_dir = ctx.paths.apps_dir();
    let config_path = ctx.paths.config_path();

    println!("CLI:");
    println!("  ✓ aware v{} (build profile)", env!("CARGO_PKG_VERSION"));
    if config_path.is_file() {
        println!("  ✓ Config at {}", config_path.display());
    } else {
        println!("  · No config.yaml yet (will be created on first write)");
    }
    println!();

    println!("Filesystem:");
    if aware_home.is_dir() {
        println!("  ✓ {} exists", aware_home.display());
    } else {
        println!("  · {} does not exist yet", aware_home.display());
    }
    println!();

    let agents = discover_agents(&ctx.paths).unwrap_or_default();
    let apps = discover_apps(&ctx.paths).unwrap_or_default();

    println!("Agents:");
    println!("  ✓ {} installed (under {})", agents.len(), agents_dir.display());
    for a in &agents {
        println!("    - {}@{}", a.manifest.agent, a.manifest.version);
    }
    println!();

    println!("Apps:");
    println!("  ✓ {} installed (under {})", apps.len(), apps_dir.display());
    for a in &apps {
        println!("    - {}@{}", a.manifest.app, a.manifest.version);
    }
    Ok(())
}
```

- [ ] **Step 4: Run — expect pass**

```powershell
cargo test --test doctor
```

- [ ] **Step 5: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/src/commands/doctor.rs cli/tests/doctor.rs
git commit -m "feat(cli): implement aware doctor (v0.1 limited)"
```

---

## Task 15: Update `tests/basic.rs` and README quickstart

`agent list` now exits 0 instead of returning `NotYetImplemented`, so the smoke test must target a still-stubbed command. Also bring the README quickstart up to date.

**Files:**
- Modify: `cli/tests/basic.rs`, `cli/README.md`

- [ ] **Step 1: Update `basic.rs`**

In `cli/tests/basic.rs`, change the unimplemented test to target a still-stubbed command. Replace the `unimplemented_subcommand_exits_nonzero_with_message` test body:

```rust
#[test]
fn unimplemented_subcommand_exits_nonzero_with_message() {
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "install", "tekla"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"))
        .stderr(predicate::str::contains("agent install"));
}
```

- [ ] **Step 2: Update README quickstart**

In `cli/README.md`, replace the "Status" and "Run" sections to reflect v0.1:

```markdown
## Status

**v0.1 — read-only foundation.** Eight surfaces shipped:

- `aware --version` / `aware --help`
- `aware doctor`
- `aware agent list` / `agent describe <id>` / `agent skill <id> <skill>`
- `aware app list` / `app show <id>`

Everything else (`install`, `validate`, `run`, `connect`, `build`, `skill ...`)
remains `NotYetImplemented` until its phase per [cli-roadmap.md](../10-core/cli-roadmap.md).

## Run

```bash
# Default reads ~/.aware/. Override via AWARE_HOME for testing:
export AWARE_HOME=$(pwd)/test-fixtures/aware

aware doctor                    # filesystem health check
aware agent list                # table of installed agents
aware agent describe tekla      # manifest summary + commands + skills
aware agent skill tekla drawing-identity   # raw skill .md content
aware app list                  # installed apps
aware app show welded-to-tc     # ASCII topology + provenance
aware --json agent list         # JSON envelope for piping
```

The `AWARE_HOME` env var overrides the default `~/.aware/` location for the
session — useful for testing against repo fixtures without polluting your
home directory.
```

- [ ] **Step 3: Run all tests**

```powershell
cargo test
```

Expected: every test passes (basic.rs + all 6 new integration test files + all unit tests).

- [ ] **Step 4: Commit**

```powershell
cargo fmt --all
cargo clippy --all-targets -- -D warnings
git add cli/tests/basic.rs cli/README.md
git commit -m "docs(cli): update tests/basic.rs and README for v0.1"
```

---

## Task 16: Final verification

The phased-delivery contract from `cli-roadmap.md` v0.1 definition-of-done.

**Files:** none (verification only).

- [ ] **Step 1: Full test suite**

```powershell
cd cli
cargo test --all-targets
```

Expected: every test passes — unit tests under `cli/src/**`, plus integration tests under `cli/tests/*.rs`.

- [ ] **Step 2: Format check**

```powershell
cargo fmt --all -- --check
```

Expected: zero diff.

- [ ] **Step 3: Clippy as errors**

```powershell
cargo clippy --all-targets -- -D warnings
```

Expected: zero warnings, zero errors.

- [ ] **Step 4: Release build**

```powershell
cargo build --release
```

Expected: `cli/target/release/aware.exe` produced.

- [ ] **Step 5: Manual end-to-end smoke**

Populate a fixture aware-home to exercise the binary end-to-end against real data:

```powershell
$tmp = "C:\Users\bimst\source\repos\aware\target\fixture-home"
Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path "$tmp\agents" | Out-Null
New-Item -ItemType Directory -Path "$tmp\apps"   | Out-Null

# Copy each agent from 20-agents/**/<id>/ → $tmp/agents/<id>/
Get-ChildItem -Recurse -Filter manifest.yaml "C:\Users\bimst\source\repos\aware\20-agents" |
  ForEach-Object {
    $src = $_.Directory.FullName
    $dst = Join-Path "$tmp\agents" $_.Directory.Name
    Copy-Item -Recurse -Force $src $dst
  }

# Copy each app .flo to $tmp/apps/<stem>/<stem>.flo
Get-ChildItem -Filter *.flo "C:\Users\bimst\source\repos\aware\30-apps\_examples" |
  ForEach-Object {
    $stem = $_.BaseName
    $dst = Join-Path "$tmp\apps" $stem
    New-Item -ItemType Directory -Path $dst -Force | Out-Null
    Copy-Item $_.FullName (Join-Path $dst $_.Name)
  }

$env:AWARE_HOME = $tmp

.\target\release\aware.exe --version
.\target\release\aware.exe doctor
.\target\release\aware.exe agent list
.\target\release\aware.exe agent describe tekla
.\target\release\aware.exe agent skill tekla drawing-identity
.\target\release\aware.exe app list
.\target\release\aware.exe app show welded-to-tc
.\target\release\aware.exe --json agent list
```

Read each output by eye against the expected shape in `10-core/cli-spec.md`.

- [ ] **Step 6: Verify exit codes match the spec**

```powershell
.\target\release\aware.exe agent describe does-not-exist
echo $LASTEXITCODE   # expect: 7

.\target\release\aware.exe agent install tekla
echo $LASTEXITCODE   # expect: 1 (NotYetImplemented)

.\target\release\aware.exe
echo $LASTEXITCODE   # expect: 2 (missing required arg per clap)
```

If anything diverges from the spec, **stop and fix**.

No commit unless something was fixed in this task. The task is a gate, not a deliverable.

---

## Self-review (run after writing this plan, before execution starts)

**Spec coverage** — every v0.1 surface from `cli-roadmap.md` has a task:

| Surface | Task |
|---|---|
| `aware --version` | Task 0 (already works) |
| `aware --help` | Task 0 (already works) |
| `aware doctor` (limited) | Task 14 |
| `aware agent list` | Task 9 |
| `aware agent describe` | Task 10 |
| `aware agent skill` | Task 11 |
| `aware app list` | Task 12 |
| `aware app show` | Task 13 |
| Internal: `cli::paths` | Task 1 |
| Internal: `cli::context` | Task 2 |
| Internal: `cli::manifest` (agent + app + loader) | Tasks 3-5 |
| Internal: `cli::envelope` | Task 6 |
| Internal: `cli::render` (table + topology) | Tasks 7-8 |
| Internal: `cli::config` | **DEFERRED to v0.2** — no v0.1 command parses config beyond a presence check, which doctor handles inline. |

**v0.1 def-of-done** (from roadmap):

- `cargo build --release` produces `aware` — Task 16 step 4
- All commands exit 0 on repo fixtures — Task 16 step 5
- Integration tests cover every command surface — Tasks 9-14 each add one
- `aware doctor` reports the 7 agents + 2 apps — Task 14 test
- README quickstart shows install + first-run flow — Task 15 step 2

**Type-consistency spot check:**
- `Paths::aware_home` (PathBuf) — used in `context.rs`, `loader.rs`, `doctor.rs`. ✓
- `Agent::skill_count()` / `command_count()` — defined Task 3, used Tasks 9, 10, 14. ✓
- `App::node_count()` / `connection_count()` / `is_dag()` — defined Task 4, used Tasks 12, 13. ✓
- `Lifecycle` enum — defined Task 3, formatted via `format!("{:?}", ...).to_lowercase()` in Task 10. ✓
- `Context` signature stable across all dispatchers — Task 2 establishes; Tasks 9-14 use. ✓
- `envelope::print_ok(command_name, data, started)` — same triplet everywhere. ✓

**Placeholder scan:** No "TBD" / "TODO" / "fill in later" / "similar to Task N". Each step ships full code.

**No scope creep:** plan defines `NotYetImplemented` is preserved for every non-v0.1 surface. Tasks 9, 12, 14 each explicitly preserve `NotYetImplemented` for sibling subcommands.

---

## Execution handoff

Plan saved to `docs/superpowers/plans/2026-05-15-aware-cli-v01-readonly.md`. Two execution options:

**1. Subagent-Driven (recommended)** — fresh subagent per task, review between tasks, fast iteration. Uses `superpowers:subagent-driven-development`.

**2. Inline Execution** — execute tasks in this session, batch with checkpoints. Uses `superpowers:executing-plans`.

Which approach?
