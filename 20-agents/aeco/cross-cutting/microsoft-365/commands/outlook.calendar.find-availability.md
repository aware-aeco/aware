# `microsoft-365.outlook.calendar.find-availability`

Find free time across a set of attendees within a window.

## When to use

The "schedule a coordination meeting for next week" primitive — given the project team and a window, return slots where everyone is free. Feed the first suggestion straight into `outlook.calendar.create-event`.

Read-only.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `attendees` | array<string> | yes | — | Email addresses to check. |
| `window-start` | string | yes | — | ISO 8601 start of the search window. |
| `window-end` | string | yes | — | ISO 8601 end of the window. |
| `duration-minutes` | number | no | `30` | Required length of the slot. |

## Output

```yaml
suggestions:
  - start: "2026-05-25T10:00:00+01:00"
    end:   "2026-05-25T11:00:00+01:00"
  - start: "2026-05-26T14:00:00+01:00"
    end:   "2026-05-26T15:00:00+01:00"
```

## Worked example

```yaml
- id: slots
  agent: microsoft-365
  command: outlook.calendar.find-availability
  inputs:
    attendees: ["arch@acme.com","struct@acme.com","mep@acme.com"]
    window-start: "{{ next.monday }}T09:00:00+01:00"
    window-end: "{{ next.friday }}T17:00:00+01:00"
    duration-minutes: 60
- id: invite
  agent: microsoft-365
  command: outlook.calendar.create-event
  inputs:
    subject: "Weekly coordination — {{ project.name }}"
    start: "{{ slots.suggestions[0].start }}"
    end: "{{ slots.suggestions[0].end }}"
    time-zone: "Europe/London"
    attendees: ["arch@acme.com","struct@acme.com","mep@acme.com"]
    online-meeting: true
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /me/findMeetingTimes`, passing the attendees and the window as a `timeConstraint` with the requested `meetingDuration`. (For raw free/busy without ranking, `POST /me/calendar/getSchedule` is the alternative.) Requires `Calendars.Read` (or `Calendars.ReadWrite`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `outlook.calendar.create-event` — book the chosen slot
- `outlook.mail.send` — send a pre-meeting briefing
