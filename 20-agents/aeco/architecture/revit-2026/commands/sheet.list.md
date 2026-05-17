# `revit-2026.sheet.list`

List every `ViewSheet` in the active Revit document, optionally filtered by issue state.

## When to use

The foundation for any "Monday sheet status" report. Use this before `sheet.export-pdfs`, `revision.bump`, or any sheet-set operation that needs to enumerate the deliverable.

Read-only. No transaction.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `filter-issued-for` | string | no | — | Match `Parameter:SheetIssuedFor` (e.g. `"Issued for Construction"`). |
| `filter-revision` | string | no | — | Match the current revision letter (e.g. `"C"`). |
| `filter-discipline` | string | no | — | Match the sheet discipline. |

## Output

```yaml
sheets:
  - sheet-number: "A-100"
    sheet-name: "Site Plan"
    current-revision: "C"
    issue-date: "2026-04-12"
    issued-for: "Issued for Construction"
    drawn-by: "JT"
    checked-by: "MP"
    current-revision-on-sheet: "C"
```

## Worked example

A 60-second Monday rollup:

```yaml
nodes:
  - id: sheets
    agent: revit-2026
    command: sheet.list
    inputs:
      filter-issued-for: "Issued for Construction"
  - id: post
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      channel-id: "{{ secrets.teams.coord }}"
      title: "Issued sheets — {{ now.date }}"
      rows: "{{ sheets.sheets }}"
```

## Implementation note

Wraps `FilteredElementCollector.OfClass(typeof(ViewSheet))` followed by per-sheet parameter reads (`Parameter:SheetIssuedFor`, `Parameter:SheetCurrentRevision`, `Parameter:DrawnBy`, `Parameter:CheckedBy`, `Parameter:CurrentRevisionOnSheet`). Always runs inside a read-only transaction subscope.

## See also

- `sheet.export-pdfs` — turn the list into PDFs
- `revision.bump` — add the next revision letter to a selection
- `schedule.export` — schedule data, not sheets
