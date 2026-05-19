---
name: sketchup-scripting-context
description: This skill should be used whenever composing app nodes that invoke SketchUp 2026 from outside its UI — via the v0.34 `aware-sketchup` sidecar or any other channel into the SketchUp Ruby interpreter. Encodes the in-process bridge architecture (TCP socket inside SketchUp opened by an auto-loaded Ruby extension, length-prefixed JSON framing, port discovery via `%TEMP%/aware-sketchup/<pid>.json`), the `UI.start_timer` main-thread marshaling pattern, the Welcome-dialog cold-boot blocker, the extension-loading gate, and the failure modes (bridge not loaded, port collision, dead PID files). Critical for any app that ships Ruby snippets through `sketchup.exec`.
---

# SketchUp scripting context

SketchUp's Ruby API is **in-process only**. Unlike Rhino (which ships `rhinocode` as an out-of-process CLI), Trimble exposes no external scripting bridge. AWARE's v0.34 sidecar gets around this by SHIPPING the bridge — an auto-loaded SketchUp extension that hosts an in-process TCP listener — and connecting to it from outside.

## The architecture

```
AI orchestrator
   ↓ JSON over stdin: { verb, code, args, version?, pid? }
aware-sketchup.exe  (the .NET 10 sidecar)
   ├─ Discover: scan %TEMP%/aware-sketchup/<pid>.json for live SketchUps
   ├─ TCP connect to 127.0.0.1:<port from discovery file>
   ├─ Send: 4-byte big-endian length + UTF-8 JSON body
   └─ Receive: same framing, parse, emit AWARE receipt
       ↓ (TCP, localhost only)
aware_sketchup_bridge.rb  (Ruby extension auto-loaded by SketchUp)
   ├─ TCPServer on first-free port in 8765..8865
   ├─ Background thread accepts connections, parses framed requests, queues them
   ├─ UI.start_timer(0.05, true) ticks every 50ms on main thread → drains queue
   └─ Evaluates user code with `Sketchup.active_model` in scope, returns JSON
       ↓ (in-process)
SketchUp.exe (user's running SketchUp)
   └─ Ruby interpreter touches the model, mutates document
```

## The cold-boot blocker

SketchUp 2026 on **first launch for a user account** opens a CEF-rendered "Welcome to SketchUp" modal dialog. This dialog blocks ALL Ruby extension loading until dismissed by a human click. The bridge's `.rb` never executes during this gate.

Symptoms:
- `aware-sketchup list-instances` returns `{instances: []}` even though `SketchUp.exe` is running
- `%TEMP%/aware-sketchup/` is empty (no discovery file written)
- The bridge's `boot-<pid>.log` is absent

Recovery: human opens SketchUp, clicks through the Welcome screen (sign in / use without signing in, pick a template, "Start Modeling"). SketchUp remembers the click — subsequent launches load the bridge automatically.

**There is no programmatic bypass** — synthetic keyboard/mouse events don't interact with CEF web views.

## Multi-instance routing

When the user has multiple SketchUp windows open, the bridge in each one writes its own discovery file:

```
%TEMP%/aware-sketchup/
├── 12345.json   { "port": 8765, "version": "2026", "active_doc": "Untitled.skp" }
├── 23456.json   { "port": 8766, "version": "2026", "active_doc": "Project.skp" }
```

`aware-sketchup list-instances` returns one row per live PID. To target a specific instance, pass `pid` in the input JSON:

```json
{ "verb": "exec", "pid": 12345, "code": "Sketchup.active_model.title" }
```

Dead PIDs (process exited without cleanup) are filtered out by the sidecar at list-time.

## The wrap-and-sentinel pattern

The user's `code` is wrapped in a synthetic context that exposes `model` (= `Sketchup.active_model`) and `args` (= the input args block). Result-capture uses a sentinel-tagged JSON response — same shape as cli-tekla / cli-rhino. Errors surface as `{ok:false, error:"...", stack:"..."}`.

## Main-thread marshaling

SketchUp's API is NOT thread-safe. Calling `Sketchup.active_model` from the TCP-accept thread is a recipe for SIGSEGV. The bridge solves this with `UI.start_timer(0.05, true)` — a SketchUp-provided main-thread tick that runs every 50ms and drains an inbox queue. Each user code block executes during a tick, on the main thread, with `Sketchup.active_model` accessible.

Practical implication: **a single exec call takes ~50-100ms minimum** (one tick to accept + one tick to respond). For sub-millisecond latency, batch operations into one larger script.

## Failure modes worth knowing

| Symptom | Likely cause | Fix |
|---|---|---|
| `{ok:false, error:"no live SketchUp instance"}` | No SketchUp running OR Welcome dialog not yet dismissed | User dismisses dialog OR launches a SketchUp |
| `{ok:false, error:"discovery file references dead PID"}` | SketchUp crashed without cleanup | Restart SketchUp; old PID files auto-purge on next bridge boot |
| `{ok:false, error:"all ports 8765..8865 in use"}` | Extreme: 101 SketchUps OR other apps holding those ports | Close some SketchUps; check `netstat` for the conflicting app |
| `{ok:false, error:"bridge timeout (no main-thread tick)"}` | UI thread blocked (e.g. user is doing something modal) | Wait for user to finish; retry |

## Composition guidance

When writing apps that depend on SketchUp:

- **Document the install step** — `aware-sketchup --install-bridge` is a one-time per-user action that copies the Ruby files into the SketchUp plugins folder.
- **Document the Welcome-dialog gate** for SketchUp 2026 first-launch flows. Subsequent launches don't need it.
- **Always check `list-instances` first** for multi-Sketch-Up users.
- **Treat `exec` as the only mutation surface** — curated workflow verbs (`entity.list-by-tag`, `material.bulk-apply`, etc.) route through it via the documented Ruby snippet.
- **Batch operations** to avoid the ~50ms tick latency multiplying.
