# Changelog

## 0.3.0 — 2026-11-12

- `check-snapshots` gains clash: three pair rules over the solids of what a change touched and their dependency closure, as data in the check profile. `clash-overlap` (parts that pass into each other, with the thickness of the shared region measured region by region), `clash-contact` (parts that touch with no bolt or weld joining them) and `clash-bolted-plies` (a ply a bolt group names that touches no ply of the rest of its stack).
- New optional inputs `base-geometry-dir` / `revised-geometry-dir` (`geometry.json` plus GLB tiles, every sha256 verified and echoed in `inputs.geometry`); a clash profile without them is refused. New check profile block `geometry` (node mapping, tolerances, coverage qualification, bolt and weld join properties, groups, budgets); the focus file gains `sets`.
- A solid that is not fully evaluated is labelled (`approximate`, `no-geometry`, `over-budget`) and a pair involving it is `not-evaluated`, never `passed`. Output gains `focusSets`, `scope.notEvaluated`, `scope.solids` and `truncated.omitted.focusSets` when a clash rule ran. A profile without a clash rule produces the same bytes as 0.2.0.
- Uses manifold-3d 3.5.4 (Apache-2.0, WASM, single-threaded), bundled inside the connection-reader exe. Needs connection-reader bridge 0.5.8 or newer.

## 0.2.0 — 2026-11-11

- New command `check-snapshots`: runs a pinned set of data-defined checks (`aware.model-check-profile/v1`) over the objects a change touched and reports `aware.model-change-findings/v1`: findings with a recomputable `correspondenceKey` and `evidenceFingerprint`, each marked `introduced`, `pre-existing`, `pre-existing-changed` (or `-worsened` / `-improved` for a rule with a numeric metric), the findings the change resolved, and a per-identity result for objects named for a re-check (`finding`, `passed`, `not-applicable`, `excluded`, `absent`).
- Scope is every added and changed object from the full pairing (not the capped change rows) plus the named identities; everything else is stated as not checked. Pairing is the shared code of `compare-snapshots`. Counts are complete, lists are capped by the check profile and the cap is stated. Output is canonical JSON, byte-identical for the same inputs.
- First rule shape: `blank` on `entity.name`. An unknown predicate or target in a check profile is rejected.
- Needs connection-reader bridge 0.5.7 or newer.

## 0.1.0 — 2026-11-09

- New agent with one command, `compare-snapshots`: pairs the objects of two saved states of one model by a native identity the caller pins and reports added, removed and changed objects as `aware.model-change-set/v1`. Inputs are each state's `entities.json`, `properties.json` and `relationships.json`, a strict comparison profile (`aware.model-compare-profile/v1`) and an optional witness change log.
- Identity-only pairing; unpaired objects are grouped by type and name for presentation, never paired. Numeric ids never decide a change. Blank and duplicate identities are excluded with receipts, a low paired share is flagged as `discontinuity`, and placement on bounding boxes is always labelled as limited in `unchecked`.
- Counts are complete; rows, group ids and receipts are capped to profile budgets in a pinned order and reconcile. Output is canonical JSON, byte-identical for the same inputs.
- Split from `model-reference-reader` so an exact `model-reference-reader` pin and change review can share one AWARE home. Runs on connection-reader bridge 0.5.6 or newer.
