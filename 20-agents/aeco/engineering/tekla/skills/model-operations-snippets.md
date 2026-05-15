---
parent: tekla-model-operations
description: Code snippets for model operations. Move objects by vector, copy objects, create grids with labels, manage views, control temporary visibility, split beams, work plane transformations.
---

## Reusable Code Snippets

### Move Object by Vector

```csharp
var model = new Model();

// Select the object to move
beam.Select();

// Define movement vector (move 3000mm in X direction)
var moveVector = new Vector(3000, 0, 0);

// Move the object
Operation.MoveObject(beam, moveVector);
model.CommitChanges();
```

### Copy Object

```csharp
var model = new Model();
beam.Select();

// Copy with translation vector
var copyVector = new Vector(0, 6000, 0);
var copiedIdentifier = Operation.CopyObject(beam, copyVector);

// Access the copied object
var copiedBeam = model.SelectModelObject(copiedIdentifier) as Beam;
if (copiedBeam is null is false)
{
    copiedBeam.Select();
    // Modify the copy if needed
    copiedBeam.Class = "3";
    copiedBeam.Modify();
}
model.CommitChanges();
```

### Create Grid

```csharp
var model = new Model();
var grid = new Grid();

// X coordinates: space-separated values in mm
grid.CoordinateX = "0 3000 6000 9000 12000";
// Y coordinates
grid.CoordinateY = "0 4000 8000";
// Z coordinates (levels)
grid.CoordinateZ = "0 3500 7000";

// Labels (space-separated, same count as coordinates)
grid.LabelX = "A B C D E";
grid.LabelY = "1 2 3";
grid.LabelZ = "+0.000 +3.500 +7.000";

grid.Insert();
model.CommitChanges();
```

### Manage Views

```csharp
// Get all views
var viewHandler = new ViewHandler();
var views = viewHandler.GetAllViews();
while (views.MoveNext())
{
    var view = views.Current as View;
    if (view is null is false)
    {
        // Read view properties
        var viewName = view.Name;
    }
}
```

### Temporary Visibility

```csharp
// Show only specific objects temporarily
var visualization = new ModelObjectVisualization();

// Show only selected objects
visualization.SetTemporaryState(
    ModelObjectVisualization.TemporaryStateEnum.SHOW_ONLY_SELECTED);

// Reset to show all
visualization.SetTemporaryState(
    ModelObjectVisualization.TemporaryStateEnum.SHOW_ALL);
```

### Split Beam at Point

```csharp
var model = new Model();
beam.Select();

// Split at a point along the beam
var splitPoint = new Point(3000, 0, 0);
var success = Operation.Split(beam, splitPoint);
model.CommitChanges();
```

### Delete Multiple Objects

```csharp
var model = new Model();
var objectsToDelete = new ArrayList();

// Collect objects to delete
objectsToDelete.Add(beam1);
objectsToDelete.Add(beam2);
objectsToDelete.Add(plate1);

Operation.DeleteObjects(objectsToDelete);
model.CommitChanges();
```

### Work Plane Transformation

```csharp
var model = new Model();
var workPlaneHandler = model.GetWorkPlaneHandler();
var originalPlane = workPlaneHandler.GetCurrentTransformationPlane();

// Set to a custom work plane based on a part's coordinate system
var beam = someBeam;
beam.Select();
var coordSystem = beam.GetCoordinateSystem();
workPlaneHandler.SetCurrentTransformationPlane(
    new TransformationPlane(coordSystem));

try
{
    // Coordinates are now relative to the beam
    var localPoint = new Point(0, 0, 0); // beam's origin
}
finally
{
    // Always restore
    workPlaneHandler.SetCurrentTransformationPlane(originalPlane);
}
```

### Copy Multiple Objects in Pattern

```csharp
var model = new Model();
beam.Select();

// Create copies in a grid pattern (e.g., 3 copies spaced 6000mm in Y)
for (int i = 1; i <= 3; i++)
{
    var offset = new Vector(0, 6000 * i, 0);
    var newId = Operation.CopyObject(beam, offset);

    var copy = model.SelectModelObject(newId) as Beam;
    if (copy is null is false)
    {
        copy.Select();
        copy.Name = $"BEAM_COPY_{i}";
        copy.Modify();
    }
}
model.CommitChanges();
```
