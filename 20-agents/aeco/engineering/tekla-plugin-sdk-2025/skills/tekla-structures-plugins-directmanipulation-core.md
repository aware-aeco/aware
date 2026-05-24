---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-core
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Core namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ControlBase, HandleBase, IGraphicsDrawer, IHandleManager, IToolbar, ManipulationContext, ManipulatorBase.
---

# Tekla.Structures.Plugins.DirectManipulation.Core

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 7 types in this namespace.

## ControlBase (class)

The ControlBase class is an abstract base class for all UI controls provided by the API.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.ControlBase)

### Methods
#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.ControlBase.Dispose)

### Properties
- `CustomProperties` (System.Collections.Generic.Dictionary<object, object>, get/set) — Gets or sets the custom properties of the control.
- `Enabled` (bool, get/set) — Gets or sets a value indicating whether the control can respond to user interaction.
- `Image` (System.Drawing.Image, get/set) — Gets or sets the image that is displayed on the item.
- `Tag` (object, get/set) — Gets or sets the object that contains data about the item.
- `Text` (string, get/set) — Gets or sets the text that is to be displayed on the item.
- `Tooltip` (string, get/set) — Gets or sets the tooltip of the control.
- `Visible` (bool, get/set) — Gets or sets a value indicating whether the control is displayed.

## HandleBase (class)

Represents a (potentially) draggable handle

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.HandleBase)

### Methods
#### `void Dispose()`

Disposes the handle. This can be used to remove a handle from the screen.

**Remarks:** The object becomes unusable after calling dispose, and it is unsafe to use the properties of the handle.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.Dispose)

#### `void SetFreeTolerationBehavior()`

Sets the toleration behavior to be free toleration.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.SetFreeTolerationBehavior)

#### `void SetLineTolerationBehavior(Tekla.Structures.Geometry3d.Line tolerationLine)`

Sets the toleration behavior to be restricted to the tolerationLine.

**Parameters:**
- `tolerationLine` (Tekla.Structures.Geometry3d.Line) — The line to define the toleration line.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.SetLineTolerationBehavior%28Tekla.Structures.Geometry3d.Line%29)

#### `void SetPlaneTolerationBehavior(Tekla.Structures.Geometry3d.GeometricPlane tolerationPlane)`

Sets the toleration behavior to be restricted to the plane defined by the tolerationPlane.

**Parameters:**
- `tolerationPlane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to define the toleration plane.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.SetPlaneTolerationBehavior%28Tekla.Structures.Geometry3d.GeometricPlane%29)

### Properties
- `IsDragOngoing` (bool, get) — Gets a value indicating whether the handle is being dragged.
- `IsHighlighted` (bool, get) — Gets a value indicating whether the handle is highlighted.
- `IsInvalid` (bool, get/set) — Gets or sets a value indicating whether the handle is invalid (which may change its appearance).
- `IsSelected` (bool, get) — Gets a value indicating whether the handle is selected.
- `IsVisible` (bool, get/set) — Gets a value indicating whether the handle is visible.
- `Tag` (object, get/set) — Gets or sets a value that contains additional info for the handle. As a user, this may be set to transport information related to the context of the handle.

### Events
#### `DragEnded` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs> DragEnded`

Occurs when dragging ends.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.DragEnded)

#### `DragOngoing` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs> DragOngoing`

Occurs while dragging continues at periodic intervals.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.DragOngoing)

#### `DragStarted` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.DragEventArgs> DragStarted`

Occurs when dragging starts.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.DragStarted)

#### `SelectedChanged` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.SelectionEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Handles.SelectionEventArgs> SelectedChanged`

Occurs when the value of the Selected property has changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Core.HandleBase.SelectedChanged)

## IGraphicsDrawer (interface)

The IGraphicsDrawer interfaces the graphics drawer.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer)

### Methods
#### `void Clear()`

Removes all graphics drawn by the current graphics from the views.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.Clear)

#### `void DrawArc(Tekla.Structures.Geometry3d.Arc arc, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the curve of the given arc to the views.

**Parameters:**
- `arc` (Tekla.Structures.Geometry3d.Arc) — The Arc defining the graphic.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawArc%28Tekla.Structures.Geometry3d.Arc%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawCircle(Tekla.Structures.Geometry3d.Point center, Tekla.Structures.Geometry3d.Vector normal, double radius, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the circle to the views.

**Parameters:**
- `center` (Tekla.Structures.Geometry3d.Point) — The center.
- `normal` (Tekla.Structures.Geometry3d.Vector) — The normal of the circle.
- `radius` (double) — The radius of the circle.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawCircle%28Tekla.Structures.Geometry3d.Point%2CTekla.Structures.Geometry3d.Vector%2CSystem.Double%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawCustomPart(string customPart, Tekla.Structures.Geometry3d.LineSegment direction, Tekla.Structures.Geometry3d.Vector offsetVector, double rotationInDegrees = 0, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the specified customPart at the given direction.

**Parameters:**
- `customPart` (string) — The custom part.
- `direction` (Tekla.Structures.Geometry3d.LineSegment) — The direction.
- `offsetVector` (Tekla.Structures.Geometry3d.Vector) — A vector, that defines the offset of the drawn object from the relative origin.
- `rotationInDegrees` (double) — The rotation angle around the direction in degrees.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawCustomPart%28System.String%2CTekla.Structures.Geometry3d.LineSegment%2CTekla.Structures.Geometry3d.Vector%2CSystem.Double%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawDimension(Tekla.Structures.Geometry3d.LineSegment line, Tekla.Structures.Geometry3d.Vector graphicNormal = null, Tekla.Structures.Plugins.DirectManipulation.Services.DimensionEndPointSizeType sizeType = 0)`

Draws a dimension line with direction arrows.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.LineSegment) — The line to define the dimension line.
- `graphicNormal` (Tekla.Structures.Geometry3d.Vector) — The normal to define the arrow orientation.
- `sizeType` (Tekla.Structures.Plugins.DirectManipulation.Services.DimensionEndPointSizeType) — Type of the size of the dimension arrows.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawDimension%28Tekla.Structures.Geometry3d.LineSegment%2CTekla.Structures.Geometry3d.Vector%2CTekla.Structures.Plugins.DirectManipulation.Services.DimensionEndPointSizeType%29)

#### `void DrawExtrema(Tekla.Structures.Geometry3d.AABB extrema, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the specified extrema.

**Parameters:**
- `extrema` (Tekla.Structures.Geometry3d.AABB) — The extrema.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawExtrema%28Tekla.Structures.Geometry3d.AABB%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawExtrema(Tekla.Structures.Geometry3d.LineSegment direction, double width, double height, Tekla.Structures.Geometry3d.Vector offsetVector, double rotationInDegrees = 0, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws an extrema box with given width, height and direction.

**Parameters:**
- `direction` (Tekla.Structures.Geometry3d.LineSegment) — The direction.
- `width` (double) — The width.
- `height` (double) — The height.
- `offsetVector` (Tekla.Structures.Geometry3d.Vector) — A vector, that defines the offset of the drawn object from the relative origin.
- `rotationInDegrees` (double) — The rotation angle around the direction in degrees.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawExtrema%28Tekla.Structures.Geometry3d.LineSegment%2CSystem.Double%2CSystem.Double%2CTekla.Structures.Geometry3d.Vector%2CSystem.Double%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawFace(System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.Point> contourPoints, Tekla.Structures.Plugins.DirectManipulation.Services.SurfaceType surfaceType = 0, bool doubleSided = true)`

Draws the mesh composed of the given contourPoints.

**Parameters:**
- `contourPoints` (System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.Point>) — The contour points.
- `surfaceType` (Tekla.Structures.Plugins.DirectManipulation.Services.SurfaceType) — Type of the surface.
- `doubleSided` (bool) — A value indicating whether the mesh is double-sided.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawFace%28System.Collections.Generic.IEnumerable%7BTekla.Structures.Geometry3d.Point%7D%2CTekla.Structures.Plugins.DirectManipulation.Services.SurfaceType%2CSystem.Boolean%29)

#### `void DrawLine(Tekla.Structures.Geometry3d.Point startPoint, Tekla.Structures.Geometry3d.Point endPoint, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the line with the given startPoint and endPoint.

**Parameters:**
- `startPoint` (Tekla.Structures.Geometry3d.Point) — The start point.
- `endPoint` (Tekla.Structures.Geometry3d.Point) — The end point.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawLine%28Tekla.Structures.Geometry3d.Point%2CTekla.Structures.Geometry3d.Point%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawLines(System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.LineSegment> polyline, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the given polyline.

**Parameters:**
- `polyline` (System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.LineSegment>) — The polyline.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawLines%28System.Collections.Generic.IEnumerable%7BTekla.Structures.Geometry3d.LineSegment%7D%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawProfile(string profile, Tekla.Structures.Geometry3d.LineSegment direction, Tekla.Structures.Geometry3d.Vector offsetVector, double rotationInDegrees = 0, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the specified profile at the given direction.

**Parameters:**
- `profile` (string) — The profile.
- `direction` (Tekla.Structures.Geometry3d.LineSegment) — The direction.
- `offsetVector` (Tekla.Structures.Geometry3d.Vector) — A vector, that defines the offset of the drawn object from the relative origin.
- `rotationInDegrees` (double) — The rotation angle around the direction in degrees.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawProfile%28System.String%2CTekla.Structures.Geometry3d.LineSegment%2CTekla.Structures.Geometry3d.Vector%2CSystem.Double%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawShape(string shape, Tekla.Structures.Geometry3d.LineSegment direction, Tekla.Structures.Geometry3d.Vector offsetVector, double rotationInDegrees = 0, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Draws the specified shape at the given direction.

**Parameters:**
- `shape` (string) — The shape.
- `direction` (Tekla.Structures.Geometry3d.LineSegment) — The direction.
- `offsetVector` (Tekla.Structures.Geometry3d.Vector) — A vector, that defines the offset of the drawn object from the relative origin.
- `rotationInDegrees` (double) — The rotation angle around the direction in degrees.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the curve.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawShape%28System.String%2CTekla.Structures.Geometry3d.LineSegment%2CTekla.Structures.Geometry3d.Vector%2CSystem.Double%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

#### `void DrawText(string text, Tekla.Structures.Geometry3d.Point location, Tekla.Structures.Plugins.DirectManipulation.Services.TextRepresentationTypes textTypes = 2)`

Draws the given text at the location.

**Parameters:**
- `text` (string) — The text.
- `location` (Tekla.Structures.Geometry3d.Point) — The location.
- `textTypes` (Tekla.Structures.Plugins.DirectManipulation.Services.TextRepresentationTypes) — Type of the text.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IGraphicsDrawer.DrawText%28System.String%2CTekla.Structures.Geometry3d.Point%2CTekla.Structures.Plugins.DirectManipulation.Services.TextRepresentationTypes%29)

## IHandleManager (interface)

This interface specifies factory methods to create handles.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.IHandleManager)

### Methods
#### `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.ArcHandle CreateArcHandle(Tekla.Structures.Geometry3d.Arc arc, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType locationType, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType effectType)`

Creates an arc handle.

**Parameters:**
- `arc` (Tekla.Structures.Geometry3d.Arc) — The actual arc that the handle uses.
- `locationType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType) — Handle location type.
- `effectType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType) — Handle effect type.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.ArcHandle` — 

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IHandleManager.CreateArcHandle%28Tekla.Structures.Geometry3d.Arc%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.LineHandle CreateLineHandle(Tekla.Structures.Geometry3d.LineSegment line, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType locationType, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType effectType)`

Creates a line handle.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.LineSegment) — Segment in space where the handle will be created.
- `locationType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType) — Handle location type.
- `effectType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType) — Handle effect type.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.LineHandle` — A newly created line handle.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IHandleManager.CreateLineHandle%28Tekla.Structures.Geometry3d.LineSegment%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PointHandle CreatePointHandle(Tekla.Structures.Geometry3d.Point location, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType locationType, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType effectType)`

Creates a point handle.

**Parameters:**
- `location` (Tekla.Structures.Geometry3d.Point) — Point in space where the handle will be created.
- `locationType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType) — Handle location type.
- `effectType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType) — Handle effect type.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PointHandle` — A newly created point handle.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IHandleManager.CreatePointHandle%28Tekla.Structures.Geometry3d.Point%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PolycurveHandle CreatePolycurveHandle(Tekla.Structures.Geometry3d.Polycurve polycurve, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType locationType, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType effectType)`

Creates a polycurve handle.

**Remarks:** It is advisable to use simpler handles when possible (for example, a LineHandle) since they
are more lightweight.

**Parameters:**
- `polycurve` (Tekla.Structures.Geometry3d.Polycurve) — Polycurve represented by the handle.
- `locationType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType) — Handle location type.
- `effectType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType) — Handle effect type.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PolycurveHandle` — A newly created polycurve handle.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IHandleManager.CreatePolycurveHandle%28Tekla.Structures.Geometry3d.Polycurve%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PolygonalSurfaceHandle CreatePolygonalSurfaceHandle(System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.Point> surfaceContour, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType locationType, Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType effectType)`

Creates a polygonal surface handle.

**Parameters:**
- `surfaceContour` (System.Collections.Generic.IEnumerable<Tekla.Structures.Geometry3d.Point>) — Contour of the surface.
- `locationType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType) — Handle location type.
- `effectType` (Tekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType) — Handle effect type.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Handles.PolygonalSurfaceHandle` — A newly created surface handle.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IHandleManager.CreatePolygonalSurfaceHandle%28System.Collections.Generic.IEnumerable%7BTekla.Structures.Geometry3d.Point%7D%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleLocationType%2CTekla.Structures.Plugins.DirectManipulation.Services.Handles.HandleEffectType%29)

## IToolbar (interface)

The IToolbar interfaces the toolbar creation service.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.IToolbar)

### Methods
#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl CreateAngleValueTextBox(double? value = 0)`

Creates angle value text box control.

**Parameters:**
- `value` (double?) — The value.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl` — The angle value text box control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateAngleValueTextBox%28System.Nullable%7BSystem.Double%7D%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ButtonControl CreateButton(string text = null, System.Drawing.Image image = null)`

Creates button control.

**Parameters:**
- `text` (string) — The text.
- `image` (System.Drawing.Image) — The image.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ButtonControl` — The button control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateButton%28System.String%2CSystem.Drawing.Image%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.CheckBoxControl CreateCheckBox(string text = null, System.Drawing.Image image = null)`

Creates check box control.

**Parameters:**
- `text` (string) — The text.
- `image` (System.Drawing.Image) — The image.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.CheckBoxControl` — The check box control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateCheckBox%28System.String%2CSystem.Drawing.Image%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl CreateDistanceValueTextBox(double? value = 0)`

Creates distance value text box control.

**Parameters:**
- `value` (double?) — The value.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl` — The distance value text box control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateDistanceValueTextBox%28System.Nullable%7BSystem.Double%7D%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl CreateDropDown()`

Creates drop down control.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl` — The drop down list control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateDropDown)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl CreateDropDown(System.Collections.Generic.IEnumerable<object> items)`

Creates the drop down.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<object>) — The items.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl` — The drop down list control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateDropDown%28System.Collections.Generic.IEnumerable%7BSystem.Object%7D%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl CreateDropDown(System.Collections.Generic.IEnumerable<object> items, object selectedItem)`

Creates the drop down.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<object>) — The items.
- `selectedItem` (object) — The selected item.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl` — The drop down list control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateDropDown%28System.Collections.Generic.IEnumerable%7BSystem.Object%7D%2CSystem.Object%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.LabelControl CreateLabel(string text = null, System.Drawing.Image image = null)`

Creates the label control.

**Parameters:**
- `text` (string) — The text.
- `image` (System.Drawing.Image) — The image.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.LabelControl` — The label control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateLabel%28System.String%2CSystem.Drawing.Image%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.RadioButtonControl CreateRadioButton(string text = null, System.Drawing.Image image = null, string group = null)`

Creates radio button control.

**Parameters:**
- `text` (string) — The text.
- `image` (System.Drawing.Image) — The image.
- `group` (string) — The group.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.RadioButtonControl` — The radio button control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateRadioButton%28System.String%2CSystem.Drawing.Image%2CSystem.String%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.TextBoxControl CreateTextBox(string text = null, System.Drawing.Image image = null)`

Creates text box control.

**Parameters:**
- `text` (string) — The text.
- `image` (System.Drawing.Image) — The image.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.TextBoxControl` — The text box control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateTextBox%28System.String%2CSystem.Drawing.Image%29)

#### `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl CreateValueTextBox(double? value = 0)`

Creates value text box control.

**Parameters:**
- `value` (double?) — The value.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl` — The value text box control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.CreateValueTextBox%28System.Nullable%7BSystem.Double%7D%29)

#### `void Remove(Tekla.Structures.Plugins.DirectManipulation.Core.ControlBase control)`

Removes the specified control.

**Parameters:**
- `control` (Tekla.Structures.Plugins.DirectManipulation.Core.ControlBase) — The control.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.IToolbar.Remove%28Tekla.Structures.Plugins.DirectManipulation.Core.ControlBase%29)

## ManipulationContext (class)

The ManipulationContext class provides a base class for custom manipulation behavior classes.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext)

### Constructors
- `ManipulationContext(Tekla.Structures.Model.Component component, Tekla.Structures.Plugins.DirectManipulation.Core.Features.PluginManipulationFeatureBase feature)` — Initializes a new instance of the ManipulationContext class.
- `ManipulationContext(Tekla.Structures.Model.CustomPart customPart, Tekla.Structures.Plugins.DirectManipulation.Core.Features.CustomPartPluginManipulationFeatureBase feature)` — Initializes a new instance of the ManipulationContext class.

### Methods
#### `void AddManipulator(Tekla.Structures.Plugins.DirectManipulation.Core.ManipulatorBase newManipulator)`

Adds the manipulator.

**Parameters:**
- `newManipulator` (Tekla.Structures.Plugins.DirectManipulation.Core.ManipulatorBase) — The manipulator.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext.AddManipulator%28Tekla.Structures.Plugins.DirectManipulation.Core.ManipulatorBase%29)

#### `void Dispose()`

Releases unmanaged and - optionally - managed resources.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext.Dispose)

#### `void UpdateContext()`

Updates the context.

**Remarks:** This method must be overridden to update the manipulation context.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext.UpdateContext)

### Properties
- `Manipulators` (System.Collections.Generic.IReadOnlyList<Tekla.Structures.Plugins.DirectManipulation.Core.ManipulatorBase>, get) — Gets the attached manipulators.

## ManipulatorBase (class)

The ManipulatorBase class is a base class used to wrap the manipulator concept for the DM API.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Core.ManipulatorBase)

### Methods
#### `void Dispose()`

Dispose method of ManipulatorBase.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Core.ManipulatorBase.Dispose)

