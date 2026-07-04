#!/usr/bin/env python
"""Generate a minimal but REAL shear/fin-plate connection IFC4 (an IDEA-StatiCa-style export): one
IfcElementAssembly aggregating an IfcBeam stub + an IfcColumn support stub + a VERTICAL IfcPlate fin plate
+ 3 horizontal IfcMechanicalFastener bolts in a single vertical line. Geometry is IfcExtrudedAreaSolid so
web-ifc tessellates it; placed at a site offset to exercise the world->local re-anchor. Recognition fixture.

Dimensions (mm, the recognition target, in the FINAL web-ifc Y-up frame where vertical = axis 1):
  fin plate   10 thick x 210 tall x 120 wide  -> plateThickness=10, plateHeight=210, plateWidth=120
  bolts       3 x M20 in one vertical line, pitch 70  -> boltCols=1, boltRows=3, boltPitch=70
  edge dist   min(210/2 - 70, 120/2 - 0) = 35

web-ifc converts IFC's Z-up world to its own Y-up output (IFC Z -> axis 1 vertical). So we author the fin
plate thin in X (bolt/web-normal axis), tall in Z (-> web-ifc height), wide in Y (-> along the beam), and the
bolts extruded along world +X (-> the horizontal web-normal axis that discriminates a fin plate from a base
plate). Only IfcPlate + IfcMechanicalFastener tessellate as parts; the beam/column are member references.
"""
import sys
import ifcopenshell

OUT = sys.argv[1] if len(sys.argv) > 1 else "shearplate-sp1.ifc"
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


# A placement whose local +Z (the extrude direction) points along world +X, so a circle profile extrudes
# into a cylinder lying HORIZONTALLY along X (the bolt / web-normal axis).
def axis3_x(x, y, z):
    return f.create_entity("IfcAxis2Placement3D", Location=pt3(x, y, z), Axis=dir3(1, 0, 0), RefDirection=dir3(0, 0, 1))


def placement(rel, x, y, z):
    return f.create_entity("IfcLocalPlacement", PlacementRelTo=rel, RelativePlacement=axis3(x, y, z))


# --- units (metre) + geometric context ---
length = f.create_entity("IfcSIUnit", UnitType="LENGTHUNIT", Name="METRE")
units = f.create_entity("IfcUnitAssignment", Units=[length])
wcs = f.create_entity("IfcAxis2Placement3D", Location=pt3(0, 0, 0), Axis=dir3(0, 0, 1), RefDirection=dir3(1, 0, 0))
ctx = f.create_entity("IfcGeometricRepresentationContext", ContextType="Model",
                      CoordinateSpaceDimension=3, Precision=1e-5, WorldCoordinateSystem=wcs)
proj = f.create_entity("IfcProject", GlobalId=gid(), Name="Shear plate fixture",
                       UnitsInContext=units, RepresentationContexts=[ctx])

site_plc = placement(None, 0, 0, 0)
site = f.create_entity("IfcSite", GlobalId=gid(), Name="Site", ObjectPlacement=site_plc, CompositionType="ELEMENT")
bldg_plc = placement(site_plc, 0, 0, 0)
bldg = f.create_entity("IfcBuilding", GlobalId=gid(), Name="Building", ObjectPlacement=bldg_plc, CompositionType="ELEMENT")
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=proj, RelatedObjects=[site])
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=site, RelatedObjects=[bldg])


# box centred in X and Y at (x,y), extruded from z upward by h. So it spans
# X in [x-w/2, x+w/2], Y in [y-d/2, y+d/2], Z in [z, z+h].
def rect_solid(w, d, h, x, y, z):
    prof = f.create_entity("IfcRectangleProfileDef", ProfileType="AREA",
                           Position=f.create_entity("IfcAxis2Placement2D", Location=pt2(0, 0)),
                           XDim=float(w), YDim=float(d))
    return f.create_entity("IfcExtrudedAreaSolid", SweptArea=prof, Position=axis3(x, y, z),
                           ExtrudedDirection=dir3(0, 0, 1), Depth=float(h))


# cylinder of radius r whose centre-line runs from world (x,y,z) to (x+length_m, y, z) — i.e. along +X, with
# the circular cross-section centred on world (y, z) in the Y-Z plane.
def cyl_x(r, length_m, x, y, z):
    prof = f.create_entity("IfcCircleProfileDef", ProfileType="AREA",
                           Position=f.create_entity("IfcAxis2Placement2D", Location=pt2(0, 0)), Radius=float(r))
    return f.create_entity("IfcExtrudedAreaSolid", SweptArea=prof, Position=axis3_x(x, y, z),
                           ExtrudedDirection=dir3(0, 0, 1), Depth=float(length_m))


def shape(solid):
    shp = f.create_entity("IfcShapeRepresentation", ContextOfItems=ctx,
                          RepresentationIdentifier="Body", RepresentationType="SweptSolid", Items=[solid])
    return f.create_entity("IfcProductDefinitionShape", Representations=[shp])


OX, OY, OZ = 10.0, 20.0, 5.0  # realistic site offset (metres) — exercises the world->local re-anchor

# Fin plate centred at (OX, OY, OZ): 0.010 thick (X) x 0.120 wide (Y, along beam) x 0.210 tall (Z, height).
plate = f.create_entity("IfcPlate", GlobalId=gid(), Name="FIN PLATE PL10", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                        Representation=shape(rect_solid(0.010, 0.120, 0.210, OX, OY, OZ - 0.105)))
# Supported beam stub running in +Y (into the span, away from the support), lapping the plate.
beam = f.create_entity("IfcBeam", GlobalId=gid(), Name="BEAM UB305", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                       Representation=shape(rect_solid(0.010, 1.0, 0.305, OX, OY + 0.560, OZ - 0.152)))
# Support column stub (the member the beam frames into), rising in +Z at the near edge.
col = f.create_entity("IfcColumn", GlobalId=gid(), Name="COLUMN UC203", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                      Representation=shape(rect_solid(0.203, 0.203, 1.0, OX - 0.107, OY, OZ - 0.5)))
# 3 M20 bolts (r=0.010, 60 long) in one vertical line at Y=OY, Z=OZ±0.070, running along +X through the plate.
bolts = []
for i, dz in enumerate([-0.070, 0.0, 0.070]):
    bolts.append(f.create_entity("IfcMechanicalFastener", GlobalId=gid(), Name=f"BOLT M20-{i+1}",
                                 ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                                 Representation=shape(cyl_x(0.010, 0.060, OX - 0.030, OY, OZ + dz))))

asm = f.create_entity("IfcElementAssembly", GlobalId=gid(), Name="SHEARPLATE SP1", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                      AssemblyPlace="FACTORY")
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=asm, RelatedObjects=[beam, col, plate] + bolts)
f.create_entity("IfcRelContainedInSpatialStructure", GlobalId=gid(), RelatingStructure=bldg, RelatedElements=[asm])

f.write(OUT)
print("wrote", OUT, "assembly GlobalId:", asm.GlobalId)
