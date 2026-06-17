# `file.read` — read a file's contents into the composition

Stateless, `mode: read`. Returns a file's bytes as UTF-8 text (default) or base64,
so a downstream node can parse or transform them. The generic "pull a file in" verb.

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `path` | string (required) | — | File to read (absolute or relative to the run's working dir). |
| `encoding` | enum (`text` `base64`) | `text` | `text` decodes as UTF-8; `base64` returns raw bytes base64-encoded (use for binary files). |

## Outputs (single)

```yaml
content: string    # the contents (UTF-8 text, or base64 when encoding=base64)
bytes:   integer    # size read, in bytes
path:    string     # the resolved absolute path
```

## Composition example — read an export, then transform its contents

`read` returns the bytes as `content`; pipe that to any node that consumes inline
text. (To hand a file to a path-based reader instead, pass the *path* to both nodes —
don't read-then-pipe.)

```yaml
nodes:
  - id: pull
    agent: file
    command: read
    config: { path: "{{ inputs.export }}" }

  - id: parse
    inline:
      kind: map
      description: Split the exported CSV text into rows
      code: c => c.content.trim().split("\n").map(line => line.split(","))

connections:
  - { from: pull, to: parse, label: Text }
```

## Failure modes

- **File not found** → `error.path-not-found`. **Permission denied** → `error.permission-denied`
  (path outside the granted scope). **Not valid UTF-8** with `encoding: text` → `error.decode`;
  re-read with `encoding: base64`.

## See also

- [read-write.md](../skills/read-write.md) · [write.md](./write.md)
