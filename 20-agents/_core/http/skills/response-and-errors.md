---
name: response-and-errors
description: How the http agent reports outcomes — the status/headers/body shape, why 4xx/5xx come back as a normal response, branching on status with predicates and assert, conditional requests, and paginating a collection.
---

# Responses & errors with the `http` agent

Every `http` command returns the same envelope:

```yaml
status:  number        # the HTTP status code
headers: { ... }        # response headers, flat map (lower-cased names)
body:    ...            # parsed JSON, or a raw string if not JSON
```

## 4xx/5xx are responses, not crashes

A completed HTTP exchange — even `404` or `500` — is returned as a normal value with that `status`. The run does **not** abort. Only a *transport* failure (DNS, connection refused, TLS, timeout) raises an error and stops the node.

This is deliberate: many workflows want to branch on status (a `404` means "doesn't exist yet → create it"). You decide what's fatal.

## Branching on status

Gate downstream work with an inline predicate:

```yaml
- id: fetch
  agent: http
  command: get
  inputs: { url: "https://api.example.com/v1/items/{{ inputs.id }}" }
- id: exists?
  inline:
    kind: predicate
    description: continue only on success
    code: "e => e.status >= 200 && e.status < 300"
```

Or make a bad status hard-fail the run with `assert`:

```yaml
- id: must-succeed
  assert:
    expr: "{{ fetch.status }} == 200"
    on-fail: "upstream API returned {{ fetch.status }}, expected 200"
```

## Conditional requests (cheap freshness)

Use `http.head` + `If-None-Match` / `If-Modified-Since` to skip work when nothing changed — a `304` response means "unchanged":

```yaml
headers:
  If-None-Match: "{{ inputs.last-etag }}"
```

## Pagination

REST collections page in one of two common ways; loop with the workflow's `for-each` / re-invoke pattern:

- **Offset/limit or page** — increment a `query.page` (or `offset`) until `body` comes back empty.
- **Cursor / `Link` header** — follow `headers.link` (RFC 8288 `rel="next"`) or a `body.next_cursor` until it's absent.

Capture the next pointer from each response (`{{ page.body.next_cursor }}`) and feed it into the next call's `query`.

## Rate limits

On `429 Too Many Requests`, the response carries a `Retry-After` header (seconds, or an HTTP date). The substrate's REST transport already honors `Retry-After` for the in-flight request; for app-level backoff, branch on `status == 429` and re-invoke after a delay.

## See also

- `rest-basics` — the request side
- `auth-and-secrets` — vault-backed credentials
- `supabase-rest` — applying all of the above to PostgREST
