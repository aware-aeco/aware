# `cli-rhino` — Rhino 8 runtime sidecar

A small .NET 10 binary that gives the AWARE runtime programmatic access to a user-running Rhino 8 instance. Wraps McNeel's `rhinocode` CLI rather than rebuilding the in-Rhino Roslyn host ourselves.

Companion to [`cli-tekla`](../cli-tekla/) (Tekla Open API sidecar). Same `{verb, code, args}` stdin-JSON contract; different vendor.

Spec / design: [`docs/superpowers/specs/2026-05-19-rhino-exec-design.md`](../docs/superpowers/specs/2026-05-19-rhino-exec-design.md).
Implementation plan: [`docs/superpowers/plans/2026-05-19-rhino-exec.md`](../docs/superpowers/plans/2026-05-19-rhino-exec.md).
Drill recipe: [`docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md`](../docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md).

## Verbs

| Verb | Purpose |
|---|---|
| `exec` | Compile + run an ad-hoc C# script against `RhinoDoc.ActiveDoc`. The bridge from catalog (knowledge) to live host (execution). |
| `list-instances` | Enumerate running Rhino instances (PID, version, active doc, named-pipe ID for multi-instance routing). |
| `send-status` | Display a transient status-bar message in Rhino. |
| `launch` | Spawn a fresh Rhino with optional `model_path` + auto `StartScriptServer`. |
| `close` | Clean shutdown via `_-Exit _N` (no save) or `_-Exit _Y` (save); `force: true` falls back to `Process.Kill`. |

All five mirror cli-tekla's surface.

## Prerequisites

- Rhino 8.11+ installed (bundles `rhinocode.exe`)
- Script Server active in Rhino — run `StartScriptServer` once per session (or use `launch` with `start_script_server: true`, the default)
- `rhinocode.exe` discoverable: standard install path `C:\Program Files\Rhino 8\System\rhinocode.exe`, on PATH, or set `RHINOCODE_EXE` env var

## Build

```powershell
dotnet build cli-rhino/cli-rhino.csproj -c Debug

# Release single-file self-contained
dotnet publish cli-rhino/cli-rhino.csproj -c Release -r win-x64 --self-contained `
  -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true
# -> bin/Release/net10.0-windows/win-x64/publish/aware-rhino.exe  (~83 MB)
```

## Quick usage

```powershell
$awareRhino = "cli-rhino\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe"

# Discover live Rhinos
'{"verb":"list-instances"}' | & $awareRhino --json-stdin

# Run a snippet
'{"verb":"exec","code":"return Rhino.RhinoDoc.ActiveDoc.Objects.Count;"}' | & $awareRhino --json-stdin

# Status bar message
'{"verb":"send-status","message":"hello from AWARE"}' | & $awareRhino --json-stdin

# Spawn Rhino with a model
'{"verb":"launch","model_path":"C:\\Models\\hello.3dm"}' | & $awareRhino --json-stdin

# Clean shutdown
'{"verb":"close"}' | & $awareRhino --json-stdin
```

## Exec contract

The user-supplied `code` is wrapped in a synthetic `static object? __AwareRun(IDictionary<string, object?> args)` local function. Use `return X;` to send a value back.

```json
{
  "verb": "exec",
  "version": "8",                                  // optional
  "rhino_id": "rhinocode_remotepipe_75029",        // optional; multi-instance routing
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc; return doc.Objects.Count;",
  "args": { "layer": "Default" }
}
```

Receipt:
```json
{
  "ok": true,
  "result": 42,
  "host": "rhino",
  "host_pid": 31204,
  "host_version": "8.13",
  "rhino_id": "rhinocode_remotepipe_75029",
  "verb": "exec",
  "stdout_log": "",
  "delivered_at": "2026-05-19T..."
}
```

Errors come back as `{ "ok": false, "error": "...", "stack": "...", ... }` — the AI can re-draft.

## Tests

```powershell
dotnet test cli-rhino/Tests/cli-rhino.Tests.csproj
```

Runs without Rhino installed — integration tests use a stub `rhinocode.cmd` shipped under `cli-rhino/Tests/stub-rhinocode/`. Test parallelism is disabled (shared env var coordination with the stub).

## Drill

Live 20-prompt drill against a running Rhino:

```powershell
powershell -ExecutionPolicy Bypass -File cli-rhino/Ingest/run-drill.ps1
# -> cli-rhino/Ingest/Output/drill-summary.md
```

## Architecture

```
AI orchestrator
   ↓ JSON over stdin: { verb: "exec", code: "...", args: {...} }
aware-rhino.exe (this sidecar, .NET 10)
   ├─ ScriptWrapper:    split usings/body, generate temp .cs
   ├─ RhinocodeClient:  shell out to rhinocode.exe with --rhino routing
   └─ Receipts:         shape stdout → {ok,result,host,verb,delivered_at}
       ↓ ($PATH)
rhinocode.exe (McNeel-shipped, Rhino 8.11+)
   ↓ named-pipe IPC → Rhino's Script Server
Rhino.exe (user's running Rhino, StartScriptServer active)
   └─ Roslyn-in-Rhino executes the .cs against RhinoDoc.ActiveDoc
```

## Curated workflow verbs

The rhino-8 agent at [`20-agents/aeco/architecture/rhino-8/`](../20-agents/aeco/architecture/rhino-8/) declares a growing set of curated workflow verbs (v0.10 framework). These are recipes — AI reads the verb's manifest entry + its `commands/<name>.md`, then composes the right `exec` call. Examples: `view.capture-bitmap`, `layer.set-by-pattern`, `block.list-instances`, `mesh.repair`, `export.layouts-to-pdf`.

## What's NOT in this sidecar

- Headless Roslyn fallback (geometry-only without a live Rhino)
- Mac / Linux (rhinocode + Rhino.exe paths are Windows-specific)
- Catalog self-patching loop (rhino-8 catalog has no empty enums, so unneeded)

These are tracked as v0.34+ candidates if needed.
