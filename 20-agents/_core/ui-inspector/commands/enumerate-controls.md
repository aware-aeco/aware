# enumerate-controls

**Lifecycle:** single
**Category:** curated
**Mode:** read
**Stability:** Win32 adapter stable; other adapters unimplemented

Enumerate every control in a window or dialog.

## What it does

Walks the host's **real** control tree via the active adapter and returns one
record per control: a stable `id`, normalized `type`, associated `label`,
current `value`, client-area `rect`, and owning `tab`. On Win32 this uses
`EnumChildWindows`, which surfaces the controls that high-level accessibility
trees collapse into opaque panes (a real connection dialog enumerated ~760
controls where UIAutomation reported ~38). See
[win32-control-enumeration](../skills/win32-control-enumeration.md).

Use this to:
- Discover the field inventory of a dialog you can't introspect any other way
- Feed `build-overlay` (rects) and `map-fields` (field ids)
- Diff a dialog's controls across host versions

## Inputs

```json
{
  "window": "End plate 144 — Properties",
  "dialog-title": null
}
```

- `window` (string) — top-level window title (substring match) or an `0x`-prefixed HWND.
- `dialog-title` (string, optional) — child-dialog title to scope enumeration to one modal.

## Outputs (receipt)

```json
{
  "ok": true,
  "verb": "enumerate-controls",
  "adapter": "win32",
  "window": "End plate 144 — Properties",
  "result": {
    "controls": [
      { "id": "edit#1043.0", "type": "edit", "label": "Bolt size",
        "value": "20", "rect": { "x": 120, "y": 48, "w": 64, "h": 22 }, "tab": "Parts" }
    ]
  },
  "delivered_at": "2026-05-21T09:14:00Z"
}
```

## Idempotency

Yes — read-only. Enumerating twice returns the same tree (modulo the user
editing the dialog between calls).

## Failure modes

| Exit | Meaning |
|---|---|
| 1 | Window not found / ambiguous (refine `window` or pass an HWND) |
| 2 | Adapter error reading the control tree |
| 5 | No adapter for this OS (only Win32 is implemented) |

## Implementation notes for sidecar authors

Read [win32-control-enumeration](../skills/win32-control-enumeration.md) **first**.
Read text with `WM_GETTEXT` (not `GetWindowText`, which fails cross-process for
some controls); map every rect into **client** coordinates so it aligns with
`capture-window`; associate labels by nearest-static. A control with no text
returns `label: ""` / `value: ""`, never an error.

## Example invocation (CLI)

```bash
echo '{"window":"End plate 144 — Properties"}' | aware-ui-inspector.exe enumerate-controls --json-stdin
```

## Example invocation (in an app)

```yaml
do:
  - id: controls
    agent: ui-inspector
    command: enumerate-controls
    inputs:
      window: "{{ inputs.dialog-title }}"
```
