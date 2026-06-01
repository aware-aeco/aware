# Agent catalog: browse / search / capability-check available agents — design (2026-06-01)

**Status:** design-approved (Pawel, 2026-06-01). Sub-project ② of a three-part floless.app
fresh-install request (③ floless.app version fixes — done; ② this; ① an onboarding skill that
teaches ② + the rest). Adds a generated, searchable **catalog** of *available* agents to the
`aware` CLI so a freshly-installed user can discover what exists before installing. No
implementation yet.

## Problem

On a fresh AWARE install, `aware agent list` shows only **installed** agents, and `aware agent
describe` works only on **installed** agents. There is no way to see *what agents exist*, what each
one does, whether an agent exposes a particular capability/method, or to search agents by
functionality. The install path fetches a **registry index** (`registry-index.json`), but that
index is **thin** — each entry is only `{versions: {ver: {tarball, subdir}}}` (no description, no
commands, no skills). The rich metadata lives in each agent's `manifest.yaml` inside the repo.

## Decision (approved)

Add a **generated catalog sidecar** `registry-catalog.json` (separate from the install-critical
thin index — lower risk) carrying the searchable per-agent metadata, plus a generator, a fetcher,
and four read commands. All three requested sub-features are in scope: **list/describe available**,
**search by functionality**, **capability check**.

## What exists (verified)

- **Registry index** `registry-index.json` (repo root). Keys are curated/SDK-aligned and differ
  from `manifest.version` (e.g. `tekla → 2025.0.1`, `revit-2025 → 2025.0.2.419`). `subdir` is
  `aware-main/20-agents/aeco/<vertical>/<agent>`. Struct: `cli/src/registry/index.rs`
  (`Index{version, updated_at, agents: BTreeMap<String, IndexEntry{versions: BTreeMap<String,
  VersionEntry{tarball, subdir}>}>, bundles}`).
- **Index fetch** `cli/src/registry/fetch.rs`: `fetch_index(cache_dir)` from
  `DEFAULT_REGISTRY_URL = raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json`,
  `AWARE_REGISTRY` override, `file://` support, 1h TTL cache, stale-cache fallback on network error.
- **Agent manifest** `cli/src/manifest/agent.rs` `Agent{agent, version, sdk_target, display_name,
  description, stateful, status(Available|Planned), vendor, license, keywords, transport,
  commands: BTreeMap<String, Command>, skills: Vec<String>}`. `Command{lifecycle(Start|Stop|Single),
  description, method, path, category(Curated|Reflected), mode(Read|Write)}`. Helper methods on the
  manifest: `kind()`, `command_count()`, `skill_count()`, `curated_count()`, `reflected_count()`,
  `category_of(cmd)`. Loader: `manifest::loader::load_agent(path)` + `discover_agents(paths)`.
- **`aware agent` command** `cli/src/commands/agent.rs`: enum `AgentCommand{List, Describe, Skill,
  Install, Uninstall, Update, Validate, Publish}` + `dispatch`. `list`/`describe` read
  `discover_agents` (installed only). `--json` via `envelope::print_ok`; tables via
  `render::table::Table`. `describe` already prints commands (with lifecycle/category/description) +
  skills — the human + JSON shapes to mirror for the catalog-backed describe.
- **63 agents** under `20-agents/aeco/{architecture,construction,cross-cutting,engineering,
  visualization}/<agent>/` (manifest.yaml + skills/ + commands/). Index/catalog are maintained
  manually (no CI regen); `aware agent publish` appends a single index entry.

## Design

### Data — `registry-catalog.json` (repo root, published to `main` beside the index)

Same `agents → versions` keying as the index (so a catalog hit maps 1:1 to an installable
`<id>[@version]`), enriched. Per agent: `display_name`, `vendor`, `keywords`. Per version:
`description`, `status`, `stateful`, `sdk_target`, `transport` (`cli|app|rest|mcp|none`),
`skills: [name]`, `commands: [{name, description, lifecycle, category, mode, method, path}]`.

```jsonc
{
  "version": "1.0",
  "updated-at": "2026-06-01T…Z",
  "agents": {
    "tekla": {
      "display_name": "Tekla Structures",
      "vendor": "trimble",
      "keywords": ["structural", "steel", …],
      "versions": {
        "2025.0.1": {
          "description": "…",
          "status": "available",
          "stateful": true,
          "sdk_target": "2025+",
          "transport": "cli",
          "skills": ["drawing-identity", …],
          "commands": [
            { "name": "run-macro", "description": "…", "lifecycle": "single",
              "category": "curated", "mode": "write", "method": null, "path": null }
          ]
        }
      }
    }
  }
}
```

New structs in `cli/src/registry/catalog.rs`: `Catalog`, `CatalogAgent`, `CatalogVersion`,
`CatalogCommand` (serde, `BTreeMap` for stable ordering), with `Catalog::parse(reader)` and small
query helpers (below).

### Generator — `aware agent reindex` (run in a repo checkout)

Reads `registry-index.json` + each entry's on-disk `manifest.yaml` (map `subdir`
`aware-main/<rel>` → `<repo>/<rel>`), and writes `registry-catalog.json`. **Pure builder**
`build_catalog(index: &Index, load: impl Fn(&str /*subdir*/) -> Result<Agent>) -> Catalog` so it's
unit-testable without disk. The command is the thin IO shell (resolve repo root by walking up for
`registry-index.json`, like `find_registry_root`; load each manifest; write pretty JSON +
`now_iso()`). Catalog is **derived from the index** — it never invents agent/version keys, so it
can't drift from what's installable. A `--check` flag (CI-friendly) regenerates in-memory and
diffs against the on-disk catalog, failing if stale (no write).

### Fetcher — `fetch_catalog(cache_dir)` (mirror `fetch_index`)

`cli/src/registry/fetch.rs`: `DEFAULT_CATALOG_URL =
raw.githubusercontent.com/aware-aeco/aware/main/registry-catalog.json`, `AWARE_CATALOG` override,
`file://`, 1h TTL cache (`<cache_dir>/registry-catalog.json`), stale-cache fallback. If the catalog
is **absent** (older registry / 404), return a typed "catalog unavailable" so the commands print a
clean message ("this AWARE's registry has no catalog yet — update AWARE, or run `aware agent
reindex`") rather than a stack.

### Commands — four new `AgentCommand` variants (all support `--json`)

1. **`aware agent catalog`** — table of **every available** agent from the catalog: `ID`,
   `DISPLAY-NAME`, `VERSION` (latest), `STATUS`, `COMMANDS`, `SKILLS`, `DESCRIPTION` (one line).
   The "list-all + what-each-does". `--installed`-style columns intentionally omitted (that's
   `agent list`).
2. **`aware agent describe <agent> [--available]`** — today `describe` only finds installed agents.
   Change: try installed first; on miss (or with `--available`), fall back to the catalog and print
   the same describe view (description, commands, skills) for a **not-yet-installed** agent, tagged
   "(from registry catalog — not installed; `aware agent install <id>`)". Keeps one `describe`
   surface for both states.
3. **`aware agent search <query> [--capability]`** — rank catalog agents by a query matched across
   `id`, `display_name`, `description`, `keywords`, command `name`+`description`, and `skills`.
   Print ranked hits with the matched field(s) + the best-matching command/skill snippet.
   `--capability` biases scoring to command `name`/`method` (functionality search). Pure
   `score_agent(query, &CatalogAgent, opts) -> Option<Match>` for testability.
4. **`aware agent has <agent> <capability> [--json]`** — scriptable checkpoint: does `<agent>`
   expose a command/method/skill matching `<capability>`? Print the matching command(s)/skill(s);
   **exit 0 if found, nonzero if not** (so scripts/onboarding can gate on it). Reads the catalog
   (works for not-yet-installed agents). Pure `agent_has(&CatalogAgent, capability) -> Vec<Hit>`.

`dispatch` routes the four; `catalog`/`search`/`has` fetch the catalog (cache); `describe` fetches
only on the installed-miss/`--available` path.

## Error handling

- **Catalog absent / unreachable:** typed result → clean guidance message, never a panic/stack.
  Network error with a stale cache → use the cache + a `warning:` line (mirrors `fetch_index`).
- **`reindex` from outside a repo checkout:** clear error (no `registry-index.json` found by walking
  up) telling the user to run inside an aware checkout.
- **`reindex` manifest load failure for one agent:** record + report the failure, skip that entry
  (don't abort the whole catalog), exit nonzero so CI/the author notices — one bad manifest must not
  silently drop from the catalog without a signal.
- **`has` / `search` unknown agent:** `has <unknown>` → clean "not in catalog" + nonzero; `search`
  with no hits → "no agents match" + exit 0 (a search with zero results is not an error).

## Testing

- **Unit (`cargo test`):** `build_catalog` (index × fake manifest loader → expected catalog,
  including the version-key-from-index not manifest.version invariant, and the one-bad-manifest
  skip+signal); `Catalog::parse`; `score_agent`/`search` ranking (name beats description beats
  skill; `--capability` biases to command/method); `agent_has` (found/​not-found, method vs name vs
  skill match); the catalog-absent typed path. Fixtures: a small in-repo catalog JSON + a couple of
  fake manifests.
- **Lint/format:** `cargo fmt --check` + `cargo clippy -- -D warnings` clean.
- **Real end-to-end (no stubs):** run `aware agent reindex` in the worktree → generate the **real
  63-agent** `registry-catalog.json`; then with `AWARE_CATALOG=file://…/registry-catalog.json`
  (and the real cache) run: `aware agent catalog` (assert it lists the real agents incl. tekla),
  `aware agent search "steel connection"` / `--capability "macro"` (assert sensible ranked real
  hits), `aware agent has tekla run-macro` (assert exit 0 + the real command) and a known-absent
  capability (assert nonzero), `aware agent describe rhino-8 --available` (assert real description +
  commands for a not-installed agent). Capture the real CLI output.
- **Dual review** (pr-toolkit:code-reviewer + Codex via codex:rescue) on the diff before "done".

## Shipping / sequencing

Build + branch + open a PR work in any permission mode. **Cutting the `aware` release that ships
the new commands + the committed `registry-catalog.json`** (tag → release.yml → npm publish) needs
the user in **bypassPermissions** mode (repo write-guard). Once released, floless.app's `@latest`
bootstrap (sub-project ③) picks it up and the onboarding skill ① teaches `catalog`/`search`/`has`/
`describe --available`.

## Out of scope (YAGNI)

- Auto-regenerating the catalog in CI / a git hook (manual `reindex` + commit, like the index today;
  the `--check` flag is provided so CI *can* enforce freshness later).
- Rebuilding `registry-index.json` from manifests (the index's version keys are curated; `reindex`
  only derives the catalog FROM the index, never rewrites the index).
- Fuzzy/semantic search or an embedding index (substring/keyword scoring is enough for ~63 agents).
- Per-command full input/output schema in the catalog (name+description+method suffice for search +
  capability-check; `describe`/install fetch the full manifest when needed).
