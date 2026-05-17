# `microsoft-365.outlook.calendar.create-event`

Create an Outlook / Exchange calendar event.

## When to use

When an app's output is "schedule a meeting" — coordination meetings, design reviews, site walks, RFI walkthroughs. Pairs with `outlook.calendar.find-availability` to pick a slot.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `subject` | string | yes | |
| `start` | string | yes | ISO 8601 with offset, e.g. `2026-05-20T14:00:00+01:00`. |
| `end` | string | yes | Same shape. |
| `time-zone` | string | yes | IANA name (e.g. `Europe/London`). Explicit, not inferred. |
| `attendees` | array<string> | no | Email addresses. |
| `location` | string | no | |
| `online-meeting` | boolean | no | If `true`, Teams auto-creates the meeting link. |

## Worked example

```yaml
- id: schedule
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
    start: "{{ schedule.suggestions[0].start }}"
    end: "{{ schedule.suggestions[0].end }}"
    time-zone: "Europe/London"
    attendees: ["arch@acme.com","struct@acme.com","mep@acme.com"]
    online-meeting: true
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /me/events`. `start.timeZone` and `end.timeZone` are populated from `time-zone`. If `online-meeting: true`, the event includes `{ "isOnlineMeeting": true, "onlineMeetingProvider": "teamsForBusiness" }`.

## See also

- `outlook.calendar.find-availability` — find a slot first
- `outlook.mail.send` — pre-meeting briefing email
