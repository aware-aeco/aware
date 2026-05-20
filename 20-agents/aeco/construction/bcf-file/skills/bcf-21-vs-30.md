---
name: bcf-21-vs-30
description: This skill should be used when choosing a BCF version to emit or when running the bcf-file convert verb — deciding between BCF 2.1 and 3.0, understanding the field-level differences, and knowing exactly what is lost converting 3.0→2.1. Encodes the version-selection rule (default 2.1 outbound) and the conversion behaviour.
---

# BCF 2.1 vs 3.0 — what changed

## The rule

**Default to BCF 2.1 for anything outbound.** Accept 3.0 inbound (the agent reads both). As of 2026, downstream support for 3.0 is still patchy — ACC Issues, BIMcollab, Revizto, and Trimble Connect do not all default to it. Emit 2.1 unless every consumer in the chain is confirmed to read 3.0. Set `target-version: 3.0` on [`convert`](../commands/convert.md) only when you control the whole chain (e.g. Solibri ↔ Solibri, or Solibri ↔ Newforma).

Both versions are buildingSMART specs; 3.0 (2021) is the successor to 2.1 (2014).

## Field-level differences

| Field | 2.1 | 3.0 |
|---|---|---|
| `Topic.Stage` | not present | new — explicit project lifecycle stage |
| `Topic.Labels` | string array | string array (unchanged) |
| `Topic.ReferenceLinks` | string array | array of `{ Url, Description }` objects |
| `Topic.ServerAssignedId` | not present | new — human-readable issue number |
| `Markup.Header` | per-topic | promoted to schema level |
| `Comment.ModifiedDate` | optional | required |
| `Comment` threading | flat | optional `ReplyToComment` reference |
| `Viewpoint.Components.Coloring` | hex colour | hex colour **with alpha** |
| Allowed status/type/priority values | free text | declared in `extensions.xml` |
| Extension schemas | limited | extensible (custom topic types) |
| Cross-vendor compatibility | High (everything reads 2.1) | Mixed (newer tools only) |

## Conversion behaviour

### 2.1 → 3.0 (additive, near-lossless)

- `Labels` → kept as labels
- `ReferenceLinks` upgraded from strings to `{ Url, Description }` with `Description` empty
- `ModifiedDate` populated from `CreationDate` where absent (best-effort)
- `Stage` **not** synthesised — left empty for a downstream tool to fill

### 3.0 → 2.1 (lossy — every drop is a `warnings` entry from `convert`)

- `Stage` dropped → preserved as a label `Stage:<value>` so the data survives, relocated
- `ReferenceLinks` objects flattened to the URL string only
- Extension-schema fields dropped (recorded as warnings)
- Per-comment images + threaded replies flattened (2.1 comments are flat)
- Viewpoint colouring alpha channel dropped to RGB

## Why 3.0 matters in AECO

- **`Stage`** lets you say "this clash is for *tender* coordination, not *construction*" — useful when running BCF round-trips across RIBA / AIA stages on one project. In 2.1 you fake it with a label.
- **Extension schemas** let domain tools attach proprietary metadata without breaking interop.
- **Threaded comments** make a long dispute readable instead of a flat wall.

The practical loss when forced back to 2.1 is `Stage` and threading — both recoverable as labels/flat text, neither fatal.

## See also

- [`convert`](../commands/convert.md) — the verb that performs the translation
- [`merge`](../commands/merge.md) — requires uniform versions; convert first
- [bcf-format-anatomy](./bcf-format-anatomy.md) — `extensions.xml` and the version marker
