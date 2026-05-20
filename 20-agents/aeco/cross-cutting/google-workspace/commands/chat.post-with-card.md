# `google-workspace.chat.post-with-card`

Post a Card v2 message to a Google Chat space.

## When to use

Rich, structured digests in the coordination space — the Google-side equivalent of `microsoft-365.teams.channel.post-with-card`. Renders labelled rows (clash counts, RFI status) as a key-value list with optional action buttons.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `space-id` | string | yes | Space resource name (e.g. `spaces/abc`). |
| `title` | string | yes | Card header. |
| `rows` | array | no | Each `{ label, value }` renders as a KeyValue widget. |
| `card` | object | no | Raw Card v2 JSON. If provided, `rows` is ignored. |

## Output

```yaml
message-name: "spaces/abc/messages/def"
web-url:      "https://chat.google.com/room/abc/def"
```

## Worked example

```yaml
- id: post-digest
  agent: google-workspace
  command: chat.post-with-card
  inputs:
    space-id: "{{ secrets.coord-space-id }}"
    title: "Monday clash digest"
    rows:
      - { label: "Unresolved", value: "14" }
      - { label: "New this week", value: "3" }
      - { label: "Model", value: "{{ run.model }}" }
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST spaces.messages.create` with a `cardsV2` payload built from `rows` (or passed through verbatim when `card` is supplied). See the v2 card schema (https://developers.google.com/chat/api/guides/v2-migration). Write scope: `chat.messages`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`chat.post-message`](./chat.post-message.md)
- [`chat.post-with-screenshot`](./chat.post-with-screenshot.md)
- [`sheets.row.append`](./sheets.row.append.md)
