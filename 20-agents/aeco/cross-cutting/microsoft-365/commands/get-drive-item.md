# `microsoft-365.get-drive-item`

Read one OneDrive / SharePoint file or folder's metadata by id.

## When to use

Confirm what an id points at before acting on it — its name, size, type, when it changed and the
link a person opens it with. Pairs with `list-folder` (which returns the ids).

Read-only. Never returns a download URL or the file's content.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id, or `me` for the signed-in person's own OneDrive. |
| `item-id` | string | yes | The item's id, or `root`. |

Both ids are 1-256 characters of letters, digits, `!`, `_` and `-` — Graph's own id shapes.
Anything else (a path, a dot, a percent sign) is refused before any request. Resolving a sharing
link is not offered yet (it needs its own consent review).

## Output

```yaml
id:            "01ABC...XYZ"
drive-id:      "b!AbC..."
name:          "DoorSchedule.xlsx"
size:          48213
is-folder:     false
mime-type:     "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
modified-time: "2026-10-01T10:00:00Z"
web-url:       "https://acme.sharepoint.com/.../DoorSchedule.xlsx"
truncated:     ["name"]   # only when a text field was cut to its limit
```

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /drives/{drive-id}/items/{item-id}`
(or `/me/drive/items/{item-id}` for `drive-id: me`) with a fixed `$select` that leaves out
`@microsoft.graph.downloadUrl`; the projection ignores a download URL even if one is sent. No redirect
is followed, and the token goes only to `graph.microsoft.com`. Needs `User.Read` (identity) and one of
`Files.Read`, `Files.Read.All`, `Files.ReadWrite`, `Files.ReadWrite.All`; connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Files.Read.All`. None needs tenant admin
consent.

## See also

- `list-folder` — the ids of a folder's children
