---
name: rest-basics
description: How to call any REST/JSON API with the http agent — the command-is-the-method model, the url/headers/query/body inputs, the uniform response shape, and when to reach for http versus a curated vendor agent.
---

# REST basics with the `http` agent

The `http` agent is the substrate's generic web client. Use it whenever you need to talk to a REST/JSON service that doesn't have a curated agent yet.

## The model: command = HTTP method

There is one command per HTTP method, and the command name **is** the method:

| Command | Method | Mode |
|---|---|---|
| `get` | GET | read |
| `head` | HEAD | read |
| `post` | POST | write |
| `put` | PUT | write |
| `patch` | PATCH | write |
| `delete` | DELETE | write |

The request itself comes entirely from the node's `config:` block (the app-spec key for an agent invocation's parameters) — the agent holds no per-endpoint knowledge:

```yaml
- id: call
  agent: http
  command: get
  config:
    url:     "https://api.example.com/v1/things"   # required, absolute
    headers: { Accept: "application/json" }          # optional flat map
    query:   { page: "2", limit: "50" }              # optional flat map
    # body:  { ... }                                 # post/put/patch/delete only
```

## The uniform response

Every command returns the same shape, so downstream nodes are predictable:

```yaml
status:  200          # numeric HTTP status
headers: { ... }      # response headers as a flat map
body:    ...          # parsed JSON when the response is JSON, else a string
```

Reference fields downstream with `{{ call.status }}`, `{{ call.body.id }}`, `{{ call.headers.etag }}`.

## Rules of the road

- **`url` is absolute.** Scheme + host + path. Build per-call URLs with templating: `url: "https://.../items/{{ inputs.id }}"`.
- **`headers` and `query` are flat maps** of string→value. Nested structures aren't valid header/query values and are ignored.
- **JSON in, JSON out.** An object/array `body` is serialized to JSON automatically (`Content-Type: application/json` unless you set your own). The response body is parsed as JSON when it parses, else returned as a raw string.
- **Non-2xx is not a crash.** A `404`/`500` is returned as a normal response with that `status` — branch on it (see the `response-and-errors` skill), don't assume success.

## When NOT to use `http`

`http` is the escape hatch, not the destination. If you find yourself hand-rolling the same vendor's URLs, auth, and pagination across many apps, that vendor has earned a **curated agent** — build one (`aware build --from-openapi …`) so the workflow-grade verbs (`list-drawings`, `upsert-row`) exist as first-class commands. That is decalog #6 ("ship agents, not apps") in motion: `http` is how organic usage reveals which agent to build next.

## See also

- `auth-and-secrets` — vault-backed API keys and bearer tokens
- `response-and-errors` — status handling, retries, pagination
- `supabase-rest` — a complete worked vendor recipe on top of `http`
