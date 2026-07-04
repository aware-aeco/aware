# connection-reader test fixtures

## `baseplate-bp1.ifc`

A minimal but real IFC4 base-plate connection (an IDEA-StatiCa-style export): one
`IfcElementAssembly` aggregating an `IfcColumn` stub + a horizontal `IfcPlate` base plate
(400×400×25 mm) + four `IfcMechanicalFastener` anchor bolts (M24 on a 240×240 grid, edge
distance 80 mm). Placed at a realistic site offset so the world→local re-anchor is also
exercised. Geometry is `IfcExtrudedAreaSolid` so web-ifc tessellates it like any real model.

This is the **recognition** fixture: `extract` on it fits a `recipe:{kind:"base-plate", …}`
(see `commands/extract.md`). Real Tekla exports (whose base plates lack modelled anchors, and
whose other connections are welded gussets / shear tabs) correctly come back with **no** recipe
— that fallback path is covered by driving the reader against a full model, not this fixture.

Regenerate with [`make-baseplate.py`](make-baseplate.py) (needs `ifcopenshell`; it is a fixture
authoring tool, not a runtime/CI dependency):

```bash
python make-baseplate.py baseplate-bp1.ifc
```

The assembly's GlobalId is random per regeneration — tests should discover it via `list` (the
first candidate), not hard-code it.
