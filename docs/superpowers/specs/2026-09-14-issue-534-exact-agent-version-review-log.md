# Plan Review Log: Bind agent registry releases to their published manifests
Started 2026-09-14 Europe/Warsaw. MAX_ROUNDS=5.

## Round 1 — Codex

Codex returned `VERDICT: REVISE` with nine findings: unsafe path-shaped manifest identities; unbound offline/custom indexes; the existing update swap's wider rollback limits; authoring checks that might permit missing bindings; provenance that did not compare fresh-index semantic bindings; an ambiguous historical viewer commit; migration from the already-wrong catalog; an impossible pre-merge fresh-official network assertion; and missing alias/partial-schema/bulk-path coverage.

### Codex's response

Accepted the identity fence, fail-closed behavior for every trust level, checked-in completeness rule, provenance comparison, exact introducing commit, direct-source migration, split pre/post-merge E2E, and focused matrix for aliases, partial fields, bundles, and update-all. Rejected widening #534 into a complete update transaction redesign: the mismatch checks run before that pre-existing transaction, and this issue's atomicity claim is limited to semantic refusal. The broader rename/rollback behavior is not required to close the reproduced wrong-version success.

## Round 2 — Codex

Codex returned `VERDICT: REVISE` with two remaining findings: `alias-of` itself could disagree with the bound payload agent, and the planned failure assertions did not require actionable field-level diagnostics.

### Codex's response

Accepted both. The plan now binds an alias target to every release's `manifest-agent`, adds a negative alias mismatch case, and requires plain-English errors to identify the registry release, failing field, expected value, and actual payload value.

## Round 3 — Codex

Codex returned `VERDICT: REVISE` with three precision findings: the existing path-segment helper is deliberately platform-dependent, pre-download errors cannot report an unseen payload value, and manifest versions were not yet required to satisfy the agent spec's strict SemVer grammar.

### Codex's response

Accepted all three. The plan now specifies one portable registry identity validator across operating systems, separates diagnostics available before and after extraction, and validates both bound and extracted manifest versions with the shared strict SemVer parser.

## Round 4 — Codex

Codex returned `VERDICT: REVISE` with three remaining gaps: producer/provenance invariants needed named tests, the public registry contract needed specification updates, and the portable ID rule needed a positive allowlist rather than an incomplete denylist.

### Codex's response

Accepted all three. The final plan adds publish, reindex, and provenance regressions; updates both Agent and CLI specs; and defines the exact cross-platform ID allowlist plus reserved-name rules.

## Round 5 — Codex

No remaining material flaws. The final plan addresses the security, compatibility, schema, provenance, migration, diagnostics, and regression-coverage concerns.

`VERDICT: APPROVED`
