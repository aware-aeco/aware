# `google-workspace.sheets.worksheet.list`

List worksheet names + IDs in a spreadsheet.

## When to use

Resolve which tab to read or append to before calling `read-sheet` / `sheets.row.append` — handy when a tracker's tab names vary by project.

**READ-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `spreadsheet-id` | string | yes | |

## Output

```yaml
worksheets:
  - name: string
    id:   number
```

## Worked example

```yaml
- id: list-tabs
  agent: google-workspace
  command: sheets.worksheet.list
  inputs:
    spreadsheet-id: "{{ secrets.issue-register-id }}"
```

## Implementation note

Calls `GET spreadsheets.get` (Sheets v4) with `fields=sheets.properties` so only tab metadata (title + `sheetId`) is returned. Read scope: `spreadsheets.readonly` (or broader `spreadsheets`). Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`read-sheet`](./read-sheet.md)
- [`write-sheet`](./write-sheet.md)
- [`sheets.row.append`](./sheets.row.append.md)
