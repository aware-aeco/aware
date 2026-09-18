# `google-workspace.list-files`

List files in Drive matching an optional query.

## When to use

Discover what's in a project share before acting — "find the issued PDFs", "list every model in the WIP folder". The read primitive most Drive flows open with.

**READ-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `query` | string | no | — | Drive search query (e.g. `"mimeType='application/pdf'"`). |
| `page-size` | int | no | `100` | Max files per page. |

## Output

```yaml
files:
  - id:        string
    name:      string
    mime-type: string
    size:      number
```

## Worked example

```yaml
- id: find-pdfs
  agent: google-workspace
  command: list-files
  inputs:
    query: "mimeType='application/pdf' and trashed=false"
    page-size: 50
```

## Implementation note

Calls `GET files.list` (Drive v3, `https://www.googleapis.com/drive/v3/files`) with the `q` parameter carrying the search query. Read scope: `drive.readonly` (or broader `drive`). On HTTP 429 / 403 `rateLimitExceeded`, back off exponentially.

## See also

- [`download-file`](./download-file.md)
- [`drive.folder.create`](./drive.folder.create.md)
- [`drive.share`](./drive.share.md)
