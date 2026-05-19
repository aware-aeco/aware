---
name: allplan-nemall_python_precast
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_Precast namespace — 48 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, Anchor, AllplanElement, AllowedElements, Direction, AssemblyGroupElement, AnchorBorderPosition, DirectionMode, and 40 more types.
---

# NemAll_Python_Precast

Auto-generated from vendor docs for allplan 2025.0. 48 types in this namespace.

## AllowedElements (enum)

allowed elements for visibility

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllowedElements/)

### Values
- `eAll` = `0`
- `eOnlyThese` = `1`
- `eTheseNot` = `2`

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/)

### Methods
#### `GetAttributes() -> object`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetAttributes)

#### `GetBaseElementAdapter() -> BaseElementAdapter`

Get the model element

**Remarks:** Get the model element

**Returns:** `BaseElementAdapter` — Model element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetBaseElementAdapter)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetCommonProperties)

#### `GetGeometryObject() -> object`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetGeometryObject)

#### `GetLabelElements() -> list`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetLabelElements)

#### `GetSubElementID() -> type`

Get the SubElementID

**Remarks:** Get the SubElementID

**Returns:** `type` — SubElementID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer: object)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey: str)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject: object)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements: list)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## Anchor (class)

(No description provided in vendor docs for NemAll_Python_Precast.Anchor.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Anchor/)

### Constructors
- `Anchor( id: int, fromId: int, fromPos: AnchorBorderPosition, toId: int, toPos: AnchorBorderPosition, )` — Constructor
- `Anchor( id: int, fromId: int, fromPos: AnchorBorderPosition, toId: int, toPos: AnchorBorderPosition, align: bool, offsetX: float, offsetY: float, )` — Constructor
- `Anchor( id: int, fromPos: AnchorBorderPosition, toId: int, toPos: AnchorBorderPosition, )` — Constructor
- `Anchor( id: int, fromPos: AnchorBorderPosition, toId: int, toPos: AnchorBorderPosition, offsetX: float, offsetY: float, )` — Constructor
- `Anchor( id: int, fromCell: Cell, fromPos: AnchorBorderPosition, toCell: Cell, toPos: AnchorBorderPosition, )` — Constructor
- `Anchor( id: int, fromCell: Cell, fromPos: AnchorBorderPosition, toCell: Cell, toPos: AnchorBorderPosition, align: bool, offsetX: float, offsetY: float, )` — Constructor

## AnchorBorderPosition (enum)

anchor border positions

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AnchorBorderPosition/)

### Values
- `eBorderBottom` = `2`
- `eBorderHorizontal` = `3`
- `eBorderInnerBottom` = `4098`
- `eBorderInnerHorizontal` = `4099`
- `eBorderInnerLeft` = `4100`
- `eBorderInnerRight` = `4104`
- `eBorderInnerTop` = `4097`
- `eBorderInnerVertical` = `4108`
- `eBorderLeft` = `4`
- `eBorderRight` = `8`
- `eBorderTop` = `1`
- `eBorderVertical` = `12`
- `eCornerInnerLeftBottom` = `4102`
- `eCornerInnerLeftTop` = `4101`
- `eCornerInnerRightBottom` = `4106`
- `eCornerInnerRightTop` = `4105`
- `eCornerLeftBottom` = `6`
- `eCornerLeftTop` = `5`
- `eCornerRightBottom` = `10`
- `eCornerRightTop` = `9`

## AssemblyGroupElement (class)

AssemblyGroupElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AssemblyGroupElement/)

### Constructors
- `AssemblyGroupElement()` — Initialize
- `AssemblyGroupElement( name: str, number: int, LibraryElementsList: list, FixtureElementsList: list, ReinforcementList: list, )` — Constructor

### Methods
#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/AssemblyGroupElement/#NemAll_Python_Precast.AssemblyGroupElement.__repr__)

### Properties
- `FixtureElementsList` (None, get) — List of parametric fixtures which should be included in the assembly group Value type: list :type: None
- `LibraryElementsList` (None, get) — List of library fixtures which should be included in the assembly group Value type: list :type: None
- `Name` (None, get) — Name of the assembly group Value type: str :type: None
- `Number` (None, get) — Number of the assembly group Value type: int :type: None
- `ReinforcementList` (None, get) — List of reinforcement placements which should be included in the assembly group Value type: list :type: None

## Direction (enum)

directions for views

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Direction/)

### Values
- `eDirectionI` = `1`
- `eDirectionII` = `2`
- `eDirectionIII` = `3`
- `eDirectionIV` = `4`
- `eDirectionV` = `5`
- `eDirectionVI` = `6`

## DirectionMode (enum)

mode for directions

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/DirectionMode/)

### Values
- `eFull` = `1`
- `eRestricted` = `0`

## FileEntryPath (enum)

paths

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FileEntryPath/)

### Values
- `eFileEntryPathLibrary` = `5`
- `eFileEntryPathOffice` = `1`
- `eFileEntryPathPrivate` = `2`
- `eFileEntryPathProject` = `3`
- `eFileEntryPathProjectPlus` = `4`
- `eFileEntryPathStandard` = `0`

## FixtureCombinationType (enum)

Fixture combination types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureCombinationType/)

### Values
- `eVx` = `1`
- `eVy` = `2`
- `eVz` = `3`

## FixtureElement (class)

FixtureElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/)

### Constructors
- `FixtureElement()` — Initialize
- `FixtureElement(fixProp: FixtureProperties, slideList: list)` — Constructor

### Methods
#### `GetFixtureProperties() -> FixtureProperties`

Get the Fixture properties

**Remarks:** Get the Fixture properties

**Returns:** `FixtureProperties` — Fixture properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.GetFixtureProperties)

#### `GetHash() -> str`

Get the hash value

**Remarks:** Get the hash value

**Returns:** `str` — Hash value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.GetHash)

#### `GetSlideList() -> list`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.GetSlideList)

#### `SetFixtureProperties(fixProp: FixtureProperties)`

Set the Fixture properties

**Remarks:** Set the Fixture properties

**Parameters:**
- `fixProp` (FixtureProperties) — fixture properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.SetFixtureProperties)

#### `SetHash(hash: str)`

Set the hash value

**Remarks:** Set the hash value

**Parameters:**
- `hash` (str) — Hash value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.SetHash)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.__repr__)

## FixtureGroupElement (class)

FixtureGroupElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/)

### Constructors
- `FixtureGroupElement()` — Initialize
- `FixtureGroupElement(FixtureGroupProp: FixtureGroupProperties, slideList: list)` — Constructor
- `FixtureGroupElement( commonProp: CommonProperties, FixtureGroupProp: FixtureGroupProperties, slideList: list, )` — Constructor

### Methods
#### `GetFixtureGroupProperties() -> FixtureGroupProperties`

Get the FixtureGroup properties

**Remarks:** Get the FixtureGroup properties

**Returns:** `FixtureGroupProperties` — FixtureGroup properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.GetFixtureGroupProperties)

#### `GetFixtureList() -> list`

Get the fixture object list

**Remarks:** Get the fixture object list

**Returns:** `list` — Fixture object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.GetFixtureList)

#### `SetFixtureGroupProperties(FixtureGroupProp: FixtureGroupProperties)`

Set the FixtureGroup properties

**Remarks:** Set the FixtureGroup properties

**Parameters:**
- `FixtureGroupProp` (FixtureGroupProperties) — FixtureGroup properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.SetFixtureGroupProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.__repr__)

## FixtureGroupProperties (class)

FixtureGroupProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupProperties/)

### Constructors
- `FixtureGroupProperties()` — Initialize

### Methods
#### `__eq__(prop: FixtureGroupProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixtureGroupProperties) — FixtureGroupProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupProperties/#NemAll_Python_Precast.FixtureGroupProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupProperties/#NemAll_Python_Precast.FixtureGroupProperties.__repr__)

### Properties
- `LeadingPoint` (None, get) — Leading point of the fixture group Value type: Point3D :type: None
- `Name` (None, get) — Name of the fixture group Value type: str :type: None
- `Type` (None, get) — Type of the fixture group (General|Dynamic|Cutted|Leading) Value type: enum :type: None

## FixturePlacementElement (class)

FixturePlacementElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/)

### Constructors
- `FixturePlacementElement()` — Initialize
- `FixturePlacementElement( commonProp: CommonProperties, macroPlacementProp: FixturePlacementProperties, macro: object, )` — Constructor

### Methods
#### `GetFixturePlacementProperties() -> FixturePlacementProperties`

Get the MacroPlacement properties

**Remarks:** Get the MacroPlacement properties

**Returns:** `FixturePlacementProperties` — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.GetFixturePlacementProperties)

#### `GetMacro() -> object`

Get the corresponding macro definition

**Remarks:** Get the corresponding macro definition

**Returns:** `object` — Macro definition element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.GetMacro)

#### `SetFixturePlacementProperties(MacroPlacementProp: FixturePlacementProperties)`

Set the MacroPlacement properties

**Remarks:** Set the MacroPlacement properties

**Parameters:**
- `MacroPlacementProp` (FixturePlacementProperties) — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.SetFixturePlacementProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.__repr__)

## FixturePlacementProperties (class)

FixturePlacementProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementProperties/)

### Constructors
- `FixturePlacementProperties()` — Initialize

### Methods
#### `__eq__(prop: FixturePlacementProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixturePlacementProperties) — FixturePlacementProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementProperties/#NemAll_Python_Precast.FixturePlacementProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementProperties/#NemAll_Python_Precast.FixturePlacementProperties.__repr__)

### Properties
- `Automation` (None, get) — Value type: bool :type: None
- `ConnectionToAIACatalog` (None, get) — Enable connection to the Precast Fixture catalog of the Fixture placement element Value type: bool :type: None
- `ConnectionToAllplanCatalog` (None, get) — Enable connection to the Allplan Catalog of the Fixture placement element Value type: bool :type: None
- `CountOfQuestions` (None, get) — Number of active question attributes of the Fixture placement element Value type: int :type: None
- `DistortionState` (None, get) — Value type: bool :type: None
- `DomainType` (None, get) — Domaintype of the Fixture placement element Value type: None :type: None
- `EnableQuestions` (None, get) — Enables the question attributes of the Fixture placement element Value type: bool :type: None
- `HasParentModificationBehaviour` (None, get) — Property for specific behavior for modification state Value type: bool :type: None
- `HeightDefinitionType` (HeightDefinitionType, get/set) — Get the Height definition type
- `HollowShaft` (None, get) — Value type: bool :type: None
- `Mass_V6` (None, get) — Value type: float :type: None
- `Mass_V7` (None, get) — Value type: float :type: None
- `Mass_V8` (None, get) — Value type: float :type: None
- `Mass_V9` (None, get) — Value type: float :type: None
- `Matrix` (Matrix3D, get/set) — Get the Matrix for location in world coordinate system
- `MirrorState` (None, get) — Property for the fixture placement mirrored state Value type: bool :type: None
- `Name` (None, get) — Name of the Fixture placement element Value type: str :type: None
- `OutlineShape` (None, get) — Value type: OutlineShape :type: None
- `OutlineType` (None, get) — Value type: OutlineType :type: None
- `OutlineTypeInGroup` (None, get) — Value type: OutlineTypeInGroup :type: None
- `PositionNr` (None, get) — Value type: int :type: None
- `ProfilType` (None, get) — Value type: ProfilType :type: None
- `SubType` (None, get) — SubType of the Fixture placement element Value type: MacroSubType :type: None
- `Type` (None, get) — Type of the Fixture placement element Value type: MacroType :type: None
- `UnitFactor` (None, get) — Value type: int :type: None
- `UseAlways2DRepInGroundView` (None, get) — Value type: bool :type: None
- `UseDrawOrder` (bool, get/set) — Get the Uses the draw order setting of the placement or the elements of the Fixture ?
- `UseFormat` (bool, get/set) — Get the Uses the format setting (pen, stroke, color) of the placement or the elements of the Fixture ?
- `Visibility` (None, get) — Value type: bool :type: None

## FixtureProperties (class)

FixtureProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureProperties/)

### Constructors
- `FixtureProperties()` — Initialize

### Methods
#### `__eq__(prop: FixtureProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixtureProperties) — FixtureProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureProperties/#NemAll_Python_Precast.FixtureProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureProperties/#NemAll_Python_Precast.FixtureProperties.__repr__)

### Properties
- `CatalogName` (None, get) — Value type: str :type: None
- `DomainType` (None, get) — Domaintype of the Fixture element Value type: None :type: None
- `InsertionPoint` (None, get) — Value type: Point3D :type: None
- `IsScaleDependent` (None, get) — Value type: bool :type: None
- `Name` (None, get) — Value type: str :type: None
- `PositionNr` (None, get) — Value type: int :type: None
- `SubType` (None, get) — SubType of the Fixture element Value type: MacroSubType :type: None
- `Type` (None, get) — Type of the Fixture element Value type: MacroType :type: None
- `UnitFactor` (None, get) — Value type: int :type: None

## FixtureSlideElement (class)

FixtureSlideElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/)

### Constructors
- `FixtureSlideElement()` — Initialize
- `FixtureSlideElement(FixtureSlideProp: FixtureSlideProperties, objectList: list)` — Constructor

### Methods
#### `GetFixtureSlideProperties() -> FixtureSlideProperties`

Get the FixtureSlide properties

**Remarks:** Get the FixtureSlide properties

**Returns:** `FixtureSlideProperties` — FixtureSlide properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.GetFixtureSlideProperties)

#### `GetObjectList() -> list`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.GetObjectList)

#### `SetFixtureSlideProperties(FixtureSlideProp: FixtureSlideProperties)`

Set the FixtureSlide properties

**Remarks:** Set the FixtureSlide properties

**Parameters:**
- `FixtureSlideProp` (FixtureSlideProperties) — FixtureSlide properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.SetFixtureSlideProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.__repr__)

## FixtureSlideProperties (class)

FixtureSlideProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideProperties/)

### Constructors
- `FixtureSlideProperties()` — Initialize

### Methods
#### `__eq__(prop: FixtureSlideProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixtureSlideProperties) — FixtureSlideProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideProperties/#NemAll_Python_Precast.FixtureSlideProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideProperties/#NemAll_Python_Precast.FixtureSlideProperties.__repr__)

### Properties
- `EndScaleRange` (None, get) — Property for end reference scale of slide Value type: float :type: None
- `OffsetOfReferencePoint1` (None, get) — Property for first offset value to reference point Value type: Vector3D :type: None
- `OffsetOfReferencePoint2` (None, get) — Property for second offset value to reference point Value type: Vector3D :type: None
- `ReferencePoint` (None, get) — Property for reference point Value type: Point3D :type: None
- `ResizeSettingVx` (None, get) — Property for resize setting for x direction Value type: eCombinationType :type: None
- `ResizeSettingVy` (None, get) — Property for resize setting for y direction Value type: eCombinationType :type: None
- `ResizeSettingVz` (None, get) — Property for resize setting for z direction Value type: eCombinationType :type: None
- `StartScaleRange` (None, get) — Property for start reference scale of slide Value type: float :type: None
- `Type` (None, get) — Property for type of slide Value type: eSlideType :type: None
- `ViewType` (None, get) — Property for view type of slide Value type: eSlideViewType :type: None
- `VisibilityGeo2D` (None, get) — Property for geometry 2D visibility of slide Value type: bool :type: None
- `VisibilityGeo3D` (None, get) — Property for geometry 3D visibility of slide Value type: bool :type: None
- `VisibilityLayerA` (None, get) — Property for layer A visibility of slide Value type: bool :type: None
- `VisibilityLayerB` (None, get) — Property for layer B visibility of slide Value type: bool :type: None
- `VisibilityLayerC` (None, get) — Property for layer C visibility of slide Value type: bool :type: None

## FixtureSlideType (enum)

fixture slide types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideType/)

### Values
- `eCode` = `1`
- `eExtension` = `5`
- `eGeometry` = `0`
- `eReinforcement` = `2`
- `eReport` = `3`
- `eUndergroundCadaster` = `4`

## FixtureSlideViewType (enum)

Fixture slide view types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideViewType/)

### Values
- `e2D_BACK_VIEW` = `6`
- `e2D_FRONT_VIEW` = `5`
- `e2D_LEFT_VIEW` = `3`
- `e2D_NO_VIEW` = `0`
- `e2D_RIGHT_VIEW` = `4`
- `e2D_SYMBOL` = `11`
- `e2D_TOP_VIEW` = `1`
- `e3D_VIEW` = `7`
- `e3D_VIEW_OLD` = `8`
- `e3D_VIEW_OUTLINE_AREA` = `10`
- `e3D_VIEW_OUTLINE_AREA_ACTUAL` = `12`
- `e3D_VIEW_OUTLINE_VOLUME` = `9`
- `eBOTTOM_VIEW` = `2`
- `eCONNECTION_POINT` = `20`
- `eCONTOUR_CUT` = `18`
- `eLOCK_FIXED` = `14`
- `eLOCK_FIXED_P1` = `16`
- `eLOCK_UML` = `15`
- `eMEASURE_POINTS` = `19`
- `eORGA_ORIGINAL` = `17`
- `eTGA_ADAPTER` = `13`

## Functions (static-class)

Module-level functions of NemAll_Python_Precast

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/)

### Methods
#### `CreateElementplan( doc: DocumentAdapter, catOffset: int, pageProps: list, elements: BaseElementAdapterList, )`

@brief Creates the elementplan @param doc DocumentAdapter for ptrArrrayData @param catOffset Offset of layout in catalog @param elements Elements for plan

**Remarks:** @brief Creates the elementplan @param doc DocumentAdapter for ptrArrrayData @param catOffset Offset of layout in catalog @param elements Elements for plan

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/#NemAll_Python_Precast.CreateElementplan)

#### `CreatePrecastElements( doc: DocumentAdapter, insertionMat: Matrix3D, elements: BaseElementAdapterList, modelEleList: list, modelUuidList: list, viewProj: ViewWorldProjection, delete_python: bool, )`

Create the precast elements

**Remarks:** Create the precast elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Insertion matrix
- `elements` (BaseElementAdapterList) — List of created elements
- `modelEleList` (list) — List of model elements which have to be created
- `modelUuidList` (list) — List with the model UUIDS in modification mode
- `viewProj` (ViewWorldProjection) — View projection
- `delete_python` (bool) — bool weather the python should be deleted after update

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/#NemAll_Python_Precast.CreatePrecastElements)

#### `GetPagePropertiesFromCatalog(catOffset: int) -> list`

@brief Gets the pages from selected layout @param catOffset Offset of layout in catalog @return List of pages

**Remarks:** @brief Gets the pages from selected layout @param catOffset Offset of layout in catalog @return List of pages

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/#NemAll_Python_Precast.GetPagePropertiesFromCatalog)

#### `LockPrecastUpdate()`

Lock the precast update

**Remarks:** Lock the precast update

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/#NemAll_Python_Precast.LockPrecastUpdate)

#### `TriggerPrecastUpdate(elements: BaseElementAdapterList) -> bool`

Trigger the precast update

**Remarks:** Trigger the precast update

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to update

**Returns:** `bool` — update successful: true/false)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/#NemAll_Python_Precast.TriggerPrecastUpdate)

#### `UnlockPrecastUpdate()`

Lock the precast update

**Remarks:** Lock the precast update

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/#NemAll_Python_Precast.UnlockPrecastUpdate)

## HeightDefinitionType (enum)

Height definition types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/HeightDefinitionType/)

### Values
- `eAverage` = `3`
- `eComponent` = `2`
- `eMacro` = `1`
- `eNone` = `0`

## LabelStyle (class)

(No description provided in vendor docs for NemAll_Python_Precast.LabelStyle.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/LabelStyle/)

### Constructors
- `LabelStyle( doc: DocumentAdapter, cellId: int, labelStyleProps: LabelStyleProperties, allowOverlapping: bool, )` — Constructor
- `LabelStyle( doc: DocumentAdapter, cellId: int, labelStyleProps: LabelStyleProperties, allowOverlapping: bool, conditionTemplate: str, )` — Constructor

## LabelStyleProperties (class)

(No description provided in vendor docs for NemAll_Python_Precast.LabelStyleProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/LabelStyleProperties/)

### Constructors
- `LabelStyleProperties()` — Initialize
- `LabelStyleProperties(fileEntryPath: FileEntryPath, fileNr: int, entryNr: int)` — Constructor
- `LabelStyleProperties( fileEntryPath: FileEntryPath, fileNr: int, entryNr: int, location: Location )` — Constructor

### Properties
- `EntryNr` (None, get) — Entry number of labelStyle props Value type: int :type: None
- `FileEntrPath` (None, get) — File entry path of labelStyle props Value type: EFileEntryPath enum :type: None
- `FileNr` (None, get) — File number of labelStyle props Value type: int :type: None
- `Location` (None, get) — Location of labelStyle props Value type: Location enum :type: None

## Legend (class)

(No description provided in vendor docs for NemAll_Python_Precast.Legend.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Legend/)

### Constructors
- `Legend(doc: DocumentAdapter, cellId: int, legendProps: LegendProperties)` — Constructor
- `Legend( doc: DocumentAdapter, cellId: int, legendProps: LegendProperties, conditionTemplate: str, )` — Constructor

## LegendProperties (class)

(No description provided in vendor docs for NemAll_Python_Precast.LegendProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/LegendProperties/)

### Constructors
- `LegendProperties()` — Initialize
- `LegendProperties( fileEntryPath: FileEntryPath, fileNr: int, entryNr: int, maxHeight: float, maxWidth: float, )` — Constructor

### Properties
- `EntryNr` (None, get) — Entry number of legend props Value type: int :type: None
- `FileEntrPath` (None, get) — File entry path of legend props Value type: EFileEntryPath enum :type: None
- `FileNr` (None, get) — File number of legend props Value type: int :type: None
- `MaxHeight` (None, get) — Max height of legend props Value type: double :type: None
- `MaxWidth` (None, get) — Max width of legend props Value type: double :type: None

## Location (enum)

locations

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Location/)

### Values
- `eLeftBottom` = `1`
- `eLeftTop` = `3`
- `eRightBottom` = `0`
- `eRightTop` = `2`

## MacroGroupType (enum)

types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroGroupType/)

### Values
- `eGroupType_CuttedGroup` = `2`
- `eGroupType_DynamicGroup` = `1`
- `eGroupType_GeneralGroup` = `0`
- `eGroupType_LeadingGroup` = `3`

## MacroSubType (enum)

Sub types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroSubType/)

### Values
- `eAnchorPlate` = `13`
- `eAnchorage` = `12`
- `eBarAccessory` = `50`
- `eBarCoupler` = `47`
- `eBarNut` = `48`
- `eBarThread` = `49`
- `eBracingElement` = `44`
- `eCatchmentArea` = `41`
- `eChannel` = `54`
- `eCirculationLoadPoint` = `80`
- `eCirculationPipeAdapter` = `81`
- `eCirculationStartPoint` = `79`
- `eConcreteArea` = `83`
- `eConcreteBeam` = `46`
- `eConcreteBlock` = `45`
- `eConnectionModeller` = `24`
- `eConnectionWallColumn` = `93`
- `eConnectorEBT` = `88`
- `eConstPrefabConnection` = `60`
- `eCorbel` = `10`
- `eCoverMountingAngle` = `53`
- `eCrossRib` = `82`
- `eElectricalBIE` = `67`
- `eElectricalLamp` = `68`
- `eElectricalRoute` = `69`
- `eFacility` = `32`
- `eFalseJoint` = `55`
- `eFill` = `29`
- `eFrame` = `9`
- `eHeatingLoadPoint` = `71`
- `eHeatingPipeAdapter` = `72`
- `eHeatingStartPoint` = `70`
- `eHollowBody` = `25`
- `eInsertion` = `35`
- `eInsulationArea` = `85`
- `eInsulationElement` = `90`
- `eInsulationStripe` = `89`
- `eJointLength` = `22`
- `eJointReinforcement` = `28`
- `eLinePointPlacement` = `58`
- `eLoadCut` = `19`
- `eMultiLine3D` = `42`
- `eNailer` = `92`
- `eNode` = `39`
- `eOverrule` = `86`
- `ePipe` = `30`
- `ePipePoint` = `31`
- `ePlacingLoop` = `26`
- `ePolyline` = `1`
- `ePrefabConnection` = `37`
- `ePrefabConnectionCorner` = `38`
- `ePrefabModeller` = `23`
- `eProfileEdge` = `21`
- `eReinforcement_Cage` = `87`
- `eRevealAnchor` = `8`
- `eRevealAnchorVirtual` = `20`
- `eRibBody` = `59`
- `eRingBeam` = `6`
- `eRoofLine` = `3`
- `eRoofParapetLine` = `4`
- `eRoofParapetSupport` = `5`
- `eRope` = `40`
- `eSTD_Formwork` = `94`
- `eSanitationLoadPoint` = `77`
- `eSanitationPipeAdapter` = `78`
- `eSanitationStartPoint` = `76`
- `eSecondaryReinf` = `27`
- `eSewageLoadPoint` = `65`
- `eSewageNetElement` = `52`
- `eSewagePipeAdapter` = `66`
- `eSewageStartPoint` = `64`
- `eShaft` = `33`
- `eSlidingRestraint` = `7`
- `eSolidStrip` = `56`
- `eSpecialBuilding` = `34`
- `eSpecialLoad_Undefined` = `17`
- `eSpecialLoad_X` = `14`
- `eSpecialLoad_Y` = `15`
- `eSpecialLoad_Z` = `16`
- `eSphere` = `63`
- `eSteelProfile` = `43`
- `eStripCorbel` = `57`
- `eStructuralRecessFaceSupport` = `62`
- `eStructuralRecessLongitudinalSupport` = `61`
- `eSurface` = `51`
- `eTieBar` = `11`
- `eTileArea` = `91`
- `eTileElement` = `84`
- `eTrimmer` = `18`
- `eUseNoSpecialSubType` = `0`
- `eUseSameSubType` = `-1`
- `eVentilationDuctAdapter` = `75`
- `eVentilationLoadPoint` = `74`
- `eVentilationStartPoint` = `73`
- `eZone` = `36`

## MacroType (enum)

types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroType/)

### Values
- `eGroup_Fixture` = `3`
- `eLine_Fixture` = `1`
- `ePlane_Fixture` = `2`
- `ePoint_Fixture` = `0`
- `eUseSameType` = `-1`

## OutlineShape (enum)

outline shapes

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineShape/)

### Values
- `eBUILTIN_OUTLINE_SHAPE_NOTHING` = `0`
- `eBUILTIN_OUTLINE_SHAPE_RECTANGLE` = `1`
- `eBUILTIN_OUTLINE_SHAPE_SYMBOL` = `3`
- `eBUILTIN_OUTLINE_SHAPE_TRAPEZOID` = `2`

## OutlineType (enum)

outline types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineType/)

### Values
- `eBUILTIN_OUTLINE_TYPE_MINUS` = `2`
- `eBUILTIN_OUTLINE_TYPE_NOTHING` = `0`
- `eBUILTIN_OUTLINE_TYPE_NO_AFFECT` = `3`
- `eBUILTIN_OUTLINE_TYPE_PLUS` = `1`

## OutlineTypeInGroup (enum)

outline types in group

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineTypeInGroup/)

### Values
- `eBUILTIN_OUTLINE_TYPE_IN_GROUP_MINUS` = `2`
- `eBUILTIN_OUTLINE_TYPE_IN_GROUP_NOTHING` = `0`
- `eBUILTIN_OUTLINE_TYPE_IN_GROUP_PLUS` = `1`

## Page (class)

(No description provided in vendor docs for NemAll_Python_Precast.Page.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Page/)

### Constructors
- `Page(doc: DocumentAdapter, pageNr: int, anchors: list)` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, anchors: list, conditionTemplate: str )` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, anchors: list, size: MinMax2D, centeringCells: bool, )` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, anchors: list, size: MinMax2D, centeringCells: bool, conditionTemplate: str, )` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, scale: float, anchors: list, size: MinMax2D, centeringCells: bool, )` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, scale: float, anchors: list, size: MinMax2D, centeringCells: bool, conditionTemplate: str, )` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, scales: list, anchors: list, size: MinMax2D, centeringCells: bool, )` — Constructor
- `Page( doc: DocumentAdapter, pageNr: int, scales: list, anchors: list, size: MinMax2D, centeringCells: bool, conditionTemplate: str, )` — Constructor

### Methods
#### `add_anchor(anchor: Anchor)`

Adds an anchor to the page

**Remarks:** Adds an anchor to the page

**Parameters:**
- `Anchor` (object) — to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Page/#NemAll_Python_Precast.Page.add_anchor)

#### `add_anchors(anchors: list)`

Adds anchors to the page

**Remarks:** Adds anchors to the page

**Parameters:**
- `Anchors` (object) — to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Page/#NemAll_Python_Precast.Page.add_anchors)

#### `add_cell(cell: Cell)`

Adds a cell (labelStyle/legend/view) to the page

**Remarks:** Adds a cell (labelStyle/legend/view) to the page

**Parameters:**
- `Cell` (object) — to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Page/#NemAll_Python_Precast.Page.add_cell)

### Properties
- `DrawingFile` (None, get) — Sets the drawing file where the page should be placed Value type: int :type: None

## PageProperties (class)

@brief Wrapper for page properties of elementplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PageProperties/)

### Constructors
- `PageProperties(label: str, sizeType: int, scaleType: int, fixedScale: float)` — @brief Creates a helper to fill python palette for UVS Elementplan @param label Label of the page @param sizeType Size type of the page (Fixed | AutomaticSelection) @param scaleType Scale type of the page (ScaleAutomaticSelection | ScaleFixed | MaximumSize) @param fixedScale Fixed scale
- `PageProperties(element: PageProperties)` — Copy constructor

### Properties
- `FixedScale` (float, get/set) — Get the Fixed scaleof the page
- `Label` (str, get/set) — Get the Label of the page
- `ScaleType` (int, get/set) — Get the Scale type of the page
- `SizeType` (int, get/set) — Get the Size type of the page

## PagePropertiesList (class)

List for PageProperties objects

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/)

### Constructors
- `PagePropertiesList()` — Initialize
- `PagePropertiesList(ele: PageProperties)` — Constructor with a PageProperties
- `PagePropertiesList(eleList: list)` — Constructor with a list of PageProperties

### Methods
#### `__contains__(value: PageProperties) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (PageProperties) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__contains__)

#### `__delitem__(value: PageProperties)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (PageProperties) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__delitem__)

#### `__eq__(compare_list: PagePropertiesList) -> bool`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (PagePropertiesList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__eq__)

#### `__getitem__(index: int) -> PageProperties`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `PageProperties` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__getitem__)

#### `__iadd__(eleList: list) -> PagePropertiesList`

Add a list

**Remarks:** Add a list

**Parameters:**
- `eleList` (list) — PageProperties list

**Returns:** `PagePropertiesList` — Lists with the added elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__iadd__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__len__)

#### `__setitem__(index: int | slice, value: PageProperties)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (PageProperties) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.__setitem__)

#### `append(value: PageProperties)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (PageProperties) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.append)

#### `extend(eleList: list)`

Extend the list

**Remarks:** Extend the list

**Parameters:**
- `eleList` (list) — PageProperties list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.extend)

#### `extend(iterable: PagePropertiesList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (PagePropertiesList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PagePropertiesList/#NemAll_Python_Precast.PagePropertiesList.extend)

## Plan (class)

(No description provided in vendor docs for NemAll_Python_Precast.Plan.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Plan/)

### Constructors
- `Plan(doc: DocumentAdapter)` — Constructor
- `Plan(arg2: DocumentAdapter, doc: BaseElementAdapterList)` — Constructor

### Methods
#### `add_page(page: Page)`

Adds a page to the plan

**Remarks:** Adds a page to the plan

**Parameters:**
- `page` (Page) — to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Plan/#NemAll_Python_Precast.Plan.add_page)

#### `create( elemPlan: BaseElementAdapter, baseElements: BaseElementAdapterList ) -> bool`

Creates the elementplan

**Remarks:** Creates the elementplan

**Parameters:**
- `Allplan` (object) — :IFW::ElementAdapter::BaseElementAdapter:-> created elementplan
- `Allplan` (object) — :IFW::ElementAdapter::BaseElementAdapterList:-> elements to create

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Plan/#NemAll_Python_Precast.Plan.create)

#### `create(elemPlan: BaseElementAdapter) -> bool`

Creates the elementplan

**Remarks:** Creates the elementplan

**Parameters:**
- `Allplan` (object) — :IFW::ElementAdapter::BaseElementAdapter:-> created elementplan

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Plan/#NemAll_Python_Precast.Plan.create)

### Properties
- `DrawingFile` (None, get) — Sets the drawing file where the elementplan should be placed Value type: int :type: None
- `Offset` (None, get) — Sets the offset of the plan Value type: Allplan::Geometry::Point2D :type: None

## Position (enum)

position for labeling

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Position/)

### Values
- `eAccordingTheUSStandard` = `7`
- `eBottomCenter` = `5`
- `eBottomLeft` = `4`
- `eBottomRight` = `6`
- `eNone` = `0`
- `eTopCenter` = `2`
- `eTopLeft` = `1`
- `eTopRight` = `3`

## PrecastElement (class)

(No description provided in vendor docs for NemAll_Python_Precast.PrecastElement.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElement/)

### Constructors
- `PrecastElement(Properties: PrecastElementProperties)` — Constructor

### Methods
#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElement/#NemAll_Python_Precast.PrecastElement.__repr__)

### Properties
- `Properties` (None, get) — Property for Precast Element Properties Value type: PrecastElementProperties :type: None
- `deletePython` (None, get) — Property to delete Python after elementation Value type: int :type: None

## PrecastElementProperties (class)

PrecastElementProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/)

### Constructors
- `PrecastElementProperties()` — Initialize

### Methods
#### `GetPrecastElementTypeFromIdx(arg2: int) -> str`

Get the name of the PrecastElementType from Cat Idx

**Remarks:** Get the name of the PrecastElementType from Cat Idx

**Returns:** `str` — PrecastElementType Cat Name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.GetPrecastElementTypeFromIdx)

#### `SetElementTypeCatalogGUID_from_Name(arg2: str) -> GUID`

Set the elementTypeCatGUID

**Remarks:** Set the elementTypeCatGUID

**Returns:** `GUID` — elementTypeCatGUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.SetElementTypeCatalogGUID_from_Name)

#### `SetFactoryCatalogAddressOffset(arg2: str) -> int`

Set the factoryCatAddressOffset

**Remarks:** Set the factoryCatAddressOffset

**Returns:** `int` — factoryCatAddressOffset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.SetFactoryCatalogAddressOffset)

#### `SetNormCatalogAddressOffset(arg2: str) -> int`

Set the normCatAddressOffset

**Remarks:** Set the normCatAddressOffset

**Returns:** `int` — normCatAddressOffset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.SetNormCatalogAddressOffset)

#### `__eq__(prop: PrecastElementProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PrecastElementProperties) — PrecastElementProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.__repr__)

### Properties
- `CreateLabeling` (None, get) — Property for CreateLabeling Value type: bool :type: None
- `DimensionCross` (None, get) — Property for DimensionCross Value type: float :type: None
- `DimensionSpan` (None, get) — Property for DimensionSpan Value type: float :type: None
- `DimensionViewing` (None, get) — Property for DimensionViewing Value type: float :type: None
- `ElemTypeAttribut` (None, get) — Property for ElemTypeAttribut Value type: str :type: None
- `ElementType` (None, get) — Property for ElementType Value type: int :type: None
- `ElementTypeCatGUID` (None, get) — Property for ElementTypeCatGUID Value type: GUID :type: None
- `Factory` (None, get) — Property for Factory Value type: str :type: None
- `FactoryCatAddressOffset` (None, get) — Property for FactoryCatAddressOffset Value type: int :type: None
- `LabelingTextRefPoint` (None, get) — Property for LabelingTextRefPoint Value type: int :type: None
- `Layers` (None, get) — Property for Layers Value type: list :type: None
- `ManualDimensions` (None, get) — Property for ManualDimensions Value type: bool :type: None
- `Norm` (None, get) — Property for Norm Value type: str :type: None
- `NormCatAddressOffset` (None, get) — Property for NormCatAddressOffset Value type: int :type: None
- `PieceFactor` (None, get) — Property for PieceFactor Value type: int :type: None
- `PosNr` (None, get) — Property for PosNr Value type: int :type: None
- `PosNrText` (None, get) — Property for PosNrText Value type: str :type: None
- `ReferencePoint` (None, get) — Property for ReferencePoint Value type: Point3D :type: None
- `SpanDirection` (None, get) — Property for SpanDirection Value type: Point3D :type: None
- `ViewDirection` (None, get) — Property for ViewDirection Value type: Point3D :type: None

## PrecastLayer (class)

PrecastLayer class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayer/)

### Constructors
- `PrecastLayer()` — Initialize
- `PrecastLayer(Properties: PrecastLayerProperties)` — Constructor

### Methods
#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayer/#NemAll_Python_Precast.PrecastLayer.__repr__)

### Properties
- `Properties` (None, get) — Property for Properties Value type: PrecastLayerProperties :type: None

## PrecastLayerProperties (class)

PrecastLayerProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/)

### Constructors
- `PrecastLayerProperties()` — Initialize

### Methods
#### `SetMaterialCatalogAddressOffset(arg2: str, arg3: str) -> int`

Set the materialCatAddressOffset

**Remarks:** Set the materialCatAddressOffset

**Returns:** `int` — materialCatAddressOffset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/#NemAll_Python_Precast.PrecastLayerProperties.SetMaterialCatalogAddressOffset)

#### `__eq__(prop: PrecastLayerProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PrecastLayerProperties) — PrecastLayerProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/#NemAll_Python_Precast.PrecastLayerProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/#NemAll_Python_Precast.PrecastLayerProperties.__repr__)

### Properties
- `CalculateLayerThickness` (None, get) — Property for CalculateLayerThickness Value type: bool :type: None
- `LayerName` (None, get) — Property for LayerName Value type: str :type: None
- `LayerNumber` (None, get) — Property for LayerNumber Value type: int :type: None
- `LayerThickness` (None, get) — Property for LayerThickness Value type: float :type: None
- `Material` (None, get) — Property for Material Value type: str :type: None
- `MaterialCatAddressOffset` (None, get) — Property for MaterialCatAddressOffset Value type: int :type: None
- `MaterialType` (None, get) — Property for MaterialType Value type: int :type: None

## PrecastMWSElement (class)

PrecastMWSElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastMWSElement/)

### Constructors
- `PrecastMWSElement()` — Initialize
- `PrecastMWSElement( factory: str, name: str, number: int, piecefactor: int, longitBarHeight: int, SegmentNumber: int, SegmentVector: Point3D, SegementPointList: list, ReinforcementList: list, )` — Constructor

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastMWSElement/#NemAll_Python_Precast.PrecastMWSElement.__repr__)

### Properties
- `Factory` (None, get) — Property for Factory Value type: str :type: None
- `IndexLongitBar` (None, get) — Index of the longitudinal bar in the Reinfrocementlist Value type: int :type: None
- `LongitBarHeight` (None, get) — Heightposition of the longitudinal bar (1 = Position 1, 2 = Position 2) Value type: int :type: None
- `Name` (None, get) — Property for Name Value type: str :type: None
- `Number` (None, get) — Property for Number Value type: int :type: None
- `Piecefactor` (None, get) — Property for Piecefactor Value type: int :type: None
- `ReinforcementList` (None, get) — list of reinforcement placements Value type: list :type: None
- `SegmentNumber` (None, get) — Number of the main segement of the transversal shape Value type: int :type: None
- `SegmentPointList` (None, get) — Pointlist of the transversal shape Value type: list :type: None
- `SegmentVector` (None, get) — Property for Norm Catalog Value type: int :type: None

## ProfilType (enum)

profil types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/ProfilType/)

### Values
- `eBUILTIN_PROFIL_TYPE_EDGE` = `1`
- `eBUILTIN_PROFIL_TYPE_JOINT` = `0`

## Rotation (enum)

rotations for views

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Rotation/)

### Values
- `eRotation0` = `0`
- `eRotation180` = `2`
- `eRotation270` = `3`
- `eRotation90` = `1`

## SubType (enum)

Sub types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/SubType/)

### Values
- `eAnchorPlate` = `13`
- `eAnchorage` = `12`
- `eBarAccessory` = `50`
- `eBarCoupler` = `47`
- `eBarNut` = `48`
- `eBarThread` = `49`
- `eBracingElement` = `44`
- `eCatchmentArea` = `41`
- `eChannel` = `54`
- `eCirculationLoadPoint` = `80`
- `eCirculationPipeAdapter` = `81`
- `eCirculationStartPoint` = `79`
- `eConcreteArea` = `83`
- `eConcreteBeam` = `46`
- `eConcreteBlock` = `45`
- `eConnectionModeller` = `24`
- `eConnectionWallColumn` = `93`
- `eConnectorEBT` = `88`
- `eConstPrefabConnection` = `60`
- `eCorbel` = `10`
- `eCoverMountingAngle` = `53`
- `eCrossRib` = `82`
- `eElectricalBIE` = `67`
- `eElectricalLamp` = `68`
- `eElectricalRoute` = `69`
- `eFacility` = `32`
- `eFalseJoint` = `55`
- `eFill` = `29`
- `eFrame` = `9`
- `eHeatingLoadPoint` = `71`
- `eHeatingPipeAdapter` = `72`
- `eHeatingStartPoint` = `70`
- `eHollowBody` = `25`
- `eInsertion` = `35`
- `eInsulationArea` = `85`
- `eInsulationElement` = `90`
- `eInsulationStripe` = `89`
- `eJointLength` = `22`
- `eJointReinforcement` = `28`
- `eLinePointPlacement` = `58`
- `eLoadCut` = `19`
- `eMultiLine3D` = `42`
- `eNailer` = `92`
- `eNode` = `39`
- `eOverrule` = `86`
- `ePipe` = `30`
- `ePipePoint` = `31`
- `ePlacingLoop` = `26`
- `ePolyline` = `1`
- `ePrefabConnection` = `37`
- `ePrefabConnectionCorner` = `38`
- `ePrefabModeller` = `23`
- `eProfileEdge` = `21`
- `eReinforcement_Cage` = `87`
- `eRevealAnchor` = `8`
- `eRevealAnchorVirtual` = `20`
- `eRibBody` = `59`
- `eRingBeam` = `6`
- `eRoofLine` = `3`
- `eRoofParapetLine` = `4`
- `eRoofParapetSupport` = `5`
- `eRope` = `40`
- `eSTD_Formwork` = `94`
- `eSanitationLoadPoint` = `77`
- `eSanitationPipeAdapter` = `78`
- `eSanitationStartPoint` = `76`
- `eSecondaryReinf` = `27`
- `eSewageLoadPoint` = `65`
- `eSewageNetElement` = `52`
- `eSewagePipeAdapter` = `66`
- `eSewageStartPoint` = `64`
- `eShaft` = `33`
- `eSlidingRestraint` = `7`
- `eSolidStrip` = `56`
- `eSpecialBuilding` = `34`
- `eSpecialLoad_Undefined` = `17`
- `eSpecialLoad_X` = `14`
- `eSpecialLoad_Y` = `15`
- `eSpecialLoad_Z` = `16`
- `eSphere` = `63`
- `eSteelProfile` = `43`
- `eStripCorbel` = `57`
- `eStructuralRecessFaceSupport` = `62`
- `eStructuralRecessLongitudinalSupport` = `61`
- `eSurface` = `51`
- `eTieBar` = `11`
- `eTileArea` = `91`
- `eTileElement` = `84`
- `eTrimmer` = `18`
- `eUseNoSpecialSubType` = `0`
- `eUseSameSubType` = `-1`
- `eVentilationDuctAdapter` = `75`
- `eVentilationLoadPoint` = `74`
- `eVentilationStartPoint` = `73`
- `eZone` = `36`

## TextAlignment (enum)

Alignments for text of labeling for view

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/TextAlignment/)

### Values
- `eBottomCenter` = `6`
- `eCenter` = `5`
- `eLeftBottom` = `1`
- `eLeftCenter` = `9`
- `eLeftTop` = `4`
- `eRightBottom` = `2`
- `eRightCenter` = `7`
- `eRightTop` = `3`
- `eTopCenter` = `8`
- `eUnknownAlignment` = `0`

## Type (enum)

types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/Type/)

### Values
- `eGroup_Fixture` = `3`
- `eLine_Fixture` = `1`
- `ePlane_Fixture` = `2`
- `ePoint_Fixture` = `0`
- `eUseSameType` = `-1`

## View (class)

(No description provided in vendor docs for NemAll_Python_Precast.View.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/View/)

### Constructors
- `View( doc: DocumentAdapter, cellId: int, direction: Direction, rotation: Rotation )` — Constructor
- `View( doc: DocumentAdapter, cellId: int, direction: Direction, rotation: Rotation, viewProps: ViewProperties, )` — Constructor
- `View( doc: DocumentAdapter, cellId: int, direction: Direction, rotation: Rotation, viewProps: ViewProperties, conditionTemplate: str, )` — Constructor

### Methods
#### `create( elements: BaseElementAdapterList, position: Point2D, view: BaseElementAdapter, ) -> bool`

Creates a standalon local uvs view without plan

**Remarks:** Creates a standalon local uvs view without plan

**Parameters:**
- `Allplan` (object) — :IFW::ElementAdapter::BaseElementAdapterList:-> elements of the view
- `Allplan` (object) — :Geometry::Point2D:-> position of the view
- `Allplan` (object) — :IFW::ElementAdapter::BaseElementAdapter:-> created view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Precast/View/#NemAll_Python_Precast.View.create)

