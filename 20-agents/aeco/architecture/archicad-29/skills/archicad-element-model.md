---
name: archicad-element-model
description: This skill should be used when composing Archicad automation that reads or writes elements, classifications, or layers. Encodes Archicad's element/attribute/property/classification distinction (different from Revit's element/parameter model), the 65-value ElementType enum, the IFC-rooted classification model, the gotcha that "layer" is an attribute index rather than a property, and the relationship between hierarchical elements (Wall + Window/Door, Stair + Riser/Tread, CurtainWall + Frame/Panel) and their subelements.
---

# Archicad element model

Archicad's data model has four orthogonal axes that look like one axis at first glance:

| Axis | What it carries | Lives on | How you query |
|---|---|---|---|
| **Element** | The geometric thing (Wall, Slab, Zone) | The project document | `GetAllElements` / `GetElementsByType` |
| **Attribute** | A document-scoped reusable definition (Layer, Building Material, Composite, Surface, ZoneCategory) | The project's attribute tables | `GetAttributesByType` |
| **Property** | A typed user/built-in field on an element | The element via PropertyDefinition | `GetPropertyValuesOfElements` |
| **Classification** | An IFC-rooted taxonomy tag on an element | The element via ClassificationSystem | `GetClassificationsOfElements` |

Revit conflates Attribute and Property under "parameter"; Archicad does not. Understanding the four-way split is the difference between writing automation that works on every project and writing automation that breaks the moment someone swaps templates.

## The ElementType enum

Archicad has 65+ `ElementType` values. The catalog enumerates the full set; the workhorses for architecture:

| Type | Notes |
|---|---|
| `Wall` | Carries `Window` and `Door` subelements; layer-bound |
| `Column` | `ColumnSegment` subelements |
| `Beam` | `BeamSegment` subelements |
| `Slab` | Floor/ceiling/roof-slab; the horizontal plate |
| `Roof` | Multi-plane roofs; `Skylight` subelements |
| `Shell` | Free-form curved roofs/walls |
| `Mesh` | Topographic terrain |
| `Morph` | Free-form modeled solid |
| `Zone` | The room (see archicad-zones-and-spaces) |
| `Object` | GDL-based catalog object (chair, sink, tree, …) |
| `Lamp` | GDL light fixture (distinct type from Object) |
| `Window`, `Door`, `Opening` | Live INSIDE a host Wall / Slab / Roof |
| `Skylight` | Lives INSIDE a host Roof / Shell |
| `Stair` | Carries `Riser`, `Tread`, `StairStructure` subelements |
| `Railing` | Carries 12+ subelement types (Toprail, Handrail, Rail, Post, Baluster, Panel, Segment, Node, …) |
| `CurtainWall` | Carries Segment / Frame / Panel / Junction / Accessory subelements |
| `Dimension`, `RadialDimension`, `LevelDimension`, `AngleDimension` | Annotation |
| `Text`, `Label` | Annotation |
| `Hotlink` | The reference to a linked .mod / .pln module |
| `Hotspot`, `CutPlane`, `Camera`, `CamSet` | Workspace/view metadata |
| `Drawing` | A placed view on a layout sheet |
| `Detail`, `Elevation`, `InteriorElevation`, `Worksheet`, `Section` | Generated views |

The enum lives in `catalog/Tapir.Schema.ElementType.json`.

## Hierarchical elements and subelements

Some types are **hierarchical** — they own subelements that exist only inside them:

```
Wall
├── Window  (lives in this wall, gone if wall is deleted)
├── Door
└── Opening

Stair
├── Riser
├── Tread
└── StairStructure

Railing
├── RailingToprail
├── RailingHandrail
├── RailingRail
├── RailingPost
├── RailingBaluster
├── RailingPanel
├── RailingSegment
├── RailingNode
└── …

CurtainWall
├── CurtainWallSegment
├── CurtainWallFrame
├── CurtainWallPanel
├── CurtainWallJunction
└── CurtainWallAccessory
```

To query subelements of a hierarchical element, use Tapir's `GetSubelementsOfHierarchicalElements`. Do NOT assume that `GetAllElements` returns subelements — by default it returns only top-level elements; subelements are reached by descent from their host.

## Classifications — the IFC-rooted taxonomy

Every element can be classified under one or more ClassificationItems in one or more ClassificationSystems. Archicad ships with the **"ARCHICAD Classification"** system (which mirrors IFC: `IfcWall`, `IfcSlab`, `IfcWindow`, etc.) — that's the source of truth for IFC export.

```
ClassificationSystem  "ARCHICAD Classification - v 2.0"
├── ClassificationItem  IfcWall
├── ClassificationItem  IfcSlab
├── ClassificationItem  IfcWindow
├── ClassificationItem  IfcSpace            ← the IFC equivalent of Zone
└── …
```

Other ClassificationSystems may be installed via Library Manager (NBS Uniclass 2015, UniFormat, OmniClass, MasterFormat, custom firm taxonomies). All are queried the same way.

Useful properties of the classification system:
- **Classification is per-element AND per-system.** A single element can be `IfcWall` (in ARCHICAD Classification) AND `Ss_25_10_30` (in Uniclass) simultaneously.
- **Subelements inherit some classifications but not others.** `GetClassificationsOfElements` works for subelements; check the response carefully.
- **Unclassified is a valid state.** An element can have NO classification in a given system; the API returns `"classificationItemId": null` for that pair.

## "Layer" is an attribute, not a property

A common source of bugs. In Archicad:

```
Element.details.layerIndex  →  Layer attribute table  →  Layer "name"
```

The element carries an **integer index** into the project's layer table. The integer is NOT stable across projects (Layer #17 in one .pln may be a different layer in another) and NOT stable across template swaps.

To rewrite an element's layer, you must:
1. Resolve target layer NAME → INDEX via `GetAttributesByType("Layer")`,
2. Call `SetDetailsOfElements` with `{ "details": { "layerIndex": <int> } }`.

This is why every layer-related curated workflow verb does index-from-name resolution on each run — there is no shortcut.

## Reading element data

| Need | Verb |
|---|---|
| List every element | `GetAllElements` (Tapir) |
| List by type | `GetElementsByType` (Tapir, official) |
| List by classification | `GetElementsByClassification` (official) |
| Filter a known list | `FilterElements` (Tapir) — composable filters |
| Bounding box | `Get3DBoundingBoxes` (Tapir) |
| Geometry/parameters | `GetDetailsOfElements` (Tapir) |
| Property values | `GetPropertyValuesOfElements` (Tapir) |
| Classifications | `GetClassificationsOfElements` (Tapir, official) |
| GDL parameters (for Objects/Lamps/Doors/Windows) | `GetGDLParametersOfElements` (Tapir) |
| Selected elements | `GetSelectedElements` (Tapir, official) |
| Connected elements | `GetConnectedElements` (Tapir) — e.g. walls touching this slab |
| Subelements of a hierarchical | `GetSubelementsOfHierarchicalElements` (Tapir) |

## Writing element data

| Need | Verb |
|---|---|
| Create Walls / Slabs / Columns / Beams / Roofs / Morphs / Objects / Meshes / Zones / Labels | `Create<Type>s` (Tapir, one verb per type) |
| Create Windows / Doors / Openings (inside host walls) | `CreateWindows` / `CreateDoors` / `CreateOpenings` |
| Modify existing | `Modify<Type>s` (Tapir, per type) |
| Set details (layer, floor, order) | `SetDetailsOfElements` (Tapir) |
| Set classification | `SetClassificationsOfElements` (Tapir, official) |
| Set property values | `SetPropertyValuesOfElements` (Tapir) |
| Move | `MoveElements` (Tapir, vector per element) |
| Delete | `DeleteElements` (Tapir) |
| Highlight (preview/debug) | `HighlightElements` (Tapir) |

## Gotchas

- **Element GUIDs are stable across saves** within the same .pln, but a save-as-new-template re-assigns GUIDs to elements that were re-created.
- **`SetDetailsOfElements` is patch-style.** Pass only the fields you want to change. Passing all fields rewrites them; missing fields are NOT cleared.
- **The Property and Attribute namespaces both have a `CreateProperty…` verb shape** — Property* verbs are for the user-defined-field system; Attribute* verbs are for the reusable-definition system. Don't confuse them.
- **The Tapir Add-On version determines which verbs exist.** Some Create*/Modify* element verbs are new in Tapir 1.4.0. Probe `GetAddOnVersion` and fall back if the user runs an older Tapir.

## See also

- [archicad-scripting-context](./archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-zones-and-spaces](./archicad-zones-and-spaces.md) — Zone is one of the 65 ElementType values, with its own details schema
- [archicad-layouts-and-issue](./archicad-layouts-and-issue.md) — Layout sheets vs the model space
