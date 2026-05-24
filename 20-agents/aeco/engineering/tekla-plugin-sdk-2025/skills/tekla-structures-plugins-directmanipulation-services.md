---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DimensionEndPointSizeType, LineType, SurfaceType, TextRepresentationTypes.
---

# Tekla.Structures.Plugins.DirectManipulation.Services

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 4 types in this namespace.

## DimensionEndPointSizeType (enum)

The DimensionEndPointSizeType enum defines the dimension end point size types. The size can be dynamic or fixed.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.DimensionEndPointSizeType)

### Values
- `Dynamic` = `0` — The size is based on the dimension value of the associated measurement graphic.
- `FixedExtraSmall` = `1` — The size is extra small. The size does not depend on the dimension value.
- `FixedSmall` = `2` — The size is small. The size does not depend on the dimension value.
- `FixedMedium` = `3` — The size is medium. The size does not depend on the dimension value.
- `FixedLarge` = `4` — The size is large. The size does not depend on the dimension value.
- `FixedExtraLarge` = `5` — The size is extra large. The size does not depend on the dimension value.

## LineType (enum)

The LineType enum defines what a line or curve represents. This will affect the line color and width.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.LineType)

### Values
- `GeometryPreview` = `0` — The line is a representation of the final result after the modification. This could be for example an edge or a mid-line of a solid resulting from the modification.
- `Error` = `1` — The line represents an error.
- `Highlight` = `2` — The line is highlighted in the model.

## SurfaceType (enum)

The SurfaceType enum defines what the surface represents.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.SurfaceType)

### Values
- `GeometryPreview` = `0` — The surface is a representation of the final result after the modification. This could be for example one or more surfaces of the solid resulting from the modification.
- `Highlight` = `1` — The surface is highlighted in the model.

## TextRepresentationTypes (enum)

The TextRepresentationTypes enum defines what a text represents. This affects the text color, size and formatting in a way that is then consistent in the GUI.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.TextRepresentationTypes)

### Values
- `Label` = `1` — The text represents a label for an object.
- `Measurement` = `2` — The text represents a measurement.
- `Angle` = `4` — The text represents an angle.
- `Message` = `8` — The text is a message for the user.
- `Warning` = `16` — The text is a warning to the user.
- `Debug` = `32` — The text is intended for debug purposes.
- `Interactive` = `64` — The text is interactive.

