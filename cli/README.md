# `aware` CLI

The runtime binary for the AWARE substrate.

This folder houses the Rust implementation. The contract it satisfies is in [`10-core/cli-spec.md`](../10-core/cli-spec.md); the phased delivery plan is in [`10-core/cli-roadmap.md`](../10-core/cli-roadmap.md); the engineering rules that apply across every session are in [`CLAUDE.md`](../CLAUDE.md) at the repo root.

## Status

**v0.4 — auth + host plugins.** Twenty-three surfaces shipped (nineteen from v0.1-v0.3 plus four new):

- `aware --version` / `aware --help`
- `aware doctor` (now includes Integrity, Running, **and Credentials** checks)
- `aware agent list` / `agent describe <id>` / `agent skill <id> <skill>`
- `aware agent install <path|id[@version]|bundle>` / `agent uninstall <id>` / `agent update <id>` / `agent validate <path>` (auto-regenerates host plugins on install/uninstall)
- `aware app list` / `app show <id>`
- `aware app install <path>` / `app uninstall <id>` / `app validate <path>` / `app export <id> <output>`
- `aware app run <app>` / `app stop <app>` / `app logs <app> [--tail | --replay]`
- `aware connect <integration>` — PKCE OAuth flow + encrypted token storage ← v0.4
- `aware connect <integration> --refresh` — force refresh ← v0.4
- `aware disconnect <integration>` ← v0.4
- `aware plugins regenerate` — rebuild host plugin folders from installed agents ← v0.4

Still stubbed: `agent publish` (registry-write), `build agent` (v0.5), `skill ...` (v0.5).

### Connect to an integration (v0.4)

```bash
# PKCE OAuth flow — opens your browser, listens on localhost:7421, stores the token encrypted
aware connect trimble-connect

# Multiple accounts of the same integration
aware connect google-workspace --as personal
aware connect google-workspace --as work

# Force refresh (refresh_token must exist)
aware connect trimble-connect --refresh

# Add extra scopes beyond the defaults
aware connect microsoft-365 --scopes Calendars.Read,Mail.Send

# Remove credentials
aware disconnect trimble-connect
aware disconnect google-workspace --as personal
```

**Encryption.** Tokens are encrypted at rest by the OS keychain (Windows DPAPI / macOS Keychain / Linux libsecret) under service `aware-aeco`. Never written to logs.

**Refresh.** When the runtime reads a credential, it lazily refreshes if the token expires within 60 seconds. Transparent.

**Client IDs.** v0.4 ships with placeholder client IDs. To run a real OAuth flow against Trimble/Microsoft/Google, set:

```
$env:AWARE_OAUTH_TRIMBLE_CLIENT_ID = "your-registered-client-id"
$env:AWARE_OAUTH_M365_CLIENT_ID    = "your-registered-client-id"
$env:AWARE_OAUTH_GOOGLE_CLIENT_ID  = "your-registered-client-id"
```

### Host plugins (v0.4)

The `aware-aeco` plugin bundle is auto-generated from installed agents whenever you run `agent install` or `agent uninstall`. It lands under:

- `~/.claude/plugins/aware-aeco/` (first-class — full plugin manifest + per-command markdown)
- `~/.codex/plugins/aware-aeco/` (scaffold — format pending)
- `~/.opencode/plugins/aware-aeco/` (scaffold — format pending)

Force a rebuild anytime:

```bash
aware plugins regenerate
```

Regeneration is idempotent — re-running produces byte-identical output, so you can safely version-control the plugin directories.

For non-default plugin locations (CI, test fixtures, custom setups), override:

```
$env:AWARE_PLUGINS_CLAUDE   = "C:\test\.claude\plugins"
$env:AWARE_PLUGINS_CODEX    = "C:\test\.codex\plugins"
$env:AWARE_PLUGINS_OPENCODE = "C:\test\.opencode\plugins"
```

## Build

```bash
cd cli
cargo build                          # debug → target/debug/aware
cargo build --release                # release → target/release/aware
```

First build pulls ~30 crates (clap, serde, tokio, etc.). Subsequent builds are incremental.

## Run

```bash
# Default reads ~/.aware/. Override via AWARE_HOME for testing:
export AWARE_HOME=$(pwd)/test-fixtures/aware

# Read-only (v0.1)
aware doctor                              # filesystem + integrity health check
aware agent list                          # table of installed agents
aware agent describe tekla
aware agent skill tekla drawing-identity
aware app list
aware app show welded-to-tc
aware --json agent list

# Install + validate (v0.2)
aware agent install ./20-agents/aeco/engineering/tekla       # local path
aware agent install tekla                                    # from registry
aware agent install aware-aeco                               # bundle: all 7 agents
aware agent uninstall tekla
aware agent update tekla
aware agent validate ./my-agent
aware app install ./my-app                                   # writes lockfile.yaml
aware app validate ./my-app
aware app export welded-to-tc /tmp/exported.flo
```

### Run an app (v0.3)

```bash
# One-shot app (all nodes stateless): runs to completion, exits
aware app run my-oneshot-app

# Long-running app (any stateful node like a watcher): blocks until Ctrl+C
aware app run welded-to-tc --instance fab-east

# Inputs and config overrides
aware app run welded-to-tc --config tc-project-id=proj-123 --config tc-folder-id=folder-456

# Stop a long-running app from another terminal
aware app stop welded-to-tc --instance fab-east

# Read execution traces
aware app logs welded-to-tc                       # raw JSONL of most recent run
aware app logs welded-to-tc --tail                # follow live
aware app logs welded-to-tc --replay              # pretty-printed replay of most recent run
aware app logs welded-to-tc --replay --run-id r_abc123   # specific run
```

The runtime invokes each agent's CLI transport binary by name (from `transport.cli.binary`).
Until v0.5 builds those binaries, expect "binary not found" errors at run-time — the orchestrator
logic itself is tested with in-process mocks under the hood.

### Templating

Node configs support Jinja-style substitution:

- `{{ inputs.X }}` — inputs passed via `--config` or app-level config
- `{{ <node-id>.<output-field> }}` — upstream outputs (kebab-case node ids are auto-translated to underscore; use bracket syntax for raw kebab access: `{{ upstream["my-node"].field }}`)
- `{{ secrets.<id> }}` — credentials from `~/.aware/credentials/<id>.json`
- `{{ config.X }}` — app-level config kv from `~/.aware/apps/<app>/config.yaml`

### Provenance

Every run writes a JSONL trace to `~/.aware/logs/<app>/<instance>/<run-id>.jsonl` with events for
`run-start`, `node-start`, `node-output`, `node-error`, `node-stop`, `run-end`. Replay them at any time
with `aware app logs ... --replay`.

By default the CLI reads the registry from
`https://raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json`.
Override with `AWARE_REGISTRY=<https-or-file-url>` for testing or alternative registries.

## Test

```bash
cargo test                           # unit + integration
cargo test -- --nocapture            # show println output
```

## Lint + format

```bash
cargo fmt --all                      # mandatory before commit
cargo clippy --all-targets -- -D warnings
```

Both must pass with zero diff / warnings before merge. CI enforces this once GitHub Actions is set up.

## Layout

```
cli/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                      # Clap entry + dispatch
│   ├── error.rs                     # AwareError + exit-code mapping
│   └── commands/
│       ├── mod.rs
│       ├── agent.rs                 # aware agent ...
│       ├── app.rs                   # aware app ...
│       ├── connect.rs               # aware connect / disconnect
│       ├── skill.rs                 # aware skill ...
│       ├── build.rs                 # aware build ...
│       └── doctor.rs                # aware doctor
└── tests/
    └── basic.rs                     # CLI smoke tests via assert_cmd
```

## Implementation order

Follow [`10-core/cli-roadmap.md`](../10-core/cli-roadmap.md). Don't ship a half-implemented phase and call it v0.x.

| Phase | Surface |
|---|---|
| v0.1 | read-only — `agent list/describe/skill`, `app list/show`, partial `doctor` |
| v0.2 | install + validate |
| v0.3 | runtime — `app run/stop/logs` (the hard one) |
| v0.4 | auth + host plugins |
| v0.5 | builders — `aware build agent`, `aware skill ...` |

## Engineering rules

See [`CLAUDE.md`](../CLAUDE.md). Non-negotiable:

- Verify before answering (read sources, not summaries)
- No corner-cutting (if a source exists, open it)
- Coding is solved (no "can't be done" as first response)
- All skill work routes through Anthropic's `skill-creator`
- Decalog is the design tiebreaker
- `cargo fmt` + `cargo clippy -D warnings` before commit

## Why Rust

| Reason | |
|---|---|
| Single static binary | Works on every OS without a runtime. `winget install aware-aeco`, done. |
| Cross-platform native | Windows-first matters for AECO; Rust's Windows story is excellent. |
| Decalog alignment | "No vendor in the loop." A statically-linked binary outlives its build environment. |
| Type safety for manifests | `serde` deserializes YAML → typed structs; schema bugs are compile errors. |
| Async runtime | `tokio` handles the long-running stateful-agent case cleanly (v0.3). |
| Distribution | `cargo install` works for devs; `winget` / `brew` / `curl-pipe` for users. |

If `cargo build` ever takes too long, consider splitting the runtime (`v0.3`) into a separate crate.
