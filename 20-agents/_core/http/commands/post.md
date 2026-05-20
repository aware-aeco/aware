# `http.post`

HTTP POST — create a resource or invoke an action on any REST/JSON endpoint.

## When to use

Create a record, fire a webhook, trigger a build, send a chat-webhook message — any "make something happen" call against a web API without a curated agent. For partial updates / upserts, prefer `http.patch`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Absolute request URL. |
| `headers` | object | no | Flat `{ name: value }` map (auth, content negotiation). |
| `query` | object | no | Flat `{ key: value }` map. |
| `body` | object | no | Request payload. An object/array is sent as JSON; a string is sent verbatim. |

## Output

```yaml
status: 201
headers:
  location: "/rest/v1/projects?id=eq.42"
body:
  id: 42
  name: "Tower A"
```

## Worked example

Insert a project row into Supabase and return the created representation:

```yaml
- id: create-project
  agent: http
  command: post
  config:
    url: "https://xyz.supabase.co/rest/v1/projects"
    headers:
      apikey:        "{{ secrets.supabase }}"
      Authorization: "Bearer {{ secrets.supabase }}"
      Prefer:        "return=representation"
    body:
      name:   "{{ inputs.project-name }}"
      status: "active"
  safety:
    transaction-group: register-sync
    snapshot: false  # remote API; no local snapshot to roll back
```

## Implementation note

Runs `POST <url>`. An object/array `body` is serialized to JSON and sent with `Content-Type: application/json` unless the caller already set a `Content-Type` header; a string `body` is sent verbatim. The response is shaped into `{ status, headers, body }`. A non-2xx status is returned as a normal response — inspect `status` rather than assuming success.

## See also

- `http.patch` — partial update / upsert (Supabase merge-duplicates)
- `http.put` — full replace
- `auth-and-secrets` skill — vault-backed auth headers
- `supabase-rest` skill — the PostgREST insert/upsert recipe
