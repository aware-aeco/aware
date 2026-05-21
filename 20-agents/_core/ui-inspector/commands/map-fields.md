# map-fields

**Lifecycle:** single
**Category:** curated
**Mode:** read
**Stability:** Win32 adapter stable; other adapters unimplemented

Map opaque dialog fields to their named settings, with no per-app field list.

## What it does

Recovers which named attribute each nameless field controls by **sentinelizing a
preset**: it copies the preset, writes a unique recognizable sentinel into each
attribute, loads the copy through the app's own loader, re-reads the dialog, and
matches each field's visible value back to the attribute that produced it. Works
for any app that round-trips settings through a dialog — no hand-maintained field
list. See [sentinel-field-mapping](../skills/sentinel-field-mapping.md).

> **On "mode: read".** map-fields writes sentinels into a *disposable copy* of
> the preset and never touches the user's original or the deliverable model, so
> it carries no safety-contract obligation. The only persistent artifact is the
> returned mapping.

## Inputs

```json
{
  "window": "End plate 144 — Properties",
  "preset": "C:/attrs/endplate.j144",
  "sentinel-strategy": "numeric-stride"
}
```

- `window` (string) — top-level window title (substring) or `0x`-prefixed HWND.
- `preset` (string) — preset/attribute file to sentinelize (a copy is used).
- `sentinel-strategy` (enum, default `numeric-stride`) — `numeric-stride`
  (write `i*11.111111`, fuzzy-match ±0.5) or `token` (write `__attr_<name>__`,
  literal-match). Real presets mix both.

## Outputs (receipt)

```json
{
  "ok": true,
  "verb": "map-fields",
  "adapter": "win32",
  "window": "End plate 144 — Properties",
  "result": {
    "mapping": [
      { "field-id": "edit#1043.0", "attribute": "bolt_size", "confidence": 1.0 },
      { "field-id": "edit#1051.0", "attribute": "edge_distance", "confidence": 0.92 }
    ],
    "unmatched": ["weld_prep_angle"]
  },
  "delivered_at": "2026-05-21T09:14:09Z"
}
```

## Idempotency

Yes — the user's preset is never modified (a copy is sentinelized), so repeated
runs are safe and return the same mapping.

## Failure modes

| Exit | Meaning |
|---|---|
| 1 | Window not found / ambiguous |
| 2 | Preset could not be parsed or loaded by the host |
| 3 | Preset path unreadable |
| 5 | No adapter for this OS (only Win32 is implemented) |

## Implementation notes for sidecar authors

Round-trip through the app's **own** preset loader (don't poke fields directly —
that's what proves the binding). `walk-tabs` first so fields on every tab are
laid out, or attributes on unvisited tabs land in `unmatched`. Numeric stride of
`11.111111` keeps adjacent attributes >10 apart so ±0.5 matching never aliases;
widen it if you see collisions. Report low-confidence/ambiguous matches rather
than guessing.

## Example invocation (CLI)

```bash
echo '{"window":"End plate 144 — Properties","preset":"C:/attrs/endplate.j144"}' \
  | aware-ui-inspector.exe map-fields --json-stdin
```

## Example invocation (in an app)

```yaml
do:
  - id: mapping
    agent: ui-inspector
    command: map-fields
    inputs:
      window: "{{ inputs.dialog-title }}"
      preset: "{{ inputs.preset-path }}"
```
