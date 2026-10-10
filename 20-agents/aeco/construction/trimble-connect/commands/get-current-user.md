# `trimble-connect.get-current-user` — who is signed in

Stateless read. Returns the authenticated Trimble Connect user — the cheapest call that
proves the stored credential works, so it doubles as a connection check.

## Lifecycle

`single` — one call, one response

## Inputs

None.

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

The REST transport returns the HTTP exchange envelope — `{ status, headers, body }` —
so an app can branch on `status` (a 4xx is returned as data, not raised).

```yaml
status:  int
headers: object
body:
  id:        string
  email:     string
  firstName: string
  lastName:  string
  status:    string          # e.g. ACTIVE
```

## REST translation

```http
GET {base}/users/me
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
- [trimble-connect.list-projects](./list-projects.md)
