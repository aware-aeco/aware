# `google-workspace.calendar.find-availability`

Find free time across attendees within a window using freebusy.

## When to use

The "schedule the next coordination meeting" primitive. Resolve when the design team, structural consultant, and MEP lead are all free before calling `calendar.create-event`.

**READ-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `attendees` | array<string> | yes | — | Email addresses to check. |
| `window-start` | string | yes | — | ISO 8601 start of search window. |
| `window-end` | string | yes | — | ISO 8601 end of search window. |
| `duration-minutes` | number | no | `30` | Length of slot to find. |

## Output

```yaml
suggestions:
  - start: string
    end:   string
```

## Worked example

```yaml
- id: find-slot
  agent: google-workspace
  command: calendar.find-availability
  inputs:
    attendees:
      - arch@acme.com
      - struct@consultant.com
    window-start: "2026-05-21T09:00:00+01:00"
    window-end:   "2026-05-21T17:00:00+01:00"
    duration-minutes: 60
```

## Implementation note

Calls `POST freebusy.query` (Calendar v3) over the attendees' calendars; gaps in the returned busy blocks long enough for `duration-minutes` become suggestions. Read scope: `calendar.readonly` (or broader `calendar`). Visibility depends on each attendee sharing free/busy. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`calendar.create-event`](./calendar.create-event.md)
- [`meet.create`](./meet.create.md)
- [`gmail.send`](./gmail.send.md)
