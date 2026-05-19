# v0.33 — revit.exec drilled live 20/20 (2026-05-19)

The `aware-revit` runtime sidecar plus the `AwareRevit` in-Revit add-in
shipped overnight, built, tested, and driven through a 20-prompt drill
against a live Revit 2026. **20/20 PASS** (vs the 13/20 target inherited
from tekla.exec).

## What shipped

- **`cli-revit/`** — three .NET 8 projects:
  - `Shared/` — protocol DTOs (`ExecRequest`, `ExecResponse`, `PipeFrame`).
  - `AwareRevit/` — the in-Revit add-in (loaded by Revit at startup via
    `AwareRevit.addin`). `IExternalApplication` + named-pipe server +
    `IExternalEventHandler` for main-thread marshalling + Roslyn script host
    referencing the live RevitAPI assemblies.
  - `Sidecar/` — `aware-revit.exe`, a single-file .NET 8 binary that
    discovers running Revit instances, opens the per-PID pipe, and ships
    `{verb, code, args}` requests through.
- **5 verbs**: `exec`, `list-instances`, `send-status`, `launch`, `close`
  (matches v0.31 tekla.exec feature parity).
- **34 unit / integration tests**, all green; no Revit dependency
  (`EndToEndTests.cs` uses an in-process pipe-server stub identical in shape
  to cli-rhino's `stub-rhinocode.cmd`).
- **20 prompt fixtures** under `cli-revit/Ingest/Output/prompt-*.json` —
  read-side heavy, three write-side prompts using opt-in `"transaction":"auto"`,
  plus file / ops / meta. Same scoring shape as v0.31 + v0.32 drills.
- **`run-drill.ps1`** — harness; iterates fixtures, writes
  `cli-revit/Ingest/Output/drill-summary.md` (gitignored).
- **`revit-2026/manifest.yaml`** — `transport.cli.binary` retargeted from
  `aware-revit-2026` → `aware-revit`; the 5 runtime verbs were prepended to
  the auto-generated catalog commands block.
- **Spec + plan + this handoff** committed under `docs/superpowers/`.

## Live drill — 20/20

```
prompt-01 ... PASS   (37,867 elements counted across 100+ categories)
prompt-02 ... PASS
prompt-03 ... PASS   ("Snowdon Towers Sample Architectural")
prompt-04 ... PASS
prompt-05 ... PASS
prompt-06 ... PASS
prompt-07 ... PASS
prompt-08 ... PASS
prompt-09 ... PASS
prompt-10 ... PASS   (wall placed: id 2661017, Exterior - Brick on Mtl. Stud)
prompt-11 ... PASS
prompt-12 ... PASS
prompt-13 ... PASS
prompt-14 ... PASS
prompt-15 ... PASS
prompt-16 ... PASS
prompt-17 ... PASS
prompt-18 ... PASS
prompt-19 ... PASS
prompt-20 ... PASS
```

Drilled against `C:\Program Files\Autodesk\Revit 2026\Samples\Snowdon Towers Sample Architectural.rvt` on a Windows 11 box with Revit 2026.4 (build 20251103_1515).

## Architecture decision (decisive vs cli-tekla and cli-rhino)

Both prior vendors had an escape hatch:
- **Tekla** — Open API is a remoting facade; constructing `Model()` in a
  separate process attaches to the live Tekla.
- **Rhino** — McNeel ships `rhinocode.exe` (out-of-process CLI with named-pipe
  IPC into the in-Rhino script server).

**Revit has neither.** The Revit API is callable only from Revit's main
thread inside Revit's process. So v0.33 is two binaries cooperating over a
named pipe:

```
aware-revit.exe (sidecar)
     │  length-prefixed JSON
     ▼
named pipe "aware-revit-<PID>"
     │
     ▼
AwareRevit.dll (loaded by Revit at startup as IExternalApplication)
     │  ExternalEvent.Raise()
     ▼
Revit main thread → IExternalEventHandler.Execute()
     │  CSharpScript.RunAsync(globals)
     ▼
RevitAPI / RevitAPIUI (references resolved from already-loaded assemblies)
     │
     ▼
result → JSON-shaped via ResultSerializer → back through the pipe
```

Static-typed `uiapp` (UIApplication, NOT `dynamic`) is the key correctness
detail — the initial design used `dynamic` (mirroring cli-tekla's `model`),
but C# 9+ refuses to bind lambdas to method calls on dynamic types
(CS1977), which breaks every read-side prompt that uses LINQ. The fix is a
two-line edit in `ScriptEngine.cs` (committed as `a201efbd7`).

## Run the drill (5 minutes — one-time setup, then push-button)

```powershell
cd C:\Users\bimst\source\repos\aware

# 1. Build everything Release + publish the sidecar single-file.
dotnet build cli-revit/Shared/Shared.csproj -c Release
dotnet build cli-revit/AwareRevit/AwareRevit.csproj -c Release
dotnet publish cli-revit/Sidecar/cli-revit.csproj -c Release -r win-x64 `
    -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true

# 2. Install the add-in to your per-user Revit Addins folder.
pwsh -NoProfile -ExecutionPolicy Bypass -File .\cli-revit\install-addin.ps1 `
    -Configuration Release -RevitVersion 2026

# 3. (FIRST TIME ONLY — see "Add-in trust prompt" below)
#    Sign the deployed DLL with a self-signed code-signing cert so Revit
#    accepts it without showing the "unsigned add-in" prompt. The cert
#    only needs to be created once; the install script copies the DLL but
#    does NOT sign it.

# 4. Launch Revit via the sidecar (or manually open Revit + a model).
$awareRevit = "cli-revit\Sidecar\bin\Release\net8.0-windows\win-x64\publish\aware-revit.exe"
'{"verb":"launch","version":"2026","model_path":"C:/Program Files/Autodesk/Revit 2026/Samples/Snowdon Towers Sample Architectural.rvt"}' | & $awareRevit --json-stdin

# 5. Poll for addin readiness.
'{"verb":"list-instances"}' | & $awareRevit --json-stdin
# Wait until "addin_loaded": true (typically ~25s after launch).

# 6. Run the drill.
pwsh -NoProfile -ExecutionPolicy Bypass -File .\cli-revit\Ingest\run-drill.ps1

# 7. Inspect.
notepad .\cli-revit\Ingest\Output\drill-summary.md
```

## Add-in trust prompt (the one autonomous-mode wrinkle)

Revit refuses to load any unsigned `.addin`-referenced DLL without showing a
modal TaskDialog: "Cannot verify the publisher of this add-in. What do you
want to do?" with default button "Do Not Load". The dialog is unmissable in
unattended mode — it parks Revit's boot until a human clicks.

Two paths exist:

1. **Sign the DLL with a self-signed cert added to the user's TrustedRoot
   + TrustedPublisher stores** (what was done here). One-time setup; the
   DLL stays trusted across rebuilds as long as it's re-signed with the same
   cert. `certutil -user -addstore Root <cer>` triggers a Win32 "Security
   Warning" Yes/No dialog (Windows policy; can't be silenced from non-admin
   non-interactive context) but only once per cert installation.
2. **Ship a real Authenticode-signed DLL** — DigiCert / SSL.com / a project
   code-signing cert. Same deployment story as path 1 but no Trust prompt
   for end users.

Recommended for v0.34: add an `install-addin.ps1 -SignWith <thumbprint>`
parameter that does the sign-and-install in one shot, and document the
one-time cert generation step. Current `install-addin.ps1` only copies +
rewrites the manifest path; signing was done manually for the live drill.

## Receipt shape divergence from cli-tekla (intentional, additive)

cli-tekla's exec receipt: `{ok, result, host, verb, delivered_at}`.

cli-rhino added `host_version`, `host_pid`, `rhino_id`, `stdout_log`.

v0.33 follows cli-rhino: `host_version`, `host_pid`, `stdout_log` —
strictly additive (no removed fields). Dropped `rhino_id` because Revit's
per-pipe key IS the PID, so it's redundant.

Recommendation for v0.34 (still owed from v0.32): backport these fields to
cli-tekla so all three vendor envelopes converge fully.

## What's NOT shipped (deferred to v0.34)

- Multi-version support (PR is 2026-only; the .addin manifest installs only
  to `Addins\2026\`).
- Catalog self-patching loop (mechanical port from tekla; v0.34).
- Mac (Revit isn't supported on Mac).
- Wiring into `aware app install`'s sidecar-distribution flow (same status as
  cli-tekla / cli-rhino).
- Headless / batch-mode Revit (RevitBatchProcessor pattern). Out-of-scope.
- Per-version-targeted .addin manifests (right now `<Assembly>` points to a
  deployed-as-absolute path under `2026/`; a multi-version build needs one
  manifest per version).
- Production-grade sign-and-deploy story (see "Add-in trust prompt" above).

## Open issues to keep an eye on

1. **Roslyn-in-Revit reference resolution** — `Assembly.Location` returned
   real on-disk paths under `C:\Program Files\Autodesk\Revit 2026\` for
   every reference in the drill. If a future Revit version moves to
   `Assembly.Load(byte[])` for any of RevitAPI / RevitAPIUI / RevitAPIIFC,
   the references would need to be probed by hard path under
   `C:\Program Files\Autodesk\Revit <version>\` instead.
2. **Transaction policy** — `transaction: "auto"` opt-in wraps user code in
   `Transaction(doc, "AWARE exec")`. Three of the drill prompts (10, 11, 12,
   15) use it. The default is no-wrap so user code has full TX control
   (matches tekla pattern).
3. **TaskDialog modality for send-status** — Revit has no status-bar API;
   `send-status` uses `TaskDialog.Show` (modal). Acceptable for v0.33 but
   considered an interrupt. A future enhancement could write to the journal
   or use a Revit ribbon panel notification instead.
4. **`close` clean-path race** — `Application.Exit()` races against the pipe
   write back to the sidecar; the sidecar treats any pipe outcome as
   success-pending and verifies via `Process.HasExited`. If Revit's
   close-document dialog opens (modified docs), the clean close times out
   after 30s and the sidecar tells the caller to use `force=true`. Worked
   correctly during the drill.

## Branch + PR

- Branch: `v0.33-revit-exec` (branched off `main` post v0.31; no v0.32
  dependencies).
- Commits: atomic per plan task — see `git log --oneline origin/main..HEAD`.
  No `Co-Authored-By: Claude` trailers. Spec + plan committed under
  `docs/superpowers/{specs,plans}/2026-05-19-revit-exec*.md`.
- PR opened (post-handoff): see GitHub notifications / session output.
- Merge after manual review confirms the architecture decision (in-Revit
  add-in vs out-of-process Roslyn).
