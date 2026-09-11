# `google-workspace.meet.create`

Create a standalone Google Meet link (not tied to a calendar event).

## When to use

Ad-hoc coordination — "we need a quick model walk-through; here's the link." When the call should sit on a calendar invite instead, use `calendar.create-event` with `add-meet-link: true`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | string | no | Optional meeting title (defaults to "AWARE meeting"). |

## Output

```yaml
meet-id:  string
meet-url: string
```

## Worked example

```yaml
- id: quick-call
  agent: google-workspace
  command: meet.create
  inputs:
    title: "Grid C/4 walk-through"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

A truly standalone link is created via the Meet REST API `spaces.create` (scope `meetings.space.created`), which returns a `meetingUri`. Where that API is unavailable, the practical fallback is a Calendar event carrying `conferenceData.createRequest` and reading back its Meet URL (scope `calendar`). The exact path depends on Workspace edition and API enablement — confirm before relying on either. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`calendar.create-event`](./calendar.create-event.md)
- [`calendar.find-availability`](./calendar.find-availability.md)
- [`chat.post-message`](./chat.post-message.md)
