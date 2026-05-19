---
name: allplan-nemall_python_ifw_elementadapter
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_IFW_ElementAdapter namespace — 16 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AllplanElement, AssocViewElementAdapter, ArchElementType, BaseElementAdapter, AxisElementAdapter, BaseElementAdapterChildElementsService, BaseElementAdapterList, BaseElementAdapterParentElementService, and 8 more types.
---

# NemAll_Python_IFW_ElementAdapter

Auto-generated from vendor docs for allplan 2024.0. 16 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/)

### Methods
#### `GetAttributes()`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetAttributes)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetCommonProperties)

#### `GetGeometryObject()`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetGeometryObject)

#### `GetLabelElements()`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetLabelElements)

#### `GetSubElementID()`

Get the sub element ID

**Remarks:** Get the sub element ID

**Returns:** `type` — Sub Element ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## ArchElementType (enum)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.ArchElementType.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ArchElementType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ArchElementType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ArchElementType/#NemAll_Python_IFW_ElementAdapter.ArchElementType.__getitem__)

### Values
- `tInvalid` = `0`
- `tSheet` = `2`
- `tSolid` = `3`
- `tWire` = `1`

## AssocViewElementAdapter (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AssocViewElementAdapter/)

### Constructors
- `AssocViewElementAdapter() | AssocViewElementAdapter(object) | AssocViewElementAdapter(ele)` — Initialize

### Methods
#### `GetTransformationMatrix()`

Get the transformation matrix of the associative view

**Remarks:** Get the transformation matrix of the associative view

**Returns:** `Matrix3D` — Transformation matrix of the associative view

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AssocViewElementAdapter/#NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter.GetTransformationMatrix)

#### `IsNull()`

Check for an empty element

**Remarks:** Check for an empty element

**Returns:** `bool` — Element is empty: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AssocViewElementAdapter/#NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter.IsNull)

## AxisElementAdapter (class)

Implementation of the axis element adapter

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/)

### Constructors
- `AxisElementAdapter() | AxisElementAdapter(element)` — Initialize

### Methods
#### `GetAxis()`

Get wall axis

**Remarks:** Get wall axis

**Returns:** `object` — IGeometry2D arbitrary curve pointer

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetAxis)

#### `GetGeometry()`

copydoc BaseElementAdapter::GetGeometry

**Remarks:** copydoc BaseElementAdapter::GetGeometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetGeometry)

#### `GetTiersCount()`

Get the amount of tiers

**Remarks:** Get the amount of tiers

**Returns:** `int` — number of tiers of this axis element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetTiersCount)

#### `HasAxis()`

Check, if this element has an axis

**Remarks:** Check, if this element has an axis

**Returns:** `bool` — true, if axis element has an axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.HasAxis)

#### `IsNull()`

copydoc BaseElementAdapter::IsNull

**Remarks:** copydoc BaseElementAdapter::IsNull

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.IsNull)

## BaseElementAdapter (class)

Implementation of the base element adapter

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/)

### Constructors
- `BaseElementAdapter() | BaseElementAdapter(element)` — Initialize

### Methods
#### `FromGUID(guid, doc)`

Get elements from GUID

**Remarks:** Get elements from GUID

**Parameters:**
- `guid` (GUID) — GUID of the element
- `doc` (DocumentAdapter) — Document

**Returns:** `BaseElementAdapter` — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.FromGUID)

#### `FromNOIGUID(guidstr, doc)`

Get elements from NOI GUID

**Remarks:** Get elements from NOI GUID

**Parameters:**
- `guidStr` (object) — NOI-GUID string of the element
- `doc` (DocumentAdapter) — Document

**Returns:** `BaseElementAdapter` — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.FromNOIGUID)

#### `GetArchElementType()`

Check the type of an architectural element (PRGBER_ELEM_ARCH)

**Remarks:** Check the type of an architectural element (PRGBER_ELEM_ARCH)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetArchElementType)

#### `GetAttributes(readState)`

Get the attributes

**Remarks:** Get the attributes

**Parameters:**
- `readState` (eAttibuteReadState) — Attribute read state

**Returns:** `list[tuple[int, Union[int, float, str]]]` — Attributes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetAttributes)

#### `GetCommonProperties()`

Get the common properties of the element

**Remarks:** Get the common properties of the element

**Returns:** `CommonProperties` — Common properties of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetCommonProperties)

#### `GetDisplayName()`

Get the displace name of the element

**Remarks:** Get the displace name of the element

**Returns:** `str` — Display name of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetDisplayName)

#### `GetDocument()`

Get the document

**Remarks:** Get the document

**Returns:** `DocumentAdapter` — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetDocument)

#### `GetDrawingfileNumber()`

Get the drawing file number of the element

**Remarks:** Get the drawing file number of the element

**Returns:** `int` — Drawing file number of the element / negative means drawing is in passive mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetDrawingfileNumber)

#### `GetElementAdapterType()`

Get the type of the element

**Remarks:** Get the type of the element

**Returns:** `ElementAdapterType` — Type ID of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetElementAdapterType)

#### `GetElementUUID()`

Get the UUID of the element

**Remarks:** Get the UUID of the element

**Returns:** `GUID` — UUID of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetElementUUID)

#### `GetGeometry()`

Get elements geometry

**Remarks:** Get elements geometry

**Returns:** `Any` — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetGeometry)

#### `GetGroundViewArchitectureElementGeometry()`

Get ground view geometry of an architecture element (if exist)

**Remarks:** Get ground view geometry of an architecture element (if exist)

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetGroundViewArchitectureElementGeometry)

#### `GetModelElementUUID()`

Get the UUID of the model element

**Remarks:** Get the UUID of the model element

**Returns:** `GUID` — UUID of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetModelElementUUID)

#### `GetModelGeometry()`

Get element model geometry

**Remarks:** Get element model geometry

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetModelGeometry)

#### `GetParentElementAdapterType()`

Get the type of the parent element

**Remarks:** Get the type of the parent element

**Returns:** `ElementAdapterType` — Type ID of the parent element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetParentElementAdapterType)

#### `GetPureArchitectureElementGeometry()`

Get the pure geometry of an architecture element (if exist). The geometry is without openings, ...

**Remarks:** Get the pure geometry of an architecture element (if exist). The geometry is without openings, ...

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetPureArchitectureElementGeometry)

#### `GetTimeStamp()`

Get the time stamp of the element

**Remarks:** Get the time stamp of the element

**Returns:** `int` — Time stamp of the element creation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetTimeStamp)

#### `Is3DElement()`

Get the 3D state of the element

**Remarks:** Get the 3D state of the element

**Returns:** `bool` — Element is a 3D element: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.Is3DElement)

#### `IsActive()`

Get the activation state of the element

**Remarks:** Get the activation state of the element

**Returns:** `bool` — True, if the element is activated

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsActive)

#### `IsChildParentType(childType, parentType)`

Check for child (current element) and parent type connection

**Remarks:** Check for child (current element) and parent type connection

**Parameters:**
- `childType` (GUID) — Type of the element
- `parentType` (GUID) — Type of the parent element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsChildParentType)

#### `IsDeleted()`

Get the deleted state of the element

**Remarks:** Get the deleted state of the element

**Returns:** `bool` — True, if the element is deleted

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsDeleted)

#### `IsGeneralElement()`

Check if element is general (PRGBER_ELEM_ALLG)

**Remarks:** Check if element is general (PRGBER_ELEM_ALLG)

**Returns:** `bool` — Element is general (PRGBER_ELEM_ALLG)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsGeneralElement)

#### `IsInActiveDocument()`

If element is in active document return true.

**Remarks:** If element is in active document return true.

**Returns:** `bool` — true if element is in active document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsInActiveDocument)

#### `IsInActiveLayer()`

If element is in active layer return true.

**Remarks:** If element is in active layer return true.

**Returns:** `bool` — true if element is in active layer.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsInActiveLayer)

#### `IsInMacro()`

Check if element has parent object

**Remarks:** Check if element has parent object

**Returns:** `bool` — Element has parent

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsInMacro)

#### `IsLabelElement()`

Check if element is a part of some label (Variables Textbild)

**Remarks:** Check if element is a part of some label (Variables Textbild)

**Returns:** `bool` — Element is a part of some label: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsLabelElement)

#### `IsNull()`

Check for an empty element

**Remarks:** Check for an empty element

**Returns:** `bool` — Element is empty: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsNull)

#### `IsValidForSelectFace()`

Check if element is valid for face select

**Remarks:** Check if element is valid for face select

**Returns:** `bool` — Element is valid for face select

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsValidForSelectFace)

#### `SetVisibilityState(visible)`

Set the visibility state of the element

**Remarks:** Set the visibility state of the element

**Parameters:**
- `visible` (bool) — She visibility state to set

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.SetVisibilityState)

#### `__eq__(element) | __eq__(eleTypeUUID)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (BaseElementAdapter) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.__eq__)

#### `__ne__(eleType) | __ne__(eleType)`

Not equal operator for checking the element type

**Remarks:** Not equal operator for checking the element type

**Parameters:**
- `eleType` (BaseElementAdapter) — Element type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.__repr__)

## BaseElementAdapterChildElementsService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/)

### Methods
#### `GetChildModelElements(ele, hidden=False)`

Get the child model elements of the element

**Remarks:** Get the child model elements of the element

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `hiddenElements` (object) — Include the hidden elements

**Returns:** `BaseElementAdapterList` — Child elements of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetChildModelElements)

#### `GetChildModelElementsFromTree(ele)`

Get the child model elements from the complete child tree of the element

**Remarks:** Get the child model elements from the complete child tree of the element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapterList` — Child model elements from the complete child tree of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetChildModelElementsFromTree)

#### `GetTierElements(ele)`

Get the tier elements from an element

**Remarks:** Get the tier elements from an element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapterVector` — Tier elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetTierElements)

## BaseElementAdapterList (class)

Implementation of the base element adapter list

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/)

### Constructors
- `BaseElementAdapterList() | BaseElementAdapterList(elements) | BaseElementAdapterList(eleList)` — Initialize

### Methods
#### `__getitem__(index)`

Get the item for the index

**Remarks:** Get the item for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__getitem__)

#### `__iadd__(elements)`

Add elements to the list

**Remarks:** Add elements to the list

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__iadd__)

#### `__iter__()`

Get the iterator

**Remarks:** Get the iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__iter__)

#### `__len__()`

Get the length of the list

**Remarks:** Get the length of the list

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__len__)

#### `__repr__()`

Get a string from the list items

**Remarks:** Get a string from the list items

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__repr__)

#### `append(element)`

Append an element

**Remarks:** Append an element

**Parameters:**
- `element` (BaseElementAdapter) — Element to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.append)

#### `clear()`

Clear the list

**Remarks:** Clear the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.clear)

## BaseElementAdapterParentElementService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.BaseElementAdapterParentElementService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterParentElementService/)

### Methods
#### `GetParentElement(ele)`

Get the parent element adapter from the element adapter

**Remarks:** Get the parent element adapter from the element adapter

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapter` — Parent element adapter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterParentElementService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterParentElementService.GetParentElement)

## BaseElementAdapterVector (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/)

### Constructors
- `BaseElementAdapterVector()` — Initialize

### Methods
#### `__getitem__(arg2)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (object) — Index of the item

**Returns:** `BaseElementAdapter` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__getitem__)

#### `__iadd__(elements)`

Add elements to the list

**Remarks:** Add elements to the list

**Parameters:**
- `elements` (BaseElementAdapterVector) — Elements.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__iadd__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__repr__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BaseElementAdapter) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.append)

#### `clear()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.clear)

## DocumentAdapter (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.DocumentAdapter.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/)

### Constructors
- `DocumentAdapter()` — Initialize

### Methods
#### `GetDocumentID()`

Get the document ID

**Remarks:** Get the document ID

**Returns:** `int` — Document ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/#NemAll_Python_IFW_ElementAdapter.DocumentAdapter.GetDocumentID)

#### `GetScalingFactor()`

Get the scaling factor

**Remarks:** Get the scaling factor

**Returns:** `float` — Scaling factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/#NemAll_Python_IFW_ElementAdapter.DocumentAdapter.GetScalingFactor)

## DocumentNameService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.DocumentNameService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/)

### Methods
#### `GetActiveDocumentName()`

Get the name of the active document

**Remarks:** Get the name of the active document

**Returns:** `str` — Name of the active document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetActiveDocumentName)

#### `GetDocumentName(ele, withNumber, withLabel, delimiter)`

Get the name of the element

**Remarks:** Get the name of the element

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `withNumber` (bool) — Add the document number: true/false
- `withLabel` (bool) — Add the label: true/false
- `delimiter` (str) — Delimiter between number and name

**Returns:** `str` — Name of the document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetDocumentName)

#### `GetDocumentNameByFileIndex(fileIndex, withNumber, withLabel, delimiter)`

Get the document name by the document index (load index)

**Remarks:** Get the document name by the document index (load index)

**Parameters:**
- `fileIndex` (int) — Document file index
- `withNumber` (bool) — Add the document number: true/false
- `withLabel` (bool) — Add the label: true/false
- `delimiter` (str) — Delimiter between number and name

**Returns:** `str` — Document name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetDocumentNameByFileIndex)

#### `GetDocumentNameByFileNumber(fileNumber, withNumber, withLabel, delimiter)`

Get the document name by the file number

**Remarks:** Get the document name by the file number

**Parameters:**
- `fileNumber` (int) — Document file number
- `withNumber` (bool) — Add the document number: true/false
- `withLabel` (bool) — Add the label: true/false
- `delimiter` (str) — Delimiter between number and name

**Returns:** `str` — Document name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetDocumentNameByFileNumber)

#### `GetLoadedDocumentsNameData()`

Get the names and file numbers of the loaded documents

**Remarks:** Get the names and file numbers of the loaded documents

**Returns:** `list[tuple[str, int]]` — Names of the loaded documents

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetLoadedDocumentsNameData)

## ElementAdapterType (class)

Implementation of the element type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/)

### Methods
#### `GetDisplayName()`

Get the display name of the element

**Remarks:** Get the display name of the element

**Returns:** `str` — Display name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetDisplayName)

#### `GetGuid()`

Get the GUID

**Remarks:** Get the GUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetGuid)

#### `GetModelType()`

Get the model type

**Remarks:** Get the model type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetModelType)

#### `GetTypeName()`

Get the type name of the element

**Remarks:** Get the type name of the element

**Returns:** `str` — Type name of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetTypeName)

#### `GetZoomState()`

Get the zoom state of the element type

**Remarks:** Get the zoom state of the element type

**Returns:** `object` — Zoom state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetZoomState)

#### `Is3DElement()`

Get the 3D element state

**Remarks:** Get the 3D element state

**Returns:** `bool` — Element has 3D geometry: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.Is3DElement)

#### `IsInfoFromParent()`

Get the element state for the element info

**Remarks:** Get the element state for the element info

**Returns:** `bool` — Use the element info from the parent element: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.IsInfoFromParent)

#### `__eq__(guid)`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `guid` (GUID) — Type to compare

**Returns:** `bool` — Types are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.__eq__)

#### `__ne__(guid)`

Not equal operator

**Remarks:** Not equal operator

**Parameters:**
- `guid` (GUID) — Type to compare

**Returns:** `bool` — Types are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.__ne__)

### Properties
- `IsVisible` (bool, get/set) — Get the visible state of the element type

## ElementAdapterTypeData (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.ElementAdapterTypeData.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterTypeData/)

### Methods
#### `GetElementAdapterType(uuid)`

Get the element adapter type for the defined UUID

**Remarks:** Get the element adapter type for the defined UUID

**Parameters:**
- `uuid` (GUID) — Type UUID

**Returns:** `ElementAdapterType` — Element adapter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterTypeData/#NemAll_Python_IFW_ElementAdapter.ElementAdapterTypeData.GetElementAdapterType)

## GUID (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.GUID.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/)

### Constructors
- `GUID()` — Initialize

### Methods
#### `FromString(strGuid)`

Create a GUID from a string

**Remarks:** Create a GUID from a string

**Parameters:**
- `strGuid` (str) — GUID as string

**Returns:** `GUID` — GUID from the string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.FromString)

#### `__eq__(compGuid)`

Compare a GUID

**Remarks:** Compare a GUID

**Parameters:**
- `compGuid` (GUID) — GUID to compare

**Returns:** `bool` — GUIDs are equal: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.__eq__)

#### `__hash__()`

Create a hash from the GUID

**Remarks:** Create a hash from the GUID

**Returns:** `int` — hash from the GUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.__hash__)

#### `__repr__()`

Create a string from the GUID

**Remarks:** Create a string from the GUID

**Returns:** `str` — string from the GUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.__repr__)

## PrecastPropertiesService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/)

### Methods
#### `GetPositionNumber()`

Get the precast element position number

**Remarks:** Get the precast element position number

**Returns:** `str` — position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/#NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.GetPositionNumber)

#### `GetPositionNumberPure()`

Get the precast element position number

**Remarks:** Get the precast element position number

**Returns:** `int` — position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/#NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.GetPositionNumberPure)

#### `GetPrecastElementTypeDescription(arg2)`

Get the precast element description (new wall system)

**Remarks:** Get the precast element description (new wall system)

**Returns:** `str` — description

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/#NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.GetPrecastElementTypeDescription)

## ReinforcementPropertiesReader (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ReinforcementPropertiesReader/)

### Methods
#### `GetPositionNumber(ele)`

Get the position number

**Remarks:** Get the position number

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ReinforcementPropertiesReader/#NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader.GetPositionNumber)

#### `GetPositionNumberFromRepresentation(ele)`

Get the position number from a representation element

**Remarks:** Get the position number from a representation element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ReinforcementPropertiesReader/#NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader.GetPositionNumberFromRepresentation)

