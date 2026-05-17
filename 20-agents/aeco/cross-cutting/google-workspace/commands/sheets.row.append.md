# `google-workspace.sheets.row.append`

Append a single row to the bottom of a Google Sheet.

## When to use

Logging-style use: every QA run, every RFI created, every issued sheet. Auto-appends without overwriting existing rows.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `spreadsheet-id` | string | yes | |
| `worksheet` | string | yes | Sheet (tab) name. |
| `values` | array | yes | Single row of values matching column order. |

## Worked example

```yaml
- id: log-run
  agent: google-workspace
  command: sheets.row.append
  inputs:
    spreadsheet-id: "{{ secrets.app-log-sheet-id }}"
    worksheet: "Runs"
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

Calls `spreadsheets.values.append` with `valueInputOption=USER_ENTERED` so dates / formulas are parsed natively.
