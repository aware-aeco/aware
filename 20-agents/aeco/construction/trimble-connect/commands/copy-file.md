# `trimble-connect.copy-file` — copy a file into a folder

Write command (`mode: write`). Copies a file version into a folder. TC copies a *version*,
so with no `version-id` the file's latest is resolved first.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `file-id` | string | Source file id. |
| `version-id` | string (optional) | Source version; default = latest. |
| `parent-id` | string | Destination folder id. |
| `merge-existing` | bool (optional) | Default `false`. When `true` and a same-named file exists in the destination, the copy becomes its new version. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_ops.rs`, floless.app#2184):
it builds TC's camelCase JSON body from the kebab-case inputs and sends the optional
`If-Match`, which the generic REST renderer cannot express. A non-2xx **fails the node**
with the step, the status and TC's own message — it is not returned as data.

```yaml
file-id:    string           # the copy
version-id: string
name:       string
parent-id:  string
project-id: string
```

## REST translation

```http
1. GET  {base}/files/{file-id}            (Bearer — only without version-id) → { versionId }
2. POST {base}/files                      (Bearer)
   { "parentId": "{parent-id}", "parentType": "FOLDER",
     "fromFileVersionId": "{version}", "mergeExisting": false }
   → 201 file
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `copy file: source metadata has no versionId` | Source unreadable | Pass `version-id` explicitly |
| `copy file: HTTP 404 …` | Source or destination missing | Verify the ids |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.upload](./upload.md)
