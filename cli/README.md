# `aware` CLI

The runtime binary for the AWARE substrate.

This folder houses the Rust implementation. The contract it satisfies is in [`10-core/cli-spec.md`](../10-core/cli-spec.md); the phased delivery plan is in [`10-core/cli-roadmap.md`](../10-core/cli-roadmap.md); the engineering rules that apply across every session are in [`CLAUDE.md`](../CLAUDE.md) at the repo root.

## Status

**v0.1 — read-only foundation.** Eight surfaces shipped:

- `aware --version` / `aware --help`
- `aware doctor`
- `aware agent list` / `agent describe <id>` / `agent skill <id> <skill>`
- `aware app list` / `app show <id>`

Everything else (`install`, `validate`, `run`, `connect`, `build`, `skill ...`)
remains `NotYetImplemented` until its phase per [cli-roadmap.md](../10-core/cli-roadmap.md).

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
