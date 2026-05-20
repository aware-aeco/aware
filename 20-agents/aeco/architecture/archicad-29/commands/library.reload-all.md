# library.reload-all

Reload every linked / external library in the project. The "I pushed an object update to the team server, refresh now" primitive.

`mode: write` — composing app must declare `safety:` block per v0.11. (Reloading libraries can cause GDL objects to re-resolve, which may visually change instances if the library content changed.)

## Inputs

This verb takes no parameters. It refreshes every library currently linked to the project — local LCF, linked folder libraries, BIMcloud libraries, and the embedded library.

## Implementation (Tapir / Graphisoft JSON-RPC)

Single call. Wraps Tapir `LibraryCommands.ReloadLibraries`. Archicad caches every linked library's object index in memory; when an Object/Door/Window/Lamp GDL file changes on disk (or the team server pushes a new version), instances in the project keep pointing at the cached version until a library reload re-indexes them.

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "ReloadLibraries"
    },
    "addOnCommandParameters": {}
  }
}
```

Response shape (Tapir `ExecutionResult`):

```json
{
  "executionResult": {
    "success": true
  }
}
```

On failure:

```json
{
  "executionResult": {
    "success": false,
    "error": {
      "code":    "<vendor-code>",
      "message": "<human-readable diagnostic>"
    }
  }
}
```

To count libraries reloaded, pre-probe with `GetLibraries` (the "how many" lives in the response, not the reload result):

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetLibraries"
    },
    "addOnCommandParameters": {}
  }
}
```

## Gotchas

- **No "ReloadHotlinks" verb exists.** Hotlinks (linked .mod / .pln modules) are a separate refresh path that Tapir does NOT expose over JSON-RPC. The closest workflow is `TeamworkReceive` (which catches hotlink updates pushed to the team server) followed by a manual Navigator → Hotlink Manager → "Update All" — there is no headless equivalent today.
- **Reload is synchronous and slow.** Re-indexing a large LCF (e.g. an ARCHICAD Library 29 install of 30k+ objects) blocks Archicad's main thread for several seconds. JSON-RPC clients block until done.
- **Embedded library changes do NOT need a reload.** Objects placed via "Embedded Library" live inside the .pln itself; their definitions are always current. Only LINKED libraries cache.
- **Reload can break missing-object placeholders.** If a library used to provide `MyObject.gsm` and the new version does not, every instance of `MyObject` becomes a "missing object" placeholder. The verb returns success but the model now has dotted-box placeholders.
- **Teamwork projects need a `TeamworkReceive` first** to pull the team-server-side library cache; otherwise reload merely re-indexes the stale local cache.
- **Open library-part editors lose work.** Any GDL editor window open against an object in the reloaded library will be force-closed; warn the user before triggering.

## See also

- [archicad-scripting-context](../skills/archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-hotlinks-and-teamwork](../skills/archicad-hotlinks-and-teamwork.md) — TeamworkReceive / hotlinks / BIMcloud library cache
