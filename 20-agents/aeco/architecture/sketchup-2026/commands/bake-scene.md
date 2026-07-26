# bake-scene

Materializes a canonical millimetre scene in the open SketchUp model as
source-owned groups. This is a write command and requires stable
`scene.meta.sourceId` and `sceneHash` values.

`mode: write` — the composing app must declare a `safety:` block per v0.11.

```powershell
$request | ConvertTo-Json -Depth 100 -Compress |
  aware-sketchup bake-scene --json-stdin
```

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `scene` | object | yes | The canonical millimetre scene. `meta` is `{name?, units?:"mm", sourceId, sceneHash}`; collections are `elements`, `operations`, `referenceSystems`. |
| `sketchup_id` | integer | no | Target PID. Required when more than one SketchUp is running. |
| `version` | string | no | Version prefix (e.g. `"26.0"`) used to pick the instance instead. |
| `label` | string | no | Name shown in the status bar while the bake runs. |

## Outputs

```json
{ "ok": true,
  "result": { "ok": true, "sourceId": "...", "sceneHash": "...",
              "materializationHash": "...", "attemptId": "...",
              "tag": "AWARE", "created": 12, "retired": 9,
              "emitted": [ { "id": "m1", "kind": "member", "status": "emitted",
                             "persistentId": "...", "profile": "W12X26" } ],
              "failed": [], "unsupported": [], "warnings": [] } }
```

Every record in the scene appears exactly once across `emitted`, `failed` and
`unsupported`. Rows are `{id, kind, status, code?, message?}` — the same receipt
vocabulary the Tekla sink emits, so one scene produces comparable receipts from
both hosts.

## What it builds

Members (`member`, `line`, `box`) become a named `Sketchup::Group` each, holding
the profile face extruded along the `from`→`to` axis and placed on the `AWARE`
tag. The outline comes from `meta.profile` plus `xsection` — a real I, channel,
angle or hollow section when the designation carries one, and the `section:{w,d}`
envelope otherwise. `rot` rolls the section about the reference line; without it
the web is stood vertical, and a vertical member keeps the authored plan
rotation.

Every other element kind, and every operation and reference system, is reported
as an `unsupported` row. Nothing is silently dropped. A record's nested children
— a plate's holes, a bolt array's instances, a grid's axes — are covered by
their parent's row.

## Units

The scene is millimetres; SketchUp's internal unit is inches. The conversion
uses SketchUp's own `Numeric#mm` / `Length#to_mm` rather than a hand-rolled
25.4, and the bake proves the pair round-trips on the live host before building
anything. Explicit units other than `mm` are rejected before the model is
touched. A member shorter than SketchUp's own 0.001" tolerance is refused rather
than silently producing nothing.

## Ownership and retire-and-replace

Each group carries four values in its `AWARE` `AttributeDictionary`: the
`sourceId`, the record id, the scene hash, and a namespaced
`AWARE_BAKE_V1:<sha256>` materialization marker.

A bake retires exactly the groups already carrying the **same** `sourceId`, and
only when all four attributes are present and the marker is well formed. A group
stamped by a different source, or never stamped at all, is never touched. This is
load-bearing: callers that want independent drops must give each one its own
derived source id, or a later bake will retire the earlier one's work.

The prior set is scanned before anything is created and erased only after the
whole replacement set has been created and its ownership stamps read back.

## Undo and failure

The whole bake runs inside one `model.start_operation(..., true)` /
`commit_operation`, so a drop is a single SketchUp Undo. Any failure calls
`abort_operation`, which reverts every entity created, modified or erased since
the operation opened — so unlike the Tekla sink there is no
`commit-state-uncertain` outcome to reconcile, and failed rows carry
`"rolledBack": true`.

Validation is staged so a refusal costs nothing: the scene envelope, id rules and
kind classification are checked in the sidecar before a socket is opened, and all
member geometry is validated before the operation starts.

## Gotchas

- The bridge advertises each running SketchUp at `%TEMP%\aware-sketchup\<pid>.json`.
  A caller whose `TEMP` differs from SketchUp's own sees zero instances.
- The materializer is injected by the sidecar through the bridge's `exec` path,
  not installed with the bridge — so it never needs a SketchUp restart to update,
  and a new sidecar can never drive a stale write script.
- The bridge's main-thread watchdog answers at 90s. A very large scene comes back
  as that clean timeout rather than a dropped socket.

## See also

- [`exec`](./exec.md) — the general-purpose Ruby path this rides on
- [`attribute.bulk-write`](./attribute.bulk-write.md) / [`selection.by-attribute`](./selection.by-attribute.md) — the same `AWARE` dictionary, read and written by hand
- Compare to Tekla's [`bake-scene`](../../../engineering/tekla/commands/bake-scene.md) — the same scene, the same receipt vocabulary, a different sink
