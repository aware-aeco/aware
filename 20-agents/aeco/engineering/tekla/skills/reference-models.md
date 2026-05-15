---
name: tekla-reference-models
description: Tekla Open API - Reference model operations. Import IFC, DWG, DGN reference models. Position, scale, rotate reference models. Enumerate reference model objects and children. Query reference model object properties. Refresh and update reference models. This skill should be used when importing, managing, or querying external reference models in Tekla.
---

# Tekla Reference Model Operations (v2025)

## Critical Patterns

- `ReferenceModel` is a ModelObject — uses Insert/Modify/Delete/Select like other objects
- Reference model file path must be accessible from the Tekla model machine
- Use `GetChildren()` to enumerate objects within the reference model
- Reference model objects (`ReferenceModelObject`) have limited properties compared to native parts
- Position/rotation is controlled via the ReferenceModel's placement properties

## Import a Reference Model

```csharp
var model = new Model();

var referenceModel = new ReferenceModel();
referenceModel.Filename = @"C:\Models\Architectural.ifc";
referenceModel.Position = new Point(0, 0, 0);
referenceModel.Insert();
model.CommitChanges();
```

## Enumerate Reference Models

```csharp
var model = new Model();
var selector = model.GetModelObjectSelector();
var enumerator = selector.GetAllObjectsWithType(
    ModelObject.ModelObjectEnum.REFERENCE_MODEL);

while (enumerator.MoveNext())
{
    if (enumerator.Current is ReferenceModel refModel)
    {
        refModel.Select();
        var filename = refModel.Filename;
        var position = refModel.Position;
    }
}
```

## Get Reference Model Children

```csharp
refModel.Select();

var children = refModel.GetChildren();
while (children.MoveNext())
{
    if (children.Current is ReferenceModelObject refObj)
    {
        refObj.Select();

        // Read properties via GetUserProperty or report properties
        string objectName = "";
        refObj.GetUserProperty("REFERENCE_OBJECT_NAME", ref objectName);
    }
}
```

## Position and Rotate Reference Model

```csharp
var model = new Model();
refModel.Select();

// Move to new position
refModel.Position = new Point(5000, 3000, 0);
refModel.Modify();
model.CommitChanges();
```

## Refresh/Update Reference Model

```csharp
// Re-read the external file to update changes
refModel.Select();
// Tekla automatically detects file changes
// Use Operation to force refresh if needed
refModel.Modify();
model.CommitChanges();
```
