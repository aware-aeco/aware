# `file.write` — write content to a file

Stateless, `mode: write`. Writes content to a path, creating missing parent
directories by default, and overwrites any existing file. The generic "land a node's
output on disk" verb — persist a report, quarantine a bad upload, dump a JSON receipt.

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `path` | string (required) | — | Destination path (absolute or relative to the run's working dir). |
| `bytes` | string \| JSON (required) | — | Content to write. A string is written as UTF-8; a JSON value is serialized to JSON text. Named `bytes` for symmetry with `read`. |
| `create-dirs` | bool | `true` | Create missing parent directories. |

## Outputs (single)

```yaml
path:          string    # the resolved absolute path written
bytes-written: integer    # number of bytes written
```

## Composition example — quarantine a malformed upload

```yaml
nodes:
  - id: route
    inline:
      kind: branch
      code: c => c.valid ? "accept" : "quarantine"

  - id: quarantine
    agent: file
    command: write
    when: "{{ route == 'quarantine' }}"
    config:
      path:  "./quarantine/{{ inputs.upload-name }}.errors.json"
      bytes: "{{ check.errors }}"
```

## Safety

`mode: write` — a composing app declares its own `safety:` block on the node when the
write matters (see the app-spec safety contract). Overwrites without prompting at the
agent level; guardrails belong on the app node.

## Failure modes

- **Parent missing and `create-dirs: false`** → `error.path-not-found`.
  **Permission denied / read-only target** → `error.permission-denied`.

## See also

- [read-write.md](../skills/read-write.md) · [write-csv.md](./write-csv.md)
