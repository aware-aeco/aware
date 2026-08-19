# `cli-tekla` — Tekla Structures runtime sidecar

A .NET Framework 4.8 binary that gives the AWARE runtime programmatic access to a user-running Tekla Structures instance via Tekla Open API. The first vendor sidecar (v0.31 ships `exec`); set the pattern that [`cli-rhino`](../cli-rhino/) and future vendor sidecars follow.

Same `{verb, code, args}` stdin-JSON contract as cli-rhino; different vendor binding.

## Why .NET Framework 4.8?

Tekla Open API targets `v4.0.30319` IL. Modern .NET (Core / 5+ / 10+) **cannot reliably load** Tekla.Structures.*.dll — Tekla's COM interop and .NET Remoting paths don't survive the transition. Empirically: `Model.GetConnectionStatus()` throws `FileNotFoundException` on net9. The sidecar must be net48.

Cross-reference: same constraint likely applies to Revit + AutoCAD. See the memory entry [`reference_tekla_dotnet_framework.md`](../C:/Users/bimst/.claude/projects/...).

## Verbs

| Verb | Purpose |
|---|---|
| `send-status` | Display a transient message in Tekla's status bar. |
| `list-instances` | Enumerate running Tekla instances (PID, version, exe path). |
| `launch` | Spawn a Tekla instance via Bypass.ini (headless startup pattern). |
| `close` | Save (via Open API + ModelSave event wait) + clean shutdown. `force: true` for force-kill. |
| `exec` | Compile + run an ad-hoc C# script against the active Tekla model via Roslyn-in-sidecar. |
| `bake-scene` | Materialize source-owned native members, connections, and structural grids. |
| `watch` | `lifecycle: start` — subscribe to `ModelObjectChanged` and stream newline-delimited JSON change events. Consumed by the runtime's streaming transport (`invoke_stream`, #172/#173). |

## Prerequisites

- Tekla Structures 2025.0 or 2026.0 installed
- For `exec`/`send-status`: Tekla running with a model open (Open API attaches via `new Model()`)
- For `launch`: license string + environment (e.g. FULL/default) — see `launch` JSON shape

## Build

```powershell
dotnet build cli-tekla/cli-tekla.csproj -c Debug
dotnet build cli-tekla/cli-tekla.csproj -c Release
# -> bin/Release/net48/aware-tekla.exe  (~5 MB + sibling DLLs)
```

Single-file publish isn't natively supported for net48 — ship `aware-tekla.exe` alongside its (minimal) sibling DLLs.

## Quick usage

```powershell
$awareTekla = "cli-tekla\bin\Release\net48\aware-tekla.exe"

# Discover live Teklas
& $awareTekla list-instances

# Send a status message
'{"message":"hello from AWARE"}' | & $awareTekla send-status --json-stdin

# Exec a snippet
'{"verb":"exec","version":"2026.0","code":"return ((Tekla.Structures.Model.ModelObjectSelector)model.GetModelObjectSelector()).GetSelectedObjects().Count;"}' `
  | & $awareTekla --json-stdin

# Launch with a model
'{"version":"2026.0","environment":"default","license":"FULL","model_path":"C:\\Models\\hello\\hello.db1"}' `
  | & $awareTekla launch --json-stdin

# Clean save + shutdown
& $awareTekla close --version 2026.0

# Watch model changes (streams JSONL until Tekla exits or the process is stopped)
'{"filter":"welded"}' | & $awareTekla watch --json-stdin
# Offline self-test (synthetic events, no live Tekla) — useful for wiring checks:
'{"filter":"all","self_test":true}' | & $awareTekla watch --json-stdin
```

## Exec contract

The user-supplied `code` is a Roslyn script. Globals available:

- `dynamic model` — `Tekla.Structures.Model.Model` instance (or null if no live Tekla)
- `IDictionary<string, object?> args` — args block from input JSON

```json
{
  "verb": "exec",
  "version": "2026.0",
  "code": "return ((Tekla.Structures.Model.Model)model).GetActiveModel().GetIdentifier().ToString();",
  "args": {}
}
```

Receipt:
```json
{
  "ok": true,
  "result": "...",
  "host": "tekla",
  "host_version": "2026.0",
  "host_pid": 22996,
  "verb": "exec",
  "delivered_at": "2026-05-19T..."
}
```

`host_pid` and `host_version` populated when a live Tekla is detected (v0.32.2 receipt convergence).

## Structural-grid materialization

Canonical grid axes may carry independent `startMm` and `endMm` values. Tekla's
native Grid has only one rectangular envelope, so `bake-scene` derives the
tightest envelope that contains every authored segment and both axis families'
coordinate spans. It verifies the native origin, coordinates, labels, extensions,
levels, and magnetic flag before replacing a prior source-owned grid. A lossy
native expansion is explicit in `warnings` as
`tekla-grid-axis-extents-expanded`; axes and levels share the parent Grid GUID
through `realizedBy` rather than becoming duplicate native objects. Tekla 2026
grids with a two-word label such as `2nd Floor` remain one native `Grid`: the
bridge writes all elevations into that parent's `CoordinateZ`/`LabelZ`, maps the
space to a deterministic native token (`2nd_Floor`), and reports the exact
authored-to-native mapping as `tekla-grid-label-tokenized`. Tekla-generated
`GridPlane` children are inspected read-only; the bridge never creates or edits
one plane per elevation.

## Double-angle materialization

Tekla has no native 2L profile — double angles exist there only inside components,
whose geometry comes from environment-specific component defaults rather than from
the scene descriptor. So `bake-scene` materializes a canonical
`xsection.shape:"double-angle"` itself, as **two plain parametric single angles**
(`L{d}*{b}*{t}`) offset along the member's rolled section X by
`gap/2 + outstanding/2` each way, reproducing the same figure the viewer, IFC and
Rhino sinks draw. Both parts go through the normal `insertBeam` path, so each keeps
the exact-profile read-back, the ownership UDAs and the B-rep roll verification.

One of the two is built on the reversed `to`→`from` axis. That is the only way to
mirror a section in Tekla, and a mirror is genuinely required: an unequal angle is
chiral, so no roll about the member axis turns one leg of a back-to-back pair into
the other. Which axis the reversal mirrors across depends on the canonical zero
frame's branch — across Y for a projected (non-vertical) member, across X on the
near-vertical seed branch — so a reversed leg of a column carries an extra 180°.
`TeklaDoubleAngleContract` owns that arithmetic with no Tekla references, so unit
tests exercise the production algorithm; `Tests/Fixtures/double-angle-scene.json`
drives a live pass across beams, columns, rolled, sloped, equal-leg and zero-gap
cases.

Neither fact is provable by a unit test — a test's model of Tekla is the model the
plan was derived from, so flipping one moves both sides together and the suite
stays green. So the bake proves the pair instead: after both legs are inserted
their solids are projected into the member's rolled section frame and compared,
vertex for vertex, against the canonical outline the plan carries. A catalog whose
parametric `L` seats differently from the probed one fails the bake rather than
committing a wrong pair. Both premises were checked by mutation — inverting the
chirality is refused on the first horizontal member, dropping the vertical half
turn on the first column. The `xsection` envelope must also agree with the authored
`section`, matching what the IFC and Rhino sinks already require.

The receipt reports the pair honestly: `nativeGuids` plus per-leg
`nativeRotation`/`nativeRotationOffset`/`offsetMm`/`reversedAxis`, `legProfile` for
the derived single angle actually built, `profile` keeping its usual meaning of the
authored designation, and a `tekla-double-angle-materialized-as-pair` warning naming
both. Because one record owns two native parts, a native connection to a
double-angle member is ambiguous, so bolt, weld and boolean-cut participants that
reference one are refused explicitly instead of silently attaching to a single leg.
`tee` remains explicitly unsupported.

## Drill

The v0.31 release proved 13/20 prompts PASS against live Tekla 2026 (see [`docs/superpowers/handoffs/2026-05-19-v031-tekla-exec-live.md`](../docs/superpowers/handoffs/2026-05-19-v031-tekla-exec-live.md)). The 7 misses were Tekla domain issues, not substrate failures. Prompt fixtures live at `cli-tekla/Ingest/Output/prompt-*.json`.

```powershell
# Reproduce a drill prompt
Get-Content cli-tekla\Ingest\Output\prompt-01.json | & $awareTekla --json-stdin
```

## Architecture

```
AI orchestrator
   ↓ JSON over stdin: { verb: "exec", code: "...", args: {...} }
aware-tekla.exe (this sidecar, net48)
   ├─ AssemblyResolve handler: probe Tekla.Structures.*.dll from install bin + Net48Runtime
   ├─ Roslyn scripting host: compile user code with Tekla refs
   ├─ ExecGlobals(model=dynamic, args=dict)
   └─ Operation.* dispatch for send-status / launch / close
       ↓ in-process .NET Remoting
Tekla Structures 2026.0 (user's running Tekla, model open)
   └─ Open API responds, mutates the model database
```

## Difference from cli-rhino

| Aspect | cli-tekla | cli-rhino |
|---|---|---|
| Runtime | .NET Framework 4.8 (forced by Tekla Open API) | .NET 10 |
| Scripting host | Roslyn IN-sidecar; loads Tekla.Structures.*.dll | Delegates to rhinocode CLI (McNeel-shipped) |
| Code length | ~1500 lines (handles all the assembly-resolution complexity) | ~500 lines (rhinocode does the heavy lifting) |
| Multi-instance | PID-filtered with per-PID child sidecars (--all fans out) | Named-pipe ID via `--rhino <pipeId>` flag |
| Lessons | The original; pattern source | Wraps-vs-builds tradeoff explored in [v0.32 design](../docs/superpowers/specs/2026-05-19-rhino-exec-design.md) |
