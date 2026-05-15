---
name: tekla-drawing-workflows
description: Tekla Open API - Drawing workflow patterns. Create GA drawings, single part drawings, assembly drawings. Export drawings to PDF. Set mark content, dimensions, annotations, bounding boxes. Create straight, radius, angle, curved dimensions. Filter and iterate drawings. Open and close drawings with DrawingHandler. Read drawing status (up to date, ready for issue, issued, locked, frozen, approved, for checking, in progress) via direct Drawing properties. This skill should be used when automating drawing creation, export, modification, dimensions, annotation, status reporting, or syncing drawing metadata to external systems.
---

# Tekla Drawing Workflow Patterns (v2025)

## Critical Patterns

- Use `DrawingHandler` to open/close/iterate drawings
- Always call `drawing.CommitChanges()` after modifications
- Use `DrawingHandler.SetActiveDrawing(drawing, false)` for background processing (no UI)
- Close active drawing before opening another: `DrawingHandler.CloseActiveDrawing()`
- Drawing creation classes: `GADrawing`, `SinglePartDrawing`, `AssemblyDrawing`, `CastUnitDrawing`
- Mark/dimension operations require an active (open) drawing

## Create GA (General Arrangement) Drawing

```csharp
var drawingHandler = new DrawingHandler();

// Create a GA drawing with a named attributes file
var gaDrawing = new GADrawing("GA_standard", "General Arrangement 1");
gaDrawing.Insert();

// Open the drawing to add content
drawingHandler.SetActiveDrawing(gaDrawing);
var activeDrawing = drawingHandler.GetActiveDrawing();

// Close when done
drawingHandler.CloseActiveDrawing();
```

## Create Single Part Drawing

```csharp
// part must be a valid Part object
part.Select();

var singlePartDrawing = new SinglePartDrawing(part, "standard");
singlePartDrawing.Insert();
```

## Create Assembly Drawing

```csharp
// assembly must be a valid Assembly object
assembly.Select();

var assemblyDrawing = new AssemblyDrawing(assembly, "standard");
assemblyDrawing.Insert();
```

## Export Drawing to PDF

```csharp
using Tekla.Structures.Drawing;

var drawingHandler = new DrawingHandler();

// Get all drawings
var drawings = drawingHandler.GetDrawings();
while (drawings.MoveNext())
{
    if (drawings.Current is Drawing drawing)
    {
        // Open drawing (hidden for batch processing)
        drawingHandler.SetActiveDrawing(drawing, false);

        // Export using DotStartAction
        Operation.RunMacro("ExportPdfCurrentDrawing");

        drawingHandler.CloseActiveDrawing();
    }
}
```

## Iterate Views in a Drawing

```csharp
var drawingHandler = new DrawingHandler();
var activeDrawing = drawingHandler.GetActiveDrawing();
if (activeDrawing is null is false)
{
    var sheet = activeDrawing.GetSheet();
    var views = sheet.GetViews();
    while (views.MoveNext())
    {
        if (views.Current is View view)
        {
            // Access view properties
            var viewName = view.Name;

            // Get drawing objects in this view
            var drawingObjects = view.GetObjects();
            while (drawingObjects.MoveNext())
            {
                var drawingObject = drawingObjects.Current;
                // Process each drawing object
            }
        }
    }
}
```

## Filter Drawings by Type

```csharp
var drawingHandler = new DrawingHandler();
var drawings = drawingHandler.GetDrawings();

var gaDrawings = new List<GADrawing>();
var assemblyDrawings = new List<AssemblyDrawing>();

while (drawings.MoveNext())
{
    switch (drawings.Current)
    {
        case GADrawing ga:
            gaDrawings.Add(ga);
            break;
        case AssemblyDrawing asm:
            assemblyDrawings.Add(asm);
            break;
    }
}
```

## Set Mark Content

```csharp
// Requires an active/open drawing
var drawingHandler = new DrawingHandler();
var activeDrawing = drawingHandler.GetActiveDrawing();
var sheet = activeDrawing.GetSheet();

var views = sheet.GetViews();
while (views.MoveNext())
{
    if (views.Current is View view)
    {
        var drawingObjects = view.GetObjects();
        while (drawingObjects.MoveNext())
        {
            if (drawingObjects.Current is Mark mark)
            {
                // Read mark attributes
                var attributes = mark.Attributes;
                // Modify mark if needed
                mark.Modify();
            }
        }
    }
}
activeDrawing.CommitChanges();
```

## Full Drawing Automation Workflow

```csharp
var drawingHandler = new DrawingHandler();

// 1. Get all drawings
var drawings = drawingHandler.GetDrawings();
while (drawings.MoveNext())
{
    if (drawings.Current is Drawing drawing)
    {
        // 2. Open drawing (hidden)
        drawingHandler.SetActiveDrawing(drawing, false);
        var active = drawingHandler.GetActiveDrawing();

        if (active is null is false)
        {
            // 3. Iterate views and modify
            var sheet = active.GetSheet();
            var views = sheet.GetViews();
            while (views.MoveNext())
            {
                // Process views...
            }

            // 4. Commit changes
            active.CommitChanges();

            // 5. Close
            drawingHandler.CloseActiveDrawing();
        }
    }
}
```

## Drawing Status Detection (Approved / For Checking / In Progress)

Read drawing status via **direct strongly-typed properties on `Drawing`** — no ref parameters, no magic strings. The Drawing API exposes all status flags as first-class properties.

**⚠ Two traps that look reasonable but are wrong:**

1. **`drawing.GetReportProperty("IS_UP_TO_DATE", ref x)` — does not exist.** `GetReportProperty` lives on `Tekla.Structures.Model.ModelObject` (the Model API). `Tekla.Structures.Drawing.Drawing` inherits from `DatabaseObject`, a different base class that doesn't have it. Compile error: `'Drawing' does not contain a definition for 'GetReportProperty'`.

2. **`drawing.GetUserProperty("IS_UP_TO_DATE", ref x)` — compiles but silently wrong.** `GetUserProperty` exists on `DatabaseObject`, but it reads **User-Defined Attributes** (custom UDA fields you or another tool have set). `IS_UP_TO_DATE` and `IS_READY_FOR_ISSUING` are not UDAs — they're system flags. The call returns `false`, leaves your variable at `0`, no exception, and **every drawing looks "in progress"**. Use `GetUserProperty` only for reading your own UDAs (e.g., `drawing.GetUserProperty("Approved_By", ref approver)`).

### Correct pattern

```csharp
using Tekla.Structures.Drawing;

string ClassifyDrawing(Drawing drawing)
{
    // UpToDate means Tekla considers the drawing fully current
    if (drawing.UpToDateStatus == DrawingUpToDateStatus.DrawingIsUpToDate)
        return "Approved";

    // Ready for issuing is a user-set flag indicating the drawing is ready for review
    if (drawing.IsReadyForIssue)
        return "For Checking";

    return "In Progress";
}
```

### Available Drawing status properties

| Property | Type | Meaning |
|----------|------|---------|
| `UpToDateStatus` | `DrawingUpToDateStatus` enum | 15 values; `DrawingIsUpToDate` = fully current, others describe why it's stale (`PartsWereModified`, `OriginalPartDeleted`, etc.) |
| `IsReadyForIssue` | `bool` | User-set "Ready for issuing" flag |
| `IsReadyForIssueBy` | `string` | Username who flagged it |
| `IsIssued` | `bool` | Drawing has been issued |
| `IsIssuedButModified` | `bool` | Issued, then modified (needs re-issue) |
| `IsLocked` | `bool` | Locked for editing |
| `IsLockedBy` | `string` | Username holding the lock |
| `IsFrozen` | `bool` | Frozen flag |
| `IssuingDate` | `DateTime` | When issued |
| `CreationDate` / `ModificationDate` / `OutputDate` | `DateTime` | Timestamps |

### Batch: export drawing status to external system (e.g., Supabase/SQL)

```csharp
var drawingHandler = new DrawingHandler();
var drawings = drawingHandler.GetDrawings();
var records = new List<object>();

while (drawings.MoveNext())
{
    if (drawings.Current is Drawing drawing)
    {
        string type = drawing switch
        {
            GADrawing         => "GA",
            AssemblyDrawing   => "Assembly",
            SinglePartDrawing => "Single Part",
            CastUnitDrawing   => "Cast Unit",
            MultiDrawing      => "Multi",
            _                 => "Other"
        };

        string status =
            drawing.UpToDateStatus == DrawingUpToDateStatus.DrawingIsUpToDate ? "Approved" :
            drawing.IsReadyForIssue                                           ? "For Checking" :
                                                                                "In Progress";

        records.Add(new
        {
            drawing_number = drawing.Name,
            title          = $"{drawing.Title1} {drawing.Title2} {drawing.Title3}".Trim(),
            mark           = drawing.Mark,
            type           = type,
            status         = status,
            issued         = drawing.IsIssued,
            locked_by      = drawing.IsLockedBy,
            modified_at    = drawing.ModificationDate.ToString("o")
        });
    }
}
```

No `Select()` or `CommitChanges()` needed for status reads — these properties are populated on enumeration.

## Create Straight Dimensions

```csharp
using Tekla.Structures.Drawing;

// Requires an active/open drawing with a view
var drawingHandler = new DrawingHandler();
var activeDrawing = drawingHandler.GetActiveDrawing();
var sheet = activeDrawing.GetSheet();

var views = sheet.GetViews();
while (views.MoveNext())
{
    if (views.Current is View view)
    {
        // Create a straight dimension between two points
        var dimPoints = new PointList();
        dimPoints.Add(new Point(0, 0, 0));
        dimPoints.Add(new Point(3000, 0, 0));

        // Distance defines offset of dimension line from measured points
        var distance = 200.0;

        var straightDim = new StraightDimensionSet(view, dimPoints,
            new Vector(0, 1, 0),  // direction perpendicular to dimension line
            distance,
            new StraightDimensionSet.StraightDimensionSetAttributes());

        straightDim.Insert();
        activeDrawing.CommitChanges();
        break;
    }
}
```

## Create StraightDimensionSet with Handler

```csharp
// Alternative: using StraightDimensionSetHandler for more control
var handler = new StraightDimensionSetHandler();

var dimPoints = new PointList();
dimPoints.Add(new Point(0, 0, 0));
dimPoints.Add(new Point(1500, 0, 0));
dimPoints.Add(new Point(3000, 0, 0));

var attributes = new StraightDimensionSet.StraightDimensionSetAttributes();
// attributes can configure text format, arrow type, etc.

var dimensionSet = handler.CreateDimensionSet(view, dimPoints,
    new Vector(0, 1, 0), 150.0, attributes);
activeDrawing.CommitChanges();
```

## Create Radius and Angle Dimensions

```csharp
// Radius dimension (for arcs/circles)
var radiusDim = new RadiusDimension(view,
    centerPoint, arcPoint,
    200.0,  // distance offset
    new RadiusDimension.RadiusDimensionAttributes());
radiusDim.Insert();

// Angle dimension
var angleDim = new AngleDimension(view,
    centerPoint, startPoint, endPoint,
    new AngleDimension.AngleDimensionAttributes());
angleDim.Insert();

activeDrawing.CommitChanges();
```

## Create Curved Dimensions

```csharp
// Curved dimension set (along an arc)
var curvedHandler = new CurvedDimensionSetHandler();

// Orthogonal curved dimension
curvedHandler.CreateCurvedDimensionSetOrthogonal(
    view, arcCenterPoint, arcStartPoint, arcEndPoint,
    200.0,  // distance
    new CurvedDimensionSet.CurvedDimensionSetAttributes());

// Radial curved dimension
curvedHandler.CreateCurvedDimensionSetRadial(
    view, arcCenterPoint, arcStartPoint, arcEndPoint,
    200.0,
    new CurvedDimensionSet.CurvedDimensionSetAttributes());

activeDrawing.CommitChanges();
```

## Drawing Object Bounding Boxes

```csharp
// Axis-aligned bounding box
if (drawingObject is IAxisAlignedBoundingBox aabbProvider)
{
    var aabb = aabbProvider.GetAxisAlignedBoundingBox();
    // aabb has MinPoint and MaxPoint
}

// Object-aligned bounding box (rotated with the object)
if (drawingObject is IObjectAlignedBoundingBox oabbProvider)
{
    var oabb = oabbProvider.GetObjectAlignedBoundingBox();
    // RectangleBoundingBox with LowerLeft, LowerRight, UpperLeft, UpperRight
    var angle = oabb.AngleToAxis;  // rotation angle
}
```

## Create Drawing Views from Model Objects

```csharp
// Create basic views (front, top, end) for a model part
var drawingHandler = new DrawingHandler();
drawingHandler.SetActiveDrawing(gaDrawing, true);

var view = new View(gaDrawing.GetSheet(),
    new CoordinateSystem(
        new Point(0, 0, 0),
        new Vector(1, 0, 0),
        new Vector(0, 1, 0)),
    new CoordinateSystem(
        new Point(200, 200, 0),  // placement on sheet
        new Vector(1, 0, 0),
        new Vector(0, 1, 0)),
    new AABB(
        new Point(-100, -100, -100),
        new Point(4000, 1000, 1000)));

view.Insert();
gaDrawing.CommitChanges();
```

## Drawing Polygon (Graphical Shape)

```csharp
// Create a polygon shape in a drawing view
var polygon = new Polygon(view, new PointList());
polygon.Points.Add(new Point(0, 0, 0));
polygon.Points.Add(new Point(100, 0, 0));
polygon.Points.Add(new Point(100, 50, 0));
polygon.Points.Add(new Point(0, 50, 0));
polygon.Points.Add(new Point(0, 0, 0));  // close

polygon.Attributes.Line.Color = DrawingColors.Red;
polygon.Insert();
activeDrawing.CommitChanges();
```

## Drawing identity: use `Mark`, not `Name`

In Tekla Structures, drawing **`Name`** is **NOT unique** — multiple drawings
(different parts, assemblies, views, or revisions) can share the same Name.
The unique identifier is **`Mark`**.

**Always use `Mark` when:**
- Building a `Dictionary<string, Drawing>` lookup to correlate trigger-payload
  entries with live `Drawing` objects (a Name-keyed dict will silently
  overwrite duplicates and use the wrong drawing for UDA reads / status mapping).
- Persisting a drawing to an external system (Supabase row, Excel column, JSON
  cache) — `Mark` is the right unique-constraint key, not `Name`.
- Logging which drawing was processed.

**Skip / warn on empty `Mark`** rather than collapsing such drawings under a
shared key. If you need persistence across renames (rare), use the GUID via
`drawing.GetIdentifier()` — for human-readable stable identity within a
project, `Mark` is the right call.

```csharp
// ✅ CORRECT — key by Mark
var liveByMark = new Dictionary<string, Drawing>(StringComparer.Ordinal);
var enumerator = drawingHandler.GetDrawings();
while (enumerator.MoveNext())
{
    if (enumerator.Current is Drawing d && string.IsNullOrWhiteSpace(d.Mark) is false)
    {
        liveByMark[d.Mark] = d;
    }
    else if (enumerator.Current is Drawing nm)
    {
        context.LogWarning($"Drawing has empty Mark, skipped: Name='{nm.Name}'");
    }
}

// ❌ WRONG — Name is not unique, this silently drops drawings on collision
// var liveByName = new Dictionary<string, Drawing>();
// liveByName[d.Name] = d;
```

## Consuming AWARE Drawing Event Triggers in downstream agent commands

When a downstream agent command is connected downstream of a Tekla drawing event trigger, the
trigger payload arrives in the downstream agent command's `inputs` dictionary. Two delivery
shapes coexist depending on which trigger fired:

### Batch shape — `tekla-drawing-changed` and `tekla-drawing-status-changed`

Tekla's `DrawingChanged` and `DrawingStatusChanged` callbacks are
parameterless — they do not identify which drawing(s) changed. The bridge
resolves the affected set from the active drawing editor (one drawing) OR the
Drawing Manager multi-selection (N drawings) and delivers them in a SINGLE
event whose top-level `drawings` field is an array.

The downstream agent command fires exactly **once per Tekla callback**, regardless of how
many drawings changed. Iterate the array to process every drawing in one run.

```csharp
// inputs["drawings"] is a List<IDictionary<string, object>>.
// Each element is a per-drawing property dictionary mirroring the Tekla Drawing
// schema (name, mark, title1/2/3, upToDateStatus, isIssued, isLocked, etc.).
// Always contains at least one element (single-drawing edits arrive as a
// one-element array, so downstream shape is stable).

if (inputs.TryGetValue("drawings", out var raw) && raw is IEnumerable<object> list)
{
    foreach (var item in list.OfType<IDictionary<string, object>>())
    {
        // Mark is the canonical identity (Name is NOT unique in Tekla).
        var mark           = item.TryGetValue("mark", out var m) ? m as string : null;
        var name           = item.TryGetValue("name", out var n) ? n as string : null;
        var upToDateStatus = item.TryGetValue("upToDateStatus", out var s) ? s as string : null;
        var isIssued       = item.TryGetValue("isIssued", out var i) && i is bool b && b;

        if (string.IsNullOrWhiteSpace(mark))
        {
            context.LogWarning($"Skipping drawing without Mark (Name='{name}')");
            continue;
        }

        // Process this drawing — use `mark` as the unique key in any external
        // store (Supabase row, Excel column, etc.).
    }
}
```

**IMPORTANT:** Trigger payloads carry serialized properties only — they are NOT
live `Tekla.Structures.Drawing.Drawing` objects. To call methods on the actual
Drawing (for example `CommitChanges()`, `Modify()`, `GetUserProperty()`, or to
write back UDAs), look the drawing up via `DrawingHandler` using the **`Mark`**
from the payload (NOT `Name` — see the "Drawing identity" section above), then
operate on the live object as shown in the sections above.

```csharp
// Build the lookup once per batch and key by Mark
var liveByMark = new Dictionary<string, Drawing>(StringComparer.Ordinal);
var enumerator = new DrawingHandler().GetDrawings();
while (enumerator.MoveNext())
{
    if (enumerator.Current is Drawing d && string.IsNullOrWhiteSpace(d.Mark) is false)
    {
        liveByMark[d.Mark] = d;
    }
}

// Inside the foreach over inputs["drawings"]:
var liveDrawing = (mark is not null && liveByMark.TryGetValue(mark, out var ld)) ? ld : null;
```

### Single shape — all other drawing triggers

`tekla-drawing-loaded`, `tekla-drawing-deleted`, `tekla-drawing-inserted`,
`tekla-drawing-updated` (Tekla fires this once per drawing with the Drawing
argument), `tekla-drawing-ready-for-issuing-changed`, and
`tekla-drawing-interrupted` deliver one drawing per invocation, nested under
`tekla.drawing.*`. The downstream agent command `inputs` flattener exposes them as dotted
keys.

```csharp
// Single-drawing shape: keys are flattened with the "tekla.drawing." prefix.
var drawingName    = inputs.TryGetValue("tekla.drawing.name", out var n) ? n as string : null;
var drawingMark    = inputs.TryGetValue("tekla.drawing.mark", out var m) ? m as string : null;
var upToDateStatus = inputs.TryGetValue("tekla.drawing.upToDateStatus", out var s) ? s as string : null;
var updateType     = inputs.TryGetValue("updateType", out var u) ? u as string : null; // tekla-drawing-updated only
```

### Choosing the right trigger for bulk Drawing Manager edits

| Need | Use | Why |
|------|-----|-----|
| ONE workflow run per user action (e.g. one summary email, one batched report row) | `tekla-drawing-changed` (batch) | Iterate `inputs["drawings"]` to handle every changed drawing in a single downstream agent command invocation. |
| ONE workflow run per drawing (e.g. update each drawing's row in Excel individually, post one webhook per drawing) | `tekla-drawing-updated` with `updateType == "Modified"` | Tekla's DrawingUpdated callback is per-drawing; the bridge forwards each one as a separate single-drawing event. |

