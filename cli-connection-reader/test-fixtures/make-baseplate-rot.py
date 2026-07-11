#!/usr/bin/env python
"""Generate a REAL base-plate connection IFC4 whose plate + anchor grid are YAWED 30° about the vertical
axis — the OBB recognition fixture. Same shape as make-baseplate.py but the plate is 400x200 (not square, so
the yaw is unambiguous) with an EQUAL-margin 2x2 anchor grid, and the whole building frame is rotated 30°
about world Z (which becomes web-ifc's vertical). A world-axis AABB would over-read it (~424x424 with a
mis-clustered grid); the oriented fit must recover the true dims.

Dimensions (mm, the recognition target, after the OBB removes the yaw):
  base plate  400 x 200 x 25                       -> plateWidth=400, plateDepth=200, thickness=25
  anchors     4 x M24 at (+-140, +-40)             -> cols=2, rows=2, edgeDist = min(200-140, 100-40) = 60
"""
import sys
import math
import ifcopenshell

OUT = sys.argv[1] if len(sys.argv) > 1 else "baseplate-rot.ifc"
YAW = math.radians(30.0)
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


length = f.create_entity("IfcSIUnit", UnitType="LENGTHUNIT", Name="METRE")
units = f.create_entity("IfcUnitAssignment", Units=[length])
wcs = f.create_entity("IfcAxis2Placement3D", Location=pt3(0, 0, 0), Axis=dir3(0, 0, 1), RefDirection=dir3(1, 0, 0))
ctx = f.create_entity("IfcGeometricRepresentationContext", ContextType="Model",
                      CoordinateSpaceDimension=3, Precision=1e-5, WorldCoordinateSystem=wcs)
proj = f.create_entity("IfcProject", GlobalId=gid(), Name="Base plate fixture (yawed)",
                       UnitsInContext=units, RepresentationContexts=[ctx])

site_plc = placement(None, 0, 0, 0)
site = f.create_entity("IfcSite", GlobalId=gid(), Name="Site", ObjectPlacement=site_plc, CompositionType="ELEMENT")
# Building frame YAWED 30 deg about world Z (RefDirection = (cos, sin, 0)); all child geometry rotates rigidly.
bldg_plc = f.create_entity("IfcLocalPlacement", PlacementRelTo=site_plc,
                           RelativePlacement=f.create_entity("IfcAxis2Placement3D", Location=pt3(0, 0, 0),
                                                             Axis=dir3(0, 0, 1), RefDirection=dir3(math.cos(YAW), math.sin(YAW), 0)))
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


OX, OY, OZ = 10.0, 20.0, 5.0

# base plate 400 x 200 x 25 (thin in Z / vertical), bottom at OZ
plate = f.create_entity("IfcPlate", GlobalId=gid(), Name="BASE PLATE PL25", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                        Representation=shape(rect_solid(0.4, 0.2, 0.025, OX, OY, OZ)))
column = f.create_entity("IfcColumn", GlobalId=gid(), Name="COLUMN UC203", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                         Representation=shape(rect_solid(0.203, 0.102, 1.0, OX, OY, OZ + 0.025)))
# 4 anchors M24 on a 280 x 80 grid (equal 60 mm margins on the 400 x 200 plate), spanning below to above it
bolts = []
for i, (dx, dy) in enumerate([(-0.14, -0.04), (0.14, -0.04), (-0.14, 0.04), (0.14, 0.04)]):
    bolts.append(f.create_entity("IfcMechanicalFastener", GlobalId=gid(), Name=f"ANCHOR M24-{i+1}",
                                 ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                                 Representation=shape(circ_solid(0.012, 0.25, OX + dx, OY + dy, OZ - 0.1))))

asm = f.create_entity("IfcElementAssembly", GlobalId=gid(), Name="BASEPLATE ROT", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                      AssemblyPlace="FACTORY")
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=asm, RelatedObjects=[column, plate] + bolts)
f.create_entity("IfcRelContainedInSpatialStructure", GlobalId=gid(), RelatingStructure=bldg, RelatedElements=[asm])

f.write(OUT)
print("wrote", OUT, "assembly GlobalId:", asm.GlobalId)
