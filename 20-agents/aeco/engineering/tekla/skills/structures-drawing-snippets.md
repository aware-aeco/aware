---
parent: tekla-structures-drawing
description: Reusable code snippets for Tekla drawing operations. Find drawings by name or mark, enumerate all drawings, read UDAs, translate UpToDateStatus to friendly text, work with DrawingHandler.
---

## Reusable Code Snippets

### Enumerate All Drawings

```csharp
var drawingHandler = new DrawingHandler();
var enumerator = drawingHandler.GetDrawings();
var drawings = new List<Drawing>();

while (enumerator.MoveNext())
{
    if (enumerator.Current is Drawing drawing)
    {
        drawings.Add(drawing);
    }
}
```

### Find a Drawing by Name

```csharp
Drawing FindDrawingByName(string targetName)
{
    var drawingHandler = new DrawingHandler();
    var enumerator = drawingHandler.GetDrawings();

    while (enumerator.MoveNext())
    {
        if (enumerator.Current is Drawing d
            && string.Equals(d.Name, targetName, StringComparison.Ordinal))
        {
            return d;
        }
    }

    return null;
}
```

### Find a Drawing by Mark

```csharp
Drawing FindDrawingByMark(string targetMark)
{
    var drawingHandler = new DrawingHandler();
    var enumerator = drawingHandler.GetDrawings();

    while (enumerator.MoveNext())
    {
        if (enumerator.Current is Drawing d
            && string.Equals(d.Mark, targetMark, StringComparison.Ordinal))
        {
            return d;
        }
    }

    return null;
}
```

> There is no `drawing.Identifier`, `drawing.Id`, or `drawing.GUID`. Match drawings by `Name` or `Mark`. If you need to track drawings across sessions and both business keys can change, persist your own mapping keyed by `(Name, Mark, Title1)` or by the `AssemblyIdentifier`/`PartIdentifier`/`CastUnitIdentifier` on the relevant subclass (which identifies the underlying MODEL object, not the drawing itself).

### Read Drawing UDAs (User-Defined Attributes)

```csharp
string approvedBy = string.Empty;
string checkedBy = string.Empty;
string assignedTo = string.Empty;

drawing.GetUserProperty("DR_APPROVED_BY", ref approvedBy);
drawing.GetUserProperty("DR_CHECKED_BY", ref checkedBy);
drawing.GetUserProperty("DR_ASSIGNED_TO", ref assignedTo);
```

### Translate UpToDateStatus to Friendly Text

```csharp
string FriendlyStatus(Drawing drawing)
{
    if (drawing.UpToDateStatus == DrawingUpToDateStatus.DrawingIsUpToDate)
    {
        return "approved";
    }

    if (drawing.IsReadyForIssue)
    {
        return "for checking";
    }

    return "in progress";
}
```

### Read All Common Drawing Status Fields

```csharp
var status = new Dictionary<string, object>
{
    ["name"] = drawing.Name,
    ["mark"] = drawing.Mark,
    ["title1"] = drawing.Title1,
    ["title2"] = drawing.Title2,
    ["title3"] = drawing.Title3,
    ["upToDateStatus"] = drawing.UpToDateStatus.ToString(),
    ["isIssued"] = drawing.IsIssued,
    ["isIssuedButModified"] = drawing.IsIssuedButModified,
    ["isReadyForIssue"] = drawing.IsReadyForIssue,
    ["isLocked"] = drawing.IsLocked,
    ["isFrozen"] = drawing.IsFrozen,
    ["isMasterDrawing"] = drawing.IsMasterDrawing,
    ["creationDate"] = drawing.CreationDate,
    ["modificationDate"] = drawing.ModificationDate,
    ["commitMessage"] = drawing.CommitMessage
};
```

### Access the Active Drawing (the one currently open in the editor)

```csharp
var drawingHandler = new DrawingHandler();
var active = drawingHandler.GetActiveDrawing();
if (active is not null)
{
    // use active.Name, active.Mark, etc.
}
```

### Open a Drawing in the Editor

```csharp
var drawingHandler = new DrawingHandler();
drawingHandler.SetActiveDrawing(drawing);
```
