# `trimble-connect.upload` — write a file to a project folder

Stateless command. Uploads bytes to a Trimble Connect folder. Idempotent by drawing mark (or any other stable key you set as a custom property).

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Trimble Connect project UUID. |
| `folder-id` | string | Destination folder UUID. |
| `filename` | string | Display filename (e.g. `"A-104.pdf"`). |
| `bytes` | bytes | File contents. |
| `properties` | object | Custom properties stored on the file. **Must include `mark`** for idempotency. |
| `auth-as` | string (optional) | Named credential if user has multiple TC accounts. See [auth-flow](../skills/auth-flow.md). |

## Outputs

```yaml
file-id:  string        # TC file UUID
url:      string        # Direct file URL (resolved via TC web UI)
replaced: bool          # true if an existing file with same mark was updated; false if new
size-kb:  number        # bytes/1024
```

## REST translation

```http
POST https://app.connect.trimble.com/tc/api/2.0/projects/{project-id}/folders/{folder-id}/files
Authorization: Bearer ****                            ← injected by agent runtime
Content-Type: multipart/form-data
X-AWARE-Idempotency-Key: {{ properties.mark }}        ← derived from inputs

file=@{filename}
properties={"mark":"...","source":"..."}
```

See [idempotency.md](../skills/idempotency.md) for the dedup mechanism. See [rate-limits.md](../skills/rate-limits.md) for retry semantics.

## Composition examples

### Linear (the canonical demo)

```yaml
- id: tekla-watch
  agent: tekla
  command: watch
  config: { filter: welded }

- id: upload
  agent: trimble-connect
  command: upload
  config:
    project-id: "{{ inputs.tc-project-id }}"
    folder-id:  "{{ inputs.tc-folder-id }}"
    filename:   "{{ tekla-watch.mark }}.pdf"
    bytes:      "{{ tekla-watch.drawing-bytes }}"
    properties:
      mark:     "{{ tekla-watch.mark }}"
      source:   "aware-fab-pipeline"

connections:
  - { from: tekla-watch, to: upload, label: AssemblyEvent }
```

### With error sink (fan-out)

```yaml
- id: upload
  agent: trimble-connect
  command: upload
  config: { ... }

- id: failure-log
  agent: slack
  command: post-message
  config:
    channel: "#fab-errors"
    text: "Upload failed: {{ upload.error.message }}"

connections:
  - { from: upstream, to: upload }
  - { from: upload, to: failure-log, when: error }     # only on error
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.auth-expired` | Refresh token has expired (~30 days idle) | User runs `aware connect trimble-connect --refresh` |
| `tc.folder-not-found` | folder-id doesn't exist or user lost access | List folders to confirm; check permissions in TC web UI |
| `tc.rate-limit-exceeded` | Hit limits after 3 retries | Throttle upstream (see [rate-limits.md](../skills/rate-limits.md)) |
| `tc.file-too-large` | TC rejects files > 5GB | Split the upload (TC chunked upload — separate `upload-chunked` command, post-v0) |
| `tc.network-timeout` | TCP timeout during upload | Agent retries; surfaces after 3 attempts |

## See also

- [auth-flow.md](../skills/auth-flow.md) — credential handling
- [idempotency.md](../skills/idempotency.md) — why `properties.mark` matters
- [rate-limits.md](../skills/rate-limits.md) — retry behavior
- [trimble-connect.download](./download.md) — the read counterpart
