# Plan Review Log: Prune one report run's AWARE artifacts

Started 2026-09-25. MAX_ROUNDS=5.

## Round 1 — Codex

REVISE. Seven findings: path/lease TOCTOU; coordinator lock does not cover child writers; unsafe pre-existing trace/artifact creation; lease/delete durability and no-trace crash window; unbounded/loose legacy trace parsing; copy/usage races; partial deletion accounting.

### Builder response

Accepted all seven. The revised plan limits interrupted cleanup to recorded in-process REST/builtin report graphs, fences actual CLI/app dispatch, uses shared read/exclusive prune locking, makes creation and deletion durable, uses capability-relative paths, defines a bounded legacy state machine, and defines per-attempt JSON counts with FloLess usage-zero reconciliation.

## Round 2 — Codex

REVISE. The lock file could be replaced while the original writer holds its old inode; cap-std alone did not supply atomic no-follow opens; removing the empty directory had a rename race; node-level command classification and exact fixture proof were unspecified; old completed runs could have detached child writers. The suggested CLI-child crash test contradicted the in-process writer rule.

### Builder response

Accepted. The marker now binds the lease's OS file identity; `cap-fs-ext::open_dir_nofollow` pins each component; the empty directory is retained as a tombstone; all legacy runs refuse; graph classification is at node-command granularity; the crash test kills AWARE while the REST spool is active. This is intentionally narrower than generic artifact cleanup.

## Round 3 — Codex

APPROVED. The final plan retains the empty directory as a durable tombstone and excludes every legacy run, including completed ones. Implementation follows this narrower contract; verified AWARE capability gap is tracked in issue #576.
