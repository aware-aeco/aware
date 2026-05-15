---
name: tekla-structures-model
description: Tekla Open API - Tekla.Structures.Model namespace. Create, modify, delete, query parts, beams, columns, plates, assemblies, bolts, welds, reinforcement, rebar, connections, components, custom parts. Model operations, object selection, UI picking, events, user properties UDA, phase, numbering, ModelObjectEnumerator, Select, Modify, Insert, CommitChanges. This skill should be used when writing Tekla model manipulation or structural element code.
---

# Tekla.Structures.Model API Reference (v2025)

## Critical Patterns

- Always call `Select()` on a ModelObject before reading its properties
- Always check `model.GetConnectionStatus()` before using the API
- Use `ModelObjectEnumerator` with `while (e.MoveNext())`, not foreach
- Always call `obj.Modify()` after changing structural properties (Profile, Material, Position, Class, Name, etc.), `obj.Insert()` for new objects
- Use `model.CommitChanges()` to save structural changes to the database
- **UDA exception:** `SetUserProperty()` and `GetUserProperty()` do NOT require `Modify()` or `CommitChanges()` — they read/write directly to the Tekla database
- Points are relative to current TransformationPlane - set to global first for consistency
- Use `ref` parameters for GetUserProperty: `obj.GetUserProperty("UDA_NAME", ref value)`
- Use `ConfigureAwait(false)` on all awaits in non-UI service code

## Inheritance Hierarchy

```
ModelObject (abstract)
├── Part (abstract)
│   ├── Beam
│   ├── ContourPlate
│   ├── PolyBeam
│   ├── BentPlate
│   ├── LoftedPlate
│   ├── Brep
│   └── SpiralBeam
├── BaseComponent (abstract)
│   ├── Connection
│   ├── Detail
│   ├── Seam
│   ├── Component
│   └── CustomPart
├── Boolean (abstract)
│   ├── BooleanPart
│   ├── Fitting
│   ├── CutPlane
│   └── EdgeChamfer
├── Assembly
├── BoltGroup (abstract)
│   ├── BoltArray
│   ├── BoltCircle
│   └── BoltXYList
├── BaseWeld (abstract)
│   ├── Weld
│   ├── PolygonWeld
│   └── LogicalWeld
├── Reinforcement (abstract)
│   ├── BaseRebarGroup (abstract)
│   │   ├── RebarGroup
│   │   ├── CircleRebarGroup
│   │   └── CurvedRebarGroup
│   ├── RebarMesh
│   ├── SingleRebar
│   └── RebarStrand
├── RebarSet
├── BaseRebarModifier (abstract)
│   ├── RebarPropertyModifier
│   ├── RebarEndDetailModifier
│   └── RebarSplitter
├── RebarSplice
├── RebarSetAddition
├── Grid (via GridBase)
├── GridSurface (abstract) -> GridPlane, GridCylindricalSurface
├── ReferenceModel
├── ReferenceModelObject
├── Load (abstract) -> LoadPoint, LoadLine, LoadArea, LoadUniform, LoadTemperature
├── LoadGroup
├── SurfaceTreatment
├── SurfaceObject -> RebarLegSurfaceObject
├── Task
├── TaskDependency
├── TaskWorktype
├── HierarchicDefinition
├── HierarchicObject
├── PourObject
├── PourUnit
├── PourBreak
├── ControlLine
├── ControlPlane
├── ControlPoint
├── ControlCircle
├── ControlArc
├── ControlPolycurve
└── Phase (not ModelObject, but has CRUD)
```

## Common Report Properties

Use `modelObject.GetReportProperty(name, ref value)` for single objects, or `model.GetReportPropertyStr/Double/Int(identifiers, property)` for batch access. Common field names:

| Property | Type | Description |
|----------|------|-------------|
| `WEIGHT` | string/double | Part weight (kg) |
| `LENGTH` | double | Part length (mm) |
| `AREA` | double | Surface area (mm²) |
| `VOLUME` | double | Volume (mm³) |
| `WIDTH` | double | Part width (mm) |
| `HEIGHT` | double | Part height (mm) |
| `PROFILE.WIDTH` | double | Profile flange width |
| `PROFILE.HEIGHT` | double | Profile height |
| `PROFILE.FLANGE_THICKNESS` | double | Profile flange thickness |
| `PROFILE.WEB_THICKNESS` | double | Profile web thickness |
| `ASSEMBLY.WEIGHT` | string/double | Assembly total weight |
| `ASSEMBLY_POS` | string | Assembly position mark |
| `PART_POS` | string | Part position mark |

## Operation Class Quick Reference

Static methods in `Tekla.Structures.Model.Operations.Operation`:

| Method | Description |
|--------|-------------|
| `MoveObject(ModelObject, Vector)` | Move object by vector |
| `CopyObject(ModelObject, Vector)` | Copy object, returns new Identifier |
| `Split(Part, Point)` | Split beam/plate at point |
| `DeleteObjects(ArrayList)` | Delete multiple objects |
| `ShowOnlySelected()` | Temporarily show only selected |
| `RunMacro(string)` | Run a Tekla macro by filename |
| `IsMacroRunning()` | Check if macro is running |

## Grid Creation

Grids use space-separated coordinate strings and matching label strings:

```csharp
var grid = new Grid();
grid.CoordinateX = "0 3000 6000 9000";  // X positions in mm
grid.CoordinateY = "0 4000 8000";        // Y positions in mm
grid.CoordinateZ = "0 3500 7000";        // Z levels in mm
grid.LabelX = "A B C D";                 // Must match coordinate count
grid.LabelY = "1 2 3";
grid.LabelZ = "+0.000 +3.500 +7.000";
grid.Insert();
model.CommitChanges();
```

> Detailed API reference (all classes, methods, properties, constructors) is available in the supporting file `tekla-structures-model-api-reference.md`.

