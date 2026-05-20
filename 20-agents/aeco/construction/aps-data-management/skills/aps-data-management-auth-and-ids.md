---
name: aps-data-management-auth-and-ids
description: This skill should be used when calling the aps-data-management agent — APS OAuth (2-legged vs 3-legged), the fact that `aware connect` doesn't wire Autodesk yet, the `b.`-prefixed hub/project IDs (this is the API that USES the prefix), the JSON:API response shape, and rate limits. Encodes the platform foundation for the lower-level Forge Data Management surface.
---

# APS Data Management — auth & IDs

Data Management is the **lower-level** Autodesk file/folder/version API beneath ACC Docs / BIM 360 / Fusion Team. It is the API where the `b.` ID prefix and the JSON:API envelope actually live, so getting both right is the prerequisite for everything.

## Auth — and the `aware connect` gap

Tokens come from `https://developer.api.autodesk.com/authentication/v2/token`:

- **3-legged** (`authorization_code`, user context) — needed for **hub/project/folder/item browsing** (the `/project/v1` and `/data/v1` endpoints). The user must have access to the hub.
- **2-legged** (`client_credentials`, app context) — works for **OSS bucket-level** object operations and some account flows, but **not** for browsing a user's hubs/projects (a 2-legged token has no user, so `GET /project/v1/hubs` returns nothing useful).

Scopes: `data:read`, `data:write`, `data:create`; `bucket:read`/`bucket:create` for OSS.

> **`aware connect` does NOT wire Autodesk/APS yet** — only `trimble-connect`, `microsoft-365`, `google-workspace`. `aware connect autodesk` returns *not supported*; supply an APS token out-of-band via the AWARE credential store until first-class support lands. (APS 3-legged is standard auth-code + PKCE — a tractable follow-up.)

## The `b.` prefix lives HERE

This is the API that **uses** the `b.`-prefixed IDs that the ACC Admin/Issues APIs reject:

- Hub ID = `b.<account-guid>`, Project ID = `b.<project-guid>`.
- The sibling [acc-issues](../../acc-issues/) and [acc-account-admin](../../acc-account-admin/) APIs use the **bare** GUIDs.

So when you carry an ID between this agent and those, **add `b.` coming here, strip it going there**. This is the single most common cross-agent ACC/APS bug. See [acc-issues · acc-platform-auth-and-ids](../../acc-issues/skills/acc-platform-auth-and-ids.md).

## JSON:API envelope

Data Management speaks **JSON:API** (jsonapi.org), not flat JSON. Responses wrap resources as `{ "data": { "type", "id", "attributes": {…}, "relationships": {…} }, "included": [...], "links": {…} }`. To get a folder's display name you read `data.attributes.displayName`, not `data.name`; to walk to children you follow `relationships`/`links`, not a flat `children` array. Lists paginate via `links.next`. Budget for this shape in every glue node that parses a response.

## Rate limits

Per-endpoint limits; over → **HTTP 429 with `Retry-After`** — honor it. Listing a large folder tree or version history will hit limits; paginate via `links.next` and back off on 429. Request increases via ADN.

## See also

- [aps-data-hierarchy](./aps-data-hierarchy.md) — hub → project → folder → item → version navigation
- [aps-upload-and-download-flow](./aps-upload-and-download-flow.md) — storage + the OSS S3 signed-upload gap
- [acc-docs](../../acc-docs/) — the high-level ACC vocabulary over this same storage
