# `trimble-connect.update-file` — rename and/or move a file

Write command (`mode: write`). Renames and/or moves a file. TC applies only the move
when one request carries both a new parent and a new name ("move will take precedence
before rename"), so asking for both sends the move, then the rename. With `if-match`,
the rename is guarded by the version the move returned, so a concurrent change between
the two requests fails with 412 instead of being overwritten.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `file-id` | string | File id. |
| `name` | string (optional) | New name — one path segment. |
| `parent-id` | string (optional) | Destination folder id (moves it). |
| `if-match` | string (optional) | Apply only if this is still the file's latest `versionId`; TC answers 412 otherwise. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_ops.rs`, floless.app#2184):
it builds TC's camelCase JSON body from the kebab-case inputs and sends the optional
`If-Match`, which the generic REST renderer cannot express. A non-2xx **fails the node**
with the step, the status and TC's own message — it is not returned as data.

```yaml
file-id:  string
version-id: string           # the file's version after the change
name:       string
parent-id:  string
project-id: string
```

## REST translation

```http
PATCH {base}/files/{file-id}                 (Bearer)
If-Match: {if-match}                              (only when given; the rename then
                                                  carries the move's new versionId)
Content-Type: application/json
{ "parentId": "{parent-id}" }     then     { "name": "{name}" }
→ 200 file
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `needs name (rename), parent-id (move), or both` | Neither `name` nor `parent-id` given | Give at least one |
| `update file: HTTP 412 …` | `if-match` is not the latest version | Re-read with `get-file` and retry |
| `update file: HTTP 404 …` | Id (or destination) does not exist | Verify the ids |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.get-file](./get-file.md) — for the current `versionId`
