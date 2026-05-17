# `revit-2026.schedule.export`

Export a `ViewSchedule` to CSV or XLSX preserving filter / sort / group order.

## When to use

Whenever a schedule needs to leave Revit — fee proposal areas, door schedules, window schedules, room schedules, equipment counts. Read-only.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `schedule-name` | string | yes | — | Exact schedule view name. |
| `output-path` | string | yes | — | Destination file. |
| `format` | enum | no | `csv` | `csv` / `xlsx`. |
| `include-totals` | boolean | no | `true` | Append totals row if the schedule defines one. |

## Output

```yaml
path: "C:/exports/door-schedule.csv"
row-count: 142
column-headers: ["Mark","Width","Height","Fire Rating","Acoustic","Hardware Set"]
```

## Worked example

```yaml
- id: door-schedule
  agent: revit-2026
  command: schedule.export
  inputs:
    schedule-name: "Door Schedule"
    output-path: "{{ run.tmp-dir }}/doors.csv"
- id: qa-pass
  agent: revit-2026
  command: schedule.find-rows-missing
  inputs:
    schedule-name: "Door Schedule"
    parameters: ["Fire Rating","Hardware Set"]
```

## Implementation note

Uses `ViewSchedule.Export(folder, name, ViewScheduleExportOptions)` with field separator `,` for CSV and the modern OpenXML pipeline for XLSX. Multi-line cells get quoted properly.

## See also

- `schedule.find-rows-missing` — QA the schedule
- `element.by-parameter-value` — find elements by parameter (not via schedule)
