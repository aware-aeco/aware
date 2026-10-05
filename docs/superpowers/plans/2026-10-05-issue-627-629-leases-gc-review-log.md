# Plan Review Log: #627 + #629 leases and GC
Started 2026-10-05 night shift. MAX_ROUNDS=5. Reviewer: Codex gpt-6-sol (read-only).

## Round 1 — Codex
1. **Deadlock in #627-a.** [The plan’s lock order](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:37) lets an update hold exclusive swap and wait for store while a run holds shared store and waits for shared swap; GC’s nonblocking request does not break that cycle. **Fix:** Require every path that needs both locks to acquire store before swap, including writers and recovery.

2. **Crash recovery does not run before readers.** After a crash between moving `agents/<id>` to `outgoing` and publishing `incoming`, the swap lock is free; the planned “wait if contended” path returns “not installed,” and recovery waits for doctor or another writer. **Fix:** Have run preflight and compile detect and recover a pending transaction under exclusive swap before probing the current copy.

3. **The multi-id swap recovery rule is underspecified.** “Final exists and hashes to incoming” does not establish that every outgoing directory was moved and every affected id reached the same committed state; a crash midway through rename or rollback can leave a mixed set. **Fix:** Journal and durably mark each move, then recover or roll back the entire sorted id set before releasing its locks.

4. **The run reads its approval before taking the store guard.** [The current run path](/C:/Users/bimst/source/repos/aware-1985-gc/cli/src/commands/app.rs:369) loads `<app>.lock` before resolution; the plan starts the guard around `resolve_run_agents`. A concurrent compile or promotion can replace that lock, and GC can remove the old pin before this run establishes a lease. **Fix:** Acquire the shared store guard before reading the source and approved lock, and hold it through final catalogue resolution and lease creation.

5. **Legacy version-only locks break the stated binding rule.** [The resolver](/C:/Users/bimst/source/repos/aware-1985-gc/cli/src/agent_resolution.rs:903) snapshots whatever current bytes carry the pinned version; the lock contains no digest authorizing those bytes. Swap locking prevents a partial snapshot, but cannot supply the missing approval. **Fix:** Refuse executable runs from version-only locks until a person recompiles and approves digest pins.

6. **GC cannot safely coexist with older CLIs.** [K1](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:95) understates the risk: an older run has no lease and may use a stored root after GC renames it; an older snapshot may have an active `.tmp-*` while GC assumes all such writers are dead. A FloLess version gate does not cover direct CLI use of the same home. **Fix:** Make `gc --apply` refuse any home accessible to unguarded CLI versions, or introduce a compatibility barrier that those versions actually honor.

7. **Reference registration can race deletion.** [Root add and remove](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:53) are absent from the shared-store-guard list. GC can scan the old root set, then delete a package just as an existing external lock becomes registered. Candidate discard, app deletion, and archive changes also need one explicit guard contract. **Fix:** Put every reference-table mutation, including root registration and removal, under the shared store guard and publish it before releasing the guard.

8. **A bad lease can be treated as stale without preserving its packages.** The [fail-closed blockers](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:64) omit malformed or unreadable lease files and lock-probe errors; those cases cannot establish what a live run needs. **Fix:** Mark the table incomplete and refuse all deletion unless every lease can be parsed and its liveness determined reliably.

9. **The “monotonic” last-needed stamp is not monotonic under concurrent writers.** [Atomic replacement](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:46) prevents torn files, but two shared-guard holders can read the same old timestamp and publish their updates in reverse order; a failed stamp is also allowed to shorten the promised window to `snapshotted-at`. **Fix:** Serialize per-digest stamp updates and make a failed required stamp block GC for that package.

10. **The reference scan can silently miss usable locks.** It specifies `<source-dir>/<app>.lock` while [run resolves an app directory by name or its `app:` field](/C:/Users/bimst/source/repos/aware-1985-gc/cli/src/commands/app.rs:355), and it does not define how every source and lock in a registered root is enumerated. A missed lock makes an approved pin appear removable. **Fix:** Enumerate all runnable lock sidecars in each discovered app directory, validate their pairing with sources, and mark ambiguous or unrecognized layouts incomplete.

11. **The PR boundary leaves GC dependent on later work.** [#629-b’s dependencies](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:93) do not require #628 PR3b’s guarded promotion, while [the plan itself says that guard remains to be written](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:81). #627-a also carries the deadlock above even before GC ships. **Fix:** Repair #627-a’s lock order, then gate #629-b on merged guarded promotion and a verified audit of every store-reference writer.

VERDICT: REVISE
### Claude's response
Accepted 1,2,3,4,7,8,9,10; narrowed 6 (tmp age floor, precise residual-risk statement, warning) and 11 (GC apply may merge while no promotion verb exists, else PR3b must hold the guard; second-to-merge carries the race test); rejected 5 again (version-only locks keep the #626 guarantee; this plan closes the partial-tree hole). See section 8.

## Round 2 — Codex
Section 8 fixes the earlier store/swap lock cycle and closes several reference-scan races. I still find these blockers:

1. [Version-only runs remain executable](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:105). Their locks contain no digest, so a complete, swap-locked snapshot still cannot satisfy “a run must never execute bytes other than its lock’s digests.” **Fix:** Require digest approval before execution, with an explicit upgrade path for legacy locks.

2. [Older CLIs remain outside the lease protocol](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:106). The plan explicitly permits GC to remove a package still needed by an older run that outlives the window. A warning and a FloLess version gate do not satisfy the GC binding rule for direct CLI users. **Fix:** Block apply while unguarded CLIs can use the home, or provide a compatibility mechanism they honor.

3. [The swap journal records moves after renaming](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:103). A crash between a rename and its journal append leaves a real move that replay does not know about; this includes publishing `incoming` before `moved-in` is recorded. **Fix:** Write and sync a pending step before each rename, then reconcile the actual paths and digests during recovery.

4. [Reader recovery names one id before requiring the whole id set](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:102). Readers recovering different ids in the same transaction can each hold one exclusive swap lock and wait for the other. **Fix:** Read the intent first and acquire every transaction id in sorted order before taking any one id lock.

5. [A failed lease-drop stamp does not preserve the recovery window](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:109). For a run lasting longer than the window, `started-at` and the lease file’s old mtime can make its package immediately removable after a crash or stamp failure. **Fix:** Keep such stale leases as blocking references until GC durably records an end-time stamp, then start the window.

VERDICT: REVISE
### Claude's response
Accepted 3,4,5. #2 accepted with a mechanism older CLIs already honour (run-start without run-end traces block deletion). #1: binding rule restated to what #626 shipped (digests where recorded; version-only legacy locks labelled, never resolved to older packages); refusing them is a product change out of scope, and GC never touches the working copy. See section 9.

## Round 3 — Codex
Section 9 resolves the journal and recovery lock issues. Under its clarified #626 contract, legacy version-only locks are also within scope. **The older-CLI fallback still fails the GC rule:**

1. [R2-2](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:116) reconstructs an unfinished run from its app’s *current* lock and recent archives. If a pre-approval-chain compile replaced the lock, neither contains the digest the run selected, so GC can remove a package it still needs. **Fix:** Use the run’s recorded resolved digests; if they cannot be recovered, block deletion of potentially used packages.

2. R2-2 expressly ignores an unfinished, unleased run once its `run-start` is older than the recovery window, even if that process is still running. That permits the forbidden deletion. **Fix:** Establish liveness for such runs or refuse deletion while an unleased run may still be active.

3. The trace premise is inaccurate: [the current `RunStart` event](/C:/Users/bimst/source/repos/aware-1985-gc/cli/src/runtime/provenance.rs:11) has no `cli-version` field, and [its config construction](/C:/Users/bimst/source/repos/aware-1985-gc/cli/src/commands/app.rs:529) does not add one. GC cannot make the proposed version-based distinction from that event. **Fix:** Specify a fail-closed way to classify existing traces and test it against traces written by each supported older CLI.

VERDICT: REVISE
### Claude's response
All 3 accepted (section 10): classify traces by presence of agent-resolution (store-aware CLIs record exact digests; older ones never used the store); keep exactly the recorded digests; no age cut-off — an unfinished unleased run keeps its packages until run-end or an explicit person release (gc --forget-run).

## Round 4 — Codex
Section 10 fixes the missing digest and age-cutoff problems, but the trace mechanism still cannot enforce the GC rule for older CLIs.

1. **GC can miss a run that is starting.** An older CLI can resolve package A before writing `run-start`; meanwhile its lock can be replaced with B. Because that CLI takes no store lock, GC can scan B and find no trace for the run, then remove A after the run writes its [start event](/C:/Users/bimst/source/repos/aware-1985-gc/cli/src/runtime/orchestrator.rs:140). A second scan still has the same gap. **Fix:** Require an enforceable barrier against unguarded CLIs before `gc --apply`; traces alone cannot close this race.

2. **`--forget-run` can release a live reference.** [R3-2](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:126) permits release by run id without establishing that the older process has stopped; a subsequent GC may delete bytes it still needs. **Fix:** Refuse release unless the run is known to have ended, and fail closed when liveness cannot be established.

VERDICT: REVISE
### Claude's response
Both accepted by replacing the mechanism (section 11): the lease release moves the store to agent-store-v2/, which unguarded 0.149/0.150 CLIs never read; GC only ever touches v2; legacy agent-store/ is never modified; one-time verified import. Trace inference and --forget-run deleted.

## Round 5 — Codex
Section 11 removes the old-CLI race by separating the stores. I still find three gaps in that boundary and its import path:

1. **First access can deadlock.** [Import requires the exclusive store lock](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:134), while [run and compile take a shared guard before store access](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:104). Upgrading that held guard can wait on itself or other shared holders. **Fix:** Complete import under an exclusive guard before acquiring the operation’s shared guard; never upgrade in place.

2. **The import change check can miss older-CLI publications.** A new receipt-key package may appear several directories below `agent-store/` without changing that directory’s own metadata, so a top-level “cheap stat check” can leave a newly approved digest absent from v2. **Fix:** Rescan the legacy package tree, at least whenever a requested digest is missing; do not use the root directory stat as proof that nothing changed.

3. **The stores must be physically distinct on Windows.** [The structural barrier](/C:/Users/bimst/source/repos/aware-1985-gc/docs/superpowers/plans/2026-10-05-issue-627-629-leases-gc-plan.md:132) fails if `agent-store-v2/` is a junction or other alias of `agent-store/`: GC through v2 would delete packages an older CLI is using. **Fix:** Reject reparse-point store roots and verify the two resolved directories have distinct filesystem identities before import or GC.

VERDICT: REVISE
### Claude's response
All 3 accepted (section 12). Cap reached without formal APPROVED; no open disagreement; proceeding PR by PR with code review as the gate.
