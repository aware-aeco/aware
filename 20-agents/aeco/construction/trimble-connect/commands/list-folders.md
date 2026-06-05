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

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (see the agent's `auth:` block).

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

The list is **one level deep** and mixes folders and files (`type`). To enumerate
recursively, call the command again per sub-folder id.

## REST translation

```http
GET https://app.connect.trimble.com/tc/api/2.0/folders/{folder-id}/items
Authorization: Bearer ****
```

> **IMPORTANT:** Folder/file endpoints are addressed by globally-unique id and do **not**
> include `/projects/{projectId}/` (see `skills/projects.md § Folders & Files`).

## Composing

The output is `{ status, headers, body }`; the items are in `body`, each tagged with a
`type` of `FOLDER` or `FILE`. Pass a `folder-id` — a project's `rootId` (from
[`list-projects`](./list-projects.md)) or a sub-folder id — typically as an app input
(`{{ inputs.folder-id }}`) or from an upstream node (`{{ node.body }}`).

The current inline glue is a boolean `predicate` gate over an event stream
(`e => e.<field> …`); it does not transform a response body, so selecting a specific
item out of `body` is done by the consuming agent (or a future map primitive), not by
inline glue.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `404` in `body` | folder-id invalid or no access | Verify the id (or the project's `rootId`) in the TC web UI |
| `401` in `body` (`INVALID_SESSION`) | Access token expired | `aware connect trimble-connect --refresh` |
