# element.set-layer-by-pattern

Move every element whose current layer name matches a regex onto a target layer. Used for cleaning imported models or normalizing DWG-derived geometry.

`mode: write` — composing app must declare `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `from-pattern` | string | yes | — | Regex matched against the source layer's name. |
| `to-layer-name` | string | yes | — | Target layer name. Must exist (create via `AttributeCommands.CreateLayers` first). |
| `element-types` | string[] | no | `[]` | Optional filter — only elements of these `ElementType` values are touched. Empty = all. |

## Implementation (Tapir / Graphisoft JSON-RPC)

Composes 4 native calls. Archicad's layers are **attributes**, not properties — they live on the element via `floorLayerIndex` (in legacy parlance) or via `details.layerIndex` (modern Tapir `GetDetailsOfElements` shape). Layer assignment is set per-element through `SetDetailsOfElements`.

### Step 1 — Resolve target layer index

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.GetAttributesByType",
  "parameters": { "attributeType": "Layer" }
}
```

Find the entry whose `.name == to-layer-name`; capture its `.attributeId.index`.

Or, equivalently (Tapir):

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetAttributesByType"
    },
    "addOnCommandParameters": { "attributeType": "Layer" }
  }
}
```

### Step 2 — Enumerate elements (optionally type-filtered)

If `element-types` is non-empty, fan out one `GetElementsByType` per type and concat:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetElementsByType"
    },
    "addOnCommandParameters": { "elementType": "Wall" }
  }
}
```

Otherwise call `GetAllElements`:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetAllElements"
    },
    "addOnCommandParameters": {}
  }
}
```

### Step 3 — Read current layer per element

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetDetailsOfElements"
    },
    "addOnCommandParameters": {
      "elements": [
        { "elementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" } }
      ]
    }
  }
}
```

Each entry's `details.layerIndex` is an integer that resolves against the layer attribute table from step 1. Cross-reference the regex against the layer's `.name` to decide which elements move.

### Step 4 — Write the target layer onto matching elements

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "SetDetailsOfElements"
    },
    "addOnCommandParameters": {
      "elementsWithDetails": [
        {
          "elementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" },
          "details":   { "layerIndex": 17 }
        },
        {
          "elementId": { "guid": "11111111-2222-3333-4444-555555555555" },
          "details":   { "layerIndex": 17 }
        }
      ]
    }
  }
}
```

The response's `executionResults[]` carries per-element success/failure — count `success: true` entries for `moved-count`.

## Gotchas

- **Layer indices, not GUIDs.** Tapir element layer references are integer indices into the per-document layer table. They are NOT stable across templates, which is why the verb resolves index-from-name on every run.
- **Locked layers reject writes.** `SetDetailsOfElements` returns `success: false, error: "Layer is locked"` per element. Check the response — do NOT assume success.
- **No transaction wrapper, no undo grouping.** Archicad does not expose a transaction primitive over JSON-RPC. Each `SetDetailsOfElements` call is atomic per element, but a partial failure mid-batch leaves the model split. The composing app should snapshot the project (`SaveProject`) before invoking.
- **Hidden-layer elements still match.** Visibility ≠ existence. The regex sees every element on the plan.
- **Element types include 60+ entries** beyond the obvious — `Stair`, `Riser`, `Tread`, `RailingNode`, `BeamSegment`, `Hotlink`, etc. See `Tapir.Schema.ElementType` enum in the catalog.

## See also

- [archicad-scripting-context](../skills/archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-element-model](../skills/archicad-element-model.md) — ElementType enum + layer-as-attribute distinction
