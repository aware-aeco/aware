# AWARE — Project Instructions

You are working on **AWARE** (`aware-aeco/aware`) — the open-source agentic substrate for AECO. The substrate is content-complete (manifesto, decalog, specs, 7 agents, 58 skills, 2 reference apps, 3 meta-primitives). The current focus is the **`aware` CLI** — the runtime binary that executes everything the substrate describes.

## Read these first (in order)

Every load-bearing decision in this repo gets checked against these. If a decision conflicts with one of them, the decision is wrong, not the rule.

1. **[`00-vision/decalog.md`](./00-vision/decalog.md)** — 5 structural truths. The tiebreaker for any design dispute.
2. **[`00-vision/manifesto.md`](./00-vision/manifesto.md)** — what AWARE is, why, how it ships.
3. **[`10-core/agent-spec.md`](./10-core/agent-spec.md)** — the agent contract.
4. **[`10-core/app-spec.md`](./10-core/app-spec.md)** — the app composition format.
5. **[`10-core/cli-spec.md`](./10-core/cli-spec.md)** — what the CLI must do (the contract you are implementing).
6. **[`10-core/cli-roadmap.md`](./10-core/cli-roadmap.md)** — phased delivery; what ships in v0.1 vs later.

## Tech stack

| Layer | Choice | Why |
|---|---|---|
| Language | **Rust** (edition 2024) | Single static binary, no runtime deps, cross-platform native, fast. Aligns with decalog #4 (no vendor in the loop) — binaries don't decay. |
| Arg parsing | `clap` (derive macros) | Industry standard, ergonomic, auto-generates `--help`. |
| Manifests | `serde` + `serde_yaml` | Native deserialization of agent / app YAML files into typed structs. |
| Async | `tokio` | Required for the runtime phase (long-running stateful agents). |
| Templating | `minijinja` | For `{{ }}` substitution in app files (lightweight, no Jinja2 dep). |
| HTTP (later) | `reqwest` | For OAuth flows + REST-agent invocation. |
| Testing | `assert_cmd` + `predicates` | CLI integration tests; one test per command-surface. |
| CI / lint | `cargo fmt` + `cargo clippy -D warnings` | Required to pass before merge. |

Code lives at **`cli/`** under the repo root (monorepo for v0; split out later if it warrants its own repo).

## Code style

- **Rust 2024 edition.** Use modern idioms (`let-else`, `if-let chains`, etc.).
- **`cargo fmt` must pass** — no manual formatting wars.
- **`cargo clippy -D warnings` must pass** — clippy lints are errors, not suggestions.
- **No `unsafe`** unless explicitly justified with a comment block explaining the invariant. Run `cargo geiger` periodically.
- **Errors as data, not exceptions.** All public functions return `Result<T, E>` with concrete error types via `thiserror`. No `unwrap()` outside of tests + main entry.
- **Functions named for behavior, not types.** `load_agent_manifest()` not `do_yaml_parse_on_path()`.
- **Tests live next to code** (`#[cfg(test)] mod tests` at the bottom of each module) for unit tests. Integration tests go under `cli/tests/`.

## Build / run / test commands

From the **repo root**:

```bash
# First-time setup
cd cli
cargo build                          # debug build → target/debug/aware
cargo build --release                # optimized → target/release/aware

# Run
cargo run -- --help                  # show top-level help
cargo run -- agent list              # run subcommand

# Tests
cargo test                           # unit + integration
cargo test -- --nocapture            # show println output during tests

# Lint + format (mandatory before commit)
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

When the binary is installed system-wide (`cargo install --path cli`) it appears as `aware` on the PATH. Until then, use `cargo run --` as the entry point.

## The CLI surface in one paragraph

The CLI manages everything below `~/.aware/`: installed agents, installed apps, encrypted credentials, execution logs. It has five command groups:

| Group | Purpose | First phase |
|---|---|---|
| `aware agent ...` | List, describe, install, validate, uninstall agents | v0.1 (read-only) → v0.2 (install / validate) |
| `aware app ...` | List, describe, install, run, stop, validate, uninstall apps | v0.1 (read-only) → v0.2 (install / validate) → v0.3 (run / stop — the runtime) |
| `aware connect ...` | Provision / refresh OAuth credentials for an agent | v0.4 |
| `aware skill ...` | Author / port / modify / eval skills (delegates to skill-creator) | v0.5 |
| `aware build ...` | Generate an agent from DLL / NuGet / OpenAPI / etc. | v0.5 |

Full surface in [`10-core/cli-spec.md`](./10-core/cli-spec.md). Per-phase delivery in [`10-core/cli-roadmap.md`](./10-core/cli-roadmap.md).

## Engineering rules — non-negotiable

These rules apply to every session in this repo. Inherited from the parent CLAUDE.md (`D:\Repos\floless-app\CLAUDE.md`).

### Verify before answering

No claims from inference, summaries, or memory alone. Before stating anything as fact about this project's plans, requirements, code, or features, read the primary source end-to-end. Filenames, section headings, README summaries, and prior assistant turns are leads — not evidence. Ground every load-bearing claim in a citable artifact (path + line, or verbatim quote).

### No corner-cutting

If a source exists, open it and use it. Don't write from scratch what already exists in production. MEMORY entries are pointers, not substitutes. Before any non-trivial write: `Glob` + `Read` the closest existing artifact in this codebase. The 30-second check is always cheaper than the redo cost.

Concrete failure modes to refuse:
- Writing "exemplary" content from MEMORY summaries when production versions sit elsewhere in the repo
- Calling something "exemplary" when it's actually "what I had time/energy for"
- Stopping when a task feels long. **If it feels long, that IS the work.**

### Coding is solved

There is nothing that cannot be done with software. The job is to figure out HOW. Walls are claims that must be earned. Before saying "I can't" or "this isn't possible," exhaust:

1. Read more codebase (`Grep`, `Read`, `Glob`)
2. Spawn a specialized agent (`Agent` tool)
3. WebSearch / WebFetch
4. Try a relevant skill (`Skill` tool)
5. Decompose into smaller pieces
6. Try alternative tools / libraries / APIs / techniques
7. Combine multiple approaches

Only after exhausting may a real wall be surfaced, framed as *"I tried X, Y, Z; the wall is W; closest workable alternative is …"*.

### Use skill-creator for all skill work

All skill creation, modification, or porting routes through Anthropic's `skill-creator` skill (via the `Skill` tool). No exceptions for "quick edits" or "just porting." See the [aware-skill-builder](./20-agents/_core/aware-skill-builder/) for the AWARE-specific pipeline that wraps skill-creator.

### Git workflow

- **Never commit automatically.** Only commit when the user explicitly asks. Wait for approval.
- **No `Co-Authored-By: Claude ...` trailers** in commit messages.
- **Session cleanup before commit** — delete `tmpclaude-*` temp files first.
- Stage specific files (`git add <path>`); avoid `git add -A` to prevent accidental secret commits.

### PR review — non-negotiable

- **Every PR must be reviewed before merge.** No PR merges without a review pass; address all findings (or justify why not) before merging.
- **Codex reviews first.** Run `codex exec review --base main` on the branch. Codex is the primary reviewer.
- **Fall back to the local reviewer only if Codex is genuinely unavailable** (rate-limited, errored, or not installed): use the `pr-review-toolkit:code-reviewer` agent instead.
- **Re-check Codex every time.** A one-time rate-limit (e.g. "try again at …") is *not* permanent — try Codex again on the next PR / next day before falling back. Don't coast on the local reviewer because Codex was down once.

## What's already shipped

The substrate is content-complete. Do not re-litigate decisions captured in the docs above unless you have a concrete new constraint.

- 7 agents under `20-agents/` (engineering: tekla; construction: trimble-connect; cross-cutting: microsoft-365, google-workspace; meta: aware-agent-builder, aware-skill-builder; utility: html-report)
- 58 skill files
- 2 reference apps under `30-apps/_examples/`
- Diagrams in `40-diagrams/` (Mermaid + Excalidraw)
- Issue + PR templates under `.github/`

## What's owed (current focus)

Build the `aware` CLI per `cli-roadmap.md`, starting with v0.1 read-only commands. The substrate gives you exact target behavior — your job is implementation, not redesign.

## Related repos

- **Parent project** (`floless-app`): `D:\Repos\floless-app`. The commercial canvas + the source of ported skills.
- **floless-web**: `D:\Repos\floless-web`. Marketing site, unrelated to AWARE substrate.
