# `google-workspace.gmail.get-message`

Read one Gmail message: its labels, time, people, subject, text and attachment list.

## When to use

The second half of a Gmail Knowledge source: for each id
[`gmail.list-message-ids`](./gmail.list-message-ids.md) returns, read the message and keep its
text. Read-only. **Never returns attachment content** — attachments are listed by name, type and
size.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | yes | A message id (1 to 64 letters and digits). |

## Output

```yaml
id: "18c1a2b3c4d5e6f7"
thread-id: "18c1a2b3c4d5e6f7"
label-ids: ["INBOX", "Label_12"]
label-ids-complete: true
received-at: "2026-10-30T09:12:00.000Z"
from: { address: "engineer@acme.com", name: "A. Engineer" }
to: [{ address: "ada@example.com", name: "Ada" }]
cc: []
recipients-omitted: 0
subject: "RE: RFI 042 - beam B12 connection"
text: "Please see the attached sketch.\nThe connection is..."
text-source: "text/plain"     # text/plain, text/html (converted) or mixed
text-state: "complete"
attachments:
  - { name: "B12-sketch.pdf", type: "application/pdf", size: 48213 }
attachments-omitted: 0
attachments-state: "complete"
```

- **`label-ids` is always present.** `label-ids-complete: false` means Gmail's value was not a
  list, held an id AWARE could not read, or had more than 500 ids — so a label missing from it is
  not evidence that the label was removed. (An absent `labelIds` is Google's form of an empty
  list: `[]`, complete.)
- **`received-at`** is Gmail's `internalDate`, RFC 3339 UTC with milliseconds. When it is missing
  or unreadable it is left out and named in `truncated`.
- **People.** `from` is the first sender (`from-count` when there are several); `to` and `cc` are
  always present, at most 100 each, the rest counted in `recipients-omitted`. Headers are decoded
  (RFC 2047). A value cut or unreadable is named in `truncated`.
- **Caps.** Subject 1,024 characters; address 320; name 256; text 65,536 (cut text is named in
  `truncated` and `text-state: truncated`); attachment name 1,024 and type 256 (cut ones named in
  that attachment's `truncated`); at most 50 attachments (the rest in `attachments-omitted`).

### Which text is returned

The walk is deterministic, in body order, at most 16 levels deep and 256 parts:

1. **An attachment is never read for text and its subtree is never entered**: a part with a file
   name, one that says `Content-Disposition: attachment`, and every enclosed message
   (`message/rfc822`). It is listed in `attachments`. So is any other non-text leaf (an inline
   image, a calendar part).
2. A `text/plain` or `text/html` part is one section.
3. `multipart/alternative` takes its first child whose sections are all `text/plain`, else its
   first child with any section — the plain-text version when there is one, the HTML version
   otherwise. So a typical message gives its `text/plain`; an HTML-only one gives its HTML.
4. `multipart/related` and `multipart/signed` take their first child (the rest are its inline
   resources or its signature).
5. `multipart/mixed` (and any other multipart) joins its children's sections in order.

HTML becomes text by parsing it — nothing in it runs, and nothing it points to is fetched. Body
data is base64url; its charset (from the part's `Content-Type`) is decoded, UTF-8 when none.

`text-state` is the first that applies:

| `text-state` | Meaning |
|---|---|
| `mime-cut` | the walk stopped at its depth or part limit (`parts` is named in `truncated`, `attachments-state: partial`); the text is what was reached |
| `external-not-fetched` | a section is stored apart (`attachmentId`) and could not be read: gone since, over 4 MiB, an unusable id, or past 4 such reads |
| `undecodable` | a section's data or charset could not be decoded |
| `truncated` | the text was cut at 65,536 characters |
| `none` | the message has no body section to read (only attachments, or nothing) |
| `complete` | every section was read in full |

Whatever sections were read are returned in `text`, whatever the state.

## Failures

`E_CALL_FAILED` carries `status`, `phase` and, where it can be told, a `reason` — never Google's
text. A 404 on the message is `reason: "gone"` (deleted, or not this account's). A rate limit is
`reason: "rate-limited"` (with `retry-after-secs` when Google sent `Retry-After`) on the message
read (`phase: "message"`) or a body read (`phase: "body"`). A body read's 404 or over-cap answer
only marks `external-not-fetched`; any other body-read failure fails the call. An answer for
another id is `reason: id-mismatch`. A raw answer over 8 MiB is `E_CALL_OUTPUT_TOO_LARGE`
(`raw-too-large`); an answer over the request's `maxOutputBytes` is `output-too-large` with
`outputBytes` — retry with a larger budget.

## Implementation note

`GET https://gmail.googleapis.com/gmail/v1/users/me/messages/{id}?format=full`; for a body section
stored apart, `GET …/messages/{id}/attachments/{attachmentId}` (never for an attachment). No
redirects, one deadline for every request, the one verified Google account, the slot checked
before every request; nothing partial is returned. `format=full` can carry a small attachment's
data inline: AWARE never decodes or returns it. Needs `gmail.readonly`, opt-in:
`aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/gmail.readonly`.

## See also

- [`gmail.list-message-ids`](./gmail.list-message-ids.md)
- [`gmail.labels.list`](./gmail.labels.list.md)
