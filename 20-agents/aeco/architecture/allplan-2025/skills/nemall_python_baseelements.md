---
name: allplan-nemall_python_baseelements
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_BaseElements namespace — 50 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, Attribute, AllplanElement, AssociationService, AttributeByteVec, AttributeDate, AttributeDataManager, AttributeDouble, and 42 more types.
---

# NemAll_Python_BaseElements

Auto-generated from vendor docs for allplan 2025.0. 50 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/)

### Methods
#### `GetAttributes() -> object`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetAttributes)

#### `GetBaseElementAdapter() -> BaseElementAdapter`

Get the model element

**Remarks:** Get the model element

**Returns:** `BaseElementAdapter` — Model element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetBaseElementAdapter)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetCommonProperties)

#### `GetGeometryObject() -> object`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetGeometryObject)

#### `GetLabelElements() -> list`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetLabelElements)

#### `GetSubElementID() -> type`

Get the SubElementID

**Remarks:** Get the SubElementID

**Returns:** `type` — SubElementID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer: object)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey: str)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject: object)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements: list)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## AssociationService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.AssociationService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AssociationService/)

### Methods
#### `AssociateObjectsWithPythonPart( doc: DocumentAdapter, objectUUIDs: list[str], pythonPartUUID: GUID )`

Associate the objects with the PythonPart

**Remarks:** Associate the objects with the PythonPart

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `objectUUIDs` (list[str]) — List with the object UUIDs
- `pythonPartUUID` (GUID) — UUID of the PythonPart

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AssociationService/#NemAll_Python_BaseElements.AssociationService.AssociateObjectsWithPythonPart)

## Attribute (class)

Base for all attribute definitions

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attribute/)

### Methods
#### `__eq__(element: Attribute) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (Attribute) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attribute/#NemAll_Python_BaseElements.Attribute.__eq__)

### Properties
- `Id` (int, get/set) — Get the attribute Id

## AttributeByteVec (class)

ByteVec attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/)

### Constructors
- `AttributeByteVec() | AttributeByteVec(id: int, value: VecByteList) | AttributeByteVec(element: AttributeByteVec)` — Initialize

### Methods
#### `__eq__(element: AttributeByteVec) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeByteVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/#NemAll_Python_BaseElements.AttributeByteVec.__eq__)

#### `__ne__(element: AttributeByteVec) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeByteVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/#NemAll_Python_BaseElements.AttributeByteVec.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/#NemAll_Python_BaseElements.AttributeByteVec.__repr__)

### Properties
- `Value` (list[int] | VecIntList, get/set) — Get the attribute value

## AttributeDataManager (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.AttributeDataManager.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDataManager/)

### Methods
#### `GetAttributeName(attributeID: int) -> str`

Get the attribute name from the ID

**Remarks:** Get the attribute name from the ID

**Parameters:**
- `attributeID` (int) — Attribute ID

**Returns:** `str` — Attribute name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDataManager/#NemAll_Python_BaseElements.AttributeDataManager.GetAttributeName)

## AttributeDate (class)

Date attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/)

### Constructors
- `AttributeDate() | AttributeDate(id: int, day: int, month: int, year: int) | AttributeDate(element: AttributeDate)` — Initialize

### Methods
#### `__eq__(element: AttributeDate) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeDate) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/#NemAll_Python_BaseElements.AttributeDate.__eq__)

#### `__ne__(element: AttributeDate) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeDate) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/#NemAll_Python_BaseElements.AttributeDate.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/#NemAll_Python_BaseElements.AttributeDate.__repr__)

### Properties
- `Day` (int, get/set) — Get the day value
- `Month` (int, get/set) — Get the month value
- `Year` (int, get/set) — Get the year value

## AttributeDouble (class)

Double attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/)

### Constructors
- `AttributeDouble() | AttributeDouble(id: int, value: float) | AttributeDouble(element: AttributeDouble)` — Initialize

### Methods
#### `__eq__(element: AttributeDouble) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeDouble) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/#NemAll_Python_BaseElements.AttributeDouble.__eq__)

#### `__ne__(element: AttributeDouble) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeDouble) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/#NemAll_Python_BaseElements.AttributeDouble.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/#NemAll_Python_BaseElements.AttributeDouble.__repr__)

### Properties
- `Value` (float, get/set) — Get the attribute value

## AttributeDoubleVec (class)

DoubleVec attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/)

### Constructors
- `AttributeDoubleVec() | AttributeDoubleVec(id: int, value: VecDoubleList) | AttributeDoubleVec(element: AttributeDoubleVec)` — Initialize

### Methods
#### `__eq__(element: AttributeDoubleVec) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeDoubleVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/#NemAll_Python_BaseElements.AttributeDoubleVec.__eq__)

#### `__ne__(element: AttributeDoubleVec) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeDoubleVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/#NemAll_Python_BaseElements.AttributeDoubleVec.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/#NemAll_Python_BaseElements.AttributeDoubleVec.__repr__)

### Properties
- `Value` (VecDoubleList, get/set) — Get the attribute value

## AttributeEnum (class)

Enum attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/)

### Constructors
- `AttributeEnum() | AttributeEnum(id: int, value: int) | AttributeEnum(element: AttributeEnum)` — Initialize

### Methods
#### `__eq__(element: AttributeEnum) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeEnum) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/#NemAll_Python_BaseElements.AttributeEnum.__eq__)

#### `__ne__(element: AttributeEnum) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeEnum) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/#NemAll_Python_BaseElements.AttributeEnum.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/#NemAll_Python_BaseElements.AttributeEnum.__repr__)

### Properties
- `Value` (int, get/set) — Get the attribute value

## AttributeInteger (class)

Integer attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/)

### Constructors
- `AttributeInteger() | AttributeInteger(id: int, value: int) | AttributeInteger(element: AttributeInteger)` — Initialize

### Methods
#### `__eq__(element: AttributeInteger) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeInteger) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/#NemAll_Python_BaseElements.AttributeInteger.__eq__)

#### `__ne__(element: AttributeInteger) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeInteger) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/#NemAll_Python_BaseElements.AttributeInteger.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/#NemAll_Python_BaseElements.AttributeInteger.__repr__)

### Properties
- `Value` (int, get/set) — Get the attribute value

## AttributeIntegerVec (class)

IntegerVec attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/)

### Constructors
- `AttributeIntegerVec() | AttributeIntegerVec(id: int, value: list[int] | AttributeIntegerVec() | AttributeIntegerVec(element: AttributeIntegerVec)` — Initialize

### Methods
#### `__eq__(element: AttributeIntegerVec) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeIntegerVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/#NemAll_Python_BaseElements.AttributeIntegerVec.__eq__)

#### `__ne__(element: AttributeIntegerVec) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeIntegerVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/#NemAll_Python_BaseElements.AttributeIntegerVec.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/#NemAll_Python_BaseElements.AttributeIntegerVec.__repr__)

### Properties
- `Value` (list[int] | VecIntList, get/set) — Get the attribute value

## AttributeService (class)

Service for reading existing attributes definitions and creating new user-defined attributes

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/)

### Methods
#### `AddUserAttribute( doc: DocumentAdapter, attributeType: AttributeType, attributeName: str, attributeDefaultValue: str, attributeMinValue: float, attributeMaxValue: float, attributeDimension: str, attributeCtrlType: AttributeControlType, attributeListValues: VecStringList, ) -> int`

Add a user attribute

**Remarks:** Add a user attribute

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeType` (AttributeType) — Type
- `attributeName` (str) — Name
- `attributeDefaultValue` (str) — Default value
- `attributeMinValue` (float) — Minimal value
- `attributeMaxValue` (float) — Maximal value
- `attributeDimension` (str) — Dimension
- `attributeCtrlType` (AttributeControlType) — Control type
- `attributeListValues` (VecStringList) — List with the allowed attribute values

**Returns:** `int` — ID of the added user attribute, -1 = not possible to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.AddUserAttribute)

#### `GetAttributeControlType( doc: DocumentAdapter, attributeID: int ) -> AttributeControlType`

Get the control type of the attribute

**Remarks:** Get the control type of the attribute

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `AttributeControlType` — Control type of the attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeControlType)

#### `GetAttributeID(doc: DocumentAdapter, attributeName: str) -> int`

Get the attribute ID

**Remarks:** Get the attribute ID

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeName` (str) — Attribute name

**Returns:** `int` — Attribute ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeID)

#### `GetAttributeName(doc: DocumentAdapter, attributeID: int) -> str`

Get the attribute name

**Remarks:** Get the attribute name

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `str` — Attribute name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeName)

#### `GetAttributeType(doc: DocumentAdapter, attributeID: int) -> AttributeType`

Get the attribute type

**Remarks:** Get the attribute type

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `AttributeType` — Attribute type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeType)

#### `GetAttributeUnit(doc: DocumentAdapter, attributeID: int) -> str`

Get the attribute unit

**Remarks:** Get the attribute unit

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `str` — Attribute unit

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeUnit)

#### `GetDefaultValue(doc: DocumentAdapter, attributeID: int) -> int | float | str`

Get the default value of an attribute

**Remarks:** Get the default value of an attribute

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `int | float | str` — Default value of the attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetDefaultValue)

#### `GetEnumIDFromValueString(attributeID: int, valueString: str) -> int`

Get the enumeration ID from the value string

**Remarks:** Get the enumeration ID from the value string

**Parameters:**
- `attributeID` (int) — Attribute ID
- `valueString` (str) — Value string

**Returns:** `int` — Enumeration ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetEnumIDFromValueString)

#### `GetEnumValueStringFromID(attributeID: int, enumID: int) -> str`

Get the enumeration value string from the enumeration ID

**Remarks:** Get the enumeration value string from the enumeration ID

**Parameters:**
- `attributeID` (int) — Attribute ID
- `enumID` (int) — Enumeration ID

**Returns:** `str` — Enumeration value string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetEnumValueStringFromID)

#### `GetEnumValues(doc: DocumentAdapter, attributeID: int) -> VecStringList`

Get the enum attribute values

**Remarks:** Get the enum attribute values

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `VecStringList` — Default attribute value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetEnumValues)

#### `GetGroupAttributeIDs( ele: BaseElementAdapter, attributes: list[tuple[int, int | float | str]], excludeHidden: bool, ) -> list[tuple[str, list[int] | VecIntList]]`

Get the attribute IDs and the name of the attribute groups

**Remarks:** Get the attribute IDs and the name of the attribute groups

**Parameters:**
- `ele` (BaseElementAdapter) — Element for the attributes
- `attributes` (list[tuple[int, int | float | str]]) — Attributes of the element
- `excludeHidden` (bool) — Exclude the hidden attributes

**Returns:** `list[tuple[str, list[int] | VecIntList]]` — Group name, group IDs

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetGroupAttributeIDs)

#### `GetInputListValues(doc: DocumentAdapter, attributeID: int) -> VecStringList`

Get the input list values

**Remarks:** Get the input list values

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `VecStringList` — Attribute input list values

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetInputListValues)

#### `OpenAttributeSelectionDialog( doc: DocumentAdapter, dialogType: AttributeSelectionDialogType ) -> int`

Open the attribute selection dialog

**Remarks:** Open the attribute selection dialog

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `dialogType` (AttributeSelectionDialogType) — Dialog type

**Returns:** `int` — Attribute ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.OpenAttributeSelectionDialog)

## AttributeSet (class)

Attribute set

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/)

### Constructors
- `AttributeSet() | AttributeSet(elements: list) | AttributeSet(element: AttributeSet)` — Initialize

### Methods
#### `GetAttributes() -> ( list[ AttributeByteVec | AttributeDate | AttributeDouble | AttributeDoubleVec | AttributeEnum | AttributeInteger | AttributeIntegerVec | AttributeString | AttributeStringVec ] )`

Get the AttributeSet vector

**Remarks:** Get the AttributeSet vector

**Returns:** `list[AttributeByteVec | AttributeDate | AttributeDouble | AttributeDoubleVec | AttributeEnum | AttributeInteger | AttributeIntegerVec | AttributeString | AttributeStringVec]` — Vector of Attribute elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.GetAttributes)

#### `SetAttributes( elements: list[ AttributeByteVec | AttributeDate | AttributeDouble | AttributeDoubleVec | AttributeEnum | AttributeInteger | AttributeIntegerVec | AttributeString | AttributeStringVec ], )`

Set the AttributeSet vector

**Remarks:** Set the AttributeSet vector

**Parameters:**
- `elements` (list[AttributeByteVec | AttributeDate | AttributeDouble | AttributeDoubleVec | AttributeEnum | AttributeInteger | AttributeIntegerVec | AttributeString | AttributeStringVec]) — Attribute elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.SetAttributes)

#### `__eq__(element: AttributeSet) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeSet) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.__eq__)

#### `__ne__(element: AttributeSet) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeSet) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.__repr__)

### Properties
- `Attributes` (list[ AttributeByteVec | AttributeDate | AttributeDouble | AttributeDoubleVec | AttributeEnum | AttributeInteger | AttributeIntegerVec | AttributeString | AttributeStringVec ], get/set) — Get the AttributeSet vector

## AttributeString (class)

String attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/)

### Constructors
- `AttributeString() | AttributeString(id: int, value: str) | AttributeString(element: AttributeString)` — Initialize

### Methods
#### `__eq__(element: AttributeString) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeString) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/#NemAll_Python_BaseElements.AttributeString.__eq__)

#### `__ne__(element: AttributeString) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeString) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/#NemAll_Python_BaseElements.AttributeString.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/#NemAll_Python_BaseElements.AttributeString.__repr__)

### Properties
- `Value` (str, get/set) — Get the attribute value

## AttributeStringVec (class)

StringVec attribute

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/)

### Constructors
- `AttributeStringVec() | AttributeStringVec(id: int, value: VecStringList) | AttributeStringVec(element: AttributeStringVec)` — Initialize

### Methods
#### `__eq__(element: AttributeStringVec) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeStringVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/#NemAll_Python_BaseElements.AttributeStringVec.__eq__)

#### `__ne__(element: AttributeStringVec) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeStringVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/#NemAll_Python_BaseElements.AttributeStringVec.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/#NemAll_Python_BaseElements.AttributeStringVec.__repr__)

### Properties
- `Value` (VecStringList, get/set) — Get the attribute value

## Attributes (class)

Attributes class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/)

### Constructors
- `Attributes() | Attributes(elements: list)` — Initialize

### Methods
#### `GetAttributeSets() -> list`

Get the AttributeSet vector

**Remarks:** Get the AttributeSet vector

**Returns:** `list` — Attribute value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.GetAttributeSets)

#### `SetAttributeSets(sets: list)`

Set the AttributeSet vector

**Remarks:** Set the AttributeSet vector

**Parameters:**
- `sets` (list) — AttributeSet vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.SetAttributeSets)

#### `__eq__(props: Attributes) -> bool`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (Attributes) — Properties to compare

**Returns:** `bool` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.__repr__)

### Properties
- `AttributeSets` (None, get) — Property for attribute set vector :type: None

## CadDataFileReader (class)

Utility for reading data from common CAD files, like IFC or DWG, and converting them into Python objects (like ModelElement3D), but without creating these objects in the drawing file. In this way, the objects can be processed further inside the script e.g., become a part of the created PythonPart.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/)

### Methods
#### `ReadIFC( doc: DocumentAdapter, fileName: str ) -> list[tuple[str, list[ModelElement3D]]]`

Import the data from an IFC file and return it as Python objects, without creating them in the drawing file. The method groups objects, that are sub objects of an IfcElementAssembly, into tuples. If there is no IfcElementAssembly entity in the IFC file, one tuple with all objects is returned. IMPORTANT: Only objects with the IfcType of IfcBuildingElementProxy are read!

**Remarks:** Import the data from an IFC file and return it as Python objects, without creating them in the drawing file. The method groups objects, that are sub objects of an IfcElementAssembly, into tuples. If there is no IfcElementAssembly entity in the IFC file, one tuple with all objects is returned. IMPORTANT: Only objects with the IfcType of IfcBuildingElementProxy are read!

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Full path to the IFC file

**Returns:** `list[tuple[str, list[ModelElement3D]]]` — Tuples like (group name, model elements)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/#NemAll_Python_BaseElements.CadDataFileReader.ReadIFC)

#### `ReadOBJ( fileName: str, designPathLocation: eDesignPathLocation ) -> list[ModelElement3D]`

Import an OBJ file

**Remarks:** Import an OBJ file

**Parameters:**
- `fileName` (str) — Full file name of the OBJ file
- `designPathLocation` (eDesignPathLocation) — Location of the design path

**Returns:** `list[ModelElement3D]` — List with the model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/#NemAll_Python_BaseElements.CadDataFileReader.ReadOBJ)

#### `ReadSKP( fileName: str, designPathLocation: eDesignPathLocation ) -> list[ModelElement3D]`

Import an SKP file

**Remarks:** Import an SKP file

**Parameters:**
- `fileName` (str) — Full file name of the SKP file
- `designPathLocation` (eDesignPathLocation) — Location of the design path

**Returns:** `list[ModelElement3D]` — List with the model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/#NemAll_Python_BaseElements.CadDataFileReader.ReadSKP)

## CommonProperties (class)

Representation of format properties, common for all kind of Allplan elements, such as General elements, architectural components or reinforcement. This class contains information about e.g. pen thickness, stroke type and color, with which an Allplan element is drawn in the viewport.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/)

### Constructors
- `CommonProperties() | CommonProperties(element: CommonProperties)` — Initialize

### Methods
#### `GetColorPenStrokeByLayerFromLayerNumber( layernumber: int, ) -> list[int] | VecIntList`

Layer number

**Remarks:** Layer number

**Returns:** `list[int] | VecIntList` — [0] = pen,[1] = stroke, [2] color

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.GetColorPenStrokeByLayerFromLayerNumber)

#### `GetGlobalProperties()`

Get the global properties

**Remarks:** Get the global properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.GetGlobalProperties)

#### `__eq__(props: CommonProperties) -> object`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (CommonProperties) — Properties to compare

**Returns:** `object` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.__repr__)

### Properties
- `Activated` (bool, get/set) — Get the activated state
- `Color` (int, get/set) — Get the color
- `ColorByLayer` (bool, get/set) — Get the color by layer state
- `ConstructionLine` (bool, get/set) — Get the construction line state
- `DrawOrder` (int, get/set) — Get the drawOrder
- `ForceColor` (bool, get/set) — Get flag if force color is set
- `HelpConstruction` (bool, get/set) — Get flag if pen and stroke is help construction
- `InSpecialWindow` (bool, get/set) — Get the special window visibility
- `Layer` (int, get/set) — Get the layer
- `Marked` (bool, get/set) — Get the marked state
- `PageNumber` (int, get/set) — Get the page number
- `Pen` (int, get/set) — Get the pen
- `PenByLayer` (bool, get/set) — Get the pen by layer state
- `Printable` (bool, get/set) — Get the printable state
- `Stroke` (int, get/set) — Get the stroke
- `StrokeByLayer` (bool, get/set) — Get the stroke by layer state
- `Visible` (bool, get/set) — Get the visible flag state
- `VisibleInsideZoomwindows` (bool, get/set) — Get the visible inside zoom window state
- `VisibleOutsideZoomwindows` (bool, get/set) — Get the visible outside zoom window state

## DocumentResourceService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.DocumentResourceService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DocumentResourceService/)

### Methods
#### `CreateSurface( doc: DocumentAdapter, surfaceDirectoryPath: str, localPathAndName: str, surfaceDef: SurfaceDefinition, createUniqueName: bool = True, bitmapDefinitions: Dict[eSurfaceTextureID, BitmapDefinition] = {}, ) -> str`

Create a surface resource with a unique name

**Remarks:** Create a surface resource with a unique name

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `surfaceDirectoryPath` (str) — Path to the surface directory (e.g. ...\std\design)
- `localPathAndName` (str) — Local path and name (extends the surface directory path)
- `surfaceDef` (SurfaceDefinition) — Surface definition
- `createUniqueName` (bool) — create a unique name state
- `bitmapDefinitions` (Dict[eSurfaceTextureID, BitmapDefinition]) — Bitmap definitions

**Returns:** `str` — Unique name extension of the surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DocumentResourceService/#NemAll_Python_BaseElements.DocumentResourceService.CreateSurface)

## DrawingFileLoadState (enum)

Load state of the drawing file

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileLoadState/)

### Values
- `ActiveBackground` = `2`
- `ActiveForeground` = `3`
- `PassiveBackground` = `1`

## DrawingFileService (class)

Service for processing the drawing files

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/)

### Constructors
- `DrawingFileService()` — Initialize

### Methods
#### `CreateBendingSchedule(doc: DocumentAdapter, refPnt: Point2D)`

Create a bending schedule

**Remarks:** Create a bending schedule

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `refPnt` (Point2D) — Reference point of the bending schedule

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.CreateBendingSchedule)

#### `DeleteDocument(doc: DocumentAdapter)`

Delete the context of the active document Args: doc: Document

**Remarks:** Delete the context of the active document Args: doc: Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.DeleteDocument)

#### `ExportBendingMachine( doc: DocumentAdapter, fileName: str, project: str, plan: str, index: str, bSplitGroups: bool, )`

Export the reinforcement data for the bending machine

**Remarks:** Export the reinforcement data for the bending machine

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the output file
- `project` (str) — Name of the project
- `plan` (str) — Name of the plan
- `index` (str) — Index as text
- `bSplitGroups` (bool) — Split the reinforcement groups

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.ExportBendingMachine)

#### `GetActiveFileNumber() -> int`

Get the drawing file number of the active document

**Remarks:** Get the drawing file number of the active document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.GetActiveFileNumber)

#### `GetFileState() -> list[tuple[int, DrawingFileLoadState]]`

Get the file state of all loaded drawing files Returns: list with a tuple(file index, drawing file load state)

**Remarks:** Get the file state of all loaded drawing files Returns: list with a tuple(file index, drawing file load state)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.GetFileState)

#### `LoadFile(doc: DocumentAdapter, fileIndex: int, loadState: DrawingFileLoadState)`

Load a drawing file

**Remarks:** Load a drawing file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileIndex` (int) — Index of the drawing file
- `loadState` (DrawingFileLoadState) — File load state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.LoadFile)

#### `SetScalingFactor(arg2: DocumentAdapter, arg3: float)`

Set the scaling factor of the current document

**Remarks:** Set the scaling factor of the current document

**Parameters:**
- `arg2` (DocumentAdapter) — Document adapter
- `arg3` (float) — Scaling factor. For a scale of 1:20, set 20.0. For a scale 2:1, set 0.5

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.SetScalingFactor)

#### `UnloadAll(doc: DocumentAdapter)`

Unload all drawing files

**Remarks:** Unload all drawing files

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.UnloadAll)

#### `UnloadFile(doc: DocumentAdapter, fileIndex: int)`

Unload a drawing file

**Remarks:** Unload a drawing file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileIndex` (int) — Index of the drawing file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.UnloadFile)

## DrawingService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.DrawingService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/)

### Methods
#### `LockGraphicsEngineUpdate(doc: DocumentAdapter, lockUpdate: bool)`

Lock the update of the graphics engine

**Remarks:** Lock the update of the graphics engine

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `lockUpdate` (bool) — Set to True to lock the update of the graphics engine. Set to False to unlock and update it.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.LockGraphicsEngineUpdate)

#### `RedrawAll(doc: DocumentAdapter)`

Redraw all Args: doc Document

**Remarks:** Redraw all Args: doc Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.RedrawAll)

#### `ResetAndDrawHiddenElement( doc: DocumentAdapter, hiddenElement: BaseElementAdapter )`

Reset and draw the hidden elements Args: doc Document hiddenElement Hidden element

**Remarks:** Reset and draw the hidden elements Args: doc Document hiddenElement Hidden element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.ResetAndDrawHiddenElement)

#### `SaveWindowToImageFile( fileName: str, pixelWidth: int = 0, pixelHeight: int = 0 ) -> bool`

Save the window to an image file

**Remarks:** Save the window to an image file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.SaveWindowToImageFile)

#### `UpdateAllWindows()`

Update the drawing in all windows

**Remarks:** Update the drawing in all windows

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.UpdateAllWindows)

#### `UpdateGraphicsEngine(doc: DocumentAdapter)`

Update the graphics engine Args: doc Document

**Remarks:** Update the graphics engine Args: doc Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.UpdateGraphicsEngine)

## DrawingTypeService (class)

Utility for processing the drawing type.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/)

### Methods
#### `GetCurrentDrawingTypeDescription(doc: DocumentAdapter) -> str`

Get the description of the current drawing type

**Remarks:** Get the description of the current drawing type

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `str` — Description of the current drawing type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetCurrentDrawingTypeDescription)

#### `GetCurrentDrawingTypeId(doc: DocumentAdapter) -> int`

Get the current drawing type ID

**Remarks:** Get the current drawing type ID

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `int` — Current drawing type ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetCurrentDrawingTypeId)

#### `GetDrawingTypeDescription(doc: DocumentAdapter, drawingTypeId: int) -> str`

Get the description of the drawing type

**Remarks:** Get the description of the drawing type

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `drawingTypeId` (int) — Drawing type ID

**Returns:** `str` — Description of the drawing type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetDrawingTypeDescription)

#### `GetDrawingTypeDescriptions(doc: DocumentAdapter) -> VecStringList`

Get the description of the drawing types

**Remarks:** Get the description of the drawing types

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `VecStringList` — Drawing type desccriptions

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetDrawingTypeDescriptions)

#### `GetDrawingTypeIdFromDescription( doc: DocumentAdapter, typeDescription: str ) -> int`

Get the ID of the drawing type from the description

**Remarks:** Get the ID of the drawing type from the description

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `typeDescription` (str) — Drawing type description

**Returns:** `int` — ID of the drawing type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetDrawingTypeIdFromDescription)

#### `SetDrawingTypeId(drawingTypeId: int)`

Set the drawing type ID

**Remarks:** Set the drawing type ID

**Parameters:**
- `drawingTypeId` (int) — ID of the drawing type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.SetDrawingTypeId)

## ElementsAttributeService (class)

Service for processing attributes of existing model elements. The service provides methods to e.g. read or modify attribute values.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/)

### Methods
#### `ChangeAttribute( attributeNumber: int, newValue: object, elements: BaseElementAdapterList ) -> BaseElementAdapterList`

Change an attribute

**Remarks:** Change an attribute

**Parameters:**
- `attributeNumber` (int) — Attribute number
- `newValue` (object) — New value
- `elements` (BaseElementAdapterList) — Elements

**Returns:** `BaseElementAdapterList` — Adapted elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/#NemAll_Python_BaseElements.ElementsAttributeService.ChangeAttribute)

#### `ChangeAttributes( attributeDataList: list, elements: BaseElementAdapterList ) -> BaseElementAdapterList`

Change an attribute

**Remarks:** Change an attribute

**Parameters:**
- `attributeDataList` (list) — List with the attribute data as tuple(number, value)
- `elements` (BaseElementAdapterList) — Elements as BaseElementAdapterList

**Returns:** `BaseElementAdapterList` — Modified elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/#NemAll_Python_BaseElements.ElementsAttributeService.ChangeAttributes)

#### `GetAttributes( ele: BaseElementAdapter, readState: eAttibuteReadState = ReadAll ) -> list`

Get the attributes from an element

**Remarks:** Get the attributes from an element

**Parameters:**
- `ele` (BaseElementAdapter) — Element adapter
- `readState` (eAttibuteReadState) — What attributes to read

**Returns:** `list` — Attributes

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/#NemAll_Python_BaseElements.ElementsAttributeService.GetAttributes)

## ElementsByAttributeService (class)

Service for selecting elements in the current document based on the value of a specific attribute. The ID of this attribute must be given during the service initialization. The service is a singleton, meaning that there can only exist one instance of it.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/)

### Methods
#### `GetElements(attributeValue: float) -> BaseElementAdapterList | GetElements(attributeValue: int) -> BaseElementAdapterList | GetElements(attributeValue: str) -> BaseElementAdapterList`

Get the elements for the double attribute value

**Remarks:** Get the elements for the double attribute value

**Parameters:**
- `attributeValue` (float) — Attribute value

**Returns:** `BaseElementAdapterList` — Elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/#NemAll_Python_BaseElements.ElementsByAttributeService.GetElements)

#### `GetInstance() -> ElementsByAttributeService`

Get the instance

**Remarks:** Get the instance

**Returns:** `ElementsByAttributeService` — Instance of the singleton

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/#NemAll_Python_BaseElements.ElementsByAttributeService.GetInstance)

#### `Init(attributeID: int, doc: DocumentAdapter)`

Initialize the service.

**Remarks:** Initialize the service.

**Parameters:**
- `attributeID` (int) — Attribute ID. Used for creating fast element access by an attribute value of this ID
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/#NemAll_Python_BaseElements.ElementsByAttributeService.Init)

## ElementsLayerService (class)

Service for processing (changing) the layer of existing model elements.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsLayerService/)

### Methods
#### `ChangeLayer( elements: BaseElementAdapterList, newValue: str ) -> BaseElementAdapterList`

Change the Layer

**Remarks:** Change the Layer

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to change
- `newValue` (str) — New value

**Returns:** `BaseElementAdapterList` — List with the modified elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsLayerService/#NemAll_Python_BaseElements.ElementsLayerService.ChangeLayer)

## ElementsPropertyService (class)

Service for modifying various properties of existing model elements, such as surface.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsPropertyService/)

### Methods
#### `ModifyFormatProperties(arg2: BaseElementAdapterList) -> BaseElementAdapterList`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsPropertyService/#NemAll_Python_BaseElements.ElementsPropertyService.ModifyFormatProperties)

#### `ModifySurface( surfacePathFile: str, elements: BaseElementAdapterList ) -> BaseElementAdapterList`

Change the surface

**Remarks:** Change the surface

**Parameters:**
- `surfacePathFile` (str) — New surface path and file
- `elements` (BaseElementAdapterList) — Elements to modify

**Returns:** `BaseElementAdapterList` — Modified elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsPropertyService/#NemAll_Python_BaseElements.ElementsPropertyService.ModifySurface)

## ElementsSelectService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ElementsSelectService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/)

### Methods
#### `SelectAllElements(ele: DocumentAdapter) -> BaseElementAdapterList`

Select all elements from the document

**Remarks:** Select all elements from the document

**Parameters:**
- `doc` (object) — Document

**Returns:** `BaseElementAdapterList` — All elements from the document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/#NemAll_Python_BaseElements.ElementsSelectService.SelectAllElements)

#### `SelectElementsByIfcGuid(ele: list) -> BaseElementAdapterList`

Select elements by IFC guids

**Remarks:** Select elements by IFC guids

**Parameters:**
- `doc` (object) — Document
- `ifcGuids` (object) — List with the IFC GUIDs

**Returns:** `BaseElementAdapterList` — Elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/#NemAll_Python_BaseElements.ElementsSelectService.SelectElementsByIfcGuid)

## ExportImportService (class)

Service for exporting/importing CAD data in common formats, like IFC or DWG.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/)

### Constructors
- `ExportImportService() | ExportImportService(element: ExportImportService)` — Initialize

### Methods
#### `ExportDWG( doc: DocumentAdapter, fileName: str, configFileName: str, version: int )`

Export the data to a DWG file by a configuration file

**Remarks:** Export the data to a DWG file by a configuration file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file
- `version` (int) — Version numer

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportDWG)

#### `ExportDWGByTheme( doc: DocumentAdapter, fileName: str, themeFileName: str, version: int = 2018 )`

Export the data to a DWG file by a theme file

**Remarks:** Export the data to a DWG file by a theme file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `themeFileName` (str) — Name of the DWG theme file
- `version` (int) — DWG version

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportDWGByTheme)

#### `ExportIFC( doc: DocumentAdapter, fileNumbers: list[int] | VecIntList, ifcVersion: IFC_Version, fileName: str, ifcThemeFileName: str = "", )`

Export the data to an IFC file

**Remarks:** Export the data to an IFC file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileNumbers` (list[int] | VecIntList) — Numbers of the drawing files to export
- `ifcVersion` (IFC_Version) — IFC version
- `fileName` (str) — Name of the IFC file
- `ifcThemeFileName` (str) — Name of the theme file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportIFC)

#### `ExportXPlan( doc: DocumentAdapter, fileNumbers: list[int] | VecIntList, xplanFilePath: str, )`

Export the data to a XPlan file

**Remarks:** Export the data to a XPlan file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileNumbers` (list[int] | VecIntList) — Numbers of the drawing files to export
- `xplanFilePath` (str) — Path and name of the XPlan file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportXPlan)

#### `ImportDWG( doc: DocumentAdapter, fileName: str, configFileName: str, placePnt: Point3D ) -> BaseElementAdapterList`

Import the data from a DWG file

**Remarks:** Import the data from a DWG file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file
- `placePnt` (Point3D) — Placement point of the data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ImportDWG)

#### `ImportIFC( doc: DocumentAdapter, fileNumber: int, fileName: str ) -> BaseElementAdapterList`

Import the data from an IFC file

**Remarks:** Import the data from an IFC file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileNumber` (int) — Number of the drawing file for the data import
- `fileName` (str) — Name of the IFC file

**Returns:** `BaseElementAdapterList` — List with the imported elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ImportIFC)

#### `ImportXPlan(doc: DocumentAdapter, xplanFilePath: str, drawingFileNumber: int)`

Import the data from a XPlan file

**Remarks:** Import the data from a XPlan file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `xplanFilePath` (str) — Path and name of the XPlan file to import
- `drawingFileNumber` (int) — Starting number of destination drawing file(s)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ImportXPlan)

## FaceSelectService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.FaceSelectService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/)

### Methods
#### `SelectPolyhedronFace( polyhedron: BaseElementAdapter, pnt: Point2D, highlightFace: bool, viewProj: ViewWorldProjection, doc: DocumentAdapter, includeUVSSelection: bool, ) -> tuple[bool, Polygon3D, IntersectionRayPolyhedron]`

Select a polyhedron face

**Remarks:** Select a polyhedron face

**Parameters:**
- `polyhedron` (BaseElementAdapter) — Polyhedron
- `pnt` (Point2D) — Selection view point
- `highlightFace` (bool) — Highlight the face state
- `viewProj` (ViewWorldProjection) — View world projection of the selected view
- `doc` (DocumentAdapter) — Document
- `includeUVSSelection` (bool) — Include an UVS selection for the face

**Returns:** `tuple[bool, Polygon3D, IntersectionRayPolyhedron]` — select state, face polygon, intersection result

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectPolyhedronFace)

#### `SelectPolyhedronFaceInUVS( polyhedron: BaseElementAdapter, pnt: Point2D, highlightFace: bool, viewProj: ViewWorldProjection, doc: DocumentAdapter, ) -> tuple[bool, Polygon3D, IntersectionRayPolyhedron, Matrix3D]`

Select a polyhedron face in an UVS

**Remarks:** Select a polyhedron face in an UVS

**Parameters:**
- `polyhedron` (BaseElementAdapter) — Polyhedron
- `pnt` (Point2D) — Selection view point
- `highlightFace` (bool) — Highlight the face state
- `viewProj` (ViewWorldProjection) — View world projection of the selected view
- `doc` (DocumentAdapter) — Document

**Returns:** `tuple[bool, Polygon3D, IntersectionRayPolyhedron, Matrix3D]` — select state, face polygon, intersection result, UVS matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectPolyhedronFaceInUVS)

#### `SelectWallFace( wallTier: BaseElementAdapter, pnt: Point2D, highlightFace: bool, viewProj: ViewWorldProjection, doc: DocumentAdapter, includeUVSSelection: bool, ) -> tuple[bool, Polygon3D, IntersectionRayPolyhedron]`

Select a wall face

**Remarks:** Select a wall face

**Parameters:**
- `wallTier` (BaseElementAdapter) — Wall tier
- `pnt` (Point2D) — Selection view point
- `highlightFace` (bool) — Highlight the face state
- `viewProj` (ViewWorldProjection) — View world projection of the selected view
- `doc` (DocumentAdapter) — Document
- `includeUVSSelection` (bool) — Include an UVS selection for the face

**Returns:** `tuple[bool, Polygon3D, IntersectionRayPolyhedron]` — select state, face polygon, intersection result

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectWallFace)

#### `SelectWallFaceInUVS( wallTier: BaseElementAdapter, pnt: Point2D, highlightFace: bool, viewProj: ViewWorldProjection, doc: DocumentAdapter, ) -> tuple[bool, Polygon3D, IntersectionRayPolyhedron, Matrix3D]`

Select a wall face in an UVS

**Remarks:** Select a wall face in an UVS

**Parameters:**
- `wallTier` (BaseElementAdapter) — Wall tier
- `pnt` (Point2D) — Selection view point
- `highlightFace` (bool) — Highlight the face state
- `viewProj` (ViewWorldProjection) — View world projection of the selected view
- `doc` (DocumentAdapter) — Document

**Returns:** `tuple[bool, Polygon3D, IntersectionRayPolyhedron, Matrix3D]` — select state, face polygon, intersection result, UVS matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectWallFaceInUVS)

## Functions (static-class)

Module-level functions of NemAll_Python_BaseElements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/)

### Methods
#### `ClearElementPreview()`

Clear the element preview

**Remarks:** Clear the element preview

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ClearElementPreview)

#### `CloseElementPreview()`

Close the element preview

**Remarks:** Close the element preview

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CloseElementPreview)

#### `CopyElements( doc: DocumentAdapter, elements: BaseElementAdapterList, fromPoint: Point3D, distanceVector: Vector3D, rotationVector: Vector3D, rotationAngle: float, numberOfCopies: int, viewProj: ViewWorldProjection, ) -> BaseElementAdapterList`

Copy the elements

**Remarks:** Copy the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — List with the elements
- `fromPoint` (Point3D) — From point
- `distanceVector` (Vector3D) — Distance vector
- `rotationVector` (Vector3D) — Rotation vector
- `rotationAngle` (float) — Rotation angle in radian
- `numberOfCopies` (int) — Number for copies
- `viewProj` (ViewWorldProjection) — View world projection

**Returns:** `BaseElementAdapterList` — List with the copied elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CopyElements)

#### `CreateAssociativeViews( doc: DocumentAdapter, insertionMat: Matrix3D, elements: BaseElementAdapterList, assoViewList: list, viewProj: ViewWorldProjection, ) -> MinMax2DList`

Create associative views

**Remarks:** Create associative views

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Placment matrix
- `elements` (BaseElementAdapterList) — List with the elements
- `assoViewList` (list) — List with the associative views
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CreateAssociativeViews)

#### `CreateBarCoupler( arg2: BaseElementAdapterList, arg3: str, arg4: str, arg5: str, arg6: str, arg7: bool, )`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CreateBarCoupler)

#### `CreateElements( doc: DocumentAdapter, insertionMat: Matrix3D, modelEleList: list, modelUuidList: list, assoRefObj: object, appendReinfPosNr: bool = True, createUndoStep: bool = True, ) -> BaseElementAdapterList`

Create the elements in the drawing file

**Remarks:** Create the elements in the drawing file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Matrix with the placement point and the rotation
- `modelEleList` (list) — List with the model elements modelUuidList List with the model UUIDS in modification mode
- `assoRefObj` (object) — reference to an associative view
- `appendReinfPosNr` (bool) — Set to True to append the reinforcement position numbers to the existing position numbers. Set to False to try to use the original reinforcement position numbers
- `createUndoStep` (bool) — Create an undo step after the creation of the PythonPart

**Returns:** `BaseElementAdapterList` — List with the created elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CreateElements)

#### `CreateLayer( doc: DocumentAdapter, groupName: str, subGroupName: str, longName: str, shortName: str, lineColorID: int, lineThicknessID: int, lineStyleID: int, bVisible: bool, bModifiable: bool, ) -> int`

Create a new layer

**Remarks:** Create a new layer

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `groupName` (str) — Group name
- `subGroupName` (str) — Subgroup name
- `lineColorID` (int) — Line color ID of the layer
- `lineThicknessID` (int) — Line thickness ID of the layer
- `lineStyleID` (int) — Line style ID of the layer
- `bVisible` (bool) — Layer is visible
- `bModifiable` (bool) — Layer is modifiable

**Returns:** `int` — Created layer number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CreateLayer)

#### `CreateLibraryElement( doc: DocumentAdapter, insertionMat: Matrix3D, path: str, elementName: str ) -> BaseElementAdapterList`

Create a library element in the data base

**Remarks:** Create a library element in the data base

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Matrix with the placement point and the rotation
- `path` (str) — Path of the library element
- `elementName` (str) — Name of the library element

**Returns:** `BaseElementAdapterList` — List with the created elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CreateLibraryElement)

#### `CreateSectionsAndViews( doc: DocumentAdapter, insertionMat: Matrix3D, elements: BaseElementAdapterList, sectionViewList: list, viewProj: ViewWorldProjection, undoRedoService: object | None = None, ) -> MinMax2DList`

Create a unified section or view

**Remarks:** Create a unified section or view

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Placement matrix
- `elements` (BaseElementAdapterList) — List with the elements
- `sectionViewList` (list) — List with the sections and views
- `viewProj` (ViewWorldProjection) — View world projection
- `undoRedoService` (object | None) — Undo redo service

**Returns:** `MinMax2DList` — list of boundaries of the created views and sections

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.CreateSectionsAndViews)

#### `DeleteElements(doc: DocumentAdapter, elements: BaseElementAdapterList)`

Delete the elements

**Remarks:** Delete the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — List with the UUIDs of the data base elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.DeleteElements)

#### `DrawElementPreview( doc: DocumentAdapter, insertionMat: Matrix3D, modelEleList: list, bDirectDraw: int, assoRefObj: object, asStaticPreview: bool = False, addToPreviewBoundingBox: bool = True, )`

Draw the preview of the elements

**Remarks:** Draw the preview of the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Matrix with the placement point and the rotation
- `modelEleList` (list) — List with the model elements
- `bDirectDraw` (int) — Direct draw of the preview elements to the screen
- `assoRefObj` (object) — Associative view reference object
- `asStaticPreview` (bool) — Draw as static preview: true/false
- `addToPreviewBoundingBox` (bool) — Add the elements to the bounding box of the preview

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.DrawElementPreview)

#### `ElementTransform(transMat: Matrix3D, modelEleList: list) | ElementTransform( transVec: Vector3D, xAngle: float, yAngle: float, zAngle: float, modelEleList: list, )`

Transform the model elements

**Remarks:** Transform the model elements

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix
- `modelEleList` (list) — List with the model elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ElementTransform)

#### `ExecutePreviewDraw(doc: DocumentAdapter)`

Trigger the draw of the preview in the viewport

**Remarks:** Trigger the draw of the preview in the viewport

**Parameters:**
- `doc` (DocumentAdapter) — document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ExecutePreviewDraw)

#### `ExplodeIFCSmartSymbols( doc: DocumentAdapter, elements: BaseElementAdapterList ) -> BaseElementAdapterList`

Explode smart symbols with an existing IFC ID oder IFC object type

**Remarks:** Explode smart symbols with an existing IFC ID oder IFC object type

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — Elements to explode

**Returns:** `BaseElementAdapterList` — List with the exploded elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ExplodeIFCSmartSymbols)

#### `ExplodeSmartSymbols(elements: BaseElementAdapterList) -> BaseElementAdapterList`

Explode the smart symbols

**Remarks:** Explode the smart symbols

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to explode

**Returns:** `BaseElementAdapterList` — List with the exploded elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ExplodeSmartSymbols)

#### `GetColorById(id: int) -> ARGB`

Get the ARGB color by the color ID

**Remarks:** Get the ARGB color by the color ID

**Parameters:**
- `id` (int) — color ID

**Returns:** `ARGB` — ARGB color

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.GetColorById)

#### `GetElement(element: BaseElementAdapter) -> object`

Get the PythonParts element

**Remarks:** Get the PythonParts element

**Parameters:**
- `element` (BaseElementAdapter) — Element as BaseElementAdapter

**Returns:** `object` — PythonParts element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.GetElement)

#### `GetElements(elementsList: BaseElementAdapterList) -> list`

Get Python objects out of element adapters

**Remarks:** Get Python objects out of element adapters

**Parameters:**
- `elementsList` (BaseElementAdapterList) — List of element adapters

**Returns:** `list` — List of python objects

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.GetElements)

#### `GetIdByColor(color: ARGB) -> int`

Get the color ID by the ARGB color

**Remarks:** Get the color ID by the ARGB color

**Parameters:**
- `color` (ARGB) — ARGB color

**Returns:** `int` — color ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.GetIdByColor)

#### `GetMinMaxBox( elements: BaseElementAdapterList, axisAngle: float = 0.0, only3DElements: bool = 0.0, ) -> MinMax3D`

Get the min/max box of the elements

**Remarks:** Get the min/max box of the elements

**Parameters:**
- `elements` (BaseElementAdapterList) — List with the elements
- `axisAngle` (float) — Angle of the x axis for the min/max calculation
- `only3DElements` (bool) — Use only 3D elements

**Returns:** `MinMax3D` — Min/max box of the elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.GetMinMaxBox)

#### `GetViewMatrices(doc: DocumentAdapter)`

Get the associative view matrices

**Remarks:** Get the associative view matrices

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `object` — Associative view matrices

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.GetViewMatrices)

#### `ModifyElements(doc: DocumentAdapter, modelEleList: list)`

Modify the elements

**Remarks:** Modify the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `modelEleList` (list) — Elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ModifyElements)

#### `RotateElements( doc: DocumentAdapter, elements: BaseElementAdapterList, refPnt: Point2D, rotAngle: float, viewProj: ViewWorldProjection, )`

Rotate the elements

**Remarks:** Rotate the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — List with the elements
- `refPnt` (Point2D) — Reference point of the rotation
- `rotAngle` (float) — Rotation angle
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.RotateElements)

#### `ScaleElements( doc: DocumentAdapter, elements: BaseElementAdapterList, refPnt: Point3D, scaleX: float, scaleY: float, scaleZ: float, theta: Angle, viewProj: ViewWorldProjection, )`

Scale the elements

**Remarks:** Scale the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — List with the UUIDs of the data base elements
- `refPnt` (Point3D) — Reference point of the scaling
- `scaleX` (float) — Scale factor in x direction
- `scaleY` (float) — Scale factor in y direction
- `scaleX` (float) — Scale factor in x direction
- `theta` (Angle) — Rotation angle in Z-axis
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/#NemAll_Python_BaseElements.ScaleElements)

## IFC_Version (enum)

IFC version

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/IFC_Version/)

### Values
- `Ifc_2x3` = `4`
- `Ifc_4` = `7`
- `Ifc_XML_2x3` = `5`
- `Ifc_XML_4` = `8`

## LayerService (class)

Utility for processing the layer definitions.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/)

### Methods
#### `GetIDByShortName(shortName: str, doc: DocumentAdapter) -> int`

Get the ID by the short name

**Remarks:** Get the ID by the short name

**Parameters:**
- `shortName` (str) — Short name of the layer
- `doc` (DocumentAdapter) — Document

**Returns:** `int` — ID by name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.GetIDByShortName)

#### `GetNameByID(layerID: int, documentID: int) -> str`

Get the name by the ID

**Remarks:** Get the name by the ID

**Parameters:**
- `layerID` (int) — Layer ID
- `documentID` (int) — Document ID

**Returns:** `str` — Name by ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.GetNameByID)

#### `GetShortNameByID(layerID: int, documentID: int) -> str`

Get the short name by the ID

**Remarks:** Get the short name by the ID

**Parameters:**
- `layerID` (int) — Layer ID
- `documentID` (int) — Document ID

**Returns:** `str` — Name by ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.GetShortNameByID)

#### `LoadFromFavoriteFile(doc: DocumentAdapter, fileName: str) -> bool`

Load the layer data from a favorite file

**Remarks:** Load the layer data from a favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — File name

**Returns:** `bool` — Data are loaded: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.LoadFromFavoriteFile)

#### `SaveToFavoriteFile(doc: DocumentAdapter, fileName: str) -> bool`

Save the layer data to a favorite file

**Remarks:** Save the layer data to a favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — File name

**Returns:** `bool` — Data are saved: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.SaveToFavoriteFile)

## LayoutBorderDefinition (class)

Representation of the print layout border.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutBorderDefinition/)

### Constructors
- `LayoutBorderDefinition()` — Initialize

### Properties
- `DistanceBinding` (None, get) — Set/get the distance binding :type: None
- `DistanceDefault` (None, get) — Set/get the default distance :type: None
- `ExtraLineColor` (None, get) — Set/get the extra line color :type: None
- `ExtraLinePen` (None, get) — Set/get the extra line peb :type: None
- `ExtraLineStroke` (None, get) — Set/get the extra line stroke :type: None
- `FoldingBindingWidth` (None, get) — Set/get the :type: None
- `FoldingPageHeight` (None, get) — Set/get the folding - height of the pages :type: None
- `FoldingPageHeightTolNeg` (None, get) — Set/get the folding - max. negative tolerance of page height :type: None
- `FoldingPageHeightTolPlus` (None, get) — Set/get the folding - max. positive tolerance of page height :type: None
- `FoldingPageWidth` (None, get) — Set/get the folding - width of the pages :type: None
- `FoldingPageWidthTolNeg` (None, get) — Set/get the folding - max. negative tolerance of page width :type: None
- `FoldingPageWidthTolPlus` (None, get) — Set/get the folding - max. positive tolerance of page width :type: None
- `FoldingType` (None, get) — Set/get the folding type :type: None
- `Index` (None, get) — Set/get the index :type: None
- `InnerLineColor` (None, get) — Set/get the inner line color :type: None
- `InnerLinePen` (None, get) — Set/get the inner line pen :type: None
- `InnerLineStroke` (None, get) — Set/get the inner line stroke :type: None
- `Name` (None, get) — Get/set the name :type: None
- `OuterLineColor` (None, get) — Set/get the outer line color :type: None
- `OuterLinePen` (None, get) — Set/get the outer line pen :type: None
- `OuterLineStroke` (None, get) — Set/get the outer line stroke :type: None
- `UseExtraLine` (None, get) — Set/get the extra line state :type: None
- `UseInnerLine` (None, get) — Set/get the inner line state :type: None
- `UserDefined` (None, get) — Set/get the user defined state :type: None

## LayoutFileService (class)

Service for processing the print layout files

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/)

### Constructors
- `LayoutFileService()` — Initialize

### Methods
#### `AssignPrintProfile(doc: DocumentAdapter, printProfile: str)`

Set the print profile of the active document

**Remarks:** Set the print profile of the active document

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `printProfile` (str) — Full name of the print profile

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.AssignPrintProfile)

#### `CreateMasterLayoutElement( doc: DocumentAdapter, layoutMasterData: LayoutMasterData )`

Create the master layout element

**Remarks:** Create the master layout element

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `layoutMasterData` (LayoutMasterData) — Layout master data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.CreateMasterLayoutElement)

#### `DeleteDocument(doc: DocumentAdapter)`

Delete the content of the active document

**Remarks:** Delete the content of the active document

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.DeleteDocument)

#### `ExportDWG( doc: DocumentAdapter, layoutFileIndex: int, fileName: str, configFileName: str, )`

Export the data to a DWG file

**Remarks:** Export the data to a DWG file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `layoutFileIndex` (int) — Index of the layout file
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.ExportDWG)

#### `ExportPDF( doc: DocumentAdapter, layoutFileIndex: int, fileName: str, configFileName: str, )`

Export the data to a PDF file

**Remarks:** Export the data to a PDF file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `layoutFileIndex` (int) — Index of the layout file
- `fileName` (str) — Name of the PDF file
- `configFileName` (str) — Name of the PDF configuration file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.ExportPDF)

#### `ImportDWG( doc: DocumentAdapter, fileName: str, configFileName: str, placePnt: Point2D ) -> BaseElementAdapterList`

Import the data from an DWG file

**Remarks:** Import the data from an DWG file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file
- `placePnt` (Point2D) — Placement point of the data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.ImportDWG)

#### `InsertDrawingFile( arg2: DocumentAdapter, doc: list, fileIndexList: Point2D, placePnt: float, rotationAngle: float, scale: Point2D, clipBoxLeftBottom: Point2D, clipBoxRightTop: list[int] | VecIntList, layerList: float, textFactor: bool, bUseRefPnt: Point2D, refPnt: MinMax3D, )`

Insert drawing files in the layout file

**Remarks:** Insert drawing files in the layout file

**Parameters:**
- `doc` (list) — Document
- `fileIndexList` (Point2D) — List with the drawing file indices placePnt Placement point
- `rotationAngle` (float) — Rotation angle
- `scale` (Point2D) — Scale
- `clipBoxLeftBottom` (Point2D) — Left bottom point of the clipping box
- `clipBoxRightTop` (list[int] | VecIntList) — Top right point of the clipping box
- `layerList` (float) — List with the insertion layers
- `textFactor` (bool) — Text factor
- `bUseRefPnt` (Point2D) — Use the reference point for the placement
- `refPnt` (MinMax3D) — Reference point of the drawing file
- `drawingMinMaxClipping` (object) — Min/max coordinates of the clipping area from the drawing file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.InsertDrawingFile)

#### `LoadFile(doc: LayoutFileService, fileIndex: DocumentAdapter, loadState: int)`

Load a layout file

**Remarks:** Load a layout file

**Parameters:**
- `doc` (LayoutFileService) — Document
- `fileIndex` (DocumentAdapter) — Index of the layout file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.LoadFile)

## LayoutMargin (class)

Representation of the print layout margins

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMargin/)

### Constructors
- `LayoutMargin()` — Initialize

### Properties
- `Bottom` (None, get) — Bottom margin
- `Left` (None, get) — Left margin
- `Right` (None, get) — Right margin
- `Top` (None, get) — Top margin

## LayoutMasterData (class)

Representation of a print layout

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterData/)

### Constructors
- `LayoutMasterData()` — Initialize

### Properties
- `BackgroundType` (None, get) — Set/get the background type (0-none, 1- color, 2-filling/gradient filling, 3 texture) :type: None
- `Border` (None, get) — Set/get the border :type: None
- `Filling` (None, get) — Set/get the filling :type: None
- `LayoutHeaderOffsetX` (None, get) — Set/get the horizontal distance between legend frame and inner line of border :type: None
- `LayoutHeaderOffsetY` (None, get) — Set/get the vertical distance between legend frame and inner line of border :type: None
- `LayoutHeaderType` (None, get) — Set/get the header type (0-none, 1-stamp, 2-legend) :type: None
- `Legend` (None, get) — Set/get the legend :type: None
- `Margin` (None, get) — Set/get the margin :type: None
- `SheetSize` (None, get) — Set/get the size of the sheet :type: None
- `Stamp` (None, get) — Set/get the stamp :type: None
- `Texture` (None, get) — Set/get the texture :type: None
- `UseBorder` (None, get) — Set/get the use border state :type: None
- `UseMargins` (None, get) — Set/get the use margins state :type: None
- `UseProperties` (None, get) — Set/get the use properties state :type: None

## LayoutMasterLegendData (class)

Representation of a legend inside a print layout.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterLegendData/)

### Constructors
- `LayoutMasterLegendData()` — Initialize

### Properties
- `FileID` (None, get) — Set/get the file ID :type: None
- `ItemID` (None, get) — Set/get the item ID :type: None
- `LegendName` (None, get) — Get/set the legend name :type: None
- `PathID` (None, get) — Set/get the path ID :type: None

## LayoutMasterStampData (class)

Representation of a stamp (aka label) inside a print layout.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterStampData/)

### Constructors
- `LayoutMasterStampData()` — Initialize

### Properties
- `FileID` (None, get) — Set/get the file ID :type: None
- `ItemID` (None, get) — Set/get the item ID :type: None
- `PathID` (None, get) — Set/get the path ID :type: None
- `StampName` (None, get) — Get/set the stamp name :type: None

## LayoutSize (class)

Data class containing information about the sheet size of a print layout.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutSize/)

### Constructors
- `LayoutSize()` — Initialize

### Properties
- `Height` (None, get) — Layout height
- `Index` (None, get) — Index
- `Name` (None, get) — Name of the layout size, e.g. 'DIN A4'
- `UserDefined` (None, get) — Whether the size of the print layout should be defined freely by width & height, or to use predefined size defined in the property name.
- `Width` (None, get) — Layout width

## ModifyPropertyID (enum)

Property ID for the modification

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ModifyPropertyID/)

### Values
- `Color` = `8`
- `ColorByLayer` = `1003`
- `FaceStyle` = `19`
- `Hatching` = `1`
- `Layer` = `14`
- `Pen` = `6`
- `PenByLayer` = `1001`
- `Stroke` = `7`
- `StrokeByLayer` = `1002`

## PathID (enum)

Path IDs for the stamp

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PathID/)

### Values
- `PathDefaultID` = `20`
- `PathOfficeID` = `30`
- `PathPrivateID` = `8`
- `PathProjectID` = `1`

## PlaneService (class)

Implementation of the plane service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PlaneService/)

### Constructors
- `PlaneService() | PlaneService(doc: DocumentAdapter) | PlaneService(element: PlaneService)` — Initialize

### Methods
#### `CreateBRepSurfacePlane( doc: DocumentAdapter, planeSurface: BRep3D, surfaceName: str, isVisible: bool, ) -> BaseElementAdapter`

Create a BRep3D surface plane

**Remarks:** Create a BRep3D surface plane

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `planeSurface` (BRep3D) — Plane surface
- `surfaceName` (str) — Name of the surface
- `isVisible` (bool) — Visible state

**Returns:** `BaseElementAdapter` — Plane element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PlaneService/#NemAll_Python_BaseElements.PlaneService.CreateBRepSurfacePlane)

#### `CreatePolyhedronSurfacePlane( doc: DocumentAdapter, planeSurface: Polyhedron3D, surfaceName: str, isVisible: bool, ) -> BaseElementAdapter`

Create a Polyhedron3D surface plane

**Remarks:** Create a Polyhedron3D surface plane

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `planeSurface` (Polyhedron3D) — Plane surface
- `surfaceName` (str) — Name of the surface
- `isVisible` (bool) — Visible state

**Returns:** `BaseElementAdapter` — Plane element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PlaneService/#NemAll_Python_BaseElements.PlaneService.CreatePolyhedronSurfacePlane)

#### `GetCurrentLevelModelGuid() -> GUID`

Get the current level model GUID of the active drawing file

**Remarks:** Get the current level model GUID of the active drawing file

**Returns:** `GUID` — Level modle GUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PlaneService/#NemAll_Python_BaseElements.PlaneService.GetCurrentLevelModelGuid)

## ProjectAttributeService (class)

Utility for processing the values of project-specific attributes

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/)

### Methods
#### `ChangeAttributeFromCurrentProject( attributeNumber: int, newValue: int, doc: DocumentAdapter ) | ChangeAttributeFromCurrentProject( attributeNumber: int, newValue: float, doc: DocumentAdapter ) | ChangeAttributeFromCurrentProject( attributeNumber: int, newValue: str, doc: DocumentAdapter )`

Change a project attribute from the current file

**Remarks:** Change a project attribute from the current file

**Parameters:**
- `attributeNumber` (int) — Attribute number
- `newValue` (int) — Attribute value
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.ChangeAttributeFromCurrentProject)

#### `ChangeAttributesFromCurrentProject( attributes: list[tuple[int, int | float | str]], doc: DocumentAdapter )`

Change attributes from the current project

**Remarks:** Change attributes from the current project

**Parameters:**
- `attributes` (list[tuple[int, int | float | str]]) — Attributes
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.ChangeAttributesFromCurrentProject)

#### `GetAttributesFromAllProjects() -> list[tuple[int, int | float | str]]`

Get the attributes from all projects

**Remarks:** Get the attributes from all projects

**Returns:** `list[tuple[int, int | float | str]]` — Attributes from all projects

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.GetAttributesFromAllProjects)

#### `GetAttributesFromCurrentProject() -> list[tuple[int, int | float | str]]`

Get the attributes from the current project

**Remarks:** Get the attributes from the current project

**Returns:** `list[tuple[int, int | float | str]]` — Attributes from the current project

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.GetAttributesFromCurrentProject)

## ProjectService (class)

Utility for processing the Allplan project

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/)

### Methods
#### `CloseAllplan()`

Close Allplan

**Remarks:** Close Allplan

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.CloseAllplan)

#### `GetCurrentProjectNameAndHost() -> tuple`

Get the project and host name

**Remarks:** Get the project and host name

**Returns:** `tuple` — tuple(project name, host name)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.GetCurrentProjectNameAndHost)

#### `GetCurrentUserAsBwsPath() -> str`

Get the current user as BWS path

**Remarks:** Get the current user as BWS path

**Returns:** `str` — User as BWS path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.GetCurrentUserAsBwsPath)

#### `GetProjectPath(projectName: str, hostName: str) -> tuple`

Get the project path

**Remarks:** Get the project path

**Parameters:**
- `hostName` (str) — Host name
- `projectName` (str) — Project name

**Returns:** `tuple` — tuple(Error, project path)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.GetProjectPath)

#### `OpenProject(hostName: DocumentAdapter, projectName: str, doc: str) -> str`

Open the project

**Remarks:** Open the project

**Parameters:**
- `doc` (str) — Document
- `hostName` (DocumentAdapter) — Host name
- `projectName` (str) — Project name

**Returns:** `str` — String with the result the attempt to open the project. Possible results are: Active project Project not exist Not possible to open the project Project opened

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.OpenProject)

## PythonPartService (class)

Utility for processing PythonParts existing in the drawing file

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/)

### Methods
#### `GetParameter(ele: BaseElementAdapter) -> tuple`

Get the parameter of the PythonPart

**Remarks:** Get the parameter of the PythonPart

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `tuple` — True, when reading was successful. False otherwise.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.GetParameter)

#### `GetPlacementMatrix(ele: BaseElementAdapter) -> tuple`

Get the placement matrix of the PythonPart

**Remarks:** Get the placement matrix of the PythonPart

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `tuple` — True, when reading was successful. False otherwise.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.GetPlacementMatrix)

#### `IsPythonPartElement(ele: BaseElementAdapter) -> bool`

Check for a PythonPart element

**Remarks:** Check for a PythonPart element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `bool` — PythonPart element state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.IsPythonPartElement)

#### `IsPythonPartGroupElement(ele: BaseElementAdapter) -> bool`

Check for a PythonPart group element

**Remarks:** Check for a PythonPart group element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `bool` — PythonPart groupd element state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.IsPythonPartGroupElement)

#### `SetParameter( ele: BaseElementAdapter, name: str, paramList: list ) -> BaseElementAdapter`

Set the parameter of the PythonPart

**Remarks:** Set the parameter of the PythonPart

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `paramList` (list) — Parameter list

**Returns:** `BaseElementAdapter` — Modified element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.SetParameter)

## ViewSectionPreview (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ViewSectionPreview.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ViewSectionPreview/)

### Constructors
- `ViewSectionPreview()` — Initialize

### Methods
#### `CollectPreviewElements( doc: DocumentAdapter, generalsectionProperties: SectionGeneralProperties )`

Rotate the elements

**Remarks:** Rotate the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `generalsectionProperties` (SectionGeneralProperties) — General section properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ViewSectionPreview/#NemAll_Python_BaseElements.ViewSectionPreview.CollectPreviewElements)

#### `DrawPreview( arg2: Point2D, placementPoint: DocumentAdapter, doc: SectionGeneralProperties, generalsectionProperties: bool, ) -> Point3D`

Rotate the elements

**Remarks:** Rotate the elements

**Parameters:**
- `placementPoint` (DocumentAdapter) — Placement point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ViewSectionPreview/#NemAll_Python_BaseElements.ViewSectionPreview.DrawPreview)

## ZoomService (class)

Implementation of the zoom service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/)

### Constructors
- `ZoomService() | ZoomService(element: ZoomService)` — Initialize

### Methods
#### `ZoomToElement( element: BaseElementAdapter, viewProj: ViewWorldProjection, inflateValue: float, bZoomAll: bool, )`

Zoom to the element

**Remarks:** Zoom to the element

**Parameters:**
- `element` (BaseElementAdapter) — Element
- `viewProj` (ViewWorldProjection) — View world projection
- `inflateValue` (float) — Inflate value for the min/max box
- `bZoomAll` (bool) — Zoom in all views state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/#NemAll_Python_BaseElements.ZoomService.ZoomToElement)

#### `ZoomToElementWithFactor( element: BaseElementAdapter, viewProj: ViewWorldProjection, factor: float, bZoomAll: bool, )`

Zoom to the element

**Remarks:** Zoom to the element

**Parameters:**
- `element` (BaseElementAdapter) — Element
- `viewProj` (ViewWorldProjection) — View world projection
- `factor` (float) — Factor for zooming, 0.5 means 50% zooming
- `bZoomAll` (bool) — Zoom in all views state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/#NemAll_Python_BaseElements.ZoomService.ZoomToElementWithFactor)

#### `ZoomToElements( elements: BaseElementAdapterList, viewProj: ViewWorldProjection, inflateValue: float, bZoomAll: bool, )`

Zoom to the elements

**Remarks:** Zoom to the elements

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements
- `viewProj` (ViewWorldProjection) — View world projection
- `inflateValue` (float) — Inflate value for the min/max box
- `bZoomAll` (bool) — Zoom in all views state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/#NemAll_Python_BaseElements.ZoomService.ZoomToElements)

#### `ZoomToElementsWithFactor( elements: BaseElementAdapterList, viewProj: ViewWorldProjection, factor: float, bZoomAll: bool, )`

Zoom to the elements

**Remarks:** Zoom to the elements

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements
- `viewProj` (ViewWorldProjection) — View world projection
- `factor` (float) — Factor for zooming, 0.5 means 50% zooming
- `bZoomAll` (bool) — Zoom in all views state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/#NemAll_Python_BaseElements.ZoomService.ZoomToElementsWithFactor)

#### `ZoomToMinMaxBox( minMaxBox: MinMax3D, viewProj: ViewWorldProjection, inflateValue: float, bZoomAll: bool, ) | ZoomToMinMaxBox( minMaxBox: MinMax3D, viewProj: ViewWorldProjection, inflateValueX: float, inflateValueY: float, inflateValueZ: float, bZoomAll: bool, )`

Zoom to the min/max box

**Remarks:** Zoom to the min/max box

**Parameters:**
- `minMaxBox` (MinMax3D) — Zoom area
- `viewProj` (ViewWorldProjection) — View world projection
- `inflateValue` (float) — Inflate value for the min/max box
- `bZoomAll` (bool) — Zoom in all views state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/#NemAll_Python_BaseElements.ZoomService.ZoomToMinMaxBox)

#### `ZoomToMinMaxBoxWithFactor( minMaxBox: MinMax3D, viewProj: ViewWorldProjection, factor: float, bZoomAll: bool, )`

Zoom to the min/max box

**Remarks:** Zoom to the min/max box

**Parameters:**
- `minMaxBox` (MinMax3D) — Zoom area
- `viewProj` (ViewWorldProjection) — View world projection
- `factor` (float) — Factor for zooming, 0.5 means 50% zooming
- `bZoomAll` (bool) — Zoom in all views state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ZoomService/#NemAll_Python_BaseElements.ZoomService.ZoomToMinMaxBoxWithFactor)

## eAttibuteReadState (enum)

Enumeration of possible attribute reading modes of the ElementAttributeService

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/eAttibuteReadState/)

### Values
- `DoNotRead` = `2` — Do not read any attributes
- `ReadAll` = `1` — Read all attributes, without computed ones
- `ReadAllAndComputable` = `3` — Read all the attributes, also the computed ones
- `ReadWithoutGeometry` = `0` — Read all non-geometrical attributes

## eDesignPathLocation (enum)

Location of the design path

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/eDesignPathLocation/)

### Values
- `AskForLocation` = `0`
- `CreateSubPath` = `2`
- `OverrideFiles` = `1`

