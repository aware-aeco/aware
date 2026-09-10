# `google-workspace.slides.export-pdf`

Export a Google Slides deck as PDF.

## When to use

The designer / architect "client-deck issue" verb. Pair with `slides.replace-text` to fill the template first, then `drive.share` (or `upload-file`) to ship the PDF straight to the client folder.

**READ-mode** (reads the deck; writes a new PDF file locally).

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `presentation-id` | string | yes | |
| `output-path` | string | yes | Local path for the exported PDF. |

## Output

```yaml
path:       string
page-count: number
size-bytes: number
```

## Worked example

```yaml
- id: export-deck
  agent: google-workspace
  command: slides.export-pdf
  inputs:
    presentation-id: "{{ secrets.client-deck-id }}"
    output-path: "out/client-deck-{{ run.date }}.pdf"
```

## Implementation note

Export goes through Drive, not the Slides API: `GET files.export?mimeType=application/pdf` on the presentation id. Read scopes: `drive.readonly` (export) + `presentations` if also editing. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`slides.replace-text`](./slides.replace-text.md)
- [`drive.share`](./drive.share.md)
- [`upload-file`](./upload-file.md)
