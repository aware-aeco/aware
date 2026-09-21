# Plan: Port the bounded metadata external sorter onto current AWARE main

_Approved in round 3 after adversarial review_

## Goal

Land the already-developed, format-neutral metadata external-sort primitive on current AWARE `main`
without reverting or changing the production Revit reader shipped by PR #480. Preserve the original
eight-commit review history, reconcile the sorter only with the current canonical-v2 metadata helpers,
and finish with a focused/full test pass plus an independently reviewed pull request.

## Approach

1. In the separate private `floless.app` repository, preserve the Tekla planning evidence before
   history operations by creating `refs/backup/tekla-reader-planning-2026-09-20-index` at stash index
   commit `ff6de802859752b17508ee360d0e9426be1411a6`. Abort before AWARE history work unless the commit and
   these four paths resolve there: the provider plan, FloLess module plan, format specification, and
   lineage audit named in the handoff. Never copy that private material into AWARE.
2. Work in a fresh branch and worktree created from fetched `origin/main` at `dc705cc04`. Leave the
   dirty root checkout and the old sorter worktree untouched.
3. Cherry-pick only these commits, oldest first: `4bd9d78ed`, `80b9f89b4`, `d4b38b02e`,
   `bcfffc6bd`, `f65c9f2b9`, `26cce5ebb`, `16a0a64a0`, and `23e6691a3`.
4. Reconcile conflicts by retaining current-main behavior everywhere except the sorter's narrowly
   required export of the canonical metadata-record helper. Do not restore any old contract, schema,
   provider, reader, snapshot, or Revit implementation. In particular, keep current shard admission
   unchanged; enforce the sorter's on-disk depth ceiling at the sorter boundary instead of silently
   changing the published shard contract.
5. Add one explicit port-hardening commit after the eight historical commits. It must:
   - traverse each input once into a detached bounded snapshot, evaluating getters once, and
     canonicalize only that snapshot after bounding the complete `{key,record}` envelope plus LF;
   - cap the number of initial and merge runs independently of record and byte totals;
   - replace `readline` with a bounded raw-byte LF reader that rejects an oversized line before
     accumulation, invalid UTF-8, CRLF, empty records, and a missing final LF;
   - pass cancellation into read streams and race async input `next()` against abort, translating
     either path into `reference-cancelled` without awaiting an uncooperative producer forever;
   - bound iterator finalization so a never-settling `return()` cannot delay owned-root cleanup;
   - await confirmed read-handle closure before deleting input runs or the owned root; and
   - preserve the primary error while attaching non-public diagnostics when owned-root cleanup fails.
6. Keep this unit format-neutral and unconnected to the production route. It intentionally lands a
   source-only primitive that is not yet bundled into the SEA executable. Provider-output ingestion,
   production packaging, semantic validation, and deterministic repartitioning are later units.
7. Add focused tests for oversized/deep input before publication, changing getters, complete-envelope
   accounting, maximum run count, duplicate/cancellation cleanup, abort during merge reading,
   oversized/invalid-UTF-8/CRLF/unterminated on-disk lines, never-settling input `next()`/`return()`,
   confirmed handle closure, and cleanup-failure diagnostics. Introduce the smallest internal
   dependency seam needed for deterministic faults without exposing it as a production option.
8. Run the focused sorter/sharder tests, then the full `cli-connection-reader` suite. Run the focused
   sorter tests on Windows locally and add them to the existing Windows connection-reader CI job so
   future Windows handle regressions are covered. Exercise the source module in a standalone Node
   process, forcing multiple runs and proving deterministic order plus explicit successful cleanup.
9. Inspect `origin/main...HEAD` for unrelated changes and verify that current Revit/provider tests stay
   green. Run `codex exec review --base origin/main`, address every finding, and repeat tests/review.
10. Push the branch and open an AWARE pull request. Do not merge or release in this unit.

## Key decisions & tradeoffs

- Preserve the eight commits instead of squashing locally: each later commit documents a distinct
  correction and the user requested the series in order. Current-main reconciliation lands after that
  series, and adversarial-review fixes remain visible as follow-up commits; the final branch therefore
  preserves, but is not identical to, the old history.
- Export the current canonical metadata-record validator so record shape and canonical bytes remain
  shared, but keep the new depth/size preflight sorter-local until a published canonical-v2 contract
  explicitly adopts those limits.
- Do not wire the sorter into the current RVT path yet. The sorter is the first format-neutral
  primitive; wiring it before disk-backed semantic validation and deterministic repartitioning would
  create a partially enforced production contract.
- Treat the sort root as returned, caller-owned successful output. On failure/cancellation the sorter
  removes only its owned root; if removal itself fails, preserve the primary error and retain the leaked
  root path only in non-public diagnostics so operators can remediate without exposing it to callers.

## Risks / open questions

- PR #480 may have changed shared canonical-number or JSON admission behavior. The port must consume
  current `canonicalJsonBytes`/`parseJsonStrict` rather than bringing back older contract code.
- The old series imposed a 128-level ceiling in the shared shard helper even though the published shard
  schema has no such rule. The port must not change current shard admission; sorter input and on-disk
  rereads use one sorter-local ceiling and tests pin both sides of that boundary.
- Windows file-handle timing can expose cleanup races. Tests must verify removal after duplicate,
  cancellation, malformed-run, and write failures, and must await stream closure before asserting.
- Bounded encoded output does not by itself bound the caller's already-materialized JavaScript object.
  The sorter guarantees that its one-read detached snapshot and encoding fail at the configured
  depth/byte ceiling without allocating an unbounded second representation; callers remain responsible
  for constructing inputs and uncooperative async producers, but neither may hold cleanup open.

## Out of scope

- Tekla-specific formats, provider code, catalogue discovery, or private FloLess integration.
- Provider protocol-v3 enrollment, source capture, semantic foreign-key/ownership validation,
  deterministic canonical repartitioning, signing, caching, or UI work.
- Any modification to the shipped Revit routing, normalization, schemas, shard admission, or cache
  identity.
- Merging the pull request or cutting an AWARE release.
