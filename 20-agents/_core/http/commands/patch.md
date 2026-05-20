# `http.patch`

HTTP PATCH — partially update a resource, or upsert.

## When to use

Update a subset of fields on **existing** rows: flip a status, patch a few attributes. For Supabase (PostgREST), PATCH is a *filtered update* — it modifies the rows your URL filter selects and inserts nothing. (Insert-or-update **upsert** is `http.post` + `Prefer: resolution=merge-duplicates` — see the `supabase-rest` skill.)

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Absolute request URL (often carrying a PostgREST filter, e.g. `?id=eq.42`). |
| `headers` | object | no | Flat `{ name: value }` map. |
| `query` | object | no | Flat `{ key: value }` map (filters, `select`). |
| `body` | object | no | The fields to update (object) — sent as JSON. |

## Output

```yaml
status: 200
headers: {}
body:
  - id: 42
    status: "issued"
```

## Worked example

Mark a known drawing as issued (update the rows matching the `number` filter):

```yaml
- id: mark-issued
  agent: http
  command: patch
  config:
    url: "https://xyz.supabase.co/rest/v1/drawings"
    headers:
      apikey:        "{{ secrets.supabase }}"
      Authorization: "Bearer {{ secrets.supabase }}"
      Prefer:        "return=representation"
    query:
      number: "eq.{{ drawing.number }}"
    body:
      status: "issued"
  safety:
    transaction-group: register-sync
    snapshot: false
```

## Implementation note

Runs `PATCH <url>`. The JSON `body` carries the changed fields; PostgREST scopes the update to the rows selected by the URL filter (`?id=eq.N`, `?number=eq.X`). With **no** filter, PATCH updates the whole table — always scope it. Response shaped into `{ status, headers, body }`.

## See also

- `http.post` — create (no conflict resolution)
- `http.put` — full replace
- `supabase-rest` skill — filters, upsert, and the register-sync pattern
