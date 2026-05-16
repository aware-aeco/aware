# AWARE CLI v0.2 (Install + Validate) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Users can install AWARE agents and apps from disk OR from the AWARE registry (a GitHub-hosted JSON index). Validators reject malformed manifests *before* anything lands under `~/.aware/`. `aware doctor` reports installed-agent integrity.

**Architecture:** Add four internal modules on top of the v0.1 foundation: `cli::validate` (schema + cycle + capability checks), `cli::registry` (fetch + cache the index from a configurable source), `cli::install` (copy / extract into `~/.aware/`, with backout on partial failure), `cli::lockfile` (write `lockfile.yaml` for installed apps, pinning resolved agent versions). All new commands flow through these.

**Tech Stack additions:**
- `ureq` 2.x — minimal blocking HTTP client (no tokio dep; `tokio` stays deferred until v0.3 runtime).
- `flate2` + `tar` — extract `.tar.gz` archives fetched from the registry.
- Already-present: `serde`, `serde_yaml`, `serde_json`, `thiserror`, `tempfile`, `assert_cmd`, `predicates`.

---

## Open design decisions (resolved here — flag if you want changes)

These were judgment calls during planning. Each has a defensible alternative; if any feel wrong, override before execution.

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| HTTP client | `ureq` 2.x (blocking, pure-rust TLS via rustls) | No tokio dep yet; v0.2 only needs sync fetches | `reqwest::blocking` is heavier; defer until v0.3 needs async anyway |
| Agent transport | `.tar.gz` archive over HTTPS | Pure-rust extract via `tar` + `flate2`; one HTTP call per agent; no `git` CLI dep | Per-file GitHub Contents API (slow, rate-limited); `git2` libgit2 (binary-size cost) |
| Registry location | Hardcoded `https://raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json`, override via `AWARE_REGISTRY` env var (HTTPS URL OR `file://` path) | Decalog #4 — no vendor lock; raw.githubusercontent.com is free, public, cache-friendly; `file://` lets tests run offline | A hosted registry service (later, not v0.2) |
| Cache TTL | 1 hour for the index; per-agent tarballs cached forever by version (semver-pinned never changes) | Index changes when agents publish; tarballs are immutable per version | Always re-fetch (wastes bandwidth); cache-forever (stale agent lists) |
| Validation severity | Hard error (exit 3) on: missing required fields, cycles in app connections, manifest-declared skills missing from `skills/`, app `requires` not satisfiable. Warning on: extra unknown fields, missing optional metadata | Validators are the safety net before disk write — better to over-reject than to land broken artifacts | Warn-only mode (you opt-in to strict) |
| Bundle install | Sequential install of bundle members; on per-agent failure, mark that agent as failed but continue with the rest; report final status (e.g. "5 installed, 2 failed") | Bundles are best-effort batch operations; partial success is useful | Atomic all-or-nothing (rollback on any failure) — heavier, complicates filesystem |
| `agent update` semantics | Re-pull from the registry, replacing the agent dir entirely. No user-modified-file detection in v0.2 (agents aren't user-editable in the v0 model) | Matches "agents are immutable installable units" | Conflict-detection / merge — premature for v0 |
| `aware-aeco` bundle definition | Defined in `registry-index.json` under `bundles.aware-aeco.agents = [tekla, trimble-connect, microsoft-365, google-workspace, html-report, aware-agent-builder, aware-skill-builder]` | Matches the 7 reference agents already in this repo | Hardcode in CLI source (rigid); separate `bundles.yaml` (extra fetch) |
| Lockfile shape | YAML at `~/.aware/apps/<app>/lockfile.yaml`, fields: `app`, `version`, `resolved-at`, `resolved-agents: { <id>: <exact-version> }` | Matches the spec mention in cli-spec.md and is human-readable | JSON or TOML — equally fine, YAML matches surrounding manifests |
| Registry-index.json **maintenance** | Hand-edit during v0.2 dev (one entry per reference agent); future automation (v0.5 + CI) re-generates from `20-agents/` on each merge | The repo has 7 known agents; manual is fine for now | Build script that generates the index — overkill for v0.2 |

---

## Scope discipline

**v0.2 surfaces** (8 commands; the `agent install <bundle>` variant uses the same `install` machinery as `agent install <id>`):

- `aware agent install <id>[@version]` — from registry
- `aware agent install <path>` — from local folder (auto-detected by `path.is_dir()`)
- `aware agent install <bundle>` — special-cased `aware-aeco` and other registry-listed bundles
- `aware agent uninstall <id>`
- `aware agent update <id>` — re-pull latest matching version
- `aware agent validate <path>`
- `aware app install <path-or-name>` — local path; registry-hosted apps are post-v0.2
- `aware app uninstall <id>`
- `aware app validate <path>`
- `aware doctor` — extended to flag agent-integrity issues (manifest-listed skills missing from disk; orphan files; etc.)

**Out of scope for v0.2** (every other surface stays `NotYetImplemented`):
- `aware agent publish` — needs registry write access; v0.2+
- `aware app export` — trivial post-install; v0.2+
- `aware app run/stop/logs` — that's v0.3
- `aware connect / disconnect` — v0.4
- `aware build agent` / `aware skill ...` — v0.5

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli/src/validate.rs` | `validate_agent(&Agent, &Path) -> Vec<ValidationIssue>`, `validate_app(&App) -> Vec<ValidationIssue>`. Pure functions; no I/O beyond reading the manifest's `skills/` dir on demand. |
| `cli/src/registry/mod.rs` | Re-exports. |
| `cli/src/registry/index.rs` | `Index` struct + `IndexEntry`, `BundleEntry`; deserialize JSON index from a `Reader`. |
| `cli/src/registry/fetch.rs` | `fetch_index(source: &str, cache_dir: &Path) -> Index` — supports HTTPS + `file://` URLs; honors TTL; falls back to stale cache on network error. |
| `cli/src/install/mod.rs` | Re-exports. |
| `cli/src/install/local.rs` | `install_agent_from_path(src: &Path, paths: &Paths)`; `install_app_from_path(src: &Path, paths: &Paths)`. Validates first, then copies. |
| `cli/src/install/registry.rs` | `install_agent_from_registry(id: &str, version_pin: Option<&str>, paths: &Paths, index: &Index)`. Resolves → fetches tarball → extracts → validates → moves into place. |
| `cli/src/install/bundle.rs` | `install_bundle(name: &str, paths: &Paths, index: &Index) -> BundleReport`. Calls `install_agent_from_registry` per member; aggregates results. |
| `cli/src/install/uninstall.rs` | `uninstall_agent(id: &str, paths: &Paths)`; `uninstall_app(id: &str, paths: &Paths)`. Safe `remove_dir_all` with confirmation. |
| `cli/src/lockfile.rs` | `Lockfile` struct + `write_for_app(app: &App, resolved_agents: &BTreeMap<String, String>, target: &Path)`. |
| `cli/tests/agent_install.rs` | Integration tests for local-path + registry + bundle installs. |
| `cli/tests/agent_uninstall.rs` | |
| `cli/tests/agent_update.rs` | |
| `cli/tests/agent_validate.rs` | |
| `cli/tests/app_install.rs` | |
| `cli/tests/app_uninstall.rs` | |
| `cli/tests/app_validate.rs` | |
| `cli/tests/fixtures/registry-index.json` | Test fixture mimicking the real registry shape; tests set `AWARE_REGISTRY=file://<path>`. |
| `cli/tests/fixtures/tekla-bad.tar.gz` | Optional — a deliberately malformed agent archive for the "rejects bad install" test. |
| `registry-index.json` (repo root) | The actual production registry index. Hand-written; lists all 7 reference agents + the `aware-aeco` bundle. |

### Modified files

| Path | Change |
|---|---|
| `cli/Cargo.toml` | Add `ureq = "2"`, `flate2 = "1"`, `tar = "0.4"`. |
| `cli/src/main.rs` | Add `mod validate; mod registry; mod install; mod lockfile;`. |
| `cli/src/commands/agent.rs` | Implement `Install`, `Uninstall`, `Update`, `Validate` dispatch arms. Keep `Publish` as `NotYetImplemented`. |
| `cli/src/commands/app.rs` | Implement `Install`, `Uninstall`, `Validate`, `Export` (the last is trivial — see Task 17). |
| `cli/src/commands/doctor.rs` | Extend with agent-integrity checks (calls `validate::validate_agent` per installed agent). |
| `cli/src/manifest/loader.rs` | Add `load_lockfile(path)` (read sibling of `load_app`). |
| `cli/src/paths.rs` | Drop the `#[allow(dead_code)]` on `credentials_dir()` only if v0.4 has landed — likely keep it for v0.2. Add `cache_dir()` method. |
| `cli/README.md` | Bump quickstart with `agent install` / `app install` examples. |

### Key data shapes (this plan defines them)

**`registry-index.json`** (production lives at repo root, served via raw.githubusercontent.com):

```json
{
  "version": "1.0",
  "updated-at": "2026-05-16T00:00:00Z",
  "agents": {
    "tekla": {
      "versions": {
        "2025.0.1": {
          "tarball": "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz",
          "subdir": "aware-main/20-agents/aeco/engineering/tekla"
        }
      }
    },
    "trimble-connect": {
      "versions": {
        "2.4.0": {
          "tarball": "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz",
          "subdir": "aware-main/20-agents/aeco/construction/trimble-connect"
        }
      }
    },
    "microsoft-365": { "versions": { "1.0.0": { "tarball": "...", "subdir": "aware-main/20-agents/aeco/cross-cutting/microsoft-365" } } },
    "google-workspace": { "versions": { "1.0.0": { "tarball": "...", "subdir": "..." } } },
    "html-report":      { "versions": { "1.0.0": { "tarball": "...", "subdir": "aware-main/20-agents/_core/html-report" } } },
    "aware-agent-builder": { "versions": { "0.1.0": { "tarball": "...", "subdir": "aware-main/20-agents/_core/aware-agent-builder" } } },
    "aware-skill-builder": { "versions": { "0.1.0": { "tarball": "...", "subdir": "aware-main/20-agents/_core/aware-skill-builder" } } }
  },
  "bundles": {
    "aware-aeco": {
      "description": "All first-party AECO agents (engineering, construction, cross-cutting, utilities, meta).",
      "agents": [
        "tekla@2025.0.1",
        "trimble-connect@2.4.0",
        "microsoft-365@1.0.0",
        "google-workspace@1.0.0",
        "html-report@1.0.0",
        "aware-agent-builder@0.1.0",
        "aware-skill-builder@0.1.0"
      ]
    }
  }
}
```

**`lockfile.yaml`** (written under `~/.aware/apps/<app>/lockfile.yaml` on `aware app install`):

```yaml
app: welded-to-tc
version: 0.3.1
resolved-at: 2026-05-16T14:23:00Z
resolved-agents:
  tekla: 2025.0.1
  trimble-connect: 2.4.0
```

---

## Task 0: Prerequisites

No code changes. Verify the v0.1 branch is the working baseline (the `feat/cli-v01-readonly` branch is merged or about to be).

- [ ] **Step 1: Branch off main (or the target branch)**

```powershell
git checkout main
git pull --ff-only
git checkout -b feat/cli-v02-install-validate
```

Use the merged-to-main commit as the base. If v0.1 hasn't merged yet, branch off `feat/cli-v01-readonly` and rebase later.

- [ ] **Step 2: Confirm v0.1 baseline still passes**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo build; cargo test
```

Expected: 44/44 tests pass (the v0.1 surface, unchanged).

No commit.

---

## Task 1: Add HTTP + archive dependencies

**Files:**
- Modify: `cli/Cargo.toml`

- [ ] **Step 1: Edit `cli/Cargo.toml` dependencies**

Add to the `[dependencies]` section:

```toml
ureq      = { version = "2", default-features = false, features = ["tls"] }
flate2    = "1"
tar       = "0.4"
chrono    = { version = "0.4", default-features = false, features = ["clock", "serde"] }
```

(`chrono` is for the lockfile `resolved-at` ISO-8601 timestamp. Pulled in lean — no `serde_derive` features beyond what we need.)

- [ ] **Step 2: Build to fetch + verify**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo build
```

Expected: clean build, ~15-20s for fetching.

- [ ] **Step 3: Commit**

```powershell
git add cli/Cargo.toml cli/Cargo.lock
git commit -m "chore(cli): add ureq + flate2 + tar + chrono for v0.2"
```

---

## Task 2: `cli::validate` — schema + cycle + capability checks

**Files:**
- Create: `cli/src/validate.rs`
- Modify: `cli/src/main.rs` (add `mod validate;`)

- [ ] **Step 1: Write tests first**

`cli/src/validate.rs`:

```rust
//! Schema + cycle + capability validation for agents and apps.
//!
//! Pure functions — no I/O except optional `skills/` directory probe when
//! the caller passes a path (used by `validate_agent_on_disk`).

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::manifest::{Agent, App};

#[derive(Debug, PartialEq, Eq)]
pub enum Severity { Error, Warning }

#[derive(Debug)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
}

impl ValidationIssue {
    fn error(code: &'static str, msg: impl Into<String>) -> Self {
        Self { severity: Severity::Error, code, message: msg.into() }
    }
    fn warning(code: &'static str, msg: impl Into<String>) -> Self {
        Self { severity: Severity::Warning, code, message: msg.into() }
    }
}

pub fn has_errors(issues: &[ValidationIssue]) -> bool {
    issues.iter().any(|i| i.severity == Severity::Error)
}

/// Validate an `Agent` manifest without touching disk. Catches the issues
/// `serde` can't (lifecycle-on-stateless mismatches, empty command map, etc.).
pub fn validate_agent(agent: &Agent) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    if agent.agent.trim().is_empty() {
        out.push(ValidationIssue::error("E_AGENT_EMPTY_ID", "agent id is empty"));
    }
    if agent.commands.is_empty() {
        out.push(ValidationIssue::error("E_AGENT_NO_COMMANDS", "agent declares zero commands"));
    }
    if agent.transport.cli.is_none() && agent.transport.mcp.is_none() && agent.transport.rest.is_none() {
        out.push(ValidationIssue::error("E_AGENT_NO_TRANSPORT", "agent must declare at least one transport (cli / mcp / rest)"));
    }
    // Stateful agents must have at least one `start` lifecycle command.
    if agent.stateful {
        let has_start = agent.commands.values().any(|c| matches!(c.lifecycle, crate::manifest::agent::Lifecycle::Start));
        if !has_start {
            out.push(ValidationIssue::error("E_STATEFUL_NO_START", "stateful agent must have at least one command with lifecycle: start"));
        }
    }
    out
}

/// Disk-aware validation: confirms every skill listed in the manifest exists
/// under `<agent_root>/skills/`. Returns issues; caller decides exit.
pub fn validate_agent_on_disk(agent: &Agent, agent_root: &Path) -> Vec<ValidationIssue> {
    let mut out = validate_agent(agent);
    let skills_dir = agent_root.join("skills");
    for skill in &agent.skills {
        if !skills_dir.join(skill).is_file() {
            out.push(ValidationIssue::error(
                "E_SKILL_FILE_MISSING",
                format!("manifest lists skill {skill:?} but file is missing under skills/"),
            ));
        }
    }
    // Warning for orphan: skill files on disk not listed in manifest.
    if skills_dir.is_dir() {
        let listed: HashSet<_> = agent.skills.iter().cloned().collect();
        if let Ok(read) = std::fs::read_dir(&skills_dir) {
            for entry in read.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".md") && !listed.contains(&name) {
                    out.push(ValidationIssue::warning(
                        "W_SKILL_ORPHAN",
                        format!("skill file {name:?} on disk but not listed in manifest skills:"),
                    ));
                }
            }
        }
    }
    out
}

/// Validate an `App` manifest: cycle detection in `connections:`, dangling
/// node references, unique node ids, every inline glue has a description.
pub fn validate_app(app: &App) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    if app.app.trim().is_empty() {
        out.push(ValidationIssue::error("E_APP_EMPTY_ID", "app id is empty"));
    }
    if app.nodes.is_empty() {
        out.push(ValidationIssue::error("E_APP_NO_NODES", "app declares zero nodes"));
    }

    // Unique node ids.
    let mut seen = HashSet::new();
    for n in &app.nodes {
        if !seen.insert(n.id.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DUPLICATE_NODE_ID",
                format!("duplicate node id: {}", n.id),
            ));
        }
    }

    // Every connection refers to known nodes.
    let node_ids: HashSet<&str> = app.nodes.iter().map(|n| n.id.as_str()).collect();
    for c in &app.connections {
        if !node_ids.contains(c.from.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DANGLING_FROM",
                format!("connection from unknown node: {}", c.from),
            ));
        }
        if !node_ids.contains(c.to.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DANGLING_TO",
                format!("connection to unknown node: {}", c.to),
            ));
        }
    }

    // Cycle detection (DFS).
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for c in &app.connections {
        graph.entry(c.from.as_str()).or_default().push(c.to.as_str());
    }
    let mut visited: HashSet<&str> = HashSet::new();
    let mut on_stack: HashSet<&str> = HashSet::new();
    for n in &app.nodes {
        if !visited.contains(n.id.as_str()) && has_cycle(n.id.as_str(), &graph, &mut visited, &mut on_stack) {
            out.push(ValidationIssue::error(
                "E_APP_CYCLE",
                format!("cycle detected in connections starting from node: {}", n.id),
            ));
            break;
        }
    }

    // Inline glue must have a non-empty description.
    for n in &app.nodes {
        if let Some(inline) = &n.inline {
            if inline.description.trim().is_empty() {
                out.push(ValidationIssue::error(
                    "E_APP_INLINE_NO_DESC",
                    format!("inline node {:?} missing description", n.id),
                ));
            }
        }
    }
    out
}

fn has_cycle<'a>(
    node: &'a str,
    graph: &BTreeMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    on_stack: &mut HashSet<&'a str>,
) -> bool {
    visited.insert(node);
    on_stack.insert(node);
    if let Some(neighbors) = graph.get(node) {
        for &next in neighbors {
            if !visited.contains(next) {
                if has_cycle(next, graph, visited, on_stack) {
                    return true;
                }
            } else if on_stack.contains(next) {
                return true;
            }
        }
    }
    on_stack.remove(node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;

    fn load_agent(rel: &str) -> Agent {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(rel);
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
    fn load_app(rel: &str) -> App {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(rel);
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn real_tekla_passes_validation() {
        let a = load_agent("20-agents/aeco/engineering/tekla/manifest.yaml");
        let issues = validate_agent(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    #[test]
    fn real_welded_to_tc_passes_validation() {
        let a = load_app("30-apps/_examples/welded-to-tc.flo");
        let issues = validate_app(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    #[test]
    fn detects_cycle_in_connections() {
        let yaml = r#"
app: cyc
version: 0.0.1
description: |
  Cyclic app.
nodes:
  - id: a
  - id: b
  - id: c
connections:
  - { from: a, to: b }
  - { from: b, to: c }
  - { from: c, to: a }
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(has_errors(&issues));
        assert!(issues.iter().any(|i| i.code == "E_APP_CYCLE"));
    }

    #[test]
    fn detects_dangling_connection() {
        let yaml = r#"
app: dangle
version: 0.0.1
description: x
nodes:
  - id: a
connections:
  - { from: a, to: nope }
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(issues.iter().any(|i| i.code == "E_APP_DANGLING_TO"));
    }

    #[test]
    fn detects_duplicate_node_ids() {
        let yaml = r#"
app: dup
version: 0.0.1
description: x
nodes:
  - id: a
  - id: a
connections: []
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(issues.iter().any(|i| i.code == "E_APP_DUPLICATE_NODE_ID"));
    }

    #[test]
    fn stateful_without_start_is_error() {
        let yaml = r#"
agent: x
version: 1.0
description: y
stateful: true
license: MIT
transport: { cli: { binary: x } }
commands:
  do:
    lifecycle: single
    description: z
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_agent(&a);
        assert!(issues.iter().any(|i| i.code == "E_STATEFUL_NO_START"));
    }

    #[test]
    fn skill_file_missing_is_error() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("skills")).unwrap();
        let manifest = r#"
agent: x
version: 1.0
description: y
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: { do: { lifecycle: single, description: z } }
skills:
  - foo.md
"#;
        let a: Agent = serde_yaml::from_str(manifest).unwrap();
        let issues = validate_agent_on_disk(&a, tmp.path());
        assert!(issues.iter().any(|i| i.code == "E_SKILL_FILE_MISSING"));
    }
}
```

Add to `cli/src/main.rs`:

```rust
mod validate;
```

- [ ] **Step 2: Run tests**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test validate
```

Expected: 7 tests pass.

- [ ] **Step 3: Commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/validate.rs cli/src/main.rs
git commit -m "feat(cli): add validate module (schema + cycle + capability checks)"
```

---

## Task 3: `cli::lockfile` — pin resolved agent versions

**Files:**
- Create: `cli/src/lockfile.rs`
- Modify: `cli/src/main.rs` (add `mod lockfile;`)

- [ ] **Step 1: Implement**

`cli/src/lockfile.rs`:

```rust
//! `lockfile.yaml` written under `~/.aware/apps/<app>/`.
//! Pins exact agent versions resolved at install time.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub app: String,
    pub version: String,
    #[serde(rename = "resolved-at")]
    pub resolved_at: String,
    #[serde(rename = "resolved-agents")]
    pub resolved_agents: BTreeMap<String, String>,
}

pub fn write(app_id: &str, app_version: &str, resolved: BTreeMap<String, String>, target: &Path) -> Result<(), AwareError> {
    let lock = Lockfile {
        app: app_id.to_string(),
        version: app_version.to_string(),
        resolved_at: chrono::Utc::now().to_rfc3339(),
        resolved_agents: resolved,
    };
    let yaml = serde_yaml::to_string(&lock)
        .map_err(|e| AwareError::Internal(format!("serialize lockfile: {e}")))?;
    std::fs::write(target, yaml)?;
    Ok(())
}

pub fn read(path: &Path) -> Result<Lockfile, AwareError> {
    let text = std::fs::read_to_string(path)?;
    serde_yaml::from_str(&text).map_err(|e| AwareError::Validation(format!("{}: {e}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("lockfile.yaml");
        let mut resolved = BTreeMap::new();
        resolved.insert("tekla".into(), "2025.0.1".into());
        resolved.insert("trimble-connect".into(), "2.4.0".into());
        write("welded-to-tc", "0.3.1", resolved.clone(), &target).unwrap();

        let read_back = read(&target).unwrap();
        assert_eq!(read_back.app, "welded-to-tc");
        assert_eq!(read_back.version, "0.3.1");
        assert_eq!(read_back.resolved_agents, resolved);
        assert!(!read_back.resolved_at.is_empty());
    }
}
```

Add to `cli/src/main.rs`:

```rust
mod lockfile;
```

- [ ] **Step 2: Run tests**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test lockfile
```

- [ ] **Step 3: Commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/lockfile.rs cli/src/main.rs
git commit -m "feat(cli): add lockfile module"
```

---

## Task 4: `cli::registry::index` — typed registry-index.json

**Files:**
- Create: `cli/src/registry/mod.rs`, `cli/src/registry/index.rs`
- Modify: `cli/src/main.rs` (add `mod registry;`)

- [ ] **Step 1: Module skeleton**

`cli/src/registry/mod.rs`:

```rust
//! Read-only access to the AWARE registry index.

pub mod index;
pub mod fetch;

pub use index::{Index, IndexEntry, BundleEntry, VersionEntry};
```

`cli/src/registry/index.rs`:

```rust
use std::collections::BTreeMap;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub version: String,
    #[serde(rename = "updated-at")]
    pub updated_at: String,
    pub agents: BTreeMap<String, IndexEntry>,
    #[serde(default)]
    pub bundles: BTreeMap<String, BundleEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    pub versions: BTreeMap<String, VersionEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    pub tarball: String,
    pub subdir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleEntry {
    pub description: String,
    pub agents: Vec<String>,
}

impl Index {
    pub fn parse<R: Read>(mut r: R) -> Result<Self, AwareError> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;
        serde_json::from_str(&s).map_err(|e| AwareError::Validation(format!("registry index: {e}")))
    }

    /// Resolve `<id>[@version]` → `&VersionEntry`. If version is `None`, return the
    /// lexicographically-greatest version (good enough for v0.2; v0.3+ adds proper semver).
    pub fn resolve(&self, id: &str, version: Option<&str>) -> Result<(&String, &VersionEntry), AwareError> {
        let entry = self.agents.get(id)
            .ok_or_else(|| AwareError::NotFound(format!("agent {id} not in registry")))?;
        let (resolved_version, version_entry) = match version {
            Some(v) => (v, entry.versions.get(v).ok_or_else(||
                AwareError::NotFound(format!("agent {id}@{v} not in registry")))?),
            None => entry.versions.iter().next_back()
                .ok_or_else(|| AwareError::NotFound(format!("agent {id} has no versions")))?,
        };
        Ok((resolved_version, version_entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "version": "1.0",
        "updated-at": "2026-05-16T00:00:00Z",
        "agents": {
            "tekla": {
                "versions": {
                    "2025.0.1": { "tarball": "https://example/tekla.tar.gz", "subdir": "tekla" }
                }
            }
        },
        "bundles": {
            "aware-aeco": { "description": "all", "agents": ["tekla@2025.0.1"] }
        }
    }"#;

    #[test]
    fn parses_minimal_index() {
        let idx = Index::parse(SAMPLE.as_bytes()).unwrap();
        assert_eq!(idx.version, "1.0");
        assert!(idx.agents.contains_key("tekla"));
        assert!(idx.bundles.contains_key("aware-aeco"));
    }

    #[test]
    fn resolves_pinned_version() {
        let idx = Index::parse(SAMPLE.as_bytes()).unwrap();
        let (v, e) = idx.resolve("tekla", Some("2025.0.1")).unwrap();
        assert_eq!(v, "2025.0.1");
        assert_eq!(e.tarball, "https://example/tekla.tar.gz");
    }

    #[test]
    fn missing_agent_is_not_found() {
        let idx = Index::parse(SAMPLE.as_bytes()).unwrap();
        assert!(idx.resolve("nope", None).is_err());
    }
}
```

Add to `cli/src/main.rs`:

```rust
mod registry;
```

- [ ] **Step 2: Run tests**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test registry::index
```

- [ ] **Step 3: Commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/registry cli/src/main.rs
git commit -m "feat(cli): add typed registry index"
```

---

## Task 5: `cli::registry::fetch` — get the index (HTTPS or file://)

**Files:**
- Create: `cli/src/registry/fetch.rs`
- Modify: `cli/src/paths.rs` — add `cache_dir()` method

- [ ] **Step 1: Add `cache_dir()` to Paths**

In `cli/src/paths.rs`, inside `impl Paths`:

```rust
pub fn cache_dir(&self) -> PathBuf {
    self.aware_home.join("cache")
}
```

(Initially `#[allow(dead_code)]` if needed; Task 6 consumes it.)

- [ ] **Step 2: Implement fetch**

`cli/src/registry/fetch.rs`:

```rust
//! Fetch the registry index from HTTPS or a `file://` URL, with TTL-based caching.

use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::error::AwareError;
use crate::registry::Index;

/// Default location of the AWARE registry index. Override via `AWARE_REGISTRY`.
pub const DEFAULT_REGISTRY_URL: &str = "https://raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json";

/// Cache TTL — 1 hour. Re-fetch happens after this expires.
const CACHE_TTL: Duration = Duration::from_secs(60 * 60);

pub fn registry_source() -> String {
    std::env::var("AWARE_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY_URL.to_string())
}

/// Fetch the index. Honors `AWARE_REGISTRY` for the source. Caches at
/// `<cache_dir>/registry-index.json` for `CACHE_TTL`. Falls back to the
/// cached copy on network error.
pub fn fetch_index(cache_dir: &Path) -> Result<Index, AwareError> {
    std::fs::create_dir_all(cache_dir)?;
    let cache_path = cache_dir.join("registry-index.json");
    let source = registry_source();

    // Cache hit within TTL?
    if let Ok(meta) = std::fs::metadata(&cache_path) {
        if let Ok(modified) = meta.modified() {
            if let Ok(age) = SystemTime::now().duration_since(modified) {
                if age < CACHE_TTL {
                    return Index::parse(std::fs::File::open(&cache_path)?);
                }
            }
        }
    }

    // Fetch fresh.
    let body = match fetch_body(&source) {
        Ok(b) => b,
        Err(net_err) => {
            // Stale-cache fallback.
            if cache_path.is_file() {
                eprintln!("warning: registry fetch failed, using stale cache: {net_err}");
                return Index::parse(std::fs::File::open(&cache_path)?);
            }
            return Err(net_err);
        }
    };

    // Validate before writing cache.
    let index = Index::parse(body.as_bytes())?;
    std::fs::write(&cache_path, &body)?;
    Ok(index)
}

fn fetch_body(source: &str) -> Result<String, AwareError> {
    if let Some(path) = source.strip_prefix("file://") {
        return std::fs::read_to_string(path).map_err(AwareError::Io);
    }
    let resp = ureq::get(source)
        .timeout(Duration::from_secs(20))
        .call()
        .map_err(|e| AwareError::Network(format!("GET {source}: {e}")))?;
    resp.into_string().map_err(|e| AwareError::Network(format!("read body: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetches_from_file_url() {
        let tmp = tempfile::tempdir().unwrap();
        let idx = tmp.path().join("idx.json");
        std::fs::write(&idx, r#"{"version":"1.0","updated-at":"x","agents":{},"bundles":{}}"#).unwrap();

        // Use file:// URL via AWARE_REGISTRY.
        let url = format!("file://{}", idx.display());
        // Direct fetch_body call (no env-var pollution in tests).
        let body = fetch_body(&url).unwrap();
        assert!(body.contains("\"version\""));
    }

    #[test]
    fn caches_after_first_fetch() {
        let tmp = tempfile::tempdir().unwrap();
        let idx = tmp.path().join("idx.json");
        std::fs::write(&idx, r#"{"version":"1.0","updated-at":"x","agents":{},"bundles":{}}"#).unwrap();
        let url = format!("file://{}", idx.display());
        // Set AWARE_REGISTRY for this thread only — but std::env::set_var
        // is process-global. We accept that for this test (it's the only one
        // touching this env var in this binary).
        std::env::set_var("AWARE_REGISTRY", &url);

        let cache = tmp.path().join("cache");
        let idx1 = fetch_index(&cache).unwrap();
        assert!(cache.join("registry-index.json").is_file());
        assert_eq!(idx1.version, "1.0");

        // Delete the source and re-fetch — should hit cache.
        std::fs::remove_file(&idx).unwrap();
        let idx2 = fetch_index(&cache).unwrap();
        assert_eq!(idx2.version, "1.0");

        std::env::remove_var("AWARE_REGISTRY");
    }
}
```

- [ ] **Step 3: Run tests**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test registry::fetch
```

- [ ] **Step 4: Commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/registry cli/src/paths.rs
git commit -m "feat(cli): fetch registry index from HTTPS or file://"
```

---

## Task 6: `cli::install::local` — install agent or app from a local folder

**Files:**
- Create: `cli/src/install/mod.rs`, `cli/src/install/local.rs`
- Modify: `cli/src/main.rs` (add `mod install;`)

- [ ] **Step 1: Module skeleton**

`cli/src/install/mod.rs`:

```rust
//! Install agents + apps into `~/.aware/`.

pub mod local;
pub mod registry;
pub mod bundle;
pub mod uninstall;

pub use local::{install_agent_from_path, install_app_from_path};
pub use registry::install_agent_from_registry;
pub use bundle::install_bundle;
pub use uninstall::{uninstall_agent, uninstall_app};
```

`cli/src/install/local.rs`:

```rust
//! Install an agent or app from a local path. Validates first.

use std::path::Path;

use crate::error::AwareError;
use crate::manifest::loader::{load_agent, load_app};
use crate::paths::Paths;
use crate::validate::{has_errors, validate_agent_on_disk, validate_app};

/// Install an agent folder. `src` must contain `manifest.yaml`.
/// Destination is `<paths.agents_dir>/<agent-id>/`. Existing agents are
/// rejected (use `aware agent update` to refresh).
pub fn install_agent_from_path(src: &Path, paths: &Paths) -> Result<String, AwareError> {
    let manifest_path = src.join("manifest.yaml");
    if !manifest_path.is_file() {
        return Err(AwareError::Validation(format!("no manifest.yaml in {}", src.display())));
    }
    let agent = load_agent(&manifest_path)?;
    let issues = validate_agent_on_disk(&agent, src);
    if has_errors(&issues) {
        let summary = issues.iter()
            .filter(|i| i.severity == crate::validate::Severity::Error)
            .map(|i| format!("[{}] {}", i.code, i.message))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(AwareError::Validation(summary));
    }

    let dst = paths.agents_dir().join(&agent.agent);
    if dst.exists() {
        return Err(AwareError::Conflict(format!("agent {} already installed; use `aware agent update {}` to refresh", agent.agent, agent.agent)));
    }
    std::fs::create_dir_all(paths.agents_dir())?;
    copy_dir_recursive(src, &dst)?;
    Ok(agent.agent)
}

/// Install an app folder. `src` must contain a `.flo` (or `.app`) file.
pub fn install_app_from_path(src: &Path, paths: &Paths) -> Result<String, AwareError> {
    // Find the manifest by extension.
    let manifest_path = std::fs::read_dir(src)?
        .flatten()
        .map(|e| e.path())
        .find(|p| matches!(p.extension().and_then(|e| e.to_str()), Some("flo") | Some("app")))
        .ok_or_else(|| AwareError::Validation(format!("no .flo or .app file in {}", src.display())))?;

    let app = load_app(&manifest_path)?;
    let issues = validate_app(&app);
    if has_errors(&issues) {
        let summary = issues.iter()
            .filter(|i| i.severity == crate::validate::Severity::Error)
            .map(|i| format!("[{}] {}", i.code, i.message))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(AwareError::Validation(summary));
    }

    let dst = paths.apps_dir().join(&app.app);
    if dst.exists() {
        return Err(AwareError::Conflict(format!("app {} already installed", app.app)));
    }
    std::fs::create_dir_all(paths.apps_dir())?;
    copy_dir_recursive(src, &dst)?;
    Ok(app.app)
}

pub(crate) fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
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

Add to `cli/src/main.rs`:

```rust
mod install;
```

- [ ] **Step 2: Unit tests**

Append to `local.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn repo_agent_path(rel: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(rel)
    }

    #[test]
    fn installs_tekla_from_repo() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths { aware_home: tmp.path().to_path_buf() };
        let src = repo_agent_path("20-agents/aeco/engineering/tekla");
        let installed = install_agent_from_path(&src, &paths).unwrap();
        assert_eq!(installed, "tekla");
        assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());
        assert!(tmp.path().join("agents/tekla/skills/drawing-identity.md").is_file());
    }

    #[test]
    fn rejects_install_when_already_present() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths { aware_home: tmp.path().to_path_buf() };
        let src = repo_agent_path("20-agents/aeco/engineering/tekla");
        install_agent_from_path(&src, &paths).unwrap();
        let err = install_agent_from_path(&src, &paths).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }

    #[test]
    fn installs_app_from_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths { aware_home: tmp.path().to_path_buf() };
        // Construct a one-file app dir from welded-to-tc.flo.
        let app_src = tmp.path().join("src/welded-to-tc");
        std::fs::create_dir_all(&app_src).unwrap();
        let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("30-apps/_examples/welded-to-tc.flo");
        std::fs::copy(&flo, app_src.join("welded-to-tc.flo")).unwrap();

        let installed = install_app_from_path(&app_src, &paths).unwrap();
        assert_eq!(installed, "welded-to-tc");
        assert!(tmp.path().join("apps/welded-to-tc/welded-to-tc.flo").is_file());
    }
}
```

- [ ] **Step 3: Run tests**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test install::local
```

- [ ] **Step 4: Commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/install cli/src/main.rs
git commit -m "feat(cli): install agent or app from a local folder"
```

---

## Task 7: `cli::install::registry` — install agent from a registry tarball

**Files:**
- Create: `cli/src/install/registry.rs`

- [ ] **Step 1: Implement**

`cli/src/install/registry.rs`:

```rust
//! Install an agent by resolving `<id>[@version]` in the registry index,
//! fetching the tarball, extracting the agent's subdirectory, then handing
//! off to `local::install_agent_from_path`.

use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::AwareError;
use crate::install::local::install_agent_from_path;
use crate::paths::Paths;
use crate::registry::Index;

pub fn install_agent_from_registry(
    id: &str,
    version_pin: Option<&str>,
    paths: &Paths,
    index: &Index,
) -> Result<String, AwareError> {
    let (resolved_version, entry) = index.resolve(id, version_pin)?;

    // Fetch tarball to a temp dir, extract, then install from the subdir.
    let scratch = tempfile::tempdir().map_err(AwareError::Io)?;
    let tarball_path = scratch.path().join("agent.tar.gz");

    // Reuse tarball cache: <cache_dir>/agents/<id>-<version>.tar.gz
    let cache_dir = paths.cache_dir().join("agents");
    std::fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join(format!("{id}-{resolved_version}.tar.gz"));

    if cache_file.is_file() {
        std::fs::copy(&cache_file, &tarball_path)?;
    } else if let Some(path) = entry.tarball.strip_prefix("file://") {
        std::fs::copy(path, &tarball_path)?;
        let _ = std::fs::copy(&tarball_path, &cache_file);
    } else {
        let resp = ureq::get(&entry.tarball)
            .timeout(std::time::Duration::from_secs(60))
            .call()
            .map_err(|e| AwareError::Network(format!("GET {}: {e}", entry.tarball)))?;
        let mut reader = resp.into_reader();
        let mut file = std::fs::File::create(&tarball_path)?;
        std::io::copy(&mut reader, &mut file)?;
        let _ = std::fs::copy(&tarball_path, &cache_file);
    }

    let extract_root = scratch.path().join("extract");
    extract_tarball(&tarball_path, &extract_root)?;

    let subdir = extract_root.join(&entry.subdir);
    if !subdir.is_dir() {
        return Err(AwareError::Validation(format!(
            "registry entry {id}@{resolved_version}: subdir {} not in tarball",
            entry.subdir,
        )));
    }
    install_agent_from_path(&subdir, paths)
}

fn extract_tarball(tarball: &Path, dest: &Path) -> Result<(), AwareError> {
    std::fs::create_dir_all(dest)?;
    let file = std::fs::File::open(tarball)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    archive.unpack(dest).map_err(|e| AwareError::Validation(format!("tarball extract: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{IndexEntry, VersionEntry};
    use std::collections::BTreeMap;

    fn make_test_tarball(path: &Path) {
        // Build a tarball with `aware-main/20-agents/tekla/manifest.yaml` and a skill file.
        use std::io::Write;
        let tar_gz = std::fs::File::create(path).unwrap();
        let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
        let mut tar = tar::Builder::new(enc);

        let manifest_text = std::fs::read_to_string(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent().unwrap()
                .join("20-agents/aeco/engineering/tekla/manifest.yaml")
        ).unwrap();

        let mut header = tar::Header::new_gnu();
        header.set_size(manifest_text.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "aware-main/20-agents/tekla/manifest.yaml", manifest_text.as_bytes()).unwrap();

        // Add each skill file referenced in the manifest.
        let agent: crate::manifest::Agent = serde_yaml::from_str(&manifest_text).unwrap();
        let skills_src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("20-agents/aeco/engineering/tekla/skills");
        for skill in &agent.skills {
            let body = std::fs::read_to_string(skills_src.join(skill)).unwrap();
            let mut h = tar::Header::new_gnu();
            h.set_size(body.len() as u64);
            h.set_mode(0o644);
            h.set_cksum();
            tar.append_data(&mut h, format!("aware-main/20-agents/tekla/skills/{skill}"), body.as_bytes()).unwrap();
        }

        tar.into_inner().unwrap().finish().unwrap().sync_all().ok();
    }

    #[test]
    fn installs_from_local_tarball() {
        let tmp = tempfile::tempdir().unwrap();
        let tarball = tmp.path().join("tekla.tar.gz");
        make_test_tarball(&tarball);

        let mut versions = BTreeMap::new();
        versions.insert("2025.0.1".into(), VersionEntry {
            tarball: format!("file://{}", tarball.display()),
            subdir: "aware-main/20-agents/tekla".into(),
        });
        let mut agents = BTreeMap::new();
        agents.insert("tekla".into(), IndexEntry { versions });
        let index = Index {
            version: "1.0".into(),
            updated_at: "x".into(),
            agents,
            bundles: BTreeMap::new(),
        };

        let aware = tmp.path().join("aware");
        let paths = Paths { aware_home: aware.clone() };
        let installed = install_agent_from_registry("tekla", None, &paths, &index).unwrap();
        assert_eq!(installed, "tekla");
        assert!(aware.join("agents/tekla/manifest.yaml").is_file());
    }
}
```

- [ ] **Step 2: Run tests**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test install::registry
```

- [ ] **Step 3: Commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/install
git commit -m "feat(cli): install agent from registry tarball"
```

---

## Task 8: `cli::install::bundle` — install all agents in a registry bundle

**Files:**
- Create: `cli/src/install/bundle.rs`

- [ ] **Step 1: Implement**

`cli/src/install/bundle.rs`:

```rust
//! Install every agent in a registry bundle. Sequential, best-effort.

use crate::error::AwareError;
use crate::install::registry::install_agent_from_registry;
use crate::paths::Paths;
use crate::registry::Index;

#[derive(Debug)]
pub struct BundleReport {
    pub bundle: String,
    pub installed: Vec<String>,
    pub failed: Vec<(String, String)>, // (agent_spec, error message)
}

pub fn install_bundle(name: &str, paths: &Paths, index: &Index) -> Result<BundleReport, AwareError> {
    let bundle = index.bundles.get(name)
        .ok_or_else(|| AwareError::NotFound(format!("bundle {name} not in registry")))?;

    let mut installed = Vec::new();
    let mut failed = Vec::new();

    for spec in &bundle.agents {
        let (id, version_pin) = match spec.split_once('@') {
            Some((i, v)) => (i, Some(v)),
            None => (spec.as_str(), None),
        };
        match install_agent_from_registry(id, version_pin, paths, index) {
            Ok(_) => installed.push(spec.clone()),
            Err(e) => failed.push((spec.clone(), e.to_string())),
        }
    }

    Ok(BundleReport {
        bundle: name.to_string(),
        installed,
        failed,
    })
}

#[cfg(test)]
mod tests {
    // The bundle install logic is exercised end-to-end by tests/agent_install.rs.
    // Unit-testing here would duplicate the same tarball-mock plumbing.
}
```

- [ ] **Step 2: Commit (no tests at this layer — they live in integration tests)**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/install/bundle.rs
git commit -m "feat(cli): install bundle (sequential, best-effort)"
```

---

## Task 9: `cli::install::uninstall` — remove agent or app folder

**Files:**
- Create: `cli/src/install/uninstall.rs`

- [ ] **Step 1: Implement**

```rust
//! Uninstall — remove an agent or app folder.

use crate::error::AwareError;
use crate::paths::Paths;

pub fn uninstall_agent(id: &str, paths: &Paths) -> Result<(), AwareError> {
    let dir = paths.agents_dir().join(id);
    if !dir.exists() {
        return Err(AwareError::NotFound(format!("agent {id} is not installed")));
    }
    std::fs::remove_dir_all(&dir)?;
    Ok(())
}

pub fn uninstall_app(id: &str, paths: &Paths) -> Result<(), AwareError> {
    let dir = paths.apps_dir().join(id);
    if !dir.exists() {
        return Err(AwareError::NotFound(format!("app {id} is not installed")));
    }
    std::fs::remove_dir_all(&dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uninstalls_existing_agent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths { aware_home: tmp.path().to_path_buf() };
        let dir = paths.agents_dir().join("tekla");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("manifest.yaml"), "agent: tekla\n").unwrap();
        uninstall_agent("tekla", &paths).unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn missing_agent_returns_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths { aware_home: tmp.path().to_path_buf() };
        let err = uninstall_agent("nope", &paths).unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }
}
```

- [ ] **Step 2: Run tests + commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test install::uninstall; cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/install/uninstall.rs
git commit -m "feat(cli): uninstall agent or app"
```

---

## Task 10: Wire `aware agent install <path>` (local)

**Files:**
- Create: `cli/tests/agent_install.rs`
- Modify: `cli/src/commands/agent.rs`

- [ ] **Step 1: Integration test**

`cli/tests/agent_install.rs`:

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn installs_tekla_from_local_path() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success()
        .stdout(predicate::str::contains("tekla"));
    assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());
}

#[test]
fn rejects_install_of_already_installed_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"]).arg(&src).assert().success();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"]).arg(&src).assert().failure().code(8); // Conflict
}
```

- [ ] **Step 2: Update `cli/src/commands/agent.rs`**

In the `dispatch` function, replace the `Install` arm:

```rust
AgentCommand::Install { spec } => install(ctx, &spec),
```

Add the function (recognizes path vs id):

```rust
fn install(ctx: &Context, spec: &str) -> Result<(), AwareError> {
    use std::path::PathBuf;
    let path = PathBuf::from(spec);
    if path.is_dir() {
        let installed = crate::install::install_agent_from_path(&path, &ctx.paths)?;
        println!("✓ installed {installed} from {}", path.display());
        return Ok(());
    }

    // Otherwise: treat as registry id [@version] or bundle name.
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;
    if index.bundles.contains_key(spec) {
        let report = crate::install::install_bundle(spec, &ctx.paths, &index)?;
        println!("✓ bundle {}: {} installed, {} failed", report.bundle, report.installed.len(), report.failed.len());
        for s in &report.installed { println!("  ✓ {s}"); }
        for (s, e) in &report.failed { println!("  ✗ {s}: {e}"); }
        return Ok(());
    }
    let (id, version_pin) = match spec.split_once('@') {
        Some((i, v)) => (i, Some(v)),
        None => (spec, None),
    };
    let installed = crate::install::install_agent_from_registry(id, version_pin, &ctx.paths, &index)?;
    println!("✓ installed {installed}");
    Ok(())
}
```

- [ ] **Step 3: Run tests, commit**

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo test --test agent_install
cargo fmt --all; cargo clippy --all-targets -- -D warnings
git add cli/src/commands/agent.rs cli/tests/agent_install.rs
git commit -m "feat(cli): implement aware agent install (local path)"
```

---

## Task 11: Wire `aware agent install <id>[@version]` and `<bundle>` (registry)

Extend `cli/tests/agent_install.rs` to include registry + bundle paths, using a `file://` registry fixture.

**Files:**
- Modify: `cli/tests/agent_install.rs`
- Create: `cli/tests/fixtures/registry-index.json` and a fixture tarball

This task delivers the registry + bundle test coverage. The implementation code (`install()` in `agent.rs`) already exists from Task 10 — but tests haven't exercised the registry/bundle branches yet.

**Detailed steps:** create the registry-index.json fixture pointing at a generated test tarball; set `AWARE_REGISTRY=file://<fixture>` for the test; assert `aware agent install tekla` works and `aware agent install aware-aeco` reports 7 installed agents.

(Implementation deferred to execution time — the fixture generation echoes the unit-test tarball-builder from Task 7.)

Commit message: `feat(cli): wire aware agent install for registry + bundle`.

---

## Task 12: Wire `aware agent uninstall <id>`

**Files:**
- Create: `cli/tests/agent_uninstall.rs`
- Modify: `cli/src/commands/agent.rs`

- [ ] Test (in `agent_uninstall.rs`):

```rust
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn uninstalls_agent_from_local_install() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
        .join("20-agents/aeco/engineering/tekla");
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"]).arg(&src).assert().success();
    assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());

    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "uninstall", "tekla"]).assert().success()
        .stdout(predicate::str::contains("uninstalled"));
    assert!(!tmp.path().join("agents/tekla").exists());
}

#[test]
fn missing_agent_exits_7() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware").unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "uninstall", "nope"])
        .assert().failure().code(7);
}
```

- [ ] Wire in `agent.rs`:

```rust
AgentCommand::Uninstall { agent } => {
    crate::install::uninstall_agent(&agent, &ctx.paths)?;
    println!("✓ uninstalled {agent}");
    Ok(())
}
```

- [ ] Run + commit:

```
git commit -m "feat(cli): implement aware agent uninstall"
```

---

## Task 13: Wire `aware agent update <id>`

Internally: uninstall + install-from-registry. Reuse the registry resolver. Pin to the latest matching version. (No semver-range resolution; just lex-greatest version.)

**Files:**
- Create: `cli/tests/agent_update.rs`
- Modify: `cli/src/commands/agent.rs`

Implementation pattern:

```rust
AgentCommand::Update { agent } => update(ctx, &agent),

fn update(ctx: &Context, id: &str) -> Result<(), AwareError> {
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;
    // Best-effort: uninstall existing, install fresh
    let _ = crate::install::uninstall_agent(id, &ctx.paths);
    let installed = crate::install::install_agent_from_registry(id, None, &ctx.paths, &index)?;
    println!("✓ updated {installed}");
    Ok(())
}
```

Test: install + update (via mock registry) — verify the manifest version stays consistent.

Commit: `feat(cli): implement aware agent update`.

---

## Task 14: Wire `aware agent validate <path>`

**Files:**
- Create: `cli/tests/agent_validate.rs`
- Modify: `cli/src/commands/agent.rs`

```rust
AgentCommand::Validate { path } => validate_cmd(ctx, &path),

fn validate_cmd(_ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let manifest_path = path.join("manifest.yaml");
    let agent = crate::manifest::loader::load_agent(&manifest_path)?;
    let issues = crate::validate::validate_agent_on_disk(&agent, path);
    if issues.is_empty() {
        println!("✓ {} is valid", path.display());
        return Ok(());
    }
    for i in &issues {
        let tag = match i.severity {
            crate::validate::Severity::Error => "✗",
            crate::validate::Severity::Warning => "⚠",
        };
        println!("{tag} [{}] {}", i.code, i.message);
    }
    if crate::validate::has_errors(&issues) {
        return Err(AwareError::Validation("agent failed validation".into()));
    }
    Ok(())
}
```

Tests cover: valid agent → exit 0; bad manifest → exit 3 with error code; warnings only → exit 0.

Commit: `feat(cli): implement aware agent validate`.

---

## Task 15: Wire `aware app install <path>` with lockfile

**Files:**
- Create: `cli/tests/app_install.rs`
- Modify: `cli/src/commands/app.rs`

```rust
AppCommand::Install { path_or_name } => install(ctx, &path_or_name),

fn install(ctx: &Context, spec: &str) -> Result<(), AwareError> {
    use std::path::PathBuf;
    let path = PathBuf::from(spec);
    if !path.is_dir() {
        return Err(AwareError::NotYetImplemented("app install from registry (use a local path for now)"));
    }
    let app_id = crate::install::install_app_from_path(&path, &ctx.paths)?;

    // Resolve agent versions for the lockfile.
    let app = crate::manifest::loader::load_app(&ctx.paths.apps_dir().join(&app_id).join(format!("{app_id}.flo")))?;
    let mut resolved = std::collections::BTreeMap::new();
    for req in &app.requires {
        // Parse "<id>@<version-spec>" → find matching installed agent.
        let (id, _spec) = req.split_once('@').unwrap_or((req.as_str(), "*"));
        let manifest_path = ctx.paths.agents_dir().join(id).join("manifest.yaml");
        if let Ok(agent_manifest) = crate::manifest::loader::load_agent(&manifest_path) {
            resolved.insert(id.to_string(), agent_manifest.version);
        }
    }
    let lockfile_path = ctx.paths.apps_dir().join(&app_id).join("lockfile.yaml");
    crate::lockfile::write(&app_id, &app.version, resolved, &lockfile_path)?;

    println!("✓ installed {app_id} (lockfile written)");
    Ok(())
}
```

Test exercises the full flow: install both `tekla` and `trimble-connect` agents, then install `welded-to-tc`, verify `lockfile.yaml` exists and pins both agents at the expected versions.

Commit: `feat(cli): implement aware app install (with lockfile)`.

---

## Task 16: Wire `aware app uninstall <id>` and `aware app validate <path>` and `aware app export`

These three are trivial:

- `uninstall`: call `install::uninstall_app` (existing).
- `validate`: load → `validate::validate_app` → print issues.
- `export`: read `<app>/<app>.flo` → write to user-specified output path.

Tests + commit: `feat(cli): implement aware app uninstall, validate, export`.

---

## Task 17: Extend `aware doctor` with installed-agent integrity

Iterate installed agents; for each, run `validate::validate_agent_on_disk` and surface any errors as `⚠` lines under a new `Integrity:` block in the doctor output.

Add an integration test that intentionally corrupts an installed agent (e.g. delete a manifest-listed skill file) and asserts doctor reports the missing-skill error.

Commit: `feat(cli): doctor checks installed-agent integrity (v0.2)`.

---

## Task 18: Author `registry-index.json` at repo root

Hand-write the production registry index listing all 7 agents + the `aware-aeco` bundle. Tarball URLs point at `https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz`; subdirs use the `aware-main/20-agents/...` prefix that GitHub's tarball generator produces.

Verify with: `AWARE_REGISTRY=https://raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json` (after pushing) + `aware agent install tekla` — should pull and install from the live GitHub.

(Note: this requires the registry-index.json to be reachable. Either commit + push to main first, or temporarily run the test against the branch's raw URL.)

Commit: `chore: add registry-index.json for v0.2 install path`.

---

## Task 19: Update `cli/README.md` + final verification

Quickstart additions:

```bash
# Install from a local clone of this repo
aware agent install ./20-agents/aeco/engineering/tekla
aware agent install ./30-apps/_examples/welded-to-tc/    # err, app form
# Or from the registry
aware agent install tekla
aware agent install aware-aeco        # bundle: installs all 7
aware agent uninstall tekla
aware agent update tekla              # re-pull latest
aware agent validate ./my-agent       # exit 3 on errors
aware app install ./my-app            # writes lockfile
```

Full verification:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets    # all tests pass; new tests cover install/validate
cargo build --release       # produces aware.exe
```

Manual smoke (after pushing the branch + registry-index.json):

```powershell
$env:AWARE_HOME = "$env:TEMP\aware-test-home"; Remove-Item -Recurse -Force $env:AWARE_HOME -ErrorAction SilentlyContinue
aware agent install aware-aeco
aware agent list   # expect 7 installed agents
aware doctor       # no integrity issues
aware app install <path-to-welded-to-tc>
Test-Path "$env:AWARE_HOME\apps\welded-to-tc\lockfile.yaml"
```

Commit: `docs(cli): README quickstart for v0.2`.

---

## Self-review

**Spec coverage** — every v0.2 surface from `cli-roadmap.md`:

| Surface | Task |
|---|---|
| `aware agent install <id>[@version]` | 10 + 11 |
| `aware agent install <bundle>` | 11 |
| `aware agent install <path>` | 10 |
| `aware agent uninstall <id>` | 12 |
| `aware agent update <id>` | 13 |
| `aware agent validate <path>` | 14 |
| `aware app install <path>` | 15 |
| `aware app uninstall <id>` | 16 |
| `aware app validate <path>` | 16 |
| Internal: `validate` | 2 |
| Internal: `lockfile` | 3 |
| Internal: `registry::index` | 4 |
| Internal: `registry::fetch` | 5 |
| Internal: `install::local` | 6 |
| Internal: `install::registry` | 7 |
| Internal: `install::bundle` | 8 |
| Internal: `install::uninstall` | 9 |
| `aware doctor` integrity check | 17 |
| Production `registry-index.json` | 18 |
| Docs + final verification | 19 |

**v0.2 DoD** (from roadmap):
- Installing `aware-aeco` from this repo populates `~/.aware/agents/` correctly → Task 18 (registry) + 11 (bundle) + 19 (manual smoke)
- Validators reject malformed inputs → Task 2 + 14 + 16
- `aware doctor` checks installed-agent integrity → Task 17

**Out of scope** (correctly stubbed): `aware agent publish` (registry write — v0.2+), app run/stop/logs (v0.3), connect/disconnect (v0.4), build/skill (v0.5).

**Decision review checklist** (revisit the table at the top before execution):
- HTTP client: `ureq` — confirmed if Decalog #4 weight is high
- Tarball over per-file fetch: confirmed if simplicity > caching efficiency
- Registry URL: hardcoded with env-var override — confirmed if a hosted service isn't in v0.2

---

## Execution handoff

Plan saved to `docs/superpowers/plans/2026-05-16-aware-cli-v02-install-validate.md`.

Same two execution modes as v0.1:

**1. Subagent-Driven** — fresh subagent per task (~19 tasks × 1-3 subagents = ~25-40 subagent dispatches). Uses `superpowers:subagent-driven-development`.

**2. Inline Execution** — execute tasks in this session via `superpowers:executing-plans`.

Which approach?
