# zone.list-with-properties

Enumerate every Zone (room) on the plan with name, number, category, area, polygon, and the requested property values. The room/zone audit primitive — produces the input for a room schedule, an area check, or a programming-vs-actual delta.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `property-ids` | string[] | no | `[]` | Tapir PropertyId GUIDs to fetch per zone. Empty = base fields only. |
| `story-filter` | number | no | — | Optional story index. Omit to scan every story. |

## Implementation (Tapir / Graphisoft JSON-RPC)

Composes 3 native Tapir calls. Archicad's Zone IS the room — it carries name, number, category (= room type / occupancy class), area, polygon, story, and any user-defined property values. The architect calls them "rooms" but the API enum is `Zone`.

### Step 1 — Enumerate all zones

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
    "addOnCommandParameters": { "elementType": "Zone" }
  }
}
```

Returns `{ "elements": [{ "elementId": { "guid": "..." } }, ...] }`.

### Step 2 — Fetch ZoneDetails per zone

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

For Zone elements, `detailsOfElements[i].details` includes:
- `name` — display name (e.g. "Living Room")
- `numberStr` — zone number (e.g. "101")
- `categoryAttributeId` — index of the ZoneCategory attribute
- `polygonCoordinates`, `polygonArcs`, `holes` — geometry
- `zCoordinate` — story-elevation
- `isManual` — true if hand-placed vs auto-detected from walls

### Step 3 — (optional) Property values

If `property-ids` is non-empty:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetPropertyValuesOfElements"
    },
    "addOnCommandParameters": {
      "elements": [
        { "elementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" } }
      ],
      "properties": [
        { "propertyId": { "guid": "11111111-2222-3333-4444-555555555555" } }
      ]
    }
  }
}
```

Built-in properties to grab for a room schedule:
- **Net Area** — `propertyId.guid` from `GetAllProperties` filtered to group `"General Element"` → name `"Net Area"`
- **Gross Area**, **Volume**, **Wall Inset Filler**, etc.

### Step 4 — (optional) Filter by story

`GetDetailsOfElements` does not return story index for Zones in every Tapir version — fetch the story index from `ProjectCommands.GetStories` and cross-reference `zCoordinate` against story `level` to bucket per story.

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName": "GetStories"
    },
    "addOnCommandParameters": {}
  }
}
```

## Gotchas

- **Zone polygon area ≠ schedule net area.** The polygon `polygonCoordinates` give a gross 2D footprint; "Net Area" subtracts wall-foot insets, columns, etc. and is a separately-stored property. Use `GetPropertyValuesOfElements` for the schedule number, NOT a polygon shoelace formula.
- **`categoryAttributeId` is an index, not a name.** Resolve via `GetAttributesByType("ZoneCategory")` to get the human-readable category (e.g. "Bedroom", "Office", "Wet Room").
- **Manually-drawn zones can have stale geometry.** `isManual: true` means the polygon does NOT auto-update when surrounding walls move. Flag manual zones in the audit output.
- **Disconnected zones.** A zone whose wall outline has been deleted persists as a "hanging" zone — its `polygonCoordinates` are still valid but unrelated to any room. Cross-check with `GetZoneBoundaries` to detect.
- **Property GUIDs are per-project for user-defined properties.** Built-in properties have stable GUIDs across templates; custom ones do not. Resolve via `GetAllProperties` per project.

## See also

- [archicad-scripting-context](../skills/archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-zones-and-spaces](../skills/archicad-zones-and-spaces.md) — Zone vs Room, categories, gross vs net area
- [archicad-element-model](../skills/archicad-element-model.md) — ElementType enum (Zone is one of 65)
- [element.list-by-classification](./element.list-by-classification.md) — analogous list+enrich pattern for IFC types
