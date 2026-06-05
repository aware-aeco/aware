# `trimble-connect.list-projects` — list the projects you can access

Stateless command. Returns every Trimble Connect project the authenticated user is a
member of. This is the entry point for the read path: each project carries a `rootId`
(its root folder) you feed to [`list-folders`](./list-folders.md).

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `top` | int (optional) | Page size, max 100. Default 100. |
| `skip` | int (optional) | Number of projects to skip (pagination). Default 0. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (see the agent's `auth:` block).

## Outputs

The REST transport returns the HTTP exchange envelope — `{ status, headers, body }` —
so an app can branch on `status` (a 4xx is returned as data, not raised).
`GET /projects` responds with a bare JSON array, so the projects are in `body`:

```yaml
status:  int
headers: object
body:                        # the projects array
  type: array
  items:
    id:       string
    name:     string
    rootId:   string         # root folder id — pass to list-folders
    location: string         # region, e.g. "northAmerica"
```

The API returns projects unsorted; sort `body` client-side if order matters.

## REST translation

```http
GET https://app.connect.trimble.com/tc/api/2.0/projects?top=100&skip=0
Authorization: Bearer ****
```

## Composing

The output is the HTTP envelope `{ status, headers, body }`; the projects array is in
`body`. Downstream nodes reference it with templating (e.g. `{{ projects.body }}`).

Selecting one project by name *at runtime* isn't expressible with the current inline
glue — a `predicate` node only gates an event stream with `e => e.<field> …`; it can't
pick a field out of a response body. So apps typically take the target `projectId` /
`rootId` as an app input and pass it straight to [`list-folders`](./list-folders.md) as
`folder-id`.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |
| `401` in `body` (`INVALID_SESSION`) | Access token expired | `aware connect trimble-connect --refresh` |
