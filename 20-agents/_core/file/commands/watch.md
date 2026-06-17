# `file.watch` — stream filesystem events from a folder

Stateful command. Opens an OS-native subscription on a directory and streams one
record per matching change until stopped. The plain-filesystem counterpart of
[`tekla.watch`](../../../aeco/engineering/tekla/commands/watch.md): use it as a
**trigger node** (no upstream connection) to react the moment a file lands or
changes — an upload inbox, an export drop, a log directory — instead of polling.

## Lifecycle

`start` — subscribe and stream until stopped
`stop` — unsubscribe and clean up

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `folder` | string (required) | — | Directory to watch (absolute, or relative to the run's working dir). Alias: `path`. |
| `pattern` | string | `*` | Glob filter on the file name (e.g. `*.pdf`, `export-*.csv`). Scope here, not downstream. Alias: `filter`. |
| `events` | string \| list | `[created, changed]` | Which events to emit: `created` `changed` `deleted` `renamed`, or `"all"`. A list selects a subset (case-insensitive). |
| `include-subdirs` | bool | `false` | Recurse into sub-directories. Default watches the top level only. |
| `once` | bool | `false` | One-shot: fire on the **first** matching event, then unsubscribe and exit. `false` = continuous (fire on **every** match). Aliases: `one_time`, `one-time`. |

## Outputs (stream)

One record per line, one per filesystem event — and only events, never an in-band
control record (a phantom record would fire connected nodes with an empty payload).
Live trigger state is observable from the run's events: `NodeStart` = *subscribed/listening*,
each `NodeOutput` = *fired*.

```yaml
signal:    string    # "fired"
path:      string    # absolute path of the affected file
name:      string    # basename — for naming downstream artifacts
change:    string    # created | changed | deleted | renamed
timestamp: string    # ISO-8601 instant observed
```

A `deleted` event's file no longer exists, so don't chain a `read` after it — route on
`change` first if a downstream step needs the bytes.

## Composition example — inbox → process

```yaml
nodes:
  - id: pdf-inbox
    agent: file
    command: watch
    config:
      folder:  "{{ inputs.inbox }}"
      pattern: "*.pdf"

  - id: extract
    agent: vision
    command: extract
    config:
      file: "{{ pdf-inbox.path }}"

connections:
  - { from: pdf-inbox, to: extract, label: PdfFile }
```

## Failure modes

- **Folder does not exist**: fails fast at start with `error.path-not-found`. Create the
  directory (or fix the path) and re-run.
- **Permission denied**: fails at start with `error.permission-denied` — the watched path
  wasn't in the granted filesystem scope. Re-grant at install.
- **Editor write bursts**: a single save can surface as several `changed` events (temp-write
  then rename). Events are **at-least-once** — dedupe downstream on `path` if exactly-once
  matters; prefer matching the final name via `pattern` over reacting to `*.part`/temp files.

## Idempotency

The stream is at-least-once. Each record carries a stable `path`; downstream sinks should
dedupe on `path` (optionally `path` + `timestamp`) if they need exactly-once semantics.

## See also

- [watch-semantics.md](../skills/watch-semantics.md) — continuous vs one-shot, burst handling
- [read.md](./read.md) — pull the file's contents once the event fires
- [`tekla.watch`](../../../aeco/engineering/tekla/commands/watch.md) — the model-event sibling
