# `microsoft-365.teams.channel.list-messages`

Read the recent threads of one Teams channel, as plain text.

## When to use

See what a project channel discussed since a given time — decisions, questions, who asked what — to
summarise it or act on it.

Read-only. Work or school accounts only. **Needs a tenant administrator's consent** for
`ChannelMessage.Read.All`; AWARE says so and never asks for it on anyone's behalf.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `team-id` | string | yes | The team's id, a GUID, from `teams.team.list`. |
| `channel-id` | string | yes | The channel's id (`19:…@thread.tacv2`), from `teams.channel.list`. |
| `since` | string | yes | UTC time, `YYYY-MM-DDTHH:MM:SSZ` (optional `.fff`). Threads with activity strictly after it are kept. |
| `page-size` | int | no | Threads to ask for, 1 to 50 (default 20). |

A thread's activity is the latest time of its first message and of every reply Microsoft sent. A
thread whose activity cannot be known (Microsoft has more replies than it sent, or a time is
unreadable) is **kept** with `activity-unknown: true`, never treated as old.

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
    replies: [ … same shape, without replies … ]
    replies-omitted: 0               # replies received and not returned (beyond 50, or malformed)
    replies-more-available: false    # Microsoft has more replies, or AWARE cut them
more-available: false                # Graph has another page, or AWARE cut the page
omitted: 0                           # threads dropped because Graph sent a malformed id
filtered-before-since: 3             # threads received whose activity is not after `since`
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

**One page.** Graph lists a channel's threads newest activity first; this read takes one page and
never follows a continuation. `more-available: true` means there may be more — never that everything
since `since` was returned.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /teams/{team-id}/channels/{channel-id}/messages?$top=N&$expand=replies` with
`Prefer: include-unknown-enum-members`. Graph documents no `$filter` here, so `since` is applied by
AWARE. Needs `User.Read` and `ChannelMessage.Read.All` (the code also accepts `Group.Read.All` or
`Group.ReadWrite.All`; all three need admin consent).
