---
name: tekla-reinforcement-patterns
description: Tekla Open API - Reinforcement creation patterns. SingleRebar, RebarGroup, RebarSet, stirrups, hooks, offsets, spacings. Create longitudinal bars, stirrups, rebar groups with distribution patterns. RebarHookData for hook shapes. This skill should be used when creating reinforcement, rebar, stirrups, hooks, or bar placement in concrete parts.
---

# Tekla Reinforcement Patterns (v2025)

## Critical Patterns

- Reinforcement `Father` must be set to the concrete part that contains the rebar
- Always call `rebar.Insert()` then `model.CommitChanges()`
- Work in the father part's coordinate system for placement
- Use `TransformationPlane` to position rebar relative to the part
- `RebarHookData` configures hook geometry at bar ends
- `RebarGroup` uses `Spacings` and `SpacingType` for distribution

## SingleRebar — Individual Bar

```csharp
using Tekla.Structures.Model;
using Tekla.Structures.Geometry3d;

var model = new Model();

// Father = the concrete part that contains this rebar
var rebar = new SingleRebar();
rebar.Father = concretePart;
rebar.Name = "Longitudinal";
rebar.Size = "20";           // Bar diameter in mm
rebar.Grade = "A500HW";      // Steel grade
rebar.Class = 3;             // Visibility class
rebar.NumberingSeries.Prefix = "R";
rebar.NumberingSeries.StartNumber = 1;

// Define bar polygon (path of the bar)
rebar.Polygon.Points.Add(new Point(0, 0, 50));
rebar.Polygon.Points.Add(new Point(3000, 0, 50));

// Cover thickness offsets
rebar.StartPointOffsetType = Reinforcement.RebarOffsetTypeEnum.OFFSET_TYPE_COVER_THICKNESS;
rebar.StartPointOffsetValue = 25.0;
rebar.EndPointOffsetType = Reinforcement.RebarOffsetTypeEnum.OFFSET_TYPE_COVER_THICKNESS;
rebar.EndPointOffsetValue = 25.0;

// Plane offsets (distance from reference plane)
rebar.OnPlaneOffsets.Add(50.0);   // offset from part face
rebar.FromPlaneOffset = 30.0;    // offset from start

// Hooks at bar ends
rebar.StartHook.Shape = RebarHookData.RebarHookShapeEnum.HOOK_90_DEGREES;
rebar.StartHook.Length = 150.0;
rebar.StartHook.Radius = 40.0;
rebar.StartHook.Angle = 90.0;

rebar.EndHook.Shape = RebarHookData.RebarHookShapeEnum.NO_HOOK;

rebar.Insert();
model.CommitChanges();
```

## RebarGroup — Multiple Bars with Spacing

```csharp
var model = new Model();
var rebarGroup = new RebarGroup();
rebarGroup.Father = concretePart;
rebarGroup.Name = "Bottom bars";
rebarGroup.Size = "16";
rebarGroup.Grade = "A500HW";
rebarGroup.Class = 4;
rebarGroup.NumberingSeries.Prefix = "R";
rebarGroup.NumberingSeries.StartNumber = 1;

// Start polygon (first bar position)
var startPolygon = new Polygon();
startPolygon.Points.Add(new Point(0, 0, 50));
startPolygon.Points.Add(new Point(3000, 0, 50));
rebarGroup.Polygons.Add(startPolygon);

// End polygon (last bar position)
var endPolygon = new Polygon();
endPolygon.Points.Add(new Point(0, 500, 50));
endPolygon.Points.Add(new Point(3000, 500, 50));
rebarGroup.Polygons.Add(endPolygon);

// Spacing
rebarGroup.Spacings.Add(150.0);  // 150mm spacing between bars
rebarGroup.SpacingType = BaseRebarGroup.RebarGroupSpacingTypeEnum.SPACING_TYPE_EXACT_SPACINGS;

// Offsets
rebarGroup.StartPointOffsetType = Reinforcement.RebarOffsetTypeEnum.OFFSET_TYPE_COVER_THICKNESS;
rebarGroup.StartPointOffsetValue = 25.0;
rebarGroup.EndPointOffsetType = Reinforcement.RebarOffsetTypeEnum.OFFSET_TYPE_COVER_THICKNESS;
rebarGroup.EndPointOffsetValue = 25.0;
rebarGroup.OnPlaneOffsets.Add(30.0);

// Hooks
rebarGroup.StartHook.Shape = RebarHookData.RebarHookShapeEnum.HOOK_90_DEGREES;
rebarGroup.StartHook.Length = 120.0;
rebarGroup.StartHook.Radius = 32.0;

rebarGroup.Insert();
model.CommitChanges();
```

## Stirrups (Closed Loop RebarGroup)

```csharp
var model = new Model();
var stirrup = new RebarGroup();
stirrup.Father = concretePart;
stirrup.Name = "Stirrup";
stirrup.Size = "10";
stirrup.Grade = "A500HW";
stirrup.Class = 5;

// Stirrup polygon defines the closed loop shape
// Start polygon (first stirrup)
var startPoly = new Polygon();
startPoly.Points.Add(new Point(100, -150, 50));
startPoly.Points.Add(new Point(100, 150, 50));
startPoly.Points.Add(new Point(100, 150, 350));
startPoly.Points.Add(new Point(100, -150, 350));
startPoly.Points.Add(new Point(100, -150, 50));  // close the loop
stirrup.Polygons.Add(startPoly);

// End polygon (last stirrup along beam length)
var endPoly = new Polygon();
endPoly.Points.Add(new Point(2900, -150, 50));
endPoly.Points.Add(new Point(2900, 150, 50));
endPoly.Points.Add(new Point(2900, 150, 350));
endPoly.Points.Add(new Point(2900, -150, 350));
endPoly.Points.Add(new Point(2900, -150, 50));
stirrup.Polygons.Add(endPoly);

// Distribute along beam with 200mm spacing
stirrup.Spacings.Add(200.0);
stirrup.SpacingType = BaseRebarGroup.RebarGroupSpacingTypeEnum.SPACING_TYPE_EXACT_SPACINGS;

// Hooks for stirrup closure
stirrup.StartHook.Shape = RebarHookData.RebarHookShapeEnum.HOOK_135_DEGREES;
stirrup.StartHook.Length = 100.0;
stirrup.StartHook.Radius = 20.0;
stirrup.StartHook.Angle = 135.0;
stirrup.EndHook.Shape = RebarHookData.RebarHookShapeEnum.HOOK_135_DEGREES;
stirrup.EndHook.Length = 100.0;

stirrup.Insert();
model.CommitChanges();
```

## RebarHookData.RebarHookShapeEnum

| Value | Description |
|-------|-------------|
| `NO_HOOK` | No hook |
| `HOOK_90_DEGREES` | 90-degree hook |
| `HOOK_135_DEGREES` | 135-degree hook (seismic/stirrup) |
| `HOOK_180_DEGREES` | 180-degree U-hook |
| `CUSTOM_HOOK` | Custom angle/length |

## Reinforcement.RebarOffsetTypeEnum

| Value | Description |
|-------|-------------|
| `OFFSET_TYPE_COVER_THICKNESS` | Offset equals concrete cover thickness |
| `OFFSET_TYPE_ABSOLUTE` | Absolute offset value in mm |
| `OFFSET_TYPE_LEG_LENGTH` | Offset equals leg length |

## BaseRebarGroup.RebarGroupSpacingTypeEnum

| Value | Description |
|-------|-------------|
| `SPACING_TYPE_EXACT_SPACINGS` | Use exact spacing values |
| `SPACING_TYPE_EXACT_NUMBER` | Distribute exact number of bars |
| `SPACING_TYPE_TARGET_SPACE` | Target spacing (auto-adjust to fit) |

## Work Plane Pattern for Rebar Placement

```csharp
// Set work plane to part's coordinate system for easier rebar placement
var workPlaneHandler = model.GetWorkPlaneHandler();
var original = workPlaneHandler.GetCurrentTransformationPlane();
var partCS = concretePart.GetCoordinateSystem();
workPlaneHandler.SetCurrentTransformationPlane(new TransformationPlane(partCS));

try
{
    // Create rebar in part-local coordinates
    var rebar = new SingleRebar();
    rebar.Father = concretePart;
    rebar.Polygon.Points.Add(new Point(0, 0, 30));    // local X start
    rebar.Polygon.Points.Add(new Point(3000, 0, 30));  // local X end
    // ... set other properties
    rebar.Insert();
}
finally
{
    workPlaneHandler.SetCurrentTransformationPlane(original);
}
model.CommitChanges();
```

## Get Rebar Geometries (Actual Bar Positions)

```csharp
// After insertion, get actual rebar geometry
if (rebarGroup is RebarGroup group)
{
    var geometries = group.GetRebarGeometries(false);
    // Returns list of actual bar polygons after distribution
}
```
