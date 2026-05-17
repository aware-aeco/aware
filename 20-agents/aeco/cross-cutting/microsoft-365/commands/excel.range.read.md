# `microsoft-365.excel.range.read`

Read a range of cells from an Excel workbook on OneDrive / SharePoint.

## When to use

When the source-of-truth for some structured data lives in Excel — fire ratings keyed by mark, parameter values for bulk-write, schedule of values for take-off. Pairs with `parameter.bulk-write` (Revit) or `excel.range.write` (M365) for round-trips.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `item-id` | string | yes | Workbook file id. |
| `worksheet` | string | yes | Sheet name (use `excel.worksheet.list` to discover). |
| `range` | string | yes | A1 notation, e.g. `A2:D500`. |

## Output

```yaml
values:
  - ["Mark", "Fire Rating", "Hardware Set"]
  - ["D-101", "FD30", "HS-01"]
  - ["D-102", "FD60", "HS-02"]
row-count:    3
column-count: 3
```

## Worked example

```yaml
- id: read-doors
  agent: microsoft-365
  command: excel.range.read
  inputs:
    drive-id: "{{ secrets.acme-drive-id }}"
    item-id:  "{{ secrets.doors-workbook-id }}"
    worksheet: "DoorSchedule"
    range: "A2:C200"
- id: write-back
  agent: revit-2026
  command: parameter.bulk-write
  inputs:
    key-parameter: mark
    target-parameter: "Fire Rating"
    rows: "{{ read-doors.values | toKeyedRows: key=0, value=1 }}"
  safety:
    transaction-group: bulk-fire-rating
    snapshot: true
    worksharing:
      check: true
      fail-if-other-user: true
    audit-stamp:
      uda-prefix: AWARE_
```

## Implementation note

Calls the Excel Online endpoint `/drives/{drive-id}/items/{item-id}/workbook/worksheets('{name}')/range(address='{range}')/values`. Values are typed (numbers stay numbers, dates as ISO strings).

## See also

- `excel.range.write` — write a range
- `excel.table.append-row` — append to a named table (preserves formulas)
- `excel.worksheet.list` — discover sheet names
