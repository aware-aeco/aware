# v0.34 — aware-sketchup runtime sidecar — design

Status: approved (autonomous design partner, brainstorming skill invoked).
Date:   2026-05-19.
Owner:  Pawel Lisowski (aware-aeco/aware).

## Goal

Ship `aware-sketchup.exe` — a 5-verb runtime sidecar (`exec`, `list-instances`, `send-status`, `launch`, `close`) that brings SketchUp 2026 to feature parity with the v0.31 Tekla sidecar (`cli-tekla`) and the v0.32 Rhino sidecar (`cli-rhino`).

Target: at least 13/20 PASS on a live 20-prompt drill against SketchUp 2026, matching the bar Tekla v0.31 set.

## Why this is non-trivial vs cli-rhino

McNeel ships a CLI bridge (`rhinocode.exe`) that talks to a running Rhino process via a named-pipe `ScriptServer`. The Rhino sidecar shells out to that CLI.

**Trimble ships no such bridge for SketchUp.** SketchUp's only public extensibility surface is the in-process Ruby interpreter exposed as `Sketchup::*` and `UI::*`. Code only runs inside SketchUp.

Therefore the SketchUp sidecar must invent the bridge itself, in two pieces:

1. An **in-SketchUp Ruby extension** (`aware_sketchup_bridge.rb`) that opens a TCP socket inside the SketchUp process, listens for `{verb, code, args}` JSON messages, evaluates them against the live document, returns JSON receipts.

2. A **.NET 10 `aware-sketchup.exe` sidecar** that discovers running SketchUp PIDs, connects to the extension's socket, ships JSON, parses the receipts. Same `{verb, code, args}` stdin-JSON CLI contract as Tekla / Rhino.

## Architecture decision record

### ADR-1: Length-prefixed JSON framing over TCP

**Decision:** Wire protocol is 4-byte big-endian unsigned int (message length) + UTF-8 JSON body.

**Alternatives considered:**
- Newline-framed JSON: simpler but breaks if user code body contains `\n` characters that survive JSON-string escaping into log lines.
- HTTP/REST: heavier (need WEBrick); no value over raw socket for our point-to-point case.

**Why length-prefix:**
- Ruby: `len = sock.read(4).unpack1('N'); body = sock.read(len)`
- .NET: `var lenBytes = BinaryReader.ReadBytes(4); Array.Reverse(lenBytes); var len = BitConverter.ToInt32(lenBytes, 0); var body = BinaryReader.ReadBytes(len)`
- Unambiguous, fast, no escaping concerns.

### ADR-2: Auto-loaded extension at `Plugins/aware_sketchup_bridge.rb`

**Decision:** The bridge is installed as a single `aware_sketchup_bridge.rb` file directly in `%APPDATA%\SketchUp\SketchUp <year>\SketchUp\Plugins\`. SketchUp auto-loads every `.rb` in that folder at startup. The file registers a `SketchupExtension` and starts the TCP listener at load time.

**Why not `.rbz`:**
- `.rbz` is a renamed zip; SketchUp expects user to install it via "Extension Manager → Install from disk." That's an interactive step — breaks the AWARE "binaries don't decay" principle.
- A flat `.rb` in `Plugins/` is the canonical zero-touch install path used by Trimble's own `su_assistant` and `su_diffusion` extensions (verified on Pawel's machine).

**Install path:** the sidecar's `--install-bridge` flag copies `aware_sketchup_bridge.rb` (and its support file `aware_sketchup_bridge_core.rb`) into the right Plugins folder. The user runs it once; SketchUp picks up the bridge on next launch.

### ADR-3: Multi-instance discovery via per-PID JSON files

**Decision:** Each SketchUp PID writes a discovery file at `%TEMP%\aware-sketchup\<pid>.json` containing `{pid, port, version, started_at, model_path}`. The sidecar reads the directory, validates each file's PID is still alive (`Process.GetProcessById`), returns the list.

**Why not netstat / pipe enumeration:**
- Netstat parsing is fragile across Windows versions.
- Native pipe enumeration requires P/Invoke into `NtQueryDirectoryObject`.
- A small JSON file in `%TEMP%` is portable, self-describing, and trivially debuggable.

**Cleanup:**
- The bridge registers an `at_exit { File.delete(...) }` hook so clean shutdowns remove the file.
- The sidecar treats discovery files whose PID is dead as stale and ignores them (and best-effort deletes them).

### ADR-4: Main-thread marshaling via `UI.start_timer` + Queue

**Decision:** The bridge spawns a background `Thread.new` running a `TCPServer#accept` loop. Each accepted request is parsed, enqueued onto a thread-safe `Queue`, and a per-connection response Queue is associated. A `UI.start_timer(0.05, true) { drain_queue }` runs on the main thread, drains the request queue, evaluates each request against `Sketchup.active_model`, writes the response into the per-connection response queue. The accept thread blocks on that response queue, writes it back to the socket.

**Why this is the right pattern:**
- SketchUp's Ruby is single-threaded *with respect to the document*. `Thread.new` works fine for I/O, but ANY call into `Sketchup.active_model.entities.*` from a background thread crashes the host (verified via SketchUp dev forum + Ruby API docs).
- `UI.start_timer` is Trimble's canonical pattern for "run a snippet on the main thread soon."
- 50 ms timer tick gives ~50 ms median end-to-end latency. Imperceptible for AECO workflows; for tighter latency we can drop to 10 ms.

### ADR-5: User code is a Ruby block evaluated with `args` + `model` in scope

**Decision:** The user's `code` (Ruby) is wrapped:

```ruby
result = (lambda { |args|
  model = Sketchup.active_model
  # ===== user code body =====
  #{user_code}
  # ==========================
}).call(args)
```

Ruby's "last expression is the return value" means the user can write `model.entities.count` as the last line and that becomes `result`. To make this work with implicit returns we wrap in a lambda, so `return` early-exits the lambda not the entire bridge.

**JSON serialization:**
- The wrapper recursively converts SketchUp types to JSON-safe primitives before encoding:
  - `Geom::Point3d` / `Geom::Vector3d` → `[x, y, z]` (Floats)
  - `Length` → Float in inches (SketchUp's internal unit) + emits a `_unit: "in"` hint when the encoder knows
  - `Sketchup::Entity` (and subclasses) → `entity.persistent_id.to_s` (stable across saves)
  - `Sketchup::Material` / `Sketchup::Layer` → `name`
  - `Geom::Transformation` → `Array#to_a` (16-element matrix)
  - `Set` / `Array` / `Hash` → recursive
  - Everything else → fallback to `.to_s`
- Errors in user code → caught, returned as `{ok: false, error: msg, stack: backtrace}`.

### ADR-6: Sentinel-marker pattern stays — but moves into the JSON receipt itself

Tekla/Rhino use `__AWARE_RESULT_BEGIN__/_END__` because they parse stdout from a child process. Our SketchUp bridge speaks JSON directly over a socket, so there's no need for textual sentinels — the JSON is already framed by the length prefix. Receipt shape is identical to Tekla/Rhino: `{ok, result, host, host_pid, host_version, verb, stdout_log, delivered_at}`.

## CLI surface

```
aware-sketchup --help
aware-sketchup --version
aware-sketchup <verb> [--json-stdin]
aware-sketchup --json-stdin                  # verb embedded in JSON body
```

Verbs:

| Verb           | Stdin JSON                                                   | Receipt                                      |
|----------------|--------------------------------------------------------------|----------------------------------------------|
| `exec`         | `{verb:"exec", code:"...", args:{}, sketchup_id?, version?}` | `{ok, result, host:"sketchup", host_pid, host_version, verb, stdout_log, delivered_at}` |
| `list-instances` | (none)                                                     | `{status:"ok", instances:[{pid, version, port, model_path}], host:"sketchup", verb, delivered_at}` |
| `send-status`  | `{verb:"send-status", message:"...", sketchup_id?, version?}` | `{status:"ok", host_session_id:"sketchup-<pid>", verb, verb_result:{message}, ...}` |
| `launch`       | `{verb:"launch", model_path?, sketchup_exe?}`                | `{status:"ok", host_pid:<spawned-pid>, verb_result:{model_path, sketchup_exe, note}, ...}` |
| `close`        | `{verb:"close", pid?, force?, version?}`                     | `{status:"ok", verb_result:[{pid, version, mode}, ...], ...}` |

The shape matches cli-tekla / cli-rhino so a `{verb:"exec"}` JSON object works across all three hosts with only the `host` field changing.

Plus management verbs (not exposed as AWARE verbs — sidecar-level flags):

```
aware-sketchup --install-bridge [--target-year 2026]   # copy aware_sketchup_bridge.rb into Plugins/
aware-sketchup --uninstall-bridge [--target-year 2026]
aware-sketchup --bridge-status [--target-year 2026]    # is the file present? is its hash current?
```

## Components

```
cli-sketchup/
├── Program.cs                          # entry point, verb dispatch
├── Receipts.cs                         # JSON envelope shapes (mirrors cli-rhino)
├── SketchupClient.cs                   # discovery + socket + length-prefix protocol
├── BridgeInstaller.cs                  # copy aware_sketchup_bridge.rb into Plugins/
├── BridgeAssets/
│   ├── aware_sketchup_bridge.rb        # auto-loaded loader (registers extension)
│   └── aware_sketchup_bridge/
│       └── core.rb                     # the actual bridge (TCP, timer, eval, encode)
├── Verbs/
│   ├── Exec.cs
│   ├── ListInstances.cs
│   ├── SendStatus.cs
│   ├── Launch.cs
│   └── Close.cs
├── Tests/
│   ├── ReceiptsTests.cs
│   ├── SketchupClientTests.cs          # against a stub socket server
│   ├── BridgeInstallerTests.cs
│   └── stub-sketchup/                  # a tiny Ruby script that mimics the bridge
├── Ingest/
│   ├── run-drill.ps1                   # 20-prompt harness
│   └── Output/
│       ├── prompt-01.json … prompt-20.json
│       └── drill-summary.md            # generated
└── cli-sketchup.csproj
```

## Verb details

### `exec`

1. Sidecar discovers SketchUp instance (by `version` filter, `sketchup_id` (= pid), or first available).
2. Connects to the instance's port.
3. Sends a length-prefixed JSON message: `{type:"exec", code, args}`.
4. Bridge background thread enqueues onto request queue.
5. Main-thread timer drains; wraps + evals user code; encodes result.
6. Bridge sends length-prefixed JSON response: `{ok, result | error, stack, stdout_log}`.
7. Sidecar reads response, wraps in AWARE receipt shape, prints to stdout, exits 0.

**Error modes:**
- No instance matches filter → exit 1, `{status:"err", error:"no sketchup instance matches..."}`.
- Connection refused (bridge not loaded) → exit 1, `{status:"err", error:"bridge not running on port X; reinstall bridge with --install-bridge"}`.
- User code throws → exit 0 (the call itself succeeded), receipt has `{ok:false, error, stack}`.
- Timeout (default 30 s) → exit 2, `{status:"err", error:"timeout after 30s"}`.

### `list-instances`

Reads `%TEMP%\aware-sketchup\*.json`, filters dead PIDs, returns array shaped:

```json
[
  {"pid": 12345, "version": "26.0", "port": 8765, "model_path": "C:\\...\\my.skp"}
]
```

### `send-status`

Synthesizes `code: 'Sketchup.set_status_text(args["message"], SB_PROMPT)'` and reuses Exec pipeline. Same shape as cli-rhino's send-status.

### `launch`

1. Discovers SketchUp.exe (default: `C:\Program Files\SketchUp\SketchUp <year>\SketchUp\SketchUp.exe`; override via `sketchup_exe`).
2. `Process.Start(exe, model_path?)` — SketchUp auto-loads the bridge (already in Plugins/).
3. Returns immediately with `host_pid`; client polls `list-instances` until the new PID appears (= bridge is ready).
4. The launch receipt includes a `note` instructing the caller to poll.

### `close`

1. Discovers target(s) by `pid` / `version` / `--all`.
2. If `force=true`: `process.Kill()`.
3. Otherwise: ships a `code: 'Sketchup.active_model.save; "ok"'` via exec, then `process.CloseMainWindow()`, waits 5s, escalates to Kill if not exited.

Same `acknowledge_data_loss` semantics as Tekla — but in this sidecar the flag is named `force` to match cli-rhino's vocabulary.

## Bridge Ruby internals

`aware_sketchup_bridge.rb` (loader, ~30 lines):

```ruby
require 'sketchup.rb'
require 'extensions.rb'

module AwareSketchupBridge
  EXTENSION_NAME = 'AWARE SketchUp Bridge'.freeze
  unless file_loaded?(__FILE__)
    loader = File.join(File.dirname(__FILE__), 'aware_sketchup_bridge', 'core')
    ex = SketchupExtension.new(EXTENSION_NAME, loader)
    ex.description = 'TCP bridge for the AWARE runtime sidecar.'
    ex.version     = '0.34.0'
    ex.copyright   = 'AWARE 2026'
    ex.creator     = 'AWARE'
    Sketchup.register_extension(ex, true)
    file_loaded(__FILE__)
  end
end
```

`aware_sketchup_bridge/core.rb` (~250 lines): TCP listener, length-prefix framing, request/response queues, `UI.start_timer` main-thread pump, `eval` wrapper, JSON encoder for SketchUp types, discovery file lifecycle, structured error handling.

## Tests

| Layer | What | Where |
|---|---|---|
| Unit | Receipt shapes match cli-rhino vocab | `Tests/ReceiptsTests.cs` |
| Unit | Length-prefix framing round-trips | `Tests/SketchupClientTests.cs` |
| Unit | Discovery file parser ignores stale PIDs | `Tests/SketchupClientTests.cs` |
| Unit | Bridge installer copies assets to right path | `Tests/BridgeInstallerTests.cs` |
| Integration | Stub socket server (Ruby script) returns canned response; sidecar parses + emits receipt | `Tests/SketchupClientTests.cs` |
| Live | 20 prompt fixtures executed against real SketchUp 2026 | `Ingest/run-drill.ps1` |

All unit + integration tests pass without SketchUp installed.

## Drill fixture coverage (20 prompts)

Mirrors the cli-rhino mix; Ruby in the `code` field:

| # | Category | Verb |
|---|---|---|
| 01 | Read | Count entities by class |
| 02 | Read | List selected entities (persistent IDs) |
| 03 | Read | Document bounding box (active geometry) |
| 04 | Read | List materials with names + RGB |
| 05 | Read | List layers (tags) with names + visibility |
| 06 | Write | Add a line `[0,0,0]→[1m,0,0]` |
| 07 | Write | Add a circle (XY plane, r=500mm) |
| 08 | Write | Add a triangular face |
| 09 | Write | Add a group with 5 random points |
| 10 | Write | Add a new layer named `AWARE` with random color |
| 11 | Ops | Move selected by `[0.1m, 0.1m, 0]` |
| 12 | Ops | Rotate selected 45° around Z |
| 13 | Ops | Scale selected ×2 from origin |
| 14 | Ops | Explode selected groups/components |
| 15 | Ops | Erase selected (best-effort, returns count) |
| 16 | File | Save model to `%TEMP%\aware-drill.skp` |
| 17 | File | Export selected to DAE at `%TEMP%\aware-drill.dae` |
| 18 | File | List scene pages (Sketchup::Page) |
| 19 | Meta | Model units + render mode + active view name |
| 20 | Meta | Model description + GUID + filename + path |

## Risks + mitigations

| Risk | Mitigation |
|---|---|
| SketchUp blocks the `TCPServer` socket via Windows Firewall on first run | Bind to `127.0.0.1` only; loopback is exempt from Defender firewall prompts on Win10/11 |
| User has bridge from older AWARE version installed | Installer overwrites unconditionally with `--install-bridge`; version check available via `--bridge-status` |
| Port 8765 taken by another tool | Bridge tries 8765..8865 in order; logs chosen port to discovery file |
| Multi-instance race (two PIDs both try to bind 8765) | Each bridge picks the next free port; discovery file is keyed by PID, so distinct |
| User's Ruby code body is too long for the receive buffer | Bridge reads `length` bytes in a loop until satisfied; no per-line buffer |
| User's Ruby code does `exit` / `abort` | Caught: the lambda wrapper re-raises as `SystemExit`, the eval rescues `Exception`, returns `{ok:false, error:"user code called exit"}` |
| User's Ruby code crashes SketchUp (e.g. infinite loop holding the main thread) | Bridge timer ticks at 50ms; if the eval takes >timeout, the sidecar's socket-side read timeout fires (default 30s configurable) — but SketchUp's main thread is still stuck until the user code returns. Documented as a "don't write infinite loops in `exec`" warning; same constraint applies to Tekla and Rhino. |

## What's NOT included

- Curated craft skills (`drawing-issue-pack`, etc.) — out of scope for v0.34; tracked for v0.35+.
- The substrate-side `aware build agent` integration to scaffold SketchUp curated commands — separate phase.
- A SketchUp Pro / Studio licensing check — the bridge runs under whatever license the user already has.
- A Mac/Linux build of `aware-sketchup.exe` — SketchUp on Mac uses a similar Ruby API but the install path differs; tracked as v0.34.1.
- Anonymized telemetry — none, ever (decalog #4).

## Branch + PR

- Branch: `feat/v0.34-sketchup-exec` off `origin/main`.
- PR title: `feat(v0.34): sketchup.exec runtime sidecar — Ruby extension + .NET sidecar`.
- PR body mirrors cli-rhino's #66.

## Roadmap entry

This phase corresponds to a new v0.34 entry in `cli-roadmap.md`'s post-v0.5 sequence, slotted into the "fan out the `*.exec` pattern" track established by v0.31 (Tekla) and v0.32 (Rhino). Roadmap entry added as part of this PR.
