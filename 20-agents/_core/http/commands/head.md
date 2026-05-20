# `http.head`

HTTP HEAD — fetch only the status line and headers, no body.

## When to use

Cheap existence / freshness checks: does a resource exist (status 200 vs 404), has it changed (`ETag` / `Last-Modified`), how big is it (`Content-Length`) — without downloading the payload. Gate a heavier `http.get` on the result.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | string | yes | Absolute request URL. |
| `headers` | object | no | Flat `{ name: value }` map (auth, conditional headers like `If-None-Match`). |
| `query` | object | no | Flat `{ key: value }` map. |

## Output

```yaml
status: 200
headers:
  etag: "\"a1b2c3\""
  content-length: "20413"
body: ""        # HEAD responses carry no body
```

## Worked example

Only re-export if the upstream workbook changed since last run:

```yaml
- id: probe
  agent: http
  command: head
  config:
    url: "https://api.example.com/v1/workbooks/{{ inputs.id }}"
    headers:
      If-None-Match: "{{ inputs.last-etag }}"
- id: changed?
  inline:
    kind: predicate
    description: proceed only when the resource changed (200, not 304)
    code: "e => e.status == 200"
```

## Implementation note

Runs `HEAD <url>`. The response `body` is always an empty string; the signal is in `status` (e.g. `304 Not Modified`) and `headers` (`ETag`, `Last-Modified`, `Content-Length`).

## See also

- `http.get` — fetch the full body once you've decided to
- `response-and-errors` skill — conditional requests + status handling
