# `microsoft-365.teams.channel.post-with-screenshot`

Post a message to a Teams channel with an attached image.

## When to use

The designer-Monday-shot + architect-RFI-with-screenshot primitive — when a picture carries the message: a rendered view, a marked-up clash, a sheet thumbnail. Pairs naturally with any agent that exports a PNG. Image must be ≤ 4 MB (Graph hosted-content limit).

**WRITE-mode** (external state — Teams). Declare `safety:` on the node; for a notification, `transaction-group: notify, snapshot: false` is honest.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `team-id` | string | yes | Team id OR team display name (auto-resolved). |
| `channel-id` | string | yes | Channel id OR channel display name. |
| `text` | string | yes | Message body accompanying the image. |
| `screenshot-path` | string | yes | Path to the image file (≤ 4 MB). |

## Output

```yaml
message-id: "1715958331234"
web-url:    "https://teams.microsoft.com/l/message/19:abc.../1715958331234?..."
```

## Worked example

```yaml
- id: render
  agent: rhino-8
  command: view.export-png
  inputs:
    view: "Perspective"
- id: shot
  agent: microsoft-365
  command: teams.channel.post-with-screenshot
  inputs:
    team-id: "Project Acme"
    channel-id: "design"
    text: "Latest massing study — feedback by Thu."
    screenshot-path: "{{ render.png-path }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /teams/{team-id}/channels/{channel-id}/messages` with the image embedded as a `hostedContents` entry referenced from the message body's `<img>` tag. Team/channel display names resolve via `/teams?$filter=displayName eq '...'`. Requires `ChannelMessage.Send`; provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `teams.channel.post-message` — plain text
- `teams.channel.post-with-card` — structured data as an adaptive card
- `outlook.mail.send-with-attachment` — image as an email attachment
