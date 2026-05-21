# capture-window

**Lifecycle:** single
**Category:** curated
**Mode:** read
**Stability:** Win32 adapter stable; other adapters unimplemented

Capture a coordinate-aligned, client-area screenshot of a window or dialog.

## What it does

Renders the window's **client area** to a PNG using the adapter's occlusion-safe
capture (Win32: `PrintWindow` with `PW_CLIENTONLY | PW_RENDERFULLCONTENT`). The
image origin `(0,0)` equals the client origin that `enumerate-controls` reports
rects against, so an overlay built from the two lines up to the pixel.
`PW_CLIENTONLY` is essential — capturing window chrome shifts every control rect
down-and-right. See [win32-control-enumeration](../skills/win32-control-enumeration.md).

Optionally switch to a named `tab` and settle before capturing.

## Inputs

```json
{
  "window": "End plate 144 — Properties",
  "tab": "Parts",
  "output-path": "out/endplate-parts.png"
}
```

- `window` (string) — top-level window title (substring) or `0x`-prefixed HWND.
- `tab` (string, optional) — switch to this tab and settle (>=300 ms) before capture.
- `output-path` (string) — destination PNG path.

## Outputs (receipt)

```json
{
  "ok": true,
  "verb": "capture-window",
  "adapter": "win32",
  "window": "End plate 144 — Properties",
  "result": { "path": "out/endplate-parts.png", "width": 612, "height": 480 },
  "delivered_at": "2026-05-21T09:14:02Z"
}
```

## Idempotency

Yes — read-only on the host. Re-capturing overwrites the output file with an
equivalent image (modulo dialog state changes).

## Failure modes

| Exit | Meaning |
|---|---|
| 1 | Window not found / ambiguous |
| 2 | Capture failed (PrintWindow returned no content) |
| 3 | Output path not writable |
| 5 | No adapter for this OS (only Win32 is implemented) |

## Implementation notes for sidecar authors

Size the bitmap from `GetClientRect`, not the window rect. Always pass
`PW_CLIENTONLY | PW_RENDERFULLCONTENT`. When `tab` is set, switch the page so the
parent swaps it (`PSM_SETCURSEL` to the property-sheet window, or synthesize the
tab click — `TCM_SETCURSEL` alone won't swap the page; see the
[win32-control-enumeration](../skills/win32-control-enumeration.md) skill), then
`Thread.Sleep(300)` before capturing — an 80 ms settle under-paints
late-laid-out controls. Write PNG (lossless) so overlay hot-zones are crisp.

## Example invocation (CLI)

```bash
echo '{"window":"End plate 144 — Properties","tab":"Parts","output-path":"out/parts.png"}' \
  | aware-ui-inspector.exe capture-window --json-stdin
```

## Example invocation (in an app)

```yaml
do:
  - id: shot
    agent: ui-inspector
    command: capture-window
    inputs:
      window: "{{ inputs.dialog-title }}"
      output-path: "{{ run.tmp-dir }}/dialog-{{ run.date }}.png"
```
