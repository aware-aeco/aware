---
name: tekla-connections-bolts-welds
description: Tekla Open API - Bolt arrays, welds, cut planes, fittings, boolean parts. Create bolt groups with size, standard, spacing. Create welds between parts. Cut parts with planes and boolean operations. BoltArray, BoltGroup, Weld, CutPlane, Fitting, BooleanPart. This skill should be used when creating bolts, welds, cuts, fittings, or boolean operations on model objects.
---

# Tekla Connections, Bolts & Welds (v2025)

## Critical Patterns

- `BoltArray` needs `PartToBeBolted` and `PartToBoltTo` set before `Insert()`
- Bolt positions defined by `FirstPosition` and `SecondPosition` (coordinate system)
- Bolt spacing added via `AddBoltDistX()` and `AddBoltDistY()`
- `Weld` connects `MainObject` to `SecondaryObject`
- `CutPlane`/`Fitting` modify parts by removing material along a plane
- `BooleanPart` uses one part to cut another (one is consumed)

## BoltArray — Full Creation Pattern

```csharp
using Tekla.Structures.Model;
using Tekla.Structures.Geometry3d;

var model = new Model();

var boltArray = new BoltArray();

// Parts to connect
boltArray.PartToBeBolted = plate;     // part with holes
boltArray.PartToBoltTo = beam;         // part bolted to
boltArray.AddOtherPartToBolt(washerPlate); // optional additional parts

// Bolt position (defines bolt group origin and direction)
boltArray.FirstPosition = new Point(1000, 0, 200);
boltArray.SecondPosition = new Point(1000, 0, 500);

// Bolt properties
boltArray.BoltSize = 20;               // M20
boltArray.BoltStandard = "7990";       // ISO standard
boltArray.BoltType = BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE; // site bolt
boltArray.Tolerance = 2.0;             // hole tolerance in mm
boltArray.Length = 100;                 // bolt length
boltArray.ExtraLength = 15;            // extra length
boltArray.ThreadInMaterial = BoltGroup.BoltThreadInMaterialEnum.THREAD_IN_MATERIAL_YES;
boltArray.CutLength = 105;

// Hole properties
boltArray.HoleType = BoltGroup.BoltHoleTypeEnum.HOLE_TYPE_ROUND;
boltArray.SlottedHoleX = 0;            // slotted hole width (0 = round)
boltArray.SlottedHoleY = 0;
boltArray.RotateSlots = BoltGroup.BoltRotateSlotsEnum.ROTATE_SLOTS_ODD;

// Bolt spacing — distances between bolts in X direction
boltArray.AddBoltDistX(0);             // first bolt at origin
boltArray.AddBoltDistX(70);            // 70mm to next bolt
boltArray.AddBoltDistX(70);            // 70mm to next bolt

// Bolt spacing — distances between rows in Y direction
boltArray.AddBoltDistY(0);             // first row
boltArray.AddBoltDistY(60);            // 60mm to second row

// Edge distances
boltArray.StartPointOffset.Dx = 40;
boltArray.StartPointOffset.Dy = 40;
boltArray.EndPointOffset.Dx = 40;
boltArray.EndPointOffset.Dy = 40;

// Washer and nut configuration
boltArray.Washer1 = true;              // washer on bolt side
boltArray.Washer2 = true;              // washer on nut side
boltArray.Washer3 = false;
boltArray.Nut1 = true;
boltArray.Nut2 = false;
boltArray.Bolt = true;

boltArray.Insert();
model.CommitChanges();
```

## BoltGroup Enums

### BoltTypeEnum

| Value | Description |
|-------|-------------|
| `BOLT_TYPE_WORKSHOP` | Workshop (shop) bolt |
| `BOLT_TYPE_SITE` | Site (field) bolt |

### BoltHoleTypeEnum

| Value | Description |
|-------|-------------|
| `HOLE_TYPE_ROUND` | Standard round hole |
| `HOLE_TYPE_SLOTTED` | Slotted hole |
| `HOLE_TYPE_OVERSIZED` | Oversized hole |
| `HOLE_TYPE_NO_HOLE` | No hole (stud bolt) |

## Weld Creation

```csharp
var model = new Model();

var weld = new Weld();
weld.MainObject = primaryPart;        // first part
weld.SecondaryObject = secondaryPart;  // second part

// Weld type
weld.TypeAbove = Weld.WeldTypeEnum.WELD_TYPE_FILLET;
weld.TypeBelow = Weld.WeldTypeEnum.WELD_TYPE_NONE;

// Weld properties
weld.SizeAbove = 6.0;                  // weld size in mm
weld.ShopWeld = true;                  // shop weld (false = site)
weld.AroundWeld = false;               // continuous weld around

// Contour
weld.ContourAbove = Weld.WeldContourEnum.WELD_CONTOUR_NONE;

weld.Insert();
model.CommitChanges();
```

### WeldTypeEnum (Common Values)

| Value | Description |
|-------|-------------|
| `WELD_TYPE_NONE` | No weld |
| `WELD_TYPE_FILLET` | Fillet weld |
| `WELD_TYPE_PLUGIN` | Plug weld |
| `WELD_TYPE_SQUARE_GROOVE_SQUARE_BUTT` | Square groove butt |
| `WELD_TYPE_SINGLE_V_BUTT` | Single V butt |
| `WELD_TYPE_SINGLE_BEVEL_BUTT` | Single bevel butt |

## CutPlane — Cut Part with a Plane

```csharp
var model = new Model();

// Define the cutting plane
var cutPlane = new CutPlane();
cutPlane.Father = beamToCut;    // part to be cut

// Plane definition (origin + X/Y axes define the cutting plane)
cutPlane.Plane = new Plane
{
    Origin = new Point(1500, 0, 0),
    AxisX = new Vector(0, 1, 0),    // plane X direction
    AxisY = new Vector(0, 0, 1)     // plane Y direction
};

cutPlane.Insert();
model.CommitChanges();
```

## Fitting — Modify Part End

```csharp
var model = new Model();

// Fitting adjusts a part end to meet a plane
var fitting = new Fitting();
fitting.Father = beam;           // part to modify

// The fitting plane
fitting.Plane = new Plane
{
    Origin = new Point(2800, 0, 100),
    AxisX = new Vector(0, 1, 0),
    AxisY = new Vector(0, 0, 1)
};

fitting.Insert();
model.CommitChanges();
```

## BooleanPart — Boolean Cut/Add

```csharp
var model = new Model();

// Create a cutting part (e.g., a plate used as cutter)
var cuttingPart = new ContourPlate();
// ... define cutter geometry
cuttingPart.Insert();

// Boolean operation: subtract cuttingPart from targetPart
var booleanPart = new BooleanPart();
booleanPart.Father = targetPart;     // part to be modified
booleanPart.SetOperativePart(cuttingPart);  // part used as tool
booleanPart.Type = BooleanPart.BooleanTypeEnum.BOOLEAN_CUT;

booleanPart.Insert();
model.CommitChanges();
// Note: cuttingPart is consumed by the boolean operation
```

### BooleanPart.BooleanTypeEnum

| Value | Description |
|-------|-------------|
| `BOOLEAN_CUT` | Subtract operative part from father |
| `BOOLEAN_ADD` | Add operative part to father |

## Connection with Bolts — Complete Pattern

```csharp
// Common pattern: end plate connection with bolts
var model = new Model();

// 1. Create end plate
var plate = new ContourPlate();
plate.AddContourPoint(new ContourPoint(new Point(0, -150, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(0, 150, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(0, 150, 400), null));
plate.AddContourPoint(new ContourPoint(new Point(0, -150, 400), null));
plate.Profile.ProfileString = "PL15";
plate.Material.MaterialString = "S355JR";
plate.Insert();

// 2. Weld plate to beam
var weld = new Weld();
weld.MainObject = beam;
weld.SecondaryObject = plate;
weld.TypeAbove = Weld.WeldTypeEnum.WELD_TYPE_FILLET;
weld.SizeAbove = 8.0;
weld.ShopWeld = true;
weld.Insert();

// 3. Bolt plate to column
var bolts = new BoltArray();
bolts.PartToBeBolted = plate;
bolts.PartToBoltTo = column;
bolts.FirstPosition = new Point(0, -80, 100);
bolts.SecondPosition = new Point(0, -80, 300);
bolts.BoltSize = 20;
bolts.BoltStandard = "7990";
bolts.BoltType = BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE;
bolts.AddBoltDistX(0);
bolts.AddBoltDistX(160);  // spacing between columns of bolts
bolts.AddBoltDistY(0);
bolts.AddBoltDistY(200);  // spacing between rows
bolts.Tolerance = 2.0;
bolts.Insert();

model.CommitChanges();
```
