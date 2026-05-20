---
name: acc-platform-auth-and-ids
description: This skill should be used when calling any Autodesk Construction Cloud / APS endpoint through the acc-issues agent — APS OAuth (2-legged vs 3-legged), the fact that `aware connect` does not yet wire Autodesk, the notorious `b.`-prefixed project-ID gotcha (Admin/Issues use the bare GUID, Data Management uses `b.<guid>`), and APS rate limits. Encodes the platform foundation every ACC call depends on.
---

# ACC / APS auth & IDs

ACC Issues is one surface of Autodesk Platform Services (APS, formerly Forge). Two things sink more ACC integrations than any API detail: getting the **token type** wrong, and getting the **project-ID format** wrong. This skill nails both.

## Auth — and the `aware connect` gap

APS tokens come from `https://developer.api.autodesk.com/authentication/v2/token` in two flavors:

| Flavor | Grant | Acts as | ACC Issues |
|---|---|---|---|
| **2-legged** | `client_credentials` | the app itself (no user) | works **only** if the app is added to the ACC account as a **service account / custom integration** |
| **3-legged** | `authorization_code` | a signed-in user | **the normal path** — issues are scoped to a user who must be a project member |

> **`aware connect` does NOT wire Autodesk/APS yet.** Its OAuth config covers only `trimble-connect`, `microsoft-365`, and `google-workspace`, so `aware connect autodesk` / `aware connect acc` returns *not supported*. Until first-class support lands, supply an APS access token out-of-band via the AWARE credential store. (APS 3-legged is a standard authorization-code + PKCE flow, so wiring it is a tractable follow-up — tracked separately.)

Scopes you'll request: `data:read`, `data:write` (issues + attachments), and for the admin agent `account:read` / `account:write`.

## The `b.` project-ID gotcha (the #1 ACC bug)

The same project has **two ID forms**, and the APIs disagree on which to use:

| API | Project ID form |
|---|---|
| ACC **Admin** API (`acc-account-admin`) | **bare** GUID — `a4be0…` |
| ACC **Issues** API (this agent) | **bare** GUID — `a4be0…` |
| APS **Data Management** API (`aps-data-management`, and **issue attachments**) | **`b.`-prefixed** — `b.a4be0…` |

So within a single workflow you switch forms: create the issue with the bare GUID, but when you **upload an attachment** (which goes through Data Management storage) you use `b.<guid>`. Passing the `b.`-prefixed ID to the Issues API — or the bare ID to Data Management — yields confusing 404/400s. Strip or add `b.` deliberately at each hop; never assume one form works everywhere. The account/hub ID has the same split (`b.<account-guid>` in Data Management).

## Region

ACC data is regional. Some endpoints require a `region` (`US`, `EMEA`, `AUS`) header/param matching where the hub lives; a US token hitting an EMEA hub returns empty/forbidden. Confirm the hub's region once and pass it consistently.

## Rate limits

APS enforces per-endpoint rate limits; over the limit returns **HTTP 429 with a `Retry-After` header** (seconds). Honor `Retry-After` exactly rather than a fixed sleep. Limits vary per API (the ACC Issues API publishes its own); request increases through ADN (Autodesk Developer Network) member support if a bulk job needs more. Paginate with `limit`/`offset` on list endpoints rather than pulling everything at once.

## See also

- [acc-issues-domain](./acc-issues-domain.md) — issue types/subtypes, custom attributes, attachments (where `b.` bites)
- The [acc-account-admin](../../acc-account-admin/) agent — shares this auth model; project IDs are bare GUIDs there too
- The [aps-data-management](../../aps-data-management/) agent — the `b.`-prefixed lower-level surface
