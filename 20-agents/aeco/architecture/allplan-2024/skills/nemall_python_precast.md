---
name: allplan-nemall_python_precast
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_Precast namespace — 26 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AllplanElement, FixtureCombinationType, FixtureGroupElement, FixtureGroupProperties, FixtureElement, FixturePlacementElement, FixturePlacementProperties, FixtureSlideElement, and 18 more types.
---

# NemAll_Python_Precast

Auto-generated from vendor docs for allplan 2024.0. 26 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/)

### Methods
#### `GetAttributes()`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetAttributes)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetCommonProperties)

#### `GetGeometryObject()`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetGeometryObject)

#### `GetLabelElements()`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetLabelElements)

#### `GetSubElementID()`

Get the sub element ID

**Remarks:** Get the sub element ID

**Returns:** `type` — Sub Element ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/AllplanElement/#NemAll_Python_Precast.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## FixtureCombinationType (enum)

Fixture combination types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureCombinationType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `FixtureCombinationType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureCombinationType/#NemAll_Python_Precast.FixtureCombinationType.__getitem__)

### Values
- `eVx` = `1`
- `eVy` = `2`
- `eVz` = `3`

## FixtureElement (class)

FixtureElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/)

### Constructors
- `FixtureElement()` — Initialize
- `FixtureElement(fixProp, slideList)` — Constructor

### Methods
#### `GetFixtureProperties()`

Get the Fixture properties

**Remarks:** Get the Fixture properties

**Returns:** `FixtureProperties` — Fixture properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.GetFixtureProperties)

#### `GetHash()`

Get the hash value

**Remarks:** Get the hash value

**Returns:** `str` — Hash value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.GetHash)

#### `GetSlideList()`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.GetSlideList)

#### `SetFixtureProperties(fixProp)`

Set the Fixture properties

**Remarks:** Set the Fixture properties

**Parameters:**
- `fixProp` (FixtureProperties) — fixture properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.SetFixtureProperties)

#### `SetHash(hash)`

Set the hash value

**Remarks:** Set the hash value

**Parameters:**
- `hash` (str) — Hash value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.SetHash)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureElement/#NemAll_Python_Precast.FixtureElement.__repr__)

## FixtureGroupElement (class)

FixtureGroupElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/)

### Constructors
- `FixtureGroupElement()` — Initialize
- `FixtureGroupElement(FixtureGroupProp, slideList)` — Constructor
- `FixtureGroupElement(commonProp, FixtureGroupProp, slideList)` — Constructor

### Methods
#### `GetFixtureGroupProperties()`

Get the FixtureGroup properties

**Remarks:** Get the FixtureGroup properties

**Returns:** `FixtureGroupProperties` — FixtureGroup properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.GetFixtureGroupProperties)

#### `GetFixtureList()`

Get the fixture object list

**Remarks:** Get the fixture object list

**Returns:** `list` — Fixture object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.GetFixtureList)

#### `SetFixtureGroupProperties(FixtureGroupProp)`

Set the FixtureGroup properties

**Remarks:** Set the FixtureGroup properties

**Parameters:**
- `FixtureGroupProp` (FixtureGroupProperties) — FixtureGroup properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.SetFixtureGroupProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupElement/#NemAll_Python_Precast.FixtureGroupElement.__repr__)

## FixtureGroupProperties (class)

FixtureGroupProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupProperties/)

### Constructors
- `FixtureGroupProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixtureGroupProperties) — FixtureGroupProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupProperties/#NemAll_Python_Precast.FixtureGroupProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureGroupProperties/#NemAll_Python_Precast.FixtureGroupProperties.__repr__)

### Properties
- `Name` (None, get) — Property for name :type: None

## FixturePlacementElement (class)

FixturePlacementElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/)

### Constructors
- `FixturePlacementElement()` — Initialize
- `FixturePlacementElement(commonProp, macroPlacementProp, macro)` — Constructor

### Methods
#### `GetFixturePlacementProperties()`

Get the MacroPlacement properties

**Remarks:** Get the MacroPlacement properties

**Returns:** `FixturePlacementProperties` — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.GetFixturePlacementProperties)

#### `GetMacro()`

Get the corresponding macro definition

**Remarks:** Get the corresponding macro definition

**Returns:** `object` — Macro definition element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.GetMacro)

#### `SetFixturePlacementProperties(MacroPlacementProp)`

Set the MacroPlacement properties

**Remarks:** Set the MacroPlacement properties

**Parameters:**
- `MacroPlacementProp` (FixturePlacementProperties) — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.SetFixturePlacementProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementElement/#NemAll_Python_Precast.FixturePlacementElement.__repr__)

## FixturePlacementProperties (class)

FixturePlacementProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementProperties/)

### Constructors
- `FixturePlacementProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixturePlacementProperties) — FixturePlacementProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementProperties/#NemAll_Python_Precast.FixturePlacementProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixturePlacementProperties/#NemAll_Python_Precast.FixturePlacementProperties.__repr__)

### Properties
- `Automation` (None, get) — Property for automation :type: None
- `ConnectionToAIACatalog` (None, get) — Property for connection to AIACatalog :type: None
- `ConnectionToAllplanCatalog` (None, get) — Property for connection to AllplanCatalog :type: None
- `CountOfQuestions` (None, get) — Property for count of questions :type: None
- `DistortionState` (None, get) — Property for distortion state :type: None
- `DomainType` (None, get) — Property for domain type :type: None
- `EnableQuestions` (None, get) — Property for enable questions :type: None
- `HasParentModificationBehaviour` (None, get) — Property for specific behavior for modification state :type: None
- `HeightDefinitionType` (None, get) — Property for height definition types :type: None
- `HollowShaft` (None, get) — Property for hollow shaft :type: None
- `Mass_V6` (None, get) — Property for V6 attribute :type: None
- `Mass_V7` (None, get) — Property for V7 attribute :type: None
- `Mass_V8` (None, get) — Property for V8 attribute :type: None
- `Mass_V9` (None, get) — Property for V9 attribute :type: None
- `Matrix` (None, get) — Property for matrix. Specifies location in world coordinate system :type: None
- `MirrorState` (None, get) — Property for the fixture placement mirrored state :type: None
- `Name` (None, get) — Property for name :type: None
- `OutlineShape` (None, get) — Property for outline shape :type: None
- `OutlineType` (None, get) — Property for outline type :type: None
- `OutlineTypeInGroup` (None, get) — Property for outline type in group :type: None
- `PositionNr` (None, get) — Property for position number :type: None
- `ProfilType` (None, get) — Property for profil type :type: None
- `SubType` (None, get) — Property for sub type :type: None
- `Type` (None, get) — Property for type :type: None
- `UnitFactor` (None, get) — Property for unit factor :type: None
- `UseAlways2DRepInGroundView` (None, get) — Property for state if always 2D representation in ground view is shown :type: None
- `UseDrawOrder` (None, get) — Property for the draw order setting of the placement or the elements of the fixture :type: None
- `UseFormat` (None, get) — Property for the format setting (pen, stroke, color) of the placement or the elements of the macro :type: None
- `Visibility` (None, get) — Property for visibility :type: None

## FixtureProperties (class)

FixtureProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureProperties/)

### Constructors
- `FixtureProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixtureProperties) — FixtureProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureProperties/#NemAll_Python_Precast.FixtureProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureProperties/#NemAll_Python_Precast.FixtureProperties.__repr__)

### Properties
- `CatalogName` (None, get) — Property for catalog name :type: None
- `DomainType` (None, get) — Property for domain type :type: None
- `InsertionPoint` (None, get) — Property for insertion point :type: None
- `IsScaleDependent` (None, get) — Property for scale dependent state :type: None
- `Name` (None, get) — Property for name :type: None
- `PositionNr` (None, get) — Property for position number :type: None
- `SubType` (None, get) — Property for sub type :type: None
- `Type` (None, get) — Property for type :type: None
- `UnitFactor` (None, get) — Property for unit factor :type: None

## FixtureSlideElement (class)

FixtureSlideElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/)

### Constructors
- `FixtureSlideElement()` — Initialize
- `FixtureSlideElement(FixtureSlideProp, objectList)` — Constructor

### Methods
#### `GetFixtureSlideProperties()`

Get the FixtureSlide properties

**Remarks:** Get the FixtureSlide properties

**Returns:** `FixtureSlideProperties` — FixtureSlide properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.GetFixtureSlideProperties)

#### `GetObjectList()`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.GetObjectList)

#### `SetFixtureSlideProperties(FixtureSlideProp)`

Set the FixtureSlide properties

**Remarks:** Set the FixtureSlide properties

**Parameters:**
- `FixtureSlideProp` (FixtureSlideProperties) — FixtureSlide properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.SetFixtureSlideProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideElement/#NemAll_Python_Precast.FixtureSlideElement.__repr__)

## FixtureSlideProperties (class)

FixtureSlideProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideProperties/)

### Constructors
- `FixtureSlideProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FixtureSlideProperties) — FixtureSlideProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideProperties/#NemAll_Python_Precast.FixtureSlideProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideProperties/#NemAll_Python_Precast.FixtureSlideProperties.__repr__)

### Properties
- `EndScaleRange` (None, get) — Property for end reference scale of slide :type: None
- `OffsetOfReferencePoint1` (None, get) — Property for first offset value to reference point :type: None
- `OffsetOfReferencePoint2` (None, get) — Property for second offset value to reference point :type: None
- `ReferencePoint` (None, get) — Property for reference point :type: None
- `ResizeSettingVx` (None, get) — Property for resize setting for x direction :type: None
- `ResizeSettingVy` (None, get) — Property for resize setting for y direction :type: None
- `ResizeSettingVz` (None, get) — Property for resize setting for z direction :type: None
- `StartScaleRange` (None, get) — Property for start reference scale of slide :type: None
- `Type` (None, get) — Property for type of slide :type: None
- `ViewType` (None, get) — Property for view type of slide :type: None
- `VisibilityGeo2D` (None, get) — Property for geometry 2D visibility of slide :type: None
- `VisibilityGeo3D` (None, get) — Property for geometry 3D visibility of slide :type: None
- `VisibilityLayerA` (None, get) — Property for layer A visibility of slide :type: None
- `VisibilityLayerB` (None, get) — Property for layer B visibility of slide :type: None
- `VisibilityLayerC` (None, get) — Property for layer C visibility of slide :type: None

## FixtureSlideType (enum)

fixture slide types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `FixtureSlideType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideType/#NemAll_Python_Precast.FixtureSlideType.__getitem__)

### Values
- `eCode` = `1`
- `eExtension` = `5`
- `eGeometry` = `0`
- `eReinforcement` = `2`
- `eReport` = `3`
- `eUndergroundCadaster` = `4`

## FixtureSlideViewType (enum)

Fixture slide view types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideViewType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `FixtureSlideViewType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/FixtureSlideViewType/#NemAll_Python_Precast.FixtureSlideViewType.__getitem__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/_functions/)

### Methods
#### `CreatePrecastElements(doc, insertionMat, elements, modelEleList, modelUuidList, viewProj, delete_python)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/_functions/#NemAll_Python_Precast.CreatePrecastElements)

#### `LockPrecastUpdate()`

Lock the precast update

**Remarks:** Lock the precast update

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/_functions/#NemAll_Python_Precast.LockPrecastUpdate)

#### `TriggerPrecastUpdate(elements)`

Trigger the precast update

**Remarks:** Trigger the precast update

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to update

**Returns:** `bool` — update successful: true/false)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/_functions/#NemAll_Python_Precast.TriggerPrecastUpdate)

#### `UnlockPrecastUpdate()`

Lock the precast update

**Remarks:** Lock the precast update

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/_functions/#NemAll_Python_Precast.UnlockPrecastUpdate)

## HeightDefinitionType (enum)

Height definition types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/HeightDefinitionType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `HeightDefinitionType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/HeightDefinitionType/#NemAll_Python_Precast.HeightDefinitionType.__getitem__)

### Values
- `eAverage` = `3`
- `eComponent` = `2`
- `eMacro` = `1`
- `eNone` = `0`

## MacroSubType (enum)

Sub types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroSubType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `MacroSubType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroSubType/#NemAll_Python_Precast.MacroSubType.__getitem__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `MacroType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/MacroType/#NemAll_Python_Precast.MacroType.__getitem__)

### Values
- `eGroup_Fixture` = `3`
- `eLine_Fixture` = `1`
- `ePlane_Fixture` = `2`
- `ePoint_Fixture` = `0`
- `eUseSameType` = `-1`

## OutlineShape (enum)

outline shapes

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineShape/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `OutlineShape` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineShape/#NemAll_Python_Precast.OutlineShape.__getitem__)

### Values
- `eBUILTIN_OUTLINE_SHAPE_NOTHING` = `0`
- `eBUILTIN_OUTLINE_SHAPE_RECTANGLE` = `1`
- `eBUILTIN_OUTLINE_SHAPE_SYMBOL` = `3`
- `eBUILTIN_OUTLINE_SHAPE_TRAPEZOID` = `2`

## OutlineType (enum)

outline types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `OutlineType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineType/#NemAll_Python_Precast.OutlineType.__getitem__)

### Values
- `eBUILTIN_OUTLINE_TYPE_MINUS` = `2`
- `eBUILTIN_OUTLINE_TYPE_NOTHING` = `0`
- `eBUILTIN_OUTLINE_TYPE_NO_AFFECT` = `3`
- `eBUILTIN_OUTLINE_TYPE_PLUS` = `1`

## OutlineTypeInGroup (enum)

outline types in group

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineTypeInGroup/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `OutlineTypeInGroup` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/OutlineTypeInGroup/#NemAll_Python_Precast.OutlineTypeInGroup.__getitem__)

### Values
- `eBUILTIN_OUTLINE_TYPE_IN_GROUP_MINUS` = `2`
- `eBUILTIN_OUTLINE_TYPE_IN_GROUP_NOTHING` = `0`
- `eBUILTIN_OUTLINE_TYPE_IN_GROUP_PLUS` = `1`

## PrecastElement (class)

(No description provided in vendor docs for NemAll_Python_Precast.PrecastElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElement/)

### Constructors
- `PrecastElement(Properties)` — Constructor

### Methods
#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElement/#NemAll_Python_Precast.PrecastElement.__repr__)

### Properties
- `Properties` (None, get) — Property for Properties :type: None
- `deletePython` (None, get) — Property to delete Python after elementation :type: None

## PrecastElementProperties (class)

PrecastElementProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/)

### Constructors
- `PrecastElementProperties()` — Initialize

### Methods
#### `GetPrecastElementTypeFromIdx(arg2)`

Get the name of the PrecastElementType from Cat Idx

**Remarks:** Get the name of the PrecastElementType from Cat Idx

**Returns:** `str` — PrecastElementType Cat Name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.GetPrecastElementTypeFromIdx)

#### `SetElementTypeCatalogGUID_from_Name(arg2)`

Set the elementTypeCatGUID

**Remarks:** Set the elementTypeCatGUID

**Returns:** `GUID` — elementTypeCatGUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.SetElementTypeCatalogGUID_from_Name)

#### `SetFactoryCatalogAddressOffset(arg2)`

Set the factoryCatAddressOffset

**Remarks:** Set the factoryCatAddressOffset

**Returns:** `int` — factoryCatAddressOffset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.SetFactoryCatalogAddressOffset)

#### `SetNormCatalogAddressOffset(arg2)`

Set the normCatAddressOffset

**Remarks:** Set the normCatAddressOffset

**Returns:** `int` — normCatAddressOffset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.SetNormCatalogAddressOffset)

#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PrecastElementProperties) — PrecastElementProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastElementProperties/#NemAll_Python_Precast.PrecastElementProperties.__repr__)

### Properties
- `CreateLabeling` (None, get) — Property for CreateLabeling :type: None
- `DimensionCross` (None, get) — Property for DimensionCross :type: None
- `DimensionSpan` (None, get) — Property for DimensionSpan :type: None
- `DimensionViewing` (None, get) — Property for DimensionViewing :type: None
- `ElemTypeAttribut` (None, get) — Property for ElemTypeAttribut :type: None
- `ElementType` (None, get) — Property for ElementType :type: None
- `ElementTypeCatGUID` (None, get) — Property for ElementTypeCatGUID :type: None
- `Factory` (None, get) — Property for Factory :type: None
- `FactoryCatAddressOffset` (None, get) — Property for FactoryCatAddressOffset :type: None
- `LabelingTextRefPoint` (None, get) — Property for LabelingTextRefPoint :type: None
- `Layers` (None, get) — Property for Layers :type: None
- `ManualDimensions` (None, get) — Property for ManualDimensions :type: None
- `Norm` (None, get) — Property for Norm :type: None
- `NormCatAddressOffset` (None, get) — Property for NormCatAddressOffset :type: None
- `PieceFactor` (None, get) — Property for PieceFactor :type: None
- `PosNr` (None, get) — Property for PosNr :type: None
- `PosNrText` (None, get) — Property for PosNrText :type: None
- `ReferencePoint` (None, get) — Property for ReferencePoint :type: None
- `SpanDirection` (None, get) — Property for SpanDirection :type: None
- `ViewDirection` (None, get) — Property for ViewDirection :type: None

## PrecastLayer (class)

PrecastLayer class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayer/)

### Constructors
- `PrecastLayer()` — Initialize
- `PrecastLayer(Properties)` — Constructor

### Methods
#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayer/#NemAll_Python_Precast.PrecastLayer.__repr__)

### Properties
- `Properties` (None, get) — Property for Properties :type: None

## PrecastLayerProperties (class)

PrecastLayerProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/)

### Constructors
- `PrecastLayerProperties()` — Initialize

### Methods
#### `SetMaterialCatalogAddressOffset(arg2, arg3)`

Set the materialCatAddressOffset

**Remarks:** Set the materialCatAddressOffset

**Returns:** `int` — materialCatAddressOffset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/#NemAll_Python_Precast.PrecastLayerProperties.SetMaterialCatalogAddressOffset)

#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PrecastLayerProperties) — PrecastLayerProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/#NemAll_Python_Precast.PrecastLayerProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/PrecastLayerProperties/#NemAll_Python_Precast.PrecastLayerProperties.__repr__)

### Properties
- `CalculateLayerThickness` (None, get) — Property for CalculateLayerThickness :type: None
- `LayerName` (None, get) — Property for LayerName :type: None
- `LayerNumber` (None, get) — Property for LayerNumber :type: None
- `LayerThickness` (None, get) — Property for LayerThickness :type: None
- `Material` (None, get) — Property for Material :type: None
- `MaterialCatAddressOffset` (None, get) — Property for MaterialCatAddressOffset :type: None
- `MaterialType` (None, get) — Property for MaterialType :type: None

## ProfilType (enum)

profil types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/ProfilType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ProfilType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/ProfilType/#NemAll_Python_Precast.ProfilType.__getitem__)

### Values
- `eBUILTIN_PROFIL_TYPE_EDGE` = `1`
- `eBUILTIN_PROFIL_TYPE_JOINT` = `0`

## SubType (enum)

Sub types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/SubType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `SubType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/SubType/#NemAll_Python_Precast.SubType.__getitem__)

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

## Type (enum)

types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/Type/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `Type` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Precast/Type/#NemAll_Python_Precast.Type.__getitem__)

### Values
- `eGroup_Fixture` = `3`
- `eLine_Fixture` = `1`
- `ePlane_Fixture` = `2`
- `ePoint_Fixture` = `0`
- `eUseSameType` = `-1`

