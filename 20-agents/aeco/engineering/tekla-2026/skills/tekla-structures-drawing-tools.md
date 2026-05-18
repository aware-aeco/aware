---
name: tekla-tekla-structures-drawing-tools
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Drawing.Tools namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DrawingCoordinateConverter, InputDefinitionFactory.
---

# Tekla.Structures.Drawing.Tools

Auto-generated from vendor docs for tekla 2026.0. 2 types in this namespace.

## DrawingCoordinateConverter (class)

The DrawingCoordinateConverter class is used to move coordinates from one view to another. This tool takes into account the empty areas in the views.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/099ea58a-7599-6e40-2bf9-d1286fd38891)

### Constructors
- `public DrawingCoordinateConverter()` — Initializes a new instance of the DrawingCoordinateConverter class

### Methods
#### `public static Point Convert(ViewBase fromView, ViewBase toView, Point point)`

Converts one coordinate point from one view's coordinate system to another view's coordinate system.

**Parameters:**
- `fromView` (Tekla.Structures.Drawing.ViewBase) — The source view.
- `toView` (Tekla.Structures.Drawing.ViewBase) — The target view.
- `point` (Tekla.Structures.Geometry3d.Point) — The point to be converted.

**Returns:** `Point` — The converted point.

[Docs](https://developer.tekla.com/topic/en/18/47/77a94c8e-e6bc-5d60-c364-c4b4e45d5eb4)

#### `public static PointList Convert(ViewBase fromView, ViewBase toView, PointList pointList)`

Converts a list of coordinate points from one view's coordinate system to another view's coordinate system.

**Parameters:**
- `fromView` (Tekla.Structures.Drawing.ViewBase) — The source view.
- `toView` (Tekla.Structures.Drawing.ViewBase) — The target view.
- `pointList` (Tekla.Structures.Drawing.PointList) — The list of points to be converted.

**Returns:** `PointList` — The converted list of points.

[Docs](https://developer.tekla.com/topic/en/18/47/cde655e6-530b-715c-d3a2-8ff1d6c7598b)

## InputDefinitionFactory (class)

The InputDefinitionFactory class contains helper functions for input definition handling.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3ce1ecfc-0320-5978-8235-52a3cd14c9a6)

### Constructors
- `public InputDefinitionFactory()` — Initializes a new instance of the InputDefinitionFactory class

### Methods
#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(Tuple<DrawingObject, ViewBase> input)`

Creates a single-object input definition.

**Parameters:**
- `input` (System.Tuple<DrawingObject,ViewBase>) — The drawing object and the view to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/84a089fa-dc54-8a22-f6ff-d912b01dca5c)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(Tuple<Point, ViewBase> input)`

Creates a single-point input definition.

**Parameters:**
- `input` (System.Tuple<Point,ViewBase>) — The point and the view to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/09f1de79-d1a5-3da0-368a-5c1e4a69174f)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(Tuple<PointList, ViewBase> input)`

Creates a multi-point input definition.

**Parameters:**
- `input` (System.Tuple<PointList,ViewBase>) — The points and the view to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/50ffa338-dfce-f96b-f27c-5509c6477d81)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(ViewBase view, DrawingObject drawingObject)`

Creates a single-object input definition.

**Parameters:**
- `view` (Tekla.Structures.Drawing.ViewBase) — The view to be used in the input.
- `drawingObject` (Tekla.Structures.Drawing.DrawingObject) — The object to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/ccd25bad-30a2-87f1-a291-99531a837a39)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(ViewBase view, Point point)`

Creates a single-point input definition.

**Parameters:**
- `view` (Tekla.Structures.Drawing.ViewBase) — The view to be used in the input.
- `point` (Tekla.Structures.Geometry3d.Point) — The point to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/c5bbd8b8-2fba-78bc-0ffd-67457e8144bf)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(ViewBase view, Point point1, Point point2)`

Creates a two-point input definition.

**Parameters:**
- `view` (Tekla.Structures.Drawing.ViewBase) — The view to be used in the input.
- `point1` (Tekla.Structures.Geometry3d.Point) — The first point to be used in the input.
- `point2` (Tekla.Structures.Geometry3d.Point) — The second point to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/ae205985-792e-1c17-edff-e88331837cb9)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(ViewBase view, Point point1, Point point2, Point point3)`

Creates a three-point input definition.

**Parameters:**
- `view` (Tekla.Structures.Drawing.ViewBase) — The view to be used in the input.
- `point1` (Tekla.Structures.Geometry3d.Point) — The first point to be used in the input.
- `point2` (Tekla.Structures.Geometry3d.Point) — The second point to be used in the input.
- `point3` (Tekla.Structures.Geometry3d.Point) — The third point to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/6dbcb73d-a621-c67f-1779-ba20d8a800ea)

#### `public static DrawingPluginBase.InputDefinition CreateInputDefinition(ViewBase view, PointList points)`

Creates a multi-point input definition.

**Parameters:**
- `view` (Tekla.Structures.Drawing.ViewBase) — The view to be used in the input.
- `points` (Tekla.Structures.Drawing.PointList) — The points to be used in the input.

**Returns:** `DrawingPluginBase.InputDefinition` — The input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/a169f862-3ab6-6393-4d2c-0919a848fc92)

#### `public static DrawingObject GetDrawingObject(DrawingPluginBase.InputDefinition input)`

Gets the object from the given single-object input.

**Parameters:**
- `input` (Tekla.Structures.Plugins.DrawingPluginBase.InputDefinition) — The input.

**Returns:** `DrawingObject` — The object.

[Docs](https://developer.tekla.com/topic/en/18/47/e07e0bd1-185a-2f57-6e97-7530043d6d89)

#### `public static Point GetPoint(DrawingPluginBase.InputDefinition input)`

Gets the point from the given single-point input.

**Parameters:**
- `input` (Tekla.Structures.Plugins.DrawingPluginBase.InputDefinition) — The input.

**Returns:** `Point` — The point.

[Docs](https://developer.tekla.com/topic/en/18/47/d118cf03-e535-2b81-6f9c-857de6299a7d)

#### `public static PointList GetPoints(DrawingPluginBase.InputDefinition input)`

Gets the points from the given n-point input.

**Parameters:**
- `input` (Tekla.Structures.Plugins.DrawingPluginBase.InputDefinition) — The input.

**Returns:** `PointList` — The points.

[Docs](https://developer.tekla.com/topic/en/18/47/d1cfbbc8-bf9f-043e-1964-539796a929cc)

#### `public static ViewBase GetView(DrawingPluginBase.InputDefinition input)`

Gets the view that the given input belongs to.

**Parameters:**
- `input` (Tekla.Structures.Plugins.DrawingPluginBase.InputDefinition) — The input.

**Returns:** `ViewBase` — The view.

[Docs](https://developer.tekla.com/topic/en/18/47/8581caba-017b-6552-f26e-bdd92e953ec0)

