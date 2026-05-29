# `tekla.watch` — subscribe to model changes

Stateful command. Starts a long-running subscription to `ModelObjectChanged` events on the active Tekla model. Emits one event per affected model object (or batched event when bursts are detected).

## Lifecycle

`start` — subscribe and stream until stopped
`stop` — unsubscribe and clean up

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `filter` | enum (`all` `welded` `bolted` `assembly`) | `all` | Pre-filter what gets emitted. Reduces downstream traffic. Matches the **changed object's kind**: `welded`→`Weld` objects, `bolted`→`Bolt*` objects (BoltArray/BoltGroup), `assembly`→`Assembly`. Drawing changes are not surfaced by `ModelObjectChanged` (they come from `Tekla.Structures.Drawing.Events`, a separate stream not yet wired), so there is no `drawing` filter. |
| `include-deleted` | bool | `false` | Emit on deletion (`OBJECT_DELETE`) as well as addition/modification. |

## Outputs (stream)

The data stream carries **one change event per line** — and only change events. The runtime forwards each emitted line to downstream nodes, so the watcher does **not** put control records (e.g. a "listening" marker) on the data stream; a phantom record there would fire connected nodes with no change payload. Live trigger state is instead observable from the run's events: `NodeStart` for the watch node means *subscribed/listening*, each `NodeOutput` means *fired*. The bridge also writes `listening` / `model-loaded` breadcrumbs to **stderr** for logs.

```yaml
signal:   string        # always "fired" on the data stream
guid:     string        # ModelObject GUID (stable identity across edits)
mark:     string        # NEVER use Name — see drawing-identity skill (null on a removed object)
type:     string        # changed object's runtime type: Beam | Assembly | Weld | BoltArray | …
change:   enum          # added | modified | removed
geometry: object        # best-effort bounding box { min:{x,y,z}, max:{x,y,z} } in the CURRENT transformation plane (see coordinate-systems skill)
```

A removed object can't be re-read from the database, so its `mark`/`geometry` are `null` — only `guid`, `type`, and `change: removed` are populated.

> **Coordinates:** `geometry` is reported in Tekla's **current transformation plane**, which equals world coordinates only when the user hasn't changed the work plane. The watcher does not switch the session to the global plane to normalize this — that would mutate the user's live session from a background thread. If a downstream node needs guaranteed world coordinates, account for the active work plane there.

## Composition examples

### Linear — emit to TC

```yaml
nodes:
  - id: tekla-watch
    agent: tekla
    command: watch
    config: { filter: welded }

  - id: upload
    agent: trimble-connect
    command: upload
    config:
      file: "{{ tekla-watch.mark }}.pdf"
      idempotency-key: "{{ tekla-watch.mark }}"

connections:
  - { from: tekla-watch, to: upload, label: AssemblyEvent }
```

### Fan-out — emit to multiple sinks

```yaml
connections:
  - { from: tekla-watch, to: upload,        label: AssemblyEvent }
  - { from: tekla-watch, to: slack-notify,  label: AssemblyEvent }
  - { from: tekla-watch, to: log-to-excel,  label: AssemblyEvent }
```

## Failure modes

- **Tekla not running**: agent fails fast at start with `error.tekla-not-running`. Restart Tekla and run `aware app run <name>` again.
- **Model not opened**: agent stays in `waiting-for-model` state, emits a `status` event when a model is opened. No retries needed from your app.
- **Connection lost (rare)**: emit `error.connection-lost`, auto-reconnect every 5s for 60s, then give up. Downstream events are buffered for 10s; older events are dropped with a warning.

## Idempotency

The agent emits a stable `guid` per object. Downstream sinks should dedupe on `guid` if they care about exactly-once semantics. The `mark` is stable too but may be reassigned (rare) if a user manually edits it.

## See also

- [drawing-identity.md](../skills/drawing-identity.md) — why `mark`, not `name`
- [event-threading.md](../skills/event-threading.md) — burst handling, marshaling
- [tekla.insert](./insert.md) — the destructive counterpart
