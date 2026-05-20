# `http.put`

HTTP PUT — replace a resource at a known URL.

## When to use

When the API uses PUT for a full replace (idempotent overwrite) of a resource you address by URL — e.g. replace an object in a key-value store, overwrite a document. For partial updates prefer `http.patch`; for create-without-known-URL prefer `http.post`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Absolute request URL identifying the resource to replace. |
| `headers` | object | no | Flat `{ name: value }` map. |
| `query` | object | no | Flat `{ key: value }` map. |
| `body` | object | no | The full replacement representation — sent as JSON. |

## Output

```yaml
status: 200
headers: {}
body:
  id: 42
  name: "Tower A (rev B)"
```

## Worked example

```yaml
- id: replace-config
  agent: http
  command: put
  config:
    url: "https://api.example.com/v1/configs/{{ inputs.config-id }}"
    headers:
      Authorization: "Bearer {{ secrets.example-api }}"
    body:
      retention-days: 30
      enabled: true
  safety:
    transaction-group: config-write
    snapshot: false
```

## Implementation note

Runs `PUT <url>` with the JSON `body` (sent with `Content-Type: application/json` unless the caller sets their own). Response shaped into `{ status, headers, body }`.

## See also

- `http.patch` — partial update / upsert
- `http.post` — create / invoke
