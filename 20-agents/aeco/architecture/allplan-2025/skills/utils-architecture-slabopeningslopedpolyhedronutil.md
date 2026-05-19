---
name: allplan-utils-architecture-slabopeningslopedpolyhedronutil
description: This skill encodes the allplan 2025.0 surface of the Utils.Architecture.SlabOpeningSlopedPolyhedronUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: SlabOpeningSlopedPolyhedronUtil.
---

# Utils.Architecture.SlabOpeningSlopedPolyhedronUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## SlabOpeningSlopedPolyhedronUtil (class)

" implementation of the utility for a slab opening created by a sloped Polyhedron3D

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/SlabOpeningSlopedPolyhedronUtil/)

### Constructors
- `SlabOpeningSlopedPolyhedronUtil(document: DocumentAdapter)` — initialize

### Methods
#### `create_opening_polygons_and_plane_surfaces( slab: BaseElementAdapter, opening_cut_geo: Polyhedron3D ) -> tuple[bool, Polygon2D, Polyhedron3D | None, Polyhedron3D | None]`

create the opening polygons and planes for a Polyhedron opening

**Remarks:** create the opening polygons and planes for a Polyhedron opening

**Parameters:**
- `slab` (BaseElementAdapter) — slab
- `opening_cut_geo` (Polyhedron3D) — geometry for the opening creation

**Returns:** `tuple[bool, Polygon2D, Polyhedron3D | None, Polyhedron3D | None]` — opening created state, opening polygon, bottom plane surface, top plane surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/Architecture/SlabOpeningSlopedPolyhedronUtil/#Utils.Architecture.SlabOpeningSlopedPolyhedronUtil.SlabOpeningSlopedPolyhedronUtil.create_opening_polygons_and_plane_surfaces)

