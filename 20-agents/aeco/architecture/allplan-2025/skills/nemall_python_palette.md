---
name: allplan-nemall_python_palette
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_Palette namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, FixtureProperties, Orientation, PaletteCtrlType, PaletteValueType, PythonWpfPalette, RefPointButtonType, RefPointPosition, and 1 more types.
---

# NemAll_Python_Palette

Auto-generated from vendor docs for allplan 2025.0. 9 types in this namespace.

## FixtureProperties (class)

(No description provided in vendor docs for NemAll_Python_Palette.FixtureProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/)

### Constructors
- `FixtureProperties()` — Initialize
- `FixtureProperties(pathShortcut: str, group: str, element: str)` — Constructor
- `FixtureProperties(FixtureProperties: FixtureProperties)` — Copy Constructor

### Methods
#### `GetElement() -> str`

Get the element

**Remarks:** Get the element

**Returns:** `str` — Element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetElement)

#### `GetGroup() -> str`

Get the group

**Remarks:** Get the group

**Returns:** `str` — Group

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetGroup)

#### `GetLastFixturePath() -> str`

Get the last fixture path

**Remarks:** Get the last fixture path

**Returns:** `str` — Last fixture path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetLastFixturePath)

#### `GetPath() -> str`

Get the path

**Remarks:** Get the path

**Returns:** `str` — Path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.GetPath)

#### `OpenFixtureDialog(fixturePath: str) -> str`

Open the symbol library dialog

**Remarks:** Open the symbol library dialog

**Parameters:**
- `fixturePath` (str) — Path to .sym file

**Returns:** `str` — result path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.OpenFixtureDialog)

#### `__eq__(other: FixtureProperties) -> object`

Comparison of fixture properties.

**Remarks:** Comparison of fixture properties.

**Parameters:**
- `other` (FixtureProperties) — Compared fixture properties.

**Returns:** `object` — True when fixture properties are equal, otherwise false.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/FixtureProperties/#NemAll_Python_Palette.FixtureProperties.__repr__)

### Properties
- `Element` (None, get) — Property access for the element :type: None
- `Group` (None, get) — Property access for the group :type: None
- `Path` (None, get) — Property access for the path :type: None
- `PathShortcut` (None, get) — Property access for the path shortcut :type: None

## Functions (static-class)

Module-level functions of NemAll_Python_Palette

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/)

### Methods
#### `GetPaletteDataColumnWidth() -> int`

Get the width of the property palette data column

**Remarks:** Get the width of the property palette data column

**Returns:** `int` — Width of the property palette data column

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/#NemAll_Python_Palette.GetPaletteDataColumnWidth)

## Orientation (enum)

Definition of the orientations

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/Orientation/)

### Values
- `eCenter` = `2`
- `eLeft` = `0`
- `eRight` = `1`

## PaletteCtrlType (enum)

Types of the palette controls

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PaletteCtrlType/)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PaletteValueType/)

### Values
- `ANGLE` = `2`
- `DOUBLE` = `6`
- `INTEGER` = `5`
- `LENGTH` = `1`
- `STRING` = `3`

## PythonWpfPalette (class)

Implementation of the WPF palette for python

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/)

### Constructors
- `PythonWpfPalette()` — Constructor

### Methods
#### `Close() -> str`

Close the palette

**Remarks:** Close the palette

**Returns:** `str` — Name of the active page

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.Close)

#### `EnableControl(name: str, iPage: int, bEnabled: bool)`

Enable/disable a control

**Remarks:** Enable/disable a control

**Parameters:**
- `name` (str) — Name of the control
- `iPage` (int) — Page index
- `bEnabled` (bool) — Enabled: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.EnableControl)

#### `GetPythonWpfPaletteBuilder() -> PythonWpfPaletteBuilder`

Get the palette builder

**Remarks:** Get the palette builder

**Returns:** `PythonWpfPaletteBuilder` — Palette builder

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.GetPythonWpfPaletteBuilder)

#### `Open( title: str, partName: str, dataColumnWidth: int, showCloseButton: bool, showFavoriteButtons: bool, doc: DocumentAdapter, activePageText: str, )`

Open the palette

**Remarks:** Open the palette

**Parameters:**
- `title` (str) — Title
- `partName` (str) — Name of the PythonPart
- `dataColumnWidth` (int) — Width of the data column
- `showCloseButton` (bool) — Show the close button state
- `showFavoriteButtons` (bool) — Show the favorites button state
- `doc` (DocumentAdapter) — Document
- `activePageText` (str) — Page text of the active page

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.Open)

#### `Reset(clearOnlyPages: bool = False)`

Reset the palette for a full refresh

**Remarks:** Reset the palette for a full refresh

**Parameters:**
- `clearOnlyPages` (bool) — Reset only pages, keep other dialog data settings

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.Reset)

#### `UpdateDialogData(activePage: int)`

Refresh palette with current dialog data

**Remarks:** Refresh palette with current dialog data

**Parameters:**
- `activePage` (int) — Index of the active page: (-1 = use current)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.UpdateDialogData)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPalette/#NemAll_Python_Palette.PythonWpfPalette.__repr__)

## PythonWpfPaletteBuilder (class)

Implementation of the Python WPF palette builder

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/)

### Constructors
- `PythonWpfPaletteBuilder(element: PythonWpfPaletteBuilder)` — Copy constructor

### Methods
#### `AddAngleValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddAngleValue)

#### `AddAreaFixtureCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add an area fixture precastcatalog reference - all, only point, line or area

**Remarks:** Add an area fixture precastcatalog reference - all, only point, line or area

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddAreaFixtureCatalogRef)

#### `AddAreaValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddAreaValue)

#### `AddBarDiameter( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddBarDiameter)

#### `AddBendingRollerValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddBendingRollerValue)

#### `AddBrickTileCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a brick/tile reference

**Remarks:** Add a brick/tile reference

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddBrickTileCatalogRef)

#### `AddButton( description: str, name: str, eventId: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontStyle: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddButton)

#### `AddCheckboxValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddCheckboxValue)

#### `AddColorValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddColorValue)

#### `AddComboBoxValue( description: str, name: str, value: str, listValues: str, ctrlType: PaletteCtrlType, valueType: PaletteValueType, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, isEditable: bool, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color
- `isEditable` (bool) — Is editable state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddComboBoxValue)

#### `AddConcreteCoverValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddConcreteCoverValue)

#### `AddConcreteGrade( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddConcreteGrade)

#### `AddConcreteGradeCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a concreteGrade reference

**Remarks:** Add a concreteGrade reference

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddConcreteGradeCatalogRef)

#### `AddDoubleValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddDoubleValue)

#### `AddFaceStyleValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFaceStyleValue)

#### `AddFactoryCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a factory precastcatalog reference

**Remarks:** Add a factory precastcatalog reference

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFactoryCatalogRef)

#### `AddFixtureCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFixtureCatalogRef)

#### `AddFixtureValues( descriptionPath: str, descriptionGroup: str, descriptionElement: str, name: str, fixture: FixtureProperties, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddFixtureValues)

#### `AddHatchValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddHatchValue)

#### `AddInsulationCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a insulation reference

**Remarks:** Add a insulation reference

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddInsulationCatalogRef)

#### `AddIntValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddIntValue)

#### `AddLayer( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddLayer)

#### `AddLengthValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddLengthValue)

#### `AddLineFixtureCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a line fixture precastcatalog reference - all, only point, line or area

**Remarks:** Add a line fixture precastcatalog reference - all, only point, line or area

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddLineFixtureCatalogRef)

#### `AddMaterialButton( description: str, name: str, value: str, buttonType: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMaterialButton)

#### `AddMeshGroup( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMeshGroup)

#### `AddMeshType( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, meshGroup: int, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMeshType)

#### `AddMultiMaterialLayoutCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, ) -> int`

Add a multimaterial layout catalog reference

**Remarks:** Add a multimaterial layout catalog reference

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
- `ex` (object) — of layout

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddMultiMaterialLayoutCatalogRef)

#### `AddNormCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a norm catalog reference

**Remarks:** Add a norm catalog reference

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddNormCatalogRef)

#### `AddPage(name: str, description: str)`

Add a page to the palette

**Remarks:** Add a page to the palette

**Parameters:**
- `name` (str) — ID name
- `description` (str) — Description text (localized)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPage)

#### `AddPatternValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPatternValue)

#### `AddPenValue( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPenValue)

#### `AddPicture( description: str, name: str, pictureName: str, libPath: str, orientation: Orientation, page: int, expanderName: str, rowName: str, )`

Add a picture to the palette

**Remarks:** Add a picture to the palette

**Parameters:**
- `description` (str) — Description used for the tooltip
- `name` (str) — ID name
- `pictureName` (str) — Name of the picture
- `libPath` (str) — Library path
- `orientation` (Orientation) — Orientation (0:LEFT, 1:MIDDLE, 2:RIGHT)
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPicture)

#### `AddPictureButton( description: str, name: str, value: str, eventId: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureButton)

#### `AddPictureButtonList( description: str, name: str, value: int, picturePath: str, pictureList: str, valueList: str, textList: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureButtonList)

#### `AddPictureComboBox( description: str, name: str, value: int, picturePath: str, pictureList: str, valueList: str, textList: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureComboBox)

#### `AddPictureResourceButton( description: str, name: str, value: int, eventId: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceButton)

#### `AddPictureResourceButtonList( description: str, name: str, value: int, pictureList: str, valueList: str, textList: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceButtonList)

#### `AddPictureResourceComboBox( description: str, name: str, value: int, pictureList: str, valueList: str, textList: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceComboBox)

#### `AddPictureResourceToggleButton( description: str, name: str, value: int, pictureList: str, valueList: str, textList: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a picture toggle button to the palette

**Remarks:** Add a picture toggle button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (int) — Value holds the selected picture button in buttons
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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPictureResourceToggleButton)

#### `AddPlaneReferencesButton( description: str, name: str, planeRefs: BasePlaneReferences, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPlaneReferencesButton)

#### `AddPointFixtureCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a point fixture precastcatalog reference - all, only point, line or area

**Remarks:** Add a point fixture precastcatalog reference - all, only point, line or area

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPointFixtureCatalogRef)

#### `AddPrecastElementTypeCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, ) -> str`

Add a precastElementType catalog reference

**Remarks:** Add a precastElementType catalog reference

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `value` (str) — Value
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddPrecastElementTypeCatalogRef)

#### `AddRadioButton( radioButtonGroupDescription: str, radioButtonGroupName: str, radioButtonDescription: str, value: object, selectedValueInGroup: object, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddRadioButton)

#### `AddRefPointButton( description: str, name: str, refPointPosition: RefPointPosition, refPointType: RefPointButtonType, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a reference point button to the palette

**Remarks:** Add a reference point button to the palette

**Parameters:**
- `description` (str) — Description
- `name` (str) — Value name
- `refPointPosition` (RefPointPosition) — Reference point ID (1,...,9)
- `refPointType` (RefPointButtonType) — Reference point type
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `bEnabled` (bool) — Control is enabled: true/false
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row
- `fontFaceCode` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddRefPointButton)

#### `AddResourcePicture( description: str, name: str, pictureResourceID: int, page: int, expanderName: str, rowName: str, height: int, width: int, )`

Add a picture from a resource to the palette

**Remarks:** Add a picture from a resource to the palette

**Parameters:**
- `description` (str) — Description used for the tooltip
- `name` (str) — ID name
- `pictureResourceID` (int) — Resource id of the picture
- `page` (int) — Page index
- `expanderName` (str) — Expander section name
- `rowName` (str) — Name of the row
- `height` (int) — Control height, only used for a row
- `width` (int) — Control width, only used for a row

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddResourcePicture)

#### `AddSeparator(page: int, expanderName: str)`

Add a separator to the palette

**Remarks:** Add a separator to the palette

**Parameters:**
- `page` (int) — Page index
- `expanderName` (str) — Expander section name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddSeparator)

#### `AddSteelGrade( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddSteelGrade)

#### `AddStringValue( description: str, name: str, str: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddStringValue)

#### `AddStroke( description: str, name: str, value: int, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddStroke)

#### `AddSurfaceCatalogRef( description: str, name: str, value: str, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontFaceCode: int, )`

Add a Surface catalog reference

**Remarks:** Add a Surface catalog reference

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddSurfaceCatalogRef)

#### `AddText( description: str, value: str, orientation: Orientation, page: int, expanderName: str, rowName: str, bEnabled: bool, height: int, width: int, fontStyle: int, fontFaceCode: int, )`

Add a text

**Remarks:** Add a text

**Parameters:**
- `description` (str) — Description
- `value` (str) — Value
- `orientation` (Orientation) — Page index
- `page` (int) — Expander section name
- `expanderName` (str) — Name of the row
- `rowName` (str) — Control is enabled: true/false
- `bEnabled` (bool) — Control height, only used for a row
- `height` (int) — Control width, only used for a row
- `width` (int) — Font size: 0=small, 1=extra small, 2=large
- `fontStyle` (int) — Face code: 0=normal, 1=bold, 2=italic, 4=underline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddText)

#### `AddVolumeValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddVolumeValue)

#### `AddWeightValue( description: str, name: str, value: float, page: int, expanderName: str, rowName: str, bEnabled: bool, minValue: float, maxValue: float, intervalValue: float, asSlider: bool, height: int, width: int, fontFaceCode: int, backgroundColor: list[int] | VecIntList, )`

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
- `backgroundColor` (list[int] | VecIntList) — Background color of the control as red, green, blue

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.AddWeightValue)

#### `IsConcreteCoverPaletteUpdate(cover: float) -> bool`

Check for a palette update due to a new concrete cover

**Remarks:** Check for a palette update due to a new concrete cover

**Parameters:**
- `cover` (float) — Concrete cover

**Returns:** `bool` — Palette update: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.IsConcreteCoverPaletteUpdate)

#### `Reset()`

Reset the data

**Remarks:** Reset the data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/PythonWpfPaletteBuilder/#NemAll_Python_Palette.PythonWpfPaletteBuilder.Reset)

## RefPointButtonType (enum)

Definition of available positions of the reference point on a reference point button.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/RefPointButtonType/)

### Values
- `eAllNinePositions` = `0` — All nine positions of the reference point are available.
- `eAllTenPositions` = `6` — All nine positions of the reference point are available, plus tenth option. of an undefined point.
- `eCorners` = `2` — Only four positions in the corners possible.
- `eCornersCenter` = `3` — Five positions possible. Four in the corners plus in the center.
- `eLeftRightCenter` = `4` — Three positions possible. Center-left, center-center and center-right.
- `eNoCorners` = `1` — Five positions possible. In the middle and on the center of each edge.
- `eTopBottomCenter` = `5` — Three positions possible. Top-center, center-center and bottom-center.

## RefPointPosition (enum)

Definition of the reference point positions get from the RefPointButton

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Palette/RefPointPosition/)

### Values
- `eBottomCenter` = `8`
- `eBottomLeft` = `7`
- `eBottomRight` = `9`
- `eCenterCenter` = `5`
- `eCenterLeft` = `4`
- `eCenterRight` = `6`
- `eNone` = `0`
- `eTopCenter` = `2`
- `eTopLeft` = `1`
- `eTopRight` = `3`

