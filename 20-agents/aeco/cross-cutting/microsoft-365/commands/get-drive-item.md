# `microsoft-365.get-drive-item`

Resolve a OneDrive / SharePoint file or folder by id, path, or sharing URL.

## When to use

The first step of almost any file workflow — turn a "here's the link my PM pasted" sharing URL into a stable `item-id` + `download-url`, or confirm a folder exists before listing it. Pairs with `download-file`, `list-folder`, and the Excel range verbs (which need a workbook `item-id`).

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | no | Drive id. If omitted, the user's default drive (`/me/drive`) is used. |
| `item-id` | string | yes | The item id to resolve. Optional if `sharing-url` is supplied. |
| `sharing-url` | string | no | Alternative to `item-id` — a OneDrive/SharePoint share link. Base64url-encoded automatically. |

## Output

```yaml
id:          "01ABC...XYZ"
name:        "DoorSchedule.xlsx"
size:        48213
is-folder:   false
web-url:     "https://acme.sharepoint.com/.../DoorSchedule.xlsx"
download-url: "https://acme-my.sharepoint.com/.../download.aspx?..."
```

## Worked example

```yaml
- id: resolve
  agent: microsoft-365
  command: get-drive-item
  inputs:
    sharing-url: "{{ inputs.pasted-link }}"
- id: read-doors
  agent: microsoft-365
  command: excel.range.read
  inputs:
    item-id: "{{ resolve.id }}"
    worksheet: "DoorSchedule"
    range: "A2:C200"
```

## Implementation note

Calls `GET /me/drive/items/{item-id}` (or `/drives/{drive-id}/items/{item-id}`). For a `sharing-url`, the transport encodes it to a share token and calls `GET /shares/{token}/driveItem`. Requires `Files.ReadWrite.All` (or `Files.Read.All`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `download-file` — fetch the bytes once resolved
- `list-folder` — enumerate a folder's children
- `excel.range.read` — read a resolved workbook
