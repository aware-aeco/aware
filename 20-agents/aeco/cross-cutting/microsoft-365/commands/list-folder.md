# `microsoft-365.list-folder`

List the children of a OneDrive / SharePoint folder.

## When to use

When you need to enumerate a directory — find every PDF in an issued-set folder, locate the latest model export, confirm an upload landed. Pairs with `download-file` (fetch each child) and `get-drive-item` (resolve the folder id from a link).

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `folder-id` | string | yes | Folder's item id (use `root` for the drive root). |

## Output

```yaml
items:
  - id: "01ABC..."
    name: "A-101.pdf"
    size: 184320
    is-folder: false
  - id: "01DEF..."
    name: "superseded"
    size: 0
    is-folder: true
```

## Worked example

```yaml
- id: list-set
  agent: microsoft-365
  command: list-folder
  inputs:
    drive-id:  "{{ secrets.acme-drive-id }}"
    folder-id: "{{ secrets.issued-set-folder-id }}"
- id: digest
  agent: microsoft-365
  command: teams.channel.post-message
  inputs:
    team-id: "Project Acme"
    channel-id: "issued-sets"
    text: "Issued set published: {{ list-set.items.length }} sheets."
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `GET /drives/{drive-id}/items/{folder-id}/children`. The transport follows `@odata.nextLink` to page through large folders and flattens the result into `items`. Requires `Files.Read.All` (or `Files.ReadWrite.All`). On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `get-drive-item` — resolve the folder id first
- `download-file` — fetch a child's bytes
- `upload-file` — add a file to the folder
