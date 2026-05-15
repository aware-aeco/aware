---
name: tekla-bent-plates-and-custom-shapes
description: Tekla Open API - Bent plates, custom shapes, B-Rep geometry. BentPlateGeometrySolver for multi-leg bent plates. FacetedBrep and ShapeItem for custom geometry in shape catalog. ConnectiveGeometry, Contour, ContourPoint, Chamfer. This skill should be used when creating bent plates, lofted plates, custom shapes, shape catalog items, or complex 3D geometry.
---

# Tekla Bent Plates & Custom Shapes (v2025)

## Critical Patterns

- `BentPlateGeometrySolver` builds geometry leg by leg
- First call `ConnectiveGeometry(contour)`, then `AddLeg()` for each additional leg
- Chamfers at contour points create curved bends between legs
- `FacetedBrep` defines custom geometry as vertices + face loops
- `ShapeItem` stores custom shapes in the Tekla shape catalog
- `Polymesh.Validate()` checks geometry integrity before catalog insertion

## Bent Plate — Simple Two-Leg

```csharp
using Tekla.Structures.Model;
using Tekla.Structures.Geometry3d;

var model = new Model();

// Define first leg contour
var firstContour = new Contour();
firstContour.AddContourPoint(new ContourPoint(new Point(0, 0, 0), null));
firstContour.AddContourPoint(new ContourPoint(new Point(500, 0, 0), null));
firstContour.AddContourPoint(new ContourPoint(new Point(500, 300, 0), null));
firstContour.AddContourPoint(new ContourPoint(new Point(0, 300, 0), null));

// Create initial geometry from first contour
var geometry = new ConnectiveGeometry(firstContour);

// Define second leg contour (extends from an edge of the first)
var secondContour = new Contour();
secondContour.AddContourPoint(new ContourPoint(new Point(500, 0, 0), null));
secondContour.AddContourPoint(new ContourPoint(new Point(500, 0, 400), null));
secondContour.AddContourPoint(new ContourPoint(new Point(500, 300, 400), null));
secondContour.AddContourPoint(new ContourPoint(new Point(500, 300, 0), null));

// Add second leg with bend radius
var solver = new BentPlateGeometrySolver();
geometry = solver.AddLeg(geometry, secondContour, 20.0);  // 20mm bend radius

// Create the bent plate
var bentPlate = new BentPlate();
bentPlate.Name = "BP";
bentPlate.Profile.ProfileString = "PL10";        // plate thickness
bentPlate.Material.MaterialString = "S355JR";
bentPlate.Geometry = geometry;
bentPlate.Insert();
model.CommitChanges();
```

## Bent Plate — Multi-Leg with Chamfers

```csharp
var model = new Model();

// First leg with chamfer at bend point
var contour1 = new Contour();
contour1.AddContourPoint(new ContourPoint(new Point(0, 0, 0), null));
contour1.AddContourPoint(new ContourPoint(
    new Point(300, 0, 0),
    new Chamfer(0, 0, Chamfer.ChamferTypeEnum.CHAMFER_ARC_POINT)));
contour1.AddContourPoint(new ContourPoint(
    new Point(300, 200, 0),
    new Chamfer(0, 0, Chamfer.ChamferTypeEnum.CHAMFER_ARC_POINT)));
contour1.AddContourPoint(new ContourPoint(new Point(0, 200, 0), null));

var geometry = new ConnectiveGeometry(contour1);
var solver = new BentPlateGeometrySolver();

// Add legs with increasing bend radii
var contour2 = new Contour();
contour2.AddContourPoint(new ContourPoint(new Point(300, 0, 0), null));
contour2.AddContourPoint(new ContourPoint(new Point(300, 0, 250), null));
contour2.AddContourPoint(new ContourPoint(new Point(300, 200, 250), null));
contour2.AddContourPoint(new ContourPoint(new Point(300, 200, 0), null));
geometry = solver.AddLeg(geometry, contour2, 15.0);

var contour3 = new Contour();
contour3.AddContourPoint(new ContourPoint(new Point(300, 0, 250), null));
contour3.AddContourPoint(new ContourPoint(new Point(600, 0, 250), null));
contour3.AddContourPoint(new ContourPoint(new Point(600, 200, 250), null));
contour3.AddContourPoint(new ContourPoint(new Point(300, 200, 250), null));
geometry = solver.AddLeg(geometry, contour3, 15.0);

var bentPlate = new BentPlate();
bentPlate.Name = "BP";
bentPlate.Profile.ProfileString = "PL12";
bentPlate.Material.MaterialString = "S355JR";
bentPlate.Geometry = geometry;
bentPlate.Insert();
model.CommitChanges();
```

## FacetedBrep — Custom 3D Geometry

Create custom geometry from vertices and face definitions.

```csharp
using Tekla.Structures.Catalogs;
using Tekla.Structures.Geometry3d;

// Define a cube (8 vertices, 6 faces)
var vertices = new Vector[]
{
    new Vector(0, 0, 0),       // 0: origin
    new Vector(100, 0, 0),     // 1: +X
    new Vector(100, 100, 0),   // 2: +X+Y
    new Vector(0, 100, 0),     // 3: +Y
    new Vector(0, 0, 100),     // 4: +Z
    new Vector(100, 0, 100),   // 5: +X+Z
    new Vector(100, 100, 100), // 6: +X+Y+Z
    new Vector(0, 100, 100)    // 7: +Y+Z
};

// Face definitions: each face is an array of vertex indices
// Outward-facing normal determined by winding order (right-hand rule)
var outerLoops = new int[][]
{
    new[] { 3, 2, 1, 0 },  // bottom face
    new[] { 4, 5, 6, 7 },  // top face
    new[] { 0, 1, 5, 4 },  // front face
    new[] { 2, 3, 7, 6 },  // back face
    new[] { 0, 4, 7, 3 },  // left face
    new[] { 1, 2, 6, 5 }   // right face
};

// Inner loops (holes in faces) — empty for solid shapes
var innerLoops = new Dictionary<int, int[][]>();

var brep = new FacetedBrep(vertices, outerLoops, innerLoops);
```

## FacetedBrep — Pyramid Shape

```csharp
// 5-vertex pyramid (4 base + 1 apex)
var vertices = new Vector[]
{
    new Vector(0, 0, 0),       // 0: base corner
    new Vector(200, 0, 0),     // 1: base corner
    new Vector(200, 200, 0),   // 2: base corner
    new Vector(0, 200, 0),     // 3: base corner
    new Vector(100, 100, 300)  // 4: apex
};

var outerLoops = new int[][]
{
    new[] { 3, 2, 1, 0 },  // base (bottom)
    new[] { 0, 1, 4 },     // front triangle
    new[] { 1, 2, 4 },     // right triangle
    new[] { 2, 3, 4 },     // back triangle
    new[] { 3, 0, 4 }      // left triangle
};

var innerLoops = new Dictionary<int, int[][]>();
var brep = new FacetedBrep(vertices, outerLoops, innerLoops);
```

## ShapeItem — Add Custom Shape to Catalog

```csharp
using Tekla.Structures.Catalogs;

// Create FacetedBrep geometry first (see above)
var brep = new FacetedBrep(vertices, outerLoops, innerLoops);

// Validate the geometry
var result = Polymesh.Validate(
    Polymesh.PolymeshCheckerFlags.All, brep);
// result should be PolymeshHealthCheckEnum.Ok

// Insert into shape catalog
var shapeItem = new ShapeItem();
shapeItem.Name = "MyCustomShape";
shapeItem.ShapeFacetedBrep = brep;
shapeItem.UpAxis = ShapeItem.ShapeUpAxis.Z_Axis;
shapeItem.Insert();
```

## Create Part from Custom Shape

```csharp
// Use the catalog shape with a Brep part
var brep = new Brep();
brep.StartPoint = new Point(0, 0, 0);
brep.EndPoint = new Point(1000, 0, 0);
brep.Profile.ProfileString = "MyCustomShape";  // shape catalog name
brep.Material.MaterialString = "S355JR";
brep.Insert();
model.CommitChanges();
```

## ContourPlate with Chamfers

```csharp
var model = new Model();
var plate = new ContourPlate();

plate.AddContourPoint(new ContourPoint(new Point(0, 0, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(500, 0, 0), null));
plate.AddContourPoint(new ContourPoint(
    new Point(500, 400, 0),
    new Chamfer(30, 0, Chamfer.ChamferTypeEnum.CHAMFER_LINE)));
plate.AddContourPoint(new ContourPoint(new Point(0, 400, 0), null));

plate.Profile.ProfileString = "PL20";
plate.Material.MaterialString = "S355JR";
plate.Class = "2";
plate.Name = "PLATE";

plate.Position.Depth = Position.DepthEnum.MIDDLE;
plate.Insert();
model.CommitChanges();
```

## Chamfer.ChamferTypeEnum

| Value | Description |
|-------|-------------|
| `CHAMFER_NONE` | No chamfer |
| `CHAMFER_LINE` | Straight chamfer (cut corner) |
| `CHAMFER_ROUNDING` | Rounded corner |
| `CHAMFER_ARC_POINT` | Arc through point |
| `CHAMFER_ARC` | Arc with radius |
| `CHAMFER_LINE_AND_ARC` | Combined line and arc |
