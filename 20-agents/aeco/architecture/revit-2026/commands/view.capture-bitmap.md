# `revit-2026.view.capture-bitmap`

Render a named view to PNG/JPEG at a target resolution.

## When to use

When an app needs a screenshot — for a Teams card, an ACC Issue attachment, a presentation deck, a Monday-morning Slack post. The designer + architect + BIM-manager personas all reached for this.

Read-only. No transaction.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `view-name` | string | yes | — | Exact view name (case-sensitive). |
| `output-path` | string | yes | — | Destination file. Parent directory must exist. |
| `width-pixels` | number | no | `1920` | |
| `height-pixels` | number | no | `1080` | |
| `format` | enum | no | `png` | `png` / `jpeg`. |
| `zoom-to-fit` | boolean | no | `true` | Zoom-extents before capture. |

## Output

```yaml
path: "C:/captures/SE-perspective.png"
width-pixels:  1920
height-pixels: 1080
size-bytes:    487231
```

## Worked example

```yaml
- id: monday-shot
  agent: revit-2026
  command: view.capture-bitmap
  inputs:
    view-name: "3D - SE Perspective"
    output-path: "{{ run.tmp-dir }}/{{ project.name }}-{{ run.date }}-se.png"
    width-pixels: 2560
    height-pixels: 1440
- id: post
  agent: microsoft-365
  command: teams.channel.post-with-screenshot
  inputs:
    channel-id: "{{ secrets.teams.design-review }}"
    text: "Monday view — {{ project.name }}"
    screenshot-path: "{{ monday-shot.path }}"
```

## Implementation note

Wraps `Document.ExportImage(ImageExportOptions { ... })`. Uses `ImageResolution.DPI_300` mapped to the requested pixel dimensions. The view must be openable (no hidden / closed). For "render with morning sun" + photo-realism, route through Enscape/Twinmotion via their respective `*-prep` agents — Revit's own raster export is intentionally simple.

## See also

- `sheet.export-pdfs` — PDF export, not raster
- `enscape-prep` (v0.17) — photoreal rendering
