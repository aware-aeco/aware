# `google-workspace.calendar.create-event`

Create a Google Calendar event.

## When to use

Schedule coordination meetings, design reviews, RFI walkthroughs in Google-shop firms. Pair with `calendar.find-availability` first.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `calendar-id` | string | no | Defaults to `primary`. |
| `summary` | string | yes | Event title. |
| `description` | string | no | |
| `start` | string | yes | ISO 8601 with offset. |
| `end` | string | yes | |
| `time-zone` | string | yes | IANA name (e.g. `America/New_York`). |
| `attendees` | array<string> | no | Email addresses. |
| `location` | string | no | |
| `add-meet-link` | boolean | no | Default `false`. If `true`, Calendar attaches a Google Meet link. |

## Worked example

```yaml
- id: invite
  agent: google-workspace
  command: calendar.create-event
  inputs:
    summary: "Weekly coordination"
    start: "2026-05-20T14:00:00-05:00"
    end:   "2026-05-20T15:00:00-05:00"
    time-zone: "America/New_York"
    attendees:
      - arch@acme.com
      - struct@acme.com
      - mep@acme.com
    add-meet-link: true
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `events.insert` with `conferenceData.createRequest` populated when `add-meet-link: true`.
