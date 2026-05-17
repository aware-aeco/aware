# `revit-2026.schedule.find-rows-missing`

QA pass over a `ViewSchedule`: list rows where one or more required parameters are empty.

## When to use

Pre-issue QA. Before tendering, run this against every door / window / room / equipment schedule with the required-parameter set for the project's BEP. Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `schedule-name` | string | yes | Exact schedule view name. |
| `parameters` | array&lt;string&gt; | yes | Parameter names that must be present. |

## Output

```yaml
missing:
  - row-index: 17
    element-id: "12345"
    missing-parameters: ["Fire Rating"]
  - row-index: 42
    element-id: "12389"
    missing-parameters: ["Fire Rating","Hardware Set"]
```

## Worked example

```yaml
- id: qa-doors
  agent: revit-2026
  command: schedule.find-rows-missing
  inputs:
    schedule-name: "Door Schedule"
    parameters: ["Fire Rating","Acoustic","Hardware Set"]
- id: post-if-issues
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    channel-id: "{{ secrets.teams.qa }}"
    title: "Door schedule QA — {{ qa-doors.missing.length }} rows need attention"
    rows: "{{ qa-doors.missing }}"
    when: "{{ qa-doors.missing.length > 0 }}"
```

## Implementation note

Iterates the schedule's `TableData.GetSectionData(SectionType.Body)` row-by-row, joins back to the source element by `ElementId`, and reads each named parameter via `Element.LookupParameter`. Parameters that exist but are empty count as missing.

## See also

- `schedule.export` — get the whole schedule out
- `element.by-parameter-value` — same QA at the element level, not schedule
