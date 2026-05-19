---
name: allplan-nemall_python_archelements
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_ArchElements namespace — 37 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AllplanElement, ArchElement, BasePlaneReferences, ArchBaseProperties, BeamProperties, BeamElement, AxisProperties, CircularShape, and 29 more types.
---

# NemAll_Python_ArchElements

Auto-generated from vendor docs for allplan 2024.0. 37 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/)

### Methods
#### `GetAttributes()`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetAttributes)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetCommonProperties)

#### `GetGeometryObject()`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetGeometryObject)

#### `GetLabelElements()`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetLabelElements)

#### `GetSubElementID()`

Get the sub element ID

**Remarks:** Get the sub element ID

**Returns:** `type` — Sub Element ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## ArchBaseProperties (enum)

Implementation of the architecture base properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/)

### Constructors
- `ArchBaseProperties()` — Initialize
- `ArchBaseProperties(macroType)` — Constructor
- `ArchBaseProperties(baseProp)` — Copy constructor

### Methods
#### `GetAreaPresentationID()`

Get the ID of the area representation

**Remarks:** Get the ID of the area representation

**Returns:** `int` — ID of the area representation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetAreaPresentationID)

#### `GetAreaPresentationType()`

Get the type of the area representation

**Remarks:** Get the type of the area representation

**Returns:** `int` — Type of the area representation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetAreaPresentationType)

#### `GetBackgroundColor()`

Get the background color

**Remarks:** Get the background color

**Returns:** `int` — Background color

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetBackgroundColor)

#### `GetBitmapName()`

Get the name of the bitmap

**Remarks:** Get the name of the bitmap

**Returns:** `str` — Name of the bitmap

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetBitmapName)

#### `GetCalculationMode()`

Get the calculation model

**Remarks:** Get the calculation model

**Returns:** `int` — Calculation model

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetCalculationMode)

#### `GetCircleDivision()`

Get the circle division

**Remarks:** Get the circle division

**Returns:** `float` — Circle division

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetCircleDivision)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetCommonProperties)

#### `GetFaceStyle()`

Get the face style ID

**Remarks:** Get the face style ID

**Returns:** `int` — Face style ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetFaceStyle)

#### `GetFactor()`

Get the factor

**Remarks:** Get the factor

**Returns:** `float` — Factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetFactor)

#### `GetFilling()`

Get the filling ID

**Remarks:** Get the filling ID

**Returns:** `int` — Filling ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetFilling)

#### `GetHatch()`

Get the hatch ID

**Remarks:** Get the hatch ID

**Returns:** `int` — Hatch ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetHatch)

#### `GetMacroType()`

Get the macro type

**Remarks:** Get the macro type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetMacroType)

#### `GetMaterial()`

Get the material

**Remarks:** Get the material

**Returns:** `str` — Material

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetMaterial)

#### `GetName()`

Get the name

**Remarks:** Get the name

**Returns:** `str` — Name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetName)

#### `GetPattern()`

Get the pattern ID

**Remarks:** Get the pattern ID

**Returns:** `int` — Pattern ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetPattern)

#### `GetPlaneReferences()`

Get the plane references

**Remarks:** Get the plane references

**Returns:** `PlaneReferences` — Plane references

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetPlaneReferences)

#### `GetPriority()`

Get the priority

**Remarks:** Get the priority

**Returns:** `int` — Priority

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetPriority)

#### `GetProfileFullName()`

Get the full name of the profile

**Remarks:** Get the full name of the profile

**Returns:** `str` — Path and name of the profile

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetProfileFullName)

#### `GetShapeType()`

Get the shape type

**Remarks:** Get the shape type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetShapeType)

#### `GetSurface()`

Get the surface name and path

**Remarks:** Get the surface name and path

**Returns:** `str` — Surface name and path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetSurface)

#### `GetTrade()`

Get the trade index

**Remarks:** Get the trade index

**Returns:** `int` — Trade index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetTrade)

#### `IsShowAreaElementInGroundView()`

Get the 'Show the area element in the ground view' state

**Remarks:** Get the 'Show the area element in the ground view' state

**Returns:** `bool` — Show the area element in the ground view: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.IsShowAreaElementInGroundView)

#### `LoadFromFavoriteFile(doc)`

Load the properties from the favorite file

**Remarks:** Load the properties from the favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.LoadFromFavoriteFile)

#### `RemoveCommonProperties()`

Remove the common properties

**Remarks:** Remove the common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.RemoveCommonProperties)

#### `ResetAreaElement()`

Reset the area element

**Remarks:** Reset the area element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.ResetAreaElement)

#### `ResetBackgroundColor()`

Reset the background color

**Remarks:** Reset the background color

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.ResetBackgroundColor)

#### `SetBackgroundColor(colorID)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `colorID` (int) — Background color ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetBackgroundColor)

#### `SetBitmapName(bitmapName)`

Set the name of the bitmap

**Remarks:** Set the name of the bitmap

**Parameters:**
- `bitmapName` (str) — Bitmap name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetBitmapName)

#### `SetCalculationMode(calculationMode)`

Set the calculation mode

**Remarks:** Set the calculation mode

**Parameters:**
- `calculationMode` (int) — Calculation mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetCalculationMode)

#### `SetCircleDivision(circleDivision)`

Set the circle division

**Remarks:** Set the circle division

**Parameters:**
- `circleDivision` (int) — Circle division

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetCircleDivision)

#### `SetCommonProperties(comProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `comProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetCommonProperties)

#### `SetFaceStyle(faceStyleID)`

Set the face style

**Remarks:** Set the face style

**Parameters:**
- `faceStyleID` (int) — Face style ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetFaceStyle)

#### `SetFactor(factor)`

Set the factor

**Remarks:** Set the factor

**Parameters:**
- `factor` (float) — Factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetFactor)

#### `SetFilling(fillingID)`

Set the filling

**Remarks:** Set the filling

**Parameters:**
- `fillingID` (int) — Filling ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetFilling)

#### `SetHatch(hatchID)`

Set the hatch

**Remarks:** Set the hatch

**Parameters:**
- `hatchID` (int) — Hatch ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetHatch)

#### `SetMaterial(material)`

Set the material

**Remarks:** Set the material

**Parameters:**
- `material` (str) — Material

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetMaterial)

#### `SetName(name)`

Set the name

**Remarks:** Set the name

**Parameters:**
- `name` (str) — Name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetName)

#### `SetPathElementAxisType(pathEleGeoType)`

Set the axis type of the path element

**Remarks:** Set the axis type of the path element

**Parameters:**
- `pathEleGeoType` (int) — Axis type of the path element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPathElementAxisType)

#### `SetPattern(patternID)`

Set the pattern

**Remarks:** Set the pattern

**Parameters:**
- `patternID` (int) — Pattern ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPattern)

#### `SetPlaneReferences(planeRef)`

Set the plane references

**Remarks:** Set the plane references

**Parameters:**
- `planeRef` (PlaneReferences) — Plane references

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPlaneReferences)

#### `SetPriority(priority)`

Set the priority

**Remarks:** Set the priority

**Parameters:**
- `priority` (int) — Priority

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPriority)

#### `SetProfileFullName(fullName)`

Set the full name of the profile

**Remarks:** Set the full name of the profile

**Parameters:**
- `fullName` (str) — Path and name of the profile

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetProfileFullName)

#### `SetShapeType(shapeType)`

Set the type of the shape

**Remarks:** Set the type of the shape

**Parameters:**
- `shapeType` (ShapeType) — Shape type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetShapeType)

#### `SetShowAreaElementInGroundView(showInGroundView)`

Set the show area element in ground view state

**Remarks:** Set the show area element in ground view state

**Parameters:**
- `showInGroundView` (bool) — Show the area element in the ground view

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetShowAreaElementInGroundView)

#### `SetSurface(surface)`

Set the surface

**Remarks:** Set the surface

**Parameters:**
- `surface` (str) — Surface

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetSurface)

#### `SetTrade(value)`

Set the trade

**Remarks:** Set the trade

**Parameters:**
- `value` (int) — Trade index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetTrade)

#### `ToString(indent)`

Convert to string

**Remarks:** Convert to string

**Parameters:**
- `indent` (int) — Indent

**Returns:** `str` — String with tier properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.ToString)

### Properties
- `BackgroundColor` (int, get/set) — Get the background color
- `BitmapName` (str, get/set) — Get the name of the bitmap
- `CalculationMode` (int, get/set) — Get the calculation model
- `CircleDivision` (float, get/set) — Get the circle division
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `FaceStyle` (int, get/set) — Get the face style ID
- `Factor` (float, get/set) — Get the factor
- `Filling` (int, get/set) — Get the filling ID
- `Hatch` (int, get/set) — Get the hatch ID
- `Material` (str, get/set) — Get the material
- `Name` (str, get/set) — Get the name
- `Pattern` (int, get/set) — Get the pattern ID
- `PlaneReferences` (PlaneReferences, get/set) — Get the plane references
- `Priority` (int, get/set) — Get the priority
- `ProfileFullName` (str, get/set) — Get the full name of the profile
- `Surface` (str, get/set) — Get the surface name and path
- `Trade` (int, get/set) — Get the trade index

## ArchElement (class)

Implementation of the abstract base class ArchElement

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchElement/)

### Methods
#### `GetConnectionUUID()`

Get the connection UUID

**Remarks:** Get the connection UUID

**Returns:** `GUID` — Connection UUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchElement/#NemAll_Python_ArchElements.ArchElement.GetConnectionUUID)

#### `SetConnectionUUID(connectionGuid)`

Set the connection UUID

**Remarks:** Set the connection UUID

**Parameters:**
- `connectionGuid` (GUID) — Connection UUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchElement/#NemAll_Python_ArchElements.ArchElement.SetConnectionUUID)

### Properties
- `ConnectionUUID` (NemAll_Python_IFW_ElementAdapter.GUID, get/set) — Get the connection UUID

## AxisProperties (enum)

Axis properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AxisProperties/)

### Constructors
- `AxisProperties()` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AxisProperties/#NemAll_Python_ArchElements.AxisProperties.__repr__)

### Properties
- `Distance` (float, get/set) — Get the distance
- `Extension` (int, get/set) — Get the extension
- `Modus` (int, get/set) — Get the modus
- `OnLayer` (int, get/set) — Get the axis layer
- `Position` (int, get/set) — Get the axis position

## BasePlaneReferences (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.BasePlaneReferences.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BasePlaneReferences/)

### Constructors
- `BasePlaneReferences()` — Initialize

## BeamElement (class)

Implementation of the beam element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/)

### Constructors
- `BeamElement()` — Initialize
- `BeamElement(beamProp, axis)` — Constructor
- `BeamElement(element)` — Copy constructor

### Methods
#### `GetProperties()`

Get the beam properties

**Remarks:** Get the beam properties

**Returns:** `BeamProperties` — Beam properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.GetProperties)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.SetCommonProperties)

#### `SetProperties(beamProp)`

Set the beam properties

**Remarks:** Set the beam properties

**Parameters:**
- `beamProp` (BeamProperties) — Beam properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.SetProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.__repr__)

### Properties
- `Properties` (BeamProperties, get/set) — Get the beam properties

## BeamProperties (class)

Implementation of the beam properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/)

### Constructors
- `BeamProperties()` — Initialize
- `BeamProperties(beamProp)` — Copy constructor

### Methods
#### `GetWidth()`

Get the width

**Remarks:** Get the width

**Returns:** `float` — Width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.GetWidth)

#### `SetAttribute(attrib)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetAttribute)

#### `SetAxis(axis)`

Set the axis

**Remarks:** Set the axis

**Parameters:**
- `axis` (AxisProperties) — Axis data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetAxis)

#### `SetWidth(width)`

Set the width

**Remarks:** Set the width

**Parameters:**
- `width` (float) — Width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetWidth)

### Properties
- `IsStartNewJoinedBeamGroup` (bool, get/set) — Get the state for starting a new beam group All beams in a group are used for to create a continues join between the beams.
- `Width` (float, get/set) — Get the width

## BottomTopPlaneService (class)

Implementation of the bottom top plane service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/)

### Methods
#### `GetAbsoluteBottomElevation(refElement, doc, planeProp)`

Get the absolute elevation of the bottom plane

**Remarks:** Get the absolute elevation of the bottom plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `float` — Absolute elevation of the bottom plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetAbsoluteBottomElevation)

#### `GetAbsoluteTopElevation(refElement, doc, planeProp)`

Get the absolute elevation of the top plane

**Remarks:** Get the absolute elevation of the top plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `float` — Absolute elevation of the top plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetAbsoluteTopElevation)

#### `GetBottomReferencePlane(refElement, doc, planeProp)`

Get the bottom reference plane

**Remarks:** Get the bottom reference plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `Union[BRep3D, Polyhedron3D, Plane3D]` — Bottom reference plane as Plan3D, BRep3D or Polyhedron3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetBottomReferencePlane)

#### `GetDocumentDefaultPlanes(doc)`

Get the default bottom and top plane of the document

**Remarks:** Get the default bottom and top plane of the document

**Parameters:**
- `doc` (DocumentAdapter)

**Returns:** `tuple[Plane3D, Plane3D]` — Default bottom and top plane of the document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetDocumentDefaultPlanes)

#### `GetTopReferencePlane(refElement, doc, planeProp)`

Get the top reference plane

**Remarks:** Get the top reference plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `Union[BRep3D, Polyhedron3D, Plane3D]` — Bottom reference plane as Plan3D, BRep3D or Polyhedron3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetTopReferencePlane)

## CircularShape (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.CircularShape.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/CircularShape/)

### Constructors
- `CircularShape(arg2)` — Initialize

### Properties
- `Radius` (None, get) — Set/get the radius) :type: None

## ColumnElement (class)

Implementation of the Column element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/)

### Constructors
- `ColumnElement()` — Initialize
- `ColumnElement(columnProp, placementPoint)` — Constructor
- `ColumnElement(element)` — Copy constructor

### Methods
#### `GetProperties()`

Get the Column properties

**Remarks:** Get the Column properties

**Returns:** `ColumnProperties` — Column properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.GetProperties)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.SetCommonProperties)

#### `SetProperties(ColumnProp)`

Set the Column properties

**Remarks:** Set the Column properties

**Parameters:**
- `ColumnProp` (ColumnProperties) — Column properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.SetProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.__repr__)

### Properties
- `Properties` (ColumnProperties, get/set) — Get the Column properties

## ColumnProperties (class)

Implementation of the Column properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnProperties/)

### Constructors
- `ColumnProperties()` — Initialize
- `ColumnProperties(columnProp)` — Copy constructor

### Methods
#### `ToString()`

Convert to string

**Remarks:** Convert to string

**Returns:** `str` — String with tier properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnProperties/#NemAll_Python_ArchElements.ColumnProperties.ToString)

## CustomBoxPoint (enum)

Emplacement of gripping

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/CustomBoxPoint/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `CustomBoxPoint` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/CustomBoxPoint/#NemAll_Python_ArchElements.CustomBoxPoint.__getitem__)

### Values
- `Center` = `5`
- `CenterOfGravity` = `10`
- `LeftBottom` = `1`
- `LeftTop` = `4`
- `MiddleBottom` = `6`
- `MiddleLeft` = `9`
- `MiddleRight` = `7`
- `MiddleTop` = `8`
- `RightBottom` = `2`
- `RightTop` = `3`

## ElementConverter (class)

Element converter

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ElementConverter/)

### Constructors
- `ElementConverter()` — Initialize

### Methods
#### `ConvertToUDElement(elements)`

Create UD element from arbitrary 3D elements

**Remarks:** Create UD element from arbitrary 3D elements

**Parameters:**
- `elements` (BaseElementAdapterList) — Elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ElementConverter/#NemAll_Python_ArchElements.ElementConverter.ConvertToUDElement)

## PlaneReferences (class)

Implementation of the plane references

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/)

### Constructors
- `PlaneReferences(doc, refElement)` — Constructor
- `PlaneReferences(element)` — Copy constructor

### Methods
#### `GetAbsBottomElevation()`

Get the absolute bottom elevation

**Remarks:** Get the absolute bottom elevation

**Returns:** `float` — Absolute bottom elevation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetAbsBottomElevation)

#### `GetAbsTopElevation()`

Get the absolute top elevation

**Remarks:** Get the absolute top elevation

**Returns:** `float` — Absolute top elevation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetAbsTopElevation)

#### `GetBottomElevation()`

Get the bottom plane elevation

**Remarks:** Get the bottom plane elevation

**Returns:** `float` — Bottom plane elevation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomElevation)

#### `GetBottomPlaneDependency()`

Get the bottom plane dependency

**Remarks:** Get the bottom plane dependency

**Returns:** `PlaneReferenceDependency` — Bottom plane dependency

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomPlaneDependency)

#### `GetDocument()`

Get the document

**Remarks:** Get the document

**Returns:** `DocumentAdapter` — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetDocument)

#### `GetReferenceElement()`

Get the reference element for the plane

**Remarks:** Get the reference element for the plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetReferenceElement)

#### `GetTopElevation()`

Get the top plane elevation

**Remarks:** Get the top plane elevation

**Returns:** `float` — Top plane elevation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopElevation)

#### `GetTopPlaneDependency()`

Get the top plane dependency

**Remarks:** Get the top plane dependency

**Returns:** `PlaneReferenceDependency` — Top plane dependency

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopPlaneDependency)

#### `SetBottomElevation(elevation)`

Set the bottom plane elevation

**Remarks:** Set the bottom plane elevation

**Parameters:**
- `elevation` (float) — Elevation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomElevation)

#### `SetBottomPlaneDependency(dependency)`

Set the bottom plane dependency

**Remarks:** Set the bottom plane dependency

**Parameters:**
- `dependency` (PlaneReferenceDependency) — Dependency

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomPlaneDependency)

#### `SetBottomToBottom(planeRef)`

Set the bottom level to the bottom level of the source plane reference

**Remarks:** Set the bottom level to the bottom level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomToBottom)

#### `SetBottomToTop(planeRef)`

Set the bottom level to the top level of the source plane reference

**Remarks:** Set the bottom level to the top level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomToTop)

#### `SetDocument(doc)`

Set the document

**Remarks:** Set the document

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetDocument)

#### `SetReferenceElement(refElement)`

Set the reference element for the plane

**Remarks:** Set the reference element for the plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetReferenceElement)

#### `SetTopElevation(elevation)`

Set the top plane elevation

**Remarks:** Set the top plane elevation

**Parameters:**
- `elevation` (float) — Elevation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopElevation)

#### `SetTopPlaneDependency(dependency)`

Set the top plane dependency

**Remarks:** Set the top plane dependency

**Parameters:**
- `dependency` (PlaneReferenceDependency) — Dependency

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopPlaneDependency)

#### `SetTopToBottom(planeRef)`

Set the top level to the bottom level of the source plane reference

**Remarks:** Set the top level to the bottom level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopToBottom)

#### `SetTopToTop(planeRef)`

Set the top level to the top level of the source plane reference

**Remarks:** Set the top level to the top level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopToTop)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.__repr__)

### Properties
- `AdaptTopLevelToPlane` (int, get/set) — Get the top level to plane adaption
- `BottomElevation` (float, get/set) — Get the bottom plane elevation
- `Direction` (int, get/set) — Get the direction
- `Document` (NemAll_Python_IFW_ElementAdapter.DocumentAdapter, get/set) — Get the document
- `LowerPlane` (ReferencePlaneID, get/set) — Get the lower plane
- `MaxHeight` (float, get/set) — Get the maximal height
- `ReferenceElement` (NemAll_Python_IFW_ElementAdapter.BaseElementAdapter, get/set) — Get the reference element for the plane
- `TopElevation` (float, get/set) — Get the top plane elevation
- `UpperPlane` (ReferencePlaneID, get/set) — Get the upper plane

## ProfileCatalogService (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.ProfileCatalogService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/)

### Methods
#### `GetDoubleProfileGap(fullProfileName)`

Gets double profile gap

**Remarks:** Gets double profile gap

**Parameters:**
- `fullProfileName` (str) — Profile name with path

**Returns:** `float` — Double profile gap

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetDoubleProfileGap)

#### `GetFullProfileBoundaryPaths(fullProfileName, overrideDefaultGap=False, overrideGap=0.0)`

Get the boundary path of the full profile (e.g. in case of double profile).

**Remarks:** Get the boundary path of the full profile (e.g. in case of double profile).

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Path2DList` — Profile boundary paths

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetFullProfileBoundaryPaths)

#### `GetFullProfileBoundaryPolylines(fullProfileName, overrideDefaultGap=False, overrideGap=0.0)`

Get the boundary polylines of the full profile (e.g. in case of double profile).

**Remarks:** Get the boundary polylines of the full profile (e.g. in case of double profile).

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Polyline2DList` — Profile boundary polylines

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetFullProfileBoundaryPolylines)

#### `GetProfileAttributes(fullProfileName, doc)`

Get the profile attributes

**Remarks:** Get the profile attributes

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `doc` (DocumentAdapter) — Document

**Returns:** `list` — Attributes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileAttributes)

#### `GetProfileBoundaryPath(fullProfileName, overrideDefaultGap=False, overrideGap=0.0)`

Get the boundary path of the single profile

**Remarks:** Get the boundary path of the single profile

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Path2D` — Profile boundary path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileBoundaryPath)

#### `GetProfileBoundaryPolyline(fullProfileName, overrideDefaultGap=False, overrideGap=0.0)`

Get the boundary polyline of the single profile

**Remarks:** Get the boundary polyline of the single profile

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Polyline2D` — Profile boundary polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileBoundaryPolyline)

#### `GetProfileGeometry(fullProfileName, overrideDefaultGap=False, overrideGap=0.0)`

Get the profile geometry

**Remarks:** Get the profile geometry

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `BRep3D` — Profile geometry as BRep3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileGeometry)

#### `GetProfilePlacementPoint(fullProfileName, overrideDefaultGap=False, overrideGap=0.0)`

Get the profile placement point

**Remarks:** Get the profile placement point

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Point3D` — Profile placement point as Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfilePlacementPoint)

## ProfileShape (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.ProfileShape.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileShape/)

### Constructors
- `ProfileShape(arg2)` — Initialize

### Properties
- `ProfilePath` (None, get) — Set/get the profile path) :type: None

## PropertyDialogs (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.PropertyDialogs.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/)

### Methods
#### `GetLastSymbolPath()`

Get the last symbol path

**Remarks:** Get the last symbol path

**Returns:** `str` — Last symbol path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.GetLastSymbolPath)

#### `GetSymbolName(symbolPath)`

Get the symbol name

**Remarks:** Get the symbol name

**Parameters:**
- `symbolPath` (str) — Symbol path

**Returns:** `str` — Symbol name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.GetSymbolName)

#### `GetTradeDescription(tradeID)`

Get the trade description

**Remarks:** Get the trade description

**Parameters:**
- `tradeID` (int) — Trade ID

**Returns:** `str` — Trade description

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.GetTradeDescription)

#### `OpenBottomPlaneReferenceDialog(refElement, doc, planeRefs)`

Open the plane references dialog

**Remarks:** Open the plane references dialog

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeRefs` (PlaneReferences) — Plane references

**Returns:** `float` — bottom height

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenBottomPlaneReferenceDialog)

#### `OpenPlaneReferencesDialog(refElement, doc, planeRefs)`

Open the plane references dialog

**Remarks:** Open the plane references dialog

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeRefs` (PlaneReferences) — Plane references

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenPlaneReferencesDialog)

#### `OpenSymbolDialog(symbolPath)`

Open the symbol library dialog

**Remarks:** Open the symbol library dialog

**Parameters:**
- `symbolPath` (str) — Path to .sym file

**Returns:** `str` — result path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenSymbolDialog)

#### `OpenTopPlaneReferenceDialog(refElement, doc, planeRefs)`

Open the plane references dialog

**Remarks:** Open the plane references dialog

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeRefs` (PlaneReferences) — Plane references

**Returns:** `float` — top height

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenTopPlaneReferenceDialog)

#### `OpenTradeDialog(doc, tradeID)`

Open the trade dialog

**Remarks:** Open the trade dialog

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `tradeID` (int) — Current trade ID

**Returns:** `int` — Trade ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenTradeDialog)

## RectangularShape (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.RectangularShape.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RectangularShape/)

### Constructors
- `RectangularShape(arg2)` — Initialize

### Properties
- `Thickness` (None, get) — Set/get the thickness) :type: None
- `Width` (None, get) — Set/get the width) :type: None

## ReferencePlaneID (class)

struct for handling reference plane IDs

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/)

### Constructors
- `ReferencePlaneID()` — Initialize
- `ReferencePlaneID(modelGuid, planeId)` — Constructor
- `ReferencePlaneID(element)` — Copy constructor

### Methods
#### `ChangeModel(inputModelGuid)`

change model id to another

**Remarks:** change model id to another

**Parameters:**
- `inputModelGuid` (GUID)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.ChangeModel)

#### `Invalidate()`

Set values to undefined state

**Remarks:** Set values to undefined state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.Invalidate)

#### `IsDefaultLowerPlane()`

Find out if reference plane is default lower reference plane

**Remarks:** Find out if reference plane is default lower reference plane

**Returns:** `bool` — true if ref plane is default lower ref plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsDefaultLowerPlane)

#### `IsDefaultUpperPlane()`

Find out if reference plane is default upper reference plane

**Remarks:** Find out if reference plane is default upper reference plane

**Returns:** `bool` — true if ref plane is default upper ref plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsDefaultUpperPlane)

#### `IsDocumentRefSurface()`

Find out if reference surface is from document

**Remarks:** Find out if reference surface is from document

**Returns:** `bool` — true is reference surface from document (not in model)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsDocumentRefSurface)

#### `IsInModel()`

Find out if refrence plane is in model

**Remarks:** Find out if refrence plane is in model

**Returns:** `bool` — true if ref plane is in model

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsInModel)

#### `IsValid()`

Get validity of ref plane

**Remarks:** Get validity of ref plane

**Returns:** `bool` — true if ref plane is valid

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsValid)

#### `SetCustomLowerPlane()`

Set values to custom lower state

**Remarks:** Set values to custom lower state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.SetCustomLowerPlane)

#### `SetCustomUpperPlane()`

Set values to custom upper state

**Remarks:** Set values to custom upper state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.SetCustomUpperPlane)

#### `__eq__(other)`

Args: other:

**Remarks:** Args: other:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__eq__)

#### `__lt__(other)`

Args: other:

**Remarks:** Args: other:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__lt__)

#### `__ne__(other)`

Args: other:

**Remarks:** Args: other:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__ne__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__repr__)

### Properties
- `ModelGuid` (None, get) — model ID :type: None
- `PlaneId` (None, get) — plane ID :type: None

## RoomElement (class)

Implementation of the Room element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/)

### Constructors
- `RoomElement()` — Initialize
- `RoomElement(RoomProp, RoomPolygon)` — Constructor
- `RoomElement(element)` — Copy constructor

### Methods
#### `GetProperties()`

Get the Room properties

**Remarks:** Get the Room properties

**Returns:** `RoomProperties` — Room properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.GetProperties)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.SetCommonProperties)

#### `SetProperties(RoomProp)`

Set the Room properties

**Remarks:** Set the Room properties

**Parameters:**
- `RoomProp` (RoomProperties) — Room properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.SetProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.__repr__)

### Properties
- `Properties` (RoomProperties, get/set) — Get the Room properties

## RoomProperties (class)

Implementation of the Room properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/)

### Constructors
- `RoomProperties()` — Initialize
- `RoomProperties(RoomProp)` — Copy constructor

### Methods
#### `GetAttributes(doc, onlyModifiable)`

List with the attributes

**Remarks:** List with the attributes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetAttributes)

#### `GetFunction()`

Get the function

**Remarks:** Get the function

**Returns:** `str` — Function

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetFunction)

#### `GetStoreyCode()`

Get the storey code

**Remarks:** Get the storey code

**Returns:** `str` — Storey code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetStoreyCode)

#### `GetText(number)`

Get the text

**Remarks:** Get the text

**Parameters:**
- `number` (int) — Number (1-5 allowed)

**Returns:** `str` — Text

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetText)

#### `SetAttribute(attrib)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (Union[AttributeInteger, AttributeDouble, AttributeString, AttributeEnum]) — Attribute

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetAttribute)

#### `SetFunction(name)`

Set the function

**Remarks:** Set the function

**Parameters:**
- `name` (str) — Function

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetFunction)

#### `SetStoreyCode(storeyCode)`

Set the storey code

**Remarks:** Set the storey code

**Parameters:**
- `storeyCode` (str) — Storey code

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetStoreyCode)

#### `SetText(text, number)`

Set the text

**Remarks:** Set the text

**Parameters:**
- `text` (str) — Text
- `number` (int) — Number (1-5 allowed)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetText)

### Properties
- `Function` (str, get/set) — Get the function
- `StoreyCode` (str, get/set) — Get the storey code

## SlabElement (class)

Implementation of the Slab element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/)

### Constructors
- `SlabElement()` — Initialize
- `SlabElement(slabProp, slabPolygon)` — Constructor
- `SlabElement(element)` — Copy constructor

### Methods
#### `GetProperties()`

Get the Slab properties

**Remarks:** Get the Slab properties

**Returns:** `SlabProperties` — Slab properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.GetProperties)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.SetCommonProperties)

#### `SetProperties(SlabProp)`

Set the Slab properties

**Remarks:** Set the Slab properties

**Parameters:**
- `SlabProp` (SlabProperties) — Slab properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.SetProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.__repr__)

### Properties
- `Properties` (SlabProperties, get/set) — Get the Slab properties

## SlabOpeningElement (class)

Implementation of the slab opening element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/)

### Constructors
- `SlabOpeningElement()` — Initialize
- `SlabOpeningElement(slabOpeningProp, placementPoint, slabConnectionUUID)` — Constructor
- `SlabOpeningElement(element)` — Copy constructor

### Methods
#### `GetProperties()`

Get the slab opening properties

**Remarks:** Get the slab opening properties

**Returns:** `SlabOpeningProperties` — Slab opening properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.GetProperties)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.SetCommonProperties)

#### `SetProperties(slabOpeningProp)`

Set the SlabOpening properties

**Remarks:** Set the SlabOpening properties

**Parameters:**
- `slabOpeningProp` (SlabOpeningProperties) — Slab opening properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.SetProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.__repr__)

### Properties
- `Properties` (SlabOpeningProperties, get/set) — Get the slab opening properties

## SlabOpeningProperties (class)

Implementation of the slab opening properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningProperties/)

### Constructors
- `SlabOpeningProperties()` — Initialize
- `SlabOpeningProperties(SlabOpeningProperties)` — Copy constructor Args:

## SlabProperties (class)

Implementation of the Slab properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabProperties/)

### Constructors
- `SlabProperties()` — Initialize
- `SlabProperties(slabProp)` — Copy constructor

### Methods
#### `SetAttribute(attrib)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabProperties/#NemAll_Python_ArchElements.SlabProperties.SetAttribute)

## SolidElementTruncationType (enum)

Type of cutting/stretching

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SolidElementTruncationType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `SolidElementTruncationType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SolidElementTruncationType/#NemAll_Python_ArchElements.SolidElementTruncationType.__getitem__)

### Values
- `FreePlane` = `3`
- `Horizontal` = `1`
- `NormalToBodyAxis` = `0`
- `Vertical` = `2`

## StructuralBeamElement (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralBeamElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/)

### Constructors
- `StructuralBeamElement()` — Initialize
- `StructuralBeamElement(structuralBeamProperties)` — Constructor
- `StructuralBeamElement(structuralBeamElement)` — Copy constructor

### Methods
#### `GetStructuralBeamProperties()`

Get the StructuralBeam properties

**Remarks:** Get the StructuralBeam properties

**Returns:** `StructuralBeamProperties` — StructuralBeam properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.GetStructuralBeamProperties)

#### `SetAxisVisibility(value)`

Set the Visibility of element axis

**Remarks:** Set the Visibility of element axis

**Parameters:**
- `value` (bool) — Bool if axis of element should be visible

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.SetAxisVisibility)

#### `SetStructuralBeamProperties(structuralBeamProp)`

Set the StructuralBeam properties

**Remarks:** Set the StructuralBeam properties

**Parameters:**
- `structuralBeamProp` (StructuralBeamProperties) — StructuralBeam properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.SetStructuralBeamProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.__repr__)

## StructuralBeamProperties (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralBeamProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/)

### Constructors
- `StructuralBeamProperties()` — Initialize
- `StructuralBeamProperties(structuralBeamProps)` — Copy constructor

### Methods
#### `GetEndPoint()`

Get the end point of the element in world coordinate system

**Remarks:** Get the end point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.GetEndPoint)

#### `GetStartPoint()`

Get the start point of the element in world coordinate system

**Remarks:** Get the start point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.GetStartPoint)

#### `SetAnchorPointProperties(anchorPointStart, offsetStart, anchorPointEnd, offsetEnd, twoAnchorPoints)`

Set the Beam Anchor Points Properties

**Remarks:** Set the Beam Anchor Points Properties

**Parameters:**
- `Values` (for anchor point) — 1 - LeftBottom 2 - RightBottom 3 - RightTop 4 - LeftTop 5 - Center 6 - MiddleBottom 7 - MiddleRight 8 - MiddleTop 9 - MiddleLeft 10 - CenterOfGravity

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetAnchorPointProperties)

#### `SetEndPoint(x, y, z)`

Set the end point of the element in world coordinate system

**Remarks:** Set the end point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetEndPoint)

#### `SetHeightProperties(doc, planeReferences)`

Set HeightProperties

**Remarks:** Set HeightProperties

**Parameters:**
- `PlaneReferences` (object) — Height Properties of Column, as PlaneReferences type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetHeightProperties)

#### `SetHeightPropsByPlacementPoints(doc, val, val1, val2)`

Set HeightProperties

**Remarks:** Set HeightProperties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetHeightPropsByPlacementPoints)

#### `SetStartPoint(x, y, z)`

Set the start point of the element in world coordinate system

**Remarks:** Set the start point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetStartPoint)

## StructuralBraceElement (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralBraceElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/)

### Constructors
- `StructuralBraceElement()` — Initialize
- `StructuralBraceElement(structuralBraceProperties)` — Constructor
- `StructuralBraceElement(structuralBraceElement)` — Copy constructor

### Methods
#### `GetStructuralBraceProperties()`

Get the StructuralBrace properties

**Remarks:** Get the StructuralBrace properties

**Returns:** `StructuralBraceProperties` — StructuralBrace properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.GetStructuralBraceProperties)

#### `SetAxisVisibility(value)`

Set the Visibility of element axis

**Remarks:** Set the Visibility of element axis

**Parameters:**
- `value` (bool) — Bool if axis of element should be visible

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.SetAxisVisibility)

#### `SetStructuralBraceProperties(structuralBraceProp)`

Set the StructuralBrace properties

**Remarks:** Set the StructuralBrace properties

**Parameters:**
- `structuralBraceProp` (StructuralBraceProperties) — StructuralBrace properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.SetStructuralBraceProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.__repr__)

## StructuralBraceProperties (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralBraceProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/)

### Constructors
- `StructuralBraceProperties()` — Initialize
- `StructuralBraceProperties(structuralBraceProps)` — Copy constructor

### Methods
#### `GetEndPoint()`

Get the end point of the element in world coordinate system

**Remarks:** Get the end point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.GetEndPoint)

#### `GetStartPoint()`

Get the start point of the element in world coordinate system

**Remarks:** Get the start point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.GetStartPoint)

#### `SetAnchorPointProperties(val, point, val1, point1, bool)`

Set the Brace Anchor Points Properties

**Remarks:** Set the Brace Anchor Points Properties

**Parameters:**
- `val` (int) — Offset with Anchor Points Properties of Brace

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetAnchorPointProperties)

#### `SetEndPoint(x, y, z)`

Set the end point of the element in world coordinate system

**Remarks:** Set the end point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetEndPoint)

#### `SetHeightProperties(doc, planeReferences)`

Set HeightProperties

**Remarks:** Set HeightProperties

**Parameters:**
- `PlaneReferences` (object) — Height Properties of Column, as PlaneReferences type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetHeightProperties)

#### `SetHeightPropsByPlacementPoints(doc, val, val1, val2)`

Set HeightProperties

**Remarks:** Set HeightProperties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetHeightPropsByPlacementPoints)

#### `SetStartPoint(x, y, z)`

Set the start point of the element in world coordinate system

**Remarks:** Set the start point of the element in world coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetStartPoint)

## StructuralColumnElement (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralColumnElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/)

### Constructors
- `StructuralColumnElement()` — Initialize
- `StructuralColumnElement(structuralColumnProperties)` — Constructor
- `StructuralColumnElement(structuralColumnElement)` — Copy constructor

### Methods
#### `GetStructuralColumnProperties()`

Get the StructuralColumn properties

**Remarks:** Get the StructuralColumn properties

**Returns:** `StructuralColumnProperties` — StructuralColumn properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.GetStructuralColumnProperties)

#### `SetAxisVisibility(value)`

Set the Visibility of element axis

**Remarks:** Set the Visibility of element axis

**Parameters:**
- `value` (bool) — Bool if axis of element should be visible

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.SetAxisVisibility)

#### `SetStructuralColumnProperties(structuralColumnProp)`

Set the StructuralColumn properties

**Remarks:** Set the StructuralColumn properties

**Parameters:**
- `structuralColumnProp` (StructuralColumnProperties) — StructuralColumn properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.SetStructuralColumnProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.__repr__)

## StructuralColumnProperties (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralColumnProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/)

### Constructors
- `StructuralColumnProperties()` — Initialize
- `StructuralColumnProperties(structuralColumnProps)` — Copy constructor

### Methods
#### `GetHeight()`

Get the Column Height

**Remarks:** Get the Column Height

**Returns:** `float` — Height

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.GetHeight)

#### `GetPosition()`

Get the Column position in world coordinate system

**Remarks:** Get the Column position in world coordinate system

**Returns:** `Point3D` — Position

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.GetPosition)

#### `SetAnchorPointProperties(val, point)`

Set the Column Anchor Point Properties

**Remarks:** Set the Column Anchor Point Properties

**Parameters:**
- `val` (int) — Offset with Anchor Point Properties of Column

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetAnchorPointProperties)

#### `SetHeight(z, doc, val)`

Set Column Height

**Remarks:** Set Column Height

**Parameters:**
- `val` (PlaneReferences) — height of Column

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetHeight)

#### `SetHeightProperties(doc, planeReferences)`

Set the Column Height Properties

**Remarks:** Set the Column Height Properties

**Parameters:**
- `PlaneReferences` (object) — Height Properties of Column, as PlaneReferences type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetHeightProperties)

#### `SetHeightPropsByPlacementPoint(doc, val, val1)`

Set HeightProperties

**Remarks:** Set HeightProperties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetHeightPropsByPlacementPoint)

#### `SetPosition(x, y, z)`

Set the Column position in world coordinate system

**Remarks:** Set the Column position in world coordinate system

**Parameters:**
- `val` (object) — position x,y,z of Column

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetPosition)

## StructuralElementProperties (class)

(No description provided in vendor docs for NemAll_Python_ArchElements.StructuralElementProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/)

### Constructors
- `StructuralElementProperties()` — Initialize
- `StructuralElementProperties(structuralElementProperties)` — Copy constructor

### Methods
#### `AddHole(holeGeometry)`

Adds a hole to the list of holes for the Structural Element

**Remarks:** Adds a hole to the list of holes for the Structural Element

**Parameters:**
- `holeGeometry` (BRep3D) — 3D BRep Geometry of the hole

**Returns:** `int` — Unique ID for the created hole

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.AddHole)

#### `GetAnglesAtEnd()`

Returns angles in X, Y direction for the end of the Structural Element, as tuple(X, Y)

**Remarks:** Returns angles in X, Y direction for the end of the Structural Element, as tuple(X, Y)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetAnglesAtEnd)

#### `GetAnglesAtStart()`

Returns angles in X, Y direction for the start of the Structural Element, as tuple(X, Y)

**Remarks:** Returns angles in X, Y direction for the start of the Structural Element, as tuple(X, Y)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetAnglesAtStart)

#### `GetBodyAxis()`

Get Body Axis

**Remarks:** Get Body Axis

**Returns:** `Line3D` — Axis as Line3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetBodyAxis)

#### `GetCenterAxis()`

Get Center Axis as a Allplan::Geometry::Line3D

**Remarks:** Get Center Axis as a Allplan::Geometry::Line3D

**Returns:** `Allplan` — Line3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetCenterAxis)

#### `GetDoubleProfileGap()`

Gets double profile gap ProfileCatalogService calls

**Remarks:** Gets double profile gap ProfileCatalogService calls

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetDoubleProfileGap)

#### `GetEndPointAdditionalOffset()`

Get the end point distance dx/dy

**Remarks:** Get the end point distance dx/dy

**Returns:** `Vector3D` — Position as Vector3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetEndPointAdditionalOffset)

#### `GetEndPointZOffset()`

Get the value of Z offset of the anchor point at end the point of SF element

**Remarks:** Get the value of Z offset of the anchor point at end the point of SF element

**Returns:** `float` — double

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetEndPointZOffset)

#### `GetEndReferencePointType()`

Get type the reference point at end

**Remarks:** Get type the reference point at end

**Returns:** `CustomBoxPoint` — CustomBoxPoint

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetEndReferencePointType)

#### `GetHeightProperties()`

Get HeightProperties

**Remarks:** Get HeightProperties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetHeightProperties)

#### `GetHoles()`

Returns existing holes as python dictionary, where key is hole ID, value is its geometry

**Remarks:** Returns existing holes as python dictionary, where key is hole ID, value is its geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetHoles)

#### `GetProfileAngle()`

Get rotation angle of element profile.

**Remarks:** Get rotation angle of element profile.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetProfileAngle)

#### `GetProfileShapeProperties()`

Get ProfileShapeProperties

**Remarks:** Get ProfileShapeProperties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetProfileShapeProperties)

#### `GetShapeTypeAtEnd()`

Returns truncation type at the end of the Structural Element

**Remarks:** Returns truncation type at the end of the Structural Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetShapeTypeAtEnd)

#### `GetShapeTypeAtStart()`

Returns truncation type at the start of the Structural Element

**Remarks:** Returns truncation type at the start of the Structural Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetShapeTypeAtStart)

#### `GetSkeletonSolidElementProperties()`

Get SkeletonSolidElementProperties

**Remarks:** Get SkeletonSolidElementProperties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetSkeletonSolidElementProperties)

#### `GetStartPointAdditionalOffset()`

Get the start point distance dx/dy

**Remarks:** Get the start point distance dx/dy

**Returns:** `Vector3D` — Position as Vector3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetStartPointAdditionalOffset)

#### `GetStartPointZOffset()`

Get the value of Z offset of the anchor point at start the point of SF element

**Remarks:** Get the value of Z offset of the anchor point at start the point of SF element

**Returns:** `float` — double

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetStartPointZOffset)

#### `GetStartReferencePointType()`

Get type of the reference point at start

**Remarks:** Get type of the reference point at start

**Returns:** `CustomBoxPoint` — CustomBoxPoint

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetStartReferencePointType)

#### `GetZCoordsVisibility()`

Get visibility of Z coordinates of insertion point

**Remarks:** Get visibility of Z coordinates of insertion point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetZCoordsVisibility)

#### `HasTwoAnchorPoints()`

Get the check box value if element have two anchor points

**Remarks:** Get the check box value if element have two anchor points

**Returns:** `bool` — True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.HasTwoAnchorPoints)

#### `RemoveHole(holeID)`

Removes a hole to the list of holes for the Structural Element

**Remarks:** Removes a hole to the list of holes for the Structural Element

**Parameters:**
- `holeID` (int) — : ID of the hole to remove. This ID is obtained by AddHole operation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.RemoveHole)

#### `SetAnglesAtEnd(angleX, angleY)`

Sets angles in X, Y direction for the end of the Structural Element

**Remarks:** Sets angles in X, Y direction for the end of the Structural Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetAnglesAtEnd)

#### `SetAnglesAtStart(angleX, angleY)`

Sets angles in X, Y direction for the start of the Structural Element

**Remarks:** Sets angles in X, Y direction for the start of the Structural Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetAnglesAtStart)

#### `SetAttribute(attrib)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetAttribute)

#### `SetCommonProperties(comProp)`

Set the Common properties

**Remarks:** Set the Common properties

**Parameters:**
- `comProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetCommonProperties)

#### `SetDoubleProfileGap(desiredGap)`

Set the gap of double profile. Returns: true if attribute Gap was correctly set and exists / false if given profile doesn't have attribute Gap.

**Remarks:** Set the gap of double profile. Returns: true if attribute Gap was correctly set and exists / false if given profile doesn't have attribute Gap.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetDoubleProfileGap)

#### `SetEndPointZOffset(desiredOffset)`

Set the value of the Z offset of the anchor point at the end point of SF element to desired value.

**Remarks:** Set the value of the Z offset of the anchor point at the end point of SF element to desired value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetEndPointZOffset)

#### `SetProfileAngle(value)`

Set the rotation angle of element profile

**Remarks:** Set the rotation angle of element profile

**Parameters:**
- `value` (Angle) — rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetProfileAngle)

#### `SetProfileShapeProperties(value)`

Set the Column ProfileShape Properties

**Remarks:** Set the Column ProfileShape Properties

**Parameters:**
- `val` (object) — ProfileShape Properties of Column

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetProfileShapeProperties)

#### `SetShapeTypeAtEnd(endTruncationType)`

Set truncation type at the end of the Structural Element

**Remarks:** Set truncation type at the end of the Structural Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetShapeTypeAtEnd)

#### `SetShapeTypeAtStart(startTruncationType)`

Set truncation type at the start of the Structural Element

**Remarks:** Set truncation type at the start of the Structural Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetShapeTypeAtStart)

#### `SetStartPointZOffset(desiredOffset)`

Set the value of the Z offset of the anchor point at the start point of SF element to desired value.

**Remarks:** Set the value of the Z offset of the anchor point at the start point of SF element to desired value.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetStartPointZOffset)

#### `SetTwoAnchorPoints(areTwoAnchorPoint)`

Set the value if element have two anchor points

**Remarks:** Set the value if element have two anchor points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetTwoAnchorPoints)

## VerticalElementProperties (class)

Implementation of the vertical element properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/)

### Constructors
- `VerticalElementProperties()` — Initialize
- `VerticalElementProperties(elementProp)` — Copy constructor

### Methods
#### `GetAngle()`

Get the angle

**Remarks:** Get the angle

**Returns:** `Angle` — Get angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetAngle)

#### `GetDepth()`

Get the depth

**Remarks:** Get the depth

**Returns:** `float` — Depth

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetDepth)

#### `GetRadius()`

Get the radius

**Remarks:** Get the radius

**Returns:** `float` — Radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetRadius)

#### `GetShapePolygon()`

Get the shape polygon

**Remarks:** Get the shape polygon

**Returns:** `Polygon2D` — Shape polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetShapePolygon)

#### `GetWidth()`

Get the width

**Remarks:** Get the width

**Returns:** `float` — Width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetWidth)

#### `LoadFromFavoriteFile(doc)`

Load the properties from the favorite file

**Remarks:** Load the properties from the favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.LoadFromFavoriteFile)

#### `SetAngle(angle)`

Set the angle

**Remarks:** Set the angle

**Parameters:**
- `angle` (Angle) — Angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetAngle)

#### `SetAttribute(attrib)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetAttribute)

#### `SetCornerRadius(radius)`

Set the corner radius

**Remarks:** Set the corner radius

**Parameters:**
- `radius` (float) — Corner radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetCornerRadius)

#### `SetDepth(depth)`

Set the depth

**Remarks:** Set the depth

**Parameters:**
- `depth` (float) — Set the depth

**Returns:** `object` — Depth

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetDepth)

#### `SetRadius(radius)`

Set the radius

**Remarks:** Set the radius

**Parameters:**
- `radius` (float) — Radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetRadius)

#### `SetShapePolygon(shapePol)`

Set the shape polygon

**Remarks:** Set the shape polygon

**Parameters:**
- `shapePol` (Polygon2D) — Shape polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetShapePolygon)

#### `SetSize(width, depth)`

Set the size of the columns

**Remarks:** Set the size of the columns

**Parameters:**
- `width` (float) — Width
- `depth` (float) — Depth

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetSize)

#### `SetWidth(width)`

Set the width

**Remarks:** Set the width

**Parameters:**
- `width` (float) — Width

**Returns:** `object` — Width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetWidth)

### Properties
- `Angle` (NemAll_Python_Geometry.Angle, get/set) — Get the angle
- `Depth` (float, get/set) — Get the depth
- `Radius` (float, get/set) — Get the radius
- `ShapePolygon` (NemAll_Python_Geometry.Polygon2D, get/set) — Get the shape polygon
- `Width` (float, get/set) — Get the width

## WallElement (class)

Implementation of the wall element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/)

### Constructors
- `WallElement()` — Initialize
- `WallElement(wallProp, axis)` — Constructor
- `WallElement(element)` — Copy constructor

### Methods
#### `GetProperties()`

Get the wall properties

**Remarks:** Get the wall properties

**Returns:** `WallProperties` — Wall properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.GetProperties)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.SetCommonProperties)

#### `SetProperties(wallProp)`

Set the wall properties

**Remarks:** Set the wall properties

**Parameters:**
- `wallProp` (WallProperties) — Wall properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.SetProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.__repr__)

### Properties
- `Properties` (WallProperties, get/set) — Get the wall properties

## WallProperties (class)

Implementation of the wall properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/)

### Constructors
- `WallProperties()` — Initialize
- `WallProperties(wallProp)` — Copy constructor

### Methods
#### `GetAxis()`

Get the axis properties

**Remarks:** Get the axis properties

**Returns:** `AxisProperties` — Axis properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetAxis)

#### `GetPathElementAxisType()`

Get the axis type of the path element

**Remarks:** Get the axis type of the path element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetPathElementAxisType)

#### `GetTierCount()`

Get the tier count

**Remarks:** Get the tier count

**Returns:** `int` — Tier count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetTierCount)

#### `GetWallTierProperties(tierIndex)`

Get the wall tier properties

**Remarks:** Get the wall tier properties

**Parameters:**
- `tierIndex` (int) — Tier index

**Returns:** `WallTierProperties` — Wall tier properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetWallTierProperties)

#### `LoadFromFavoriteFile(doc)`

Load the properties from the favorite file

**Remarks:** Load the properties from the favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.LoadFromFavoriteFile)

#### `SetAxis(axis)`

Set the axis

**Remarks:** Set the axis

**Parameters:**
- `axis` (AxisProperties) — Axis properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.SetAxis)

#### `SetPathElementAxisType(pathEleAxisType)`

Set the axis type of the path element

**Remarks:** Set the axis type of the path element

**Parameters:**
- `pathEleAxisType` (int) — Axis type of the path element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.SetPathElementAxisType)

#### `SetTierCount(tierCount)`

Set the tier count

**Remarks:** Set the tier count

**Parameters:**
- `tierCount` (int) — Tier count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.SetTierCount)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.__repr__)

### Properties
- `Axis` (AchseDataT, get/set) — Get the axis properties
- `IsStartNewJoinedWallGroup` (bool, get/set) — Get the state for starting a new wall group All walls in a group are used for to create a continues join between the walls.
- `PathElementAxisType` (int, get/set) — Get the axis type of the path element
- `TierCount` (int, get/set) — Get the tier count

## WallTierProperties (class)

Implementation of the wall tier properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/)

### Constructors
- `WallTierProperties(tierProp)` — Copy constructor

### Methods
#### `GetThickness()`

Get the thickness

**Remarks:** Get the thickness

**Returns:** `float` — Thickness

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/#NemAll_Python_ArchElements.WallTierProperties.GetThickness)

#### `SetThickness(thickness)`

Set the thickness

**Remarks:** Set the thickness

**Parameters:**
- `thickness` (float) — Thickness

**Returns:** `object` — missing !!!

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/#NemAll_Python_ArchElements.WallTierProperties.SetThickness)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/#NemAll_Python_ArchElements.WallTierProperties.__repr__)

### Properties
- `Thickness` (float, get/set) — Get the thickness

