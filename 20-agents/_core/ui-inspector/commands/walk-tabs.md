# walk-tabs

**Lifecycle:** single
**Category:** curated
**Mode:** read
**Stability:** Win32 adapter stable; other adapters unimplemented

Walk every tab of a tabbed dialog, enumerating controls and capturing a
screenshot per tab.

## What it does

The whole-property-sheet version of `enumerate-controls` + `capture-window`: for
each tab it switches, settles (>=300 ms — late-laid-out controls under-count
otherwise), enumerates that tab's controls, and captures a per-tab screenshot.
Tab labels are read **cross-process** (Win32: `TCM_GETITEMW` via `VirtualAllocEx`),
so non-ASCII tab names survive. See
[win32-control-enumeration](../skills/win32-control-enumeration.md). Read-only.

This is the usual front door: one call yields everything `build-overlay` and
`map-fields` need for a full dialog.

## Inputs

```json
{
  "window": "End plate 144 — Properties",
  "output-dir": "out/endplate"
}
```

- `window` (string) — top-level window title (substring) or `0x`-prefixed HWND.
- `output-dir` (string) — directory for the per-tab screenshots.

## Outputs (receipt)

```json
{
  "ok": true,
  "verb": "walk-tabs",
  "adapter": "win32",
  "window": "End plate 144 — Properties",
  "result": {
    "tabs": [
      { "tab": "Parts", "controls": [ /* enumerate-controls shape */ ],
        "screenshot-path": "out/endplate/01-parts.png" },
      { "tab": "Bolts", "controls": [], "screenshot-path": "out/endplate/02-bolts.png" }
    ]
  },
  "delivered_at": "2026-05-21T09:14:05Z"
}
```

## Idempotency

Yes — read-only on the host (leaves the dialog on its last tab). Re-running
overwrites the per-tab screenshots with equivalent images.

## Failure modes

| Exit | Meaning |
|---|---|
| 1 | Window not found / ambiguous |
| 2 | Tab control not found, or a tab read/switch failed |
| 3 | Output directory not writable |
| 5 | No adapter for this OS (only Win32 is implemented) |

## Implementation notes for sidecar authors

Find the `SysTabControl32`, read its item count, and `TCM_GETITEMW`
cross-process for each label (allocate the `TCITEM` + text buffer in the target
process with `VirtualAllocEx`; free it after). **Switch the page so the parent
swaps it** — `PSM_SETCURSEL` to the property-sheet window, or synthesize the tab
click; `TCM_SETCURSEL` alone moves the highlight without swapping the page, so
you'd re-read tab 1 every time (see the
[win32-control-enumeration](../skills/win32-control-enumeration.md) skill). Then
`Thread.Sleep(300)` before enumerating/capturing. Number screenshots in tab order
so `build-overlay` can pair them. Restore the original tab if you can; it's not
required (read-mode).

## Example invocation (CLI)

```bash
echo '{"window":"End plate 144 — Properties","output-dir":"out/endplate"}' \
  | aware-ui-inspector.exe walk-tabs --json-stdin
```

## Example invocation (in an app)

```yaml
do:
  - id: walk
    agent: ui-inspector
    command: walk-tabs
    inputs:
      window: "{{ inputs.dialog-title }}"
      output-dir: "out/{{ inputs.connection }}"
```
