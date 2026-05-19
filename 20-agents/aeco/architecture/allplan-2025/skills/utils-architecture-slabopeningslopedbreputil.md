---
name: allplan-utils-architecture-slabopeningslopedbreputil
description: This skill encodes the allplan 2025.0 surface of the Utils.Architecture.SlabOpeningSlopedBRepUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: SlabOpeningSlopedBRepUtil.
---

# Utils.Architecture.SlabOpeningSlopedBRepUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## SlabOpeningSlopedBRepUtil (class)

" implementation of the utility for a slab opening created by a sloped BRep3D

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/SlabOpeningSlopedBRepUtil/)

### Constructors
- `SlabOpeningSlopedBRepUtil(document: DocumentAdapter)` — initialize

### Methods
#### `create_opening_polygons_and_plane_surfaces( slab: BaseElementAdapter, opening_cut_geo: BRep3D ) -> tuple[bool, Polygon2D, Polyhedron3D | None, Polyhedron3D | None]`

create the opening polygons and planes for a BRep opening

**Remarks:** create the opening polygons and planes for a BRep opening

**Parameters:**
- `slab` (BaseElementAdapter) — slab
- `opening_cut_geo` (BRep3D) — geometry for the opening creation

**Returns:** `tuple[bool, Polygon2D, Polyhedron3D | None, Polyhedron3D | None]` — opening created state, opening polygon, bottom plane surface, top plane surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/SlabOpeningSlopedBRepUtil/#Utils.Architecture.SlabOpeningSlopedBRepUtil.SlabOpeningSlopedBRepUtil.create_opening_polygons_and_plane_surfaces)

