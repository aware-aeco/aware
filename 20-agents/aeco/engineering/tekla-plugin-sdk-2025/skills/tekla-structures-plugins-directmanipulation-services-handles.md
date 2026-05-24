---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services-handles
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services.Handles namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ArcHandle, DragEventArgs, HandleEffectType, HandleLocationType, LineHandle, PointHandle, PolycurveHandle, PolygonalSurfaceHandle, and 1 more types.
---

# Tekla.Structures.Plugins.DirectManipulation.Services.Handles

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 9 types in this namespace.

## ArcHandle (class)

Represents a handle that is represented as an arc in the model.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.ArcHandle)

### Properties
- `Arc` (Tekla.Structures.Geometry3d.Arc, get/set) — Gets or sets the underlying arc of the handle.
- `Radius` (double, get/set) — Gets or sets the radius value of the arc.

## DragEventArgs (class)

Contains the event arguments for a drag event on a handle.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs)

### Properties
- `DeltaTranslation` (Tekla.Structures.Geometry3d.Vector, get) — Gets a vector indicating the translation since the last drag event.
- `Interrupted` (bool, get) — Gets a value indicating whether the event arg was interrupted.
- `StartPosition` (Tekla.Structures.Geometry3d.Point, get) — Gets a point indicating the original position of the handle before the drag event. Property returns middle point in case of LineHandler.
- `TotalTranslation` (Tekla.Structures.Geometry3d.Vector, get) — Gets a vector indicating the total translation since the drag started.

## HandleEffectType (enum)

Defines what the effect of a handle is.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType)

### Values
- `Geometry` = `0` — The handle modifies the geometry by changing input points.
- `Property` = `1` — The handle modifies any object properties, or the handle modifies both object properties and input points.
- `Addition` = `2` — The handle makes additions to the geometry when clicked.

## HandleLocationType (enum)

Defines a handle by where it is located.

**Remarks:** The choice of location type may affect the appearance of the handle.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType)

### Values
- `InputPoint` = `0` — The handle is located on top of an object's input point.
- `MidPoint` = `1` — The handle is located at the exact midpoint of an edge or face.
- `EndPoint` = `2` — The handle is located at one end of an open shape.
- `PointExtension` = `3` — The handle extends the clickable area of a point.
- `Other` = `4` — The handle is located at another location.
- `First` = `5` — The handle is the first defining an open shape.
- `Last` = `6` — The handle is the last defining an open shape.

## LineHandle (class)

Represents a handle that is represented as a line segment on the model.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.LineHandle)

### Properties
- `Line` (Tekla.Structures.Geometry3d.LineSegment, get/set) — Gets the segment that the handle represents.

## PointHandle (class)

Represents a handle that is represented as a point on the model.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PointHandle)

### Properties
- `Point` (Tekla.Structures.Geometry3d.Point, get/set) — Gets the segment that the handle represents.

## PolycurveHandle (class)

Represents a handle that is represented as a polycurve on the model.

**Remarks:** It is advisable to use simpler handles when possible (for example, a LineHandle) since they are more lightweight.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PolycurveHandle)

### Properties
- `Polycurve` (Tekla.Structures.Geometry3d.Polycurve, get/set) — Gets the polycurve that the handle represents.

## PolygonalSurfaceHandle (class)

Represents a handle that is represented as a polygonal surface on the model.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PolygonalSurfaceHandle)

### Properties
- `Contour` (System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.Point>, get/set) — Gets the contour of the surface that the handle represents.

## SelectionEventArgs (class)

Contains the event arguments for a drag event on a handle.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Handles.SelectionEventArgs)

### Properties
- `HitPosition` (Tekla.Structures.Geometry3d.Vector, get) — Gets a point indicating the position of the handle.
- `State` (bool, get) — Gets a value of the selection state.

