# Chorus vs Source — which NBS product?

NBS ships two products. Same content library, different surfaces.

| Tool | What it is | Who uses it |
|---|---|---|
| **NBS Source** | Read-only product library — manufacturer datasheets + Uniclass codes + Pset templates | Designers researching products; specifiers gathering options |
| **NBS Chorus** | Cloud spec authoring — write specs, link to models, output PDFs/Word | Specifiers writing the deliverable |

This agent wraps **Chorus** (the authoring side). For the read-only
product-library side, no agent yet — `nbs-source` is queued as a
follow-up when audit demand surfaces.

## Auth

`aware connect nbs-chorus` provisions OAuth via NBS's API portal.
Requires an `nbs-chorus` workspace + API key (Standard tier or higher
— Personal Chorus accounts don't have API access).

For tenant-restricted setups, the device-code flow (v0.13) works:

```bash
aware connect nbs-chorus --device-code
```

## RIBA Stages

UK specs are explicitly mapped to RIBA Plan of Work stages 0-7. Chorus
exposes this via the `stage` field on specs. Use `spec.list --stage 4`
to filter to Stage 4 (Technical Design) specs only — the typical
"production spec" filter.

## Uniclass classification

Chorus is Uniclass-native. Section IDs look like `EF_25_10` for "Wall
construction systems / Brick walls." If you need cross-referencing
to MasterFormat / OmniClass / CAWS, use `csi-masterformat.translate`
(this agent's sibling).

## Rate limits

Chorus's API limits to ~120 requests/min per workspace. The transport
rate-limits at 100/min. Bulk clause updates (e.g. re-applying a master
spec across 100 sections) should batch via `clauses.update` calls —
each takes ~600ms on the wire.
