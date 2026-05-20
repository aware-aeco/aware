# `google-workspace.chat.post-with-screenshot`

Post a Chat message with an attached image.

## When to use

Drop a clash-detection screenshot, a model view, or a marked-up snippet into the coordination space alongside a short note. The visual sibling of `chat.post-message`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `space-id` | string | yes | Space resource name (e.g. `spaces/abc`). |
| `text` | string | yes | Caption / note. |
| `screenshot-path` | string | yes | Local path to the image to attach. |

## Output

```yaml
message-name: "spaces/abc/messages/def"
web-url:      "https://chat.google.com/room/abc/def"
```

## Worked example

```yaml
- id: post-clash-shot
  agent: google-workspace
  command: chat.post-with-screenshot
  inputs:
    space-id: "{{ secrets.coord-space-id }}"
    text: "Grid C/4 — beam clashes duct. See view."
    screenshot-path: "{{ render-clash.path }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

The image is uploaded first (Drive `files.create` or the Chat media upload endpoint), then referenced from a `POST spaces.messages.create` call. Write scope: `chat.messages` (plus a Drive write scope if the image is staged in Drive). Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`chat.post-message`](./chat.post-message.md)
- [`chat.post-with-card`](./chat.post-with-card.md)
- [`upload-file`](./upload-file.md)
