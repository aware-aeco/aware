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
| `auth-as` | string (optional) | Named credential. |

## Outputs

```yaml
projects:
  type: array
  items:
    id:       string
    name:     string
    rootId:   string         # root folder id — pass to list-folders
    location: string         # region, e.g. "northAmerica"
```

The API returns projects unsorted; sort client-side if order matters.

## REST translation

```http
GET https://app.connect.trimble.com/tc/api/2.0/projects?top=100&skip=0
Authorization: Bearer ****
```

## Composition example

### Resolve a project's root folder by name

```yaml
- id: projects
  agent: trimble-connect
  command: list-projects

- id: root
  inline:
    kind: pick
    description: Take the target project's root folder id.
    code: p => p.name == "Fab Pipeline" ? p.rootId : null

- id: folders
  agent: trimble-connect
  command: list-folders
  config: { folder-id: "{{ root }}" }
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |
| `tc.auth-expired` | Refresh expired | `aware connect trimble-connect --refresh` |
