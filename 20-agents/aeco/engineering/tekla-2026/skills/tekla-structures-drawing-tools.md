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
- `DrawingCoordinateConverter(...)` — Initializes a new instance of the DrawingCoordinateConverter class

### Methods
#### `Convert(ViewBase, ViewBase, Point)(...)`

Converts one coordinate point from one view's coordinate system to another view's coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/77a94c8e-e6bc-5d60-c364-c4b4e45d5eb4)

#### `Convert(ViewBase, ViewBase, PointList)(...)`

Converts a list of coordinate points from one view's coordinate system to another view's coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/cde655e6-530b-715c-d3a2-8ff1d6c7598b)

## InputDefinitionFactory (class)

The InputDefinitionFactory class contains helper functions for input definition handling.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3ce1ecfc-0320-5978-8235-52a3cd14c9a6)

### Constructors
- `InputDefinitionFactory(...)` — Initializes a new instance of the InputDefinitionFactory class

### Methods
#### `CreateInputDefinition(Tuple.DrawingObject, ViewBase.)(...)`

Creates a single-object input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/84a089fa-dc54-8a22-f6ff-d912b01dca5c)

#### `CreateInputDefinition(Tuple.Point, ViewBase.)(...)`

Creates a single-point input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/09f1de79-d1a5-3da0-368a-5c1e4a69174f)

#### `CreateInputDefinition(Tuple.PointList, ViewBase.)(...)`

Creates a multi-point input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/50ffa338-dfce-f96b-f27c-5509c6477d81)

#### `CreateInputDefinition(ViewBase, DrawingObject)(...)`

Creates a single-object input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/ccd25bad-30a2-87f1-a291-99531a837a39)

#### `CreateInputDefinition(ViewBase, Point)(...)`

Creates a single-point input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/c5bbd8b8-2fba-78bc-0ffd-67457e8144bf)

#### `CreateInputDefinition(ViewBase, Point, Point)(...)`

Creates a two-point input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/ae205985-792e-1c17-edff-e88331837cb9)

#### `CreateInputDefinition(ViewBase, Point, Point, Point)(...)`

Creates a three-point input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/6dbcb73d-a621-c67f-1779-ba20d8a800ea)

#### `CreateInputDefinition(ViewBase, PointList)(...)`

Creates a multi-point input definition.

[Docs](https://developer.tekla.com/topic/en/18/47/a169f862-3ab6-6393-4d2c-0919a848fc92)

#### `GetDrawingObject(...)`

Gets the object from the given single-object input.

[Docs](https://developer.tekla.com/topic/en/18/47/e07e0bd1-185a-2f57-6e97-7530043d6d89)

#### `GetPoint(...)`

Gets the point from the given single-point input.

[Docs](https://developer.tekla.com/topic/en/18/47/d118cf03-e535-2b81-6f9c-857de6299a7d)

#### `GetPoints(...)`

Gets the points from the given n-point input.

[Docs](https://developer.tekla.com/topic/en/18/47/d1cfbbc8-bf9f-043e-1964-539796a929cc)

#### `GetView(...)`

Gets the view that the given input belongs to.

[Docs](https://developer.tekla.com/topic/en/18/47/8581caba-017b-6552-f26e-bdd92e953ec0)

