# `trimble-connect.list-folder-by-path` — list a folder addressed by path

Stateless read. Lists the items in the folder at a path within a project — so an app can
address `Project/Drawings/MRN` without first walking ids down from the root.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `path` | string | Folder path within the project, e.g. `Project Name/Drawings/MRN`. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_ops.rs`, floless.app#2184):
it builds TC's camelCase JSON body from the kebab-case inputs and sends the optional
`If-Match`, which the generic REST renderer cannot express. A non-2xx **fails the node**
with the step, the status and TC's own message — it is not returned as data.

```yaml
items:
  type: array
  items:
    id:       string
    name:     string
    type:     string         # FOLDER | FILE
    parentId: string
```

## REST translation

```http
GET {base}/folders/by_path?path={path}&projectId={project-id}
Authorization: Bearer ****
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `list folder by path: HTTP 404 …` | No folder at that path | Check the path's first segment (the project's root folder name) |
| `list folder by path: TC did not return a list of items` | Unexpected response shape | Report it — the API contract changed |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
