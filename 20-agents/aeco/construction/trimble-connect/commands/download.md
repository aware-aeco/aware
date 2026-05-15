# `trimble-connect.download` — read a file by ID

Stateless command. Downloads a file's bytes by its TC UUID.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project UUID. |
| `file-id` | string | File UUID. |
| `revision` | string (optional) | Specific revision ID. Default = latest. |
| `auth-as` | string (optional) | Named credential. |

## Outputs

```yaml
filename:   string
bytes:      bytes
size-kb:    number
properties: object        # custom properties stored on the file (including `mark` if set)
revision:   string        # which revision was returned
```

## REST translation

```http
GET https://app.connect.trimble.com/tc/api/2.0/projects/{project-id}/files/{file-id}/download
Authorization: Bearer ****
```

Optionally with `?revision={revision-id}` for non-latest versions.

## Composition example — sync to local cache

```yaml
- id: list
  agent: trimble-connect
  command: list-folders
  config: { project-id: "..." }

- id: per-folder-files
  agent: trimble-connect
  command: list-files
  config: { folder-id: "{{ list.folders.*.id }}" }       # fan-out

- id: download
  agent: trimble-connect
  command: download
  config: { file-id: "{{ per-folder-files.id }}" }

- id: write-local
  agent: file
  command: write
  config:
    path: "D:/tc-cache/{{ list.path }}/{{ download.filename }}"
    bytes: "{{ download.bytes }}"
```

## Resolving by mark, not file-id

When integrating with Tekla (which thinks in marks, not TC UUIDs), do the lookup first:

```yaml
- id: find
  agent: trimble-connect
  command: find-by-property
  config:
    project-id: "..."
    property: mark
    value: "{{ tekla-event.mark }}"

- id: dl
  agent: trimble-connect
  command: download
  config:
    file-id: "{{ find.file-id }}"
```

(`find-by-property` is a v0.2 command; in v0 use `list-files` + a filter glue node.)

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tc.file-not-found` | UUID invalid or user no longer has access | Verify in TC web UI |
| `tc.revision-not-found` | Specific revision deleted | Drop the `revision` param to get latest |
| `tc.auth-expired` | Refresh expired | `aware connect trimble-connect --refresh` |
| `tc.network-timeout` | Large file + slow link | Agent retries; consider chunked download for files > 100MB (post-v0) |
