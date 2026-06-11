# `tekla.watch` — subscribe to live model events

Stateful command. Starts a long-running subscription to Tekla model events on the active model. By default it streams `ModelObjectChanged` (one event per affected model object); the `events` input widens it to the full `Tekla.Structures.Model.Events` surface (saves, selections, numbering, clashes, view changes, …).

> **How delivery works (implementation note).** Two things must be right for an out-of-process watcher to receive events (both verified live on Tekla 2025 + 2026 — see the [`event-threading`](../skills/event-threading.md) skill):
> 1. **Real-method delegate shape.** Tekla silently never invokes a reflection-emitted `DynamicMethod` *or* a closed-static delegate (this was the cause of [aware-aeco/aware#219](https://github.com/aware-aeco/aware/issues/219) — `Register()` succeeded but zero events arrived). The bridge binds `ModelObjectChanged` to a real static method by contravariance and every other event to a per-event **instance** emitter.
> 2. **STA thread + message pump.** `ModelObjectChanged` fires on a worker thread (no pump needed), but UI-thread events (`SelectionChange`, `ViewClosed`, …) are posted to the message queue and only fire while it's pumped — exactly the WinForms/STA configuration the Open API's own `TeklaEvents` sample uses. The bridge runs registration + handlers on a dedicated STA thread with a Win32 message pump.

## Lifecycle

`start` — subscribe and stream until stopped
`stop` — unsubscribe and clean up

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `filter` | enum (`all` `welded` `bolted` `assembly`) | `all` | Pre-filter the **`fired`** (ModelObjectChanged) stream by the **changed object's kind**: `welded`→`Weld` objects, `bolted`→`Bolt*` objects (BoltArray/BoltGroup), `assembly`→`Assembly`. Drawing changes are not surfaced by `ModelObjectChanged` (they come from `Tekla.Structures.Drawing.Events`, a separate stream not yet wired), so there is no `drawing` filter. |
| `include-deleted` | bool | `false` | Emit on deletion (`OBJECT_DELETE`) as well as addition/modification. |
| `events` | string \| list | `ModelObjectChanged` | Which Tekla events to stream. `"all"` covers the whole supported Events surface; a list selects by name (case-insensitive; kebab- or PascalCase both match), e.g. `["ModelObjectChanged", "model-save", "ClashDetected"]`. |

### `events` vocabulary

`ModelObjectChanged` is the rich model-change stream (emitted as `fired`, see below). Every other selected event is emitted as a generic `event` record. Supported names include: `SelectionChange`, `AnnotationSelectionChange`, `CommandStatusChange`, `ModelSave`, `ModelSaveAs`, `ModelSaveInfo`, `ModelLoadInfo`, `ModelUnloading`, `ModelUnloadingSync`, `Numbering`, `ModelObjectNumbered`, `ClashCheckDone`, `ClashDetected`, `Interrupted`, `UndoClicked`, `ProjectInfoChanged`, `HiddenObjectsChanged`, `TemporaryStatesChanged`, `ViewClosed`, `ViewCameraChanged`, and any same-shaped events present on newer Tekla versions. `ModelLoad` and `TeklaStructuresExit` are always surfaced as stderr breadcrumbs / the stop signal. A handful of exotic signatures (`ClipPlaneChanged(int,int,enum)`, `TrackEvent(string,string,string)`) are not yet surfaced and are skipped.

## Outputs (stream)

The data stream carries **one event per line**, and only events — never control records (a phantom record would fire connected nodes with no payload). Live trigger state is observable from the run's events: `NodeStart` means *subscribed/listening*, each `NodeOutput` means *fired*. The bridge writes `listening` / `model-loaded` breadcrumbs to **stderr** for logs.

Two record shapes share the stream, discriminated by `signal`:

**`fired`** — a `ModelObjectChanged` model-object change:

```yaml
signal:   string        # "fired"
guid:     string        # ModelObject GUID (stable identity across edits)
mark:     string        # NEVER use Name — see drawing-identity skill (null on a removed object)
type:     string        # changed object's runtime type: Beam | Assembly | Weld | BoltArray | …
change:   enum          # added | modified | removed
geometry: object        # best-effort bounding box { min:{x,y,z}, max:{x,y,z} } in the CURRENT transformation plane (see coordinate-systems skill)
```

A removed object can't be re-read from the database, so its `mark`/`geometry` are `null` — only `guid`, `type`, and `change: removed` are populated.

**`event`** — any other Tekla event (only present when selected via `events`):

```yaml
signal:   string        # "event"
event:    string        # the Tekla event name (ModelSave, SelectionChange, ClashDetected, …)
data:     object|null   # the event's payload, shaped to its arguments:
                        #   no-arg events            → null
                        #   count-bearing (List<…>)  → { count: N }
                        #   single value (int)       → the value
                        #   CommandStatusChange      → { command, param, active }
                        #   object w/ identity        → { type, guid }
```

Downstream nodes route on `signal`/`event` (e.g. trigger only on `signal == "event" && event == "ModelSave"`).

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
