---
name: ifc-psets-and-cobie
description: This skill should be used when extracting or checking IFC property sets with ifc-inspector — how Psets attach via IfcRelDefinesByProperties, instance vs type Pset resolution (instance wins), the difference between Psets and quantity sets, the standard Pset_*Common names, and what COBie deliverables require. Encodes the property model behind psets.export and psets.find-missing.
---

# IFC Psets & COBie

## How properties attach

IFC keeps properties *out* of the entity and attaches them through relationships. The chain for an instance:

```
IfcWall  ──IsDefinedBy──▶  IfcRelDefinesByProperties  ──▶  IfcPropertySet "Pset_WallCommon"
                                                              ├─ IfcPropertySingleValue FireRating = "REI 60"
                                                              ├─ IfcPropertySingleValue IsExternal = .T.
                                                              └─ IfcPropertySingleValue LoadBearing = .F.
```

A property set (`IfcPropertySet`, conventionally named `Pset_*`) groups `IfcPropertySingleValue` name/value pairs. [`entities.get-by-guid`](../commands/entities.get-by-guid.md) flattens these into a `{ propertyName: value }` map per Pset; [`psets.export`](../commands/psets.export.md) spreads them into CSV columns named `<PsetName>.<PropertyName>`.

## Instance vs type — and who wins

Properties can live in two places:

- **Instance Psets** — on the individual `IfcWall` (via `IfcRelDefinesByProperties`).
- **Type Psets** — on the shared `IfcWallType` the wall references (via `IfcRelDefinesByType` → `IsTypedBy`). One door type carries the manufacturer/model once; 200 door instances inherit it.

The agent resolves **effective** properties = type Psets overlaid with instance Psets, **instance wins** on conflict. This matches how viewers and COBie exporters compute the value a user actually sees. Both [`entities.get-by-guid`](../commands/entities.get-by-guid.md) and [`psets.find-missing`](../commands/psets.find-missing.md) use this resolution — so a property satisfied only at the type level still counts as present.

## Psets vs quantities

Two different things, often confused:

- **`IfcPropertySet`** (`Pset_*`) — qualitative/descriptive: `FireRating`, `IsExternal`, `AcousticRating`.
- **`IfcElementQuantity`** (`Qto_*`, `BaseQuantities`) — measured quantities: `NetVolume`, `GrossSideArea`, `Length`.

`ifc-inspector`'s pset verbs cover **`IfcPropertySet` only**. Quantity sets are not surfaced by `psets.export` / `get-by-guid` — read the raw line if you need `Qto_`/`BaseQuantities`. Don't go looking for `NetVolume` in a Pset export; it isn't a property, it's a quantity.

## The standard Pset names

buildingSMART defines canonical Psets per class — the ones a well-behaved export populates:

| Class | Canonical Pset | Common properties |
|---|---|---|
| `IfcWall` | `Pset_WallCommon` | `FireRating`, `IsExternal`, `LoadBearing`, `ThermalTransmittance` |
| `IfcDoor` | `Pset_DoorCommon` | `FireRating`, `AcousticRating`, `IsExternal`, `SelfClosing` |
| `IfcSpace` | `Pset_SpaceCommon` | `Reference`, `IsExternal`, `GrossPlannedArea`, `NetPlannedArea` |
| `IfcBuildingStorey` | `Pset_BuildingStoreyCommon` | `EntranceLevel`, `AboveGround` |

Authoring tools also emit non-standard Psets (`Pset_Revit_*`, project-custom). Use [`entities.count-by-class`](../commands/entities.count-by-class.md) + [`psets.export`](../commands/psets.export.md) with no `pset-names` filter to discover what a given file actually carries before assuming the canonical names are present.

## COBie

COBie (Construction-Operations Building information exchange) is the FM handover deliverable — a structured extract of the asset data an owner needs to operate the building. It maps onto specific IFC Psets:

- **Spaces** → `Pset_SpaceCommon` + `COBie_Space` (RoomTag, UsableHeight).
- **Components** (serviceable equipment) → `COBie_Component`, `COBie_Asset`, plus type-level `COBie_Type` (manufacturer, model, warranty, expected life).
- The "Type vs Component" split in COBie is literally the IFC type/instance split above — which is why correct **type-Pset resolution** matters: a door's manufacturer lives on `COBie_Type` (the type), not the instance.

The data-side COBie pre-check is: [`psets.find-missing`](../commands/psets.find-missing.md) with the project's required `COBie_*` properties as the `required` contract → an empty `missing` array is the go/no-go for issuance. Full rule-based COBie validation (value ranges, classification) is a [`solibri`](../../../architecture/solibri-26.4.1/) job; presence/non-emptiness is this agent's.

## See also

- [`psets.export`](../commands/psets.export.md) · [`psets.find-missing`](../commands/psets.find-missing.md) · [`entities.get-by-guid`](../commands/entities.get-by-guid.md) — the verbs
- [ifc-guid-and-class-model](./ifc-guid-and-class-model.md) — Pset-reference values as a stable identity fallback
- [what-this-agent-does](./what-this-agent-does.md) — when presence-checking is enough vs needing `solibri`'s rule engine
