# BCF 2.1 vs 3.0 — what changed

## Default

This agent defaults to **BCF 2.1** for compatibility. Set `target-version: 3.0` in `convert` to upgrade. Downstream tool support for 3.0 is still patchy as of 2026; default to 2.1 unless you've confirmed every consumer in the chain supports 3.0.

## Field-level differences

| Field | 2.1 | 3.0 |
|---|---|---|
| Topic.Stage | not present | new — explicit lifecycle stage |
| Topic.Labels | string array | string array (unchanged) |
| Topic.ReferenceLinks | string array | array of objects |
| Markup.Header | per-topic | promoted to schema-level |
| Comment.ModifiedDate | optional | required |
| Viewpoint.Components.Coloring | hex colour | hex colour with alpha |
| Viewpoint.PerspectiveCamera | only one per viewpoint | unchanged |
| Viewpoint.OrthogonalCamera | only one per viewpoint | unchanged |
| Extension schemas | limited | extensible (custom topic types) |

## Conversion behaviour

### 2.1 → 3.0

- Topic `Labels` → kept as labels
- `ReferenceLinks` upgraded from strings to `{ Url, Description }` objects with `Description` empty
- `ModifiedDate` populated from `CreationDate` where absent (best-effort)
- `Stage` not synthesised — left empty (user fills in via downstream tool)

### 3.0 → 2.1

- `Stage` dropped (no equivalent in 2.1) — recorded as a label `Stage:<value>` to preserve the data
- `ReferenceLinks` objects flattened to strings (only the URL kept)
- Extension-schema fields dropped (recorded as warnings)
- Alpha channel of viewpoint colourings dropped to RGB only

## Why 3.0 matters in AECO

The biggest practical wins:

- **Stage** lets you express "this clash is for the *tender* coordination, not the *construction* one" — useful when running multiple BCF round-trips at different RIBA / AIA stages of the project.
- **Extension schemas** let domain-specific tools attach proprietary metadata without breaking interop.

The biggest practical loss if you're forced back to 2.1:
- You can't express stage; you have to fake it with a label.

## Recommendation

Until ACC Issues / BIMcollab / Revizto / Trimble Connect all default to 3.0 (they will, eventually), stay on 2.1 for outbound. Accept 3.0 inbound (the agent reads both).
