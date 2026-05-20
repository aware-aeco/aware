# `microsoft-365.sharepoint.list.read`

Read items from a SharePoint List as objects keyed by column display-name.

## When to use

The "pull the project roster" / "read the door-spec list" / "get the RFI register" primitive. SharePoint Lists are the flat, many-column M365 store — choose them for bulk queries (Planner is for ball-in-court). Pairs with `sharepoint.list.append-row` for round-trips.

Read-only.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `site-id` | string | yes | — | SharePoint site id. |
| `list-id` | string | yes | — | List id (NOT title). |
| `filter` | string | no | — | OData `$filter` expression. |
| `top` | number | no | `200` | Max rows to return. |

## Output

```yaml
items:
  - Title:   "RFI-042"
    Project: "Acme Tower"
    Status:  "Open"
    DueDate: "2026-05-22"
  - Title:   "RFI-043"
    Project: "Acme Tower"
    Status:  "Closed"
```

## Worked example

```yaml
- id: open-rfis
  agent: microsoft-365
  command: sharepoint.list.read
  inputs:
    site-id: "{{ secrets.projects-site-id }}"
    list-id: "{{ secrets.rfi-list-id }}"
    filter:  "fields/Status eq 'Open'"
    top: 500
- id: digest
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    team-id: "Project Acme"
    channel-id: "coordination"
    title: "Open RFIs — {{ now.date }}"
    rows: "{{ open-rfis.items }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `GET /sites/{site-id}/lists/{list-id}/items?$expand=fields` (with `$filter` / `$top` appended). The transport projects each item's `fields` map up to the top level so rows are keyed by column name; `@odata.nextLink` is followed up to `top`. Note that `$filter` on list columns often requires the `Prefer: HonorNonIndexedQueriesWarningMayFailRandomly` header on non-indexed columns. Requires `Sites.Read.All` (or `Sites.ReadWrite.All`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `sharepoint.list.append-row` — write a row back
- `outlook.mail.search` — the inbox-side query equivalent
- `planner.task.list` — ball-in-court rather than a flat list
