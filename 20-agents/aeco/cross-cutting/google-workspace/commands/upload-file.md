# `google-workspace.upload-file`

Upload bytes to Drive. Uses simple (≤ 5MB) or resumable (large) upload.

## When to use

Push a generated artifact into a project share — a clash report, an exported PDF, a QA log. The counterpart to `download-file`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `filename` | string | yes | Name the file lands under in Drive. |
| `bytes` | bytes | yes | File content. |
| `parent-folder-id` | string | no | If omitted, file lands at My Drive root. |

## Output

```yaml
file-id: string
web-url: string
```

## Worked example

```yaml
- id: upload-report
  agent: google-workspace
  command: upload-file
  inputs:
    filename: "Clash report — {{ run.date }}.pdf"
    bytes: "{{ build-report.bytes }}"
    parent-folder-id: "{{ secrets.project-folder-id }}"
  safety:
    transaction-group: publish
    snapshot: false
```

## Implementation note

Calls `POST files.create` (Drive v3) with `uploadType=multipart` for payloads ≤ 5MB and `uploadType=resumable` for larger ones. Write scope: `drive.file` (or broader `drive`). Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`download-file`](./download-file.md)
- [`drive.folder.create`](./drive.folder.create.md)
- [`drive.share`](./drive.share.md)
