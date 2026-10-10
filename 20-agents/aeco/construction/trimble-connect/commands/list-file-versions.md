# `trimble-connect.list-file-versions` — a file's version history

Stateless read. Lists a file's versions; long histories are paged. TC answers **206 Partial Content** on
success, so branch on `status < 300`, not `status == 200`.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `file-id` | string | File id. |
| `range` | string (optional) | A page, e.g. `items=0-99` (zero-based, inclusive). Lists are paged: `headers.content-range` (`items 0-99/240`) gives the total. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

The REST transport returns the HTTP exchange envelope — `{ status, headers, body }` —
so an app can branch on `status` (a 4xx is returned as data, not raised).

```yaml
status:  int                 # 206 on success
headers: object
body:
  type: array
  items:
    id:        string
    versionId: string        # pass to download's version-id
    name:      string
    size:      int
    createdOn: string
```

## REST translation

```http
GET {base}/files/{file-id}/versions
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
