# `http.get`

HTTP GET — fetch a resource from any REST/JSON endpoint.

## When to use

The read primitive for any web API without a curated agent yet: pull a Supabase row set, read a webhook's status, fetch a JSON config. For an existence/freshness check where you don't need the body, prefer `http.head`.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Absolute request URL (scheme + host + path). |
| `headers` | object | no | Flat `{ name: value }` map. Put auth here (see the `auth-and-secrets` skill). |
| `query` | object | no | Flat `{ key: value }` map, percent-encoded and appended to the URL. |

## Output

```yaml
status: 200
headers:
  content-type: "application/json"
body:           # parsed JSON when the response is JSON, else a string
  - id: 1
    name: "Tower A"
```

`status` is the numeric HTTP status. A non-2xx response is returned here (not raised as an error) — branch on `status` downstream. See the `response-and-errors` skill.

## Worked example

Read the open drawings for a project from a Supabase (PostgREST) table:

```yaml
- id: drawings
  agent: http
  command: get
  config:
    url: "https://xyz.supabase.co/rest/v1/drawings"
    headers:
      apikey:        "{{ secrets.supabase }}"
      Authorization: "Bearer {{ secrets.supabase }}"
    query:
      select: "id,number,status"
      status: "eq.open"
```

## Implementation note

Runs `GET <url>` via the substrate's built-in REST transport (the `ureq` client). Headers and query params are applied as given; secrets in header values are resolved from the vault by the template layer before the request is built. The response is shaped into `{ status, headers, body }` with the body parsed as JSON when it parses.

## See also

- `http.head` — status + headers only, no body
- `http.post` — create / invoke
- `auth-and-secrets` skill — vault-backed auth headers
- `response-and-errors` skill — status handling + pagination
