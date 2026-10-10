# `trimble-connect.get-file` — one file's details

Stateless read. Returns the latest version's details for a file. For an older version use
[`list-file-versions`](./list-file-versions.md); to fetch content use [`download`](./download.md).

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `file-id` | string | File id. |

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
  versionId: string
  name:      string
  size:      int             # bytes
  parentId:  string
  projectId: string
```

## REST translation

```http
GET {base}/files/{file-id}
Authorization: Bearer ****
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `404` in `body` | Id invalid or no access | Verify the id in the TC web UI |
| `401` in `body` (`INVALID_SESSION`) | Access token expired | `aware connect trimble-connect --refresh` |
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.download](./download.md)
