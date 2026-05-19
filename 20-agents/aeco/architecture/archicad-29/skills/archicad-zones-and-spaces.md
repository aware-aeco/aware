---
name: archicad-zones-and-spaces
description: This skill should be used when composing Archicad automation that touches rooms, spaces, areas, or room schedules. Encodes Archicad's Zone concept (which the API calls "Zone" but architects call "Room"), the gross-vs-net area distinction, zone categories (occupancy / use type), manual vs auto zones, the polygon+arc+holes geometry shape, zone boundaries (which walls/slabs bound this room), and the relationship between Archicad Zones and IFC IfcSpace.
---

# Archicad zones and spaces

Archicad's **Zone** is the architect's **Room**. The two words mean the same thing throughout this skill — `Zone` is the API enum value, "room" is the human label.

A Zone is a typed element that carries:
- A **name** ("Living Room")
- A **number** ("101")
- A **category** (Bedroom / Office / Wet Room / …) — references a ZoneCategory attribute
- A **polygon** describing its 2D footprint
- A **story** (which floor it sits on)
- Optional **holes** (e.g. an atrium void)
- Computed **areas** (gross, net, reduced, …) and **volume** as properties
- Any user-defined property values (occupancy, lighting load, hourly rate, FFE budget, …)

## Why Zone ≠ generic geometry

A Zone is NOT just a polygon. It is a first-class BIM element that:

1. **Schedules.** A Zone shows up in Archicad's interactive room schedule and in any IFC export as `IfcSpace`.
2. **Auto-resolves boundaries.** An "auto" zone discovers its outline from the surrounding walls — when a wall moves, the zone's polygon (and area) updates.
3. **Stamps the plan.** Every zone has a "zone stamp" — a placed annotation block showing name/number/area, which the architect can format per ZoneCategory.
4. **Joins to building physics.** Zones feed energy analysis, thermal simulation, IFC4 spatial structure, and BIMcloud cost-takeoff workflows.

The polygon geometry is a side-effect. The Zone identity persists across edits.

## Manual vs automatic zones

Two ways to create a Zone:

| Mode | How created | Auto-updates when walls move? | When to use |
|---|---|---|---|
| **Automatic** | Click "magic wand" inside a wall-bounded space — Archicad traces the outline | Yes, polygon and area re-resolve on every regenerate | Standard architectural workflow — the room IS what the walls enclose |
| **Manual** | Draw the polygon by hand | NO — polygon is frozen at draw time | Pre-design / programming phase (no walls yet); odd spaces (the porch outside the building envelope); "leased area" boundaries that ignore physical walls |

The `isManual: true` flag in `ZoneDetails` flags manual zones. Audit code should always surface manual zones — they ARE common bug sources because they silently drift as the design evolves.

## ZoneCategory — the occupancy / use-type axis

Every Zone references a **ZoneCategory** attribute. The category drives:
- The default zone stamp style (an "Office" stamp looks different from a "Wet Room" stamp)
- The default name (creating a "Bedroom"-category zone defaults its name to "Bedroom")
- The default color tint shown on plan
- IFC export's `IfcSpace.PredefinedType`
- Per-firm cost / area-bonus rules (a Wet Room counts as 0.5x for letting-area calcs, etc.)

```
ZoneCategory  "Office"
ZoneCategory  "Bedroom"
ZoneCategory  "Wet Room"
ZoneCategory  "Circulation"
ZoneCategory  "Roof Terrace (Outdoor)"
```

Categories live in `catalog/Tapir.AdditionalCommands.AttributeCommands.json` — query with `GetAttributesByType("ZoneCategory")`. The ZoneDetails reports a `categoryAttributeId` (integer index into the attribute table); resolve to the human name via the attribute query.

## Gross area vs net area vs reduced area

A common source of "the schedule doesn't match my hand calc" bugs.

| Area | What it is | Source |
|---|---|---|
| **Polygon area** | The 2D footprint area computed from `polygonCoordinates` + `polygonArcs` minus `holes` via the shoelace formula | The polygon itself |
| **Gross Area** | The full polygon area (what the polygon describes) | Property `"Gross Area"` |
| **Net Area** | Gross minus wall-foot insets, columns, fixed cabinetry, etc. | Property `"Net Area"` |
| **Reduced Area** | Net adjusted by ZoneCategory's per-category area-reduction rule (e.g. low ceilings, slope) | Property `"Reduced Area"` |
| **Measured Area** | A configured "official" area used in lease/sale documents — UK RICS / DIN 277 / etc. | Project-dependent property |

For an audit, use `GetPropertyValuesOfElements` to fetch the explicit area properties — DO NOT shoelace the polygon and assume that's the schedule number.

## Geometry shape

A Zone's footprint geometry is three fields:

```json
{
  "polygonCoordinates": [
    { "x": 0.0, "y": 0.0 },
    { "x": 5.0, "y": 0.0 },
    { "x": 5.0, "y": 4.0 },
    { "x": 0.0, "y": 4.0 },
    { "x": 0.0, "y": 0.0 }
  ],
  "polygonArcs": [
    { "begIndex": 1, "endIndex": 2, "arcAngle": 0.5236 }
  ],
  "holes": [
    {
      "polygonCoordinates": [
        { "x": 1.0, "y": 1.0 },
        { "x": 2.0, "y": 1.0 },
        { "x": 2.0, "y": 2.0 },
        { "x": 1.0, "y": 2.0 },
        { "x": 1.0, "y": 1.0 }
      ]
    }
  ]
}
```

- `polygonCoordinates` is closed (first point repeats at end).
- `polygonArcs[]` upgrades polygon edges to arcs by referring to the edge's `begIndex`/`endIndex`; angle in radians.
- `holes[]` describes inner voids (atria, light wells, internal courtyards).

Use `Coordinate2D` for 2D points. The `zCoordinate` field on `ZoneDetails` gives the story-level elevation.

## Zone boundaries — which walls bound this room?

Use Tapir's `GetZoneBoundaries`:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "GetZoneBoundaries"
    },
    "addOnCommandParameters": {
      "zoneElementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" }
    }
  }
}
```

Returns the connected elements (walls, slabs, columns) that form the room's envelope, plus neighbour zones (rooms sharing a boundary). Useful for:
- Adjacency graphs (room → neighbours)
- "Which wall is between Office 101 and the corridor?" queries
- Fire-compartment audits
- Disconnected-zone detection (the call returns no boundaries → the zone has no walls)

## Zone → IFC mapping

When Archicad exports IFC:

| Archicad concept | IFC entity |
|---|---|
| Zone | `IfcSpace` |
| ZoneCategory | `IfcSpace.PredefinedType` (+ classification reference) |
| Zone polygon | `IfcSpace.Representation` (footprint sweep) |
| Zone story | `IfcBuildingStorey` containment |
| Zone name | `IfcSpace.Name` |
| Zone number | `IfcSpace.LongName` (or `IfcSpace.Number` — depends on Archicad's IFC mapping config) |

If your audit needs to round-trip with an IFC export, prefer the Archicad property `"IfcSpace_Name"` over the raw `name` — the IFC mapping may transform it.

## Gotchas

- **Zone is a top-level element, not a subelement of anything.** It does NOT live inside a Wall the way Window does.
- **Zone polygon does NOT include the bounding walls' thickness.** If you need the "envelope to envelope" outline for area takeoff, use Gross Area / a thickened polygon — NOT the raw polygon.
- **An auto zone with all surrounding walls deleted survives** as a "hanging" zone with stale geometry. Filter on `GetZoneBoundaries` returning empty.
- **Two zones can overlap.** Archicad does not enforce non-overlapping rooms (e.g. a "Suite 5" zone may overlap "Bedroom 5A" + "Bedroom 5B"). For occupancy schedules, dedup carefully.
- **The "zone stamp" element is separate.** The stamp is a `Zone` element's automatic annotation; it lives on the plan view. You cannot move the stamp independently of the zone — moving the stamp moves the zone's `stampPosition`.
- **Zones do not auto-create on import.** A DWG / IFC import does NOT create Archicad Zones; the architect must place them. An automated room-tagging workflow has to call `CreateZones` per detected room polygon.

## See also

- [archicad-scripting-context](./archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-element-model](./archicad-element-model.md) — Zone in the broader ElementType context
- [zone.list-with-properties](../commands/zone.list-with-properties.md) — the curated audit verb
