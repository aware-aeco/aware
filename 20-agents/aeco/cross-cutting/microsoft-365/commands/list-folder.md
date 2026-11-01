# `microsoft-365.list-folder`

List the children of a OneDrive / SharePoint folder.

## When to use

When you need to enumerate a directory — find every PDF in an issued-set folder, locate the latest model export, confirm an upload landed. Pairs with `download-file` (fetch each child) and `get-drive-item` (resolve the folder id from a link).

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id, or `me` for the signed-in person's own OneDrive. |
| `folder-id` | string | yes | Folder's item id (use `root` for the drive root). |
| `page-size` | int | no | Children to return, 1 to 200 (default 100). |

Both ids are 1-256 characters of letters, digits, `!`, `_` and `-` — Graph's own id
shapes. Anything else (a path, a dot, a percent sign) is refused before any request.

## Output

```yaml
items:
  - id: "01ABC..."
    drive-id: "b!AbC..."
    name: "A-101.pdf"
    size: 184320
    is-folder: false
    mime-type: "application/pdf"
    modified-time: "2026-10-01T10:00:00Z"
  - id: "01DEF..."
    drive-id: "b!AbC..."
    name: "superseded"
    size: 0
    is-folder: true
more-available: false   # true when Graph has another page, or entries were cut at page-size
omitted: 0              # entries dropped because Graph sent a malformed id
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

Runs in-process through AWARE's code-owned Graph read (`aware agent call` and
workflow nodes alike; #667): `GET /drives/{drive-id}/items/{folder-id}/children`
(or `/me/drive/items/{folder-id}/children` for `drive-id: me`) with a fixed
`$select` and `$top` from `page-size`. **One page:** `@odata.nextLink` sets
`more-available`; it is never followed. No redirect is followed, and the token
goes only to `graph.microsoft.com`. Needs `User.Read` (identity) and one of
`Files.Read`, `Files.Read.All`, `Files.ReadWrite`, `Files.ReadWrite.All`; connect
with `aware connect microsoft-365 --oauth --scopes User.Read,Files.Read.All`.
None of these needs tenant admin consent.

## See also

- `get-drive-item` — resolve the folder id first
- `download-file` — fetch a child's bytes
- `upload-file` — add a file to the folder
