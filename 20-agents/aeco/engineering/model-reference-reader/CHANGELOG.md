# Changelog

## 0.9.0 — 2026-11-09

- New command `compare-snapshots`: pairs the objects of two saved states of one model by a native identity the caller pins, and reports what was added, removed and changed as `aware.model-change-set/v1`. It reads each state's `entities.json`, `properties.json` and `relationships.json` plus a caller-supplied comparison profile (`aware.model-compare-profile/v1`, validated strictly) and an optional witness change log. Pure and format-blind: no provider, host, authorization or source file is involved, and format meaning stays in the profile.
- Identity-only pairing; unpaired objects are grouped by type and name for presentation, never paired. Numeric ids never decide a change (properties are keyed by profile key rules, relationships by `(kind, identity(from), identity(to))`). Blank and duplicate identities are excluded with receipts, a low paired share is flagged as `discontinuity`, and a placement comparison on bounding boxes within a pinned tolerance is always labelled as limited in `unchecked`.
- Counts are complete; change rows, group ids and receipts are capped to profile budgets in a pinned order and reconcile (`summary[status] = emitted + truncated.omitted[status]`). Output is canonical JSON, byte-identical for the same inputs.
- Ships with connection-reader bridge 0.5.6. Snapshot packager identity reports reader 0.9.0 and bridge `aware-connection-reader@0.5.6`. The existing commands are unchanged.

## 0.8.2 — 2026-10-13

- Model-reader command failures retain bounded operator-only diagnostic records keyed by the public diagnostic ID under `AWARE_HOME/model-reader-diagnostics/`. Captured provider stderr stays private; unexpected exceptions remain opaque in run traces.
- Preserve provider evidence for refusals, invalid responses, managed-host failures and timeouts. A staging cleanup failure no longer replaces a primary conversion failure or its diagnostic ID.
- Ships with connection-reader bridge 0.5.5. Snapshot packager identity reports reader 0.8.2 and bridge `aware-connection-reader@0.5.5`.

## 0.8.1 — 2026-10-07

- Managed protocol-2 provider launches can receive one independent host-configured authority directory through `AWARE_MODEL_PROVIDER_AUTHORITY_STORE_DIR`. When supplied, it must be a bounded canonical absolute path exactly matching the requested authority store before either describe or convert launches. Conflicting Windows key aliases fail explicitly.
- Existing minimal environments remain unchanged for protocol 1 and package/v3 providers. No credentials, licence decisions or provider-specific environment overrides are added to AWARE; providers still authenticate independently.
- Ships with connection-reader bridge 0.5.4. Snapshot packager identity reports reader 0.8.1 and bridge `aware-connection-reader@0.5.4`.
