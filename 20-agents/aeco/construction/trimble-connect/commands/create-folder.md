# `trimble-connect.create-folder` — create a folder (idempotent by name)

Write command (`mode: write`, so `--dry-run` stubs it). Creates a folder under a parent.
Before posting it looks for a folder of the same name under that parent: with
`if-exists: reuse` (the default) the existing folder is returned with `created: false`, so a
retried or re-run scheduled workflow converges on one folder instead of failing or
duplicating; `if-exists: fail` refuses instead.

Typical use — a timestamped drop folder, then an upload into it:

```yaml
- id: drop
  agent: trimble-connect
  command: create-folder
  config:
    parent-id: "{{ inputs.tc-root-id }}"
    name:      "{{ inputs.stamp }}"
- id: put
  agent: trimble-connect
  command: upload
  config:
    folder-id: "{{ drop.folder-id }}"
    filename:  "model.ifc"
    bytes:     "{{ export.bytes }}"
```

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `parent-id` | string | Parent folder id (a project's `rootId` for the top level). |
| `name` | string | New folder name — one path segment; `/` and `\` are refused, as is a blank name. |
| `if-exists` | `reuse` \| `fail` (optional) | Default `reuse`. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_ops.rs`, floless.app#2184):
it builds TC's camelCase JSON body from the kebab-case inputs and sends the optional
`If-Match`, which the generic REST renderer cannot express. A non-2xx **fails the node**
with the step, the status and TC's own message — it is not returned as data.

```yaml
folder-id:  string
version-id: string
name:       string
parent-id:  string
project-id: string
created:    bool             # false when an existing folder was reused
```

## REST translation

```http
1. GET  {base}/folders/{parent-id}/item?name={name}&type=FOLDER   (Bearer)
   → 200: existing folder (reuse → return it; fail → refuse)   404: continue
2. POST {base}/folders                                            (Bearer)
   Content-Type: application/json
   { "name": "{name}", "parentId": "{parent-id}" }
   → 201 folder
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `path separator` / `blank` | Bad input — refused before any request | Fix the node config |
| `… already exists under …` | `if-exists: fail` and the name is taken | Use another name, or `reuse` |
| `item lookup: HTTP 403 …` | No access to the parent | Check permissions in the TC web UI |
| `create folder: HTTP 404 …` | `parent-id` does not exist | Confirm it with `get-folder` / `get-project` (`rootId`) |
| `create folder: TC response has no id` | 2xx without identifiers | Report it — nothing usable was returned |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.upload](./upload.md)
- [trimble-connect.find-item](./find-item.md)
