# v0.32 — rhino.exec design (2026-05-19)

The bridge from v0.30 Rhino-8 catalog (knowledge) to a live Rhino host (execution). Fans out the `*.exec` pattern proven in v0.31 (Tekla) to the next vendor.

## Goal

Ship `aware-rhino` runtime sidecar with three verbs: `exec`, `list-instances`, `send-status`. Drive a 20-prompt drill against a running Rhino 8 to prove end-to-end: AI reads the `rhino-8` catalog → drafts C# → ships through `aware-rhino.exec` → operates on real `RhinoDoc.ActiveDoc` objects → returns structured JSON.

## Architecture

`aware-rhino.exe` is a .NET 10 single-file binary that translates AWARE's `{verb, code, args}` stdin-JSON contract into [McNeel `rhinocode` CLI](https://developer.rhino3d.com/guides/scripting/advanced-cli/) invocations against a user-running Rhino 8.11+. Rhino must have **`StartScriptServer`** active (one-time per Rhino session — McNeel does not start it on launch). The sidecar owns the AWARE-shaped envelope and the C#-script wrapper that gives users `return X;` semantics; rhinocode owns the in-Rhino Roslyn host, named-pipe IPC, and multi-instance routing via `--rhino <pipeId>`.

This is fundamentally different from `cli-tekla`, which had to load Tekla.Structures DLLs directly and host Roslyn in-sidecar — because Tekla has no `rhinocode` analog. For Rhino we stand on McNeel's shoulders.

```
AI orchestrator
   └─ JSON over stdin:
      { verb: "exec", code: "...", args: {...} }
       │
       ▼
aware-rhino.exe (this sidecar, .NET 10)
   ├─ ScriptWrapper: split usings/body, generate temp .cs
   ├─ RhinocodeClient: shell out to rhinocode.exe with --rhino routing
   └─ Receipts: shape stdout → {ok,result,host,verb,delivered_at}
       │
       ▼ ($PATH)
rhinocode.exe (McNeel-shipped CLI, bundled with Rhino 8.11+)
   └─ named-pipe IPC → Rhino's Script Server
       │
       ▼
Rhino.exe (user's running Rhino, with StartScriptServer active)
   └─ Roslyn-in-Rhino executes the .cs against RhinoDoc.ActiveDoc
```

## Repo layout

A new `cli-rhino/` directory at repo root mirroring `cli-tekla/`:

```
cli-rhino/
├── Program.cs            # entry point: argv parsing + verb dispatch
├── cli-rhino.csproj      # net10.0-windows; PublishSingleFile + ReadyToRun
├── Verbs/
│   ├── Exec.cs           # JSON in → wrap user code → rhinocode script → parse result
│   ├── ListInstances.cs  # rhinocode list --json → reshape
│   └── SendStatus.cs     # generated C# → rhinocode script → parse
├── RhinocodeClient.cs    # one place to shell out (path discovery, --rhino routing,
│                         # stdout/stderr/exit-code capture, error normalization)
├── ScriptWrapper.cs      # split using-directives from body, wrap in __AwareRun(),
│                         # inject preamble + result-capture, write to a per-call temp .cs
├── Receipts.cs           # {ok, result, host, verb, delivered_at, ...} envelope
├── Ingest/
│   └── Output/
│       ├── prompt-01.json … prompt-20.json   # 20-prompt drill fixtures
│       └── drill-summary.md                  # post-drill PASS/FAIL summary (written by harness)
└── Tests/
    ├── cli-rhino.Tests.csproj   # xUnit
    ├── ScriptWrapperTests.cs    # pure logic; no Rhino, no rhinocode
    ├── ReceiptsTests.cs         # shape conformance
    └── stub-rhinocode/          # tiny stub binary for CI (cmd/.bat that echoes canned JSON)
```

Plus a tiny edit to the Rhino agent manifest:

```yaml
# 20-agents/aeco/architecture/rhino-8/manifest.yaml
transport:
  cli:
    binary: aware-rhino

commands:
  exec:
    lifecycle: single
    description: Compile + run an ad-hoc C# script against the active Rhino document via rhinocode.
  # …existing entries…
```

## Verb contracts

All verbs accept stdin JSON; all emit a single stdout JSON receipt; exit 0 = success, exit 1 = no matching Rhino, exit 2 = error.

### `exec`

**Input:**
```json
{
  "verb": "exec",
  "version": "8",                              // optional; default = first running Rhino
  "rhino_id": "rhinocode_remotepipe_75029",    // optional; overrides version, picks exact instance
  "code": "var doc = Rhino.RhinoDoc.ActiveDoc;\nreturn new { count = doc.Objects.Count };",
  "args": { "layer": "Default" }
}
```

**Output (success):**
```json
{
  "ok": true,
  "result": { "count": 42 },
  "host": "rhino",
  "host_pid": 31204,
  "host_version": "8.13",
  "rhino_id": "rhinocode_remotepipe_75029",
  "verb": "exec",
  "stdout_log": "",
  "delivered_at": "2026-05-19T21:14:22.871Z"
}
```

**Output (compile or runtime error):**
```json
{
  "ok": false,
  "error": "CS0103: 'RhinoDoc' could not be found (are you missing a using directive?)",
  "stack": "…",
  "host": "rhino",
  "verb": "exec",
  "stdout_log": "",
  "delivered_at": "2026-05-19T..."
}
```

### `list-instances`

**Input:** none (or empty stdin).

**Output:**
```json
{
  "status": "ok",
  "instances": [
    {
      "pid": 31204,
      "version": "8.13",
      "rhino_id": "rhinocode_remotepipe_75029",
      "active_doc": "C:\\Models\\hello.3dm"
    }
  ]
}
```

Internally: `rhinocode list --json` → reshape McNeel's keys (`pipeId`, `processId`, `processVersion`) into our snake_case schema.

### `send-status`

**Input:**
```json
{ "verb": "send-status", "message": "AWARE pinged you", "version": "8" }
```

**Output:**
```json
{
  "status": "ok",
  "host": "rhino",
  "host_version": "8.13",
  "host_pid": 31204,
  "host_session_id": "rhino-31204",
  "verb": "send-status",
  "verb_result": { "message": "AWARE pinged you" },
  "delivered_at": "..."
}
```

Internally: send-status synthesizes a tiny C# snippet (`Rhino.UI.StatusBar.ShowMessage(args["message"]);`) and pipes it through the same exec path. Validates the wrapper end-to-end on every invocation.

## Exec wrapper deep-dive

The user's `code` is plain C# that may contain `using` directives at the top plus a body. Rhino 8 script-server runs files as **C# 9 top-level statements** — there is no `Main`, no class wrapper. That means a bare `return X;` at the top is an exit-code return, not a value return. To preserve tekla.exec's `return X;` UX, the sidecar wraps the user's body inside a synthetic local function.

### Generated temp file shape

For an input like:
```cs
using Rhino.Geometry;
var doc = Rhino.RhinoDoc.ActiveDoc;
return new { count = doc.Objects.Count };
```

ScriptWrapper produces a temp `.cs` file like:

```cs
// AWARE-generated script (do not edit)
using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;
using Rhino;
using Rhino.DocObjects;
using Rhino.Geometry;
using Rhino.Input;
using Rhino.Input.Custom;
using Rhino.PlugIns;
using Rhino.UI;
// user-supplied using directives, deduped:
// (Rhino.Geometry was already in the preamble — deduped)

var __awareArgsJson = Environment.GetEnvironmentVariable("AWARE_ARGS_JSON") ?? "{}";
var args = JsonSerializer.Deserialize<Dictionary<string, object>>(__awareArgsJson)
           ?? new Dictionary<string, object>();

var __result = __AwareRun(args);
Console.WriteLine("__AWARE_RESULT_BEGIN__");
Console.WriteLine(JsonSerializer.Serialize(__result, new JsonSerializerOptions {
    WriteIndented = false,
    ReferenceHandler = ReferenceHandler.IgnoreCycles,
    MaxDepth = 6,
}));
Console.WriteLine("__AWARE_RESULT_END__");

static object __AwareRun(IDictionary<string, object> args)
{
    // === user body begins ===
    var doc = Rhino.RhinoDoc.ActiveDoc;
    return new { count = doc.Objects.Count };
    // === user body ends ===
}
```

C# 9 top-level statements compile into a synthetic `Program.<Main>` method; nested static local functions (`static object __AwareRun(...) { ... }`) appear at file-bottom and are reachable from the top-level statements above. This pattern is documented and used in plenty of Rhino script samples.

### Splitting usings from body

Naive line-scan: lines matching `^\s*using\s+[A-Za-z0-9_.]+\s*;\s*$` go to the directives bucket; everything else is the body. Edge cases:
- `using (var x = ...)` — using-statement, not directive. Pattern requires the trailing semicolon after an identifier path, not after a `(`. Safe.
- Multi-line `using X = SomeNamespace.SomeType;` aliases — supported by the same regex.
- A body that starts with a comment block — fine, comments don't match the directive regex.

For v0.32 we accept that pathological inputs (e.g. `using` inside a `#if`) round-trip incorrectly. The AI generating these snippets never does that.

### Args injection

The sidecar serializes the input JSON's `args` block and sets it on the child process as `AWARE_ARGS_JSON=<json>` environment variable. The wrapper deserializes it into a `Dictionary<string, object>` and passes it to `__AwareRun(args)`. No marshaling magic, no codegen per-call — args are always a string-keyed JSON object.

### Result capture

The wrapper prints the result between sentinel markers:
```
__AWARE_RESULT_BEGIN__
<json>
__AWARE_RESULT_END__
```

The sidecar reads child stdout line-by-line, finds the block between sentinels, parses the JSON. Everything else from stdout (user's `Console.WriteLine` debug prints, Rhino's own chatter) is captured into `stdout_log` for diagnostics. If sentinels are missing, the receipt is `{ok:false, error:"result sentinel not found", stdout_log:"<full output>"}`.

### Result serialization caveats

`JsonSerializerOptions` is set to `ReferenceHandler.IgnoreCycles` (Rhino geometry types like `Brep` have parent-child cycles) and `MaxDepth = 6` (defensive). Native Rhino types with unmanaged pointers (`Brep`, `Mesh`, `Curve`) MAY still throw during serialization — the user's snippet is expected to project these to JSON-friendly shapes (numbers, strings, dicts of primitives) before returning. tekla.exec has the same constraint. Documented in the agent's exec command description.

### Compile errors

rhinocode prints the Roslyn diagnostics to stderr when compilation fails. The sidecar captures stderr and surfaces it as `{ok:false, error:"<first diagnostic line>", stack:"<full diagnostic output>"}`. The AI can re-draft based on the diagnostic.

## rhinocode discovery & routing

`RhinocodeClient` discovers `rhinocode.exe` in this order:
1. `RHINOCODE_EXE` env var (override / dev escape hatch)
2. `C:\Program Files\Rhino 8\System\rhinocode.exe` (standard install)
3. `where rhinocode` (PATH search; works if user followed McNeel's PATH-setup recipe)

If none found, exec/list/send-status all fail with exit 3 and a guide message that tells the user where to install Rhino or set the env var.

Multi-instance routing: when `rhino_id` is supplied on input, the sidecar passes `--rhino <id>` to rhinocode. When `version` is supplied without `rhino_id`, the sidecar calls `rhinocode list --json` first, picks the first instance whose `processVersion` startswith the requested `version`, then routes to it. When neither is supplied, the sidecar lets rhinocode pick the default (its own first-match rule).

## Tests

Three layers, all runnable without Rhino installed:

1. **Unit — `ScriptWrapperTests.cs`** (~15 tests): split usings vs body, dedupe usings, wrap simple body, wrap body with no `return`, escape edge cases, preamble correctness.
2. **Unit — `ReceiptsTests.cs`** (~6 tests): success receipt shape, error receipt shape, result-sentinel parsing.
3. **Integration — stub-rhinocode** (~6 tests): integration tests use a stub `rhinocode.cmd` on PATH that reads stdin/argv and echoes a canned response. Verifies the shell-out, env-var setting, stdout parsing, exit-code translation. No Rhino, no Roslyn.

CI: `dotnet test` runs all three layers. No Rhino dependency.

## 20-prompt drill

Drill fixtures live at `cli-rhino/Ingest/Output/prompt-NN.json`, one per prompt. Each is the exact stdin JSON to pipe through `aware-rhino.exe`. Categories mirror tekla.exec's drill mix (read / write / ops):

| #     | Category | Prompt |
|------:|----------|---|
|  1    | read     | Count objects in active doc by layer |
|  2    | read     | List GUIDs of selected objects |
|  3    | read     | Bounding box of all visible geometry |
|  4    | read     | List all layers with object counts |
|  5    | read     | List blocks (instance definitions) and their member counts |
|  6    | write    | Add a line from (0,0,0) to (1000,0,0) |
|  7    | write    | Add a circle radius=500 at origin in XY plane |
|  8    | write    | Add a sphere radius=250 at (1000,1000,0) |
|  9    | write    | Add a box 1000×1000×500 at origin |
| 10    | write    | Create a layer "AWARE" with random color |
| 11    | write    | Add 10 random points and group them |
| 12    | write    | Set selected objects' layer to "AWARE" |
| 13    | ops      | Move selected objects by (100,100,0) |
| 14    | ops      | Rotate selected objects 45° about Z at origin |
| 15    | ops      | Scale selected objects by 2 about origin |
| 16    | ops      | Mirror selected objects across the YZ plane |
| 17    | ops      | Boolean union all selected breps |
| 18    | file     | Save current document as `%TEMP%/aware-drill.3dm` |
| 19    | file     | Export selected as STL to `%TEMP%/aware-drill.stl` |
| 20    | meta     | List viewports with name + projection + camera location |

The drill harness (`cli-rhino/Ingest/run-drill.ps1`) iterates the 20 fixtures, ships each via `aware-rhino.exe exec --json-stdin`, captures the receipt, writes a markdown summary table to `cli-rhino/Ingest/Output/drill-summary.md` with PASS / FAIL per prompt + the result blob.

PASS = `receipt.ok == true && receipt.result != null && (no exception class in receipt.stack)`. FAIL = either `ok == false` or `result == null` when result was expected. The harness does not assert the *semantic* correctness of the result (a wrong-layer move still "passes" if rhinocode returned a result) — that's eyeball validation in the morning. This matches how tekla.exec's drill was scored.

## Build, run, distribution

Build:
```powershell
cd C:\Users\bimst\source\repos\aware\cli-rhino
dotnet publish -c Release -r win-x64 --self-contained `
  -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true
# → bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe (~15 MB)
```

Smoke test (no Rhino required — just the path-discovery + JSON envelope):
```powershell
'{"verb":"exec"}' | & .\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe --json-stdin
# Expected: {"ok":false,"error":"missing required field: code", ...}
```

Live drill (Rhino + StartScriptServer required):
```powershell
# 1. Launch Rhino 8, open a model.
# 2. In Rhino, run: StartScriptServer
# 3. From repo root:
powershell -ExecutionPolicy Bypass -File cli-rhino/Ingest/run-drill.ps1
# → cli-rhino/Ingest/Output/drill-summary.md  (PASS/FAIL table)
```

The main `aware` CLI (Rust, under `cli/`) is unchanged in this PR. It already invokes sidecars by binary name from the manifest; once `aware-rhino.exe` is installed and the rhino-8 manifest has `transport.cli.binary: aware-rhino`, the existing `aware app run` path picks it up. (Wiring into `aware app install`'s sidecar-distribution flow is a v0.33 follow-up — same status as cli-tekla today.)

## What's NOT in this PR (non-goals)

| Item | Why deferred |
|---|---|
| `launch` verb | Rhino has no clean equivalent of Tekla's Bypass.ini for headless startup. Needs its own design pass. |
| `close` verb | Rhino's save-prompt dialog + the difference between "close model" vs "exit Rhino" warrants careful handling. v0.33. |
| Headless Roslyn-against-RhinoCommon fallback | Would let geometry-only scripts run without a live Rhino. Real value-add but ~2× ship time. v0.33. |
| Mac/Linux | Rhino has Mac; `rhinocode` on Mac has its own path conventions. v0.33 along with cross-platform `aware` CLI. |
| Catalog self-patching loop (the Tekla v0.30.1 pattern, on Rhino) | Mechanical port once exec is proven. v0.33. |
| Wiring into `aware app install` sidecar-distribution | Same status as cli-tekla today. Sidecar distribution is its own roadmap item. |
| Connection / connection-rule attribute schemas (Rhino doesn't have these — Rhino-specific equivalent would be Layers + Blocks + UserDictionary; covered in the drill) | n/a |

## Open risks

1. **rhinocode wire-format assumptions.** I'm inferring rhinocode's stdin/stdout/exit-code conventions from McNeel's online docs. They're sparse on the C# script contract (top-level statements + sentinel parsing). The build-and-test loop will surface any mismatch immediately — the integration tests with the stub binary lock our assumed contract, and the live drill validates against the real rhinocode.
2. **No live-Rhino on the build machine.** Pawel will run the drill in the morning. Everything else (build, unit tests, stub-integration tests, fixture authoring) lands without Rhino. The handoff doc tells him exactly what to run.
3. **Top-level + local function ordering.** C# 9 requires top-level statements to come BEFORE any type declarations. Local functions at file bottom are fine because they're inside the synthetic `Program.<Main>`. Tested in unit suite to lock the wrapper shape.
4. **RhinocodeClient PATH discovery on a fresh machine.** If Rhino is installed but `rhinocode.exe` isn't on PATH, we fall back to the standard install path. If neither works, the error message tells the user the two fixes.

## Acceptance criteria (DoD)

- `cli-rhino/` builds cleanly via `dotnet publish` → single-file `aware-rhino.exe`.
- All three test layers pass: `dotnet test cli-rhino/Tests/`.
- `aware-rhino.exe --help` works.
- `'{"verb":"exec"}' | aware-rhino.exe --json-stdin` returns a structured error envelope (not a stack trace) — proves no-Rhino path is sane.
- 20 prompt fixtures committed under `cli-rhino/Ingest/Output/`.
- `cli-rhino/Ingest/run-drill.ps1` exists and is documented in the handoff.
- `rhino-8/manifest.yaml` updated with `transport.cli.binary` + `exec` command entry.
- Handoff doc `docs/superpowers/handoffs/2026-05-19-v032-rhino-exec-ready.md` written.
- PR opened against main with plain-English summary.
- **Drill run itself** is owed by Pawel in the morning (no Rhino on the build machine).
