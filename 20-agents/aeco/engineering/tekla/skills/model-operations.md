---
name: tekla-model-operations
description: Tekla Open API - Model operations. Move, copy, rotate, mirror, split, delete objects. Grid creation, base points, view management, temporary visibility, WorkPlaneHandler. This skill should be used when transforming, moving, copying, or manipulating existing model objects, creating grids, or controlling views and visibility.
---

# Tekla Model Operations API Reference (v2025)

## Critical Patterns

- The `Operation` class is static — call methods directly: `Operation.MoveObject(obj, vector)`
- Always call `model.CommitChanges()` after move, copy, split, delete, or insert operations
- Always call `Select()` on an object before passing it to an operation — the API needs populated properties
- Work in global coordinates for consistency: set `TransformationPlane` to global before operations, restore afterward
- Copy operations return an `Identifier` — use `model.SelectModelObject(identifier)` to get the new object
- Grid coordinates and labels are space-separated strings, not arrays or lists
- Always restore the original `TransformationPlane` in a `finally` block when using `WorkPlaneHandler`

## Object Transformations

### Move Object
`Operation.MoveObject(ModelObject obj, Vector displacement)` moves an object by a displacement vector. The object must be selected first. Call `CommitChanges()` after.

### Copy Object
`Operation.CopyObject(ModelObject obj, Vector displacement)` copies an object and returns the new object's `Identifier`. Use `model.SelectModelObject(identifier)` to access the copy. Modify the copy's properties and call `Modify()` if needed.

### Delete Objects
`Operation.DeleteObjects(ArrayList objects)` deletes multiple objects at once. Pass an `ArrayList` of `ModelObject` instances. Call `CommitChanges()` after.

### Show Only Selected
`Operation.ShowOnlySelected()` hides all objects except the currently selected ones in the active view.

## Grid Creation

`Grid` is a `ModelObject` that defines the structural grid system. Coordinates are space-separated millimeter values (e.g., `"0 3000 6000 9000"`). Labels are space-separated strings matching the coordinate count (e.g., `"A B C D"`). Use `Insert()` for new grids, `Modify()` for existing.

Key properties:
- `CoordinateX`, `CoordinateY`, `CoordinateZ` — space-separated coordinate values in mm
- `LabelX`, `LabelY`, `LabelZ` — space-separated labels matching coordinate count

## View Management

`ViewHandler` provides access to model views. Use `GetAllViews()` to iterate all views via `ModelObjectEnumerator` pattern (`while (views.MoveNext())`). Each view is a `View` object with `Name`, filter settings, and depth controls.

## Temporary Visibility

`ModelObjectVisualization` controls temporary display states. Use `SetTemporaryState()` with `TemporaryStateEnum` values:
- `SHOW_ONLY_SELECTED` — hide everything except selected objects
- `SHOW_ALL` — reset visibility to show all objects

## Utility Operations

### Split
`Operation.Split(Part part, Point splitPoint)` splits a beam or plate at the given point. Returns `true` on success. Call `CommitChanges()` after.

### WorkPlaneHandler
Use `model.GetWorkPlaneHandler()` to get/set the current transformation plane. Set to a part's coordinate system for local operations, always restore the original plane in a `finally` block.

> Detailed code snippets for all operations are available in the supporting file `tekla-model-operations-snippets.md`.
