# Agent catalog — Implementation Plan

> **For Claude:** Use superpowers:executing-plans (or subagent-driven-development) to implement. Steps use `- [ ]`.

**Goal:** Add a generated, searchable catalog of *available* agents to the `aware` CLI: a `registry-catalog.json` sidecar + generator (`aware agent reindex`), a fetcher, and four read commands (`catalog`, `search`, `has`, `describe --available`).

**Architecture:** A new `cli/src/registry/catalog.rs` defines the catalog structs + pure query/score/build helpers. `aware agent reindex` derives the catalog FROM `registry-index.json` × each agent's on-disk `manifest.yaml` (never invents keys). `fetch_catalog` mirrors `fetch_index`. Four new `AgentCommand` variants read the catalog. Pure cores (`build_catalog`, `score_agent`, `agent_has`) are unit-tested; commands are thin IO shells.

**Tech Stack:** Rust (the `cli` crate), serde/serde_json, `ureq` (already used by fetch.rs), `cargo test`/`fmt`/`clippy`.

**Spec:** `docs/superpowers/specs/2026-06-01-agent-catalog-design.md`. **Worktree:** `D:/Repos/aware-aeco-agent-catalog` (branch `feat/agent-catalog`). Build/test from `cli/`.

**Standing rule (CLAUDE.md):** verified-AWARE-bug discipline doesn't apply (we're adding a feature); dual review (pr-toolkit + Codex) before "done"; commit at discretion, no `Co-Authored-By`; branch + PR (no merge/release — that needs bypassPermissions).

---

## Task 1: Catalog data model + parse + query helpers
**Files:** Create `cli/src/registry/catalog.rs`; modify `cli/src/registry/mod.rs` (add `pub mod catalog;` + re-exports).
- [ ] Write failing tests (in `catalog.rs` `#[cfg(test)]`): `Catalog::parse` a fixture; `CatalogAgent::latest()` returns the greatest version key; `agent_has` matches a command name, a `method`, and a skill (case-insensitive), and returns empty for a miss.
- [ ] `cargo test -p aware catalog::` → FAIL (module missing).
- [ ] Implement structs (`Catalog{version, updated_at, agents: BTreeMap<String,CatalogAgent>}`, `CatalogAgent{display_name:Option, vendor:Option, keywords:Vec, versions: BTreeMap<String,CatalogVersion>}`, `CatalogVersion{description, status, stateful, sdk_target:Option, transport:String, skills:Vec<String>, commands:Vec<CatalogCommand>}`, `CatalogCommand{name, description, lifecycle, category, mode:Option, method:Option, path:Option}`), `parse`, `latest()`, and `agent_has(&CatalogAgent, cap)->Vec<Hit>`.
- [ ] `cargo test -p aware catalog::` → PASS. Commit.

## Task 2: `build_catalog` (pure generator core)
**Files:** `cli/src/registry/catalog.rs`.
- [ ] Failing tests: `build_catalog(&Index, load)` where `load(subdir)->Result<Agent>` is a fake returning 2 agents → a `Catalog` whose agent+version KEYS come from the Index (not `manifest.version`), metadata from the manifest; a loader that errors for one subdir → that agent skipped + returned in the errors vec (catalog still has the others).
- [ ] Run → FAIL.
- [ ] Implement `build_catalog(index, load) -> (Catalog, Vec<(String,String)>)`: for each `(id, IndexEntry)` and each `(ver, VersionEntry)`, call `load(&version_entry.subdir)`; map `Agent` → `CatalogVersion` (+ agent-level `display_name`/`vendor`/`keywords` from the first/any version's manifest); collect load errors.
- [ ] Run → PASS. Commit.

## Task 3: `aware agent reindex` (IO shell + `--check`)
**Files:** `cli/src/commands/agent.rs` (new `Reindex{check:bool}` variant + handler), reuse `find_registry_root`.
- [ ] Implement `reindex(ctx, check)`: walk up for `registry-index.json`; parse it; `build_catalog` with a loader that maps `subdir` (`aware-main/<rel>` → `<repo>/<rel>`) and `manifest::loader::load_agent(<dir>/manifest.yaml)`; on `--check`, compare serialized output to the on-disk `registry-catalog.json` and exit nonzero if different (no write); else write pretty JSON + `now_iso()`. Report skipped agents; nonzero exit if any failed to load.
- [ ] Manual smoke: `cargo run -p aware -- agent reindex` in the worktree → writes `registry-catalog.json`; eyeball tekla/rhino entries.
- [ ] Commit (code only; the generated catalog lands in Task 9).

## Task 4: `fetch_catalog` (mirror `fetch_index`)
**Files:** `cli/src/registry/fetch.rs`.
- [ ] Failing test: `fetch_catalog` from a `file://` fixture caches + returns; an absent source with no cache → a typed `CatalogUnavailable` (not a hard error).
- [ ] Run → FAIL.
- [ ] Implement `DEFAULT_CATALOG_URL`, `AWARE_CATALOG` override, TTL cache at `<cache>/registry-catalog.json`, stale-fallback, and a typed "unavailable" result for 404/missing.
- [ ] Run → PASS. Commit.

## Task 5: `score_agent` + `aware agent search`
**Files:** `cli/src/registry/catalog.rs` (scoring), `cli/src/commands/agent.rs` (`Search{query, capability:bool}` + handler).
- [ ] Failing tests for `score_agent(query, &CatalogAgent, opts)`: id/name match outranks description match outranks skill match; `--capability` biases to command name/method; no-match → None.
- [ ] Run → FAIL. Implement scoring + the command (fetch catalog → rank → print ranked hits with matched field + best snippet; `--json`). Run → PASS. Commit.

## Task 6: `aware agent catalog` (list all available)
**Files:** `cli/src/commands/agent.rs` (`Catalog` variant + handler).
- [ ] Implement: fetch catalog → `Table[ID, DISPLAY-NAME, VERSION, STATUS, COMMANDS, SKILLS, DESCRIPTION]` (latest version per agent; one-line description); `--json` via `envelope::print_ok`. Smoke against a `file://` fixture. Commit.

## Task 7: `aware agent has <agent> <capability>`
**Files:** `cli/src/commands/agent.rs` (`Has{agent, capability}` + handler).
- [ ] Implement: fetch catalog → `agent_has` → print matches; exit 0 if any, nonzero if none; unknown agent → clean message + nonzero; `--json`. Commit.

## Task 8: `describe --available` (catalog fallback)
**Files:** `cli/src/commands/agent.rs` (`Describe` gains `--available`; handler falls back to catalog).
- [ ] Change `Describe{agent, available:bool}`. Handler: try `discover_agents` (installed); on miss or `--available`, fetch catalog → print the same describe view from `CatalogVersion` (description, commands, skills) tagged "(from registry catalog — not installed)". Keep installed-path output byte-identical. `--json` includes an `installed:bool`. Commit.

## Task 9: Generate the real catalog + wire help
**Files:** generated `registry-catalog.json` (repo root); `agent.rs` doc-comments/help.
- [ ] Update the `//! Phase` doc-comment + variant `///` help for the four commands.
- [ ] `cargo run -p aware -- agent reindex` → commit the real generated `registry-catalog.json`.

## Task 10: Real E2E + fmt/clippy + dual review
- [ ] `cargo fmt --check` + `cargo clippy -p aware -- -D warnings` clean; `cargo test -p aware` green.
- [ ] Real run (no stubs), `AWARE_CATALOG=file://<worktree>/registry-catalog.json`: `aware agent catalog` (lists real agents incl. tekla), `aware agent search "steel"` + `search --capability macro`, `aware agent has tekla <real-cmd>` (exit 0) + a bogus capability (nonzero), `aware agent describe rhino-8 --available` (real desc+commands, not installed). Capture output.
- [ ] Dual review (pr-toolkit + codex:rescue) on `git diff main...HEAD`; apply sound findings.
- [ ] `superpowers:finishing-a-development-branch` → open a PR (no merge/release; that needs bypassPermissions).

## Notes
- `cargo` builds the whole workspace; scope test runs with `-p aware` (the cli crate package name — confirm via `cli/Cargo.toml`).
- The catalog is DERIVED from the index: never emit an agent/version key the index doesn't have (install alignment).
- Match the existing `--json` envelope (`envelope::print_ok`) + `render::table::Table` patterns already used by `list`/`describe`.
