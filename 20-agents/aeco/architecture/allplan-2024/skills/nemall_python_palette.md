---
name: allplan-nemall_python_palette
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_Palette namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: FixtureProperties, PaletteCtrlType, PaletteValueType, PythonWpfPalette, PythonWpfPaletteBuilder.
---

# NemAll_Python_Palette

Auto-generated from vendor docs for allplan 2024.0. 5 types in this namespace.

## FixtureProperties (class)

(No description provided in vendor docs for NemAll_Python_Palette.FixtureProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/)

### Constructors
- `FixtureProperties() | FixtureProperties(pathShortcut, group, element) | FixtureProperties(FixtureProperties)` — Initialize

### Methods
#### `GetElement()`

Get the element

**Remarks:** Get the element

**Returns:** `str` — Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetElement)

#### `GetGroup()`

Get the group

**Remarks:** Get the group

**Returns:** `str` — Group

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetGroup)

#### `GetLastFixturePath()`

Get the last fixture path

**Remarks:** Get the last fixture path

**Returns:** `str` — Last fixture path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetLastFixturePath)

#### `GetPath()`

Get the path

**Remarks:** Get the path

**Returns:** `str` — Path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetPath)

#### `OpenFixtureDialog(fixturePath)`

Open the symbol library dialog

**Remarks:** Open the symbol library dialog

**Parameters:**
- `fixturePath` (str) — Path to .sym file

**Returns:** `str` — result path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.OpenFixtureDialog)

#### `__eq__(other)`

Comparison of fixture properties.

**Remarks:** Comparison of fixture properties.

**Parameters:**
- `other` (FixtureProperties) — Compared fixture properties.

**Returns:** `object` — True when fixture properties are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.__repr__)

### Properties
- `Element` (None, get) — Property access for the element :type: None
- `Group` (None, get) — Property access for the group :type: None
- `Path` (None, get) — Property access for the path :type: None
- `PathShortcut` (None, get) — Property access for the path shortcut :type: None

## PaletteCtrlType (enum)

Types of the palette controls

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PaletteCtrlType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PaletteCtrlType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PaletteCtrlType/#NemAll_Python_Palette.PaletteCtrlType.__getitem__)

### Values
- `ARRAYBUTTON` = `26`
- `BUTTON` = `9`
- `CANVAS` = `32`
- `CHECK` = `4`
- `COMBO` = `5`
- `COMBOCOLOR` = `6`
- `COMBODRAWINGTYPE` = `19`
- `COMBOFACESTYLE` = `18`
- `COMBOHATCH` = `17`
- `COMBOLAYER` = `13`
- `COMBOMATNAME` = `14`
- `COMBOPATTERN` = `16`
- `COMBOPEN` = `7`
- `COMBOREINFBARDIAM` = `23`
- `COMBOREINFBARGRADE` = `22`
- `COMBOREINFCONCRGRADE` = `21`
- `COMBOREINFMESH` = `25`
- `COMBOREINFMESHGROUP` = `24`
- `COMBOREINFNORM` = `20`
- `COMBOSTROKE` = `8`
- `EDIT` = `3`
- `ELLIPS` = `29`
- `GROUPBOX` = `10`
- `INFIELD` = `2`
- `LINE` = `28`
- `PICTURE` = `1`
- `PICTUREBUTTON` = `15`
- `PICTURELIST` = `12`
- `POLY` = `31`
- `RECT` = `27`
- `STATIC` = `0`
- `TEXT` = `30`

## PaletteValueType (enum)

Types of the palette values

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PaletteValueType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PaletteValueType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PaletteValueType/#NemAll_Python_Palette.PaletteValueType.__getitem__)

### Values
- `ANGLE` = `2`
- `DOUBLE` = `6`
- `INTEGER` = `5`
- `LENGTH` = `1`
- `STRING` = `3`

## PythonWpfPalette (class)

(No description provided in vendor docs for NemAll_Python_Palette.PythonWpfPalette.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/)

### Constructors
- `PythonWpfPalette()` — Constructor

### Methods
#### `Close()`

Close the palette

**Remarks:** Close the palette

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.Close)

#### `EnableControl(arg2, name, bEnabled)`

Enable/disable a control

**Remarks:** Enable/disable a control

**Parameters:**
- `name` (int) — Name of the control
- `bEnabled` (bool) — Enabled: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.EnableControl)

#### `GetPythonWpfPaletteBuilder()`

Get the palette builder

**Remarks:** Get the palette builder

**Returns:** `PythonWpfPaletteBuilder` — Palette builder

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.GetPythonWpfPaletteBuilder)

#### `Open(arg2, title, partName, dataColumnWidth, showCloseButton, doc)`

Open the palette

**Remarks:** Open the palette

**Parameters:**
- `title` (str) — Title
- `partName` (int) — Name of the PythonPart
- `dataColumnWidth` (bool) — Width of the data column
- `showCloseButton` (DocumentAdapter) — Show the close button
- `doc` (str) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.Open)

#### `Reset(clearOnlyPages)`

Reset the palette for a full refresh

**Remarks:** Reset the palette for a full refresh

**Parameters:**
- `clearOnlyPages` (bool) — Reset only pages, keep other dialog data settings

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.Reset)

#### `UpdateDialogData(arg2)`

Refresh palette with current dialog data

**Remarks:** Refresh palette with current dialog data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.UpdateDialogData)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.__repr__)

## PythonWpfPaletteBuilder (class)

Implementation of the Python WPF palette builder

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/)

### Constructors
- `PythonWpfPaletteBuilder(element)` — Copy constructor

### Methods
#### `AddAngleValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add a angle value to the palette

**Remarks:** Add a angle value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddAngleValue)

#### `AddAreaFixtureCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Args: description: name: value: page: expanderName: rowName: bEnabled: height: width: fontFaceCode:

**Remarks:** Args: description: name: value: page: expanderName: rowName: bEnabled: height: width: fontFaceCode:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddAreaFixtureCatalogRef)

#### `AddAreaValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add a area value to the palette

**Remarks:** Add a area value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddAreaValue)

#### `AddBarDiameter(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add an bar diameter value to the palette

**Remarks:** Add an bar diameter value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddBarDiameter)

#### `AddBendingRollerValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a bending roller value to the palette

**Remarks:** Add a bending roller value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddBendingRollerValue)

#### `AddBrickTileCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a brick/tile catalog reference

**Remarks:** Add a brick/tile catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddBrickTileCatalogRef)

#### `AddButton(description, name, eventId, page, expanderName, rowName, bEnabled, height, width, fontStyle, fontFaceCode)`

Add a button to the palette

**Remarks:** Add a button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `eventId` (int) — Value holds the event ID pressing the button
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontStyle` (int) — Font size: 0=small, 1=extra small, 2=large
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddButton)

#### `AddCheckboxValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a checkbox value to the palette

**Remarks:** Add a checkbox value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddCheckboxValue)

#### `AddColorValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a color value to the palette

**Remarks:** Add a color value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddColorValue)

#### `AddComboBoxValue(description, name, value, listValues, ctrlType, valueType, page, expanderName, rowName, bEnabled, height, width, fontFaceCode, backgroundColor)`

Add a combo box value to the palette

**Remarks:** Add a combo box value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value
- `listValues` (str) — List values
- `ctrlType` (PaletteCtrlType) — Control type
- `valueType` (PaletteValueType) — Value type
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList])

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddComboBoxValue)

#### `AddConcreteCoverValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a concrete cover value to the palette

**Remarks:** Add a concrete cover value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddConcreteCoverValue)

#### `AddConcreteGrade(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a concrete grade value to the palette

**Remarks:** Add a concrete grade value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddConcreteGrade)

#### `AddConcreteGradeCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a concreteGrade catalog reference

**Remarks:** Add a concreteGrade catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddConcreteGradeCatalogRef)

#### `AddDoubleValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add a double value to the palette

**Remarks:** Add a double value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddDoubleValue)

#### `AddFaceStyleValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a face style combobox to the palette

**Remarks:** Add a face style combobox to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — ID name
- `value` (int) — Selected value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFaceStyleValue)

#### `AddFactoryCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a factory precastcatalog reference

**Remarks:** Add a factory precastcatalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFactoryCatalogRef)

#### `AddFixtureCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a fixture precastcatalog reference - all, only point, line or area

**Remarks:** Add a fixture precastcatalog reference - all, only point, line or area

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFixtureCatalogRef)

#### `AddFixtureValues(descriptionPath, descriptionGroup, descriptionElement, name, fixture, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add the fixture values

**Remarks:** Add the fixture values

**Parameters:**
- `descriptionPath` (str) — Description of the path value
- `descriptionGroup` (str) — Description of the group value
- `descriptionElement` (str) — Description of the element value
- `name` (str) — Value name
- `fixture` (FixtureProperties) — Properties of the fixture
- `page` (int) — Pate
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFixtureValues)

#### `AddHatchValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a hatch combobox to the palette

**Remarks:** Add a hatch combobox to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — ID name
- `value` (int) — Selected value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddHatchValue)

#### `AddInsulationCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a insulation catalog reference

**Remarks:** Add a insulation catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddInsulationCatalogRef)

#### `AddIntValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add an integer value to the palette

**Remarks:** Add an integer value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddIntValue)

#### `AddLayer(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a layer value to the palette

**Remarks:** Add a layer value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddLayer)

#### `AddLengthValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add a length value to the palette

**Remarks:** Add a length value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddLengthValue)

#### `AddLineFixtureCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Args: description: name: value: page: expanderName: rowName: bEnabled: height: width: fontFaceCode:

**Remarks:** Args: description: name: value: page: expanderName: rowName: bEnabled: height: width: fontFaceCode:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddLineFixtureCatalogRef)

#### `AddMaterialButton(description, name, value, buttonType, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a material button to the palette

**Remarks:** Add a material button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — String of material
- `buttonType` (int) — Button type (0: simple material button, 1: mat button + switch off button)
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMaterialButton)

#### `AddMeshGroup(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a mesh group value to the palette

**Remarks:** Add a mesh group value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMeshGroup)

#### `AddMeshType(description, name, value, page, expanderName, rowName, bEnabled, meshGroup, height, width, fontFaceCode)`

Add a mesh type value to the palette

**Remarks:** Add a mesh type value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `meshGroup` (int) — Mesh group
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMeshType)

#### `AddNormCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a norm catalog reference

**Remarks:** Add a norm catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddNormCatalogRef)

#### `AddPage(name, description)`

Add a page to the palette

**Remarks:** Add a page to the palette

**Parameters:**
- `name` (str) — ID name
- `description` (str) — Description text (localized)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPage)

#### `AddPatternValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a pattern combobox to the palette

**Remarks:** Add a pattern combobox to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — ID name
- `value` (int) — Selected value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPatternValue)

#### `AddPenValue(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a pen value to the palette

**Remarks:** Add a pen value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPenValue)

#### `AddPicture(description, name, pictureName, libPath, orientation, page, expanderName, rowName)`

Add a picture to the palette

**Remarks:** Add a picture to the palette

**Parameters:**
- `description` (str) — Description used for the tooltip
- `name` (str) — ID name
- `pictureName` (str) — Name of the picture
- `libPath` (str) — Library path
- `orientation` (int) — Orientation (0:LEFT, 1:MIDDLE, 2:RIGHT)
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPicture)

#### `AddPictureButton(description, name, value, eventId, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a picture button to the palette

**Remarks:** Add a picture button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value
- `eventId` (int) — Value holds the event ID pressing the button
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureButton)

#### `AddPictureButtonList(description, name, value, picturePath, pictureList, valueList, textList, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a picture button list to the palette

**Remarks:** Add a picture button list to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value holds the selected picture button in buttons
- `picturePath` (str) — Path of pictures
- `pictureList` (str) — Picture list holds the images for the buttons - example: a.png|b.png|c.png
- `valueList` (str) — Value list of possible values - example: 0|1|2
- `textList` (str) — Text list for the tooltips
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureButtonList)

#### `AddPictureComboBox(description, name, value, picturePath, pictureList, valueList, textList, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a picture combobox to the palette

**Remarks:** Add a picture combobox to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value holds the selected picture button in buttons
- `picturePath` (str) — Path of pictures
- `pictureList` (str) — Picture list holds the images for the buttons - example: a.png|b.png|c.png
- `valueList` (str) — Value list of possible values - example: 0|1|2
- `textList` (str) — Text list for the tooltips
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureComboBox)

#### `AddPictureResourceButton(description, name, value, eventId, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a picture button to the palette

**Remarks:** Add a picture button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value holds the resource ID
- `eventId` (int) — Value holds the event ID pressing the button
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceButton)

#### `AddPictureResourceButtonList(description, name, value, pictureList, valueList, textList, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a picture resource button list to the palette

**Remarks:** Add a picture resource button list to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value holds the selected picture button in buttons
- `pictureList` (str) — Picture list holds the images for the buttons - example: 16433|16441|16449
- `valueList` (str) — Value list of possible values - example: 0|1|2
- `textList` (str) — Text list for the tooltips
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceButtonList)

#### `AddPictureResourceComboBox(description, name, value, pictureList, valueList, textList, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a picture resource combobox to the palette

**Remarks:** Add a picture resource combobox to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value holds the selected picture button in buttons
- `pictureList` (str) — Picture list holds the images for the buttons - example: 16433|16441|16449
- `valueList` (str) — Value list of possible values - example: 0|1|2
- `textList` (str) — Text list for the tooltips
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceComboBox)

#### `AddPlaneReferencesButton(description, name, planeRefs, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a plane references button to the palette

**Remarks:** Add a plane references button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `planeRefs` (BasePlaneReferences) — Plane references
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPlaneReferencesButton)

#### `AddPointFixtureCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Args: description: name: value: page: expanderName: rowName: bEnabled: height: width: fontFaceCode:

**Remarks:** Args: description: name: value: page: expanderName: rowName: bEnabled: height: width: fontFaceCode:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPointFixtureCatalogRef)

#### `AddPrecastElementTypeCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a precastElementType catalog reference

**Remarks:** Add a precastElementType catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value int (name of the CatEntry)
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPrecastElementTypeCatalogRef)

#### `AddRadioButton(radioButtonGroupDescription, radioButtonGroupName, radioButtonDescription, value, selectedValueInGroup, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a radio button to the palette

**Remarks:** Add a radio button to the palette

**Parameters:**
- `radioButtonGroupDescription` (str) — Radio button group description
- `radioButtonGroupName` (str) — Radio button group ID name
- `radioButtonDescription` (str) — Radio button description
- `value` (object) — Double value of this radio button
- `selectedValueInGroup` (object) — Selected value of radio button group
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddRadioButton)

#### `AddRefPointButton(description, name, refPointId, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a reference point button to the palette

**Remarks:** Add a reference point button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `refPointId` (int) — Reference point ID (1,...,9)
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddRefPointButton)

#### `AddSeparator(page, expanderName)`

Add a separator to the palette

**Remarks:** Add a separator to the palette

**Parameters:**
- `page` (int) — Page index
- `expanderName` (str) — Expander section name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddSeparator)

#### `AddSteelGrade(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a steel grade value to the palette

**Remarks:** Add a steel grade value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddSteelGrade)

#### `AddStringValue(description, name, str, page, expanderName, rowName, bEnabled, height, width, fontFaceCode, backgroundColor)`

Add a string value to the palette

**Remarks:** Add a string value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `str` (str) — String
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList])

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddStringValue)

#### `AddStroke(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a stroke value to the palette

**Remarks:** Add a stroke value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddStroke)

#### `AddSurfaceCatalogRef(description, name, value, page, expanderName, rowName, bEnabled, height, width, fontFaceCode)`

Add a norm catalog reference

**Remarks:** Add a norm catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value string
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false height Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddSurfaceCatalogRef)

#### `AddText(description, value, page, expanderName, rowName, bEnabled, height, width, fontStyle, fontFaceCode)`

Add a text

**Remarks:** Add a text

**Parameters:**
- `description` (str) — Description
- `value` (str) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontStyle` (int) — Font size: 0=small, 1=extra small, 2=large
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddText)

#### `AddVolumeValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add a volume value to the palette

**Remarks:** Add a volume value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddVolumeValue)

#### `AddWeightValue(description, name, value, page, expanderName, rowName, bEnabled, minValue, maxValue, intervalValue, asSlider, height, width, fontFaceCode, backgroundColor)`

Add a weight value to the palette

**Remarks:** Add a weight value to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (float) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `minValue` (float) — Minimal value
- `maxValue` (float) — Maximal value
- `intervalValue` (float) — Interval value for the slider
- `asSlider` (bool) — Show as slider: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline
- `backgroundColor` ([list[int] | VecIntList]) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddWeightValue)

#### `IsConcreteCoverPaletteUpdate(cover)`

Check for a palette update due to a new concrete cover

**Remarks:** Check for a palette update due to a new concrete cover

**Parameters:**
- `cover` (float) — Concrete cover

**Returns:** `bool` — Palette update: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.IsConcreteCoverPaletteUpdate)

#### `Reset()`

Reset the data

**Remarks:** Reset the data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.Reset)

