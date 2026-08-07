#!/usr/bin/env python
"""Generate a minimal but REAL base-plate connection IFC4 (an IDEA-StatiCa-style export):
one IfcElementAssembly aggregating an IfcColumn stub + a horizontal IfcPlate base plate +
4 IfcMechanicalFastener anchor bolts. Geometry is IfcExtrudedAreaSolid (rect/circle profiles)
so web-ifc tessellates it. Placed at a realistic site offset so the world->local re-anchor is
also exercised. This is the connection-reader's base-plate recognition fixture.

Dimensions (mm, the recognition target):
  base plate  400 x 400 x 25, bottom face at top-of-foundation
  anchors     4 x M24 on a 240 x 240 grid  -> cols=2 rows=2, edge dist = 200-120 = 80
  column      UC203 box stub, seats on the plate top
"""
import sys
import ifcopenshell

OUT = sys.argv[1] if len(sys.argv) > 1 else "baseplate-bp1.ifc"
# The site offset is a PARAMETER because `probe`'s bbox error is a property of it, not of the
# algorithm (aware-aeco/aware#348): the box runs from the world origin to the model, so its midpoint
# is off by roughly half the offset. Passing 0 0 0 authors the same connection AT the origin, which
# is the only way to exercise the other half of that claim — the case where the midpoint is exact.
# Defaults are unchanged, so `python make-baseplate.py baseplate-bp1.ifc` still regenerates bp1.
OX, OY, OZ = (float(v) for v in (sys.argv[2:5] if len(sys.argv) > 4 else (10.0, 20.0, 5.0)))
f = ifcopenshell.file(schema="IFC4")


def gid():
    return ifcopenshell.guid.new()


def dir3(x, y, z):
    return f.create_entity("IfcDirection", DirectionRatios=(float(x), float(y), float(z)))


def pt3(x, y, z):
    return f.create_entity("IfcCartesianPoint", Coordinates=(float(x), float(y), float(z)))


def pt2(x, y):
    return f.create_entity("IfcCartesianPoint", Coordinates=(float(x), float(y)))


def axis3(x, y, z):
    return f.create_entity("IfcAxis2Placement3D", Location=pt3(x, y, z))


def placement(rel, x, y, z):
    return f.create_entity("IfcLocalPlacement", PlacementRelTo=rel, RelativePlacement=axis3(x, y, z))


# --- units (metre) + geometric context ---
length = f.create_entity("IfcSIUnit", UnitType="LENGTHUNIT", Name="METRE")
units = f.create_entity("IfcUnitAssignment", Units=[length])
wcs = f.create_entity("IfcAxis2Placement3D", Location=pt3(0, 0, 0), Axis=dir3(0, 0, 1), RefDirection=dir3(1, 0, 0))
ctx = f.create_entity("IfcGeometricRepresentationContext", ContextType="Model",
                      CoordinateSpaceDimension=3, Precision=1e-5, WorldCoordinateSystem=wcs)
proj = f.create_entity("IfcProject", GlobalId=gid(), Name="Base plate fixture",
                       UnitsInContext=units, RepresentationContexts=[ctx])

# --- spatial structure ---
site_plc = placement(None, 0, 0, 0)
site = f.create_entity("IfcSite", GlobalId=gid(), Name="Site", ObjectPlacement=site_plc, CompositionType="ELEMENT")
bldg_plc = placement(site_plc, 0, 0, 0)
bldg = f.create_entity("IfcBuilding", GlobalId=gid(), Name="Building", ObjectPlacement=bldg_plc, CompositionType="ELEMENT")
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=proj, RelatedObjects=[site])
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=site, RelatedObjects=[bldg])


def rect_solid(w, d, h, x, y, z):
    prof = f.create_entity("IfcRectangleProfileDef", ProfileType="AREA",
                           Position=f.create_entity("IfcAxis2Placement2D", Location=pt2(0, 0)),
                           XDim=float(w), YDim=float(d))
    return f.create_entity("IfcExtrudedAreaSolid", SweptArea=prof, Position=axis3(x, y, z),
                           ExtrudedDirection=dir3(0, 0, 1), Depth=float(h))


def circ_solid(r, h, x, y, z):
    prof = f.create_entity("IfcCircleProfileDef", ProfileType="AREA",
                           Position=f.create_entity("IfcAxis2Placement2D", Location=pt2(0, 0)), Radius=float(r))
    return f.create_entity("IfcExtrudedAreaSolid", SweptArea=prof, Position=axis3(x, y, z),
                           ExtrudedDirection=dir3(0, 0, 1), Depth=float(h))


def shape(solid):
    shp = f.create_entity("IfcShapeRepresentation", ContextOfItems=ctx,
                          RepresentationIdentifier="Body", RepresentationType="SweptSolid", Items=[solid])
    return f.create_entity("IfcProductDefinitionShape", Representations=[shp])


# base plate 0.4 x 0.4 x 0.025, bottom at OZ (top of foundation)
plate = f.create_entity("IfcPlate", GlobalId=gid(), Name="BASE PLATE PL25", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                        Representation=shape(rect_solid(0.4, 0.4, 0.025, OX, OY, OZ)))
# column UC203 box stub seats on the plate top (OZ + 0.025), 1 m tall
column = f.create_entity("IfcColumn", GlobalId=gid(), Name="COLUMN UC203", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                         Representation=shape(rect_solid(0.203, 0.203, 1.0, OX, OY, OZ + 0.025)))
# 4 anchors M24 (r=0.012) on a 0.24 x 0.24 grid, spanning below the plate to above it
bolts = []
for i, (dx, dy) in enumerate([(-0.12, -0.12), (0.12, -0.12), (-0.12, 0.12), (0.12, 0.12)]):
    bolts.append(f.create_entity("IfcMechanicalFastener", GlobalId=gid(), Name=f"ANCHOR M24-{i+1}",
                                 ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                                 Representation=shape(circ_solid(0.012, 0.25, OX + dx, OY + dy, OZ - 0.1))))

asm = f.create_entity("IfcElementAssembly", GlobalId=gid(), Name="BASEPLATE BP1", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                      AssemblyPlace="FACTORY")
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=asm, RelatedObjects=[column, plate] + bolts)
f.create_entity("IfcRelContainedInSpatialStructure", GlobalId=gid(), RelatingStructure=bldg, RelatedElements=[asm])

f.write(OUT)
print("wrote", OUT, "assembly GlobalId:", asm.GlobalId)
