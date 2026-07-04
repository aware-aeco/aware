# connection-reader test fixtures

## `baseplate-bp1.ifc`

A minimal but real IFC4 base-plate connection (an IDEA-StatiCa-style export): one
`IfcElementAssembly` aggregating an `IfcColumn` stub + a horizontal `IfcPlate` base plate
(400×400×25 mm) + four `IfcMechanicalFastener` anchor bolts (M24 on a 240×240 grid, edge
distance 80 mm). Placed at a realistic site offset so the world→local re-anchor is also
exercised. Geometry is `IfcExtrudedAreaSolid` so web-ifc tessellates it like any real model.

This is the base-plate **recognition** fixture: `extract` on it fits a `recipe:{kind:"base-plate",
…}` (see `commands/extract.md`). Exports whose base plates lack modelled anchors correctly come
back with **no** recipe — that fallback path is covered by driving the reader against a full
model, not this fixture.

Regenerate with [`make-baseplate.py`](make-baseplate.py) (needs `ifcopenshell`; it is a fixture
authoring tool, not a runtime/CI dependency):

```bash
python make-baseplate.py baseplate-bp1.ifc
```

The assembly's GlobalId is random per regeneration — tests should discover it via `list` (the
first candidate), not hard-code it.

## `shearplate-sp1.ifc`

A minimal but real IFC4 shear / fin-plate connection: one `IfcElementAssembly` aggregating an
`IfcBeam` stub + an `IfcColumn` support stub + a vertical `IfcPlate` fin plate (10 mm thick ×
210 mm tall × 120 mm wide) + three `IfcMechanicalFastener` bolts in a single vertical line
(M20, pitch 70 mm, edge distance 35 mm). Placed at a site offset so the world→local re-anchor is
exercised. The fin plate is authored vertical with **horizontal** bolts — the inverse of the base
plate's horizontal-plate/vertical-anchor layout — which is exactly what discriminates the two.

This is the shear-plate **recognition** fixture: `extract` on it fits a
`recipe:{kind:"shear-plate", params:{plateThickness:10, plateHeight:210, plateWidth:120,
boltDia:20, boltCols:1, boltRows:3, boltPitch:70, edgeDist:35}}` (see `commands/extract.md`).

Regenerate with [`make-shearplate.py`](make-shearplate.py) (needs `ifcopenshell`; a fixture
authoring tool, not a runtime/CI dependency):

```bash
python make-shearplate.py shearplate-sp1.ifc
```

The assembly's GlobalId is random per regeneration — discover it via `list`, don't hard-code it.
