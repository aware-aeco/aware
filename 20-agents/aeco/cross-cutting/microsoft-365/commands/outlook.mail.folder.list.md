# `microsoft-365.outlook.mail.folder.list`

List the mail folders inside one Outlook folder.

## When to use

Let a person pick a mail folder (for example, to follow it as a source), or walk the folder tree.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `parent-id` | string | yes | A folder's `id`, or a well-known name: `msgfolderroot` (the top level), `inbox`, `sentitems`, `drafts`, `deleteditems`, `archive`, `junkemail`, … |
| `page-size` | int | no | Folders to return, 1 to 100 (default 50). |
| `skip` | int | no | Folders to skip: the `next-skip` of the previous page, 0 to 100,000 (default 0). |

## Output

```yaml
folders:
  - id: "AAMkAGI2…AAA="
    name: "Project 1234 - RFIs"
    parent-id: "AAMkAGI2…AQA="
    child-count: 2
    total-count: 148
    unread-count: 3
more-available: true     # Graph has another page
next-skip: 50            # pass as `skip` to read it (only when more-available)
omitted: 0               # folders dropped because Graph sent a malformed id
```

- **Ids are exact.** An `id` is kept byte for byte or the folder is dropped and counted in
  `omitted`. A malformed `parent-id` is withheld (`parent-id-withheld: true`), never cleaned.
- **Names are text.** `name` is cut at 1,024 characters and named in `truncated`.
- **Hidden folders are not listed** (Graph's default).
- **Paging.** Read the next page with `skip: <next-skip>` while `more-available` is true. When
  the next page would start past skip 100,000, `next-skip` is replaced by `limit-reached: true`.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#763):
`GET /me/mailFolders/{parent-id}/childFolders?$select=…&$top=N&$skip=S`. Well-known names are sent
lower-case; any other `parent-id` must be an Outlook id. Needs `User.Read` and one of
`Mail.ReadBasic`, `Mail.Read` (requested) or `Mail.ReadWrite`.
