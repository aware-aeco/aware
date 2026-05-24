---
name: tekla-plugin-sdk-fusion-features-panels
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Fusion.Features.Panels namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IContentPanel, LayoutPanelVisibilityToggleMode, PanelCloseAction, PanelSelectorNavigationAction, PanelsFeature, ToolTipConverter.
---

# Fusion.Features.Panels

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 6 types in this namespace.

## IContentPanel (interface)

IContentPanel interface in Fusion.Features.Panels.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Features.Panels.IContentPanel)

### Properties
- `Availability` (string, get) — Availability property of IContentPanel.
- `HeaderGeometryKey` (string, get) — HeaderGeometryKey property of IContentPanel.
- `HeaderTextKey` (string, get) — HeaderTextKey property of IContentPanel.
- `ViewName` (string, get) — ViewName property of IContentPanel.

## LayoutPanelVisibilityToggleMode (enum)

LayoutPanelVisibilityToggleMode enum in Fusion.Features.Panels.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Features.Panels.LayoutPanelVisibilityToggleMode)

### Values
- `Disabled` = `0`
- `SelectedOnly` = `1`
- `AllLayoutPanels` = `2`

## PanelCloseAction (enum)

PanelCloseAction enum in Fusion.Features.Panels.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Features.Panels.PanelCloseAction)

### Values
- `Close` = `0`
- `CloseAndDock` = `1`

## PanelSelectorNavigationAction (enum)

PanelSelectorNavigationAction enum in Fusion.Features.Panels.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Features.Panels.PanelSelectorNavigationAction)

### Values
- `Replace` = `0`
- `ToggleVisibility` = `1`

## PanelsFeature (class)

PanelsFeature class in Fusion.Features.Panels.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Features.Panels.PanelsFeature)

### Constructors
- `PanelsFeature()` — Constructs a new PanelsFeature.

### Methods
#### `Fusion.ViewModel CreatePanelSelector(object parameter)`

CreatePanelSelector method of PanelsFeature.

**Parameters:**
- `parameter` (object)

**Returns:** `Fusion.ViewModel` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.CreatePanelSelector%28System.Object%29)

#### `Fusion.ViewModel CreatePanels(object parameter)`

CreatePanels method of PanelsFeature.

**Parameters:**
- `parameter` (object)

**Returns:** `Fusion.ViewModel` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.CreatePanels%28System.Object%29)

#### `bool Panels_GetIsEnabled(string viewName)`

Panels_GetIsEnabled method of PanelsFeature.

**Parameters:**
- `viewName` (string)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_GetIsEnabled%28System.String%29)

#### `bool Panels_GetIsTabVisible(string viewName)`

Panels_GetIsTabVisible method of PanelsFeature.

**Parameters:**
- `viewName` (string)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_GetIsTabVisible%28System.String%29)

#### `System.Collections.Generic.IEnumerable<string> Panels_GetVisible()`

Panels_GetVisible method of PanelsFeature.

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_GetVisible)

#### `void Panels_Hide(string viewName)`

Panels_Hide method of PanelsFeature.

**Parameters:**
- `viewName` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_Hide%28System.String%29)

#### `void Panels_HideAll()`

Panels_HideAll method of PanelsFeature.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_HideAll)

#### `void Panels_HideAllLocations()`

Panels_HideAllLocations method of PanelsFeature.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_HideAllLocations)

#### `void Panels_HideAttached()`

Panels_HideAttached method of PanelsFeature.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_HideAttached)

#### `void Panels_HideDetached()`

Panels_HideDetached method of PanelsFeature.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_HideDetached)

#### `void Panels_HideLocation(string location)`

Panels_HideLocation method of PanelsFeature.

**Parameters:**
- `location` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_HideLocation%28System.String%29)

#### `void Panels_ResetLayout()`

Panels_ResetLayout method of PanelsFeature.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_ResetLayout)

#### `void Panels_ResetLayout(bool ignorePersistentState)`

Panels_ResetLayout method of PanelsFeature.

**Parameters:**
- `ignorePersistentState` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_ResetLayout%28System.Boolean%29)

#### `void Panels_SetIsEnabled(string viewName, bool enabled)`

Panels_SetIsEnabled method of PanelsFeature.

**Parameters:**
- `viewName` (string)
- `enabled` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_SetIsEnabled%28System.String%2CSystem.Boolean%29)

#### `void Panels_SetIsTabVisible(string viewName, bool tabVisible)`

Panels_SetIsTabVisible method of PanelsFeature.

**Parameters:**
- `viewName` (string)
- `tabVisible` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_SetIsTabVisible%28System.String%2CSystem.Boolean%29)

#### `void Panels_Show(string viewName, string location = null)`

Panels_Show method of PanelsFeature.

**Parameters:**
- `viewName` (string)
- `location` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_Show%28System.String%2CSystem.String%29)

#### `void Panels_Show(string viewName, string location = null, bool showWithOthers = false)`

Panels_Show method of PanelsFeature.

**Parameters:**
- `viewName` (string)
- `location` (string)
- `showWithOthers` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_Show%28System.String%2CSystem.String%2CSystem.Boolean%29)

#### `void Panels_ShowAllLocations()`

Panels_ShowAllLocations method of PanelsFeature.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_ShowAllLocations)

#### `void Panels_ShowLocation(string location)`

Panels_ShowLocation method of PanelsFeature.

**Parameters:**
- `location` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Panels_ShowLocation%28System.String%29)

#### `void Stopping(System.ComponentModel.CancelEventArgs stopping)`

Stopping method of PanelsFeature.

**Parameters:**
- `stopping` (System.ComponentModel.CancelEventArgs)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.PanelsFeature.Stopping%28System.ComponentModel.CancelEventArgs%29)

### Properties
- `FloatPanelsAfterDockingInit` (bool, get) — FloatPanelsAfterDockingInit property of PanelsFeature.
- `FloatPanelsOnStartUp` (bool, get) — FloatPanelsOnStartUp property of PanelsFeature.
- `HidePanelsOnStartUp` (bool, get) — HidePanelsOnStartUp property of PanelsFeature.
- `PanelCloseAction` (Fusion.Features.Panels.PanelCloseAction, get) — PanelCloseAction property of PanelsFeature.
- `PanelSelectorNavigationAction` (Fusion.Features.Panels.PanelSelectorNavigationAction, get) — PanelSelectorNavigationAction property of PanelsFeature.
- `PersistPanelLayout` (bool, get) — PersistPanelLayout property of PanelsFeature.
- `RespectVisiblePanels` (bool, get) — RespectVisiblePanels property of PanelsFeature.

## ToolTipConverter (class)

ToolTipConverter class in Fusion.Features.Panels.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Features.Panels.ToolTipConverter)

### Constructors
- `ToolTipConverter()` — Constructs a new ToolTipConverter.

### Methods
#### `object Convert(object value, System.Type targetType, object parameter, System.Globalization.CultureInfo culture)`

Convert method of ToolTipConverter.

**Parameters:**
- `value` (object)
- `targetType` (System.Type)
- `parameter` (object)
- `culture` (System.Globalization.CultureInfo)

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.ToolTipConverter.Convert%28System.Object%2CSystem.Type%2CSystem.Object%2CSystem.Globalization.CultureInfo%29)

#### `object ConvertBack(object value, System.Type targetType, object parameter, System.Globalization.CultureInfo culture)`

ConvertBack method of ToolTipConverter.

**Parameters:**
- `value` (object)
- `targetType` (System.Type)
- `parameter` (object)
- `culture` (System.Globalization.CultureInfo)

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Features.Panels.ToolTipConverter.ConvertBack%28System.Object%2CSystem.Type%2CSystem.Object%2CSystem.Globalization.CultureInfo%29)

