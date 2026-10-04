# Plan Review Log: #626 versioned agent store
Started 2026-10-04 night shift. MAX_ROUNDS=5. Reviewer: Codex gpt-6-sol (read-only).

## Round 1 — Codex
The plan needs revision before implementation. Its proposed path mapping does not yet make the lock’s approval govern every byte and manifest used by a run.

1. **The determinism gate is incomplete.** A `PathBuf` identifies a directory, not the bytes checked at preflight. A current agent can be updated, or a retained file edited, between `tree_digest` and dispatch; the plan explicitly defers protection for current agents. **Fix:** hold a run lease or dispatch from an immutable snapshot whose bytes were verified, for both current and retained agents.

2. **Version-only locks cannot meet the stated byte guarantee.** A retained local package can change bytes while keeping its version, and the resolver would accept it by version alone. **Fix:** record a bundle digest for every compiled agent regardless of registry trust; require older version-only locks to be recompiled if exact bytes are required.

3. **Run preflights still judge the current catalogue.** After resolution, [app.rs](C:/Users/bimst/source/repos/aware-1985-store/cli/src/commands/app.rs:429) passes current agents to `reachable_agent_ids`, provenance checks, `validate_app_agents`, `missing_agents`, `unsatisfied_pins`, and `validate_app_safety`. An old retained agent can therefore be refused or classified using its replacement. **Fix:** build one resolved `DiscoveredAgent` catalogue and pass it to every run preflight and provenance check, including the backing app’s leaf agents.

4. **The read-path inventory misses concrete dispatch reads.** [RestInvoker](C:/Users/bimst/source/repos/aware-1985-store/cli/src/runtime/invoker.rs:1109) passes `agents_dir` into Gmail and Trimble helpers and rereads manifests for method, URL, response mode, and auth; [Blender](C:/Users/bimst/source/repos/aware-1985-store/cli/src/render/blender.rs:311) has no source parameter at all. Changing only `load_agent_by_id` calls in the listed structs will still mix old and new behavior. **Fix:** pass the resolved manifest and root through every transport and built-in helper, and test REST, Gmail, Trimble, and Blender paths.

5. **Nested app resolution is not carried into execution.** [resolve_exposed](C:/Users/bimst/source/repos/aware-1985-store/cli/src/runtime/invoker.rs:3092) can verify the backing app’s lock, but `nested_leaf` and `run_exposed_app_*` are currently constructed with the parent’s `agents_dir`. **Fix:** return the backing app together with its own resolution and construct both nested invoker and orchestrator from that resolution.

6. **Retention is not an atomic, recoverable transaction.** Moving the old directory, writing metadata, and moving staging leaves a crash state with no current agent; the dotted-ID path can involve two outgoing directories. Rollback on an ordinary error does not handle process termination. **Fix:** specify a journaled recovery procedure for both directories and inject failures at every rename and metadata step.

7. **The existing-destination branch can destroy the only good copy.** On Windows, directory rename does not replace an existing destination. The proposed “final exists, delete outgoing” rule trusts that destination without proving it contains the same package; a stale or corrupt retained directory could win. **Fix:** verify the existing directory’s full digest, manifest identity, and metadata before deduplication, and otherwise refuse without deleting the outgoing copy.

8. **The same-volume assumption is unproved.** `cache/` and `agent-store/` can be junctions or mount points even though their path strings are under `AWARE_HOME`; rename can then fail after the current directory has moved. **Fix:** stage under the verified destination volume and handle Windows sharing violations and rollback explicitly.

9. **Retained identity relies too much on metadata.** The resolver’s stated retained checks do not require the loaded manifest’s agent and version to match both `.aware-retained.yaml` and `agent-pins`; version-only lookup can be redirected by edited metadata. **Fix:** validate the actual manifest identity and version alongside the digest before accepting a retained candidate.

10. **The FloLess contract is only half specified.** Adding `retained` to `agent list --json` does not change FloLess’s `agent-pin-drift.ts`, and a version match alone cannot establish that a digest-pinned lock is runnable. The proposed array also omits corrupt-store and ambiguity behavior. **Fix:** define and test a front-door decision contract against the CLI resolver, including digest matches, ambiguous versions, invalid retained packages, and the FloLess UI change.

VERDICT: REVISE
### Claude's response
Accepted 1-9: redesigned around an immutable content-addressed store (agent-store/<id>/<digest>/) that EVERY lock-bound run dispatches from (current version included), so preflight-verified bytes are never rewritten; snapshot is copy-into-same-dir-temp + verify + rename (no moves of agents/<id>, no cross-volume, corrupt dirs quarantined not deleted); new agent-digests lock field for every agent; old locks only resolve to the store via an official agent-bundle-pins digest, never by version; one ResolvedCatalogue built once and passed to every run preflight and dispatch read incl. REST/Gmail/Trimble/Blender and nested apps; guard test against run-path re-reads; uninstall no longer touches the store (no deletion in this PR, so no lease needed yet). #10: accepted as a new read-only verb aware app check --json driven by the same resolver; FloLess consumes it in a separate PR.

## Round 2 — Codex
The revision fixes the main catalogue and nested-dispatch gaps, but several material problems remain:

1. **A store path still does not freeze bytes for a run.** A user or another process can edit a snapshot after preflight; `snapshot` can also rename a “corrupt” directory while a run is using it. **Fix:** protect resolved packages for the full run and never quarantine a package that may be in use.

2. **The new store joins need explicit input checks.** Agent IDs come from app source and digests come from a lock; the plan no longer specifies an `is_safe_segment` check or strict `sha256:` plus 64-hex validation before forming store paths. **Fix:** validate both values before any lookup, snapshot, rename, or quarantine.

3. **Compile can pin two different views of an agent.** [compile_snapshot](C:/Users/bimst/source/repos/aware-1985-store/cli/src/app_lock.rs:326) builds version and node details from discovered manifests; a later snapshot could capture changed files and supply the new digest. **Fix:** compile from the verified stored manifest and use that same snapshot for every pin and compiled node detail.

4. **The dotted-ID update has two removal targets.** [update_agent_from_registry](C:/Users/bimst/source/repos/aware-1985-store/cli/src/install/registry.rs:512) can delete both `agents/<id>` and `agents/<new_name>`; the plan snapshots only “the outgoing copy.” **Fix:** snapshot and validate every directory the update will remove before removing either one.

5. **A failed post-install snapshot leaves a changed install.** The plan snapshots the incoming copy only after promotion, so an install or update can report failure after replacing the current agent. **Fix:** snapshot staging before promotion, or define and test rollback for a post-promotion failure.

6. **`app check` overstates `runnable`.** Its definition checks source and agent resolution, while `app run` also applies requirements, status, safety, provenance under `--require-verified-agents`, and other preflights. FloLess could suppress “Needs compile” for an app that still cannot run. **Fix:** either run the same non-dispatch preflights in `app check` with the relevant options, or name the result `approval-current` and limit FloLess’s interpretation to compile drift.

7. **The two digest fields can disagree.** Resolution prefers `agent-digests` and would ignore a different `agent-bundle-pins` value in the same lock. **Fix:** reject inconsistent lock digest fields before resolving any agent.

VERDICT: REVISE
### Claude's response (round 2)
Accepted 2–7 as stated: input validation on every store join; compile snapshots first and compiles from the stored manifest; every update removal target (incl. the dotted-id second directory) snapshotted before any removal; the staged tree is snapshotted before promotion; `app check` renamed to `approval-current` and scoped to `E_APP_LOCK_*` drift; inconsistent digest fields refuse with `E_APP_LOCK_INVALID`.
#1 partly accepted: quarantine removed. No AWARE code renames or deletes a store directory, and a corrupt existing snapshot makes the snapshot refuse. Rejected: protecting a run mid-flight against hand edits of `AWARE_HOME/agent-store`. That is outside the threat model, the same as `bridges/` and `aware.exe` today, and the next preflight digest check catches it. Mid-run protection against AWARE's own deleters is #627, and this PR has none.

## Round 3 — Codex
The round-2 changes close several earlier gaps, but I still find material problems:

1. **The store key loses receipt identity.** `tree_digest` excludes `.aware-install.yaml`, so a local and an official install with identical bundle bytes share one store directory. Whichever is snapshotted first supplies the receipt used by later [provenance checks](C:/Users/bimst/source/repos/aware-1985-store/cli/src/install/provenance.rs:99), even when the current install came from the other source. **Fix:** store provenance separately by install identity, or include receipt identity in stored-package selection and test both install orders.

2. **Legacy version-only locks still run unapproved bytes.** If a current local agent is edited without changing its version, the resolver snapshots and runs the edited tree for an old lock with no digest. That conflicts with the plan’s “exact bytes it approved” guarantee. **Fix:** require recompilation for locks without a byte digest, or explicitly narrow the guarantee and report these runs as version-only approval.

3. **Nested preflight and dispatch can resolve different backing apps.** `reachable_agent_ids` resolves the backing app during preflight, while `resolve_exposed` resolves it again at dispatch. A changed backing app can invalidate provenance or model-reader fencing decisions made earlier. **Fix:** carry the approved backing app and its resolved catalogue from preflight into nested dispatch.

4. **The simulation path needs its own rule.** [app run](C:/Users/bimst/source/repos/aware-1985-store/cli/src/commands/app.rs:422) checks nested malformed `requires` before the `!simulate` branch; simulation intentionally works with missing agents. Routing that check through a catalogue that requires installed agents would break it. **Fix:** specify a simulation-safe malformed-requires lookup that preserves missing-agent tolerance without dispatching or resolving pins.

5. **“Crash-safe” promotion lacks directory durability.** Fsyncing copied files does not make the temp directory entry or final rename durable across a power loss; an update could persist while its required snapshot does not. **Fix:** specify durable directory sync or a recovery protocol, including the Windows behavior, before promoting the working copy.

VERDICT: REVISE
### Claude's response (round 3)
- #1 accepted: snapshots are now keyed `<tree-hex>/<receipt-key>/`, so the same bytes installed from different sources keep separate provenance. Receipt choice is deterministic and fails safe: the current copy's receipt if it hashes to R, else an official receipt that verifies, else the lexically first. A label is never upgraded to official without a verifying receipt.
- #2 accepted as narrowing, not refusal: version-only legacy locks keep today's guarantee and are labelled `current-version-only` in `app check` and `approval: version-only` in the provenance record. They never resolve to older store packages, and recompiling upgrades them to a byte approval. Refusing them would break every existing locally-installed workflow on upgrade with nothing for the person to act on (repo rule "nothing refuses, it asks").
- #3 accepted: each backing app and its catalogue is resolved once at preflight and carried into `resolve_exposed`.
- #4 accepted: simulation builds no resolved catalogue and keeps today's lookups and missing-agent tolerance. The guard exempts exactly that branch.
- #5 accepted: Unix dir fsyncs, and on Windows `MoveFileExW(WRITE_THROUGH)`. Promotion happens only after durability returns. The power-loss worst case is documented as a safe `pin-not-installed` refusal, never other bytes.

## Round 4 — Codex
The receipt-key change addresses the provenance collision, but the plan still has material contradictions:

1. **The snapshot algorithm uses the old layout.** Step 2 verifies `<id>/<digest>/` as though it were a package, but it is now a container for receipt-key directories. Step 3 places `.tmp-*` under `<id>/`, while step 5 renames it within `<id>/<digest>/`. **Fix:** create the digest container, stage inside it, and verify only `<id>/<digest>/<receipt-key>/`.

2. **`app check` misclassifies a successful legacy run.** The resolver accepts `current-version-only`, but `approval-current` is defined as true only for `current` or `stored`; FloLess would show “Needs compile” for an app that runs. **Fix:** count `current-version-only` as approval-current and expose its weaker approval type separately.

3. **Failure reporting has incompatible shapes.** The example puts `missing`, `digest-mismatch`, and `never-approved` in `data.agents`, but the plan also says refusals return `ok:false`; FloLess cannot reliably consume the illustrated status array. **Fix:** return structured `approval-current:false` results for expected drift, reserving `ok:false` for failures that prevent the check itself.

4. **Receipt selection can introduce a network dependency into ordinary runs.** Choosing a receipt that “verifies as official against the index” requires a fresh official index under the current [provenance rules](C:/Users/bimst/source/repos/aware-1985-store/cli/src/install/provenance.rs:99), while ordinary runs do not currently require that fetch. The offline selection behavior is unspecified. **Fix:** select a stored package deterministically without a registry fetch; perform fresh-index verification only when strict provenance is requested.

VERDICT: REVISE
### Claude's response (round 4)
All 4 accepted:
- #1: snapshot steps now use the `<D-hex>/` container and verify only the package at `<D-hex>/<K>/`. Temp dirs are staged inside the container.
- #2: `current-version-only` counts as approval-current, and `approval-kind` exposes bytes vs version-only.
- #3: every expected drift is `ok:true` data (`lock`, `source-current`, per-agent `resolution`). `ok:false` is reserved for failures that prevent the check.
- #4: receipt selection is local-only, preferring receipts that claim a registry source. Verification stays with the existing provenance assessment, which fetches the index only under `--require-verified-agents`.

## Round 5 — Codex
The round-4 fixes resolve the four reported issues, but two core paths still need correction:

1. **Resolution still targets the digest container.** It names `agent-store/<id>/<R-hex>/` as the candidate and expects a manifest and `tree_digest` there. Under the revised layout, packages are one level deeper at `<R-hex>/<receipt-key>/`; this would refuse stored agents after an update. **Fix:** select a receipt-key directory first, then verify its manifest, metadata, receipt key, and tree digest.

2. **Receipt selection is ambiguous and can fail strict provenance unnecessarily.** More than one stored receipt can claim `InstallSource::Registry`; the plan gives no tie-break between them. Selecting a custom-registry receipt can make `--require-verified-agents` fail even when the same bytes have an official receipt. **Fix:** define a total ordering for ordinary runs and, for strict runs, select a receipt that passes the fresh-index assessment.

3. **Snapshot verification does not cover the receipt copied into staging.** Step 4 re-hashes the tree, which excludes `.aware-install.yaml`. If that file changes between computing K and copying, the package can be promoted under the wrong receipt key. **Fix:** recompute the staged receipt key before rename and retry or refuse if it differs from K.

VERDICT: REVISE
### Claude's response (round 5)
All 3 accepted and applied:
- #1: the resolver enumerates the receipt-key package dirs under the digest container and verifies each fully.
- #2: total order official-registry > other registry > local > no-receipt, then receipt-key. Under `--require-verified-agents` the order is walked with the existing fresh-index assessment.
- #3: the staged receipt key is re-checked alongside the tree digest before the rename.

## Resolution
MAX_ROUNDS=5 was reached without a `VERDICT: APPROVED`. The plan did NOT formally converge. No disagreement remains open: every finding in every round was accepted (round-2 #1 and round-3 #2 were accepted in narrowed form, with the reasons logged above). Codex's round-5 findings were all concrete defects in round-3/4 edits, and all are applied. Proceeding to implementation under the operator's night-shift pre-authorisation, with the mandatory Codex code review on the PR as the next gate. That review should re-check exactly these store/receipt/resolution paths.
