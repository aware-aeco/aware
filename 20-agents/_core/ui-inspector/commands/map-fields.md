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

> **On "mode: read".** map-fields never modifies the user's preset file or the
> deliverable model — it sentinelizes a *disposable copy*. It does load those
> sentinels into the **live dialog**, so it MUST restore the dialog to its
> original state before returning (re-load the original preset, or close without
> applying) — leave the host exactly as found. With that leave-as-found guarantee
> it carries no safety-contract obligation; the only persistent artifact is the
> returned mapping. A sidecar that cannot guarantee the restore must instead run
> this command write-mode and safety-gated.

## Inputs

```json
{
  "window": "End plate 144 — Properties",
  "preset": "C:/attrs/endplate.j144",
  "load-via": { "control": "button#load-attributes" },
  "sentinel-strategy": "numeric-stride"
}
```

- `window` (string) — top-level window title (substring) or `0x`-prefixed HWND.
- `preset` (string) — preset/attribute file to sentinelize (a copy is used).
- `load-via` (object, required) — the host-specific action that makes the dialog
  load a preset file. This is the one step map-fields can't infer generically
  (every host loads presets through different UI), so the caller — who knows the
  host — supplies it. Exactly one of: `{ control: "<id>" }` (invoke a control from
  `enumerate-controls`; its file picker is pointed at the sentinel copy),
  `{ menu: "<path>" }` (invoke a menu item), or `{ agent: "<verb>" }` (call a host
  agent's preset-load verb). The other four commands need no such hook — they are
  pure window introspection; only map-fields drives a load action.
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

Trigger the load through the caller-supplied `load-via` (a control click, menu
item, or host-agent verb) — the sidecar invokes it pointing at the sentinel copy,
rather than poking fields directly; that round-trip through the app's **own**
loader is what proves the binding. **Restore the dialog before returning** — re-load
the original preset (or close without applying) so the open dialog is left exactly
as found; the sentinels are loaded into the live UI and must not be left there for
a user/automation to commit. `walk-tabs` first so fields on every tab are laid
out, or attributes on unvisited tabs land in `unmatched`. Numeric stride of
`11.111111` keeps adjacent attributes >10 apart so ±0.5 matching never aliases;
widen it if you see collisions. Report low-confidence/ambiguous matches rather
than guessing.

## Example invocation (CLI)

```bash
echo '{"window":"End plate 144 — Properties","preset":"C:/attrs/endplate.j144","load-via":{"control":"button#load-attributes"}}' \
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
      load-via: { control: "{{ inputs.load-button-id }}" }   # host-specific load trigger
```
