# `microsoft-365.excel.range.write`

Write a 2D array of values into a workbook range. Replaces existing cell contents.

## When to use

When the app's output is structured data bound for Excel — write computed load take-downs back to a schedule, push a parameter export to a coordination workbook, populate a status grid. For appending a single row to a structured Table (auto-grow + formula fill), prefer `excel.table.append-row`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `item-id` | string | yes | Workbook file id. |
| `worksheet` | string | yes | Sheet name (use `excel.worksheet.list` to discover). |
| `range` | string | yes | A1 notation, e.g. `A1:D200`. Must match the values' shape. |
| `values` | array<array> | yes | 2D array of cell values (rows of columns). |

## Output

```yaml
cells-written: 800
```

## Worked example

```yaml
- id: write-loads
  agent: microsoft-365
  command: excel.range.write
  inputs:
    drive-id:  "{{ secrets.acme-drive-id }}"
    item-id:   "{{ secrets.loads-workbook-id }}"
    worksheet: "Takedown"
    range:     "A2:C{{ loads.rows.length | plus: 1 }}"
    values:    "{{ loads.rows }}"
  safety:
    transaction-group: bulk-load-write
    snapshot: false  # remote workbook; capture a copy upstream if rollback is needed
```

## Implementation note

Calls `PATCH /drives/{drive-id}/items/{item-id}/workbook/worksheets('{worksheet}')/range(address='{range}')` with body `{ "values": [...] }`. The `range` address must match the dimensions of `values` exactly or Graph returns 400. Requires `Files.ReadWrite` (or `Files.ReadWrite.All`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `excel.range.read` — read first to compute / dedupe
- `excel.table.append-row` — append a row, preserving formulas
- `excel.worksheet.list` — discover the sheet name
