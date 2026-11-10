# `google-workspace.gmail.search`

Search the signed-in person's Gmail by words, sender and/or label.

## When to use

Find the mail a task depends on — the engineer's RFI answer, the client's approval, the
transmittal for an issued set — and hand its subject, sender, time and preview to the next step.

Read-only. Never the message body and never attachment bytes. Spam and Trash are never searched.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `text` | string | one of | 1 to 16 words of letters, digits, `.`, `_`, `-`, `@`, separated by spaces, each at most 64 characters. |
| `from` | string | one of | One sender address, e.g. `client@acme.com`. |
| `label` | string | one of | One Gmail label id, e.g. `INBOX`, `STARRED`, `Label_12` (1 to 64 letters, digits, `_`, `-`). |
| `page-size` | int | no | Messages to return, 1 to 50 (default 25). |

At least one of `text`, `from` and `label`; given more than one, a message must match all of them.
These are typed inputs, not Gmail's search language: AWARE writes the search itself. A word may not
be `OR`, `AND`, `AROUND` or `NOT` (in any case), start with `-`, or hold `:`, `*`, quotes or
brackets — anything else is refused before any request. A label is a label **id**, not its display
name; `SPAM` and `TRASH` are refused. Example: `text: "RFI 042"`, `from: "engineer@acme.com"`.

## Output

```yaml
messages:
  - id: "18c1a2b3c4d5e6f7"
    thread-id: "18c1a2b3c4d5e6f7"
    subject: "RE: RFI 042 - beam B12 connection"
    from: { address: "engineer@acme.com", name: "A. Engineer" }
    received-at: "2026-10-30T09:12:00.000Z"
    preview: "Please see attached sketch. The connection is..."
    label-ids: ["INBOX", "UNREAD", "Label_12"]
    web-url: "https://mail.google.com/mail/?authuser=ada%40example.com#all/18c1a2b3c4d5e6f7"
more-available: true   # Gmail has another page, or entries were cut at page-size
omitted: 0             # messages dropped: a malformed or repeated id, or deleted before it was read
```

Results are one page in Gmail's order (newest first). Every field but `id` may be missing when Gmail
did not send it or sent something AWARE cannot read; a field AWARE had to cut or leave out is named
in that message's `truncated`. A message with several senders keeps the first in `from` and gives
the count in `from-count`. `received-at` is Gmail's internal date, in UTC with milliseconds.
`web-url` is a convenience link: it opens the message in whichever browser session holds the
connected account, and is left out when Google has not verified that account's email address. It is
never an identity. The list and the per-message reads are not one snapshot: a message's labels can
change, or the message be deleted, between them.

## Implementation note

Runs in-process through AWARE's code-owned Gmail read: one
`GET /gmail/v1/users/me/messages` with `q` (the sender as `from:<address>`, then the words),
`labelIds`, `maxResults` from `page-size` and `includeSpamTrash=false`; then one
`GET .../messages/<id>?format=metadata` per listed message for its From, Subject and Date headers.
One page; `nextPageToken` is reported, never followed. Both steps run under one deadline, one output
budget and one access token, for the one verified Google account; the slot is checked before every
request, so a disconnect or reconnect stops the call before the next one (a request already in
flight cannot be recalled) and nothing partial is returned. A message deleted between the two steps
is counted in `omitted`; any other failure fails the call.

Needs `gmail.readonly`, which is opt-in: connect with
`aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/gmail.readonly`.
A slot connected that way still sends mail. `gmail.readonly` is a Google *restricted* scope: the
Google project of the app that signs the person in must be verified for it (see
`10-core/oauth-registration.md`).

## See also

- [`gmail.send`](./gmail.send.md)
- [`list-files`](./list-files.md)
