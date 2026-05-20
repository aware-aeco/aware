# element.list-by-classification

List every element on the plan that is classified under the given ClassificationItem in the given ClassificationSystem. The audit primitive for "show me every IfcWall" or "every fire-rated assembly".

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `classification-system-name` | string | yes | Display name of the ClassificationSystem (e.g. `"ARCHICAD Classification - v 2.0"`, `"NBS Uniclass 2015"`). |
| `classification-item-id-guid` | string | yes | The ClassificationItem GUID. Resolve human names via `GetAllClassificationsInSystem` first. |

## Implementation (Tapir / Graphisoft JSON-RPC)

Archicad's automation surface is JSON-RPC over HTTP on `127.0.0.1:19723` while Archicad is running with the Tapir Add-On loaded. The verb composes 3 native calls:

1. **`GetClassificationSystemIds`** (official) — resolve the ClassificationSystem GUID from its display name.
2. **`GetElementsByClassification`** (official) — list every element under the ClassificationItem.
3. **`GetTypesOfElements`** (official) — annotate each returned ElementId with its ElementType.

### Step 1 — Resolve the system GUID

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.GetClassificationSystemIds",
  "parameters": {}
}
```

Find the system whose `.name` matches `classification-system-name`; capture its `classificationSystemId.guid`.

### Step 2 — List elements under the item

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.GetElementsByClassification",
  "parameters": {
    "classificationItemId": {
      "guid": "{{ classification-item-id-guid }}"
    }
  }
}
```

Response shape:

```json
{
  "elements": [
    { "elementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" } },
    { "elementId": { "guid": "11111111-2222-3333-4444-555555555555" } }
  ]
}
```

### Step 3 — Annotate with ElementType

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.GetTypesOfElements",
  "parameters": {
    "elements": [
      { "elementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" } },
      { "elementId": { "guid": "11111111-2222-3333-4444-555555555555" } }
    ]
  }
}
```

Returns per element: `typesOfElements[i].typeOfElement.elementType` (e.g. `"Wall"`, `"Slab"`, `"CurtainWall"`).

## Gotchas

- **System display names contain a version suffix.** `"ARCHICAD Classification - v 2.0"` is not `"ARCHICAD Classification"`. Match by `startsWith` if the version drifts across templates.
- **`classificationItemId.guid` is global, not per-system.** Don't pass the system GUID here — they are different objects.
- **GUID format is dashed lowercase hex.** Tapir is strict; `{...}`-wrapped GUIDs fail silently.
- **IFC classification system has a fixed GUID.** If you target IFC every project, hardcode the GUID from a probe instead of resolving every run.
- **HTTP port is 19723 by default** but is configurable in Archicad's preferences. Probe `GetAddOnVersion` first to confirm reachability before composing the work.

## See also

- [archicad-scripting-context](../skills/archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-element-model](../skills/archicad-element-model.md) — IFC-rooted classification, ElementType enum
- [zone.list-with-properties](./zone.list-with-properties.md) — analogous list+enrich pattern for Zones
