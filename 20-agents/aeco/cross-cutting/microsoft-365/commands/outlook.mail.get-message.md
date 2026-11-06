# `microsoft-365.outlook.mail.get-message`

Read one Outlook message: its headers, folder and version, its body as plain text, and its
attachments by name, type and size.

## When to use

Read the body of a message found with `outlook.mail.folder.list-messages` or
`outlook.mail.search`, to summarise it, cite it or keep a copy of it.

Read-only. Never attachment bytes.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | yes | The message's `id`, from `outlook.mail.folder.list-messages`. |

## Output

```yaml
id: "AAkALgAA…"
kind: "message"
internet-message-id: "<CAF1…@mail.acme.com>"
conversation-id: "AAQkAGI2…"
subject: "RFI 017 - beam B12 bearing"
from: { address: "anna@acme.com", name: "Anna Nowak" }
to: [ { address: "bob@acme.com", name: "Bob Lee" } ]
cc: []
recipients-omitted: 0
received-at: "2026-11-06T08:00:00Z"
sent-at: "2026-11-06T07:59:58Z"
last-modified-at: "2026-11-06T08:00:01Z"
draft: false
has-attachments: true
preview: "Please confirm the bearing length at B12…"
web-url: "https://outlook.office365.com/owa/?ItemID=…"
parent-folder-id: "AAMkAGI2…AAA="
change-key: "CQAAABYAAAD…"
text: "Please confirm the bearing length at B12. See the attached sketch.\n\nAnna"
unique-text: "Please confirm the bearing length at B12. See the attached sketch."
attachments:
  - id: "AAMkAGI2…BBB="
    kind: "file"                     # or item, reference, other
    name: "B12-sketch.pdf"
    content-type: "application/pdf"
    size: 182344
    inline: false
attachments-omitted: 0
attachments-state: "complete"        # or partial, unavailable
```

- **`text` and `unique-text` are plain text.** `text` is the whole body; `unique-text` is the part
  this message added to its thread. HTML is reduced to its text in AWARE; each is cut at 65,536
  characters and named in `truncated`. Render them as text.
- **Exact ids.** `internet-message-id`, `conversation-id`, `parent-folder-id` and `change-key` are
  kept byte for byte; a malformed value is withheld (`<field>-withheld: true`), never cleaned.
  `change-key` says the message changed, never in which order.
- **Presence is kept.** A field Graph did not send is absent; one it sent as `null` is `null`.
- **Attachments.** At most 20 are listed, the rest counted in `attachments-omitted`.
  `attachments-state` is `complete` only when Graph sent the whole list and every entry was kept.
  An attachment whose payload Graph included anyway is marked `content-withheld: true`; the
  payload is never returned.
- **Too large.** Call with `maxOutputBytes: 1048576`. A message whose raw answer is over 8 MiB
  fails with `raw-too-large`, and retrying will not help. One whose projected answer is over your
  `maxOutputBytes` fails with `output-too-large` (with `outputBytes`): retry with a larger budget.
- **Absence.** A 404 means the message is gone from this mailbox; a valid `parent-folder-id` says
  which folder holds it now.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#763):
`GET /me/messages/{id}?$select=…&$expand=attachments($select=id,name,contentType,size,isInline)`
with `Prefer: IdType="ImmutableId"`. Needs `User.Read` and `Mail.Read` (the code also accepts
`Mail.ReadWrite`).
