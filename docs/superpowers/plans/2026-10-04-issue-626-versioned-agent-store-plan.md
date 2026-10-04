# Plan — #626 Versioned agent store: keep agent versions side by side, dispatch from the lock's pin

_Round 3 revision (after Codex rounds 1–3). Part of pawellisowski/floless.app#1985 (owner decision 2026-10-04). Follow-ups: #627 (atomic installs + run leases), #628 (migration machinery), #629 (reference table + GC)._

## Goal (this PR)

Updating an agent must stop breaking approved apps. After `aware agent update X`, an app whose `.lock` pins the old X keeps running **on the exact bytes it approved** with no recompile; a freshly compiled app pins the new X. A front door can ask AWARE, with the same resolver the run uses, whether an app is runnable.

## Non-goals (deferred, each has its own issue)

- Leases that make removal wait for a running app (#627). This PR **adds no deletion of store packages at all**, so there is nothing for a lease to protect yet.
- Migration / candidate locks / successor records (#628). GC (#629) — store packages accumulate until then.
- A bridge protocol-compatibility declaration (see Sidecars) — filed as a follow-up.

## Design — immutable content-addressed store; every lock-bound run dispatches from it

`agents/<id>/` stays **the current, mutable working copy** (list/describe/probe/compile/install/skill-builder read and write it exactly as today — ~40 call sites untouched). A new **immutable store** holds verified snapshots, and **every lock-bound run resolves every pinned agent to a store snapshot** — current version included — so the bytes checked at preflight are bytes nothing in AWARE ever rewrites (fixes round-1 #1 for current agents too, without a lease).

### On-disk layout

```
<AWARE_HOME>/agents/<id>/                          # current working copy — unchanged
<AWARE_HOME>/agent-store/<id>/<tree-hex>/<receipt-key>/   # immutable snapshot
    manifest.yaml, …, .aware-install.yaml          # copied tree, receipt included (if the working copy had one)
    .aware-package.yaml                            # { agent, version, digest, receipt-key, snapshotted-at }
<AWARE_HOME>/agent-store/<id>/<tree-hex>/.tmp-<random>/   # in-progress snapshot; ignored by every reader
```

- `<tree-hex>` = the full `tree_digest` (64 hex). `<receipt-key>` = sha256 hex of the receipt file bytes, or the literal `no-receipt` (fixes round-3 #1): identical bytes installed from an official registry and from a local folder are two snapshots with the same tree digest and different provenance, never one directory whose receipt depends on install order.
- `tree_digest` excludes `.aware-package.yaml` exactly as it excludes the receipt, so a snapshot's digest equals the working copy it came from.
- **Receipt choice for a digest** (deterministic, fails safe): when a run resolves digest R, it uses the snapshot whose receipt-key equals the current working copy's receipt if the current copy hashes to R; otherwise a **total order** over the valid candidates (round-5 #2): (a) receipt claims `InstallSource::Registry` with `official-source: true`, then (b) any other `Registry` receipt, then (c) `Local`, then (d) `no-receipt`; ties broken by receipt-key ascending. Selection reads stored receipts only — never fetches the index (round-4 #4), so ordinary runs stay offline exactly as today. Under `--require-verified-agents` (the only path that already fetches a fresh index), the existing assessment is run over the candidates **in that order** and the first that verifies is selected; if none verifies, the existing strict refusal applies. Bytes are identical in every case — only the provenance label can differ, and the label is whatever that existing assessment says.
- Store dirs are created only by **snapshot** (below) and never modified or deleted by this PR's code. `uninstall` leaves the store alone (removal is GC, #629, which will need #627's leases).

### Snapshot (the only writer) — crash-safe, same-directory rename

`agent_store::snapshot(paths, current_root) -> Result<StoredPackage>`:
1. Load the working copy's manifest; compute `tree_digest(current_root)` = D.
0. Validate inputs before forming any store path: `id` passes `is_safe_segment`; a digest is exactly `sha256:` + 64 lowercase hex (the hex is the dir name). Applies to every store join (snapshot, resolver, `app check`, `agent list`).
1b. K = receipt-key of the working copy (sha256 hex of `.aware-install.yaml`, or `no-receipt`). The package path is `P = agent-store/<id>/<D-hex>/<K>/`; `agent-store/<id>/<D-hex>/` is only a container (created with `create_dir_all`, never verified as a package).
2. If `P` exists: **verify it** (fresh `tree_digest(P) == D`; `.aware-package.yaml` agent/version/digest/receipt-key == manifest agent/version/D/K; receipt bytes hash to K). Valid → return it. Invalid → **refuse** with an error naming `P`. No AWARE code ever renames or deletes a store directory; repair is GC's job (#629, under #627's leases).
3. Copy the tree into `agent-store/<id>/<D-hex>/.tmp-<random>/` (temp dir **inside the digest container**, so the final rename is same-directory — no cross-volume or junction case between `cache/` and the store), write `.aware-package.yaml`, `fsync` files.
4. Re-hash the temp copy; its tree digest must equal D **and** the sha256 of its copied `.aware-install.yaml` (or its absence) must equal K (round-5 #3 — the tree digest excludes the receipt, so the key is checked separately). Either differs → the working copy changed during the copy: delete our temp, retry once from step 1, else refuse with a clear error.
5. `rename(<D-hex>/.tmp-…, <D-hex>/<K>)`. If the rename fails because the target appeared concurrently, verify that one as in step 2 and use it; delete only our own temp dir.
6. Durability (fixes round-3 #5): on Unix, `fsync` the snapshot dir, the `<tree-hex>` dir and the `<id>` dir after the rename; on Windows, the rename uses `MoveFileExW(MOVEFILE_WRITE_THROUGH)` (directory handles can't be flushed portably). Update/install promotion happens only after step 6 returns.
- A crash at any point leaves at most a `.tmp-*` dir (ignored, removable by GC later) and never touches `agents/<id>` or an existing snapshot.
- Power-loss worst case (Windows metadata not yet durable): the promoted new working copy survives but the outgoing snapshot does not. The resolver then finds no package for the lock's digest and refuses with `pin-not-installed` — a safe "compile again", **never** a run on other bytes. Determinism holds; only availability of the old version is lost, which is exactly today's behaviour on every update.
- Threat model: the store protects approved bytes from **AWARE's own writers** (install, update, skill-builder, build, compile), which is what broke approvals before. A person or process hand-editing files under `AWARE_HOME/agent-store` is out of scope, exactly as hand-editing `bridges/` or `aware.exe` is today; such an edit is caught by the next run's preflight digest check.

**When snapshots are taken**
- `aware agent install` / `update` (registry and local): snapshot the **staged** tree **before promotion**, so a snapshot failure refuses the operation with nothing installed or replaced.
- `aware agent update`, **before** removing anything: snapshot and verify **every** directory the swap will remove — `agents/<id>` and, for a dotted-id rename (`registry.rs:512-519`), `agents/<new_name>` too. Any failure refuses the update before the first removal. These are the only changes to update's existing stage → swap sequence.
- `aware app compile`: snapshot every agent it pins **first**, then compile **from the verified stored manifests** — the same snapshot supplies `agent-pins`, `agent-digests` and every compiled node detail (mode, output schema, notes), so a lock never mixes two views of an agent.
- `aware app run` preflight: if a pinned *current* package has no snapshot yet (pre-upgrade install, compile by an older AWARE), snapshot it then.

### Pins — digest for every agent

- Compile writes a new map `agent-digests: { id: "sha256:<D>" }` for **every** pinned agent, whatever its source (registry, local, built, edited). Existing `agent-pins` (version) and `agent-bundle-pins` (official verified digest) keep their meaning and are still written.
- A lock **without** `agent-digests` (compiled by AWARE ≤ 0.148) can only run on the **current** working copy, with today's checks unchanged (version, plus bundle digest when present) — it never resolves to an older store package by version string (fixes round-1 #2). The refusal tells the person to compile again. Exception: if it has an `agent-bundle-pins` digest (official registry installs — e.g. every Tekla install), that digest **is** a byte identity and resolves exactly like `agent-digests`. This is what keeps every existing FloLess-approved Tekla workflow running after its first update.

### Resolution — one resolver, one resolved catalogue, used by every run-path read

`app_lock::resolve_agents(paths, app, lock) -> Result<ResolvedCatalogue, AwareError>`:

For each id in `dispatchable_agents(app)`:
- Lock consistency first: if a lock carries both `agent-digests[id]` and `agent-bundle-pins[id]` and they differ, or any digest string is malformed, refuse the run (`E_APP_LOCK_INVALID`) before resolving anything.
- Required digest R = `agent-digests[id]` else `agent-bundle-pins[id]` else none.
- **R present:** candidates = the receipt-key package dirs `agent-store/<id>/<R-hex>/<K>/` (round-5 #1; `<R-hex>/` is only a container, `.tmp-*` ignored). A candidate is valid only if: fresh `tree_digest == R`, its `manifest.yaml` loads, manifest `agent == id`, manifest `version == agent-pins[id]`, its receipt bytes (or absence) hash to `K`, and `.aware-package.yaml` agrees with agent/version/R/K (fixes round-1 #9). Pick one valid candidate by the receipt-choice order above; invalid candidates are skipped (and reported in `app check`). If no such snapshot but the current working copy hashes to R → snapshot it now, then accept the snapshot. Otherwise refuse:
  - no package with R anywhere → `E_APP_LOCK_AGENT_PIN_MISMATCH` "the approved version of X (0.1.4) is no longer installed; compile the app again to use 0.1.6";
  - a package claims R but fails verification → `E_APP_LOCK_AGENT_BUNDLE_PIN_MISMATCH` (never falls back to another package).
- **R absent (old lock, unofficial install) — a stated, narrower guarantee (round-3 #2):** today's behaviour, unchanged — the current working copy must match `agent-pins[id]` by version; it is snapshotted and the run dispatches the snapshot (immutable for the run). Such a lock never approved bytes, only a version, so AWARE does not claim byte identity for it: `app check` reports `resolution: "current-version-only"`, and the run's provenance record marks the agent `approval: version-only`. It is not refused — refusing would break every existing locally-installed workflow on the AWARE upgrade with nothing to act on; recompiling (which writes `agent-digests`) upgrades it to a byte approval. It never resolves to an older store package.
- Current working copy missing (agent uninstalled) → today's missing-agent refusal, regardless of the store: uninstall means the person removed the tool.
- An installed agent absent from the lock → today's "never approved" refusal.

`ResolvedCatalogue` = `Vec<DiscoveredAgent>` whose `root` is the **store snapshot** dir, plus `fn root(id)` / `fn manifest(id)`. Built **once** at `app run` preflight and passed — instead of a fresh `discover_agents` — to every run preflight and every dispatch read (fixes round-1 #3, #4):

- `commands/app.rs` run path: `reachable_agent_ids`, per-agent provenance assessment, `validate_app_agents`, `missing_agents`, `unsatisfied_pins`, `validate_app_safety`, long-running detection (`:547`), private REST header (`:360`, `:382`), `app_uses_model_reader`, `nested_malformed_requires`.
- `Orchestrator` / `DispatchInvoker`: hold `Arc<ResolvedCatalogue>`; keep `agents_dir` only for the `parent()` = AWARE_HOME derivations (credentials). Every `load_agent_by_id(agents_dir, id)` on the run path → `catalogue.manifest(id)` (no re-read of mutable files): `CliInvoker::spawn_cli` (`invoker.rs:592`), `RestInvoker` method/URL/response-mode/auth/base-url (`:1109-1185`, `:1372`, `:1384`, `:1456`), `transport_kind` (`:3058`), `resolve_exposed` (`:3099`), orchestrator mode/output/stateful/trace (`:675-692`, `:751`, `:891`, `:914`, `:1001`), `artifact_retention::classify_nodes` (`:200`).
- Helpers that take `agents_dir` today get the resolved root/manifest instead: `runtime/google_mail.rs:211-220`, `runtime/trimble_files.rs:54-65`, `render/blender.rs:310-315` (scripts read from the resolved root), `installed_app_routes_to_gmail` (`orchestrator.rs:1874`).
- **Nested apps** (fixes round-1 #5): `resolve_exposed` runs `resolve_agents` on the backing app's own lock and returns `(backing app, its ResolvedCatalogue)`; the nested invoker **and** nested orchestrator are constructed from that catalogue, never from the parent's. Resolution happens **once**, at preflight (round-3 #3): `reachable_agent_ids` resolves each backing app (approved `App` + its lock + its `ResolvedCatalogue`) and stores them in the parent `ResolvedCatalogue` keyed by the app-backed agent id; `resolve_exposed` takes that pre-resolved entry and never re-reads the backing app or its lock, so the provenance and model-reader fencing decisions made at preflight are the ones dispatch runs under.
- **Simulation** (round-3 #4): `--simulate` dispatches no agent and must keep tolerating missing/different agents. It builds no `ResolvedCatalogue`; its preflights (incl. `nested_malformed_requires`) keep today's current-catalogue lookups with today's missing-agent tolerance. The run-path guard test exempts exactly the simulate branch.
- A guard test (source scan, like `agent_id_joins_are_fenced.rs`) fails if a file under `runtime/` or the run path of `commands/app.rs` calls `load_agent_by_id` / `discover_agents*` — with a negative control proving it trips.

### Sidecars / host bridges

Bridges live in `<AWARE_HOME>/bridges/`, versioned by the **CLI** version, not the agent version. An older agent already runs on whatever bridge the current CLI installed whenever someone updates AWARE but not the agent — that is the status quo. Dispatching a stored 0.1.5 Tekla agent on the current bridge is that same situation, no new risk. A real compatibility declaration needs a bridge protocol version that doesn't exist yet; filed as a follow-up issue.

### Front-door contract (fixes round-1 #10) — a check that uses the run's own resolver

New verb `aware app check <app> --json` (read-only, no dispatch, no snapshot writes). It answers **only** the approval / compile-drift question — would `app run` refuse with an `E_APP_LOCK_*` code — not whether every other run preflight (requirements, status, safety, `--require-verified-agents`, host availability) passes; those report at run time as today:
```json
{ "ok": true, "data": {
  "app": "tekla-bom", "approval-current": true,
  "approval-kind": "bytes" | "version-only",  // weakest kind across agents; null when approval-current is false
  "source-current": true,                     // .flo hash == lock source-hash
  "lock": "valid" | "missing" | "invalid",
  "agents": [ { "agent": "tekla", "pinned-version": "0.1.5", "pinned-digest": "sha256:…",
                "installed-version": "0.1.6",
                "resolution": "stored" | "current" | "current-version-only" | "missing" | "pin-not-installed" | "digest-mismatch" | "never-approved" | "legacy-pin-mismatch",
                "detail": "plain sentence" } ] } }
```
- `approval-current` is true iff the lock is valid and consistent, the source hash matches, and every agent resolves to `current`, `stored` or `current-version-only` (round-4 #2 — a legacy lock that runs is approval-current; its weaker kind is in `approval-kind`). Computed by the same `resolve_agents` minus the snapshot side effect (a missing snapshot of a matching current copy reports `current`).
- **Every expected drift is data, never an error** (round-4 #3): a missing/invalid/inconsistent lock, a stale source hash, and every agent `resolution` come back as `ok:true` with `approval-current:false` and the per-agent/`lock`/`source-current` fields saying why. `ok:false` (JSON envelope with a code) is reserved for failures that prevent the check itself: unknown app, unreadable app source, AWARE_HOME unreadable. Never prints outside the envelope (#622's bypass is not repeated).
- `aware agent list --json` rows also gain `stored: [{version, digest}]` for display only; the decision is `app check`.
- **FloLess change** (separate floless PR, child of #1985): `server/agent-pin-drift.ts` asks `aware app check --json` through `aware-adapter.ts` when the CLI supports it, and shows "Needs compile" only when `approval-current` is false (every other run failure stays a run-time outcome, as today); on older CLIs it keeps today's comparison. Contract test in floless against a recorded `app check` fixture for each `resolution` value.

### Spec

- `10-core/agent-spec.md` §Install: the store, snapshot rules, immutability, uninstall leaves the store, `.aware-package.yaml`.
- `10-core/app-spec.md`: `agent-digests` lock field; resolution order; what each refusal means now; `aware app check`.

## Tests (TDD)

Unit:
1. Snapshot: input validation rejects `..` ids and non-hex digests; creates `<D>` with metadata; idempotent; a corrupt existing `<D>` makes the snapshot refuse and is left untouched; a working copy changed mid-copy is detected (digest re-check); crash simulation — a leftover `.tmp-*` is ignored by the resolver.
2. `tree_digest` ignores `.aware-package.yaml`.
3. Resolver: digest lock → stored after update; current matches → snapshot made and dispatched from store; stored package tampered → `E_APP_LOCK_AGENT_BUNDLE_PIN_MISMATCH`, no fallback; manifest identity/version mismatch vs metadata → refuse; old lock with no digest and current ≠ version → refuse (never resolves by version); old lock with `agent-bundle-pins` → resolves stored; uninstalled → missing-agent refusal.
4. Update: staged copy and every outgoing copy (incl. the dotted-id second directory) snapshotted before any removal; a failure injected at each snapshot step aborts with `agents/*` byte-identical. Install: staged-snapshot failure → nothing installed.
4b. Compile: pins and node details come from the stored manifest (a working copy edited between snapshot and compile cannot leak in). A lock whose `agent-digests` and `agent-bundle-pins` disagree → `E_APP_LOCK_INVALID`.
5. Compile writes `agent-digests` for registry, local and app-built agents.
6. `app check --json`: every `resolution` value produced from a real fixture home.

Integration (temp AWARE_HOME, real CLI binary, a fixture CLI agent that prints its own version):
7. Install X 1.0.0, compile A, `agent update X` → 1.1.0; `app run A` dispatches 1.0.0 bytes (agent output says 1.0.0); compile B → `app run B` dispatches 1.1.0.
8. App-backed (`exposes-as-agent`) agent whose backing app pins X 1.0.0 → nested dispatch uses 1.0.0 after the update.
9. Editing a file in `agents/X/` after compile: run A still dispatches the snapshot (unchanged bytes); a lock compiled before the edit is unaffected.
10. Guard test with negative control (run-path `load_agent_by_id`).

Real E2E (temp AWARE_HOME, real registry): `aware agent install tekla@2025.0.1` (0.1.5), compile a model-free tekla exec app, `aware agent update tekla` → 0.1.6, `aware app check --json` → `stored`, `aware app run` runs on 0.1.5 through the real bridge with no recompile.

## Risks / questions for the reviewer

- R1 Disk: one stored copy per distinct version an app was compiled against or that was ever current — bounded per update; GC is #629.
- R2 Run-start cost: `tree_digest` of each pinned store package per run (today the bundle-pinned ones are already hashed per run). Reflected agents with large manifests make this a few ms–100s of ms; acceptable, can be cached by (path, mtime) later.
- R3 Old locks without any digest on an updated agent still need a compile once — honest, and the `app check` resolution `legacy-pin-mismatch` says so.
- R4 Uninstall leaves store packages: they are unreachable by runs (current missing → refusal) and are GC's to remove.
