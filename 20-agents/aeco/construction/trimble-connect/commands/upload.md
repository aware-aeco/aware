# `trimble-connect.upload` — write a file to a project folder

Stateless command. Uploads bytes to a Trimble Connect folder via TC's **3-step package
upload**, handled in-process by the runtime (`cli/src/runtime/trimble_files.rs`, #200)
since it can't be expressed as a single REST call.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `folder-id` | string | Destination folder UUID (a project's `rootId`, or a sub-folder id). |
| `filename` | string | Display filename (e.g. `"A-104.pdf"`). |
| `bytes` | bytes | File contents. Binary travels as a **base64 string** (or a byte array). |
| `project-id` | string (optional) | Accepted for composition convenience; not required (folders are addressed by global id). |
| `properties` | object (optional) | Accepted; not yet applied — TC dedups by content hash (see below). |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

```yaml
file-id:    string        # TC file UUID
version-id: string        # the new version UUID
replaced:   bool          # true when TC returned DUPLICATE (content already present)
```

## REST translation (3-step package upload)

```http
1. POST {base}/files/fs/upload?parentId={folder-id}&parentType=FOLDER     (Bearer)
   body: { "name": "{filename}", "contents": [ {} ] }
   → { uploadId, status, contents: [ { url } ] }

2. PUT  {presigned-url}                                                   (NO auth — S3)
   Content-Type: application/octet-stream
   body: <raw bytes>

3. GET  {base}/files/fs/upload?uploadId={uploadId}&wait=true              (Bearer)
   → { fileId, versionId }
```

> A content-identical file already present yields `status: DUPLICATE` (TC dedups by
> content hash); the command returns the existing `file-id` with `replaced: true`. Do
> **not** use `POST /files` with `multipart/form-data` (returns 415).

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |
| `upload initiate: HTTP 404` | folder-id doesn't exist or no access | List folders to confirm; check permissions in the TC web UI |
| `upload PUT: …` | The pre-signed storage PUT failed | Retry; check network / file size |

## See also

- [auth-flow.md](../skills/auth-flow.md) — credential handling
- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.download](./download.md) — the read counterpart
