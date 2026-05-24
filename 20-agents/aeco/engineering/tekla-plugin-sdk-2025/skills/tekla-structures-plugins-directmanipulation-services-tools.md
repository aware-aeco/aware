---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services-tools
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services.Tools namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DistanceManipulator, Highlighter, PolygonRequirements.
---

# Tekla.Structures.Plugins.DirectManipulation.Services.Tools

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 3 types in this namespace.

## DistanceManipulator (class)

The DistanceManipulator class defines a manipulator to show and modify the distance between two points.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.DistanceManipulator)

### Constructors
- `DistanceManipulator(object parent, Tekla.Structures.Plugins.DirectManipulation.Core.ManipulationContext manipulationContext, Tekla.Structures.Geometry3d.LineSegment segment)` — Initializes a new instance of the DistanceManipulator class.

### Properties
- `Parent` (object, get/set) — Gets or sets the parent object.
- `Segment` (Tekla.Structures.Geometry3d.LineSegment, get/set) — Gets or sets the dimensioned LineSegment.

### Events
#### `MeasureChangeOngoing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> MeasureChangeOngoing`

Event to be called when the measure change is ongoing.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.DistanceManipulator.MeasureChangeOngoing)

#### `MeasureChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> MeasureChanged`

Event to be called when the current measure has been changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.DistanceManipulator.MeasureChanged)

## Highlighter (class)

The Highlighter class is used to highlight objects in the model.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Highlighter)

### Methods
#### `void ClearHighlights()`

Clears the highlights.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Highlighter.ClearHighlights)

#### `void HighlightPart(Tekla.Structures.Model.Part part, Tekla.Structures.Plugins.DirectManipulation.Services.LineType lineType = 0)`

Highlights the part.

**Parameters:**
- `part` (Tekla.Structures.Model.Part) — The part.
- `lineType` (Tekla.Structures.Plugins.DirectManipulation.Services.LineType) — Type of the line.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Tools.Highlighter.HighlightPart%28Tekla.Structures.Model.Part%2CTekla.Structures.Plugins.DirectManipulation.Services.LineType%29)

## PolygonRequirements (class)

The PolygonRequirements class defines the requirements for valid polygon behavior.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Tools.PolygonRequirements)

### Constructors
- `PolygonRequirements()` — Constructs a new PolygonRequirements.

### Properties
- `Closed` (bool, get/set) — Gets or sets a value indicating whether the polygon must be considered as a closed loop.
- `MaximumVertexCount` (int, get) — Gets the maximum number of vertices in the polygon.
- `MinimumVertexCount` (int, get) — Gets the minimum number of vertices in the polygon.
- `Planar` (bool, get/set) — Gets or sets a value indicating whether the polygon must be planar.

