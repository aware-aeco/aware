# `microsoft-365.excel.worksheet.list`

List the worksheet names in an Excel workbook.

## When to use

When the sheet name isn't known ahead of time — discover tabs before reading, or confirm a "Takedown" sheet exists before writing. Pair with `excel.range.read` / `excel.range.write`, which both require a worksheet name.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `item-id` | string | yes | Workbook file id. |

## Output

```yaml
worksheets:
  - name: "DoorSchedule"
    id:   "{00000000-0001-...}"
  - name: "Takedown"
    id:   "{00000000-0002-...}"
```

## Worked example

```yaml
- id: sheets
  agent: microsoft-365
  command: excel.worksheet.list
  inputs:
    drive-id: "{{ secrets.acme-drive-id }}"
    item-id:  "{{ secrets.doors-workbook-id }}"
- id: read-first
  agent: microsoft-365
  command: excel.range.read
  inputs:
    drive-id:  "{{ secrets.acme-drive-id }}"
    item-id:   "{{ secrets.doors-workbook-id }}"
    worksheet: "{{ sheets.worksheets[0].name }}"
    range:     "A1:Z1"
```

## Implementation note

Calls `GET /drives/{drive-id}/items/{item-id}/workbook/worksheets`, returning each sheet's `name` and `id` (the `id` is stable across renames; the `name` is not). Requires `Files.Read.All` (or `Files.ReadWrite`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `excel.range.read` — read a range once the sheet is known
- `excel.range.write` — write a range
- `excel.table.append-row` — append to a named table
