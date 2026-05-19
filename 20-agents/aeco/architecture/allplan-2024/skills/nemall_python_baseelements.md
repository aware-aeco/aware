---
name: allplan-nemall_python_baseelements
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_BaseElements namespace — 49 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AllplanElement, AssociationService, AttributeByteVec, Attribute, AttributeDataManager, AttributeDouble, AttributeDate, AttributeDoubleVec, and 41 more types.
---

# NemAll_Python_BaseElements

Auto-generated from vendor docs for allplan 2024.0. 49 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/)

### Methods
#### `GetAttributes()`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetAttributes)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetCommonProperties)

#### `GetGeometryObject()`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetGeometryObject)

#### `GetLabelElements()`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetLabelElements)

#### `GetSubElementID()`

Get the sub element ID

**Remarks:** Get the sub element ID

**Returns:** `type` — Sub Element ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AllplanElement/#NemAll_Python_BaseElements.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## AssociationService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.AssociationService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AssociationService/)

### Methods
#### `AssociateObjectsWithPythonPart(doc, objectUUIDs, pythonPartUUID)`

Associate the objects with the PythonPart

**Remarks:** Associate the objects with the PythonPart

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `objectUUIDs` (List[str]) — List with the object UUIDs
- `pythonPartUUID` (GUID) — UUID of the PythonPart

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AssociationService/#NemAll_Python_BaseElements.AssociationService.AssociateObjectsWithPythonPart)

## Attribute (class)

Base for all attribute definitions

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attribute/)

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (Attribute) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attribute/#NemAll_Python_BaseElements.Attribute.__eq__)

### Properties
- `Id` (int, get/set) — Get the attribute Id

## AttributeByteVec (class)

ByteVec attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/)

### Constructors
- `AttributeByteVec() | AttributeByteVec(id, value) | AttributeByteVec(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeByteVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/#NemAll_Python_BaseElements.AttributeByteVec.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeByteVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/#NemAll_Python_BaseElements.AttributeByteVec.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeByteVec/#NemAll_Python_BaseElements.AttributeByteVec.__repr__)

### Properties
- `Value` ([list[int] | VecIntList], get/set) — Get the attribute value

## AttributeDataManager (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.AttributeDataManager.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDataManager/)

### Methods
#### `GetAttributeName(attributeID)`

Get the attribute name from the ID

**Remarks:** Get the attribute name from the ID

**Parameters:**
- `attributeID` (int) — Attribute ID

**Returns:** `str` — Attribute name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDataManager/#NemAll_Python_BaseElements.AttributeDataManager.GetAttributeName)

## AttributeDate (class)

Date attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/)

### Constructors
- `AttributeDate() | AttributeDate(id, day, month, year) | AttributeDate(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeDate) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/#NemAll_Python_BaseElements.AttributeDate.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeDate) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/#NemAll_Python_BaseElements.AttributeDate.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDate/#NemAll_Python_BaseElements.AttributeDate.__repr__)

### Properties
- `Day` (int, get/set) — Get the day value
- `Month` (int, get/set) — Get the month value
- `Year` (int, get/set) — Get the year value

## AttributeDouble (class)

Double attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/)

### Constructors
- `AttributeDouble() | AttributeDouble(id, value) | AttributeDouble(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeDouble) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/#NemAll_Python_BaseElements.AttributeDouble.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeDouble) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/#NemAll_Python_BaseElements.AttributeDouble.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDouble/#NemAll_Python_BaseElements.AttributeDouble.__repr__)

### Properties
- `Value` (float, get/set) — Get the attribute value

## AttributeDoubleVec (class)

DoubleVec attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/)

### Constructors
- `AttributeDoubleVec() | AttributeDoubleVec(id, value) | AttributeDoubleVec(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeDoubleVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/#NemAll_Python_BaseElements.AttributeDoubleVec.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeDoubleVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/#NemAll_Python_BaseElements.AttributeDoubleVec.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeDoubleVec/#NemAll_Python_BaseElements.AttributeDoubleVec.__repr__)

### Properties
- `Value` (NemAll_Python_Utility.VecDoubleList, get/set) — Get the attribute value

## AttributeEnum (class)

Enum attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/)

### Constructors
- `AttributeEnum() | AttributeEnum(id, value) | AttributeEnum(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeEnum) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/#NemAll_Python_BaseElements.AttributeEnum.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeEnum) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/#NemAll_Python_BaseElements.AttributeEnum.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeEnum/#NemAll_Python_BaseElements.AttributeEnum.__repr__)

### Properties
- `Value` (int, get/set) — Get the attribute value

## AttributeInteger (class)

Integer attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/)

### Constructors
- `AttributeInteger() | AttributeInteger(id, value) | AttributeInteger(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeInteger) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/#NemAll_Python_BaseElements.AttributeInteger.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeInteger) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/#NemAll_Python_BaseElements.AttributeInteger.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeInteger/#NemAll_Python_BaseElements.AttributeInteger.__repr__)

### Properties
- `Value` (int, get/set) — Get the attribute value

## AttributeIntegerVec (class)

IntegerVec attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/)

### Constructors
- `AttributeIntegerVec() | AttributeIntegerVec(id, value) | AttributeIntegerVec(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeIntegerVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/#NemAll_Python_BaseElements.AttributeIntegerVec.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeIntegerVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/#NemAll_Python_BaseElements.AttributeIntegerVec.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeIntegerVec/#NemAll_Python_BaseElements.AttributeIntegerVec.__repr__)

### Properties
- `Value` ([list[int] | VecIntList], get/set) — Get the attribute value

## AttributeService (enum)

(No description provided in vendor docs for NemAll_Python_BaseElements.AttributeService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/)

### Methods
#### `AddUserAttribute(doc, attributeType, attributeName, attributeDefaultValue, attributeMinValue, attributeMaxValue, attributeDimension, attributeCtrlType, attributeListValues)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.AddUserAttribute)

#### `GetAttributeControlType(doc, attributeID)`

Get the control type of the attribute

**Remarks:** Get the control type of the attribute

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeControlType)

#### `GetAttributeID(doc, attributeName)`

Get the attribute ID

**Remarks:** Get the attribute ID

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeName` (str) — Attribute name

**Returns:** `int` — Attribute ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeID)

#### `GetAttributeName(doc, attributeID)`

Get the attribute name

**Remarks:** Get the attribute name

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `str` — Attribute name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeName)

#### `GetAttributeType(doc, attributeID)`

Get the attribute type

**Remarks:** Get the attribute type

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetAttributeType)

#### `GetDefaultValue(doc, attributeID)`

Get the default value of an attribute

**Remarks:** Get the default value of an attribute

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `Union[int, float, str]` — Default value of the attribute

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetDefaultValue)

#### `GetEnumIDFromValueString(attributeID, valueString)`

Get the enumeration ID from the value string

**Remarks:** Get the enumeration ID from the value string

**Parameters:**
- `attributeID` (int) — Attribute ID
- `valueString` (str) — Value string

**Returns:** `int` — Enumeration ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetEnumIDFromValueString)

#### `GetEnumValueStringFromID(attributeID, enumID)`

Get the enumeration value string from the enumeration ID

**Remarks:** Get the enumeration value string from the enumeration ID

**Parameters:**
- `attributeID` (int) — Attribute ID
- `enumID` (int) — Enumeration ID

**Returns:** `str` — Enumeration value string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetEnumValueStringFromID)

#### `GetEnumValues(doc, attributeID)`

Get the enum attribute values

**Remarks:** Get the enum attribute values

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `list[str]` — Default attribute value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetEnumValues)

#### `GetInputListValues(doc, attributeID)`

Get the input list values

**Remarks:** Get the input list values

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `attributeID` (int) — Attribute ID

**Returns:** `list[str]` — Attribute input list values

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.GetInputListValues)

#### `OpenAttributeSelectionDialog(doc, dialogType)`

Open the attribute selection dialog

**Remarks:** Open the attribute selection dialog

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `dialogType` (AttributeSelectionDialogType) — Dialog type

**Returns:** `int` — Attribute ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/#NemAll_Python_BaseElements.AttributeService.OpenAttributeSelectionDialog)

## AttributeSet (class)

Attribute set

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/)

### Constructors
- `AttributeSet() | AttributeSet(elements) | AttributeSet(element)` — Initialize

### Methods
#### `GetAttributes()`

Get the AttributeSet vector

**Remarks:** Get the AttributeSet vector

**Returns:** `List[Union[AttributeByteVec, AttributeDate, AttributeDouble, AttributeDoubleVec, AttributeEnum, AttributeInteger, AttributeIntegerVec, AttributeString, AttributeStringVec]]` — Vector of Attribute elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.GetAttributes)

#### `SetAttributes(elements)`

Set the AttributeSet vector

**Remarks:** Set the AttributeSet vector

**Parameters:**
- `elements` (List[Union[AttributeByteVec, AttributeDate, AttributeDouble, AttributeDoubleVec, AttributeEnum, AttributeInteger, AttributeIntegerVec, AttributeString, AttributeStringVec]]) — Attribute elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.SetAttributes)

#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeSet) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeSet) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeSet/#NemAll_Python_BaseElements.AttributeSet.__repr__)

### Properties
- `Attributes` (typing.List[typing.Union[AttributeByteVec, AttributeDate, AttributeDouble, AttributeDoubleVec, AttributeEnum, AttributeInteger, AttributeIntegerVec, AttributeString, AttributeStringVec]], get/set) — Return type: typing.List[typing.Union[AttributeByteVec, AttributeDate, AttributeDouble, AttributeDoubleVec, AttributeEnum, AttributeInteger, AttributeIntegerVec, AttributeString, AttributeStringVec]] Get the AttributeSet vector

## AttributeString (class)

String attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/)

### Constructors
- `AttributeString() | AttributeString(id, value) | AttributeString(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeString) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/#NemAll_Python_BaseElements.AttributeString.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeString) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/#NemAll_Python_BaseElements.AttributeString.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeString/#NemAll_Python_BaseElements.AttributeString.__repr__)

### Properties
- `Value` (str, get/set) — Get the attribute value

## AttributeStringVec (class)

StringVec attribute

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/)

### Constructors
- `AttributeStringVec() | AttributeStringVec(id, value) | AttributeStringVec(element)` — Initialize

### Methods
#### `__eq__(element)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (AttributeStringVec) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/#NemAll_Python_BaseElements.AttributeStringVec.__eq__)

#### `__ne__(element)`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `element` (AttributeStringVec) — Element to compare

**Returns:** `bool` — Elements are unequal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/#NemAll_Python_BaseElements.AttributeStringVec.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeStringVec/#NemAll_Python_BaseElements.AttributeStringVec.__repr__)

### Properties
- `Value` (NemAll_Python_Utility.VecStringList, get/set) — Get the attribute value

## Attributes (class)

Attributes class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/)

### Constructors
- `Attributes() | Attributes(elements)` — Initialize

### Methods
#### `GetAttributeSets()`

Get the AttributeSet vector

**Remarks:** Get the AttributeSet vector

**Returns:** `list` — Attribute value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.GetAttributeSets)

#### `SetAttributeSets(sets)`

Set the AttributeSet vector

**Remarks:** Set the AttributeSet vector

**Parameters:**
- `sets` (list) — AttributeSet vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.SetAttributeSets)

#### `__eq__(props)`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (Attributes) — Properties to compare

**Returns:** `bool` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/Attributes/#NemAll_Python_BaseElements.Attributes.__repr__)

### Properties
- `AttributeSets` (None, get) — Property for attribute set vector :type: None

## CadDataFileReader (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.CadDataFileReader.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/)

### Methods
#### `ReadIFC(doc, fileName)`

Import the data from an IFC file to be used as PythonPart content

**Remarks:** Import the data from an IFC file to be used as PythonPart content

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the IFC file

**Returns:** `list[tuple[str, list[ModelElement3D]]]` — List with a tuple (group name, model elements)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/#NemAll_Python_BaseElements.CadDataFileReader.ReadIFC)

#### `ReadOBJ(fileName, designPathLocation)`

Import an OBJ file

**Remarks:** Import an OBJ file

**Parameters:**
- `fileName` (str) — Full file name of the OBJ file
- `designPathLocation` (eDesignPathLocation) — Location of the design path

**Returns:** `list[ModelElement3D]` — List with the model elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/#NemAll_Python_BaseElements.CadDataFileReader.ReadOBJ)

#### `ReadSKP(fileName, designPathLocation)`

Import an SKP file

**Remarks:** Import an SKP file

**Parameters:**
- `fileName` (str) — Full file name of the SKP file
- `designPathLocation` (eDesignPathLocation) — Location of the design path

**Returns:** `list[ModelElement3D]` — List with the model elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CadDataFileReader/#NemAll_Python_BaseElements.CadDataFileReader.ReadSKP)

## CommonProperties (class)

Implementation of the common properties of the element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/)

### Constructors
- `CommonProperties() | CommonProperties(element)` — Initialize

### Methods
#### `GetColorPenStrokeByLayerFromLayerNumber(layernumber)`

Layer number

**Remarks:** Layer number

**Returns:** `[list[int] | VecIntList]` — [0] = pen,[1] = stroke, [2] color

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.GetColorPenStrokeByLayerFromLayerNumber)

#### `GetGlobalProperties()`

Get the global properties

**Remarks:** Get the global properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.GetGlobalProperties)

#### `__eq__(props)`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (CommonProperties) — Properties to compare

**Returns:** `object` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/#NemAll_Python_BaseElements.CommonProperties.__repr__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DocumentResourceService/)

### Methods
#### `CreateSurface(doc, surfaceDirectoryPath, localPathAndName, surfaceDef, createUniqueName=True, bitmapDefinitions={})`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DocumentResourceService/#NemAll_Python_BaseElements.DocumentResourceService.CreateSurface)

## DrawingFileLoadState (enum)

Load state of the drawing file

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileLoadState/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `DrawingFileLoadState` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileLoadState/#NemAll_Python_BaseElements.DrawingFileLoadState.__getitem__)

### Values
- `ActiveBackground` = `2`
- `ActiveForeground` = `3`
- `PassiveBackground` = `1`

## DrawingFileService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.DrawingFileService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/)

### Constructors
- `DrawingFileService()` — Initialize

### Methods
#### `CreateBendingSchedule(doc, refPnt)`

Create a bending schedule

**Remarks:** Create a bending schedule

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `refPnt` (Point2D) — Reference point of the bending schedule

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.CreateBendingSchedule)

#### `DeleteDocument(doc)`

Delete the context of the active document Args: doc: Document

**Remarks:** Delete the context of the active document Args: doc: Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.DeleteDocument)

#### `ExportBendingMachine(doc, fileName, project, plan, index, bSplitGroups)`

Export the reinforcement data for the bending machine

**Remarks:** Export the reinforcement data for the bending machine

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the output file
- `project` (str) — Name of the project
- `plan` (str) — Name of the plan
- `index` (str) — Index as text
- `bSplitGroups` (bool) — Split the reinforcement groups

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.ExportBendingMachine)

#### `GetActiveFileNumber()`

Get the drawing file number of the active document

**Remarks:** Get the drawing file number of the active document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.GetActiveFileNumber)

#### `GetFileState()`

Get the file state of all loaded drawing files Returns: list with a tuple(file index, drawing file load state)

**Remarks:** Get the file state of all loaded drawing files Returns: list with a tuple(file index, drawing file load state)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.GetFileState)

#### `LoadFile(doc, fileIndex, loadState)`

Load a drawing file

**Remarks:** Load a drawing file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileIndex` (int) — Index of the drawing file
- `loadState` (DrawingFileLoadState) — File load state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.LoadFile)

#### `SetScalingFactor(arg2, arg3)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.SetScalingFactor)

#### `UnloadAll(doc)`

Unload all drawing files

**Remarks:** Unload all drawing files

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.UnloadAll)

#### `UnloadFile(doc, fileIndex)`

Unload a drawing file

**Remarks:** Unload a drawing file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileIndex` (int) — Index of the drawing file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingFileService/#NemAll_Python_BaseElements.DrawingFileService.UnloadFile)

## DrawingService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.DrawingService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/)

### Methods
#### `LockGraphicsEngineUpdate(doc, lockUpdate)`

Lock the update of the graphics engine Args: doc: Document lockUpdate: True: Lock the update of the graphics engine False:: Unlock the update of the graphics engine and update the graphics engine

**Remarks:** Lock the update of the graphics engine Args: doc: Document lockUpdate: True: Lock the update of the graphics engine False:: Unlock the update of the graphics engine and update the graphics engine

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.LockGraphicsEngineUpdate)

#### `RedrawAll(doc)`

Redraw all Args: doc: Document

**Remarks:** Redraw all Args: doc: Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.RedrawAll)

#### `ResetAndDrawHiddenElement(doc, hiddenElement)`

Reset and draw the hidden elements Args: doc: Document hiddenElement: Hidden element

**Remarks:** Reset and draw the hidden elements Args: doc: Document hiddenElement: Hidden element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.ResetAndDrawHiddenElement)

#### `UpdateAllWindows()`

Update the drawing in all windows

**Remarks:** Update the drawing in all windows

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.UpdateAllWindows)

#### `UpdateGraphicsEngine(doc)`

Update the graphics engine Args: doc: Document

**Remarks:** Update the graphics engine Args: doc: Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingService/#NemAll_Python_BaseElements.DrawingService.UpdateGraphicsEngine)

## DrawingTypeService (enum)

Implementation of the drawing type service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/)

### Methods
#### `GetCurrentDrawingTypeDescription(doc)`

Get the description of the current drawing type

**Remarks:** Get the description of the current drawing type

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetCurrentDrawingTypeDescription)

#### `GetCurrentDrawingTypeId(doc)`

Get the current drawing type ID

**Remarks:** Get the current drawing type ID

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetCurrentDrawingTypeId)

#### `GetDrawingTypeDescription(doc, drawingTypeId)`

Get the description of the drawing type

**Remarks:** Get the description of the drawing type

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `drawingTypeId` (int) — Drawing type ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetDrawingTypeDescription)

#### `GetDrawingTypeDescriptions(doc)`

Get the description of the drawing types

**Remarks:** Get the description of the drawing types

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetDrawingTypeDescriptions)

#### `GetDrawingTypeIdFromDescription(doc, typeDescription)`

Get the ID of the drawing type from the description

**Remarks:** Get the ID of the drawing type from the description

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `typeDescription` (str) — Drawing type description

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.GetDrawingTypeIdFromDescription)

#### `SetDrawingTypeId(drawingTypeId)`

Set the drawing type ID

**Remarks:** Set the drawing type ID

**Parameters:**
- `drawingTypeId` (int) — ID of the drawing type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/DrawingTypeService/#NemAll_Python_BaseElements.DrawingTypeService.SetDrawingTypeId)

## ElementsAttributeService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ElementsAttributeService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/)

### Methods
#### `ChangeAttribute(attributeNumber, newValue, elements)`

Change an attribute

**Remarks:** Change an attribute

**Parameters:**
- `attributeNumber` (int) — Attribute number
- `newValue` (object) — New value
- `elements` (BaseElementAdapterList) — Elements

**Returns:** `BaseElementAdapterList` — Adapted elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/#NemAll_Python_BaseElements.ElementsAttributeService.ChangeAttribute)

#### `ChangeAttributes(attributeDataList, elements)`

Change an attribute

**Remarks:** Change an attribute

**Parameters:**
- `attributeDataList` (list) — List with the attribute data as tuple(number, value)
- `elements` (BaseElementAdapterList) — Elements as BaseElementAdapterList

**Returns:** `BaseElementAdapterList` — Modified elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/#NemAll_Python_BaseElements.ElementsAttributeService.ChangeAttributes)

#### `GetAttributes(ele, readState=ReadAll)`

Get the attributes from an element

**Remarks:** Get the attributes from an element

**Parameters:**
- `element` (object) — Element

**Returns:** `list` — Attributes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/#NemAll_Python_BaseElements.ElementsAttributeService.GetAttributes)

## ElementsByAttributeService (class)

Implementation of the elements by attribute service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/)

### Methods
#### `GetElements(attributeValue) | GetElements(attributeValue) | GetElements(attributeValue)`

Get the elements for the double attribute value

**Remarks:** Get the elements for the double attribute value

**Parameters:**
- `attributeValue` (float) — Attribute value

**Returns:** `BaseElementAdapterList` — Elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/#NemAll_Python_BaseElements.ElementsByAttributeService.GetElements)

#### `GetInstance()`

Get the instance

**Remarks:** Get the instance

**Returns:** `ElementsByAttributeService` — Instance of the singleton

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/#NemAll_Python_BaseElements.ElementsByAttributeService.GetInstance)

#### `Init(attributeID, doc)`

Initialize the service.

**Remarks:** Initialize the service.

**Parameters:**
- `attributeID` (int) — Attribute ID. Used for creating fast element access by an attribute value of this ID
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsByAttributeService/#NemAll_Python_BaseElements.ElementsByAttributeService.Init)

## ElementsLayerService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ElementsLayerService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsLayerService/)

### Methods
#### `ChangeLayer(elements, newValue)`

Change the Layer

**Remarks:** Change the Layer

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to change
- `newValue` (str) — New value

**Returns:** `BaseElementAdapterList` — List with the modified elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsLayerService/#NemAll_Python_BaseElements.ElementsLayerService.ChangeLayer)

## ElementsPropertyService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ElementsPropertyService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsPropertyService/)

### Methods
#### `ModifyFormatProperties(arg2)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsPropertyService/#NemAll_Python_BaseElements.ElementsPropertyService.ModifyFormatProperties)

#### `ModifySurface(surfacePathFile, elements)`

Change the surface

**Remarks:** Change the surface

**Parameters:**
- `surfacePathFile` (str) — New surface path and file
- `elements` (BaseElementAdapterList) — Elements to modify

**Returns:** `BaseElementAdapterList` — Modified elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsPropertyService/#NemAll_Python_BaseElements.ElementsPropertyService.ModifySurface)

## ElementsSelectService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ElementsSelectService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/)

### Methods
#### `SelectAllElements(ele)`

Select all elements from the document

**Remarks:** Select all elements from the document

**Parameters:**
- `doc` (object) — Document

**Returns:** `BaseElementAdapterList` — All elements from the document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/#NemAll_Python_BaseElements.ElementsSelectService.SelectAllElements)

#### `SelectElementsByIfcGuid(ele)`

Select elements by IFC guids

**Remarks:** Select elements by IFC guids

**Parameters:**
- `doc` (object) — Document
- `ifcGuids` (object) — List with the IFC GUIDs

**Returns:** `BaseElementAdapterList` — Elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/#NemAll_Python_BaseElements.ElementsSelectService.SelectElementsByIfcGuid)

## ExportImportService (class)

Implementation of the export/import service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/)

### Constructors
- `ExportImportService() | ExportImportService(element)` — Initialize

### Methods
#### `ExportDWG(doc, fileName, configFileName, version)`

Export the data to a DWG file by a configuration file

**Remarks:** Export the data to a DWG file by a configuration file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file
- `version` (int) — Version numer

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportDWG)

#### `ExportDWGByTheme(doc, fileName, themeFileName, version=2018)`

Export the data to a DWG file by a theme file

**Remarks:** Export the data to a DWG file by a theme file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `themeFileName` (str) — Name of the DWG theme file
- `version` (int) — DWG version

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportDWGByTheme)

#### `ExportIFC(doc, fileNumbers, ifcVersion, fileName, ifcThemeFileName='')`

Export the data to an IFC file

**Remarks:** Export the data to an IFC file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileNumbers` ([list[int] | VecIntList]) — Numbers of the drawing files to export
- `ifcVersion` (IFC_Version) — IFC version
- `fileName` (str) — Name of the IFC file
- `ifcThemeFileName` (str) — Name of the theme file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportIFC)

#### `ExportXPlan(doc, fileNumbers, xplanFilePath)`

Export the data to a XPlan file

**Remarks:** Export the data to a XPlan file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileNumbers` ([list[int] | VecIntList]) — Numbers of the drawing files to export
- `xplanFilePath` (str) — Path and name of the XPlan file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ExportXPlan)

#### `ImportDWG(doc, fileName, configFileName, placePnt)`

Import the data from a DWG file

**Remarks:** Import the data from a DWG file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file
- `placePnt` (Point3D) — Placement point of the data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ImportDWG)

#### `ImportIFC(doc, fileNumber, fileName)`

Import the data from an IFC file

**Remarks:** Import the data from an IFC file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileNumber` (int) — Number of the drawing file for the data import
- `fileName` (str) — Name of the IFC file

**Returns:** `BaseElementAdapterList` — List with the imported elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ImportIFC)

#### `ImportXPlan(doc, xplanFilePath, drawingFileNumber)`

Import the data from a XPlan file

**Remarks:** Import the data from a XPlan file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `xplanFilePath` (str) — Path and name of the XPlan file to import
- `drawingFileNumber` (int) — Starting number of destination drawing file(s)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ExportImportService/#NemAll_Python_BaseElements.ExportImportService.ImportXPlan)

## FaceSelectService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.FaceSelectService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/)

### Methods
#### `SelectPolyhedronFace(polyhedron, pnt, highlightFace, viewProj, doc, includeUVSSelection)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectPolyhedronFace)

#### `SelectPolyhedronFaceInUVS(polyhedron, pnt, highlightFace, viewProj, doc)`

Select a polyhedron face in an UVS

**Remarks:** Select a polyhedron face in an UVS

**Parameters:**
- `polyhedron` (BaseElementAdapter) — Polyhedron
- `pnt` (Point2D) — Selection view point
- `highlightFace` (bool) — Highlight the face state
- `viewProj` (ViewWorldProjection) — View world projection of the selected view
- `doc` (DocumentAdapter) — Document

**Returns:** `tuple[bool, Polygon3D, IntersectionRayPolyhedron, Matrix3D]` — select state, face polygon, intersection result, UVS matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectPolyhedronFaceInUVS)

#### `SelectWallFace(wallTier, pnt, highlightFace, viewProj, doc, includeUVSSelection)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectWallFace)

#### `SelectWallFaceInUVS(wallTier, pnt, highlightFace, viewProj, doc)`

Select a wall face in an UVS

**Remarks:** Select a wall face in an UVS

**Parameters:**
- `wallTier` (BaseElementAdapter) — Wall tier
- `pnt` (Point2D) — Selection view point
- `highlightFace` (bool) — Highlight the face state
- `viewProj` (ViewWorldProjection) — View world projection of the selected view
- `doc` (DocumentAdapter) — Document

**Returns:** `tuple[bool, Polygon3D, IntersectionRayPolyhedron, Matrix3D]` — select state, face polygon, intersection result, UVS matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/FaceSelectService/#NemAll_Python_BaseElements.FaceSelectService.SelectWallFaceInUVS)

## Functions (static-class)

Module-level functions of NemAll_Python_BaseElements

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/)

### Methods
#### `ClearElementPreview()`

Clear the element preview

**Remarks:** Clear the element preview

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ClearElementPreview)

#### `CloseElementPreview()`

Close the element preview

**Remarks:** Close the element preview

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CloseElementPreview)

#### `CopyElements(doc, elements, fromPoint, distanceVector, rotationVector, rotationAngle, numberOfCopies, viewProj)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CopyElements)

#### `CreateAssociativeViews(doc, insertionMat, elements, assoViewList, viewProj)`

Create associative views

**Remarks:** Create associative views

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Placment matrix
- `elements` (BaseElementAdapterList) — List with the elements
- `assoViewList` (list) — List with the associative views
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CreateAssociativeViews)

#### `CreateBarCoupler(arg2, arg3, arg4, arg5, arg6, arg7)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CreateBarCoupler)

#### `CreateElements(doc, insertionMat, modelEleList, modelUuidList, assoRefObj, appendReinfPosNr=True, createUndoStep=True)`

Create the elements in the data base

**Remarks:** Create the elements in the data base

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Matrix with the placement point and the rotation
- `modelEleList` (list) — List with the model elements modelUuidList List with the model UUIDS in modification mode
- `assoRefObj` (object) — Associative view reference object
- `appendReinfPosNr` (bool) — True: Append the reinforcement position numbers to the existing position numbers
- `False` (object) — : Try to use the original reinforcement position numbers
- `createUndoStep` (bool) — Create an undo step after the creation of the PythonPart

**Returns:** `BaseElementAdapterList` — List with the created elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CreateElements)

#### `CreateLayer(doc, groupName, subGroupName, longName, shortName, lineColorID, lineThicknessID, lineStyleID, bVisible, bModifiable)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CreateLayer)

#### `CreateLibraryElement(doc, insertionMat, path, elementName)`

Create a library element in the data base

**Remarks:** Create a library element in the data base

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Matrix with the placement point and the rotation
- `path` (str) — Path of the library element
- `elementName` (str) — Name of the library element

**Returns:** `BaseElementAdapterList` — List with the created elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CreateLibraryElement)

#### `CreateSectionsAndViews(doc, insertionMat, elements, sectionViewList, viewProj, undoRedoService=None)`

Create sections and views

**Remarks:** Create sections and views

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Placment matrix
- `elements` (BaseElementAdapterList) — List with the elements
- `sectionViewList` (list) — List with the sections and views
- `viewProj` (ViewWorldProjection) — View world projection undoRedoService Undo redo service

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.CreateSectionsAndViews)

#### `DeleteElements(doc, elements)`

Delete the elements

**Remarks:** Delete the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — List with the UUIDs of the data base elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.DeleteElements)

#### `DrawElementPreview(doc, insertionMat, modelEleList, bDirectDraw, assoRefObj, asStaticPreview=False, addToPreviewBoundingBox=True)`

Draw the preview of the elements

**Remarks:** Draw the preview of the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Matrix with the placement point and the rotation
- `modelEleList` (list) — List with the model elements bDirectDraw Direct draw of the preview elements to the screen
- `assoRefObj` (object) — Associative view reference object
- `asStaticPreview` (bool) — Draw as static preview: true/false
- `addToPreviewBoundingBox` (bool) — Add the elements to the bounding box of the preview

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.DrawElementPreview)

#### `ElementTransform(transMat, modelEleList) | ElementTransform(transVec, xAngle, yAngle, zAngle, modelEleList)`

Transform the model elements

**Remarks:** Transform the model elements

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix
- `modelEleList` (list) — List with the model elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ElementTransform)

#### `ExecutePreviewDraw(doc)`

Execute the preview drawing

**Remarks:** Execute the preview drawing

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ExecutePreviewDraw)

#### `ExplodeIFCSmartSymbols(doc, elements)`

Explode smart symbols with an existing IFC ID oder IFC object type

**Remarks:** Explode smart symbols with an existing IFC ID oder IFC object type

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — Elements to explode

**Returns:** `BaseElementAdapterList` — List with the exploded elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ExplodeIFCSmartSymbols)

#### `ExplodeSmartSymbols(elements)`

Explode the smart symbols

**Remarks:** Explode the smart symbols

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to explode

**Returns:** `BaseElementAdapterList` — List with the exploded elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ExplodeSmartSymbols)

#### `GetColorById(id)`

Get the ARGB color by the color ID

**Remarks:** Get the ARGB color by the color ID

**Parameters:**
- `id` (int) — color ID

**Returns:** `ARGB` — ARGB color

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.GetColorById)

#### `GetElements(elementsList)`

Get the PythonParts elements

**Remarks:** Get the PythonParts elements

**Parameters:**
- `elementsList` (BaseElementAdapterList) — Elements as BaseElementAdapterList

**Returns:** `list` — PythonParts elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.GetElements)

#### `GetIdByColor(color)`

Get the color ID by the ARGB color

**Remarks:** Get the color ID by the ARGB color

**Parameters:**
- `color` (ARGB) — ARGB color

**Returns:** `int` — color ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.GetIdByColor)

#### `GetMinMaxBox(elements, axisAngle=0.0, only3DElements=0.0)`

Get the min/max box of the elements Returns: Min/max box of the elements

**Remarks:** Get the min/max box of the elements Returns: Min/max box of the elements

**Parameters:**
- `doc` (object) — Document
- `elements` (BaseElementAdapterList) — List with the elements
- `axisAngle` (float) — Angle of the x axis for the min/max calculation
- `only3DElements` (bool) — Use only 3D elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.GetMinMaxBox)

#### `GetViewMatrices(doc)`

Get the associative view matrices

**Remarks:** Get the associative view matrices

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `object` — Associative view matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.GetViewMatrices)

#### `ModifyElements(modelEleList)`

Modify the elements

**Remarks:** Modify the elements

**Parameters:**
- `modelEleList` (list) — Elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ModifyElements)

#### `RotateElements(doc, elements, refPnt, rotAngle, viewProj)`

Rotate the elements

**Remarks:** Rotate the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `elements` (BaseElementAdapterList) — List with the elements
- `refPnt` (Point2D) — Reference point of the rotation
- `rotAngle` (float) — Rotation angle
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.RotateElements)

#### `ScaleElements(doc, elements, refPnt, scaleX, scaleY, scaleZ, theta, viewProj)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/_functions/#NemAll_Python_BaseElements.ScaleElements)

## IFC_Version (enum)

IFC version

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/IFC_Version/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `IFC_Version` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/IFC_Version/#NemAll_Python_BaseElements.IFC_Version.__getitem__)

### Values
- `Ifc_2x3` = `4`
- `Ifc_4` = `7`
- `Ifc_XML_2x3` = `5`
- `Ifc_XML_4` = `8`

## LayerService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.LayerService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/)

### Methods
#### `GetIDByShortName(shortName, doc)`

Get the ID by the short name

**Remarks:** Get the ID by the short name

**Parameters:**
- `shortName` (str) — Short name of the layer
- `doc` (DocumentAdapter) — Document

**Returns:** `int` — ID by name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.GetIDByShortName)

#### `GetNameByID(layerID, documentID)`

Get the name by the ID

**Remarks:** Get the name by the ID

**Parameters:**
- `layerID` (int) — Layer ID
- `documentID` (int) — Document ID

**Returns:** `str` — Name by ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.GetNameByID)

#### `GetShortNameByID(layerID, documentID)`

Get the short name by the ID

**Remarks:** Get the short name by the ID

**Parameters:**
- `layerID` (int) — Layer ID
- `documentID` (int) — Document ID

**Returns:** `str` — Name by ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.GetShortNameByID)

#### `LoadFromFavoriteFile(doc, fileName)`

Load the layer data from a favorite file

**Remarks:** Load the layer data from a favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — File name

**Returns:** `bool` — Data are loaded: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.LoadFromFavoriteFile)

#### `SaveToFavoriteFile(doc, fileName)`

Save the layer data to a favorite file

**Remarks:** Save the layer data to a favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — File name

**Returns:** `bool` — Data are saved: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/#NemAll_Python_BaseElements.LayerService.SaveToFavoriteFile)

## LayoutBorderDefinition (class)

Border definition

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutBorderDefinition/)

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

(No description provided in vendor docs for NemAll_Python_BaseElements.LayoutFileService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/)

### Constructors
- `LayoutFileService()` — Initialize

### Methods
#### `AssignPrintProfile(doc, printProfile)`

Set the print profile of the active document

**Remarks:** Set the print profile of the active document

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `printProfile` (str) — Full name of the print profile

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.AssignPrintProfile)

#### `CreateMasterLayoutElement(doc, layoutMasterData)`

Create the master layout element

**Remarks:** Create the master layout element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.CreateMasterLayoutElement)

#### `DeleteDocument(doc)`

Delete the content of the active document

**Remarks:** Delete the content of the active document

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.DeleteDocument)

#### `ExportDWG(doc, layoutFileIndex, fileName, configFileName)`

Export the data to a DWG file

**Remarks:** Export the data to a DWG file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `layoutFileIndex` (int) — Index of the layout file
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.ExportDWG)

#### `ExportPDF(doc, layoutFileIndex, fileName, configFileName)`

Export the data to a PDF file

**Remarks:** Export the data to a PDF file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `layoutFileIndex` (int) — Index of the layout file
- `fileName` (str) — Name of the PDF file
- `configFileName` (str) — Name of the PDF configuration file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.ExportPDF)

#### `ImportDWG(doc, fileName, configFileName, placePnt)`

Import the data from an DWG file

**Remarks:** Import the data from an DWG file

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fileName` (str) — Name of the DWG file
- `configFileName` (str) — Name of the DWG configuration file
- `placePnt` (Point2D) — Placement point of the data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.ImportDWG)

#### `InsertDrawingFile(arg2, doc, fileIndexList, placePnt, rotationAngle, scale, clipBoxLeftBottom, clipBoxRightTop, layerList, textFactor, bUseRefPnt, refPnt)`

Insert drawing files in the layout file

**Remarks:** Insert drawing files in the layout file

**Parameters:**
- `doc` (list) — Document
- `fileIndexList` (Point2D) — List with the drawing file indices placePnt Placement point
- `rotationAngle` (float) — Rotation angle
- `scale` (Point2D) — Scale
- `clipBoxLeftBottom` (Point2D) — Left bottom point of the clipping box
- `clipBoxRightTop` ([list[int] | VecIntList]) — Top right point of the clipping box
- `layerList` (float) — List with the insertion layers
- `textFactor` (bool) — Text factor
- `bUseRefPnt` (Point2D) — Use the reference point for the placement
- `refPnt` (MinMax3D) — Reference point of the drawing file
- `drawingMinMaxClipping` (object) — Min/max coordinates of the clipping area from the drawing file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.InsertDrawingFile)

#### `LoadFile(doc, fileIndex, loadState)`

Load a layout file

**Remarks:** Load a layout file

**Parameters:**
- `doc` (LayoutFileService) — Document
- `fileIndex` (DocumentAdapter) — Index of the layout file

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/#NemAll_Python_BaseElements.LayoutFileService.LoadFile)

## LayoutMargin (class)

Margin data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMargin/)

### Constructors
- `LayoutMargin()` — Initialize

### Properties
- `Bottom` (None, get) — Set/get bottom margin :type: None
- `Left` (None, get) — Set/get the left margin :type: None
- `Right` (None, get) — Set/get the right margin :type: None
- `Top` (None, get) — Set/get the top margin :type: None

## LayoutMasterData (class)

Layout master data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterData/)

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

Legend data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterLegendData/)

### Constructors
- `LayoutMasterLegendData()` — Initialize

### Properties
- `FileID` (None, get) — Set/get the file ID :type: None
- `ItemID` (None, get) — Set/get the item ID :type: None
- `LegendName` (None, get) — Get/set the legend name :type: None
- `PathID` (None, get) — Set/get the path ID :type: None

## LayoutMasterStampData (class)

Stamp data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterStampData/)

### Constructors
- `LayoutMasterStampData()` — Initialize

### Properties
- `FileID` (None, get) — Set/get the file ID :type: None
- `ItemID` (None, get) — Set/get the item ID :type: None
- `PathID` (None, get) — Set/get the path ID :type: None
- `StampName` (None, get) — Get/set the stamp name :type: None

## LayoutSize (class)

Layout size data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutSize/)

### Constructors
- `LayoutSize()` — Initialize

### Properties
- `Height` (None, get) — Set/get the height :type: None
- `Index` (None, get) — Set/get the index :type: None
- `Name` (None, get) — Get/set the name :type: None
- `UserDefined` (None, get) — Set/get the user defined state :type: None
- `Width` (None, get) — Set/get the width :type: None

## ModifyPropertyID (enum)

Property ID for the modification

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ModifyPropertyID/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ModifyPropertyID` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ModifyPropertyID/#NemAll_Python_BaseElements.ModifyPropertyID.__getitem__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PathID/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PathID` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PathID/#NemAll_Python_BaseElements.PathID.__getitem__)

### Values
- `PathDefaultID` = `20`
- `PathOfficeID` = `30`
- `PathPrivateID` = `8`
- `PathProjectID` = `1`

## PlaneService (class)

Implementation of the plane service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PlaneService/)

### Constructors
- `PlaneService() | PlaneService(doc)` — Initialize

### Methods
#### `GetCurrentLevelModelGuid()`

Get the current level model GUID of the active drawing file

**Remarks:** Get the current level model GUID of the active drawing file

**Returns:** `GUID` — Level modle GUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PlaneService/#NemAll_Python_BaseElements.PlaneService.GetCurrentLevelModelGuid)

## ProjectAttributeService (class)

Implementation of the project attribute service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/)

### Methods
#### `ChangeAttributeFromCurrentProject(attributeNumber, newValue, doc) | ChangeAttributeFromCurrentProject(attributeNumber, newValue, doc) | ChangeAttributeFromCurrentProject(attributeNumber, newValue, doc)`

Change a project attribute from the current file

**Remarks:** Change a project attribute from the current file

**Parameters:**
- `attributeNumber` (int) — Attribute number
- `newValue` (int) — Attribute value
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.ChangeAttributeFromCurrentProject)

#### `ChangeAttributesFromCurrentProject(attributes, doc)`

Change attributes from the current project

**Remarks:** Change attributes from the current project

**Parameters:**
- `attributes` (list[tuple[int, Union[int, float, str]]]) — Attributes
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.ChangeAttributesFromCurrentProject)

#### `GetAttributesFromAllProjects()`

Get the attributes from all projects

**Remarks:** Get the attributes from all projects

**Returns:** `list[tuple[int, Union[int, float, str]]]` — Attributes from all projects

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.GetAttributesFromAllProjects)

#### `GetAttributesFromCurrentProject()`

Get the attributes from the current project

**Remarks:** Get the attributes from the current project

**Returns:** `list[tuple[int, Union[int, float, str]]]` — Attributes from the current project

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectAttributeService/#NemAll_Python_BaseElements.ProjectAttributeService.GetAttributesFromCurrentProject)

## ProjectService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ProjectService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/)

### Methods
#### `CloseAllplan()`

Close Allplan

**Remarks:** Close Allplan

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.CloseAllplan)

#### `GetCurrentProjectNameAndHost()`

Get the project and host name

**Remarks:** Get the project and host name

**Returns:** `tuple` — tuple(project name, host name)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.GetCurrentProjectNameAndHost)

#### `GetCurrentUserAsBwsPath()`

Get the current user as BWS path

**Remarks:** Get the current user as BWS path

**Returns:** `str` — User as BWS path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.GetCurrentUserAsBwsPath)

#### `GetProjectPath(projectName, hostName)`

Get the project path

**Remarks:** Get the project path

**Parameters:**
- `hostName` (str) — Host name
- `projectName` (str) — Project name

**Returns:** `tuple` — tuple(Error, project path)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.GetProjectPath)

#### `OpenProject(hostName, projectName, doc)`

Open the project

**Remarks:** Open the project

**Parameters:**
- `doc` (str) — Document
- `hostName` (DocumentAdapter) — Host name
- `projectName` (str) — Project name

**Returns:** `str` — Active project

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ProjectService/#NemAll_Python_BaseElements.ProjectService.OpenProject)

## PythonPartService (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.PythonPartService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/)

### Methods
#### `GetParameter(ele)`

Get the parameter of the PythonPart

**Remarks:** Get the parameter of the PythonPart

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `Successful` — tuple

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.GetParameter)

#### `GetPlacementMatrix(ele)`

Get the placement matrix

**Remarks:** Get the placement matrix

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `Successful` — tuple

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.GetPlacementMatrix)

#### `SetParameter(ele, name, paramList)`

Set the parameter of the PythonPart

**Remarks:** Set the parameter of the PythonPart

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `paramList` (list) — Parameter list

**Returns:** `BaseElementAdapter` — Modified element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/PythonPartService/#NemAll_Python_BaseElements.PythonPartService.SetParameter)

## ViewSectionPreview (class)

(No description provided in vendor docs for NemAll_Python_BaseElements.ViewSectionPreview.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ViewSectionPreview/)

### Constructors
- `ViewSectionPreview()` — Initialize

### Methods
#### `CollectPreviewElements(doc, generalsectionProperties)`

Rotate the elements

**Remarks:** Rotate the elements

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `generalsectionProperties` (SectionGeneralProperties) — General section properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ViewSectionPreview/#NemAll_Python_BaseElements.ViewSectionPreview.CollectPreviewElements)

#### `DrawPreview(arg2, placementPoint, doc, generalsectionProperties)`

Rotate the elements

**Remarks:** Rotate the elements

**Parameters:**
- `placementPoint` (DocumentAdapter) — Placement point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ViewSectionPreview/#NemAll_Python_BaseElements.ViewSectionPreview.DrawPreview)

## eAttibuteReadState (enum)

Attribute read state

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/eAttibuteReadState/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eAttibuteReadState` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/eAttibuteReadState/#NemAll_Python_BaseElements.eAttibuteReadState.__getitem__)

### Values
- `DoNotRead` = `2`
- `ReadAll` = `1`
- `ReadAllAndComputable` = `3`
- `ReadWithoutGeometry` = `0`

## eDesignPathLocation (enum)

Location of the design path

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/eDesignPathLocation/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eDesignPathLocation` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BaseElements/eDesignPathLocation/#NemAll_Python_BaseElements.eDesignPathLocation.__getitem__)

### Values
- `AskForLocation` = `0`
- `CreateSubPath` = `2`
- `OverrideFiles` = `1`

