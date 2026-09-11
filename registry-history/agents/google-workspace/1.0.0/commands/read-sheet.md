# `google-workspace.read-sheet`

Read a range of cells from a Google Sheet.

## When to use

Pull a schedule, a parameter map, or a checklist out of a project tracker so an app can act on it — read the issue register before generating an email. Resolve tab names with `sheets.worksheet.list` first if unsure.

**READ-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `spreadsheet-id` | string | yes | |
| `range` | string | yes | A1 notation (e.g. `"Sheet1!A1:D100"`). |

## Output

```yaml
values:
  - [cell, cell, ...]
  - [cell, cell, ...]
```

## Worked example

```yaml
- id: read-register
  agent: google-workspace
  command: read-sheet
  inputs:
    spreadsheet-id: "{{ secrets.issue-register-id }}"
    range: "Issues!A2:F500"
```

## Implementation note

Calls `GET spreadsheets.values.get` (Sheets v4, `https://sheets.googleapis.com/v4/spreadsheets/`). Empty trailing cells are omitted from each row. Read scope: `spreadsheets` (or `spreadsheets.readonly`). Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`write-sheet`](./write-sheet.md)
- [`sheets.row.append`](./sheets.row.append.md)
- [`sheets.worksheet.list`](./sheets.worksheet.list.md)
