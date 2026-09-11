# `google-workspace.chat.post-message`

Post a message to a Google Chat space.

## When to use

Internal team notifications in Google-shop firms. Same role as `microsoft-365.teams.channel.post-message` on the M365 side.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `space-id` | string | yes | Space resource name (e.g. `spaces/abc`). |
| `text` | string | yes | Plain text body. |
| `card` | object | no | Optional Card v2 JSON; `text` becomes fallback. |

## Output

```yaml
message-name: "spaces/abc/messages/def"
web-url:      "https://chat.google.com/room/abc/def"
```

## Worked example

```yaml
- id: digest
  agent: google-workspace
  command: chat.post-message
  inputs:
    space-id: "{{ secrets.coord-space-id }}"
    text: "Monday clash digest — 14 unresolved (see Drive)."
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST chat.spaces.messages.create`. Cards use the v2 schema (https://developers.google.com/chat/api/guides/v2-migration).
