---
name: acc-issues-domain
description: This skill should be used when creating, querying, or updating ACC issues with the acc-issues agent — the issue type/subtype taxonomy, per-project custom attributes, the permission-gated status lifecycle, comments, and the attachment flow (which routes through Data Management and the `b.`-prefixed project ID). Encodes how ACC Issues actually models an issue.
---

# ACC Issues domain

ACC Issues (the v1 API, `/construction/issues/v1/projects/{projectId}/...`) is the issue tracker behind RFIs, observations, punch-list items, design issues, safety, and commissioning. The shape is project-scoped and more structured than BCF.

## Type → subtype, not free text

An issue's category is a **type → subtype** pair, not a free string. You create an issue against an **`issueSubtypeId`**, and the subtype belongs to a type (e.g. type *Quality* → subtype *Punch list*). The available types/subtypes are **per project** — fetch them from the issue-types endpoint and resolve the subtype id before creating; don't hardcode ids across projects.

## Custom attributes are per-project

Beyond the built-in fields, ACC projects define **custom attributes** (attribute definitions) and map them to specific issue subtypes. To set them on an issue you reference the attribute's id and supply a value of the declared type (list/text/numeric/date). Like subtypes, these are project-specific — discover them, don't assume.

## Status is a permission-gated, configurable lifecycle

Issues move through a status set (commonly `open`, `pending`, `in_progress`, `in_review`, `closed`, and dispute/approval states) — but **which statuses exist and who may set them is configured per project and gated by the caller's role**. A field user typically can't jump an issue straight to `closed`; only certain roles can. Treat the status set as data you read from the project, and expect `403`/validation errors when a transition isn't allowed for the token's user — that's a permission signal, not a bug. (This is also why most calls need a **3-legged** user token — see [acc-platform-auth-and-ids](./acc-platform-auth-and-ids.md).)

## Comments and attachments

- **Comments**: `POST .../issues/{issueId}/comments` — the audit trail.
- **Attachments**: this is where the `b.` gotcha bites. An attachment is a file, and files live in **Data Management**, so the flow is: create a storage location → upload the bytes → associate it with the issue. The storage/Data-Management calls use the **`b.`-prefixed** project id, while the issue itself was created with the **bare** GUID. See [acc-platform-auth-and-ids](./acc-platform-auth-and-ids.md).

## AECO patterns

- **Clash → issue.** Navisworks/Tekla/Solibri detects a clash → `create-issue` with a *Coordination* subtype, the clashing element refs in the description, and a viewpoint attachment. Pair with the [bcf-file](../../bcf-file/) agent when the upstream tool speaks BCF (map a BCF topic → an ACC issue; keep the BCF GUID in a custom attribute so the round-trip can reconcile).
- **RFI / punch rollup.** List issues filtered by subtype + status for a weekly site-PM digest (paginate; honor 429).
- **Cross-tool issue sync.** Mirror Procore RFIs or Trimble Connect topics into ACC issues; store the source system's id in a custom attribute as the join key.

## See also

- [acc-platform-auth-and-ids](./acc-platform-auth-and-ids.md) — token type, the `b.` project-ID gotcha, rate limits
- [bcf-file](../../bcf-file/skills/bcf-round-trip-interop.md) — BCF ↔ ACC issue round-tripping
- The [acc-docs](../../acc-docs/) agent — the files side (attachments ultimately live there)
