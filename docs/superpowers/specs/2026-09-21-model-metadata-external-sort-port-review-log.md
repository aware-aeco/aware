# Plan Review Log: Port the bounded metadata external sorter onto current AWARE main

Started 2026-09-21 Europe/Warsaw. MAX_ROUNDS=5.

## Round 1 — Codex

1. Resource limits are checked only after full canonicalization, so a huge/deep object can exhaust
   memory or stack before `recordBytes`/`totalBytes` fires. Fix: use bounded, depth-aware preflight or
   encoding and stress-test oversized/deep inputs.
2. Run count/inode usage is unbounded when `runBytes` is tiny. Fix: add `maxRuns` or a meaningful
   minimum run size and test refusal before excessive files are created.
3. Windows cleanup races `stream.destroy()` because close is not awaited. Fix: await confirmed closure
   before deleting runs/root, with bounded handling for documented transient errors.
4. Cleanup failures are discarded despite the absolute cleanup promise. Fix: preserve the primary
   error while attaching/reporting cleanup failure and leaked-root diagnostics.
5. Cancellation is ineffective while a merge read is blocked. Fix: pass the signal to file streams,
   define iterator cancellation, and test abort during a merge read.
6. The 128-level rule changes shared shard behavior without a published schema rule. Fix: establish it
   in the contract or keep shard admission unchanged and enforce it only at the sorter boundary.
7. Promised malformed/write/close/removal fault tests are absent and lack an injection seam. Fix:
   specify a narrow injectable filesystem/run-reader seam and deterministic tests.
8. Sorter tests run only on Ubuntu CI while the Windows job exercises an unrelated packaged harness.
   Fix: run the focused sorter tests in the Windows job or add a dedicated Windows Node job.
9. Calling a test-only source import a production-module E2E is misleading because the sorter is not
   in the SEA bundle. Fix: identify this as a source-only primitive or intentionally package it.
10. The private stash object is absent from AWARE and private material must not be copied there. Fix:
    name/verify artifacts in the private source repository and abort if unavailable.
11. Exact eight-commit preservation conflicts with required port fixes. Fix: define a ninth
    port-hardening commit or amendments and stop claiming identical history.
12. `--base main` may be stale. Fix: review against verified `origin/main`.

VERDICT: REVISE

### Builder response

Accepted all twelve findings. The revised plan keeps current shard semantics, explicitly treats the
sorter as a source-only primitive, adds a ninth hardening commit with pre-allocation bounds/run caps,
awaited Windows cleanup/cancellable reads/non-public cleanup diagnostics, adds deterministic fault
seams and Windows CI execution, clarifies the private FloLess backup location, and reviews against
`origin/main`.

## Round 2 — Codex

1. A separate preflight and canonicalization pass can observe different getter values, letting a
   changing getter bypass bounds. Fix: create a bounded detached snapshot in one traversal and
   canonicalize only it; count the full envelope plus LF.
2. `readline` accumulates before the size check and decodes invalid UTF-8 before strict parsing; it also
   normalizes line endings. Fix: use a bounded raw-byte LF reader that rejects oversize, invalid UTF-8,
   CRLF, and missing final LF, with corruption tests.
3. Awaiting an async iterator's unresolved `next()` or `return()` can indefinitely block cancellation
   and root cleanup. Fix: make input waits abort-aware and iterator finalization bounded; test
   never-settling methods with a real `AbortController`.

VERDICT: REVISE

### Builder response

Accepted all three findings. The plan now requires a one-read detached bounded snapshot, strict raw
byte line framing, abort-raced producer reads, bounded finalization, and explicit tests for each bypass.

## Round 3 — Codex

The revision addresses all three remaining findings with concrete implementation requirements and
regression tests. No further material plan-level blockers remain. Implementation review must verify
the allocation bounds, abort races, and handle-closure guarantees.

VERDICT: APPROVED
