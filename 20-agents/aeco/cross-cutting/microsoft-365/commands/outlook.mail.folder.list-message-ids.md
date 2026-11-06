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
next-skip: 500           # only while more-available: the skip of the next page
omitted: 0               # messages dropped because Graph sent a malformed id
```

`next-skip` is present only while `more-available` is true. It is Graph's own offset for the next
page when Graph names one past the `skip` sent: a mailbox scan can pass over items that are not
returned, so it can be more than this page's count. Otherwise it is the `skip` sent plus every entry
Graph returned, the omitted ones included, which can repeat a message but never skips one. When it
would pass 100,000, it is left out and `limit-reached: true` is set instead. When a page returned
nothing and Graph named no later offset, neither is set: `more-available` without either means the
page cannot be continued by `skip`, and the window is not complete.

- **Newest received first.** Messages come in descending `received-at`.
- **Paging.** While `more-available` is true, call again with the same `received-after` and with
  `received-before` set to the **oldest** `received-at` of this page. That message, and any other
  with the same `received-at`, comes back again, because the bound is included: keep each id once.
- **Messages sharing one received time.** When the oldest `received-at` of a page equals the
  `received-before` you sent (T), the date key did not move: every message on the page has that
  received time, whether the page is full or short. List the window of that one instant
  (`received-after` = `received-before` = T) with `skip` 0, then each page's `next-skip`, until
  `more-available: false`. Then continue below T with `received-before` = T minus 100 ns
  (`.0000001` seconds, the finest unit a timestamp here carries), which no longer includes T. When
  T is `received-after` itself, the window is finished instead: nothing is left below it.
  Always page by `next-skip`, never by a multiple of `page-size`: Graph can send fewer entries than
  asked, and a page size can change between calls.
  A walk that ends at `limit-reached` has not listed every message at T, so the window is not
  complete. `skip` pages by position, so a message deleted or moved during that walk can shift
  another out of it; only a later listing catches that.
- **Complete or not.** A window is complete when a page answers `more-available: false`, every
  page had `omitted: 0`, and no walk ended at `limit-reached`. Otherwise, some messages in the
  window were not listed.
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
