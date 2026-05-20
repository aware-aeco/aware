# `microsoft-365.upload-file`

Upload bytes as a new file into a OneDrive / SharePoint folder.

## When to use

When the app produces a deliverable that has to land in the document library — an exported PDF set, a generated report, a CSV of clashes. The transport picks simple vs resumable upload by size automatically. Pair with `get-drive-item` / `list-folder` to resolve the destination folder id first.

**WRITE-mode** (external state — the document library). Declare `safety:` on the node. There is no in-place snapshot for a brand-new file; `transaction-group: notify, snapshot: false` is honest.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `folder-id` | string | yes | Destination folder's item id. |
| `filename` | string | yes | Name for the new file (extension included). |
| `bytes` | bytes | yes | File content. |

## Output

```yaml
item-id: "01NEW...XYZ"
web-url: "https://acme.sharepoint.com/.../clashes-2026-05-20.csv"
```

## Worked example

```yaml
- id: publish-csv
  agent: microsoft-365
  command: upload-file
  inputs:
    drive-id:  "{{ secrets.acme-drive-id }}"
    folder-id: "{{ secrets.reports-folder-id }}"
    filename:  "clashes-{{ now.date }}.csv"
    bytes:     "{{ clash-report.csv }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

For payloads ≤ 4 MB the transport calls `PUT /drives/{drive-id}/items/{folder-id}:/{filename}:/content`. Larger files use a resumable upload session (`POST .../createUploadSession`) chunked under the 4 MB-per-request ceiling. Requires `Files.ReadWrite.All`; provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `download-file` — the read-side counterpart
- `get-drive-item` — resolve the destination folder id
- `list-folder` — confirm the upload landed
