# `trimble-connect.find-item` — look up a file or folder by name

Stateless read. Finds the file or folder with an exact name directly under a folder. A
miss is the answer `{ found: false }`, not an error — so an app can branch on `found`.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `folder-id` | string | Parent folder id to look in. |
| `name` | string | Exact item name (sent URL-encoded). |
| `type` | `FOLDER` \| `FILE` (optional) | Restrict the match to one kind. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_ops.rs`, floless.app#2184):
it builds TC's camelCase JSON body from the kebab-case inputs and sends the optional
`If-Match`, which the generic REST renderer cannot express. A non-2xx **fails the node**
with the step, the status and TC's own message — it is not returned as data.

```yaml
found:      bool             # false → no other field is present
id:         string           # the item id, whichever kind it is
type:       string           # FOLDER | FILE
folder-id:  string           # when type is FOLDER (== id)
file-id:    string           # when type is FILE (== id)
version-id: string
name:       string
parent-id:  string
project-id: string
size:       int
```

## REST translation

```http
GET {base}/folders/{folder-id}/item?name={name}[&type=FOLDER|FILE]
Authorization: Bearer ****
→ 200 item | 404 (→ found: false)
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `item lookup: HTTP 403 …` | No access to the folder | Check permissions in the TC web UI |
| `item lookup: HTTP 401 …` | Access token expired | `aware connect trimble-connect --refresh` |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.create-folder](./create-folder.md) — uses the same lookup
