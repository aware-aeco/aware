# `google-workspace.gmail.list-message-ids`

List the ids of the messages received in one time window, by label, words and/or sender, a page
at a time.

## When to use

Sync a Gmail label (or a search) as a Knowledge source: list the window — say the last 90 days —
page by page, then read each message with [`gmail.get-message`](./gmail.get-message.md).

Read-only. Ids only: no dates, subjects or bodies. Spam and Trash are never listed.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | string | one of | One label id from [`gmail.labels.list`](./gmail.labels.list.md), e.g. `Label_12` (1 to 64 letters, digits, `_`, `-`). `SPAM` and `TRASH` are refused. |
| `text` | string | one of | 1 to 16 words, the [`gmail.search`](./gmail.search.md) rule. |
| `from` | string | one of | One sender address. |
| `received-after` | string | yes | The window's start: an exact UTC instant, `YYYY-MM-DDTHH:MM:SS[.fraction]Z`, at or after `1970-01-01T00:00:01Z`. |
| `received-before` | string | no | The window's end, same form; earlier than `received-after` is refused. |
| `page-size` | int | no | Ids per page, 1 to 500 (default 500). |
| `page-token` | string | no | The previous page's `next-page-token`, unchanged (1 to 1024 printable ASCII characters, no space). Send the same other inputs with it. |

At least one of `label`, `text` and `from`; a date window alone is refused. Given more than one, a
message must match all of them. Anything outside a rule is refused before any request.

### How the window is sent

Gmail's `after:` and `before:` take whole epoch seconds. AWARE widens the window by one second at
each end so it holds every message in your bounds whatever Gmail's meaning at the boundary second:

- `received-after` → `after:<floor(epoch seconds) − 1>`
- `received-before` → `before:<ceil(epoch seconds) + 1>`

Example: `received-after: 2026-08-17T00:00:00Z` sends `after:1786924799`;
`received-before: 2026-11-15T12:00:00.250Z` sends `before:1794744002`.

The listing is therefore a **superset** of the window and carries no dates: post-filter on the
`received-at` that `gmail.get-message` returns (Gmail's `internalDate`). A message whose
`received-at` is missing cannot be post-filtered; say so rather than guess. The exact boundary
meaning Gmail applies is to be measured by the live check and recorded here.

## Output

```yaml
messages:
  - id: "18c1a2b3c4d5e6f7"
    thread-id: "18c1a2b3c4d5e6f7"
next-page-token: "09876543210987654321"   # send it back as page-token for the next page
more-available: true
omitted: 0            # entries dropped: a malformed or repeated id
```

Ids come in Gmail's order (newest first). Every entry Gmail sent is returned or counted in
`omitted` — the page is never cut, so a page token never skips a message. A missing or malformed
thread id leaves `thread-id` out and names it in that entry's `truncated`. `more-available` is
true when Gmail named another page; when its token cannot be sent back (outside the token rule)
it is withheld (`next-page-token-withheld: true`) rather than dropped. Stop paging on a withheld or
repeated token.

## Failures

`E_CALL_FAILED` carries `status`, `phase` and, where it can be told, a `reason` — never Google's
text:

| `reason` | When | What to do |
|---|---|---|
| `rate-limited` | a 429, or a 403 whose Google reason is `rateLimitExceeded`/`userRateLimitExceeded`; `retry-after-secs` when Google sent `Retry-After` | wait and retry |
| `label-not-found` | the listing answered 400/404 and the label, read again (`GET …/labels/{label}`), is gone | the source's label was deleted or renamed away |
| `page-token-invalid` | the listing answered 400/404, a `page-token` was sent and the label (if any) still exists | restart the window without the token; if that fails too it reads `request-invalid` |
| `request-invalid` | any other 400/404 | report it |

These are evidence, not proof: the label read is a second request, made under the same deadline
and slot check. Its own rate limit is `rate-limited` with `phase: "label-probe"`. A plain 403 is
the status alone (a permission problem, not a rate limit). A raw answer over 256 KiB is
`E_CALL_OUTPUT_TOO_LARGE` (`raw-too-large`); an answer over `maxOutputBytes`, `output-too-large`.

## Implementation note

One `GET https://gmail.googleapis.com/gmail/v1/users/me/messages` with `q` (the sender as
`from:<address>`, the words, then `after:`/`before:`), `labelIds`, `maxResults`, `pageToken` and
`includeSpamTrash=false`, no redirects, for the one verified Google account, the slot checked
before every request. Needs `gmail.readonly`, opt-in:
`aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/gmail.readonly`.

## See also

- [`gmail.get-message`](./gmail.get-message.md)
- [`gmail.labels.list`](./gmail.labels.list.md)
- [`gmail.search`](./gmail.search.md)
