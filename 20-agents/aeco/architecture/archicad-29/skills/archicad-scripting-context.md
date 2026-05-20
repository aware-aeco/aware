---
name: archicad-scripting-context
description: This skill should be used when composing any Archicad automation — every Archicad workflow primitive is a POST against a JSON-RPC endpoint, not an in-process .NET / Python call. Encodes the dual command surface (Graphisoft's built-in JSON Interface + the Tapir community Add-On), the HTTP transport (localhost:19723 by default), the request envelope shape (`{"command": "API.<Name>", "parameters": {...}}`), the `ExecuteAddOnCommand` wrapper used to reach Tapir-namespaced verbs, and the prerequisites (Archicad running + Tapir Add-On loaded). Read first before any other archicad skill.
---

# Archicad scripting context — Tapir + JSON-RPC

Archicad's automation surface is **JSON-RPC over HTTP**, not an in-process library binding. There is no `using Archicad;` to add, no `import archicad_api` to pip-install. The runtime model is:

1. **The user runs Archicad on their machine** with a project loaded.
2. **The Tapir Add-On** (bundled or sideloaded into Archicad's Add-Ons folder) **opens an HTTP listener** on `127.0.0.1:19723`.
3. **External tooling — Python, curl, PowerShell, Rust, whatever — POSTs JSON to that endpoint.**
4. **The Add-On marshals the call onto Archicad's main thread,** runs the corresponding API verb against the live document, and returns a JSON result.

This is closer to a REST API than to Rhino's CLI or Tekla's .NET Open API. There is **no compiled SDK to link against** — the contract is the JSON shape.

## The two command surfaces

Archicad exposes TWO command catalogs through the same endpoint:

| Surface | Origin | Command-name shape | Coverage |
|---|---|---|---|
| **Graphisoft built-in** | Ships with Archicad | `"API.GetClassificationSystemIds"` | Read-heavy: element listing, classification, properties, layout-book queries. The "official" surface. |
| **Tapir Add-On (community)** | https://github.com/ENZYME-APD/tapir-archicad-automation | Wrapped via `"API.ExecuteAddOnCommand"` | Write-heavy + workflow primitives the official surface lacks: bulk element creation, layer/property writes, publisher set triggering, library reload, BCF issue management, teamwork sync. |

The Tapir Add-On is the **de-facto standard** for any non-trivial automation. The official surface alone cannot create a Wall, set an element's layer, or trigger a Publish. Always assume both surfaces are available.

## Request envelope (Graphisoft built-in)

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command":    "API.GetClassificationSystemIds",
  "parameters": {}
}
```

Response:

```json
{
  "succeeded": true,
  "result": {
    "classificationSystemIds": [
      { "classificationSystemId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" } }
    ]
  }
}
```

## Request envelope (Tapir Add-On)

Tapir commands are reached through the **`ExecuteAddOnCommand`** wrapper:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "GetAllElements"
    },
    "addOnCommandParameters": {}
  }
}
```

Response:

```json
{
  "succeeded": true,
  "result": {
    "addOnCommandResponse": {
      "elements": [
        { "elementId": { "guid": "..." } }
      ]
    }
  }
}
```

## Preflight — is anything listening?

Before composing real work, probe `GetAddOnVersion`:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "GetAddOnVersion"
    },
    "addOnCommandParameters": {}
  }
}
```

A connection refused on `:19723` means:
- Archicad is not running, OR
- The Tapir Add-On is not loaded (check Options → Work Environment → Add-Ons), OR
- The user changed the listener port (look under Tapir's settings panel).

A successful response confirms both Archicad is up AND Tapir is reachable.

## Port and listener configuration

- **Default port:** `19723`
- **Configurable:** Yes, in Tapir Add-On settings. Composing apps SHOULD read the port from a config rather than hardcoding.
- **Bind address:** `127.0.0.1` only — Tapir does not bind 0.0.0.0 by default; cross-machine automation requires an SSH tunnel or a deliberate config change.
- **Concurrent requests:** Serialized on Archicad's main thread. A long-running publish blocks every other call.

## Main-thread marshaling and timeouts

Every JSON-RPC call enqueues onto Archicad's main thread. Implications:

- **A dialog box (modal Save / progress / error) blocks every JSON-RPC call** until dismissed by the user. Headless automation MUST suppress dialogs (where the verb supports it) and detect via a follow-up probe whether a dialog stole focus.
- **A 200-sheet Publish takes minutes.** Set HTTP client timeouts to 5+ minutes for `PublishPublisherSet`, `ReloadLibraries`, or any `Create*s` batch with hundreds of entries.
- **No async / streaming.** Every response is a single blob delivered when the verb finishes; no progress events.

## ExecutionResult shape (mutating commands)

Tapir mutating commands return a uniform shape:

```json
{
  "executionResult": {
    "success": true
  }
}
```

Or, on failure:

```json
{
  "executionResult": {
    "success": false,
    "error": {
      "code":    "<vendor-code>",
      "message": "Layer is locked"
    }
  }
}
```

Batch-mutating commands return `executionResults` (plural) — one entry per input. Per-element partial failure is common (locked layer, reserved-in-teamwork, missing property). Always iterate and count successes, never assume.

## Why this differs from other AEC automation

| Tool | Transport | Compiled SDK? | Runs in-process? |
|---|---|---|---|
| Rhino + RhinoCommon | C# CLI bridge | Yes (`RhinoCommon.dll`) | Yes |
| SketchUp + Ruby API | TCP + Ruby console | Yes (Ruby C extensions) | Yes |
| Revit + Revit API | Named pipe + Roslyn | Yes (`RevitAPI.dll`) | Yes (add-in) |
| AutoCAD + .NET API | In-process .NET | Yes (`acmgd.dll`) | Yes |
| **Archicad + Tapir** | **HTTP REST-ish** | **No** | **No** |

The Archicad surface is the closest of any major AEC tool to a stateless web API. This makes it the easiest to drive from any language — but also the strictest about the JSON shape (no IntelliSense; mistakes are runtime).

## Reference docs

- **Tapir command catalog (canonical):** https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/
- **Tapir source:** https://github.com/ENZYME-APD/tapir-archicad-automation
- **Graphisoft built-in JSON Interface:** https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/
- This agent's catalog: `catalog/Tapir.AdditionalCommands.*.json` and `catalog/Archicad.OfficialCommands.*.json` carry every verb + signature for v29.
