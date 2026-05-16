# `aware` CLI

The runtime binary for the AWARE substrate.

This folder houses the Rust implementation. The contract it satisfies is in [`10-core/cli-spec.md`](../10-core/cli-spec.md); the phased delivery plan is in [`10-core/cli-roadmap.md`](../10-core/cli-roadmap.md); the engineering rules that apply across every session are in [`CLAUDE.md`](../CLAUDE.md) at the repo root.

## Status

**v0.2 — install + validate.** Sixteen surfaces shipped (eight from v0.1 plus eight new):

- `aware --version` / `aware --help`
- `aware doctor` (now includes agent-integrity checks)
- `aware agent list` / `agent describe <id>` / `agent skill <id> <skill>`
- `aware agent install <path>` / `agent install <id>[@version]` / `agent install <bundle>`
- `aware agent uninstall <id>` / `agent update <id>` / `agent validate <path>`
- `aware app list` / `app show <id>`
- `aware app install <path>` / `app uninstall <id>` / `app validate <path>` / `app export <id> <output>`

Still stubbed: `agent publish`, `app run/stop/logs` (v0.3), `connect / disconnect` (v0.4),
`build agent`, `skill ...` (v0.5).

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
