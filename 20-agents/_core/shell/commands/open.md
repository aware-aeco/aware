# `shell.open` — open a path with the OS default handler

Stateless, `mode: read`. Hands a local file or folder to the operating system's
default handler — the programmatic equivalent of double-clicking it. A folder opens
in the system file browser; a document opens in its registered application. The safe,
scoped complement to an `exec` sandbox that blocks arbitrary process launches: the only
verb is "open this path", with no program name, arguments, or shell string.

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `path` | string (required) | — | Local file or folder to open (absolute, or relative to the run's working dir). |

## Outputs (single)

```yaml
path:   string    # the resolved absolute path that was opened
opened: boolean    # true once the OS handler was launched for the path
```

`open` returns as soon as the handler is *launched*, not when the target app is ready —
it is a fire-and-forget UX convenience, so nothing should depend on its result.

## Composition example — write a report, then reveal its folder

```yaml
nodes:
  - id: report
    agent: html-report
    command: render
    config:
      data:        "{{ summary.rows }}"
      output-path: "./out/run-report.html"

  - id: reveal
    agent: shell
    command: open
    config:
      path: "./out"

connections:
  - { from: report, to: reveal, label: Done }
```

## Failure modes

- **Path does not exist** → `error.path-not-found`.
- **No registered handler / no desktop session** (headless CI, service account) →
  `error.no-handler`. Treat `open` as best-effort UX, never a load-bearing step.
- **Permission denied** (path outside the granted scope) → `error.permission-denied`.

## Safety

`mode: read` — opening reveals/launches a path and mutates no system of record, so a
node needs no `safety:` block. The command is intentionally argument-less: there is no
`shell.exec`/`shell.run`. To invoke a specific tool, use an agent with a typed contract,
not a generic shell escape.

## See also

- [open-semantics.md](../skills/open-semantics.md) — why open-only, cross-platform mapping
- [`file.write`](../../file/commands/write.md) — produce the artifact you then `open`
