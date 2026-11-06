# `microsoft-365.outlook.mail.folder.list-message-ids`

List what one Outlook mail folder holds now, within a received-date window: each message's id and
dates, and nothing else.

## When to use

Keep a copy of a folder: list the window, read new or changed messages with
`outlook.mail.get-message`, and treat a message that is no longer listed as gone from the folder.
Unlike `outlook.mail.folder.list-messages` (the change stream), this is the folder's current
content, so it needs no cursor and is complete once every page of the window is read.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `folder-id` | string | yes | The folder's `id`, from `outlook.mail.folder.list`. Not a well-known name. |
| `received-after` | string | yes | The window's start, a UTC timestamp (`2026-08-08T00:00:00Z`). Included. |
| `received-before` | string | no | The window's end, a UTC timestamp. Included. Omit it for the newest page. Never earlier than `received-after`. |
| `page-size` | int | no | Messages per page, 1 to 1000 (default 500). |
| `skip` | int | no | Messages to skip within the window, 0 to 100,000 (default 0). Use it only to page past messages sharing one received time (below). |

## Output

```yaml
messages:
  - id: "AAkALgAAAAAAHYQDEapmEc2…"
    received-at: "2026-11-06T08:00:00Z"
    last-modified-at: "2026-11-06T09:30:00Z"
    draft: false
more-available: true     # more of the window remains
omitted: 0               # messages dropped because Graph sent a malformed id
```

- **Newest received first.** Messages come in descending `received-at`.
- **Paging.** While `more-available` is true, call again with the same `received-after` and with
  `received-before` set to the **oldest** `received-at` of this page. That message, and any other
  with the same `received-at`, comes back again, because the bound is included: keep each id once.
- **Messages sharing one received time.** When a full page has a single `received-at` T, the date
  key cannot move past it. List the window of that one instant (`received-after` = `received-before`
  = T) with `skip` 0, then `page-size`, then 2 × `page-size`, … until `more-available: false`. Then
  continue below T with `received-before` = T minus 100 ns (`.0000001` seconds, the finest unit a
  timestamp here carries), which no longer includes T.
  `skip` pages by position, so a message deleted or moved during that walk can shift another out of
  it; only a later listing catches that.
- **Complete or not.** A window is complete when a page answers `more-available: false` and every
  page had `omitted: 0`. Otherwise, some messages in the window were not listed.
- **Dates are exact or withheld.** `received-at` and `last-modified-at` are real UTC instants that
  can be sent back as a bound. A date Graph sent malformed is withheld (`received-at-withheld:
  true`), never passed on. A date Graph did not send is absent, and one it sent as `null` is `null`.
- **Ids are exact and immutable.** An `id` is kept byte for byte, or the message is dropped and
  counted in `omitted`. Ids are the ones `outlook.mail.get-message` reads, and they stay the same
  when a message moves between folders.
- **The window is on the received date.** Mail moved into the folder is listed by when it was
  received, not when it was moved.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#763):
`GET /me/mailFolders/{folder-id}/messages?$filter=receivedDateTime ge A[ and receivedDateTime le B]&$top=N&$select=id,receivedDateTime,lastModifiedDateTime,isDraft&$orderby=receivedDateTime desc`,
with `Prefer: IdType="ImmutableId"`. A raw answer over 8 MiB fails with `E_CALL_OUTPUT_TOO_LARGE`,
`reason: raw-too-large`. Needs `User.Read` and one of `Mail.ReadBasic`, `Mail.Read` (requested) or
`Mail.ReadWrite`.
