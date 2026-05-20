---
name: acc-account-admin-model
description: This skill should be used when managing ACC projects, users, or companies with the acc-account-admin agent — the account→project hierarchy, project create/import/archive, user assignment and roles, member vs partner companies, and the account-vs-project admin distinction. Encodes the administration model behind the 30 endpoints.
---

# ACC Account Admin model

The Account Admin API manages the **account** (a.k.a. hub) and the **projects**, **users**, and **companies** inside it — the provisioning layer beneath Issues/Docs.

## Account → project → membership

- An **account** (hub) contains many **projects**. The Admin `GET projects` returns **all** projects in the account — including ones the calling user isn't a member of (this is the key difference from Data Management's `getHubProjects`, which returns only the user's projects). Use Admin when you need the full portfolio view.
- **Project create**: `POST .../accounts/{accountId}/projects` with name, type, and optionally a **project template** (clones folders, roles, and settings from an existing project — the right way to onboard consistently). Projects carry a **status** (`active`, `archived`, `suspended`, …); archive/restore rather than delete.
- IDs are **bare GUIDs** (see [acc-admin-auth-and-scopes](./acc-admin-auth-and-scopes.md)).

## Users and roles

- Users exist at the **account** level (the directory) and are **assigned to projects** with an **access level** — `project_admin` vs a member role. Account admin ≠ project admin: an account admin manages the account + can administer any project; a project admin manages one project.
- Provisioning a hire = ensure the account user exists → assign to the right projects with the right roles → associate their company. De-provisioning reverses it; do it on offboarding to close access.
- Role/permission changes are themselves permission-gated — a token without `account:write` (or an app not provisioned as a custom integration) gets `403`.

## Companies: member vs partner

The account **company directory** distinguishes **member companies** (your own org's entities) from **partner companies** (external collaborators — architects, GCs, subs). Users are associated with a company; many ACC permission and visibility behaviors key off company. Sync this from an ERP/HR or billing system to keep the directory authoritative.

## AECO patterns

- **Project onboarding from ERP.** New job in the ERP → `create` the ACC project from a standard template → assign the core team with roles → associate companies. One deterministic app replaces a day of manual setup.
- **Cross-tool user provisioning.** On hire, fan out: add to ACC (here) + the [slack](../../slack/) workspace + the engineering tools, all from one HR trigger.
- **Access audit.** Walk every project (Admin `GET projects` — the full list) → list members + roles → flag external/partner-company users with admin access for a quarterly review.

## See also

- [acc-admin-auth-and-scopes](./acc-admin-auth-and-scopes.md) — 2-legged custom-integration auth, account scopes, bare-GUID IDs
- The [acc-issues](../../acc-issues/) and [acc-docs](../../acc-docs/) agents — what users do once provisioned
