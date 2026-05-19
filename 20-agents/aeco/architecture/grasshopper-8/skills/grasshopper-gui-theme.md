---
name: grasshopper-grasshopper-gui-theme
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.GUI.Theme namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_BackgroundSettings, GH_ObjectSettings, GH_PaletteSettings, GH_PageSettings, GH_Theme, GH_WireSettings, GH_BackgroundStyle.
---

# Grasshopper.GUI.Theme

Auto-generated from vendor docs for grasshopper 8.0. 7 types in this namespace.

## GH_BackgroundSettings (class)

(No description provided in vendor docs for Grasshopper.GUI.Theme.GH_BackgroundSettings.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_BackgroundSettings.htm)

### Properties
- `Colour1` (Color, get/set) — First background colour. Depending on the background style, this colour is used in different ways.
- `Colour2` (Color, get/set) — Second background colour. Depending on the background style, this colour is used in different ways.
- `Hatch` (HatchStyle, get/set) — Background hatch brush style. This property is only used if BackgroundStyle is set to GH_ThemeBackgroundStyle.Hatch.
- `Style` (GH_BackgroundStyle, get/set) — Canvas monochromatic flag. If True, the canvas background is a solid colour.

## GH_BackgroundStyle (enum)

Enumerates the pre-defined canvas background themes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_BackgroundStyle.htm)

### Values
- `Solid` = `0` — Canvas background will be a single, solid colour.
- `Hatch` = `1` — Canvas background will be a GDI hatch brush.
- `GradientLeftRight` = `10` — Canvas background will be a linear horizontal gradient.
- `GradientTopBottom` = `11` — Canvas background will be a linear vertical gradient.

## GH_ObjectSettings (class)

(No description provided in vendor docs for Grasshopper.GUI.Theme.GH_ObjectSettings.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_ObjectSettings.htm)

### Properties
- `GroupColour` (Color, get/set) — Gets or sets the default fill colour for group objects.
- `PanelColour` (Color, get/set) — Gets or sets the default fill colour for panel objects.
- `ZuiEdgeColour` (Color, get/set) — Gets or sets the edge colour for ZUI elements.
- `ZuiFillColour` (Color, get/set) — Gets or sets the fill colour for ZUI elements.

## GH_PageSettings (class)

(No description provided in vendor docs for Grasshopper.GUI.Theme.GH_PageSettings.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_PageSettings.htm)

### Properties
- `DrawGrid` (Boolean, get/set) — Gets or sets whether the grid ought to be drawn.
- `DrawPage` (Boolean, get/set) — Gets or sets whether the page ought to be drawn.
- `DrawShadow` (Boolean, get/set) — Gets or sets whether the page shadow ought to be drawn. If the page is not drawn, neither is the page shadow, irregardless of this property.
- `EdgeColour` (Color, get/set) — Gets or sets the page edge colour to be used.
- `FillColour` (Color, get/set) — Gets or sets the page fill colour to be used.
- `GridColour` (Color, get/set) — Gets or sets the page grid colour to be used.
- `GridHeight` (Int32, get/set) — Gets or sets the height of a page grid cell.
- `GridPattern` (Single[], get/set) — Gets or sets the dash pattern to apply to grid lines. Leave null or empty for a solid line.
- `GridWidth` (Int32, get/set) — Gets or sets the width of a page grid cell.
- `ShadowColour` (Color, get/set) — Gets or set the page shadow colour to be used.
- `ShadowSize` (Int32, get/set) — Gets or sets the size of the page shadow.

## GH_PaletteSettings (class)

(No description provided in vendor docs for Grasshopper.GUI.Theme.GH_PaletteSettings.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_PaletteSettings.htm)

### Properties
- `ErrorSelected` (GH_PaletteStyle, get/set) — Gets or sets the palette style for selected Error palettes.
- `ErrorStandard` (GH_PaletteStyle, get/set) — Gets or sets the palette style for unselected Error palettes.
- `HiddenSelected` (GH_PaletteStyle, get/set) — Gets or sets the palette style for selected Hidden palettes.
- `HiddenStandard` (GH_PaletteStyle, get/set) — Gets or sets the palette style for unselected Hidden palettes.
- `LockedSelected` (GH_PaletteStyle, get/set) — Gets or sets the palette style for selected Locked palettes.
- `LockedStandard` (GH_PaletteStyle, get/set) — Gets or sets the palette style for unselected Locked palettes.
- `NormalSelected` (GH_PaletteStyle, get/set) — Gets or sets the palette style for selected Normal palettes.
- `NormalStandard` (GH_PaletteStyle, get/set) — Gets or sets the palette style for unselected Normal palettes.
- `WarningSelected` (GH_PaletteStyle, get/set) — Gets or sets the palette style for selected Warning palettes.
- `WarningStandard` (GH_PaletteStyle, get/set) — Gets or sets the palette style for unselected Warning palettes.

## GH_Theme (class)

(No description provided in vendor docs for Grasshopper.GUI.Theme.GH_Theme.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_Theme.htm)

### Constructors
- `public GH_Theme()` — Create a new default theme.
- `public GH_Theme(GH_Theme other)` — Make a duplicate of another theme.

### Methods
#### `public void LoadTheme()`

Instantiate all palette and gui defaults. This function reads the colour values out of grasshopper_gui.xml settings database if it exists.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Theme_GH_Theme_LoadTheme.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Theme_GH_Theme_Read.htm)

#### `public void SaveTheme()`

Store all palette and gui defaults. This function writes the colour values out to grasshopper_gui.xml settings database.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Theme_GH_Theme_SaveTheme.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Theme_GH_Theme_Write.htm)

### Properties
- `BackGround` (GH_BackgroundSettings, get) — Gets the theme background settings.
- `DefaultTheme` (GH_Theme, get) — 
- `Objects` (GH_ObjectSettings, get) — Gets the theme objects and ZUI settings.
- `Page` (GH_PageSettings, get) — Gets the theme page and grid settings.
- `Palettes` (GH_PaletteSettings, get) — Gets the theme palette settings.
- `Wires` (GH_WireSettings, get) — Gets the theme wire settings.

## GH_WireSettings (class)

(No description provided in vendor docs for Grasshopper.GUI.Theme.GH_WireSettings.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Theme_GH_WireSettings.htm)

### Properties
- `DefaultColour` (Color, get/set) — Gets or sets the colour to use for standard wires.
- `EmptyColour` (Color, get/set) — Gets or sets the colour to use for empty wires.
- `SelectedColour0` (Color, get/set) — Gets or sets the colour of a wire at the selected extreme.
- `SelectedColour1` (Color, get/set) — Gets or sets the colour of a wire at the unselected extreme.

