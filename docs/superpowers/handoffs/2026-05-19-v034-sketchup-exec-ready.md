# v0.34 — aware-sketchup runtime sidecar ready for live drill (2026-05-19)

The bridge from the v0.30 SketchUp catalog (knowledge) to a live SketchUp 2026 host (execution) is **built**, **tested in isolation**, and **ready for the live drill** — pending one user-interactive step that the autonomous run could not perform.

## Components shipped

- **`cli-sketchup/` runtime sidecar** — single-file .NET 10 binary `aware-sketchup.exe`. Five verbs: `exec`, `list-instances`, `send-status`, `launch`, `close`. Plus three sidecar-management flags: `--install-bridge`, `--uninstall-bridge`, `--bridge-status`. Identical receipt vocabulary to `cli-tekla` and `cli-rhino` — only the `host` field changes ("sketchup").
- **`cli-sketchup/BridgeAssets/` Ruby bridge** — a SketchUp extension (loader `.rb` + support folder) that auto-loads from `%APPDATA%\SketchUp\SketchUp 2026\SketchUp\Plugins\`, opens a TCP socket on 127.0.0.1:8765 (first free in 8765..8865), listens for length-prefixed JSON requests, marshals each onto SketchUp's main thread via `UI.start_timer(0.05, true)`, evaluates against `Sketchup.active_model`, returns a JSON receipt. Discovery file at `%TEMP%/aware-sketchup/<pid>.json` advertises the port.
- **`20-agents/aeco/architecture/sketchup-2026/commands/exec.md`** — the catalog command entry for `sketchup.exec`, mirroring the shape of `tekla.exec` and `rhino.exec`.
- **`cli-sketchup/Ingest/Output/prompt-{01..20}.json`** — 20 live-drill fixtures (read / write / ops / file / meta mix) ready to ship through the sidecar.
- **`cli-sketchup/Ingest/run-drill.ps1`** — drill harness mirroring `cli-rhino`'s.
- **`cli-sketchup/Tests/`** — 34 unit/integration tests, all passing on `cargo`-equivalent `dotnet test` (no SketchUp required to run).
- **`docs/superpowers/specs/2026-05-19-v034-sketchup-exec-design.md`** — full design doc + ADRs (length-prefix framing, discovery files, main-thread marshaling, asset-install layout).

## Architecture in one paragraph

SketchUp has no out-of-process CLI bridge — Trimble only exposes scripting through SketchUp's in-process Ruby 3.2 interpreter. So we invented one: an auto-loaded Ruby extension that exposes an in-process TCP listener inside the SketchUp process. The .NET sidecar discovers running SketchUp instances by reading `%TEMP%/aware-sketchup/<pid>.json` files the bridge drops at startup, connects to the right port, ships a `{type,code,args}` JSON request, receives a `{ok,result,stack,stdout_log}` JSON response. The bridge's tick handler (`UI.start_timer`) drains an inbox queue on SketchUp's main thread so user code can safely touch `Sketchup.active_model`. The wire protocol is 4-byte big-endian length prefix + UTF-8 JSON body — unambiguous, fast, no escaping concerns. Identical AWARE receipt shape across `cli-tekla` / `cli-rhino` / `cli-sketchup` so orchestrators see one vocabulary.

## Unit + integration test results

```text
Passed!  - Failed:     0, Passed:    34, Skipped:     0
```

Coverage:
- `ReceiptsTests` — receipt envelope shapes match cli-rhino vocab (ok / fail / list / send-status / generic-fail).
- `SketchupClientTests` — discovery file parser, dead-PID filter, corrupt-file cleanup, version/PID/no-filter resolution, length-prefix big-endian round-trip, end-to-end against a stub TCP listener.
- `BridgeInstallerTests` — install/uninstall copies+removes assets, status reports installed version, arg parser handles `--target-year` and `--plugins-dir`.
- `ExecVerbTests` — no-instance failure mode, missing-code failure mode, exec against stub TCP server returns ok receipt with correct PID/version/result, bridge-reported user error returns `{ok:false}` with exit 0.

## Live drill — STATUS: NOT RUN

The 20-prompt live drill against SketchUp 2026 **could not complete in this autonomous run** because of a UI gate that requires human interaction. Details below.

### What works on Pawel's machine (verified during this run):

1. `aware-sketchup.exe --install-bridge` copies `aware_sketchup_bridge.rb` + `aware_sketchup_bridge/core.rb` into `%APPDATA%\SketchUp\SketchUp 2026\SketchUp\Plugins\`. **Verified.**
2. `aware-sketchup.exe --bridge-status` reads `BRIDGE_VERSION = '0.34.0'.freeze` out of the installed loader and confirms parity with the packaged version. **Verified.**
3. `aware-sketchup.exe list-instances` returns `{instances:[]}` cleanly when no SketchUp is running. **Verified.**
4. `aware-sketchup.exe launch --target-year 2026` finds `C:\Program Files\SketchUp\SketchUp 2026\SketchUp\SketchUp.exe` (note nested path; the launcher tries the nested form first then the flat form) and spawns it. **Verified.**

### The wall:

SketchUp 2026 on first cold boot for this user account opens a Chromium-embedded (CEF) **"Welcome to SketchUp"** modal dialog that blocks **all Ruby extension loading** until dismissed. The dialog is a `CommonWebDialog` class — fully web-rendered, so:

- Keyboard input (TAB, ENTER, ESC via SendKeys) does not interact with the inner controls.
- Programmatic mouse clicks via Win32 `mouse_event` do not register inside the CEF web view (CEF gates against synthetic input from external processes by default).
- UI Automation enumeration finds only `CefBrowserWindow` / `Chrome_RenderWidgetHostHWND` panes — no buttons.
- Even passing a real `.skp` file on the command line (`SketchUp.exe "Temp01a - Simple.skp"`) does not bypass the Welcome dialog on a first-launch account.

Until the dialog is dismissed manually, the bridge's loader `.rb` never executes. The diagnostic logging (`boot-<pid>.log`) confirms zero plugin-loader execution under this condition.

### Action Pawel needs to take (one time, ~10 seconds):

1. Open SketchUp 2026 manually (Start menu → SketchUp 2026, OR `aware-sketchup launch`).
2. Click through the Welcome screen — sign in OR pick "Use without signing in" if available, then click "Start Modeling" / "Create New" with any template.
3. Once a model is open, **the bridge auto-loads.** Verify with `aware-sketchup list-instances` — a non-empty array means it worked.

SketchUp remembers the click-through state, so subsequent launches go straight to the modeling environment and the bridge loads automatically.

### To run the drill:

```powershell
# After dismissing Welcome and confirming list-instances returns a populated array:
cd C:\Users\bimst\source\repos\aware\.claude\worktrees\agent-adcdcaaf9ca4f1881\cli-sketchup
.\Ingest\run-drill.ps1
# → writes Ingest/Output/drill-summary.md
```

## Reproducing the test run (no SketchUp required)

```powershell
cd C:\Users\bimst\source\repos\aware\.claude\worktrees\agent-adcdcaaf9ca4f1881\cli-sketchup\Tests
dotnet test --nologo
# Passed!  - Failed: 0, Passed: 34, Skipped: 0
```

## Files changed

```
cli-sketchup/
├── .gitignore
├── BridgeAssets/
│   ├── aware_sketchup_bridge.rb       (loader, ~45 lines)
│   └── aware_sketchup_bridge/
│       └── core.rb                    (~280 lines — the bridge)
├── BridgeInstaller.cs                 (~190 lines)
├── Ingest/
│   ├── run-drill.ps1                  (~95 lines)
│   └── Output/
│       └── prompt-{01..20}.json       (20 fixtures)
├── Program.cs                         (~140 lines)
├── Receipts.cs                        (~95 lines)
├── SketchupClient.cs                  (~190 lines)
├── Tests/
│   ├── AssemblyInfo.cs
│   ├── BridgeInstallerTests.cs        (10 tests)
│   ├── ExecVerbTests.cs               (4 tests)
│   ├── ReceiptsTests.cs               (7 tests)
│   ├── SketchupClientTests.cs         (13 tests)
│   └── cli-sketchup.Tests.csproj
├── Verbs/
│   ├── Close.cs
│   ├── Exec.cs
│   ├── Launch.cs
│   ├── ListInstances.cs
│   └── SendStatus.cs
└── cli-sketchup.csproj

20-agents/aeco/architecture/sketchup-2026/
└── commands/exec.md                   (new — catalog entry for sketchup.exec)

10-core/cli-roadmap.md                 (new v0.34 phase entry)

docs/superpowers/
├── specs/2026-05-19-v034-sketchup-exec-design.md   (full design + ADRs)
└── handoffs/2026-05-19-v034-sketchup-exec-ready.md (this file)
```

## What's NOT shipped yet

- **The live drill PASS/FAIL count.** Run `Ingest/run-drill.ps1` after dismissing the Welcome screen once.
- **Curated SketchUp craft skills** (`drawing-issue-pack`, `geometry-quality-audit`, `material-cleanup`, etc.) — tracked for v0.35+.
- **Mac/Linux build of `aware-sketchup.exe`** — SketchUp's Mac Ruby API is identical but the install path differs; tracked as v0.34.1.
- **A SketchUp "agent" entry that wires `sketchup.exec` into the registry index** — the `20-agents/aeco/architecture/sketchup-2026/` agent already exists; only the `commands/exec.md` was added in this PR. The `manifest.yaml` `commands:` table will need a new `exec: { lifecycle: single, description: "..." }` entry on follow-up.

## Memory entry

Append to `C:\Users\bimst\.claude\projects\C--Users-bimst-source-repos-aware\memory\`:

> `project_v034_sketchup_exec_ready.md` — aware-sketchup runtime sidecar shipped (Ruby in-process bridge + .NET sidecar); 34/34 unit+integration tests PASS; live drill blocked on SketchUp 2026 first-launch Welcome dialog that requires one-time human click-through. Main tip: feat/v0.34-sketchup-exec.

## Branch + PR

- Branch: `feat/v0.34-sketchup-exec` off `origin/main`.
- PR title: `feat(v0.34): sketchup.exec runtime sidecar — Ruby extension + .NET sidecar`.
