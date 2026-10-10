# `trimble-connect.delete-folder` — delete a folder

Write command (`mode: write`). Deletes a folder **by its id** — not its version id. Treat it as
destructive.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `folder-id` | string | Folder id. |
| `if-match` | string (optional) | Delete only if this is still the folder's latest `versionId`; TC answers 412 otherwise. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_ops.rs`, floless.app#2184):
it builds TC's camelCase JSON body from the kebab-case inputs and sends the optional
`If-Match`, which the generic REST renderer cannot express. A non-2xx **fails the node**
with the step, the status and TC's own message — it is not returned as data.

```yaml
folder-id: string
deleted:   bool              # always true — any failure fails the node
```

## REST translation

```http
DELETE {base}/folders/{folder-id}                (Bearer)
If-Match: {if-match}                              (only when given)
→ 204
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `delete folder: HTTP 404 …` | Already gone, or no access | Verify the id |
| `delete folder: HTTP 412 …` | `if-match` is not the latest version | Re-read with `get-folder` |
| `delete folder: HTTP 403 …` | Not permitted | Check permissions in the TC web UI |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
