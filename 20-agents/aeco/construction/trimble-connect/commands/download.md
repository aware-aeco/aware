# `trimble-connect.download` — read a file's bytes by ID

Stateless command. Downloads a file's bytes by its TC UUID via TC's **2-step pre-signed
download**, handled in-process by the runtime (`cli/src/runtime/trimble_files.rs`, #200).

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `file-id` | string | File UUID. |
| `version-id` | string (optional) | Specific version; default = latest. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

```yaml
bytes:    string          # base64-encoded file content
encoding: string          # "base64"
size-kb:  number          # decoded size / 1024
```

## REST translation (2-step)

```http
1. GET {base}/files/fs/{file-id}/downloadurl    (Bearer)   → { url }
2. GET {presigned-url}                           (NO auth — S3)   → <raw bytes>
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `download url: HTTP 404` | UUID invalid or no access | Verify the id in the TC web UI |
| `download GET: …` | The pre-signed storage GET failed | Retry; check network |
| `tc.auth-missing` | No credential provisioned | `aware connect trimble-connect --oauth` |

## See also

- [files.md](../skills/files.md) — the full Files & Folders API reference
- [trimble-connect.upload](./upload.md) — the write counterpart
