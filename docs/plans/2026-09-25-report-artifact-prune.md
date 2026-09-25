# Plan: Prune one report run's AWARE artifacts

## Goal

Give a licensed front door a bounded, exact-run cleanup command for report retention, including an interrupted run, without deleting a live writer's files or following links.

## Contract

`aware app artifact <app> --instance <instance> --run-id <run> --prune --reservation-id <reservation>` accepts no artifact id, output, byte limit, or usage flag. It prints one JSON object `{app,instance,runId,pruned:true,bytes,files}`. A missing artifact directory returns zero and is idempotent. The provenance trace, reservation marker, and run lease remain for audit and replay. Other artifact operations remain unchanged.

## Approach

1. Create a run-specific kernel lease at `<logs>/<app>/<instance>/<run>.artifact-lease` with exclusive file creation. Its durable JSON names the exact app, instance, run and reservation, plus `writerClass`. The writer holds a **shared** lock from before reservation publication through all dispatch/cleanup; artifact copy/usage also take a shared lock; prune requires an exclusive nonblocking lock. Shared readers therefore remain possible during progressive output. New reservation markers record the lease's operating-system file identity (Unix device+inode; Windows volume+file index via Win32). Prune compares the locked handle's identity to this marker, so a replacement lock file cannot prove the original writer exited. If a platform cannot supply a stable identity, interruption pruning refuses.
2. Classify actual report writers conservatively before marker publication: `in-process` only if every dispatchable agent has REST transport or the exact built-in `html-report-stream.render-stream` command, with no app/CLI transport. Pass this policy into dispatch and refuse any CLI/app invocation if a formerly safe manifest changes before use. Other graphs get `external-possible` and interrupted prune refuses, even if their AWARE coordinator has died. The two current FloLess report workflows qualify: their read is REST spooled in AWARE and the renderer is an AWARE builtin; they launch no artifact-writing child.
3. Acquire/construct the lease before reservation publication, sync its contents and newly created hierarchy, then publish the extended immutable reservation marker containing the writer class and lease identity, then exclusively create trace and artifact directory. Refuse any pre-existing trace or artifact target for a new run. Crashes between these steps leave a safe, resumable state: a marker plus matching durable lease suffices even when no trace or artifact directory was created.
4. Prune validates each selector as one portable path component, loads the exact existing reservation marker, and matches app/instance/run/reservation and OS lease identity against the locked lease. It takes the exclusive lock without waiting, refusing live writers and readers. An interrupted or completed run requires a matching `in-process` writer lease; **all legacy runs** lacking the lease identity are refused, even with terminal `RunEnd(ok)`, because an old CLI/app transport could have detached a writer. An `external-possible` run is refused as well. The marker+lease suffice if a crash left no trace or a torn trace; a parseable trace must not contradict the run identity.
5. Use compatible `cap-std` and `cap-fs-ext` versions on Windows, Linux and macOS, anchored at the trusted AWARE home. Open each directory component atomically with `DirExt::open_dir_nofollow` and pin handles. Reject symlinks, Windows reparse points, nested directories and non-regular entries. Perform no path-based recursive deletion. Validate the whole flat artifact directory before mutation; remove entries relative to its pinned open handle. Keep the now-empty artifact directory as a tombstone to avoid the non-atomic `remove_open_dir` step. Sync that still-open directory and its parent before success. A concurrent rename cannot redirect a handle-relative removal outside the pinned run directory. Keep trace, marker, lease and sibling runs.
6. The two real FloLess report graphs are test fixtures: `floless-workspace.read-model-complete` REST source and `html-report-stream.render-stream` builtin, with an inline predicate between. Classify at node/command granularity and descend into live `do:` bodies; use the same effective transport priority as dispatch. A real CLI E2E against a local HTTP REST stream proves both the writer classification and crash recovery. Kill AWARE during the REST spool, not a CLI child graph (which must refuse interrupted prune).
7. JSON `bytes`/`files` mean logical lengths/count removed **in this successful invocation**. On a partial deletion error, return no JSON; retry removes the remainder. FloLess must call `--usage` and observe zero before releasing its durable reserved bytes, so cumulative deletion counts are not used for accounting. Zero on a repeat is idempotent.
8. Update `10-core/cli-spec.md`. Add argument, scope, path/link, live-writer, interrupted, legacy, repeat, reader, crash-boundary, and partial-failure tests. Run fmt/clippy/test. Run real CLI E2E in isolated AWARE_HOME, including a verified test writer process killed after artifacts appear, live prune refusal, post-death success, and a completed legacy trace case.

## Key decisions and tradeoffs

- Retention is a caller policy. AWARE deletes only one run's artifacts and does not choose age or count.
- Reservation ID is mandatory to bind destructive cleanup to a front door's durable journal entry.
- Legacy runs have no writer lease or trustworthy external-child proof and must not be auto-pruned, completed or interrupted.
- The lease and marker persist to prevent a replayed run ID or reservation ID from masquerading as an unowned directory.

## Risks / questions for reviewer

- Ordering of lease, marker, and artifact creation across crash points.
- Windows link/reparse and file-open behavior; capability-relative deletion must stay flat and path-scoped.
- A malformed or partial trace must not be mistaken for writer identity; the marker and lease remain authoritative.
- A run error may leave a `RunEnd(error)`; only an in-process lease-backed cleanup should allow it, and legacy must refuse.

## Out of scope

- Deleting the provenance trace, reservation marker, run lease, or user model files.
- Automatically selecting runs for expiry; FloLess owns retention policy.
