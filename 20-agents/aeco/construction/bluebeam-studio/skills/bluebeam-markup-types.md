---
name: bluebeam-markup-types
description: This skill should be used when composing snippets that read, create, or filter Bluebeam markups — redlines, callouts, cloud markups, measurements, stamps, hyperlinks, or any of the 30+ markup types. Encodes the markup-type taxonomy (Annotation / Measurement / Form / Signature), the per-markup metadata fields (status, author, page, location, layer, subject), the markup-disposition workflow (Pending / Accepted / Rejected / Resolved), and the gotcha that many markup types are SUBJECT-tagged for filtering (a free-form string field that's the closest thing to a category system).
---

# Bluebeam markup types

Bluebeam Revu's value is its rich markup vocabulary — 30+ markup types beyond what PDF tools normally support. Get the type system right and you can pull architect comments, contractor RFIs, owner sign-offs, and engineer-stamps as STRUCTURED data, not just colored shapes on a PDF.

## The four broad families

| Family | Examples | Use case |
|---|---|---|
| **Annotation** | Note, Text Box, Callout, Cloud, Line, Arrow, Polygon, Polyline | Free-form comment + visual marker |
| **Measurement** | Distance, Area, Perimeter, Volume, Count | Quantitative — used for takeoffs |
| **Form** | Text Field, Checkbox, Radio Button, Signature Field | Interactive fillable forms |
| **Stamp** | Approved, Reviewed, Rejected, custom stamps | Branded "decision" markers |

When AWARE pulls markups via `markup.list`, the response includes a `markupType` field that tells you which family + which specific subtype.

## The common metadata model

Every markup, regardless of type, has these fields:

```json
{
  "id": "9a8b...",                  // UUID
  "type": "Cloud",                  // markup subtype
  "subject": "RFI-047",             // FREE-FORM tag string (see below)
  "author": "john.doe@firm.com",
  "status": "Pending",              // see disposition workflow
  "page": 12,                       // 1-based PDF page
  "createdAt": "2026-05-15T...",
  "modifiedAt": "2026-05-19T...",
  "color": "#FF0000",
  "layer": "Issues",                // user-defined layer name
  "boundingBox": { ... },
  "comments": [ ... ]               // threaded comments below
}
```

Type-specific fields (e.g. `measurementValue` for measurement markups, `signatureData` for signed fields) appear conditionally.

## The Subject field — Bluebeam's category system

The `subject` field is FREE-FORM text. Teams use it as their lightweight category system:

| Subject pattern | Meaning |
|---|---|
| `RFI-NNN` | RFI reference |
| `IFC-Issue-NNN` | IFC clash from Solibri/Navisworks |
| `Code-Comment` | Code-compliance issue |
| `Owner-Markup` | Owner-side comment |
| `As-Built` | Field-collected as-built deviation |

There's NO enforced enum — teams pick their own subject conventions. AWARE workflows should match by regex on subject (`subject =~ /^RFI-/`) rather than expecting structured tags.

## The disposition workflow

The `status` field cycles through:

```
Pending → Accepted   → Resolved
        → Rejected   → (terminal)
        → Cancelled  → (terminal)
        → (custom statuses defined by team's template)
```

Disposition is a manual action by a reviewer in Revu OR via `markup.update-status` REST call. AWARE workflows that "auto-resolve fixed issues" toggle status from Pending → Resolved programmatically.

| Status | Meaning | Typical next |
|---|---|---|
| `Pending` | Newly created, needs review | Accepted or Rejected |
| `Accepted` | Reviewer agrees, work to do | Resolved when fixed |
| `Rejected` | Reviewer disagrees | Cancelled, or re-opened with new info |
| `Resolved` | Fixed; verified | (terminal) |
| `Cancelled` | Withdrawn; no action | (terminal) |

The transitions aren't enforced by the API — any status can change to any other. But the lifecycle above is the convention every coordinator follows.

## Comments — threaded discussions per markup

Each markup carries a Comments array. Comments are threaded (each comment can reply to another). The schema:

```json
"comments": [
  {
    "id": "comm-1",
    "author": "jane.doe",
    "text": "Discussed with structural; will fix",
    "createdAt": "...",
    "replyTo": null                // top-level
  },
  {
    "id": "comm-2",
    "author": "engineer",
    "text": "Awaiting drawing rev",
    "createdAt": "...",
    "replyTo": "comm-1"            // reply to first
  }
]
```

For audit purposes, the comment chain is the SINGLE source of truth on "what happened with this markup". Don't trust `status` alone.

## The Stamps category

Stamps are visual "decision" markers — branded company logos with "Approved", "Reviewed", "Rejected" stamped over a drawing. They're a markup TYPE but carry no comment chain by default. To track approver identity for audit, pair stamps with a Note that has author + text.

## Common gotchas

- **`subject` is unstructured.** Don't trust naming conventions across firms — always allow regex matching.
- **Measurement markups round-trip imperfectly.** Editing a Distance markup in Revu after creation can lose its `measurementValue` field if Revu can't re-calibrate. Read the value once on creation, store separately.
- **Per-page coords are PDF-space (points, 72dpi).** Not document-space. Convert via `pageHeight - y` for cartesian coords.
- **Layer names are case-sensitive in API filters.** "Issues" ≠ "issues". Match exact.
- **Stamp images don't transfer cross-account.** A custom firm stamp only works for users in that firm's Bluebeam account.

## Standard pattern (via `markup.list-by-status`)

```python
import requests

base = "https://studioapi.bluebeam.com/v1"
session_id = args["session-id"]
target_status = args.get("status", "Pending")
session_token = ...  # from join

resp = requests.get(
    f"{base}/sessions/{session_id}/markups",
    headers={"Authorization": f"Bearer {session_token}"},
    params={"status": target_status},
).json()

rows = []
for m in resp.get("markups", []):
    rows.append({
        "id": m["id"],
        "type": m["type"],
        "subject": m["subject"],
        "author": m["author"],
        "page": m["page"],
        "status": m["status"],
        "comment_count": len(m.get("comments", [])),
    })

return {"count": len(rows), "markups": rows}
```

## See also

- `sessions-vs-projects.md` — where markups live
- `markup-disposition-workflow.md` — the lifecycle in depth
- `bluebeam-rest-auth.md` — auth required to reach these endpoints
