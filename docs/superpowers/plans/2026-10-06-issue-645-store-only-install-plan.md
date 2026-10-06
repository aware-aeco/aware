# Issue #645 — put an exact approved tool version back into the store without installing it

## Problem

When an approved app pins a tool version whose store package is gone (`aware app check` →
`pin-not-installed`, possibly `removed: {by: gc}`), the only way to get those bytes back is
`aware agent update <id>@<version>`, which also makes that version the INSTALLED working copy.
That changes what every later `aware app compile` resolves, and a compile racing the swap
approves whichever copy is installed at that instant. `aware agent install <id>@<version>`
refuses (`conflict: already installed`).

## Design chosen

`aware agent install <id>@<version> --store-only`

Why a flag on `install` rather than a new verb (`agent fetch`):

- The fetch, release-binding checks, bundle validation, official digest check and receipt
  are exactly `install`'s; a flag makes it obvious the verification is the same, and the
  issue suggested this spelling first.
- `install` already owns the `<id>@<version>` grammar and registry resolution (`Index::resolve`).
- A new top-level verb would need its own help, docs and envelope for one behaviour difference:
  "do not promote to `agents/<id>`".

Rules of the flag:

- Requires an exact `<id>@<version>` (both sides non-empty), where `<id>` is the **registry key**
  and `<version>` the **registry version** — exactly what `agent install` takes (for
  `tekla@2025.0.1` the stored manifest is tekla 0.1.5; for a suffixed release the package is
  stored under the manifest's own id, e.g. `alpha.2024`). Success means "this release is stored";
  whether the approval is satisfied is `app check`'s answer — a non-official registry may serve
  other bytes under the same version, and then the pin stays `pin-not-installed` (the JSON
  reports the stored `digest` so a front door can compare it with the pin first). `--store-only` without a version is
  refused (exit 3, `E_AGENT_STORE_ONLY_NEEDS_VERSION`): the point is to put back one *exact*
  approved version; "whatever is newest" is not that.
- A local folder path or a bundle name with `--store-only` is refused (`E_AGENT_STORE_ONLY_REGISTRY`)
  — out of scope; a folder can be installed or compiled normally.
- Works whether or not the agent is installed, and whatever version is installed.
- `agents/<id>/` is never read, written, renamed or locked. No swap lock is taken, no swap
  transaction is created (`agents/.aware-swap/` is not touched), no host-plugin / diagram
  regeneration runs (nothing installed changed).

## Algorithm (`install::store_agent_from_registry`)

1. Command layer: fetch the index (`fetch_index_for_install`, same as install — this one
   read happens BEFORE the guard, exactly as for install), then `agent_store::open(paths)` →
   shared `RefGuard`, held from the tarball fetch through staging, snapshot and stamp. GC `--apply` takes the store lock exclusive, so it cannot run
   between the snapshot's publish and the stamp, nor delete the package while it is being
   published. (Same lock order as install: store lock first; no swap locks at all.)
2. `index.agents.get(id)` + `index.resolve(id, Some(version))` +
   `validate_release_contract` — identical to `install_agent_from_registry`.
3. `stage_agent_from_registry` — the same fetch (cache → file:// → network), extraction, full
   release validation (`validate_staged_release_for_cache`, including the official digest) into
   a private `tempfile::TempDir` scratch (system temp, outside `agents/`).
4. NEW shared helper `verify_and_receipt(src, staging, trust, key, version, index_entry, release)`
   extracted from `install_staged_registry` so both paths run the *same code*:
   load manifest → `validate_release_payload` → `validate_agent_on_disk` → `copy_dir_recursive(src,
   staging)` → `tree_digest(staging)` → official: digest must equal `bundle-digest` (missing
   digest refuses) → `provenance::write_required(staging, receipt)` with the identical
   `InstallSource::Registry {…, official_source}` fields. Returns `(agent manifest, digest)`.
   `install_staged_registry` calls it with `staged.incoming()`; store-only calls it with
   `scratch/staged` (a fresh dir inside its own TempDir).
5. `agent_store::snapshot(paths, &staging, &guard)` — the unchanged single writer: temp dir
   inside the digest container, re-hash digest + receipt key, one no-replace rename, existing
   package verified and reused (a corrupt existing package refuses, never repaired).
   The returned `StoredPackage.digest` must equal the digest computed in step 4 (it does by
   construction; asserted defensively as `Internal` if not).
6. `stamps::stamp(paths, id, digest, None)` — last-needed = now, so an already-present package
   with an old `snapshotted-at` (or a freshly published one) starts a fresh recovery window:
   GC classifies it `recent` (in-window) for `agent-store.recovery-window` even when the lock
   that pins it is outside every searched root. A stamp failure (revised after plan review R1):
   for a package published BY THIS fetch it is a warning (`stamped: false`) — its fresh
   `snapshotted-at` already starts the window; for an ALREADY-stored package it is an error,
   `E_AGENT_STORE_STAMP` ("stored and verified, but its recovery window could not be
   restarted"), because restarting the window is the point of re-fetching it. Re-running is safe.
7. Drop the scratch, release the guard.

Idempotent: a second run reuses the verified package (snapshot step 2) and only moves the stamp.

The GC tombstone `.removed-<key>.yaml` left in the digest container by an earlier GC is not
touched: `app check`/run consult it only when no verified package exists (resolution order:
verified package first), and `agent refs` already skips tombstone names. Verified by test.

## Output

Plain: `✓ stored <agent> <manifest-version> (<digest>) from <key>@<registry-version>; the installed copy was not changed`
(or `already stored` when the package existed).

`--json` (envelope command `agent install`), data:
`{ "store-only": true, "agent", "version", "registry-key", "registry-version", "digest",
   "receipt-key", "path", "official-source": bool, "already-stored": bool }`.
`already-stored` is decided by probing the package path before the snapshot (under the guard).

## Specs

- `10-core/cli-spec.md`: `aware agent install` section — add `--store-only`.
- `10-core/agent-spec.md` § Installation examples + § agent store "When snapshots are taken" and
  "After a removal": a pin-not-installed version can be put back with `--store-only`; it is kept
  while a found lock pins it, else for the recovery window from the fetch.

## Part 2 (`app compile --expect <agent>@sha256:…`) — deferred to its own issue

It changes the compile/approval seam (a new refusal in `compile_to_disk_for`, a new flag on a
determinism-gate path, a front-door contract), which deserves its own plan and review rather
than riding on a store-writer change. Filed as a separate aware issue, linked from #645.

## Tests (TDD — written first, seen failing)

Unit (`install/registry.rs` tests, local tarball + `Index` fixtures already there):
- U1 store-only puts the exact version into the store, `agents/<id>` byte-identical to before
  (other version installed), `agents/.aware-swap` has no transaction, package verifies, receipt
  is the registry receipt (same receipt key as a normal install of the same release).
- U2 works when the agent is not installed at all; `agents/<id>` still absent.
- U3 official trust + wrong bundle-digest → refused, nothing stored (container empty).
- U4 payload manifest version ≠ `manifest-version` binding → refused, nothing stored.
- U5 already-stored → `already_stored: true`, same package, last-needed stamp moved forward.
- U6 a snapshot fault (inject_fault Copy/Recheck/Rename) → error, nothing stored, agents untouched.
Command-level: `--store-only` without version / with a local dir → refused with the codes above.

Integration (`cli/tests/agent_store_only.rs`, verbot 1.0.0/1.1.0 file-registry fixture):
- I1 install 1.0.0 → compile app `a` under `apps/` → update to 1.1.0 → remove the 1.0.0 store
  package (`rm`, as in the issue repro) → `app check` pin-not-installed → `install verbot@1.0.0
  --store-only` → `app check` stored, installed manifest still 1.1.0, `app run a` prints 1.0.0.
- I2 GC interplay (asserting the restored package's version AND digest): app compiled in a folder NOT under any searched root → update → `agent gc
  --apply --recovery-window 0s` removes 1.0.0 → `app check` pin-not-installed with
  `removed.by == gc` → store-only → `app check` stored (tombstone present, ignored) → `agent gc
  --apply` (default window) keeps it (`recent`) → `agent refs` complete. And with the app under
  `apps/`, `gc --apply --recovery-window 0s` keeps it (approved-lock).
- I3 `--json` envelope fields; second run `already-stored: true`.

Added after plan review R1:
- U7 the shared `ReleaseCheck` driven directly: broken binding refused; official with no digest /
  wrong digest refused with no receipt written (the store-only path also checks these inside
  `stage_agent_from_registry`, so only direct tests can catch the helper's checks being deleted).
- U8 official trust: a normal install and a store-only fetch of the same release produce the SAME
  package path / receipt key, receipt rank 0, and `assess_against_index(..).verified`.
- U9 suffixed manifest id (`alpha` key → `alpha.2024` manifest) stored under `alpha.2024`.
- U10 stamp failure: new package → warning; reused package → `E_AGENT_STORE_STAMP`.
- U11 `agents` is a FILE → store-only still succeeds (proves no swap staging/lock/working copy).
- U12 GC's `RefGuard::exclusive_within` from another thread during the publish → `None`.
- U13 four concurrent store-only fetches of one release → one package, no `.tmp-*` leftover.

Mutation checks by hand: drop the stamp; drop the already-stored stamp refusal; skip the official
digest compare in the helper; skip `validate_release_payload` in the helper; stage via
`swap::Staged`; promote into agents — each must fail a test.

## Risks considered

- **GC racing the fetch**: guard shared for the whole op; GC exclusive cannot interleave.
- **Two concurrent store-only fetches of one version**: snapshot's no-replace rename + verify
  of the winner (existing behaviour).
- **Concurrent install/update of the same id**: store-only never touches `agents/` or swap state;
  snapshots of identical bytes converge on one package.
- **Stale index (non-fresh official)**: receipt says `official-source: false` → a different
  receipt key from an official install; the lock pins digest only, so `app check` says stored;
  `--require-verified-agents` correctly does not accept it. Same as install today.
- **Wrong bytes**: official → digest must match; any trust → release binding + manifest identity
  + full validation; and the run verifies every byte it dispatches against the lock digest, so
  a non-official registry serving different bytes for the version simply does not satisfy the
  pin (`app check` stays pin-not-installed) — reported, never run.
