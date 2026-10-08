# `google-workspace.list-files`

List Drive files' metadata (id, name, MIME type, size, modified time) matching an
optional Drive query. Read-only.

## When to use

Discover what's in a project share before acting — "find the issued PDFs", "list
every model in the WIP folder". The read primitive most Drive flows open with.

**READ-mode.** Runs in a workflow node, and as the reviewed account-bound read of
`aware agent call` (#618), which FloLess uses for chat reads.

## Access

The Google account must be connected with the Drive read scope — it is **opt-in**,
not part of the default consent:

```
aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/drive.readonly
```

`drive.readonly` is a Google *restricted* scope. It would allow reading file
content, but AWARE never does: this command calls only `files.list` with a fixed
metadata field mask (below). The narrower `drive.metadata.readonly` is accepted
as well. Without either, the read is refused (`E_CALL_SCOPE_MISSING`) and nothing
is sent to Drive. Mail keeps working on a slot that also holds one of them; any
other extra Google scope is still refused by `gmail.send`.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `query` | string | no | — | Drive search query (`q`), e.g. `"mimeType='application/pdf' and trashed=false"`. At most 2048 bytes. |
| `page-size` | int | no | `100` | Files to return, 1 to 100. |

## Output

```yaml
files:
  - id:            string
    name:          string
    mime-type:     string
    size:          number   # absent for Google-native documents
    modified-time: string
more-available:    boolean  # Drive has another page (no page token input yet)
incomplete-search: boolean
```

Only these fields leave AWARE; the response is projected, never passed through.

## Worked example

```yaml
- id: find-pdfs
  agent: google-workspace
  command: list-files
  inputs:
    query: "mimeType='application/pdf' and trashed=false"
    page-size: 50
```

## Implementation note

The request is code-owned in AWARE (`cli/src/runtime/agent_call.rs`): `GET
https://www.googleapis.com/drive/v3/files` with `q`, `pageSize` and the fixed field
mask `nextPageToken,incompleteSearch,files(id,name,mimeType,size,modifiedTime)`, no
redirects, a bounded body. The credential goes only to Google's Drive and OpenID
origins, and its OAuth refresh only to Google's token endpoint.

## See also

- [`download-file`](./download-file.md)
- [`drive.folder.create`](./drive.folder.create.md)
- [`drive.share`](./drive.share.md)
