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
