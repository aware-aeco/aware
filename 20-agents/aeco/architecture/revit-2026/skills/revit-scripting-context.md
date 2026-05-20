---
name: revit-scripting-context
description: This skill should be used whenever composing app nodes that invoke Revit 2026 from outside its UI — via the v0.33 `aware-revit` sidecar, pyRevit, RevitPythonShell, Dynamo, or any other channel into the Revit API. Encodes Revit's main-thread-only API constraint, the `IExternalApplication` / `IExternalCommand` / `IExternalEventHandler` lifecycle, why direct sidecar-to-RevitAPI access is impossible (the add-in + pipe pattern v0.33 uses), the .addin manifest format, and the per-version Revit runtime targets. Critical for any app that ships C# snippets through `revit.exec`.
---

# Revit scripting context

The Revit API has a non-negotiable constraint: **it can only be called from Revit's main thread, from inside the Revit process**. There is no out-of-process scripting bridge like Rhino's `rhinocode`. The v0.33 `aware-revit` sidecar gets around this by SHIPPING an in-Revit add-in that hosts a Roslyn engine + named-pipe server.

## The architecture

```
AI orchestrator
   ↓ JSON over stdin: { verb, code, args, version?, host_pid? }
aware-revit.exe  (the .NET sidecar)
   ├─ Discover: enumerate Revit.exe processes, pick by version/pid
   ├─ NamedPipeClientStream connect to "aware-revit-<PID>"
   ├─ Send: length-prefixed JSON request
   └─ Receive: length-prefixed JSON receipt
       ↓ (named pipe, local IPC)
AwareRevit.dll  (in-Revit add-in, loaded at Revit startup)
   ├─ IExternalApplication.OnStartup: spawn NamedPipeServerStream listener thread
   ├─ On request: enqueue + ExternalEvent.Raise()
   └─ ExecuteEventHandler.Execute(uiApp):  ← runs on Revit's main thread
       ├─ Roslyn compile the user's C# against RevitAPI.dll + RevitAPIUI.dll
       ├─ Globals: app=UIApplication, doc=ActiveUIDocument.Document, args=dict
       └─ Serialize result via ResultSerializer (Element → {id, category, name})
       ↓ (in-process)
RevitAPI:  mutates the Document; commits via Transaction
```

## The .addin manifest

Revit loads add-ins via `.addin` XML files in `%PROGRAMDATA%\Autodesk\Revit\Addins\<year>\` (machine-wide) OR `%APPDATA%\Autodesk\Revit\Addins\<year>\` (per-user). AWARE's installer drops a per-user copy:

```xml
<?xml version="1.0" encoding="utf-8"?>
<RevitAddIns>
  <AddIn Type="Application">
    <Name>AwareRevit</Name>
    <Assembly>AwareRevit.dll</Assembly>
    <FullClassName>AwareRevit.AddInApplication</FullClassName>
    <ClientId>...GUID...</ClientId>
    <VendorId>com.aware-aeco</VendorId>
    <VendorDescription>AWARE — agentic substrate for AECO</VendorDescription>
  </AddIn>
</RevitAddIns>
```

Per-version subfolder: `Addins\2026\AwareRevit.addin`. The sidecar's `install-addin.ps1` deploys the .addin + .dll into the appropriate subfolder.

## The three external-API patterns

| Pattern | When fires | Use when |
|---|---|---|
| `IExternalCommand` | User clicks a ribbon button | Interactive commands (we don't ship one) |
| `IExternalApplication` | Revit startup / shutdown | Initialize our add-in (start the pipe server here) |
| `IExternalEventHandler` | App-defined trigger (`ExternalEvent.Raise()`) | **The only safe cross-thread mechanism.** Pipe-server thread queues requests, raises the event, main thread drains the queue. |

The v0.33 add-in uses all three: `IExternalApplication` for startup, `IExternalEventHandler` for execution, no `IExternalCommand` (we have no ribbon UI).

## The cross-thread invariant

**Anything that touches `Document` or `UIApplication` MUST run on the main thread.** Calling `doc.GetElement(id)` from the pipe-server thread is a recipe for hard crashes (sometimes — the failures are race-dependent, which makes them worse).

The `IExternalEventHandler.Execute(uiApp)` method is guaranteed to run on the main thread. Use it as the single entry point for ALL user-snippet execution.

```
Request arrives on pipe thread
   ↓ enqueue
   ↓ ExternalEvent.Raise()
Revit's UI dispatcher schedules Execute()
   ↓ next idle moment, ON MAIN THREAD:
Execute(uiApp) drains the queue
   ↓ for each request:
   ├─ Compile user C# via Roslyn
   ├─ Run with (app=uiApp, doc=uiApp.ActiveUIDocument.Document, args=...)
   └─ Push response back to pipe thread for sending
```

## Per-version runtime

| Revit version | .NET runtime | RevitAPI.dll target |
|---|---|---|
| 2024 | .NET Framework 4.8 | net48 |
| 2025 | .NET 8 (modern) | net8.0-windows |
| 2026 | .NET 8 (modern) | net8.0-windows |

The v0.33 add-in and sidecar are both **.NET 8 (net8.0-windows)** because we target Revit 2026. The Shared/ DTOs target `net8.0` (no -windows; pure types) so they're consumable from any host.

Cross-version support (running the same add-in against 2024 + 2025 + 2026) is NOT in this PR — would require dual-targeting + version-specific .addin files.

## Discovery

The sidecar enumerates running Revit instances via `Process.GetProcessesByName("Revit")` and matches by:
1. `pid` if supplied (exact)
2. `version` if supplied (extracted from the exe path's `\Revit 2026\` segment)
3. Otherwise: pick the first match

The pipe name is `aware-revit-<PID>` so multi-instance routing is by PID, similar to how the SketchUp bridge writes per-PID discovery files.

## Failure modes worth knowing

| Symptom | Likely cause | Fix |
|---|---|---|
| `{ok:false, error:"no Revit instance matches version='2026'"}` | No Revit 2026 running | Launch Revit 2026, open a model |
| `{ok:false, error:"add-in not responding (pipe timeout)"}` | Add-in not installed OR Revit didn't load it (check Revit's add-in dialog) | Run `install-addin.ps1`, restart Revit |
| `{ok:false, error:"Compile error: CS0246: 'Element' could not be found"}` | User code missing `using Autodesk.Revit.DB;` | Add the using; or AI re-drafts |
| `{ok:false, error:"InvalidOperationException: ... must be called from outside the document"}` | User code's transaction nesting is wrong | Wrap writes in `using (var t = new Transaction(doc, "..."))` |
| `{ok:false, error:"Active document is null"}` | Revit is in family editor OR no doc open | User opens a project/family doc |

## Composition guidance

- **Always check `list-instances` first** in multi-Revit environments
- **Pin `host_pid`** when precision matters; pin `version` when "any Revit 2026 will do" is good enough
- **Wrap writes in `Transaction`** — without one, the API throws. The v0.11 safety contract's `transaction-group:` field hints at this requirement.
- **Treat `exec` as the only mutation surface** — curated workflow verbs (`sheet.list`, `revision.bump`, etc.) route through it via the documented C# snippet

## See also

- [v0.33 add-in spec](../../../../docs/superpowers/specs/2026-05-19-revit-exec-design.md)
- [revit-transactions](./revit-transactions.md)
- [revit-element-collector](./revit-element-collector.md)
