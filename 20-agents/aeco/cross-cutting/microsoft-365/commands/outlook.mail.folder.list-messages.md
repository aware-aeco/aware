# `microsoft-365.outlook.mail.folder.list-messages`

Read what changed in one Outlook mail folder: messages added, changed, deleted, moved out or
recalled, page by page, with a cursor for the next call.

## When to use

Keep a copy of a mail folder up to date, for example a folder followed as a Knowledge source. The
first round reads the whole folder; every later round reads only what changed since.

Read-only. Headers only: read a body with `outlook.mail.get-message`.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `folder-id` | string | yes | The folder's `id`, from `outlook.mail.folder.list`. A well-known name (`inbox`) is not accepted: list the folder to get its id. |
| `page-size` | int | no | Changes Graph should send per page, 1 to 50 (default 25). A preference: Graph may send more. |
| `cursor` | string | no | The `cursor` of the previous answer. Omit it to start a new round. |

## Output

```yaml
changes:
  - change: "upsert"                 # added or changed
    id: "AAkALgAA…"
    fields:
      kind: "message"                # or event-message, other
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
  - change: "removed"                # Graph says it left the folder (deleted, moved, recalled)
    id: "AAkALgBB…"
    reason: "deleted"
  - change: "unidentifiable"         # Graph sent an entry with no usable id
    position: 2
unidentifiable: 1
cursor: "odc1.eyJiaW5kaW5nIjoi…"
more-available: true                 # the round is not finished: call again with `cursor`
```

- **Graph's order, not time order.** Changes come in the order Graph sent them, which is not
  promised to be chronological, and the same id can appear more than once in a round. Decide each
  id from evidence, not from its position.
- **A removal is evidence, not proof.** To confirm one, read the message with
  `outlook.mail.get-message`: a 404, or a valid `parent-folder-id` other than this folder, proves
  it is no longer here; a missing or withheld `parent-folder-id` proves nothing.
- **A whole page, every time.** Every entry Graph sent is returned, however many: once the cursor
  moves on, an entry left out would be lost. Call with `maxOutputBytes: 1048576`.
- **Too large.** `raw-too-large` means Graph's answer passed AWARE's 8 MiB read cap: retry the same
  `cursor` with a smaller `page-size` (at `page-size: 1` it cannot progress; nothing is returned).
  `output-too-large` (with `outputBytes`) means the projected answer passed your `maxOutputBytes`:
  retry the same request with a larger budget, up to 1 MiB, and only past that with a smaller
  `page-size`. Neither means a bad record.
- **Presence is kept.** A field Graph did not send is absent (unknown), never a guessed `false` or
  empty value; a field it sent as `null` is `null`. An `upsert` with no `fields` beyond its id says
  only that the message changed: read it with `outlook.mail.get-message`.
- **Unidentifiable entries stay in place** with their `position` and are counted. The cursor is
  still returned, so the stream moves on. Any non-zero `unidentifiable` in a round makes the whole
  round incomplete: do not treat messages that round did not mention as gone, and do not call the
  copy complete until a later fresh round (no cursor) finishes with none.
- **Rounds.** `more-available: true` means call again with `cursor`. `false` means the round is
  done, and that answer's `cursor` asks, later, for what changed since.

## Cursors

- **Opaque and bound.** A cursor works only for the folder, the account and this exact version of
  the read that produced it. Another folder, another account, or a changed read fails with
  `cursor-context` before any data request is sent.
- **Expiry.** When Graph no longer knows a cursor the call fails with `cursor-expired`: start a new
  round without a cursor and reconcile against what you already hold.
- **Not a secret.** A cursor is not signed; it holds Graph's own state token. Store it with the
  same care as the data it reads.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#763):
`GET /me/mailFolders/{folder-id}/messages/delta?$select=…` with
`Prefer: odata.maxpagesize=N, IdType="ImmutableId"`. A cursor request sends only its
`$skiptoken` or `$deltatoken`, which carries the first request's selection. Ids are immutable,
so a message keeps its id when it moves between folders. AWARE verifies the signed-in account on
every call. Needs `User.Read` and `Mail.Read` (the code also accepts `Mail.ReadWrite`).
