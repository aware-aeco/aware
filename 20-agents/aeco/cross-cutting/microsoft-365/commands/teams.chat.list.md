# `microsoft-365.teams.chat.list`

List the signed-in person's Teams chats, most recent message first.

## When to use

Find a chat's id before reading it with `teams.chat.list-messages`.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `page-size` | int | no | Chats to return, 1 to 50 (default 25). |

## Output

```yaml
chats:
  - id: "19:2da4c29f6d7041eca70b638b43d45437@thread.v2"
    topic: "Acme Tower - steel"     # absent for one-on-one chats
    chat-type: "group"              # oneOnOne, group, meeting
    created-at: "2026-10-01T08:00:00Z"
    last-updated-at: "2026-11-02T10:00:00Z"
more-available: true
omitted: 0
```

Members are not listed yet (a follow-up); `chat-type` says what each chat is.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /me/chats?$top=N&$orderby=lastMessagePreview/createdDateTime desc`. A `@odata.nextLink` is
reported, never followed. Needs `User.Read` and `Chat.ReadBasic` (the code also accepts `Chat.Read`
or `Chat.ReadWrite`).
