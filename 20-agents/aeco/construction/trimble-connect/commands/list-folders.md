# `trimble-connect.list-folders` — enumerate items under a folder

Stateless command. Returns the items (sub-folders **and** files) one level deep under
a folder. Trimble Connect addresses folders by a globally-unique id — there is **no**
`/projects/{projectId}/folders` route — so to enumerate a project's top level, pass the
project's `rootId` (from [`list-projects`](./list-projects.md)) as the `folder-id`.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `folder-id` | string | Folder UUID. A project's `rootId` lists its top level. |
| `auth-as` | string (optional) | Named credential. |

## Outputs

The REST transport returns the HTTP exchange envelope — `{ status, headers, body }` —
so an app can branch on `status` (a 4xx is returned as data, not raised).
`GET /folders/{id}/items` responds with a bare JSON array, so the items are in `body`:

```yaml
status:  int
headers: object
body:                         # the folder's items (sub-folders and files)
  type: array
  items:
    id:       string
    name:     string
    type:     string          # FOLDER | FILE
    parentId: string
    size:     int
```

The list is **one level deep** and mixes folders and files. Filter `body` to
`type == "FOLDER"` for folders only; to enumerate recursively, call the command per
sub-folder id.

## REST translation

```http
GET https://app.connect.trimble.com/tc/api/2.0/folders/{folder-id}/items
Authorization: Bearer ****
```

> **IMPORTANT:** Folder/file endpoints are addressed by globally-unique id and do **not**
> include `/projects/{projectId}/` (see `skills/projects.md § Folders & Files`).

## Composition examples

### List a project's root, then pick a folder by name (common interactive pattern)

```yaml
- id: projects
  agent: trimble-connect
  command: list-projects

- id: root
  inline:
    kind: pick
    description: Find the target project and take its root folder id (from `body`).
    code: out => out.body.find(p => p.name == "Fab Pipeline").rootId

- id: list
  agent: trimble-connect
  command: list-folders
  config: { folder-id: "{{ root }}" }

- id: target-folder
  inline:
    kind: pick
    description: First sub-folder named "Fab Drawings" (items are in `body`).
    code: out => out.body.find(f => f.type == "FOLDER" && f.name == "Fab Drawings")

- id: upload
  agent: trimble-connect
  command: upload
  config:
    folder-id: "{{ target-folder.id }}"
    ...
```

The inline glue steps make the folder resolution visible in the topology rather than
buried in a query string.

### Cache the folder ID once

For long-running apps that always upload to the same folder, resolve the folder ID at
app start and cache it:

```yaml
nodes:
  - id: list
    agent: trimble-connect
    command: list-folders
    config: { folder-id: "{{ inputs.root-folder-id }}" }
    cache: app-lifetime           # only resolve once per app run

  - id: pick
    inline:
      kind: pick
      code: out => out.body.find(f => f.type == "FOLDER" && f.name == "Fab Drawings")

  - id: upload
    agent: trimble-connect
    command: upload
    config:
      folder-id: "{{ pick.id }}"
      ...
```

`cache: app-lifetime` is an app-level hint — the orchestrator caches `list`'s output
across all event invocations within a single app run.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.folder-not-found` (404 in `body`) | folder-id invalid or no access | Verify the id (or the project's `rootId`) in the TC web UI |
| `tc.auth-expired` (401 in `body`) | Refresh expired | `aware connect trimble-connect --refresh` |
