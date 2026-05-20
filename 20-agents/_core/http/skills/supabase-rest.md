---
name: supabase-rest
description: A complete worked recipe for driving Supabase (PostgREST) through the http agent — base URL, apikey+bearer auth, select/filter reads, insert, the POST upsert (merge-duplicates), filtered update and delete, and the model-driven drawing-register sync pattern from issue #101.
---

# Supabase REST with the `http` agent

Supabase exposes every table as a REST endpoint via **PostgREST**. The `http` agent is enough to read and sync a Supabase project end-to-end — no Supabase-specific agent required.

## Setup

- **Base URL:** `https://<project-ref>.supabase.co/rest/v1/`
- **Auth:** Supabase wants the key in **two** headers — provision the key once in the vault (`aware connect supabase` or a hand-placed `~/.aware/credentials/supabase.json`):

```yaml
headers:
  apikey:        "{{ secrets.supabase }}"
  Authorization: "Bearer {{ secrets.supabase }}"
```

Use the **service-role** key for unattended back-end sync (bypasses row-level security); use the **anon** key only when RLS policies intentionally allow the operation.

## Read — `http.get`

`select` picks columns; filters use `column=op.value` (operators: `eq`, `neq`, `gt`, `gte`, `lt`, `lte`, `like`, `in`, `is`):

```yaml
- id: open-drawings
  agent: http
  command: get
  config:
    url: "https://xyz.supabase.co/rest/v1/drawings"
    headers: { apikey: "{{ secrets.supabase }}", Authorization: "Bearer {{ secrets.supabase }}" }
    query:
      select: "id,number,status"
      status: "eq.open"
      order:  "number.asc"
```

## Insert — `http.post`

`Prefer: return=representation` echoes the created rows back in `body`:

```yaml
- id: add-project
  agent: http
  command: post
  config:
    url: "https://xyz.supabase.co/rest/v1/projects"
    headers:
      apikey:        "{{ secrets.supabase }}"
      Authorization: "Bearer {{ secrets.supabase }}"
      Prefer:        "return=representation"
    body: { name: "{{ inputs.project }}", status: "active" }
```

## Upsert — `http.post` + `merge-duplicates`

Upsert is **POST** (not PATCH) with `Prefer: resolution=merge-duplicates`. When the conflict target is a UNIQUE column rather than the primary key, name it with `on_conflict`:

```yaml
- id: sync-drawing
  agent: http
  command: post
  config:
    url: "https://xyz.supabase.co/rest/v1/drawings"
    headers:
      apikey:        "{{ secrets.supabase }}"
      Authorization: "Bearer {{ secrets.supabase }}"
      Prefer:        "resolution=merge-duplicates,return=representation"
    query:
      on_conflict: "number"            # unique drawing number, not the PK
    body:
      number: "{{ drawing.number }}"
      status: "{{ drawing.status }}"
  safety:
    transaction-group: register-sync
    snapshot: false
```

## Filtered update — `http.patch`

PATCH updates only the rows your filter selects (it never inserts). Always scope it:

```yaml
query: { number: "eq.{{ drawing.number }}" }
body:  { status: "issued" }
```

## Delete — `http.delete`

```yaml
query: { number: "eq.{{ inputs.drawing-number }}" }
```

## The drawing-register sync pattern (issue #101 #4)

On `ModelLoaded` / `DrawingChanged`, keep the Supabase register in lock-step with the Tekla model:

1. **Read** the Tekla drawings (a curated host-agent verb) → list of `{ number, status }`.
2. For each, **upsert** with `http.post` + `merge-duplicates` on `on_conflict=number` — new drawings insert, existing ones update in one idempotent call.
3. Optionally **delete** register rows whose drawing no longer exists in the model.

Because upsert is idempotent, the app is safe to re-run on every model event without creating duplicates — exactly the property an event-triggered sync needs.

## See also

- `rest-basics` — the request/response model
- `auth-and-secrets` — vault-backed keys
- `response-and-errors` — branching on status, pagination
