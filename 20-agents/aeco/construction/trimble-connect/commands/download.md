# `trimble-connect.download` — read a file by ID

Stateless command. Downloads a file's bytes by its TC UUID.

> **REST wiring deferred (follow-up to #196).** Trimble Connect download is a two-step,
> binary flow (request a download URL, then `GET` the bytes) that the single-call REST
> transport cannot yet express, so this command is declared but not dispatchable. The
> contract below documents the intended shape.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project UUID. |
| `file-id` | string | File UUID. |
| `revision` | string (optional) | Specific revision ID. Default = latest. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (see the agent's `auth:` block).

## Outputs (intended)

```yaml
filename:   string
bytes:      bytes
size-kb:    number
properties: object        # custom properties stored on the file (including `mark` if set)
revision:   string        # which revision was returned
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.file-not-found` | UUID invalid or user no longer has access | Verify in TC web UI |
| `tc.revision-not-found` | Specific revision deleted | Drop the `revision` param to get latest |
| `401` (`INVALID_SESSION`) | Access token expired | `aware connect trimble-connect --refresh` |
