# Changelog

## 0.1.5 — 2026-09-12

- Mark the 19 commands the `aware-tekla` bridge does not dispatch as `status: planned`, so an app using `insert`, `part-list`, `report-create`, `uda-get` or any of the others is refused at validate and compile with `E_APP_COMMAND_UNAVAILABLE` instead of compiling clean and then failing at run with `aware-tekla: unknown verb`. No verb's runtime behaviour changed; the contract now says which ones exist.
- **What this does take away, stated plainly:** 10 of the 19 are write-mode, and a write-mode node never reaches the transport under `aware app run --dry-run` — it emits `would-write:` instead. So those 10 could be compiled and dry-run as a preview without ever producing `unknown verb`, and that loop now closes at compile. The preview was of a node that could never really run, which is why refusing is still right, but it is a capability an author had yesterday and does not have today.
- **Two shipped reference apps stop installing as a result, and that is the point.** `30-apps/_examples/detailer-issue-pack.app` (`drawing-issue`, `drawing-export`, `nc-export-phase`, `bolt-list`) and `30-apps/_examples/qa-drawings-to-tekla.app` (`insert`) compose verbs the bridge cannot dispatch. They were already unrunnable — they compiled and died at run — and are now refused up front. Nothing in CI installs the examples, so the suite does not notice.
- Declare `list-instances` and `close`, which the bridge has always dispatched and this manifest never published. `close` carries an explicit `mode: write` — it terminates the host, and the name matches no write-by-convention suffix — so a node calling it must supply a `safety:` block. Its `force` input authorises a force-kill only where the Open API cannot be attached; with the Open API reachable the clean save happens regardless.

## 0.1.4 — 2026-08-19

- Select the member-roll zero frame from the raw axis as `|q|² <= 1e-6·|d|²`, rather than a cancelling `1 - (n·u)²` on the normalized axis, so the bridge and AWARE's canonical `member_frame` never seed the same near-vertical member differently — including at the boundary, where this mirror's reciprocal-multiply normalization and Rust's divide disagreed by an ulp.

## 0.1.3 — 2026-08-18

- Keep every structural-grid elevation on one native Tekla Grid while mapping supported Tekla 2026 two-word labels to deterministic native tokens with transactional receipt warnings and read-only automatic-plane verification.

## 0.1.2 — 2026-08-18

- Materialize finite canonical member roll against Tekla's measured native FRONT frame, verify native rotation plus B-rep orientation before retirement, and add an optional fail-closed expected-model QA guard.

## 0.1.1 — 2026-08-18

- Preserve independent structural-grid axis extents with a verified non-truncating native envelope, exhaustive receipts, and explicit lossy-expansion warnings.
