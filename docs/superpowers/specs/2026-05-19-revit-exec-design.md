# v0.33 — revit.exec design (2026-05-19)

The bridge from v0.30 Revit-2026 catalog (knowledge) to a live Revit host (execution). Fans the `*.exec` pattern proven in v0.31 (Tekla) and v0.32 (Rhino) to the next vendor — and, critically, to the first vendor that cannot host Roslyn directly from a sidecar process.

## Goal

Ship `aware-revit` runtime sidecar with five verbs: `exec`, `list-instances`, `send-status`, `launch`, `close`. Drive a 20-prompt drill against a running Revit 2026 to prove end-to-end: AI reads the `revit-2026` catalog → drafts C# → ships through `aware-revit.exec` → operates on real `Document` objects on Revit's main thread → returns structured JSON.

## Why the architecture differs from cli-tekla / cli-rhino

Both prior vendors had an escape hatch:

- **Tekla**: the Open API is a remoting facade. Constructing `Tekla.Structures.Model.Model()` in a *separate* .NET process attaches to the live Tekla's IPC channel — the sidecar's host process owns the API; Tekla's main thread serves it.
- **Rhino**: McNeel ships `rhinocode.exe`, an out-of-process CLI that talks to an in-Rhino script server over a named pipe. The sidecar shells out; McNeel owns the Roslyn-in-Rhino hosting.

**Revit has neither.** The Revit API can only be called from the Revit main thread inside the Revit process. There is no shipped out-of-process CLI. The only safe cross-thread mechanism is `IExternalEventHandler` — and to register one you have to be loaded as an `IExternalApplication` add-in at Revit startup.

Therefore the v0.33 sidecar is **two binaries cooperating over a named pipe**:

1. **`AwareRevit.dll`** — a Revit add-in (`IExternalApplication` + `IExternalEventHandler`) loaded by Revit at startup via an `.addin` manifest in `%PROGRAMDATA%\Autodesk\Revit\Addins\2026\`. Owns the server end of a per-process named pipe. On a request, raises its registered `ExternalEvent`, executes the user's C# (compiled via Roslyn) on Revit's main thread, and writes the receipt back to the pipe. .NET 8 (Revit 2026's runtime).
2. **`aware-revit.exe`** — the AWARE-CLI-facing sidecar. Speaks the `{verb, code, args}` stdin-JSON contract. Discovers running Revit instances (`Process.GetProcessesByName("Revit")`), connects to the add-in's pipe, ships the request, returns the receipt. .NET 8 (matches the add-in for ABI consistency and lets us share `Receipts.cs` / `Protocol.cs` as a third project).

```
AI orchestrator
   └─ JSON over stdin: { verb, code, args, version?, host_pid? }
       │
       ▼
aware-revit.exe (this sidecar, .NET 8)
   ├─ Discover: Process.GetProcessesByName("Revit") → pick target by version/pid
   ├─ Pipe client: connect to "aware-revit-<PID>"
   ├─ Send: length-prefixed JSON request
   ├─ Receive: length-prefixed JSON receipt
   └─ Emit on stdout
       │
       ▼ (named pipe)
AwareRevit.dll (in-Revit add-in, .NET 8, loaded at Revit startup)
   ├─ NamedPipeServerStream listener (background thread)
   ├─ On request: enqueue + ExternalEvent.Raise()
   └─ AwareEventHandler.Execute() (main thread)
       └─ Roslyn compile+run against current UIApplication.ActiveUIDocument
       │
       ▼
Revit's main thread → Document → returns JSON-serialized result
```

## Repo layout

A new `cli-revit/` directory at repo root, mirroring `cli-tekla/` / `cli-rhino/` shape but with two .csproj files:

```
cli-revit/
├── AwareRevit/                   # in-Revit add-in (loaded by Revit)
│   ├── AwareRevit.csproj         # net8.0-windows; references RevitAPI.dll (CopyLocal=false)
│   ├── AddInApplication.cs       # IExternalApplication: OnStartup → start pipe server thread
│   ├── ExecuteEventHandler.cs    # IExternalEventHandler: Execute() on Revit main thread
│   ├── PipeServer.cs             # NamedPipeServerStream listener, request/response framing
│   ├── ScriptEngine.cs           # Roslyn compile + run against RevitAPI loaded in-Revit
│   ├── ResultSerializer.cs       # JSON output, Revit-aware (Element → {id,category,name})
│   └── AwareRevit.addin          # the manifest installed to ProgramData\Autodesk\Revit\Addins\2026\
├── Sidecar/                      # the aware-revit.exe binary
│   ├── cli-revit.csproj          # net8.0-windows; OutputType Exe; AssemblyName aware-revit
│   ├── Program.cs                # argv parsing + verb dispatch + pipe-client driver
│   ├── PipeClient.cs             # NamedPipeClientStream → length-prefixed JSON
│   ├── RevitProcessFinder.cs     # enumerate running Revit.exe → (pid, version, exe-path)
│   ├── Receipts.cs               # {ok, result, host, host_pid, host_version, ...}
│   └── Verbs/
│       ├── Exec.cs
│       ├── ListInstances.cs
│       ├── SendStatus.cs
│       ├── Launch.cs
│       └── Close.cs
├── Shared/                       # protocol DTOs shared between add-in and sidecar
│   ├── Shared.csproj             # net8.0 (no -windows; pure DTOs)
│   ├── PipeFrame.cs              # length-prefixed JSON serializer/deserializer
│   └── ExecRequest.cs / ExecResponse.cs etc.
├── Tests/
│   ├── cli-revit.Tests.csproj    # xUnit, net8.0-windows
│   ├── PipeFrameTests.cs         # round-trip frames without a pipe
│   ├── PipeClientTests.cs        # uses an in-process pipe-server stub
│   ├── ResultSerializerTests.cs  # Element shape, error-receipt shape
│   ├── RevitProcessFinderTests.cs # FakeProcessSnapshot injection
│   └── stub-revit-addin/         # tiny in-process pipe server that echoes canned receipts
├── Ingest/
│   ├── run-drill.ps1             # iterate fixtures, write drill-summary.md
│   └── Output/
│       └── prompt-01.json ... prompt-20.json
└── install-addin.ps1             # copies AwareRevit.dll + .addin into the Revit addin folder
```

The add-in folder for Revit 2026 (per Autodesk docs):

- Per-machine: `%PROGRAMDATA%\Autodesk\Revit\Addins\2026\`
- Per-user: `%APPDATA%\Autodesk\Revit\Addins\2026\`

`install-addin.ps1` targets the per-user folder by default (avoids elevation) and rewrites the `<Assembly>` path inside the manifest to the absolute deployed location.

## Verb contracts

All verbs accept stdin JSON; all emit a single stdout JSON receipt; exit 0 = success, exit 1 = no matching Revit, exit 2 = error, exit 3 = Revit not installed, exit 4 = ambiguous target, exit 6 = pipe connect failure (add-in not loaded).

### `exec`

**Input:**
```json
{
  "verb": "exec",
  "version": "2026",                    // optional; default = first running Revit
  "host_pid": 31204,                    // optional; overrides version for exact targeting
  "code": "var doc = uiapp.ActiveUIDocument.Document;\nreturn new { title = doc.Title };",
  "args": { "layer": "Default" }
}
```

**Output (success):**
```json
{
  "ok": true,
  "result": { "title": "Snowdon Towers Sample Architectural.rvt" },
  "host": "revit",
  "host_pid": 31204,
  "host_version": "2026",
  "verb": "exec",
  "stdout_log": "",
  "delivered_at": "2026-05-19T21:14:22.871Z"
}
```

**Output (error):**
```json
{
  "ok": false,
  "error": "CS0103: 'doc' could not be found",
  "stack": "…",
  "host": "revit",
  "verb": "exec",
  "stdout_log": "",
  "delivered_at": "2026-05-19T..."
}
```

Globals exposed to user code:
- `dynamic uiapp` — Revit's `UIApplication` (use `uiapp.ActiveUIDocument.Document` for the active doc).
- `IDictionary<string,object?> args` — the `args` block from the input JSON.

Use of `dynamic` mirrors cli-tekla and lets the sidecar avoid a compile-time reference to RevitAPI from any non-add-in code path.

### `list-instances`

**Input:** none (or empty stdin).

**Output:**
```json
{
  "status": "ok",
  "instances": [
    {
      "pid": 31204,
      "version": "2026",
      "exe_path": "C:\\Program Files\\Autodesk\\Revit 2026\\Revit.exe",
      "addin_loaded": true,
      "active_doc": "Snowdon Towers Sample Architectural.rvt"
    }
  ]
}
```

Internally: enumerate `Process.GetProcessesByName("Revit")`, parse version from `MainModule.FileName`, probe pipe `aware-revit-<pid>` with a short timeout to set `addin_loaded`. `active_doc` set via an exec call when `addin_loaded == true`, otherwise null.

### `send-status`

**Input:**
```json
{ "verb": "send-status", "message": "AWARE pinged you", "version": "2026" }
```

**Output:**
```json
{
  "status": "ok",
  "host": "revit",
  "host_version": "2026",
  "host_pid": 31204,
  "host_session_id": "revit-31204",
  "verb": "send-status",
  "verb_result": { "message": "AWARE pinged you" },
  "delivered_at": "..."
}
```

Internally: synthesizes a tiny C# snippet that calls `Autodesk.Revit.UI.TaskDialog.Show("AWARE", message)` (Revit has no status-bar API like Rhino's `StatusBar.ShowMessage`; a transient TaskDialog is the equivalent attention-getter) and routes through `exec`. Same end-to-end validation pattern as cli-rhino.

### `launch`

**Input:**
```json
{
  "verb": "launch",
  "version": "2026",
  "model_path": "C:/Program Files/Autodesk/Revit 2026/Samples/Snowdon Towers Sample Architectural.rvt",
  "language": "ENU"
}
```

**Output:**
```json
{
  "status": "ok",
  "host": "revit",
  "host_version": "2026",
  "host_pid": 41832,
  "verb": "launch",
  "verb_result": {
    "model_path": "C:/Program Files/Autodesk/Revit 2026/Samples/Snowdon Towers Sample Architectural.rvt",
    "note": "Revit is starting; poll list-instances until addin_loaded=true to confirm IPC readiness (typically ~20s)"
  },
  "delivered_at": "..."
}
```

Mirrors `cli-tekla launch`: resolves `C:\Program Files\Autodesk\Revit <version>\Revit.exe`, starts it, returns the PID immediately. Does NOT wait for Revit's UI to be ready — caller polls `list-instances` for `addin_loaded == true`.

### `close`

**Input:**
```json
{ "verb": "close", "version": "2026", "force": false }
```

**Output:**
```json
{
  "status": "ok",
  "host": "revit",
  "host_version": "2026",
  "host_pid": 31204,
  "mode": "clean",
  "verb": "close",
  "delivered_at": "..."
}
```

Clean path (default): sends an `exec`-shaped request to the add-in that calls `uiapp.PostCommand(RevitCommandId.LookupPostableCommandId(PostableCommand.Close))` (closes the active doc with dialog suppression) then `uiapp.Application.Exit()` (Revit graceful shutdown). Waits up to 30s for `Process.HasExited`. Force path (`force=true`): `Process.Kill()` after 5s grace.

If the add-in isn't loaded (pipe connect fails) the clean path is impossible — same data-loss-acknowledgement guard as cli-tekla.

## Pipe protocol

Per-Revit-process pipe name: `aware-revit-<PID>`.

Frame format (each direction): a 4-byte big-endian length prefix, then UTF-8 JSON. One frame per request, one frame per response. The add-in's pipe is `PipeDirection.InOut` with `PipeOptions.Asynchronous`, max 1 connection at a time (the sidecar serializes its requests). Pipe access mode: per-user (no cross-user surface — Revit and the sidecar run as the same user).

Request:
```json
{ "id": "uuid", "verb": "exec", "code": "...", "args": { ... }, "timeout_seconds": 60 }
```

Response (one of):
```json
{ "id": "uuid", "ok": true,  "result": { ... },             "stdout_log": "...", "host_version": "2026", "host_pid": 31204 }
{ "id": "uuid", "ok": false, "error": "...", "stack": "...", "stdout_log": "...", "host_version": "2026", "host_pid": 31204 }
```

The pipe DTOs live in `Shared/` and are referenced by both binaries — single source of truth. Sidecar wraps them in the AWARE-shaped envelope (adds `delivered_at`, `verb`, `host="revit"`) on the way out.

## Exec wrapper

Same shape as cli-tekla and cli-rhino — split user code into `using` directives + body, dedupe against a preamble, wrap the body in a synthetic local function so `return X;` works. The wrapper lives **inside the add-in** (`ScriptEngine.cs`) because Roslyn must reference the in-Revit RevitAPI assembly that's already loaded — adding it as a script reference is straightforward via `MetadataReference.CreateFromFile(typeof(Autodesk.Revit.UI.UIApplication).Assembly.Location)`.

Preamble:
```cs
using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Text.Json;
using Autodesk.Revit.ApplicationServices;
using Autodesk.Revit.Attributes;
using Autodesk.Revit.DB;
using Autodesk.Revit.DB.Structure;
using Autodesk.Revit.UI;
using Autodesk.Revit.UI.Selection;
```

References (resolved from already-loaded assemblies in the Revit AppDomain):
- RevitAPI.dll, RevitAPIUI.dll, RevitAPIIFC.dll
- System.Runtime, System.Collections, System.Linq, System.Text.Json
- Microsoft.CSharp (for `dynamic` if user code uses it)

Globals exposed via `CSharpScript.Create<object>(..., globalsType: typeof(ExecGlobals))`:
```cs
public sealed class ExecGlobals
{
    public dynamic uiapp = null!;                            // Autodesk.Revit.UI.UIApplication
    public IDictionary<string, object?> args = new Dictionary<string, object?>();
}
```

## Transaction semantics

Revit requires every `Document` write to occur inside a `Transaction`. Two policies:

1. **User-managed (default)**: user code is responsible for opening / committing transactions. The exec handler doesn't wrap anything. This keeps the pattern symmetric with cli-tekla (where user code calls `model.CommitChanges()` explicitly).
2. **Auto-wrap (opt-in)**: if request includes `"transaction": "auto"`, the add-in wraps the user code in `using (var tx = new Transaction(doc, "AWARE exec")) { tx.Start(); ...user...; tx.Commit(); }`. On thrown exception, `tx.RollBack()`.

The 20-prompt drill fixtures use user-managed transactions explicitly so the wrapper logic doesn't hide behaviour. Auto-wrap is documented and tested but not the default.

## Result serialization

Revit elements are complex (often hold COM references to model state) so blind `JsonSerializer.Serialize` will explode. The add-in uses a defensive serializer that:

- Returns primitives as-is (`bool`, `int`, `long`, `double`, `string`, `Guid`, `DateTime`).
- For `Element`: emits `{ id, category, name, level? }`.
- For `ElementId`: emits the integer value.
- For `XYZ`: emits `{ x, y, z }`.
- For `Parameter`: emits `{ name, type, value }`.
- For other Revit types: tries `ToString()` with a type-name hint.
- For `IDictionary` / `IEnumerable`: recurses.
- For POCO / anonymous types: `JsonSerializer` with `MaxDepth=6` and `ReferenceHandler.IgnoreCycles`.

This mirrors cli-tekla's `SerializeResult` defensively but adds Revit-specific shapes for the most common return types (Elements, IDs, points).

## Testing approach

| Layer | Test type | Where | Runs without Revit? |
|---|---|---|---|
| `ScriptEngine.SplitUsingsAndBody` | unit (xUnit) | `Tests/ScriptEngineTests.cs` | yes — pure regex logic |
| `PipeFrame.Read/Write` | unit | `Tests/PipeFrameTests.cs` | yes — uses `MemoryStream` |
| `PipeClient.Roundtrip` | integration | `Tests/PipeClientTests.cs` | yes — in-process pipe-server stub on a worker thread |
| `Receipts.*` | unit | `Tests/ReceiptsTests.cs` | yes — shape conformance |
| `RevitProcessFinder` | unit | `Tests/RevitProcessFinderTests.cs` | yes — fake snapshot injection |
| `ResultSerializer` | unit | `Tests/ResultSerializerTests.cs` | yes — covers primitives + Dictionary + anonymous types (Revit-specific shapes are TDD'd against `Type.GetType` to avoid the dep) |
| End-to-end verb dispatch (sidecar → stub pipe server → mock add-in receipt) | integration | `Tests/EndToEndTests.cs` | yes — stub pipe server echoes canned JSON, same pattern as cli-rhino's stub `rhinocode.cmd` |
| Live drill | system | `Ingest/run-drill.ps1` | NO — requires Revit + add-in installed |

Tests assert shapes match cli-tekla / cli-rhino's receipt envelopes (for cross-vendor consistency).

## 20-prompt drill (live)

Mirrors the tekla/rhino mix — read-side heavy, three write-side with explicit transactions, file ops, meta.

| # | Category | Prompt summary | Read/Write/File |
|---|---|---|---|
| 01 | Read | Count elements by category | R |
| 02 | Read | List currently selected element ids | R |
| 03 | Read | Document title + path | R |
| 04 | Read | List all named views | R |
| 05 | Read | List families with usage count | R |
| 06 | Read | List worksets + ownership | R |
| 07 | Read | List linked models | R |
| 08 | Read | Project info (number, name, address) | R |
| 09 | Read | List levels with elevations | R |
| 10 | Write | Place a wall at origin (TX wrap, fail-soft) | W |
| 11 | Write | Place a column at origin (TX wrap, fail-soft) | W |
| 12 | Write | Set a parameter on the selected element | W |
| 13 | Ops | Get bounding box of selection | R |
| 14 | Ops | List active view's visible categories | R |
| 15 | Ops | Regenerate the document | W |
| 16 | File | Export active sheet to PDF (to %TEMP%) | F |
| 17 | File | Save model (no-op if nothing dirty) | W |
| 18 | File | Export to IFC (to %TEMP%) | F |
| 19 | Meta | Revit version + build + addon list | R |
| 20 | Meta | List all open documents | R |

Scoring: PASS if `receipt.ok == true` (matches cli-tekla / cli-rhino harness behaviour). Write-side prompts use opt-in `"transaction": "auto"` so the drill can run unattended. Pass target: ≥13/20 (matching tekla's v0.31 bar).

## What's NOT included (deferred)

- Multi-version support (this PR is 2026-only; tekla/rhino's per-version installs are out of scope here — the .addin manifest installs only to `Addins\2026\`).
- Catalog self-patching loop (mechanical port from tekla; v0.34).
- Mac (Revit isn't supported on Mac).
- Wiring into `aware app install`'s sidecar-distribution flow (same as cli-tekla / cli-rhino).
- Headless / batch-mode Revit (RevitBatchProcessor pattern). Out-of-scope; the design is interactive-Revit only.

## Open risks (will be exercised by the drill)

1. **Add-in load failure** — if Revit refuses to load `AwareRevit.dll` (signing, trust prompt, runtime mismatch), the sidecar gets `pipe not found` and can't recover without user intervention. Mitigation: clear error path + install-addin.ps1 documents the trust prompt.
2. **Roslyn-in-Revit reference resolution** — Revit's plugin-load context may not surface `MetadataReference.CreateFromFile` paths cleanly for assemblies that are already loaded as `Assembly.Load(byte[])`. If `Assembly.Location` returns empty, we fall back to a hard-coded probe under `C:\Program Files\Autodesk\Revit 2026\`.
3. **TaskDialog modality** — `send-status` uses `TaskDialog.Show` because Revit has no transient status equivalent. Modal dialog interrupts the user; acceptable for v0.33.
4. **Result serialization explosions on Element graphs** — Revit Elements hold internal pointers to the doc; naive `JsonSerializer` will throw. Defensive serializer handles known shapes; unknown types fall back to `ToString()` rather than throwing.
5. **PipeClient flakiness on first connect after Revit launch** — the add-in's `OnStartup` runs late in Revit's boot sequence; connecting too early may fail. Sidecar retries connect for up to 30s with 500ms intervals.

## Receipt-shape convergence (continuing v0.32's recommendation)

cli-tekla's receipt is `{ok, result, host, verb, delivered_at}`. cli-rhino added `host_pid`, `host_version`, `rhino_id`, `stdout_log`. v0.33 follows cli-rhino: emits `host_pid`, `host_version`, `stdout_log` — strictly additive, drops `rhino_id` (Revit's per-pipe key IS the PID so it's redundant). Recommendation: backport these fields to cli-tekla in a future PR so all three vendors emit identical envelopes.

## Branch + PR

- Branch: `v0.33-revit-exec` (this branch — already created in the worktree).
- Commits: atomic per plan task. No `Co-Authored-By: Claude` trailers. Stage specific files only.
- PR title: `feat(v0.33): revit.exec runtime sidecar — in-Revit add-in + .NET sidecar`.
- PR body mirrors v0.32: summary, architecture decision, receipt shape, test plan, what's NOT included.
