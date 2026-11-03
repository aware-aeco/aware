# `microsoft-365.teams.chat.list-messages`

Read the messages of one Teams chat changed after a given time, as plain text.

## When to use

See what a chat discussed recently, to summarise it or act on it.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `chat-id` | string | yes | The chat's id (`19:…`), from `teams.chat.list`. |
| `since` | string | yes | UTC time, `YYYY-MM-DDTHH:MM:SSZ` (optional `.fff`). Messages changed strictly after it are returned. |
| `page-size` | int | no | Messages to return, 1 to 50 (default 20). |

## Output

```yaml
messages:
  - id: "1730541600000"
    created-at: "2026-11-02T10:00:00Z"
    last-modified-at: "2026-11-02T10:00:00Z"
    message-type: "message"          # also systemEventMessage, chatEvent, …
    sender: { kind: "user", id: "fbab97d0-…", display-name: "Anna Nowak" }
    text: "Grid C beam reaction is 120 kN, see attached calc."
    attachments:
      - id: "153fa47d-…"
        name: "calc.pdf"
        content-type: "reference"
        url: "https://contoso.sharepoint.com/sites/Project/Shared%20Documents/calc.pdf"
    attachments-omitted: 0
more-available: false    # Graph has another page, or AWARE cut the page
omitted: 0               # messages dropped because Graph sent a malformed id
```

- **`text` is plain text.** HTML is reduced to its text in AWARE (scripts and styles dropped, line
  breaks kept, cut at 4096 chars and named in `truncated`). Render it as text: a `<b>` in it is
  literal.
- **Attachments are references.** `url` is kept only for a plain SharePoint address (`https`, no
  query, no fragment, no port, no credentials, not a Graph hosted-content URL); otherwise
  `url-withheld: true`. A card, a forwarded message or any other payload is never returned:
  `content-withheld: true`. AWARE never downloads an attachment.
- **Sender.** `kind` is `user`, `application` or `unknown`; `sender-withheld: true` when Microsoft
  sent a sender AWARE could not accept.
- **Deleted** messages stay, with `deleted: true` and empty `text`; system messages stay, with their
  `message-type`.

**One page, newest first.** `more-available: true` means there may be more since `since`.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /chats/{chat-id}/messages?$filter=lastModifiedDateTime gt <since>&$top=N&$orderby=lastModifiedDateTime desc`
with `Prefer: include-unknown-enum-members`. AWARE renders the filter from the typed `since`; no
caller-written OData reaches Graph. Needs `User.Read` and `Chat.Read` (the code also accepts
`Chat.ReadWrite`).
