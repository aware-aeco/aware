# `microsoft-365.outlook.mail.search`

Search the signed-in person's mailbox by words and/or sender.

## When to use

Find the mail a task depends on — the RFI response from the engineer, the client's approval, the
transmittal for an issued set — and hand its subject, sender, time and preview to the next step.

Read-only. Never the message body and never attachment bytes.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `text` | string | one of | 1 to 16 words of letters, digits, `.`, `_`, `-`, `@`, separated by spaces. |
| `from` | string | one of | One sender address, e.g. `client@acme.com`. |
| `page-size` | int | no | Messages to return, 1 to 50 (default 25). |

At least one of `text` and `from`; given both, a message must match all of them. These are typed
inputs, not a query language: AWARE writes the search itself. A word may not be `AND`, `OR`, `NOT`,
`NEAR` or `ONEAR`, start with `-`, or hold `:`, `*`, quotes or brackets — anything else is refused
before any request. Example: `text: "RFI 042"`, `from: "engineer@acme.com"`.

## Output

```yaml
messages:
  - id: "AAMkAG..."
    subject: "RE: RFI 042 - beam B12 connection"
    from: { address: "engineer@acme.com", name: "A. Engineer" }
    received-at: "2026-10-30T09:12:00Z"
    preview: "Please see attached sketch. The connection is..."
    has-attachments: true
    web-url: "https://outlook.office365.com/owa/?ItemID=..."
more-available: true   # Graph has another page, or entries were cut at page-size
omitted: 0             # messages dropped because Graph sent a malformed id
```

Results are one page of matches **sorted by sent date**, not by relevance. Microsoft returns at most
1,000 matches for a search. A cut text field is named in that message's `truncated`.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /me/messages` with
`$search="<terms>"` (the words, then `from:<address>`), `$top` from `page-size` and a fixed `$select`
(`bodyPreview`, never `body`). One page; `@odata.nextLink` is reported, never followed. Needs
`User.Read` and one of `Mail.Read`, `Mail.ReadWrite` (`Mail.ReadBasic` has no preview, so it is not
enough); connect with `aware connect microsoft-365 --oauth --scopes User.Read,Mail.Read`. None needs
tenant admin consent.
