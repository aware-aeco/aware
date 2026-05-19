# `sketchup.exec` — evaluate an ad-hoc Ruby script against the active SketchUp model

Stateless command. Receives a chunk of Ruby code, ships it to the in-process bridge running inside a live SketchUp 2026 process, runs it against `Sketchup.active_model` on SketchUp's main thread, returns the result as JSON.

The verb that closes the loop between the **catalog** (155 types, 1471 methods documented in `20-agents/aeco/architecture/sketchup-2026/catalog/`) and the **live host** (where those types come alive). Mirrors the shape of `tekla.exec` (v0.31) and `rhino.exec` (v0.32). This is the AWARE pattern in one verb: **prompt → catalog lookup → executable Ruby → live host**.

## Architecture in one paragraph

SketchUp has no out-of-process CLI bridge (unlike Rhino's `rhinocode`). Trimble only exposes scripting through SketchUp's in-process Ruby 3.2 interpreter. So AWARE ships an auto-loaded Ruby extension — `aware_sketchup_bridge.rb` — that opens a TCP socket on 127.0.0.1:8765 (first free port in 8765..8865) inside the SketchUp process. `aware-sketchup.exe` connects to that socket, ships a length-prefixed JSON request, the bridge marshals the request onto SketchUp's main thread via `UI.start_timer(0.05, true)`, evaluates the user's code, returns a JSON receipt. The bridge writes a discovery file at `%TEMP%/aware-sketchup/<pid>.json` so the sidecar can find which port belongs to which SketchUp PID.

## Lifecycle

`single` — one call, one response.

## Category

`curated` — open-ended, like `tekla.exec` and `rhino.exec`. Does not encode a single SketchUp workflow; gives the orchestrator a generic execution channel.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | string | **yes** | Ruby snippet. The script's last expression is the return value (standard Ruby semantics). |
| `version` | string | recommended | Target SketchUp version prefix (e.g. `"26"` for SketchUp 2026). Used to pick which instance to talk to. |
| `sketchup_id` | string | no | A specific PID-as-string to target. Overrides `version`. Use the value returned from `list-instances`. |
| `args` | object | no | Free-form input bag, exposed to the script as a `Hash` named `args` (string keys). |

### Script globals

The user's code runs inside a wrapper that exposes:

| Name | Type | Description |
|---|---|---|
| `model` | `Sketchup::Model` | The active document, i.e. `Sketchup.active_model`. |
| `args` | `Hash` | The input `args` block (string keys). |

The user code is wrapped in a lambda, so a top-level `return X` short-circuits the lambda (not the bridge). Implicit return via "last expression" also works.

## Outputs

On success:

```json
{
  "ok": true,
  "result": <serialized return value>,
  "host": "sketchup",
  "host_version": "26.0.123",
  "host_pid": 12345,
  "sketchup_id": "12345",
  "verb": "exec",
  "stdout_log": "<anything the script printed via puts>",
  "delivered_at": "2026-05-19T..."
}
```

The `result` field is serialized defensively:

- Primitives (`nil`, `true`, `false`, Integer, Float, String) → direct JSON.
- `Symbol` → string form.
- `Hash`, `Array` → recursive.
- `Geom::Point3d`, `Geom::Vector3d` → `[x, y, z]` (Floats, internal units = inches).
- `Geom::Point2d`, `Geom::Vector2d` → `[x, y]`.
- `Geom::BoundingBox` → `{ min: [x,y,z], max: [x,y,z], valid: bool }`.
- `Geom::Transformation` → 16-element flat matrix (Floats).
- `Length` → Float (inches).
- `Sketchup::*` entity → `{ _type: "Sketchup::Edge", persistent_id: "...", entityID: 42, name: "..." }`.
- Anything else → `.to_s` fallback.

Recursion depth is bounded at 8 (defensive against circular references).

On runtime exception (script threw):

```json
{
  "ok": false,
  "error": "<RubyClass>: <message>",
  "stack": "<first 50 backtrace lines>",
  "host": "sketchup",
  "host_version": "26.0.123",
  "host_pid": 12345,
  "verb": "exec",
  "stdout_log": "<anything captured before the throw>",
  "delivered_at": "..."
}
```

## Transaction semantics

SketchUp's transaction primitive is `Model#start_operation` / `Model#commit_operation` (or `Model#abort_operation`). The bridge does **not** wrap your script in an implicit operation. Idiomatic write-side scripts should open + commit their own:

```ruby
model.start_operation('AWARE my-op', true)
# ...mutate...
model.commit_operation
```

Without `start_operation`, mutations land directly in the undo stack as individual ungrouped events.

## Failure modes

| Error shape | Cause | Recovery |
|---|---|---|
| `missing required field: code` | Stdin JSON didn't include `code`. | Add it. |
| `no SketchUp instance found` | No PID is running the bridge. | Run `aware-sketchup launch` first, then poll `list-instances` until your PID appears. |
| `no SketchUp instance matches version='X'` / `sketchup_id='Y'` | Filter doesn't match any running instance. | Check `aware-sketchup list-instances` output. |
| `ambiguous target` | Multiple instances are running, none specified. | Pass `sketchup_id` in the request. |
| `bridge I/O failed` / `timeout` | Socket died mid-call or `UI.start_timer` is starved (e.g. SketchUp is mid-modal-dialog). | Inspect SketchUp's UI state; retry. |
| `<RubyClass>: <msg>` runtime | Script threw. | Inspect `stack`. |

## Example invocations

### Read-side smoke test

```bash
echo '{
  "verb": "exec",
  "version": "26",
  "code": "{ \"entities\" => model.entities.length, \"selected\" => model.selection.length }"
}' | aware-sketchup.exe exec --json-stdin
# → {"ok":true,"result":{"entities":42,"selected":0},"host":"sketchup",...}
```

### Write-side with explicit operation

```bash
echo '{
  "verb": "exec",
  "version": "26",
  "code": "model.start_operation(\"AWARE add line\", true); edge = model.entities.add_line(Geom::Point3d.new(0,0,0), Geom::Point3d.new(1000.mm, 0, 0)); model.commit_operation; { id: edge.persistent_id.to_s }"
}' | aware-sketchup.exe exec --json-stdin
```

### Multi-instance routing

```bash
# list first to find the PID
aware-sketchup.exe list-instances
# → {"status":"ok","instances":[{"pid":12345,"version":"26.0","port":8765,...}], ...}

# target it explicitly
echo '{"verb":"exec","sketchup_id":"12345","code":"model.guid"}' | aware-sketchup.exe exec --json-stdin
```

## Prerequisites

1. **SketchUp 2026 installed** (or pass `--target-year 2024/2025/2026` to `--install-bridge` for older).
2. **Bridge installed** via `aware-sketchup --install-bridge` once.
3. **SketchUp restarted** after install (extension auto-loads at next launch).
4. **A model open**. `Sketchup.active_model` is reliably non-nil only after the Welcome screen dismisses or a `.skp` file opens. Launch with `model_path` set to short-circuit the Welcome screen on cold boot.

## Implementation notes

**Read the design doc at `docs/superpowers/specs/2026-05-19-v034-sketchup-exec-design.md`** for the full architecture rationale (length-prefix framing, discovery files, main-thread marshaling).

The bridge eval wrapper:

```ruby
result = (lambda { |args|
  model = Sketchup.active_model
  # === user code body ===
  #{user_code}
  # ======================
}).call(args)
```

`$stdout` is captured into a `StringIO` for the duration of the eval, so `puts "debug"` lands in the receipt's `stdout_log` instead of SketchUp's Ruby Console. The 90s eval timeout is a safety net for runaway loops; the 30s sidecar-side socket timeout fires first in normal use.

## See also

- [send-status.md](./send-status.md) — simplest verb; smoke-tests the bridge end-to-end.
- [list-instances.md](./list-instances.md) — discovery via `%TEMP%/aware-sketchup/<pid>.json`.
- [launch.md](./launch.md) / [close.md](./close.md) — process lifecycle.
- [`cli-tekla` exec.md](../../../engineering/tekla/commands/exec.md) — the v0.31 reference pattern.
