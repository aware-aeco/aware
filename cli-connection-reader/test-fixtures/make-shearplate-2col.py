#!/usr/bin/env python
"""Generate a REAL bolted fin-plate connection IFC4 with a 2-COLUMN bolt grid — the multi-column shear
recognition fixture. Same shape as make-shearplate.py but the plate is wider (200 along the beam) and carries
2 columns x 3 rows of bolts (an independent horizontal boltColPitch the single-column recipe couldn't fit).

Dimensions (mm, the recognition target, in the FINAL web-ifc Y-up frame where vertical = axis 1):
  fin plate  10 thick x 210 tall x 200 wide          -> plateThickness=10, plateHeight=210, plateWidth=200
  bolts      2 cols (Y = +-40, pitch 80) x 3 rows (Z pitch 70)
             -> boltCols=2, boltRows=3, boltPitch=70, boltColPitch=80, edgeDist = 210/2 - 70 = 35
"""
import sys
import ifcopenshell

OUT = sys.argv[1] if len(sys.argv) > 1 else "shearplate-2col.ifc"
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


def axis3_x(x, y, z):
    return f.create_entity("IfcAxis2Placement3D", Location=pt3(x, y, z), Axis=dir3(1, 0, 0), RefDirection=dir3(0, 0, 1))


def placement(rel, x, y, z):
    return f.create_entity("IfcLocalPlacement", PlacementRelTo=rel, RelativePlacement=axis3(x, y, z))


length = f.create_entity("IfcSIUnit", UnitType="LENGTHUNIT", Name="METRE")
units = f.create_entity("IfcUnitAssignment", Units=[length])
wcs = f.create_entity("IfcAxis2Placement3D", Location=pt3(0, 0, 0), Axis=dir3(0, 0, 1), RefDirection=dir3(1, 0, 0))
ctx = f.create_entity("IfcGeometricRepresentationContext", ContextType="Model",
                      CoordinateSpaceDimension=3, Precision=1e-5, WorldCoordinateSystem=wcs)
proj = f.create_entity("IfcProject", GlobalId=gid(), Name="Shear plate fixture (2-col)",
                       UnitsInContext=units, RepresentationContexts=[ctx])

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


def cyl_x(r, length_m, x, y, z):
    prof = f.create_entity("IfcCircleProfileDef", ProfileType="AREA",
                           Position=f.create_entity("IfcAxis2Placement2D", Location=pt2(0, 0)), Radius=float(r))
    return f.create_entity("IfcExtrudedAreaSolid", SweptArea=prof, Position=axis3_x(x, y, z),
                           ExtrudedDirection=dir3(0, 0, 1), Depth=float(length_m))


def shape(solid):
    shp = f.create_entity("IfcShapeRepresentation", ContextOfItems=ctx,
                          RepresentationIdentifier="Body", RepresentationType="SweptSolid", Items=[solid])
    return f.create_entity("IfcProductDefinitionShape", Representations=[shp])


OX, OY, OZ = 10.0, 20.0, 5.0

# Fin plate: 0.010 thick (X) x 0.200 wide (Y, along beam) x 0.210 tall (Z, height).
plate = f.create_entity("IfcPlate", GlobalId=gid(), Name="FIN PLATE PL10", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                        Representation=shape(rect_solid(0.010, 0.200, 0.210, OX, OY, OZ - 0.105)))
beam = f.create_entity("IfcBeam", GlobalId=gid(), Name="BEAM UB305", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                       Representation=shape(rect_solid(0.010, 1.0, 0.305, OX, OY + 0.600, OZ - 0.152)))
col = f.create_entity("IfcColumn", GlobalId=gid(), Name="COLUMN UC203", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                      Representation=shape(rect_solid(0.203, 0.203, 1.0, OX - 0.107, OY, OZ - 0.5)))
# 6 M20 bolts: 2 columns (Y = OY +- 0.040, 80 mm apart) x 3 rows (Z = OZ +- 0.070), each running +X through the plate.
bolts = []
i = 0
for dy in [-0.040, 0.040]:
    for dz in [-0.070, 0.0, 0.070]:
        i += 1
        bolts.append(f.create_entity("IfcMechanicalFastener", GlobalId=gid(), Name=f"BOLT M20-{i}",
                                     ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                                     Representation=shape(cyl_x(0.010, 0.060, OX - 0.030, OY + dy, OZ + dz))))

asm = f.create_entity("IfcElementAssembly", GlobalId=gid(), Name="SHEARPLATE 2COL", ObjectPlacement=placement(bldg_plc, 0, 0, 0),
                      AssemblyPlace="FACTORY")
f.create_entity("IfcRelAggregates", GlobalId=gid(), RelatingObject=asm, RelatedObjects=[col, beam, plate] + bolts)
f.create_entity("IfcRelContainedInSpatialStructure", GlobalId=gid(), RelatingStructure=bldg, RelatedElements=[asm])

f.write(OUT)
print("wrote", OUT, "assembly GlobalId:", asm.GlobalId)
