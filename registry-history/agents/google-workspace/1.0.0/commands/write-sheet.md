# `google-workspace.write-sheet`

Write values to a range in a Google Sheet.

## When to use

Overwrite an existing block of cells — refresh a status column, stamp a computed summary into a tracker. Use `sheets.row.append` instead when you only want to add a row without touching existing data.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `spreadsheet-id` | string | yes | |
| `range` | string | yes | A1 notation target block. |
| `values` | array | yes | 2-D array (rows of cells) matching the range. |

## Output

```yaml
updated-cells: int
```

## Worked example

```yaml
- id: stamp-status
  agent: google-workspace
  command: write-sheet
  inputs:
    spreadsheet-id: "{{ secrets.issue-register-id }}"
    range: "Issues!F2:F4"
    values:
      - ["Closed"]
      - ["Open"]
      - ["Closed"]
  safety:
    transaction-group: publish
    snapshot: true
```

## Implementation note

Calls `PUT spreadsheets.values.update` (Sheets v4) with `valueInputOption=USER_ENTERED` so dates / formulas parse natively. Write scope: `spreadsheets`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`read-sheet`](./read-sheet.md)
- [`sheets.row.append`](./sheets.row.append.md)
- [`sheets.worksheet.list`](./sheets.worksheet.list.md)
