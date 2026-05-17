# `microsoft-365.excel.table.append-row`

Append a row to a named Excel Table (not a raw range).

## When to use

When the destination is a structured Table (Insert → Table). Tables auto-grow when you append; formulas in the new row are filled automatically. The "log every QA run / every RFI / every issued sheet" primitive.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `item-id` | string | yes | Workbook file id. |
| `worksheet` | string | yes | Sheet hosting the table. |
| `table-name` | string | yes | The table's defined name (e.g. `AppRuns`). |
| `values` | array | yes | Single row of values matching the table's column order. |

## Output

```yaml
row-index: 142    # new row's index in the table
```

## Worked example

```yaml
- id: log-run
  agent: microsoft-365
  command: excel.table.append-row
  inputs:
    drive-id:   "{{ secrets.ops-drive-id }}"
    item-id:    "{{ secrets.app-log-workbook-id }}"
    worksheet:  "Runs"
    table-name: "AppRuns"
    values:
      - "{{ run.id }}"
      - "{{ run.app }}"
      - "{{ run.operator }}"
      - "{{ now.iso }}"
      - "{{ run.status }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /workbook/tables('{table-name}')/rows/add`. The table must exist; create it once via the Excel UI or the `create-table` Graph endpoint (not currently exposed — out of scope for the audit-driven first wave).

## See also

- `excel.range.write` — for raw cell writes
- `excel.range.read` — read first if you need to dedupe
- `sharepoint.list.append-row` — same pattern, against a SharePoint List
