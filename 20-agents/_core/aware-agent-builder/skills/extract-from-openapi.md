# Agent Builder Skill · Extract from an OpenAPI spec

**Source type: `openapi`. Point at a published OpenAPI / Swagger spec. Each endpoint becomes a command; tags become skill files.**

Best path for cloud / SaaS / REST-shaped AECO software (Procore, Trimble Connect, Autodesk APS, BIMcollab, Speckle).

## Inputs

- URL or file path to an OpenAPI 3.x spec (also handles 2.0 Swagger via auto-conversion)
- Optional: filter to specific tags or paths
- Optional: auth scheme override (when the spec under-specifies)

## Pipeline

```
1. Fetch spec               (curl if URL, read if path)
2. Validate                 (openapi-spec-validator or similar)
3. Convert if Swagger 2.0   (swagger-to-openapi tool)
4. Bucket endpoints by tag  (one tag → one skill file)
5. Generate commands        (one command per endpoint with non-trivial usage)
6. Generate skills          (one per tag: auth notes, common parameters, gotchas)
7. Generate manifest        (with transport.rest set to the spec's base URL)
8. Detect auth scheme       (oauth2 / apiKey / bearer → maps to AWARE credential type)
9. Validate output
```

## Endpoint → command mapping

```
GET    /projects                    →  command: list-projects
GET    /projects/{id}               →  command: get-project
POST   /projects                    →  command: create-project
PUT    /projects/{id}               →  command: update-project
DELETE /projects/{id}               →  command: delete-project

GET    /projects/{id}/folders       →  command: list-folders
POST   /projects/{id}/folders/{fid}/files  →  command: upload-file
```

Heuristic for command naming:

- `GET /collection` → `list-<collection>`
- `GET /collection/{id}` → `get-<collection-singular>`
- `POST /collection` → `create-<collection-singular>`
- `PUT/PATCH /collection/{id}` → `update-<collection-singular>`
- `DELETE /collection/{id}` → `delete-<collection-singular>`
- Anything else → `<http-verb>-<derived-from-path>` (and flag for human review)

## Skills bucketed by tag

OpenAPI's `tags` field groups endpoints. Each tag becomes one skill file:

```
trimble-connect/
├── manifest.yaml
├── skills/
│   ├── projects.md      ← all endpoints tagged "Projects"
│   ├── folders.md       ← all endpoints tagged "Folders"
│   ├── files.md         ← all endpoints tagged "Files"
│   └── auth.md          ← generated from spec's security schemes
└── commands/
    ├── list-projects.md
    ├── get-project.md
    ├── ...
```

Untagged endpoints go into `skills/general.md`.

## Auth scheme detection

| Spec security scheme | AWARE credential type | Notes |
|---|---|---|
| `oauth2` (authorizationCode flow) | `oauth2-pkce` | Generates the `aware connect <agent>` flow |
| `oauth2` (clientCredentials) | `oauth2-client` | Server-to-server; needs client id/secret in credential file |
| `apiKey` (header) | `api-key` | Single header value |
| `apiKey` (query) | `api-key-query` | Discouraged — agent emits a warning |
| `http` (bearer) | `bearer-token` | User-provided token; no refresh |
| `http` (basic) | `basic-auth` | Discouraged for SaaS — agent warns |

The detected scheme drives the `auth.md` skill and the `aware connect <agent>` behavior.

## What's auto-generated vs. what needs refinement

Auto-generated (high confidence):
- Input/output schemas (from `requestBody` and `responses`)
- HTTP method and path
- Auth header injection pattern

Needs refinement by a human:
- **Idempotency keys** — OpenAPI specs rarely declare these
- **Rate limits** — usually in vendor docs, not the spec
- **Retry semantics** — what's safe to retry, what isn't
- **Pagination patterns** — many specs are inconsistent
- **Common gotchas** — anything learned from production use

The builder marks each generated skill with `> Status: auto-generated, refinement welcome` to invite contributors to add their gotchas.

## Limitations

OpenAPI specs vary in quality. Common issues:

- Polymorphic responses (`oneOf` / `anyOf`) may not map cleanly to AWARE's flat schema. Builder emits a flat schema with a note pointing to the original.
- Webhooks / async patterns aren't well-modeled. Builder skips webhook endpoints with a warning.
- Vendor-extension fields (`x-*`) are preserved in manifest as `provenance.source.extensions` but not surfaced as command fields.
