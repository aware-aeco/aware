# Changelog

## 0.1.0 — 2026-11-09

- New agent with one command, `compare-snapshots`: pairs the objects of two saved states of one model by a native identity the caller pins and reports added, removed and changed objects as `aware.model-change-set/v1`. Inputs are each state's `entities.json`, `properties.json` and `relationships.json`, a strict comparison profile (`aware.model-compare-profile/v1`) and an optional witness change log.
- Identity-only pairing; unpaired objects are grouped by type and name for presentation, never paired. Numeric ids never decide a change. Blank and duplicate identities are excluded with receipts, a low paired share is flagged as `discontinuity`, and placement on bounding boxes is always labelled as limited in `unchecked`.
- Counts are complete; rows, group ids and receipts are capped to profile budgets in a pinned order and reconcile. Output is canonical JSON, byte-identical for the same inputs.
- Split from `model-reference-reader` so an exact `model-reference-reader` pin and change review can share one AWARE home. Runs on connection-reader bridge 0.5.6 or newer.
