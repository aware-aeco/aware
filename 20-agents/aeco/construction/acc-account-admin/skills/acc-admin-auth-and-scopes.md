---
name: acc-admin-auth-and-scopes
description: This skill should be used when calling the acc-account-admin agent — the account-level auth model (2-legged via a custom integration vs 3-legged account-admin user), the account:read/write scopes, the bare-GUID account/project IDs, the `aware connect` gap, and rate limits. Encodes how to authorize account-administration calls.
---

# ACC Account Admin — auth & scopes

Account administration (creating projects, provisioning users) is privileged and authorizes differently from the user-facing surfaces.

## 2-legged via a custom integration is the admin pattern

Unlike ACC Issues (which needs a **3-legged** user token), account-admin operations are usually done **server-to-server with a 2-legged (`client_credentials`) token** — but only after an **account admin adds your app as a "custom integration"** to the ACC/BIM 360 account (Account Admin → Settings → Custom Integrations / app provisioning). Provisioning is what grants the app account-level reach; without it a 2-legged token can authenticate but sees nothing. A 3-legged token from a human account admin also works.

Required scopes: **`account:read`** and **`account:write`** (project + user + company management). Add `data:read`/`data:write` if the same token also touches files/issues.

> **`aware connect` does NOT wire Autodesk/APS yet** — only `trimble-connect`, `microsoft-365`, `google-workspace`. `aware connect autodesk` returns *not supported*; supply an APS token out-of-band via the AWARE credential store until first-class support lands. See the [acc-issues](../../acc-issues/skills/acc-platform-auth-and-ids.md) foundation skill for the full APS auth picture.

## IDs are bare GUIDs here

The Admin API uses the **bare** account and project GUIDs (`a4be0…`), **not** the `b.`-prefixed form. The `b.<guid>` form is only for the Data Management API ([aps-data-management](../../aps-data-management/)). If you take a project id from a Data Management response (which is `b.`-prefixed) and feed it to an Admin endpoint, strip the `b.` first. This is the most common account-admin integration bug.

## Region

Account data is regional (`US`, `EMEA`, `AUS`). Admin endpoints expect the `region` matching the account's data residency; a mismatched region returns empty or forbidden. Resolve it once per account.

## Rate limits

Per-endpoint limits; over the limit → **HTTP 429 with `Retry-After`** (seconds) — honor it exactly. Bulk provisioning (onboarding hundreds of users) will hit limits; batch, paginate (`limit`/`offset`), and back off on 429 rather than retrying immediately. Request increases via ADN member support if a migration needs them.

## See also

- [acc-account-admin-model](./acc-account-admin-model.md) — projects, users, companies
- [acc-issues · acc-platform-auth-and-ids](../../acc-issues/skills/acc-platform-auth-and-ids.md) — the shared APS auth + `b.` gotcha, in full
