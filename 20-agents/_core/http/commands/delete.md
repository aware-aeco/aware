# `http.delete`

HTTP DELETE — remove a resource.

## When to use

Delete a record, revoke a token, tear down a resource. Often paired with a URL filter (PostgREST `?id=eq.N`) or an idempotency guard.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Absolute request URL identifying the resource(s) to remove. |
| `headers` | object | no | Flat `{ name: value }` map. |
| `query` | object | no | Flat `{ key: value }` map (e.g. a delete filter). |
| `body` | object | no | Optional payload — some APIs accept a filter/body on DELETE. |

## Output

```yaml
status: 204
headers: {}
body: ""        # 204 No Content commonly returns an empty body
```

## Worked example

Delete a stale drawing-register row by number:

```yaml
- id: drop-drawing
  agent: http
  command: delete
  config:
    url: "https://xyz.supabase.co/rest/v1/drawings"
    headers:
      apikey:        "{{ secrets.supabase }}"
      Authorization: "Bearer {{ secrets.supabase }}"
    query:
      number: "eq.{{ inputs.drawing-number }}"
  safety:
    transaction-group: register-sync
    snapshot: false  # remote delete is irreversible; confirm upstream
```

## Implementation note

Runs `DELETE <url>`. A `204 No Content` yields an empty-string `body`; APIs that echo the deleted rows return them in `body`. Response shaped into `{ status, headers, body }`.

## See also

- `http.patch` — soft-delete via a status field instead of a hard delete
- `supabase-rest` skill — PostgREST delete filters
