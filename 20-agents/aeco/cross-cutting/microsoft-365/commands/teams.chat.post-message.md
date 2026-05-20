# `microsoft-365.teams.chat.post-message`

Post a message to a 1:1 or group Teams chat.

## When to use

When the recipient is a person (or small ad-hoc group), not a project channel — a direct "your model audit is done" ping, a quick nudge to the ball-in-court engineer. Use `teams.channel.post-message` instead when the message belongs in a shared project channel. Returns the `message-id` for reply threading.

**WRITE-mode** (external state — Teams). Declare `safety:` on the node; for a pure ping, `transaction-group: notify, snapshot: false` is honest — there's nothing to snapshot.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `chat-id` | string | yes | — | Chat id, or `me` to use the user's default 1:1. |
| `text` | string | yes | — | Plain text or HTML (set `content-type` accordingly). |
| `content-type` | enum | no | `text` | `text` / `html`. |

## Output

```yaml
message-id: "1715958331234"
web-url:    "https://teams.microsoft.com/l/message/19:abc.../1715958331234?..."
```

## Worked example

```yaml
- id: ping-engineer
  agent: microsoft-365
  command: teams.chat.post-message
  inputs:
    chat-id: "{{ secrets.coordination-chat-id }}"
    content-type: html
    text: "<p>Your peer-review delta is ready — 12 changed members.</p>"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /chats/{chat-id}/messages`. Microsoft Graph caps a chat-message POST at **~1 request/second per user** for a given chat (higher per-app/per-tenant ceilings apply); the transport honors the `Retry-After` header on HTTP 429. Requires the `ChatMessage.Send` scope; provision via `aware connect microsoft-365`.

## See also

- `teams.channel.post-message` — post to a shared channel instead
- `teams.channel.post-with-screenshot` — attach an image
- `outlook.mail.send` — the email equivalent
