---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services-tools-picking
description: This skill encodes the tekla-plugin-sdk 2026.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking namespace — 12 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomSnappingEventArgs, HitObject, HitObject`1, HitPolygon, HitSegment, InputRange, InputTypes, InputValidationEventArgs, and 4 more types.
---

# Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking

Auto-generated from vendor docs for tekla-plugin-sdk 2026.0. 12 types in this namespace.

## CustomSnappingEventArgs (class)

The CustomSnappingEventArgs class provides the event arguments for custom snapping.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.CustomSnappingEventArgs)

### Properties
- `CustomSnappedPoint` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.SnappedPoint, get) — Gets the custom snapped point.

## HitObject (class)

The HitObject class represents the tolerated object.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitObject)

## HitObject`1 (class)

The HitObject`1 class represents the tolerated object.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitObject`1)

### Properties
- `HitPoint` (Tekla.Structures.Geometry3d.Point, get) — Gets the hit point.
- `Object` (T, get) — Gets the tolerated object.

## HitPolygon (class)

The HitPolygon class represents the tolerated Polygon face.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitPolygon)

## HitSegment (class)

The HitSegment class represents the tolerated LineSegment.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitSegment)

## InputRange (class)

The InputRange class defines the expected amount of model object inputs for the picker..

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange)

### Methods
#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange AtLeast(uint inputAmount)`

Factory method for specifying an input amount with a lower limit.

**Parameters:**
- `inputAmount` (uint) — The input amount.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange` — An InputRange instance.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange.AtLeast%28System.UInt32%29)

#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange AtMost(uint inputAmount)`

Factory method for specifying an input amount with an upper limit.

**Parameters:**
- `inputAmount` (uint) — The input amount.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange` — An InputRange instance.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange.AtMost%28System.UInt32%29)

#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange Exactly(uint inputAmount)`

Factory method for specifying an exact input amount.

**Parameters:**
- `inputAmount` (uint) — The input amount.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange` — An InputRange instance.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange.Exactly%28System.UInt32%29)

#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange InRangeOf(uint minAmount, uint maxAmount)`

Factory method for specifying input amount in a certain range.

**Parameters:**
- `minAmount` (uint) — The minimum amount.
- `maxAmount` (uint) — The maximum amount.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange` — An InputRange instance.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange.InRangeOf%28System.UInt32%2CSystem.UInt32%29)

### Properties
- `Maximum` (uint, get) — Gets the maximum input amount.
- `Minimum` (uint, get) — Gets the minimum input amount.

## InputTypes (enum)

The InputTypes enum defines the different kind of input types for which a picker can be started.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes)

### Values
- `None` = `0` — Default, pick nothing.
- `Point` = `1` — Pick any point.
- `Segment` = `2` — Pick a segment.
- `Face` = `4` — Pick a face.
- `Object` = `8` — Pick an object.

## InputValidationEventArgs (class)

The InputValidationEventArgs class provides the event argument for pick ending events.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputValidationEventArgs)

### Constructors
- `InputValidationEventArgs()` — Constructs a new InputValidationEventArgs.

### Properties
- `ContinueSession` (bool, get/set) — Gets or sets a value indicating whether the picking session should continue even though the picking is done.

## PickSkipCountChangedEventArgs (class)

The PickSkipCountChangedEventArgs class provides the event arguments for a skip count change.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickSkipCountChangedEventArgs)

### Properties
- `Decreased` (bool, get) — Gets a value indicating whether the skip count has decreased. If false, the skip count has Increased.
- `Increased` (bool, get) — Gets a value indicating whether the skip count has increased. If false, the skip count has Decreased.
- `SkipCount` (int, get) — Gets the skip count.

## PickingTool (class)

The PickingTool class enables the user to pick points, faces, segments and objects in the model.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool)

### Methods
#### `void Dispose()`

Dispose method of PickingTool.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.Dispose)

#### `void StartPickingSession(string message)`

Starts the picking session.

**Parameters:**
- `message` (string)

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.StartPickingSession%28System.String%29)

### Properties
- `InputRange` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange, get) — Gets the input range.
- `InputTypes` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes, get) — Gets the input types.

### Events
#### `CustomSnappedPointRequested` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.CustomSnappingEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.CustomSnappingEventArgs> CustomSnappedPointRequested`

Occurs when custom snapping becomes available.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.CustomSnappedPointRequested)

#### `InputValidationRequested` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputValidationEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputValidationEventArgs> InputValidationRequested`

Occurs when the user has tried to end the session before reaching the Maximum.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.InputValidationRequested)

#### `ObjectPicked` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.ToleratedObjectEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.ToleratedObjectEventArgs> ObjectPicked`

Occurs when an object has been picked.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.ObjectPicked)

#### `PickSessionEnded` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> PickSessionEnded`

Occurs when the tool is finished.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.PickSessionEnded)

#### `PickSessionInterrupted` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> PickSessionInterrupted`

Occurs when the session gets interrupted by the DM platform.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.PickSessionInterrupted)

#### `PickUndone` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> PickUndone`

Occurs when the last pick should be undone.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.PickUndone)

#### `PreviewRequested` (`System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.ToleratedObjectEventArgs>`)

**Signature:** `event System.EventHandler<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.ToleratedObjectEventArgs> PreviewRequested`

Occurs when the preview should be drawn.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool.PreviewRequested)

## SnappedPoint (class)

Initializes a new instance of the SnappedPoint class.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.SnappedPoint)

### Constructors
- `SnappedPoint()` — Constructs a new SnappedPoint.

### Properties
- `Point` (Tekla.Structures.Geometry3d.Point, get/set) — Gets or sets the location of the snapping point.

## ToleratedObjectEventArgs (class)

The ToleratedObjectEventArgs class provides the event arguments for a picked object.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.ToleratedObjectEventArgs)

### Properties
- `Faces` (System.Collections.Generic.IReadOnlyList<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitPolygon>, get) — Gets the faces.
- `HitPoint` (Tekla.Structures.Geometry3d.Point, get) — Gets the hit point.
- `IsValid` (bool, get/set) — Gets or set s a value indicating whether the new pick is valid.
- `Objects` (System.Collections.Generic.IReadOnlyList<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitObject>, get) — Gets the objects.
- `Segments` (System.Collections.Generic.IReadOnlyList<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitSegment>, get) — Gets the segments.
- `VisibleObjects` (System.Collections.Generic.IReadOnlyList<Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.HitObject>, get) — Gets the visible objects.

