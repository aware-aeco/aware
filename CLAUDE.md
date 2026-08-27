# AWARE — Project Instructions

You are working on **AWARE** (`aware-aeco/aware`) — the open-source agentic substrate for AECO. The substrate is content-complete (manifesto, decalog, specs, <!--stat:agents_total-->79<!--/stat--> agents, <!--stat:skills-->3,346<!--/stat--> skills, <!--stat:apps-->8<!--/stat--> reference apps, <!--stat:meta_primitives-->11<!--/stat--> meta-primitives) and the `aware` CLI has shipped (v<!--stat:cli_version-->0.131.0<!--/stat-->). The current focus is the **`aware` CLI** — the runtime binary that executes everything the substrate describes.

## Read these first (in order)

Every load-bearing decision in this repo gets checked against these. If a decision conflicts with one of them, the decision is wrong, not the rule.

1. **[`00-vision/decalog.md`](./00-vision/decalog.md)** — 9 structural truths. The tiebreaker for any design dispute.
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

These rules apply to every session in this repo.

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

- **Committing is pre-approved for this project.** You have standing approval to create commits when a unit of work is complete — no need to ask per commit. (Pushing and force-pushing still require explicit approval each time. Merging to `main` does too, with one carve-out below.)
- **Carve-out — a gated PR may merge itself.** The scheduled maintenance routines that open `routine/*` PRs may merge their own PR without asking, but only when Codex's GitHub code review has reviewed the **final** commit and has nothing outstanding, and CI is green on that same commit. Codex names the commit it read, so an approval from before a fix does not cover the fix — pushing changes forces a re-review. If Codex never responded and only a same-model in-harness reviewer ran, the PR stays open for a human: merging is earned by a cross-model review, not by the absence of one. Granted by Pawel 2026-08-02, when that Codex gate went in; before it there was no cross-model review available to an unattended run, which is why this rule was absolute. Nothing else about `main` changes: direct pushes still need approval.
- **`--admin` is the merge mechanism here, not a bypass of review.** `gh pr merge <n> --squash --admin --delete-branch` is authorized and is the *only* thing that works: the `protect-main` ruleset requires one approving code-owner review, `.github/CODEOWNERS` is `*  @pawellisowski`, and GitHub forbids approving your own PR — so on a PR you opened, that requirement can never be satisfied and a plain `gh pr merge` always fails with *"the base branch policy prohibits the merge"*, however green the PR is. `--admin` clears that unsatisfiable self-approval and nothing else. It does **not** license clearing the gate above: no merge without Codex having read the final commit with nothing outstanding and CI green on that same commit. Working as Pawel carries his admin rights — use them to land reviewed work, never to skip the review. `main`'s history stays append-only regardless: no force-push, no rewrite.
- **No `Co-Authored-By: Claude ...` trailers** in commit messages.
- **Supply the squash body explicitly when merging** — `gh pr merge --body '...'`, or the REST merge API's `commit_message`. Leave it out and GitHub generates one, and a generated body carries a `Co-authored-by:` trailer for every commit *author* on the branch other than the account clicking merge. No message anywhere has to contain the string, so a branch whose commits are all clean still lands the trailer: that is how 0789633b (#408) and four merges before it broke the rule above with `trailers.yml` green. Commit under the identity the work lands under and there is nothing for GitHub to synthesise; `scripts/no-claude-coauthor-trailers.py` fails a PR whose commits are Claude-*authored* for that reason. See #411.
- **Session cleanup before commit** — delete `tmpclaude-*` temp files first.
- Stage specific files (`git add <path>`); avoid `git add -A` to prevent accidental secret commits.

### PR review — non-negotiable

- **Every PR must be reviewed before merge.** No PR merges without a review pass; address all findings (or justify why not) before merging.
- **Codex reviews first.** Codex is the primary reviewer, and it reaches the branch two ways. On a machine with the CLI, run `codex exec review --base main`. In an environment without it — notably the scheduled cloud routines, which have no Codex CLI, no credential and no egress of their own — comment `@codex review` on the PR and use Codex's GitHub code review, which runs from GitHub's side on the maintainer's ChatGPT subscription. **Both are Codex; either satisfies this rule.** Do not treat an absent CLI as an absent reviewer.
- **Fall back to the local reviewer only if Codex is genuinely unavailable by BOTH routes** (CLI rate-limited/errored/not installed *and* no GitHub review inside the poll window): use the `pr-review-toolkit:code-reviewer` agent instead. A PR reviewed only that way must say so, and — per the Git workflow carve-out above — must not self-merge.
- **Re-check Codex every time.** A one-time rate-limit (e.g. "try again at …") is *not* permanent — try Codex again on the next PR / next day before falling back. Don't coast on the local reviewer because Codex was down once.

## What's already shipped

The substrate is content-complete. Do not re-litigate decisions captured in the docs above unless you have a concrete new constraint.

- <!--stat:agents_total-->79<!--/stat--> agents under `20-agents/` — <!--stat:agents_curated-->30<!--/stat--> curated (tekla, trimble-connect, navisworks, microsoft-365, google-workspace, …) + <!--stat:agents_reflected-->49<!--/stat--> reflected (revit, rhino, autocad, idea-statica, …); full list in `registry-index.json`
- <!--stat:skills-->3,346<!--/stat--> skill files
- <!--stat:apps-->8<!--/stat--> reference apps under `30-apps/_examples/`
- Diagrams in `40-diagrams/` (Mermaid + Excalidraw)
- Issue + PR templates under `.github/`

## What's owed (current focus)

Build the `aware` CLI per `cli-roadmap.md`, starting with v0.1 read-only commands. The substrate gives you exact target behavior — your job is implementation, not redesign.

## Related repos

- **Ported-skill source** (`floless-app` at `D:\Repos\floless-app`) — the external codebase several AWARE skills were ported from; the skill-builder strips its source-runtime-isms on port.
