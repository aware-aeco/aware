# build-overlay

**Lifecycle:** single
**Category:** curated
**Mode:** read
**Stability:** stable (pure HTML generation, adapter-independent)

Emit a self-contained interactive HTML overlay from controls + screenshots.

## What it does

Composes the captured screenshot(s) with absolutely-positioned hot zones over
each control's `rect`, producing one portable HTML file: hover a control and a
tooltip shows its name / type / value (and mapped attribute, if `map-fields`
output is supplied). Inline CSS/JS, no external assets — opens in any browser.

This command is **adapter-independent**: it consumes the structured output of
`enumerate-controls` / `walk-tabs` (+ optional `map-fields`) and the screenshot
paths, so it works the same regardless of which host produced them. It pairs
with the `html-report` agent — the `tekla-connection-xray` template is this
overlay's first skin. For shared styling conventions (dark theme, self-contained
output) see the html-report [styling](../../html-report/skills/styling.md) skill.

## Inputs

```json
{
  "controls": { "tabs": [ /* walk-tabs output */ ] },
  "screenshots": ["out/endplate/01-parts.png", "out/endplate/02-bolts.png"],
  "output-path": "out/endplate-xray.html",
  "title": "End plate 144 — X-ray"
}
```

- `controls` (object) — `enumerate-controls` (`{controls:[…]}`) or `walk-tabs`
  (`{tabs:[…]}`) output. May also carry `map-fields` `mapping` to label hot zones.
- `screenshots` (array of path strings, optional) — screenshot paths in tab
  order, embedded as base64 so the file is self-contained. **Optional when
  `controls` is `walk-tabs` output** — each tab already carries its own
  `screenshot-path`, which `build-overlay` uses directly; supply `screenshots`
  for single-dialog `enumerate-controls` output, which has no embedded path.
- `output-path` (string) — destination HTML path.
- `title` (string, optional) — page title; defaults to the window title.

## Outputs (receipt)

```json
{
  "ok": true,
  "verb": "build-overlay",
  "adapter": "n/a",
  "window": "End plate 144 — Properties",
  "result": { "path": "out/endplate-xray.html", "bytes": 248104 },
  "delivered_at": "2026-05-21T09:14:12Z"
}
```

## Idempotency

Yes — pure function of its inputs. Same controls + screenshots produce an
equivalent file; re-running overwrites the output.

## Failure modes

| Exit | Meaning |
|---|---|
| 2 | A referenced screenshot path is missing or unreadable |
| 3 | Output path not writable |
| 6 | controls/screenshots shape not recognized (expected enumerate-controls/walk-tabs output) |

## Implementation notes for sidecar authors

Embed each screenshot as a base64 `data:` URI so the file is self-contained
(no sibling assets). Position hot zones with absolute CSS using the control
`rect` (client coords align with the `PW_CLIENTONLY` capture). Keep CSS/JS inline.
Reuse the html-report dark theme. One `<section>` per tab; a tab strip toggles
which screenshot + hot-zone layer is visible. Hovering a hot zone reveals
`label / type / value` (+ `attribute` when present).

## Example invocation (CLI)

```bash
echo '{"controls":{...},"screenshots":["out/01.png"],"output-path":"out/xray.html"}' \
  | aware-ui-inspector.exe build-overlay --json-stdin
```

## Example invocation (in an app)

```yaml
do:
  - id: walk
    agent: ui-inspector
    command: walk-tabs
    inputs: { window: "{{ inputs.dialog-title }}", output-dir: "out/x" }
  - id: overlay
    agent: ui-inspector
    command: build-overlay
    inputs:
      controls: "{{ walk }}"        # walk-tabs output embeds each tab's screenshot-path,
      output-path: "out/{{ inputs.connection }}-xray.html"  # so `screenshots` isn't needed here
```
