# `ifc-inspector.entities.get-by-guid` — resolve one GUID to its full record

Stateless, read-only. Takes a single IFC `GlobalId` and returns its class, name, description, world placement, and all property sets. The spot-check verb — confirm a specific element before trusting it in a clash report or a COBie row.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |
| `guid` | string | The 22-char IFC `GlobalId` (not the `#n` line id). |

## Outputs

```yaml
class:       string
name:        string
description: string
placement:                # absolute world coordinates of the object placement origin
  x: number
  y: number
  z: number
psets:
  type: array
  items:
    name:       string    # the IfcPropertySet Name, e.g. "Pset_WallCommon"
    properties: object    # { propertyName: value } — IfcPropertySingleValue pairs
```

## Under the hood

`web-ifc` indexes by integer line id, so the agent first resolves the `GlobalId` to a line id (it builds a `GlobalId → expressID` map on open), then:

1. **Class / name / description** — read directly off the resolved line.
2. **Placement** — walk the `ObjectPlacement` → `IfcLocalPlacement` chain up to the project, composing each `RelativePlacement` transform, to get the **absolute** origin. (A bare `IfcLocalPlacement` is relative to its parent; the agent composes the whole chain so `placement` is world-space, consistent with what a clash tool sees.)
3. **Psets** — follow the inverse `IsDefinedBy` → `IfcRelDefinesByProperties` → `IfcPropertySet` relationships, flattening each `IfcPropertySingleValue` into the `properties` map.

Quantity sets (`IfcElementQuantity`) are **not** included here — `psets` covers `IfcPropertySet` only. For quantities, read the raw line. Type-level Psets (on the `IfcTypeObject`, via `IsTypedBy`) are merged in after instance Psets, with instance values taking precedence (this matches how viewers resolve effective properties).

## Composition example — verify a clashing element before reporting

```yaml
- id: el
  agent: ifc-inspector
  command: entities.get-by-guid
  config: { path: "{{ inputs.ifc }}", guid: "{{ clash.element-guid }}" }

- id: enrich
  inline:
    kind: map
    description: Attach human-readable identity + fire rating to the clash row
    code: e => ({
      identity: `${e.class} "${e.name}" @ (${e.placement.x.toFixed(2)}, ${e.placement.y.toFixed(2)})`,
      fireRating: e.psets.find(p => p.name === "Pset_WallCommon")?.properties?.FireRating ?? "n/a"
    })
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.guid-not-found` | No entity with that `GlobalId` | Confirm the GUID came from *this* file (GUIDs differ per discipline model); list with [`entities.list-guids`](./entities.list-guids.md) |
| `ifc.no-placement` | Entity has no `ObjectPlacement` (e.g. a non-located proxy) | `placement` returns `{0,0,0}` with the class/psets still resolved |

## See also

- [`entities.list-guids`](./entities.list-guids.md) — where the GUIDs come from
- [`psets.export`](./psets.export.md) — the same Pset data, in bulk, as CSV
- [ifc-psets-and-cobie](../skills/ifc-psets-and-cobie.md) — instance vs type Psets, and COBie property naming
