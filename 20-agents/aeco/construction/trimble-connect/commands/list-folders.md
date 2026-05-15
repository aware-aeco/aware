# `trimble-connect.list-folders` — enumerate folders in a project

Stateless command. Returns the folders one level deep under a parent (or under the project root if no parent is given).

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project UUID. |
| `parent-folder-id` | string (optional) | List children of this folder. Default `null` = list project root. |
| `auth-as` | string (optional) | Named credential. |

## Outputs

```yaml
folders:
  type: array
  items:
    id:         string
    name:       string
    parent-id:  string
    file-count: int
    path:       string         # human-readable path from root, e.g. "Fab/East Tower/Drawings"
```

The list is **one level deep**. To enumerate recursively, call the command per folder (or use the `walk-folders` command in v0.2).

## REST translation

```http
GET https://app.connect.trimble.com/tc/api/2.0/projects/{project-id}/folders?parent={parent-folder-id}
Authorization: Bearer ****
```

## Composition examples

### Pick a folder by name (common interactive pattern)

```yaml
- id: list
  agent: trimble-connect
  command: list-folders
  config: { project-id: "{{ inputs.project-id }}" }

- id: filter
  inline:
    kind: predicate
    code: |
      f => f.name == "Fab Drawings"

- id: target-folder
  inline:
    kind: pick-first
    description: Take the first matching folder (assumes unique names at this level)

- id: upload
  agent: trimble-connect
  command: upload
  config:
    folder-id: "{{ target-folder.id }}"
    ...
```

The two inline glue steps (`predicate` + `pick-first`) make the folder resolution visible in the topology rather than buried in a query string.

### Cache the folder ID once

For long-running apps that always upload to the same folder, resolve the folder ID at app start and cache it:

```yaml
nodes:
  - id: resolve
    agent: trimble-connect
    command: list-folders
    config: { project-id: "..." }
    cache: app-lifetime           # only resolve once per app run

  - id: pick
    inline:
      kind: pick
      code: f => f.name == "Fab Drawings"

  - id: upload
    agent: trimble-connect
    command: upload
    config:
      folder-id: "{{ pick.id }}"
      ...
```

`cache: app-lifetime` is an app-level hint — the orchestrator caches `resolve`'s output across all event invocations within a single app run.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.project-not-found` | project-id invalid or no access | Verify in TC web UI |
| `tc.parent-folder-not-found` | parent-folder-id invalid | List from root and walk down |
| `tc.auth-expired` | Refresh expired | `aware connect trimble-connect --refresh` |
