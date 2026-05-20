# `google-workspace.download-file`

Download a Drive file's bytes by id.

## When to use

Pull a model, drawing, or spreadsheet down for local processing — feed a downloaded IFC into a checker, or grab a sheet to diff. Pair with `list-files` to resolve the id first.

**READ-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `file-id` | string | yes | Drive file id. |

## Output

```yaml
bytes:    bytes
filename: string
```

## Worked example

```yaml
- id: fetch-model
  agent: google-workspace
  command: download-file
  inputs:
    file-id: "{{ find-model.files[0].id }}"
```

## Implementation note

Calls `GET files.get?alt=media` (Drive v3) to stream the raw bytes. For native Google formats (Docs / Sheets / Slides) use `files.export` instead — `alt=media` only works on binary blobs (`files.export` also works under `drive.readonly`). Read scope: `drive.readonly`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`list-files`](./list-files.md)
- [`upload-file`](./upload-file.md)
- [`slides.export-pdf`](./slides.export-pdf.md)
