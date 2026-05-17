# `microsoft-365.sharepoint.list.append-row`

Append a row to a SharePoint List.

## When to use

When the org runs a central tracking list (RFIs, submittals, app runs, training tracker). SharePoint Lists are the M365 alternative to Planner — flatter shape, more columns, no kanban view. Choose Lists when you need bulk queries; Planner when you need ball-in-court.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `site-id` | string | yes | SharePoint site id. |
| `list-id` | string | yes | List id (NOT title). |
| `fields` | object | yes | Map of column display-name → value. |

## Output

```yaml
item-id: "157"
web-url: "https://acme.sharepoint.com/sites/Projects/Lists/RFIs/DispForm.aspx?ID=157"
```

## Worked example

```yaml
- id: log-rfi
  agent: microsoft-365
  command: sharepoint.list.append-row
  inputs:
    site-id: "{{ secrets.projects-site-id }}"
    list-id: "{{ secrets.rfi-list-id }}"
    fields:
      Title: "RFI-{{ rfi.number }}"
      Project: "{{ project.name }}"
      Author: "{{ run.operator }}"
      Status: "Open"
      DueDate: "{{ rfi.due-date }}"
      Description: "{{ rfi.description }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /sites/{site-id}/lists/{list-id}/items`. The `fields` object's keys must be the *internal* column names (often the display-name with spaces stripped). Use `sharepoint.list.read` once to discover the schema.

## See also

- `sharepoint.list.read` — query the list
- `excel.table.append-row` — the workbook-side equivalent
- `planner.task.create` — for tasks specifically
