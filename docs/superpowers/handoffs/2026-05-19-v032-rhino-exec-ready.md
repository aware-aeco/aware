# v0.32 — rhino.exec ready for live drill (2026-05-19)

The `aware-rhino` runtime sidecar is built, tested, and waiting on the
20-prompt drill against a live Rhino 8. Pawel: when you wake up, follow the
"Run the drill" section below.

## What shipped

- **`cli-rhino/`** — .NET 10 single-file sidecar (publishes ~15 MB). Three verbs:
  `exec`, `list-instances`, `send-status`. Wraps McNeel's `rhinocode` CLI
  rather than rebuilding the in-Rhino Roslyn bridge ourselves (decisive
  architectural win vs cli-tekla — ~⅓ the code).
- **21 unit + integration tests, all green** (12 ScriptWrapper + 6 RhinocodeClient + 3 Exec).
  Tests run without Rhino installed via a stub `rhinocode.cmd` under
  `cli-rhino/Tests/stub-rhinocode/` that echoes a canned sentinel block.
- **End-to-end stub validation** — the full pipeline (Program → ScriptWrapper →
  RhinocodeClient → stub → sentinel parse → receipt envelope) was exercised
  against all 20 prompt fixtures via `run-drill.ps1`. 20/20 PASS through the
  pipeline. Live-Rhino is the only thing that hasn't been exercised.
- **20 prompt fixtures** under `cli-rhino/Ingest/Output/prompt-*.json` — mix
  of read/write/ops/file/meta, same scoring shape as v0.31 tekla.exec drill.
- **`run-drill.ps1` harness** — iterates fixtures, writes
  `cli-rhino/Ingest/Output/drill-summary.md` (gitignored, regenerated per run)
  with PASS/FAIL table and full receipt JSON per prompt.
- **`rhino-8/manifest.yaml`** — `transport.cli.binary: aware-rhino` was
  already present; this PR adds the three command entries.
- **Spec + plan + this handoff** committed under `docs/superpowers/`.

## What's NOT shipped (deferred to v0.33)

- `launch` verb (Rhino has no Bypass.ini equivalent — needs design work)
- `close` verb (save-dialog handling needs care)
- Headless Roslyn-against-RhinoCommon fallback (geometry-only path; ~2× ship time)
- Mac/Linux (rhinocode on Mac has its own path conventions)
- Catalog self-patching loop (mechanical port from tekla)
- Wiring into `aware app install`'s sidecar-distribution flow (same status as cli-tekla)

## Run the drill (5 minutes)

```powershell
# 1. Launch Rhino 8 (8.11+), open a 3dm model with some objects to play with.
# 2. In Rhino's command prompt, type:
StartScriptServer

# 3. From the repo root, publish the sidecar (one-time):
cd C:\Users\bimst\source\repos\aware
dotnet publish cli-rhino/cli-rhino.csproj -c Release -r win-x64 --self-contained `
  -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true

# 4. Sanity check: list instances should find your Rhino
$awareRhino = "cli-rhino\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe"
'{"verb":"list-instances"}' | & $awareRhino --json-stdin
# Expected: { "status": "ok", "instances": [ { "pid": ..., "version": "8.13",
#            "rhino_id": "rhinocode_remotepipe_..." } ] }

# 5. Run the 20-prompt drill:
powershell -ExecutionPolicy Bypass -File .\cli-rhino\Ingest\run-drill.ps1

# 6. Inspect results:
notepad .\cli-rhino\Ingest\Output\drill-summary.md
```

If Rhino is installed but `rhinocode.exe` isn't found, set:
```powershell
$env:RHINOCODE_EXE = "C:\Program Files\Rhino 8\System\rhinocode.exe"
```

## How to interpret PASS/FAIL

The harness scores PASS if `receipt.ok == true` (or `receipt.status == "ok"`).
It does NOT check semantic correctness — a wrong-layer move still passes if
rhinocode returned a result. Eyeball the result JSON in the summary for actual
behavior. Same scoring approach as v0.31 tekla.exec.

Prompts 02, 12-17, 19 require selected objects in the doc. Select something
appropriate before running each, or accept the "no objects selected" /
"need at least 2 selected breps" graceful failures as the expected result.

## Architectural notes (for the v0.33 planning session)

- rhinocode CLI's `script` subcommand takes a file path, not stdin. We write
  a per-call temp .cs file under `%TEMP%`, delete it after run.
- C# 9 top-level statements are the script's surface. We wrap the user's body
  in `static object? __AwareRun(IDictionary<string,object?> args)` so
  `return X;` works -- same UX as tekla.exec.
- `args` ships from sidecar → script via the `AWARE_ARGS_JSON` env var; the
  wrapper deserializes into `Dictionary<string,object?>`.
- Results travel back via `__AWARE_RESULT_BEGIN__` / `__AWARE_RESULT_END__`
  sentinel markers on stdout. Anything outside the markers lands in
  `receipt.stdout_log` for diagnostics.
- Compile errors surface as `{ok:false, error:"CS0103: ...", stack:"..."}` --
  the AI can re-draft.
- `RHINOCODE_EXE` env var overrides discovery; standard install path
  (`C:\Program Files\Rhino 8\System\rhinocode.exe`) is auto-detected.
- The build machine here had no Rhino installed -- we developed against a
  stub `rhinocode.cmd` to lock the wire format. Discovery + shell-out + sentinel
  parsing all verified against the stub.

## Open issues to surface during the live drill

1. **Top-level statement ordering** -- our wrapper places top-level code BEFORE
   the local function. C# 9 requires this. If any prompt fails with CS8803
   ("Top-level statements must precede namespace and type declarations"), the
   wrapper is buggy and needs the local function moved differently.
2. **JSON serialization of Rhino types** -- `JsonSerializer` with
   `ReferenceHandler.IgnoreCycles` and `MaxDepth=6` should handle most things,
   but native pointer-backed types (`Brep`, `Mesh`, raw `Curve`) may explode.
   Each fixture projects to JSON-friendly shapes (numbers, strings, dicts).
   If a prompt fails with a serialization stack trace, the fixture needs to
   project harder before returning.
3. **rhinocode --rhino routing** -- the named-pipe ID from `list-instances`
   is used as the `--rhino` argument. Per McNeel docs this should work; if a
   multi-instance test crosses wires, switch to filtering by `processId`.
4. **Top-level using directives** -- The wrapper places all preamble usings
   at the top. If user-supplied `using` lines accidentally land INSIDE the
   `__AwareRun` body (i.e. the splitter missed them), the script won't compile.
   ScriptWrapperTests.cs locks the splitter behaviour; if a prompt's `using`s
   look weird in `bin\Debug\...\aware-rhino-exec-<guid>.cs` artifacts, that's
   the place to look.

## TEMP-path quirk on this build machine

The development machine had a quirky environment where bash sees
`%TEMP%` resolving to `C:\Users\bimst\AppData\Local\Temp;` (with a trailing
semicolon, short name `TEMP_~1`) while .NET's `Path.GetTempPath()` returns
`C:\Users\bimst\AppData\Local\Temp` (no semicolon). They are two physically
different NTFS directories. The stub-rhinocode tests work around this by
using an `AWARE_STUB_DUMP_DIR` env var so the .cmd stub and the .NET test
agree on the artifact path. Production code is unaffected -- it doesn't
write to %TEMP% from cmd.exe; only the test stub does.

## Branch + PR

- Branch: `feature/v0.29-runtime-hello-world` (same as v0.30 / v0.31).
- Commits: atomic per plan task -- see `git log --oneline origin/main..HEAD`.
  No `Co-Authored-By: Claude` trailers.
- PR opened: see PR URL in the session output / GitHub notifications.
- Merge after live drill confirms PASS rate at least matches tekla's 13/20.
