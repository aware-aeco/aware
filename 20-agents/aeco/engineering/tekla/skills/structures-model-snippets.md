---
parent: tekla-structures-model
description: Reusable code snippets for Tekla model operations. Enumerate model objects, create beams, create contour plates, read user properties, set transformation plane, iterate assemblies, query by filter, position enums, component input retrieval.
---

## Reusable Code Snippets

### Enumerate All Model Objects by Type

```csharp
var model = new Model();
var selector = model.GetModelObjectSelector();
var enumerator = selector.GetAllObjectsWithType(
    ModelObject.ModelObjectEnum.BEAM);

var beams = new List<Dictionary<string, object>>();
while (enumerator.MoveNext())
{
    if (enumerator.Current is Beam beam)
    {
        beam.Select();
        beams.Add(new Dictionary<string, object>
        {
            ["id"] = beam.Identifier.ID,
            ["profile"] = beam.Profile.ProfileString,
            ["material"] = beam.Material.MaterialString,
            ["class"] = beam.Class,
            ["name"] = beam.Name
        });
    }
}
```

### Create a Beam

```csharp
var model = new Model();
var beam = new Beam
{
    StartPoint = new Point(0, 0, 0),
    EndPoint = new Point(6000, 0, 0),
    Profile = new Profile { ProfileString = "HEA300" },
    Material = new Material { MaterialString = "S355JR" },
    Class = "1",
    Name = "BEAM"
};
beam.Position.Depth = Position.DepthEnum.MIDDLE;
beam.Position.Plane = Position.PlaneEnum.MIDDLE;
beam.Position.Rotation = Position.RotationEnum.TOP;
beam.Insert();
model.CommitChanges();
```

### Read and Write User Properties (UDAs)

```csharp
// Read string UDA
string udaValue = "";
obj.GetUserProperty("MY_UDA", ref udaValue);

// Read double UDA
double dblValue = 0.0;
obj.GetUserProperty("MY_DOUBLE_UDA", ref dblValue);

// Read int UDA
int intValue = 0;
obj.GetUserProperty("MY_INT_UDA", ref intValue);

// Write UDA (no Modify() or CommitChanges() needed)
obj.SetUserProperty("MY_UDA", "new value");
obj.SetUserProperty("MY_DOUBLE_UDA", 42.5);
```

### Global Coordinates Pattern

```csharp
var model = new Model();
var workPlaneHandler = model.GetWorkPlaneHandler();
var originalPlane = workPlaneHandler.GetCurrentTransformationPlane();

// Switch to global coordinates
workPlaneHandler.SetCurrentTransformationPlane(new TransformationPlane());

try
{
    // All coordinates are now in global space
    beam.Select();
    var startPoint = beam.StartPoint;
    var endPoint = beam.EndPoint;
}
finally
{
    // Always restore original plane
    workPlaneHandler.SetCurrentTransformationPlane(originalPlane);
}
```

### Query Objects with Filter

Requires: `using Tekla.Structures.Filtering;` and `using Tekla.Structures.Filtering.Categories;`

```csharp
var model = new Model();
var selector = model.GetModelObjectSelector();

// Simple filter: parts with profile starting with "HEA"
var filter = new Filter(
    new BinaryFilterExpression(
        new PartFilterExpressions.Profile(),
        StringOperatorType.STARTS_WITH,
        new StringConstantFilterExpression("HEA")));

var enumerator = selector.GetObjectsByFilter(filter.FilterExpression);
```

Multi-condition filter (e.g., profile AND material):

```csharp
var expressions = new BinaryFilterExpressionCollection();

expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.Profile(),
        StringOperatorType.STARTS_WITH,
        new StringConstantFilterExpression("HEA")),
    BinaryFilterOperatorType.BOOLEAN_AND));

expressions.Add(new BinaryFilterExpressionItem(
    new BinaryFilterExpression(
        new PartFilterExpressions.Material(),
        StringOperatorType.IS_EQUAL,
        new StringConstantFilterExpression("S355JR"))));

var filter = new Filter(expressions);
var enumerator = selector.GetObjectsByFilter(filter.FilterExpression);
```

> **Tip:** If you only need to filter by object type (e.g., all beams), use `selector.GetAllObjectsWithType(ModelObject.ModelObjectEnum.BEAM)` instead.

### Report Property Read (Single Object)

```csharp
// Use ModelObject.GetReportProperty (overloaded by type: string, double, int)
beam.Select();

string weight = "";
beam.GetReportProperty("WEIGHT", ref weight);

double length = 0.0;
beam.GetReportProperty("LENGTH", ref length);

double area = 0.0;
beam.GetReportProperty("AREA", ref area);

double volume = 0.0;
beam.GetReportProperty("VOLUME", ref volume);

// Common report property names:
// WEIGHT, LENGTH, AREA, VOLUME, WIDTH, HEIGHT
// PROFILE.WIDTH, PROFILE.HEIGHT, PROFILE.FLANGE_THICKNESS
// ASSEMBLY.WEIGHT, ASSEMBLY_POS, PART_POS
```

### Batch Report Property Read (Multiple Objects)

```csharp
// Use Model.GetReportPropertyStr/Double/Int with IList<Identifier>
var model = new Model();
var ids = new List<Identifier> { beam1.Identifier, beam2.Identifier };

List<string> weights = model.GetReportPropertyStr(ids, "WEIGHT");
List<double> lengths = model.GetReportPropertyDouble(ids, "LENGTH");
List<int> phases = model.GetReportPropertyInt(ids, "PHASE");
```

### Bulk UDA Set (Multiple Properties)

```csharp
// Set multiple user properties at once using Hashtable
var properties = new Hashtable
{
    { "USER_FIELD_1", "Foundation" },
    { "USER_FIELD_2", "Zone A" },
    { "COMMENT", "Reviewed 2025-01" }
};
obj.SetUserProperties(properties);
// No Modify() or CommitChanges() needed for UDAs
```

### Programmatic Object Selection (UI)

```csharp
// Set Tekla UI selection programmatically
var objectsToSelect = new ArrayList();
while (enumerator.MoveNext())
{
    objectsToSelect.Add(enumerator.Current);
}

var modelObjectSelector = new Tekla.Structures.Model.UI.ModelObjectSelector();
modelObjectSelector.Select(objectsToSelect);
// Objects are now selected in Tekla UI
```

### Assembly Enumeration

```csharp
var model = new Model();
var selector = model.GetModelObjectSelector();
var enumerator = selector.GetAllObjectsWithType(
    ModelObject.ModelObjectEnum.ASSEMBLY);

while (enumerator.MoveNext())
{
    if (enumerator.Current is Assembly assembly)
    {
        assembly.Select();

        // Get main (primary) part
        var mainPart = assembly.GetMainPart() as Part;
        mainPart?.Select();

        // Get secondary parts
        var secondaries = assembly.GetSecondaries();
        foreach (Part secondary in secondaries)
        {
            secondary.Select();
            // Process secondary part
        }
    }
}
```

### Connection/Component Creation

```csharp
var model = new Model();

// Create a connection between two parts
var connection = new Connection();
connection.Name = "End plate (11)";
connection.Number = 11;

// Primary part (the part the connection attaches to)
connection.SetPrimaryObject(primaryBeam);

// Secondary parts (parts that connect)
connection.SetSecondaryObject(secondaryBeam);

// Set connection attributes
connection.SetAttribute("cut1", 1);
connection.SetAttribute("d1", 20.0);

connection.Insert();
model.CommitChanges();
```

### Move/Copy Objects

```csharp
var model = new Model();
beam.Select();

// Move object by a vector
var moveVector = new Vector(3000, 0, 0);
Operation.MoveObject(beam, moveVector);
model.CommitChanges();

// Copy object with translation
var copyVector = new Vector(0, 6000, 0);
var copiedId = Operation.CopyObject(beam, copyVector);
model.CommitChanges();
```

### Create ContourPlate

```csharp
var model = new Model();
var plate = new ContourPlate();

plate.AddContourPoint(new ContourPoint(new Point(0, 0, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(500, 0, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(500, 400, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(0, 400, 0), null));

plate.Profile.ProfileString = "PL20";     // 20mm thick
plate.Material.MaterialString = "S355JR";
plate.Class = "2";
plate.Name = "PLATE";
plate.Position.Depth = Position.DepthEnum.MIDDLE;

plate.Insert();
model.CommitChanges();
```

### Position Enum Quick Reference

```csharp
// Position controls part placement relative to its reference line/plane
beam.Position.Depth = Position.DepthEnum.MIDDLE;   // FRONT, MIDDLE, BACK
beam.Position.Plane = Position.PlaneEnum.MIDDLE;   // LEFT, MIDDLE, RIGHT
beam.Position.Rotation = Position.RotationEnum.TOP; // FRONT, TOP, BACK, BELOW

// Offsets fine-tune placement (in mm)
beam.Position.DepthOffset = 50.0;
beam.Position.PlaneOffset = 25.0;
beam.Position.RotationOffset = 0.0;
```

### Get Component Input Data

```csharp
// Read input data from an existing parametric component
var model = new Model();

if (modelObject is Component component)
{
    component.Select();
    var input = component.GetComponentInput();

    if (input is ComponentInput componentInput)
    {
        // Iterate input items
        foreach (InputItem item in componentInput)
        {
            var inputType = item.GetInputType();

            switch (inputType)
            {
                case InputItem.InputTypeEnum.INPUT_1_OBJECT:
                    var obj = item.GetData() as Identifier;
                    break;
                case InputItem.InputTypeEnum.INPUT_N_OBJECTS:
                    var objects = item.GetData() as ArrayList;
                    break;
                case InputItem.InputTypeEnum.INPUT_1_POINT:
                    var point = item.GetData() as Point;
                    break;
                case InputItem.InputTypeEnum.INPUT_2_POINTS:
                    var points = item.GetData() as ArrayList;
                    break;
                case InputItem.InputTypeEnum.INPUT_POLYGON:
                    var polygon = item.GetData() as ArrayList;
                    break;
            }
        }
    }
}
```

### Solid Geometry Traversal

```csharp
// Get solid geometry of a part (faces, vertices)
var solid = part.GetSolid();

// Bounding box
var minPoint = solid.MinimumPoint;
var maxPoint = solid.MaximumPoint;

// Iterate faces
var faces = solid.GetFaceEnumerator();
while (faces.MoveNext())
{
    var face = faces.Current as Face;
    var normal = face.Normal;

    // Iterate vertices of the face
    var loops = face.GetLoopEnumerator();
    while (loops.MoveNext())
    {
        var loop = loops.Current as Loop;
        var vertices = loop.GetVertexEnumerator();
        while (vertices.MoveNext())
        {
            var point = vertices.Current;
        }
    }
}
```
