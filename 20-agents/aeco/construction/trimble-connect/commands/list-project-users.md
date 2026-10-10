# `trimble-connect.list-project-users` — who has access to a project

Stateless read. Lists the users with access to a project.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

The REST transport returns the HTTP exchange envelope — `{ status, headers, body }` —
so an app can branch on `status` (a 4xx is returned as data, not raised).

```yaml
status:  int
headers: object
body:
  type: array
  items:
    id:    string
    email: string
    role:  string
```

## REST translation

```http
GET {base}/projects/{project-id}/users
Authorization: Bearer ****
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `404` in `body` | Id invalid or no access | Verify the id in the TC web UI |
| `401` in `body` (`INVALID_SESSION`) | Access token expired | `aware connect trimble-connect --refresh` |
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |

## See also

- [projects.md](../skills/projects.md) — projects, users and members
