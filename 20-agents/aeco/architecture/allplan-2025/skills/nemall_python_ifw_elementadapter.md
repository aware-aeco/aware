---
name: allplan-nemall_python_ifw_elementadapter
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_IFW_ElementAdapter namespace — 19 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AllplanElement, ArchElementType, Functions, AxisElementAdapter, AssocViewElementAdapter, BaseElementAdapterParentElementService, BaseElementAdapterList, BaseElementAdapterChildElementsService, and 11 more types.
---

# NemAll_Python_IFW_ElementAdapter

Auto-generated from vendor docs for allplan 2025.0. 19 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/)

### Methods
#### `GetAttributes() -> object`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetAttributes)

#### `GetBaseElementAdapter() -> BaseElementAdapter`

Get the model element

**Remarks:** Get the model element

**Returns:** `BaseElementAdapter` — Model element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetBaseElementAdapter)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetCommonProperties)

#### `GetGeometryObject() -> object`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetGeometryObject)

#### `GetLabelElements() -> list`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetLabelElements)

#### `GetSubElementID() -> type`

Get the SubElementID

**Remarks:** Get the SubElementID

**Returns:** `type` — SubElementID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer: object)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey: str)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject: object)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements: list)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AllplanElement/#NemAll_Python_IFW_ElementAdapter.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## ArchElementType (enum)

Architecture element types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ArchElementType/)

### Values
- `tInvalid` = `0`
- `tSheet` = `2`
- `tSolid` = `3`
- `tWire` = `1`

## AssocViewElementAdapter (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AssocViewElementAdapter/)

### Constructors
- `AssocViewElementAdapter() | AssocViewElementAdapter(ele: BaseElementAdapter)` — Initialize

### Methods
#### `GetTransformationMatrix() -> Matrix3D`

Get the transformation matrix of the associative view

**Remarks:** Get the transformation matrix of the associative view

**Returns:** `Matrix3D` — Transformation matrix of the associative view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AssocViewElementAdapter/#NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter.GetTransformationMatrix)

#### `IsNull() -> bool`

Check for an empty element

**Remarks:** Check for an empty element

**Returns:** `bool` — Element is empty: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AssocViewElementAdapter/#NemAll_Python_IFW_ElementAdapter.AssocViewElementAdapter.IsNull)

## AxisElementAdapter (class)

Implementation of the axis element adapter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/)

### Constructors
- `AxisElementAdapter() | AxisElementAdapter(element: BaseElementAdapter) | AxisElementAdapter(element: AxisElementAdapter)` — Initialize

### Methods
#### `GetAxis() -> Any`

Get wall axis

**Remarks:** Get wall axis

**Returns:** `Any` — IGeometry2D arbitrary curve pointer

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetAxis)

#### `GetGeometry() -> Any`

Get elements geometry

**Remarks:** Get elements geometry

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetGeometry)

#### `GetOffset() -> float`

Get the element offset from the axis

**Remarks:** Get the element offset from the axis

**Returns:** `float` — Element offset from the axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetOffset)

#### `GetThickness() -> float`

Get the element thickness

**Remarks:** Get the element thickness

**Returns:** `float` — Element thickness

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetThickness)

#### `GetTierThickness() -> VecDoubleList`

Get the thickness of the tiers

**Remarks:** Get the thickness of the tiers

**Returns:** `VecDoubleList` — Thickness of the tiers

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetTierThickness)

#### `GetTiersCount() -> int`

Get the amount of tiers

**Remarks:** Get the amount of tiers

**Returns:** `int` — number of tiers of this axis element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.GetTiersCount)

#### `HasAxis() -> bool`

Check, if this element has an axis

**Remarks:** Check, if this element has an axis

**Returns:** `bool` — true, if axis element has an axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.HasAxis)

#### `IsElementParallelToAxis() -> bool`

Is element direction parallel to axis direction

**Remarks:** Is element direction parallel to axis direction

**Returns:** `bool` — Is element direction parallel to axis direction state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.IsElementParallelToAxis)

#### `IsNull() -> bool`

Check for an empty element

**Remarks:** Check for an empty element

**Returns:** `bool` — Element is empty: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/AxisElementAdapter/#NemAll_Python_IFW_ElementAdapter.AxisElementAdapter.IsNull)

## BaseElementAdapter (class)

Implementation of the base element adapter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/)

### Constructors
- `BaseElementAdapter() | BaseElementAdapter(element: BaseElementAdapter)` — Initialize

### Methods
#### `FromGUID(guid: GUID, doc: DocumentAdapter) -> BaseElementAdapter`

Get elements from GUID

**Remarks:** Get elements from GUID

**Parameters:**
- `guid` (GUID) — GUID of the element
- `doc` (DocumentAdapter) — Document

**Returns:** `BaseElementAdapter` — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.FromGUID)

#### `FromNOIGUID(guidstr: str, doc: DocumentAdapter) -> BaseElementAdapter`

Get elements from NOI GUID

**Remarks:** Get elements from NOI GUID

**Parameters:**
- `guidStr` (object) — NOI-GUID string of the element
- `doc` (DocumentAdapter) — Document

**Returns:** `BaseElementAdapter` — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.FromNOIGUID)

#### `GetArchElementType() -> ArchElementType`

Check the type of an architectural element (PRGBER_ELEM_ARCH)

**Remarks:** Check the type of an architectural element (PRGBER_ELEM_ARCH)

**Returns:** `ArchElementType` — Arch Element type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetArchElementType)

#### `GetAttributes( readState: eAttibuteReadState, ) -> list[tuple[int, int | float | str]]`

Get the attributes

**Remarks:** Get the attributes

**Parameters:**
- `readState` (eAttibuteReadState) — Attribute read state

**Returns:** `list[tuple[int, int | float | str]]` — Attributes

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetAttributes)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties of the element

**Remarks:** Get the common properties of the element

**Returns:** `CommonProperties` — Common properties of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetCommonProperties)

#### `GetDisplayName() -> str`

Get the displace name of the element

**Remarks:** Get the displace name of the element

**Returns:** `str` — Display name of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetDisplayName)

#### `GetDocument() -> DocumentAdapter`

Get the document

**Remarks:** Get the document

**Returns:** `DocumentAdapter` — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetDocument)

#### `GetDrawingfileNumber() -> int`

Get the drawing file number of the element

**Remarks:** Get the drawing file number of the element

**Returns:** `int` — Drawing file number of the element / negative means drawing is in passive mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetDrawingfileNumber)

#### `GetElementAdapterType() -> ElementAdapterType`

Get the type of the element

**Remarks:** Get the type of the element

**Returns:** `ElementAdapterType` — Type ID of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetElementAdapterType)

#### `GetElementUUID() -> GUID`

Get the UUID of the element

**Remarks:** Get the UUID of the element

**Returns:** `GUID` — UUID of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetElementUUID)

#### `GetGeometry() -> Any`

Get elements geometry

**Remarks:** Get elements geometry

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetGeometry)

#### `GetGroundViewArchitectureElementGeometry() -> Any`

Get ground view geometry of an architecture element (if exist)

**Remarks:** Get ground view geometry of an architecture element (if exist)

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetGroundViewArchitectureElementGeometry)

#### `GetModelElementUUID() -> GUID`

Get the UUID of the model element

**Remarks:** Get the UUID of the model element

**Returns:** `GUID` — UUID of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetModelElementUUID)

#### `GetModelGeometry() -> Any`

Get element model geometry

**Remarks:** Get element model geometry

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetModelGeometry)

#### `GetNOIGUID() -> str`

Get the NOI GUID

**Remarks:** Get the NOI GUID

**Returns:** `str` — NOI GUID as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetNOIGUID)

#### `GetParentElementAdapterType() -> ElementAdapterType`

Get the type of the parent element

**Remarks:** Get the type of the parent element

**Returns:** `ElementAdapterType` — Type ID of the parent element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetParentElementAdapterType)

#### `GetPureArchitectureElementGeometry() -> Any`

Get the pure geometry of an architecture element (if exist). The geometry is without openings, ...

**Remarks:** Get the pure geometry of an architecture element (if exist). The geometry is without openings, ...

**Returns:** `Any` — Geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetPureArchitectureElementGeometry)

#### `GetTimeStamp() -> int`

Get the time stamp of the element

**Remarks:** Get the time stamp of the element

**Returns:** `int` — Time stamp of the element creation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.GetTimeStamp)

#### `Is3DElement() -> bool`

Get the 3D state of the element

**Remarks:** Get the 3D state of the element

**Returns:** `bool` — Element is a 3D element: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.Is3DElement)

#### `IsActive() -> bool`

Get the activation state of the element

**Remarks:** Get the activation state of the element

**Returns:** `bool` — True, if the element is activated

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsActive)

#### `IsChildParentType(childType: GUID, parentType: GUID) -> bool`

Check for child (current element) and parent type connection

**Remarks:** Check for child (current element) and parent type connection

**Parameters:**
- `childType` (GUID) — Type of the element
- `parentType` (GUID) — Type of the parent element

**Returns:** `bool` — Child and parent are connected by the define types

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsChildParentType)

#### `IsDeleted() -> bool`

Get the deleted state of the element

**Remarks:** Get the deleted state of the element

**Returns:** `bool` — True, if the element is deleted

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsDeleted)

#### `IsGeneralElement() -> bool`

Check if element is general (PRGBER_ELEM_ALLG)

**Remarks:** Check if element is general (PRGBER_ELEM_ALLG)

**Returns:** `bool` — Element is general (PRGBER_ELEM_ALLG)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsGeneralElement)

#### `IsInActiveDocument() -> bool`

If element is in active document return true.

**Remarks:** If element is in active document return true.

**Returns:** `bool` — true if element is in active document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsInActiveDocument)

#### `IsInActiveLayer() -> bool`

If element is in active layer return true.

**Remarks:** If element is in active layer return true.

**Returns:** `bool` — true if element is in active layer.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsInActiveLayer)

#### `IsInMacro() -> bool`

Check if element has parent object

**Remarks:** Check if element has parent object

**Returns:** `bool` — Element has parent

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsInMacro)

#### `IsLabelElement() -> bool`

Check if element is a part of some label (Variables Textbild)

**Remarks:** Check if element is a part of some label (Variables Textbild)

**Returns:** `bool` — Element is a part of some label: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsLabelElement)

#### `IsNull() -> bool`

Check for an empty element

**Remarks:** Check for an empty element

**Returns:** `bool` — Element is empty: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsNull)

#### `IsValid() -> bool`

check if element is valid

**Remarks:** check if element is valid

**Returns:** `bool` — bool, if it's possible to work with it

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsValid)

#### `IsValidForSelectFace() -> bool`

Check if element is valid for face select

**Remarks:** Check if element is valid for face select

**Returns:** `bool` — Element is valid for face select

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.IsValidForSelectFace)

#### `SetVisibilityState(visible: bool)`

Set the visibility state of the element

**Remarks:** Set the visibility state of the element

**Parameters:**
- `visible` (bool) — She visibility state to set

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.SetVisibilityState)

#### `__eq__(element: BaseElementAdapter) -> bool | __eq__(eleTypeUUID: GUID) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `element` (BaseElementAdapter) — Element to compare

**Returns:** `bool` — Elements are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.__eq__)

#### `__ne__(eleType: BaseElementAdapter) -> bool | __ne__(eleType: GUID) -> bool`

Not equal operator for checking the element type

**Remarks:** Not equal operator for checking the element type

**Parameters:**
- `eleType` (BaseElementAdapter) — Element type

**Returns:** `bool` — Element has not the type: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapter.__repr__)

## BaseElementAdapterChildElementsService (class)

Implementation of the service functions for the child elements determination of a base element adapter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/)

### Methods
#### `GetChildElements( ele: BaseElementAdapter, hiddenElements: bool ) -> BaseElementAdapterVector`

Get the child elements of the element (includes the childs which have the same model element)

**Remarks:** Get the child elements of the element (includes the childs which have the same model element)

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `hiddenElements` (bool) — Include hidden elements: true/false

**Returns:** `BaseElementAdapterVector` — Child elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetChildElements)

#### `GetChildModelElements(ele: BaseElementAdapter) -> BaseElementAdapterList | GetChildModelElements( ele: BaseElementAdapter, hiddenElements: bool ) -> BaseElementAdapterList`

Get the child model elements of the element

**Remarks:** Get the child model elements of the element

**Parameters:**
- `ele` (BaseElementAdapter) — BaseElementAdapter Element

**Returns:** `BaseElementAdapterList` — True, if element pass and can be selected

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetChildModelElements)

#### `GetChildModelElementsFromTree( ele: BaseElementAdapter, ) -> BaseElementAdapterList`

Get the child model elements from the complete child tree of the element

**Remarks:** Get the child model elements from the complete child tree of the element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapterList` — Child model elements from the complete child tree of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetChildModelElementsFromTree)

#### `GetChildPropertyElements( ele: BaseElementAdapter, modifyState: object, propGroup: object ) -> BaseElementAdapterVector`

Get the child property elements

**Remarks:** Get the child property elements

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `modifyState` (object) — Modify state
- `propGroup` (object) — Property group

**Returns:** `BaseElementAdapterVector` — Child property elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetChildPropertyElements)

#### `GetTierElements(ele: BaseElementAdapter) -> BaseElementAdapterVector`

Get the tier elements from an element

**Remarks:** Get the tier elements from an element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapterVector` — Tier elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetTierElements)

#### `GetTierNumber(ele: BaseElementAdapter) -> int`

Get the tier number from an element

**Remarks:** Get the tier number from an element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Tier number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterChildElementsService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterChildElementsService.GetTierNumber)

## BaseElementAdapterList (class)

Implementation of the base element adapter list

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/)

### Constructors
- `BaseElementAdapterList() | BaseElementAdapterList(elements: List[BaseElementAdapter]) | BaseElementAdapterList(eleList: BaseElementAdapterList)` — Initialize

### Methods
#### `__getitem__(index: int) -> BaseElementAdapter`

Get the item for the index

**Remarks:** Get the item for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__getitem__)

#### `__iadd__(elements: BaseElementAdapterList) -> BaseElementAdapterList`

Add elements to the list

**Remarks:** Add elements to the list

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__iadd__)

#### `__iter__() -> Iterator[BaseElementAdapter]`

Get the iterator

**Remarks:** Get the iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__iter__)

#### `__len__() -> int`

Get the length of the list

**Remarks:** Get the length of the list

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__len__)

#### `__repr__() -> str`

Get a string from the list items

**Remarks:** Get a string from the list items

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.__repr__)

#### `append(element: BaseElementAdapter)`

Append an element

**Remarks:** Append an element

**Parameters:**
- `element` (BaseElementAdapter) — Element to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.append)

#### `clear()`

Clear the list

**Remarks:** Clear the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterList/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterList.clear)

## BaseElementAdapterParentElementService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.BaseElementAdapterParentElementService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterParentElementService/)

### Methods
#### `GetParentElement(ele: BaseElementAdapter) -> BaseElementAdapter`

Get the parent element adapter from the element adapter

**Remarks:** Get the parent element adapter from the element adapter

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapter` — Parent element adapter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterParentElementService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterParentElementService.GetParentElement)

## BaseElementAdapterService (class)

Implementation of the base element adapter service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/)

### Methods
#### `AdaptElementsAfterTransaction( elements: BaseElementAdapterList, ) -> BaseElementAdapterList`

Adapt the elements after a transaction (set new address, ...)

**Remarks:** Adapt the elements after a transaction (set new address, ...)

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements to adapt

**Returns:** `BaseElementAdapterList` — Adapted elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.AdaptElementsAfterTransaction)

#### `GetConnectedElements(ele: BaseElementAdapter) -> BaseElementAdapterList`

Get the connected elements (ivk2)

**Remarks:** Get the connected elements (ivk2)

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapterList` — Connected elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.GetConnectedElements)

#### `GetDocumentIndex(ele: BaseElementAdapter) -> int`

Get the internal index of the element-document

**Remarks:** Get the internal index of the element-document

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Internal index of the element-document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.GetDocumentIndex)

#### `GetLinkedElements(ele: BaseElementAdapter) -> BaseElementAdapterVector`

Get the linked elements

**Remarks:** Get the linked elements

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `BaseElementAdapterVector` — Linked elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.GetLinkedElements)

#### `GetMacroElementAddress(ele: BaseElementAdapter) -> int`

Get the array address of macro element, if exist

**Remarks:** Get the array address of macro element, if exist

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Address of macro if exist, otherwise 0

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.GetMacroElementAddress)

#### `IsConnectedElement(ele1: BaseElementAdapter, ele2: BaseElementAdapter) -> bool`

Check, whether the two elements are connected

**Remarks:** Check, whether the two elements are connected

**Parameters:**
- `ele1` (BaseElementAdapter) — First element
- `ele2` (BaseElementAdapter) — Second element

**Returns:** `bool` — Elements are connected: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.IsConnectedElement)

#### `IsElementFromElementGroup(ele: BaseElementAdapter) -> bool`

Check, whether the element is inside an element group

**Remarks:** Check, whether the element is inside an element group

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `bool` — Element is inside an element group

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.IsElementFromElementGroup)

#### `IsElementOrChildDeleted(ele: BaseElementAdapter) -> bool`

Check, whether the element or a child element is deleted

**Remarks:** Check, whether the element or a child element is deleted

**Parameters:**
- `ele` (BaseElementAdapter) — Element to check

**Returns:** `bool` — Element or child element is deleted

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.IsElementOrChildDeleted)

#### `IsElementVisible(ele: BaseElementAdapter) -> bool`

Check, whether the element is visible

**Remarks:** Check, whether the element is visible

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `bool` — Element is visible: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.IsElementVisible)

#### `IsPostActivation(ele: BaseElementAdapter) -> bool`

Get the post activation state from the element

**Remarks:** Get the post activation state from the element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `bool` — Element with post activation: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.IsPostActivation)

#### `IsTypeConnectedElement(ele: BaseElementAdapter, eleTypes: object) -> bool`

Check, whether the element is a type connected element

**Remarks:** Check, whether the element is a type connected element

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `eleTypes` (object) — Type(s) of the connection element

**Returns:** `bool` — Text connected element: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterService/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterService.IsTypeConnectedElement)

## BaseElementAdapterVector (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/)

### Constructors
- `BaseElementAdapterVector()` — Initialize

### Methods
#### `__getitem__(arg2: int) -> BaseElementAdapter`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (object) — Index of the item

**Returns:** `BaseElementAdapter` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__getitem__)

#### `__iadd__(elements: BaseElementAdapterVector) -> BaseElementAdapterVector`

Add elements to the list

**Remarks:** Add elements to the list

**Parameters:**
- `elements` (BaseElementAdapterVector) — Elements.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__iadd__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__len__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.__repr__)

#### `append(value: BaseElementAdapter)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BaseElementAdapter) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.append)

#### `clear()`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapterVector/#NemAll_Python_IFW_ElementAdapter.BaseElementAdapterVector.clear)

## DocumentAdapter (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.DocumentAdapter.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/)

### Constructors
- `DocumentAdapter()` — Initialize

### Methods
#### `GetDocumentID() -> int`

Get the document ID

**Remarks:** Get the document ID

**Returns:** `int` — Document ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/#NemAll_Python_IFW_ElementAdapter.DocumentAdapter.GetDocumentID)

#### `GetScalingFactor() -> float`

Get the scaling factor

**Remarks:** Get the scaling factor

**Returns:** `float` — Scaling factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/#NemAll_Python_IFW_ElementAdapter.DocumentAdapter.GetScalingFactor)

## DocumentNameService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.DocumentNameService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/)

### Methods
#### `GetActiveDocumentName() -> str`

Get the name of the active document

**Remarks:** Get the name of the active document

**Returns:** `str` — Name of the active document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetActiveDocumentName)

#### `GetDocumentName( ele: BaseElementAdapter, withNumber: bool, withLabel: bool, delimiter: str ) -> str`

Get the name of the element

**Remarks:** Get the name of the element

**Parameters:**
- `ele` (BaseElementAdapter) — Element
- `withNumber` (bool) — Add the document number: true/false
- `withLabel` (bool) — Add the label: true/false
- `delimiter` (str) — Delimiter between number and name

**Returns:** `str` — Name of the document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetDocumentName)

#### `GetDocumentNameByFileIndex( fileIndex: int, withNumber: bool, withLabel: bool, delimiter: str ) -> str`

Get the document name by the document index (load index)

**Remarks:** Get the document name by the document index (load index)

**Parameters:**
- `fileIndex` (int) — Document file index
- `withNumber` (bool) — Add the document number: true/false
- `withLabel` (bool) — Add the label: true/false
- `delimiter` (str) — Delimiter between number and name

**Returns:** `str` — Document name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetDocumentNameByFileIndex)

#### `GetDocumentNameByFileNumber( fileNumber: int, withNumber: bool, withLabel: bool, delimiter: str ) -> str`

Get the document name by the file number

**Remarks:** Get the document name by the file number

**Parameters:**
- `fileNumber` (int) — Document file number
- `withNumber` (bool) — Add the document number: true/false
- `withLabel` (bool) — Add the label: true/false
- `delimiter` (str) — Delimiter between number and name

**Returns:** `str` — Document name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetDocumentNameByFileNumber)

#### `GetLoadedDocumentsNameData() -> list[tuple[str, int]]`

Get the names and file numbers of the loaded documents

**Remarks:** Get the names and file numbers of the loaded documents

**Returns:** `list[tuple[str, int]]` — Names of the loaded documents

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentNameService/#NemAll_Python_IFW_ElementAdapter.DocumentNameService.GetLoadedDocumentsNameData)

## ElementAdapterType (class)

Implementation of the element type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/)

### Methods
#### `GetDisplayName() -> str`

Get the display name of the element

**Remarks:** Get the display name of the element

**Returns:** `str` — Display name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetDisplayName)

#### `GetGuid() -> GUID`

Get the GUID

**Remarks:** Get the GUID

**Returns:** `GUID` — GUID of the element type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetGuid)

#### `GetModelType() -> GUID`

Get the model type

**Remarks:** Get the model type

**Returns:** `GUID` — Model type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetModelType)

#### `GetTypeName() -> str`

Get the type name of the element

**Remarks:** Get the type name of the element

**Returns:** `str` — Type name of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetTypeName)

#### `GetZoomState() -> object`

Get the zoom state of the element type

**Remarks:** Get the zoom state of the element type

**Returns:** `object` — Zoom state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.GetZoomState)

#### `Is3DElement() -> bool`

Get the 3D element state

**Remarks:** Get the 3D element state

**Returns:** `bool` — Element has 3D geometry: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.Is3DElement)

#### `IsInfoFromParent() -> bool`

Get the element state for the element info

**Remarks:** Get the element state for the element info

**Returns:** `bool` — Use the element info from the parent element: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.IsInfoFromParent)

#### `IsTypeGroup(typeGroup: ElementAdapterTypeGroup) -> bool`

Check for the type group

**Remarks:** Check for the type group

**Parameters:**
- `typeGroup` (ElementAdapterTypeGroup) — Type group to check

**Returns:** `bool` — Element is in the type group: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.IsTypeGroup)

#### `__eq__(guid: GUID) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `guid` (GUID) — Type to compare

**Returns:** `bool` — Types are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.__eq__)

#### `__ne__(guid: GUID) -> bool`

Not equal operator

**Remarks:** Not equal operator

**Parameters:**
- `guid` (GUID) — Type to compare

**Returns:** `bool` — Types are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterType/#NemAll_Python_IFW_ElementAdapter.ElementAdapterType.__ne__)

### Properties
- `DisplayName` (str, get/set) — Get the display name of the element
- `Guid` (GUID, get/set) — Get the GUID
- `IsVisible` (bool, get/set) — Get the visible state of the element type

## ElementAdapterTypeData (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.ElementAdapterTypeData.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterTypeData/)

### Methods
#### `GetElementAdapterType(uuid: GUID) -> ElementAdapterType`

Get the element adapter type for the defined UUID

**Remarks:** Get the element adapter type for the defined UUID

**Parameters:**
- `uuid` (GUID) — Type UUID

**Returns:** `ElementAdapterType` — Element adapter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterTypeData/#NemAll_Python_IFW_ElementAdapter.ElementAdapterTypeData.GetElementAdapterType)

## ElementAdapterTypeGroup (enum)

Implementation of the element adapter type groups

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ElementAdapterTypeGroup/)

### Values
- `eAREAS3D_GROUP` = `5`
- `eAXIS_ELEMENT_GROUP` = `17`
- `eBARS_PLACEMENT_GROUP` = `18`
- `eBREPS3D_GROUP` = `1`
- `eBREPS3D_SURFACE_GROUP` = `2`
- `eCURVES3D_GROUP` = `4`
- `eDEFECT_ELEMENT_GROUP` = `19`
- `eDIMENSION_GROUP` = `8`
- `eFIXTURE_GROUP` = `11`
- `eHYPERELEMENT_GROUP` = `23`
- `eHYPERELEMENT_TIER_GROUP` = `25`
- `eLAST_GROUP` = `26`
- `eLINES3D_GROUP` = `3`
- `eNO_GROUP` = `0`
- `eOPENINGPART_GROUP` = `24`
- `ePIPEBUNDLE_GROUP` = `16`
- `ePIPEWORK_GROUP` = `15`
- `ePRECAST_GROUP` = `22`
- `ePYTHON_PART_GROUP` = `14`
- `eREVEAL_GROUP` = `12`
- `eSMART_PART_GROUP` = `10`
- `eSMART_SYMBOL_GROUP` = `9`
- `eTEXT_GROUP` = `7`
- `eVOLUMES3D_GROUP` = `6`
- `eWALL_OPENING_GROUP` = `20`
- `eWALL_OPENING_SUB_ELEMENTS_GROUP` = `21`
- `eXREF_GROUP` = `13`

## Functions (static-class)

Module-level functions of NemAll_Python_IFW_ElementAdapter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/)

## GUID (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.GUID.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/)

### Constructors
- `GUID()` — Initialize

### Methods
#### `FromString(strGuid: str) -> GUID`

Create a GUID from a string

**Remarks:** Create a GUID from a string

**Parameters:**
- `strGuid` (str) — GUID as string

**Returns:** `GUID` — GUID from the string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.FromString)

#### `__eq__(compGuid: GUID) -> bool`

Compare a GUID

**Remarks:** Compare a GUID

**Parameters:**
- `compGuid` (GUID) — GUID to compare

**Returns:** `bool` — GUIDs are equal: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.__eq__)

#### `__hash__() -> int`

Create a hash from the GUID

**Remarks:** Create a hash from the GUID

**Returns:** `int` — hash from the GUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.__hash__)

#### `__repr__() -> str`

Create a string from the GUID

**Remarks:** Create a string from the GUID

**Returns:** `str` — string from the GUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/GUID/#NemAll_Python_IFW_ElementAdapter.GUID.__repr__)

## PrecastPropertiesService (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/)

### Methods
#### `GetPositionNumber() -> str`

Get the precast element position number

**Remarks:** Get the precast element position number

**Returns:** `str` — position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/#NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.GetPositionNumber)

#### `GetPositionNumberPure() -> int`

Get the precast element position number

**Remarks:** Get the precast element position number

**Returns:** `int` — position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/#NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.GetPositionNumberPure)

#### `GetPrecastElementTypeDescription(arg2: bool) -> str`

Get the precast element description (new wall system)

**Remarks:** Get the precast element description (new wall system)

**Returns:** `str` — description

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/PrecastPropertiesService/#NemAll_Python_IFW_ElementAdapter.PrecastPropertiesService.GetPrecastElementTypeDescription)

## ReinforcementPropertiesReader (class)

(No description provided in vendor docs for NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ReinforcementPropertiesReader/)

### Methods
#### `GetPositionNumber(ele: BaseElementAdapter) -> int`

Get the position number

**Remarks:** Get the position number

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ReinforcementPropertiesReader/#NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader.GetPositionNumber)

#### `GetPositionNumberFromRepresentation(ele: BaseElementAdapter) -> int`

Get the position number from a representation element

**Remarks:** Get the position number from a representation element

**Parameters:**
- `ele` (BaseElementAdapter) — Element

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/ReinforcementPropertiesReader/#NemAll_Python_IFW_ElementAdapter.ReinforcementPropertiesReader.GetPositionNumberFromRepresentation)

