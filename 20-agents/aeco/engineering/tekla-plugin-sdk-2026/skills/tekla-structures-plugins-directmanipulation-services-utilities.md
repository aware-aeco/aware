---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services-utilities
description: This skill encodes the tekla-plugin-sdk 2026.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services.Utilities namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ComponentExtensions, ServiceFactory.
---

# Tekla.Structures.Plugins.DirectManipulation.Services.Utilities

Auto-generated from vendor docs for tekla-plugin-sdk 2026.0. 2 types in this namespace.

## ComponentExtensions (static-class)

The ComponentExtensions class adds extension methods to the Component type.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions)

### Methods
#### `static System.Collections.Generic.Dictionary<string, object> GetAppliedAttributes(Tekla.Structures.Model.Component component)`

Gets the applied attributes of the component.

**Parameters:**
- `component` (Tekla.Structures.Model.Component) — The component.

**Returns:** `System.Collections.Generic.Dictionary<string, object>` — The Dictionary`2 of applied attributes.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions.GetAppliedAttributes%28Tekla.Structures.Model.Component%29)

#### `static System.Collections.Generic.Dictionary<string, object> GetAppliedAttributes(Tekla.Structures.Model.CustomPart customPart)`

Gets the applied attributes of the custom part.

**Parameters:**
- `customPart` (Tekla.Structures.Model.CustomPart) — The custom part.

**Returns:** `System.Collections.Generic.Dictionary<string, object>` — The Dictionary`2 of applied attributes.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions.GetAppliedAttributes%28Tekla.Structures.Model.CustomPart%29)

#### `static System.Collections.Generic.List<Tekla.Structures.Geometry3d.Point> GetInputPoints(Tekla.Structures.Model.Component component)`

Gets the current input points from the component.

**Parameters:**
- `component` (Tekla.Structures.Model.Component) — The component.

**Returns:** `System.Collections.Generic.List<Tekla.Structures.Geometry3d.Point>` — A List`1 that contains the input points as of type Point.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions.GetInputPoints%28Tekla.Structures.Model.Component%29)

#### `static System.Collections.Generic.List<Tekla.Structures.Geometry3d.Point> GetInputPoints(Tekla.Structures.Model.CustomPart component)`

Gets the current input points from the custom part.

**Parameters:**
- `component` (Tekla.Structures.Model.CustomPart) — The custom part.

**Returns:** `System.Collections.Generic.List<Tekla.Structures.Geometry3d.Point>` — A List`1 that contains the input points as of type Point.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions.GetInputPoints%28Tekla.Structures.Model.CustomPart%29)

#### `static bool SetAppliedAttributes(Tekla.Structures.Model.Component component, System.Collections.Generic.Dictionary<string, object> attributes)`

Sets the applied attributes of the component.

**Parameters:**
- `component` (Tekla.Structures.Model.Component) — The component.
- `attributes` (System.Collections.Generic.Dictionary<string, object>) — The attributes.

**Returns:** `bool` — A Boolean to indicate whether the setting succeded.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions.SetAppliedAttributes%28Tekla.Structures.Model.Component%2CSystem.Collections.Generic.Dictionary%7BSystem.String%2CSystem.Object%7D%29)

#### `static bool SetAppliedAttributes(Tekla.Structures.Model.CustomPart customPart, System.Collections.Generic.Dictionary<string, object> attributes)`

Sets the applied attributes of the custom part.

**Parameters:**
- `customPart` (Tekla.Structures.Model.CustomPart) — The custom part.
- `attributes` (System.Collections.Generic.Dictionary<string, object>) — The attributes.

**Returns:** `bool` — A Boolean to indicate whether the setting succeded.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ComponentExtensions.SetAppliedAttributes%28Tekla.Structures.Model.CustomPart%2CSystem.Collections.Generic.Dictionary%7BSystem.String%2CSystem.Object%7D%29)

## ServiceFactory (static-class)

The ServiceFactory class provides utility factory methods for the API.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Utilities.ServiceFactory)

### Methods
#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Highlighter CreateHighlighter(Tekla.Structures.Plugins.DirectManipulation.Internal.Features.DirectManipulationApiFeatureBase feature)`

Creates a highlighter.

**Parameters:**
- `feature` (Tekla.Structures.Plugins.DirectManipulation.Internal.Features.DirectManipulationApiFeatureBase) — The feature.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Highlighter` — A Highlighter to highlight objects in the model.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ServiceFactory.CreateHighlighter%28Tekla.Structures.Plugins.DirectManipulation.Internal.Features.DirectManipulationApiFeatureBase%29)

#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool CreatePickingTool(Tekla.Structures.Plugins.DirectManipulation.Internal.Features.ComponentCreationFeatureBase feature, Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange inputRange, Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes inputTypes)`

Creates a picking tool.

**Parameters:**
- `feature` (Tekla.Structures.Plugins.DirectManipulation.Internal.Features.ComponentCreationFeatureBase) — The feature.
- `inputRange` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange) — The input range.
- `inputTypes` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes) — The input types.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool` — A picking tool.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ServiceFactory.CreatePickingTool%28Tekla.Structures.Plugins.DirectManipulation.Internal.Features.ComponentCreationFeatureBase%2CTekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange%2CTekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes%29)

#### `static Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool CreatePickingTool(Tekla.Structures.Plugins.DirectManipulation.Internal.Features.CustomPartCreationFeatureBase feature, Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange inputRange, Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes inputTypes)`

Creates a picking tool.

**Parameters:**
- `feature` (Tekla.Structures.Plugins.DirectManipulation.Internal.Features.CustomPartCreationFeatureBase) — The feature.
- `inputRange` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange) — The input range.
- `inputTypes` (Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes) — The input types.

**Returns:** `Tekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.PickingTool` — A picking tool.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Utilities.ServiceFactory.CreatePickingTool%28Tekla.Structures.Plugins.DirectManipulation.Internal.Features.CustomPartCreationFeatureBase%2CTekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputRange%2CTekla.Structures.Plugins.DirectManipulation.Services.Tools.Picking.InputTypes%29)

