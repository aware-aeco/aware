# `tekla.watch` — subscribe to model changes

Stateful command. Starts a long-running subscription to `ModelObjectChanged` events on the active Tekla model. Emits one event per affected model object (or batched event when bursts are detected).

## Lifecycle

`start` — subscribe and stream until stopped
`stop` — unsubscribe and clean up

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `filter` | enum (`all` `welded` `bolted` `assembly` `drawing`) | `all` | Pre-filter what gets emitted. Reduces downstream traffic. |
| `debounce-ms` | int | `200` | Coalesce bursts within this window (see [event-threading skill](../skills/event-threading.md)). |
| `include-deleted` | bool | `false` | Emit on deletion as well as addition/modification. |

## Outputs (stream)

```yaml
guid:     string        # ModelObject GUID (stable identity across edits)
mark:     string        # NEVER use Name — see drawing-identity skill
type:     string        # AssemblyType: Welded | Bolted | …
change:   enum          # added | modified | removed
geometry: object        # bounding-box in world space (see coordinate-systems skill)
```

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
