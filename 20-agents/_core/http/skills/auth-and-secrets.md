---
name: auth-and-secrets
description: How to authenticate http-agent calls using the AWARE credential vault — referencing secrets in header values, the bearer/apikey/basic patterns, and why secrets never belong inline in the app file.
---

# Auth & secrets for the `http` agent

A generic client needs credentials, and credentials must never be written into the app file. AWARE keeps them in the **credential vault** (`~/.aware/credentials/<name>.json`); you reference them by name and the runtime substitutes the value just before the request is built.

## The rule

Put `{{ secrets.<name> }}` in a **header value**. Never paste a raw token into the `.flo`.

```yaml
- id: call
  agent: http
  command: get
  config:
    url: "https://api.example.com/v1/me"
    headers:
      Authorization: "Bearer {{ secrets.example-api }}"
```

`{{ secrets.example-api }}` resolves from `~/.aware/credentials/example-api.json`. The token never appears in the app file, the lockfile, or the run trace's `would-write` preview — only the reference does.

## Common auth patterns

**Bearer token** (OAuth2, most modern APIs):
```yaml
headers:
  Authorization: "Bearer {{ secrets.service-token }}"
```

**API key header** (Supabase, many SaaS APIs):
```yaml
headers:
  apikey:        "{{ secrets.supabase }}"
  Authorization: "Bearer {{ secrets.supabase }}"   # Supabase wants both
```

**Custom key header**:
```yaml
headers:
  X-API-Key: "{{ secrets.vendor-key }}"
```

**Basic auth** — pre-encode `user:pass` to base64 and store the *encoded* string as the secret, then:
```yaml
headers:
  Authorization: "Basic {{ secrets.legacy-basic }}"
```

## Provisioning a secret

The vault entry is created out-of-band (the agent never mints credentials):

```bash
aware connect <name>          # for OAuth-style flows the substrate knows
# or place the value at ~/.aware/credentials/<name>.json by hand
```

Name the secret after the *service*, not the workflow, so multiple apps can share one vault entry (`supabase`, not `drawing-register-key`).

## Why this matters

- **Portability** — the same app file runs against anyone's account; only their vault differs.
- **Auditability** — the run trace shows *which* secret was used, never its value.
- **Rotation** — rotate the token in one vault file; every app picks it up.

This is the "header/secret handling via the vault" the http agent exists to provide — the substrate does the resolution, you only ever name the secret.

## See also

- `rest-basics` — the request/response model
- `supabase-rest` — apikey + bearer in a real recipe
