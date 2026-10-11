# `trimble-connect.get-todo` — one ToDo

Stateless read. Returns one ToDo's details. To find ToDos use [`list-todos`](./list-todos.md).

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `todo-id` | string | ToDo id. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

The REST transport returns the HTTP exchange envelope — `{ status, headers, body }` —
so an app can branch on `status` (a 4xx is returned as data, not raised).

```yaml
status:  int
headers: object
body:
  id:              string
  label:           string
  title:           string
  description:     string
  type:            string
  status:          string      # NEW | IN_PROGRESS | RESOLVED | BLOCKED | CLOSED
  priority:        string      # LOW | NORMAL | HIGH | CRITICAL
  percentComplete: int
  dueDate:         string
  projectId:       string
  assignees:       array
```

## REST translation

```http
GET {base}/todos/{todo-id}
Authorization: Bearer ****
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `404` in `body` | Id invalid or no access | Check it with `list-todos` |
| `401` in `body` (`INVALID_SESSION`) | Access token expired | `aware connect trimble-connect --refresh` |
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |

## See also

- [projects.md](../skills/projects.md) — the API reference
- [trimble-connect.update-todo](./update-todo.md)
