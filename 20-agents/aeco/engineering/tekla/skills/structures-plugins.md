---
name: tekla-structures-plugins
description: Tekla Open API - Tekla.Structures.Plugins namespace. Plugin base classes, connection development, detail development, custom part development, input definitions, plugin attributes, PluginBase, StructuresData, plugin registration. This skill should be used when creating Tekla plugins, connections, details, or custom parts.
---

# Tekla.Structures.Plugins API Reference (v2025)

## Overview

Base classes and attributes for developing Tekla Structures plugins: connections, details, custom parts, and drawing plugins. Plugins are .NET assemblies loaded by Tekla with specific attributes and base class implementations.

## Plugin Architecture

```
PluginBase (model plugins)
  ├── Uses InputDefinition for point/object inputs
  ├── DefineInput() → returns input requirements
  └── Run(List<InputDefinition>) → creates model objects

ConnectionBase (connections/details/seams)
  ├── Gets Primary + Secondaries as identifiers
  ├── Gets Positions for details/seams
  └── Run() → creates connection objects

CustomPartBase (custom parts)
  ├── Gets Positions for placement
  └── Run() → creates custom part objects

DrawingPluginBase (drawing plugins)
  ├── Uses DrawingPluginBase.InputDefinition
  ├── DefineInput() → returns drawing input requirements
  └── Run(List<InputDefinition>) → creates drawing objects
```

## Core Base Classes

### PluginBase : MarshalByRefObject (abstract)
> Base class for model plugins (components that create/modify model objects).

**Properties:**
- `Identifier Identifier { get; }` - identifier of the plugin instance

**Abstract Methods:**
- `List<InputDefinition> DefineInput()` - define what inputs the plugin needs
- `bool Run(List<InputDefinition> Input)` - execute the plugin logic

**Helper Methods:**
- `bool IsDefaultValue(double Value)` - check if value is default
- `bool IsDefaultValue(string Value)`
- `bool IsDefaultValue(int Value)`

### PluginBase.InputDefinition : MarshalByRefObject
> Defines plugin dependency over input (points or identifiers).

**Constructors:**
- `InputDefinition(Point Point)` - single point input
- `InputDefinition(Point P1, Point P2)` - two-point input
- `InputDefinition(ArrayList _Input)` - multiple inputs
- `InputDefinition(Identifier ID)` - object identifier input

**Methods:**
- `object GetInput()` - get the input data
- `InputTypeEnum GetInputType()` - get the input type

### PluginBase.InputDefinition.InputTypeEnum (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| INPUT_ONE_POINT | 1 | One point (x,y,z) |
| INPUT_TWO_POINTS | 2 | Two points |
| INPUT_POLYGON | 3 | Multiple points |
| INPUT_ONE_OBJECT | 4 | One ID of any type |
| INPUT_N_OBJECTS | 5 | N IDs of any type |

### PluginBase.InputObjectDependency (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| NOT_DEPENDENT | 0 | No dependency on input. Plugin dialog only, no relation to created objects. CommitChanges needed. |
| DEPENDENT | 1 | Re-executes when input part changes (profile, position). Not on boolean operations. |
| GEOMETRICALLY_DEPENDENT | 2 | Re-executes when input geometry changes (e.g., fitting). |
| NOT_DEPENDENT_MODIFIABLE | 3 | No dependency but instance is modifiable. Dialog can be opened from created objects. |

### PluginBase.CoordinateSystemType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| FROM_FIRST_POINT_AND_GLOBAL | 0 | Origin from first point, x/y from global |
| FROM_FIRST_AND_SECOND_POINT | 1 | Origin from first point, x-direction from second |

### PluginBase.SymbolVisibility (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| DRAW_SYMBOL | 0 | Symbol visible in model view |
| DO_NOT_DRAW_SYMBOL | 1 | Symbol not drawn; select via created objects only |

---

### ConnectionBase : MarshalByRefObject (abstract)
> Base class for connections, details, and seams.

**Properties:**
- `string Code { get; set; }` - connection code (max 22 chars)
- `Identifier Identifier { get; }` - plugin instance identifier
- `Identifier Primary { get; }` - primary object identifier
- `List<Identifier> Secondaries { get; }` - secondary object identifiers
- `List<Point> Positions { get; }` - positional attributes (1 for detail, N for seam)

**Methods:**
- `bool Run()` - execute the connection logic
- `bool IsDefaultValue(double/string/int Value)`

### ConnectionBase.InputObjectType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| INPUTOBJECT_PART | 0 | Input is a part |
| INPUTOBJECT_CUSTOMPART | 1 | Input can be a custom part (component) |

### ConnectionBase.SecondaryType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| SECONDARYTYPE_ZERO | 0 | Zero secondaries (detail) |
| SECONDARYTYPE_ONE | 1 | One secondary |
| SECONDARYTYPE_TWO | 2 | Two secondaries |
| SECONDARYTYPE_MULTIPLE | 3 | Multiple secondaries |

### ConnectionBase.SeamInputType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| INPUT_2_POINTS | 1 | Two points input |
| INPUT_POLYGON | 2 | Any number of points (polygon) |

---

### CustomPartBase : MarshalByRefObject (abstract)
> Base class for custom parts.

**Properties:**
- `Identifier Identifier { get; }` - plugin instance identifier
- `List<Point> Positions { get; }` - positional attributes

**Methods:**
- `bool Run()` - execute the custom part logic
- `bool IsDefaultValue(double/string/int Value)`

### CustomPartBase.CustomPartInputType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| INPUT_2_POINTS | 1 | Two points input (default) |
| INPUT_1_POINT | 2 | One point input |

### CustomPartBase.CustomPartPositioningType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| POSITIONING_BY_CENTER_OF_BOUNDING_BOX | 1 | Position by total extrema center (default) |
| POSITIONING_BY_INPUTPOINTS | 2 | Position by input points and extrema |

---

### DrawingPluginBase : MarshalByRefObject (abstract)
> Base class for drawing plugins.

**Methods:**
- `List<InputDefinition> DefineInput()` - define drawing input requirements
- `bool Run(List<InputDefinition> Input)` - execute the drawing plugin
- `bool IsDefaultValue(double/string/int Value)`

### DrawingPluginBase.InputDefinition : MarshalByRefObject
> Input definition for drawing plugins.

**Constructors:**
- `InputDefinition(Identifier viewId, Point point)` - one point in a view
- `InputDefinition(Identifier viewId, Point point1, Point point2)` - two points
- `InputDefinition(Identifier viewId, Point point1, Point point2, Point point3)` - three points
- `InputDefinition(Identifier viewId, List<Point> points)` - N points
- `InputDefinition(Identifier viewId, Identifier ObjectId)` - one drawing object

**Properties:**
- `InputTypes Type { get; set; }` - input type
- `Identifier ViewId { get; set; }` - view identifier

**Methods:**
- `object GetInput()` - get the input data

### DrawingPluginBase.InputDefinition.InputTypes (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| INPUT_ONE_POINT | 1 | One point |
| INPUT_TWO_POINTS | 2 | Two points |
| INPUT_THREE_POINTS | 3 | Three points |
| INPUT_N_POINTS | 4 | Multiple points |
| INPUT_ONE_OBJECT | 5 | One drawing object (bolt, part, rebar) |

### DrawingPluginBase.UpdateMode (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| CREATE_ONLY | 0 | Never updated. No relation to plugin after creation. |
| INPUT_CHANGED | 1 | Updated when input changes (default) |
| DRAWING_OPENED | 2 | Updated on input change AND when drawing opens |

## Plugin Attributes

These C# attributes are applied to plugin classes to configure their behavior:

| Attribute | Constructor | Purpose |
|-----------|-------------|---------|
| `[Plugin("name")]` | `string name` | Register plugin name in system |
| `[PluginUserInterface("desc")]` | `string description` | Plugin dialog definition (inp format or class name) |
| `[InputObjectDependency(type)]` | `InputObjectDependency` | When plugin re-executes |
| `[InputObjectType(type)]` | `InputObjectType` | What can be selected as input |
| `[SecondaryType(type)]` | `SecondaryType` | Number of secondaries |
| `[DetailType(type)]` | `DetailTypeEnum` | Detail positioning type |
| `[PositionType(type)]` | `PositionTypeEnum` | Connection position type |
| `[AutoDirectionType(type)]` | `AutoDirectionTypeEnum` | Auto up-direction calculation |
| `[SeamInputType(type)]` | `SeamInputType` | Seam input method |
| `[PluginCoordinateSystem(type)]` | `CoordinateSystemType` | Coordinate system for point inputs |
| `[PluginSymbolVisiblity(type)]` | `SymbolVisibility` | Symbol visibility in model view |
| `[CustomPartInputType(type)]` | `CustomPartInputType` | Custom part input method |
| `[CustomPartPositioningType(type)]` | `CustomPartPositioningType` | Custom part positioning |
| `[UpdateMode(type)]` | `UpdateMode` | Drawing plugin update mode |
| `[StructuresField("attrName")]` | `string attributeName` | Map database attribute to plugin field |
| `[PluginPropertyFileLocation(suffix, subdir)]` | `string, string` | Property file location |

## Usage Pattern

```csharp
[Plugin("MyConnection")]
[PluginUserInterface("MyConnection.MainForm")]
[InputObjectDependency(PluginBase.InputObjectDependency.DEPENDENT)]
[SecondaryType(ConnectionBase.SecondaryType.SECONDARYTYPE_ONE)]
public class MyConnection : ConnectionBase
{
    [StructuresField("plate_thickness")]
    public double PlateThickness;

    public override bool Run()
    {
        var model = new Model();
        var primary = model.SelectModelObject(Primary) as Part;
        var secondary = model.SelectModelObject(Secondaries[0]) as Part;
        // Create connection objects...
        return true;
    }
}
```

## PluginBase Example — Beam Creator with Picker

```csharp
[Plugin("BeamCreator")]
[PluginUserInterface("BeamCreator.MainForm")]
[InputObjectDependency(PluginBase.InputObjectDependency.DEPENDENT)]
public class BeamCreator : PluginBase
{
    [StructuresField("profile")]
    public string ProfileStr = "HEA300";

    [StructuresField("material")]
    public string MaterialStr = "S355JR";

    public override List<InputDefinition> DefineInput()
    {
        // Pick two points from the user
        var picker = new Tekla.Structures.Model.UI.Picker();
        var points = picker.PickPoints(
            Tekla.Structures.Model.UI.Picker.PickPointEnum.PICK_TWO_POINTS,
            "Pick start and end points");

        var inputs = new List<InputDefinition>();
        inputs.Add(new InputDefinition(
            points[0] as Point, points[1] as Point));
        return inputs;
    }

    public override bool Run(List<InputDefinition> input)
    {
        var model = new Model();
        var points = input[0].GetInput() as ArrayList;
        var startPoint = points[0] as Point;
        var endPoint = points[1] as Point;

        var beam = new Beam(startPoint, endPoint);
        beam.Profile.ProfileString = IsDefaultValue(ProfileStr)
            ? "HEA300" : ProfileStr;
        beam.Material.MaterialString = IsDefaultValue(MaterialStr)
            ? "S355JR" : MaterialStr;
        beam.Insert();
        model.CommitChanges();
        return true;
    }
}
```

## ConnectionBase Example — Splice Connection with Bolts

```csharp
[Plugin("SpliceConn")]
[PluginUserInterface("SpliceConn.MainForm")]
[InputObjectDependency(PluginBase.InputObjectDependency.DEPENDENT)]
[SecondaryType(ConnectionBase.SecondaryType.SECONDARYTYPE_ONE)]
[AutoDirectionType(PluginBase.AutoDirectionTypeEnum.AUTODIR_BASIC)]
public class SpliceConn : ConnectionBase
{
    [StructuresField("plate_t")]
    public double PlateThickness = 15.0;

    [StructuresField("bolt_size")]
    public double BoltSize = 20.0;

    public override bool Run()
    {
        var model = new Model();
        var primaryPart = model.SelectModelObject(Primary) as Part;
        var secondaryPart = model.SelectModelObject(Secondaries[0]) as Part;

        if (primaryPart is null || secondaryPart is null)
            return false;

        primaryPart.Select();
        secondaryPart.Select();

        // Create splice plates and bolts between primary and secondary
        var plate = new ContourPlate();
        // ... define plate geometry at connection point
        plate.Profile.ProfileString = "PL" + PlateThickness;
        plate.Material.MaterialString = "S355JR";
        plate.Insert();

        var bolts = new BoltArray();
        bolts.PartToBeBolted = plate;
        bolts.PartToBoltTo = primaryPart;
        bolts.AddOtherPartToBolt(secondaryPart);
        bolts.BoltSize = BoltSize;
        bolts.BoltStandard = "7990";
        bolts.BoltType = BoltGroup.BoltTypeEnum.BOLT_TYPE_SITE;
        // ... set positions and spacing
        bolts.Insert();

        model.CommitChanges();
        return true;
    }
}
```

## Drawing Plugin Example

```csharp
[Plugin("MyDrawingMark")]
[InputObjectDependency(PluginBase.InputObjectDependency.NOT_DEPENDENT)]
public class MyDrawingMark : DrawingPluginBase
{
    public override List<InputDefinition> DefineInput()
    {
        var drawingHandler = new DrawingHandler();
        var picker = drawingHandler.GetPicker();

        // Pick a point in a drawing view
        var picked = picker.PickPoint("Pick location for mark");
        var viewId = picked.Item1;  // view identifier
        var point = picked.Item2;   // picked point

        return new List<InputDefinition>
        {
            new InputDefinition(viewId, point)
        };
    }

    public override bool Run(List<InputDefinition> input)
    {
        // Create drawing objects at picked location
        return true;
    }
}
```
