---
name: allplan-nemall_python_archelements
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_ArchElements namespace — 66 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ArchElement, Functions, AllplanElement, ArchBaseProperties, BasePlaneReferences, ArchitectureElementsGeometryService, AxisProperties, BeamElement, and 58 more types.
---

# NemAll_Python_ArchElements

Auto-generated from vendor docs for allplan 2025.0. 66 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/)

### Methods
#### `GetAttributes() -> object`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetAttributes)

#### `GetBaseElementAdapter() -> BaseElementAdapter`

Get the model element

**Remarks:** Get the model element

**Returns:** `BaseElementAdapter` — Model element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetBaseElementAdapter)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetCommonProperties)

#### `GetGeometryObject() -> object`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetGeometryObject)

#### `GetLabelElements() -> list`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetLabelElements)

#### `GetSubElementID() -> type`

Get the SubElementID

**Remarks:** Get the SubElementID

**Returns:** `type` — SubElementID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer: object)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey: str)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject: object)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements: list)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AllplanElement/#NemAll_Python_ArchElements.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## ArchBaseProperties (class)

Base class representing properties of all kinds of architectural components

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/)

### Constructors
- `ArchBaseProperties(element: ArchBaseProperties)` — Copy constructor

### Methods
#### `GetAreaPresentationID() -> int`

Get the ID of the area representation

**Remarks:** Get the ID of the area representation

**Returns:** `int` — ID of the area representation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetAreaPresentationID)

#### `GetAreaPresentationType() -> int`

Get the type of the area representation

**Remarks:** Get the type of the area representation

**Returns:** `int` — Type of the area representation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetAreaPresentationType)

#### `GetBackgroundColor() -> int`

Get the background color

**Remarks:** Get the background color

**Returns:** `int` — Background color

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetBackgroundColor)

#### `GetBitmapName() -> str`

Get the name of the bitmap

**Remarks:** Get the name of the bitmap

**Returns:** `str` — Name of the bitmap

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetBitmapName)

#### `GetCalculationMode() -> int`

Get the calculation mode

**Remarks:** Get the calculation mode

**Returns:** `int` — Integer representing the calculation mode as follows: 0: m3 1: m2 2: m 3: Pcs 4: kg

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetCalculationMode)

#### `GetCircleDivision() -> int`

Get the circle division (relevant only for circular profiles))

**Remarks:** Get the circle division (relevant only for circular profiles))

**Returns:** `int` — Number of circle segments

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetCircleDivision)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetCommonProperties)

#### `GetFaceStyle() -> int`

Get the face style ID

**Remarks:** Get the face style ID

**Returns:** `int` — Face style ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetFaceStyle)

#### `GetFactor() -> float`

Get the factor

**Remarks:** Get the factor

**Returns:** `float` — Factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetFactor)

#### `GetFilling() -> int`

Get the filling ID

**Remarks:** Get the filling ID

**Returns:** `int` — Filling ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetFilling)

#### `GetHatch() -> int`

Get the hatch ID

**Remarks:** Get the hatch ID

**Returns:** `int` — Hatch ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetHatch)

#### `GetMaterial() -> str`

Get the material

**Remarks:** Get the material

**Returns:** `str` — Material

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetMaterial)

#### `GetName() -> str`

Get the name

**Remarks:** Get the name

**Returns:** `str` — Name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetName)

#### `GetPattern() -> int`

Get the pattern ID

**Remarks:** Get the pattern ID

**Returns:** `int` — Pattern ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetPattern)

#### `GetPlaneReferences() -> PlaneReferences`

Get the plane references

**Remarks:** Get the plane references

**Returns:** `PlaneReferences` — Plane references

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetPlaneReferences)

#### `GetPriority() -> int`

Get the priority

**Remarks:** Get the priority

**Returns:** `int` — Priority

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetPriority)

#### `GetStatus() -> int`

Get the Status ID

**Remarks:** Get the Status ID

**Returns:** `int` — Status ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetStatus)

#### `GetSurface() -> str`

Get the surface name and path

**Remarks:** Get the surface name and path

**Returns:** `str` — Surface name and path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetSurface)

#### `GetTrade() -> int`

Get the trade index

**Remarks:** Get the trade index

**Returns:** `int` — Trade index according to the definition of the enumeration attribute trade (@209@)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.GetTrade)

#### `IsShowAreaElementInGroundView() -> bool`

Get the 'Show the area element in the ground view' state

**Remarks:** Get the 'Show the area element in the ground view' state

**Returns:** `bool` — Show the area element in the ground view: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.IsShowAreaElementInGroundView)

#### `LoadFromFavoriteFile(doc: DocumentAdapter)`

Load the properties from the favorite file

**Remarks:** Load the properties from the favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.LoadFromFavoriteFile)

#### `RemoveCommonProperties()`

Remove the common properties

**Remarks:** Remove the common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.RemoveCommonProperties)

#### `ResetAreaElement()`

Reset the area element

**Remarks:** Reset the area element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.ResetAreaElement)

#### `ResetBackgroundColor()`

Reset the background color

**Remarks:** Reset the background color

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.ResetBackgroundColor)

#### `SetBackgroundColor(colorID: int)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `colorID` (int) — Background color ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetBackgroundColor)

#### `SetBitmapName(bitmapName: str)`

Set the name of the bitmap

**Remarks:** Set the name of the bitmap

**Parameters:**
- `bitmapName` (str) — Bitmap name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetBitmapName)

#### `SetCalculationMode(calculationMode: int)`

Set the calculation mode

**Remarks:** Set the calculation mode

**Parameters:**
- `calculationMode` (int) — Calculation mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetCalculationMode)

#### `SetCircleDivision(circleDivision: int)`

Set the circle division

**Remarks:** Set the circle division

**Parameters:**
- `circleDivision` (int) — Circle division

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetCircleDivision)

#### `SetCommonProperties(comProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `comProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetCommonProperties)

#### `SetFaceStyle(faceStyleID: int)`

Set the face style

**Remarks:** Set the face style

**Parameters:**
- `faceStyleID` (int) — Face style ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetFaceStyle)

#### `SetFactor(factor: float)`

Set the factor

**Remarks:** Set the factor

**Parameters:**
- `factor` (float) — Factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetFactor)

#### `SetFilling(fillingID: int)`

Set the filling

**Remarks:** Set the filling

**Parameters:**
- `fillingID` (int) — Filling ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetFilling)

#### `SetHatch(hatchID: int)`

Set the hatch

**Remarks:** Set the hatch

**Parameters:**
- `hatchID` (int) — Hatch ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetHatch)

#### `SetMaterial(material: str)`

Set the material

**Remarks:** Set the material

**Parameters:**
- `material` (str) — Material

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetMaterial)

#### `SetName(name: str)`

Set the name

**Remarks:** Set the name

**Parameters:**
- `name` (str) — Name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetName)

#### `SetPattern(patternID: int)`

Set the pattern

**Remarks:** Set the pattern

**Parameters:**
- `patternID` (int) — Pattern ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPattern)

#### `SetPlaneReferences(planeRef: PlaneReferences)`

Set the plane references

**Remarks:** Set the plane references

**Parameters:**
- `planeRef` (PlaneReferences) — Plane references

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPlaneReferences)

#### `SetPriority(priority: int)`

Set the priority

**Remarks:** Set the priority

**Parameters:**
- `priority` (int) — Priority

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetPriority)

#### `SetShowAreaElementInGroundView(showInGroundView: bool)`

Set the show area element in ground view state

**Remarks:** Set the show area element in ground view state

**Parameters:**
- `showInGroundView` (bool) — Show the area element in the ground view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetShowAreaElementInGroundView)

#### `SetStatus(statusID: int)`

Set the Status ID

**Remarks:** Set the Status ID

**Parameters:**
- `statusID` (int) — Status ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetStatus)

#### `SetSurface(surface: str)`

Set the surface

**Remarks:** Set the surface

**Parameters:**
- `surface` (str) — Surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetSurface)

#### `SetTrade(value: int)`

Set the trade index

**Remarks:** Set the trade index

**Parameters:**
- `value` (int) — Trade index according to the definition of the enumeration attribute trade (@209@)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchBaseProperties/#NemAll_Python_ArchElements.ArchBaseProperties.SetTrade)

### Properties
- `BackgroundColor` (int, get/set) — Get the background color
- `BitmapName` (str, get/set) — Get the name of the bitmap
- `CalculationMode` (int, get/set) — Calculation mode, represented by an integer as follows:
- `CircleDivision` (int, get/set) — Number the circle segments in case of a circular cross section
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `FaceStyle` (int, get/set) — Get the face style ID
- `Factor` (float, get/set) — Get the factor
- `Filling` (int, get/set) — Get the filling ID
- `Hatch` (int, get/set) — Get the hatch ID
- `Material` (str, get/set) — Get the material
- `Name` (str, get/set) — Get the name
- `Pattern` (int, get/set) — Get the pattern ID
- `PlaneReferences` (PlaneReferences, get/set) — Get the plane references
- `Priority` (int, get/set) — Get the priority
- `Status` (int, get/set) — Get the Status ID
- `Surface` (str, get/set) — Get the surface name and path
- `Trade` (int, get/set) — Trade index according to the definition of the enumeration attribute trade (@209@)

## ArchElement (class)

Abstract base class representing all architectural components

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchElement/)

### Methods
#### `GetConnectionUUID() -> GUID`

Get the connection UUID

**Remarks:** Get the connection UUID

**Returns:** `GUID` — Connection UUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchElement/#NemAll_Python_ArchElements.ArchElement.GetConnectionUUID)

#### `SetConnectionUUID(connectionGuid: GUID)`

Set the connection UUID

**Remarks:** Set the connection UUID

**Parameters:**
- `connectionGuid` (GUID) — Connection UUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchElement/#NemAll_Python_ArchElements.ArchElement.SetConnectionUUID)

### Properties
- `ConnectionUUID` (GUID, get/set) — Get the connection UUID

## ArchitectureElementsGeometryService (class)

Implementation of the architecture elements geometry service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchitectureElementsGeometryService/)

### Methods
#### `CreatePlacementArc( element: BaseElementAdapter, elementAxis: Arc2D, placementPnt: Point2D, directionPnt: Point2D, ) -> tuple[Arc2D, bool]`

Create a placement arc at the placement point

**Remarks:** Create a placement arc at the placement point

**Parameters:**
- `element` (BaseElementAdapter) — Element
- `elementAxis` (Arc2D) — Element axis
- `placementPnt` (Point2D) — Placement point
- `directionPnt` (Point2D) — Direction point

**Returns:** `tuple[Arc2D, bool]` — Placement arc, placement a bottom/top (true/false)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchitectureElementsGeometryService/#NemAll_Python_ArchElements.ArchitectureElementsGeometryService.CreatePlacementArc)

#### `GetOpeningOffsetPoints( parentEle: BaseElementAdapter, placementPnt: Point2D ) -> tuple[bool, Point2D, Point2D]`

Get the offset points for the opening

**Remarks:** Get the offset points for the opening

**Parameters:**
- `parentEle` (BaseElementAdapter) — Parent element of the opening
- `placementPnt` (Point2D) — Placement point

**Returns:** `tuple[bool, Point2D, Point2D]` — Offset point for the opening

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchitectureElementsGeometryService/#NemAll_Python_ArchElements.ArchitectureElementsGeometryService.GetOpeningOffsetPoints)

#### `GetOuterPolyline( elementPolygon: Polygon2D, refLine: Line2D, elementAxis: object ) -> Polyline2D`

Get the outer polyline from a curved element located at the reference line

**Remarks:** Get the outer polyline from a curved element located at the reference line

**Parameters:**
- `elementPolygon` (Polygon2D) — Polygon of the curved element
- `refLine` (Line2D) — Reference line of the polyline
- `elementAxis` (object) — Axis of the element

**Returns:** `Polyline2D` — Outer polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchitectureElementsGeometryService/#NemAll_Python_ArchElements.ArchitectureElementsGeometryService.GetOuterPolyline)

#### `GetOutlineSegmentAndPoint( element: BaseElementAdapter, refPoint: Point2D ) -> tuple[Line2D, Point2D, BaseElementAdapter]`

Get the outline segment and point related to the reference point

**Remarks:** Get the outline segment and point related to the reference point

**Parameters:**
- `element` (BaseElementAdapter) — Element
- `refPoint` (Point2D) — Reference point

**Returns:** `tuple[Line2D, Point2D, BaseElementAdapter]` — Outline segment, point and element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ArchitectureElementsGeometryService/#NemAll_Python_ArchElements.ArchitectureElementsGeometryService.GetOutlineSegmentAndPoint)

## AxisProperties (class)

Properties of the axis of a linear architectural component (beam, wall, etc.).

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AxisProperties/)

### Constructors
- `AxisProperties()` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/AxisProperties/#NemAll_Python_ArchElements.AxisProperties.__repr__)

### Properties
- `Distance` (float, get/set) — Offset distance between the axis and the outer side of the architectural component. IMPORTANT: The value must be set in the range between <0.0 , component_width>
- `Extension` (int, get/set) — Defines, on which side of the axis to generate the architectural component (or its tiers). Assuming the axis origins in (0,0) and points in X+ direction, setting the value to: IMPORTANT: Default value is 0 and must be changed to either 1 or -1!
- `Modus` (int, get/set) — Get the modus
- `OnTier` (int, get/set) — Get the axis tier
- `Position` (int, get/set) — Position of the axis relative to the architectural component. IMPORTANT: When setting this property, remember to set the Distance accordingly! Otherwise, the architectural component may behave unexpectedly when being modified.

## BasePlaneReferences (class)

Base class for the plane references

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BasePlaneReferences/)

### Constructors
- `BasePlaneReferences()` — Initialize

## BeamElement (class)

Representation of the upstand/downstand beam

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/)

### Constructors
- `BeamElement() | BeamElement(beamProp: BeamProperties, axis: object) | BeamElement(element: BeamElement)` — Initialize

### Methods
#### `GetProperties() -> BeamProperties`

Get the beam properties

**Remarks:** Get the beam properties

**Returns:** `BeamProperties` — Beam properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.GetProperties)

#### `SetProperties(beamProp: BeamProperties)`

Set the beam properties

**Remarks:** Set the beam properties

**Parameters:**
- `beamProp` (BeamProperties) — Beam properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamElement/#NemAll_Python_ArchElements.BeamElement.__repr__)

### Properties
- `Properties` (BeamProperties, get/set) — Properties of the beam

## BeamProperties (class)

Representation of the properties of an upstand/downstand beam

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/)

### Constructors
- `BeamProperties() | BeamProperties(beamProp: BeamProperties)` — Initialize

### Methods
#### `GetShapeType() -> ShapeType`

Get the type of the cross-section shape

**Remarks:** Get the type of the cross-section shape

**Returns:** `ShapeType` — Shape type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.GetShapeType)

#### `GetWidth() -> float`

Get the width of the rectangular cross-section shape

**Remarks:** Get the width of the rectangular cross-section shape

**Returns:** `float` — Width

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.GetWidth)

#### `SetAttribute(attrib: object)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetAttribute)

#### `SetAxis(axis: AxisProperties)`

Set the axis

**Remarks:** Set the axis

**Parameters:**
- `axis` (AxisProperties) — Axis data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetAxis)

#### `SetShapeType(shapeType: ShapeType)`

Set the type of the cross-section shape

**Remarks:** Set the type of the cross-section shape

**Parameters:**
- `shapeType` (ShapeType) — Shape type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetShapeType)

#### `SetWidth(width: float)`

Set the width of the rectangular cross-section shape

**Remarks:** Set the width of the rectangular cross-section shape

**Parameters:**
- `width` (float) — Width

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BeamProperties/#NemAll_Python_ArchElements.BeamProperties.SetWidth)

### Properties
- `IsStartNewJoinedBeamGroup` (bool, get/set) — Should this beam be the first of a new group of beams joined together Set this property to False in the second or subsequent beams in order to join all of them with each other at their ends to create a continous chain of beams.
- `ShapeType` (ShapeType, get/set) — Type of the cross-section shape
- `Width` (float, get/set) — Width of the rectangular cross-section shape

## BottomTopPlaneService (class)

Service providing methods for reading various properties of the reference planes set in the current document.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/)

### Methods
#### `GetAbsoluteBottomElevation( refElement: BaseElementAdapter, doc: DocumentAdapter, planeProp: BasePlaneReferences, ) -> float`

Get the absolute elevation of the bottom plane

**Remarks:** Get the absolute elevation of the bottom plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `float` — Absolute elevation of the bottom plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetAbsoluteBottomElevation)

#### `GetAbsoluteTopElevation( refElement: BaseElementAdapter, doc: DocumentAdapter, planeProp: BasePlaneReferences, ) -> float`

Get the absolute elevation of the top plane

**Remarks:** Get the absolute elevation of the top plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `float` — Absolute elevation of the top plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetAbsoluteTopElevation)

#### `GetBottomReferencePlane( refElement: BaseElementAdapter, doc: DocumentAdapter, planeProp: BasePlaneReferences, ) -> BRep3D | Polyhedron3D | Plane3D`

Get the bottom reference plane

**Remarks:** Get the bottom reference plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `BRep3D | Polyhedron3D | Plane3D` — Bottom reference plane as Plan3D, BRep3D or Polyhedron3D

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetBottomReferencePlane)

#### `GetDocumentBottomElevation( refElement: BaseElementAdapter, doc: DocumentAdapter, planeProp: BasePlaneReferences, ) -> float`

Get the document elevation of the bottom plane

**Remarks:** Get the document elevation of the bottom plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `float` — Absolute elevation of the bottom plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetDocumentBottomElevation)

#### `GetDocumentDefaultPlanes(doc: DocumentAdapter) -> tuple[Plane3D, Plane3D]`

Get the default bottom and top plane of the document

**Remarks:** Get the default bottom and top plane of the document

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `Plane3D` — Default bottom plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetDocumentDefaultPlanes)

#### `GetDocumentTopElevation( refElement: BaseElementAdapter, doc: DocumentAdapter, planeProp: BasePlaneReferences, ) -> float`

Get the document elevation of the top plane

**Remarks:** Get the document elevation of the top plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `float` — Absolute elevation of the top plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetDocumentTopElevation)

#### `GetTopReferencePlane( refElement: BaseElementAdapter, doc: DocumentAdapter, planeProp: BasePlaneReferences, ) -> BRep3D | Polyhedron3D | Plane3D`

Get the top reference plane

**Remarks:** Get the top reference plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeProp` (BasePlaneReferences) — Plane properties

**Returns:** `BRep3D | Polyhedron3D | Plane3D` — Bottom reference plane as Plan3D, BRep3D or Polyhedron3D

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/BottomTopPlaneService/#NemAll_Python_ArchElements.BottomTopPlaneService.GetTopReferencePlane)

## CircularShape (class)

Representation of a circular cross section of a structural framing element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/CircularShape/)

### Constructors
- `CircularShape() | CircularShape(value: CircularShape)` — Initialize

### Properties
- `Radius` (None, get) — Radius of the cross section

## ColumnElement (class)

Representation of the architectural column

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/)

### Constructors
- `ColumnElement() | ColumnElement(columnProp: ColumnProperties, placementPoint: object) | ColumnElement(element: ColumnElement)` — Initialize

### Methods
#### `GetProperties() -> ColumnProperties`

Get the Column properties

**Remarks:** Get the Column properties

**Returns:** `ColumnProperties` — Column properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.GetProperties)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.SetCommonProperties)

#### `SetProperties(ColumnProp: ColumnProperties)`

Set the Column properties

**Remarks:** Set the Column properties

**Parameters:**
- `ColumnProp` (ColumnProperties) — Column properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnElement/#NemAll_Python_ArchElements.ColumnElement.__repr__)

### Properties
- `Properties` (ColumnProperties, get/set) — Get the Column properties

## ColumnProperties (class)

Implementation of the Column properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ColumnProperties/)

### Constructors
- `ColumnProperties() | ColumnProperties(columnProp: ColumnProperties)` — Initialize

## CustomBoxPoint (enum)

Enumeration of possible anchor points of structural framing elements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/CustomBoxPoint/)

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

## DoorOpeningElement (class)

Representation of door opening

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningElement/)

### Constructors
- `DoorOpeningElement() | DoorOpeningElement( openingProp: DoorOpeningProperties, generalEle: BaseElementAdapter, startPnt: Point2D, endPnt: Point2D, drawPlacementPreview: bool, ) | DoorOpeningElement(element: DoorOpeningElement)` — Initialize

### Methods
#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningElement/#NemAll_Python_ArchElements.DoorOpeningElement.__repr__)

### Properties
- `Properties` (DoorOpeningProperties, get/set) — Door opening properties

## DoorOpeningProperties (class)

Properties of door opening

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/)

### Constructors
- `DoorOpeningProperties() | DoorOpeningProperties(doorProp: DoorOpeningProperties)` — Initialize

### Methods
#### `GetDoorSwingProperties() -> DoorSwingProperties`

Get a reference of the door swing properties

**Remarks:** Get a reference of the door swing properties

**Returns:** `DoorSwingProperties` — Door swing properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/#NemAll_Python_ArchElements.DoorOpeningProperties.GetDoorSwingProperties)

#### `GetGeometryProperties() -> VerticalOpeningGeometryProperties`

Get a reference of the geometry properties

**Remarks:** Get a reference of the geometry properties

**Returns:** `VerticalOpeningGeometryProperties` — Geometry properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/#NemAll_Python_ArchElements.DoorOpeningProperties.GetGeometryProperties)

#### `GetOpeningSymbolsProperties() -> OpeningSymbolsProperties`

Get a reference of the opening symbols properties

**Remarks:** Get a reference of the opening symbols properties

**Returns:** `OpeningSymbolsProperties` — Opening symbols properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/#NemAll_Python_ArchElements.DoorOpeningProperties.GetOpeningSymbolsProperties)

#### `GetRevealProperties() -> VerticalOpeningRevealProperties`

Get a reference of the reveal properties

**Remarks:** Get a reference of the reveal properties

**Returns:** `VerticalOpeningRevealProperties` — Reveal properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/#NemAll_Python_ArchElements.DoorOpeningProperties.GetRevealProperties)

#### `GetSillProperties() -> VerticalOpeningSillProperties`

Get a reference of the sill properties

**Remarks:** Get a reference of the sill properties

**Returns:** `VerticalOpeningSillProperties` — Sill properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/#NemAll_Python_ArchElements.DoorOpeningProperties.GetSillProperties)

#### `GetTierOffsetProperties() -> VerticalOpeningTierOffsetProperties`

Get a reference of the tier offset properties

**Remarks:** Get a reference of the tier offset properties

**Returns:** `VerticalOpeningTierOffsetProperties` — Tier offset properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorOpeningProperties/#NemAll_Python_ArchElements.DoorOpeningProperties.GetTierOffsetProperties)

### Properties
- `FrenchDoor` (bool, get/set) — Get the french door attribute state
- `Independent2DInteraction` (bool, get/set) — Set to True, when the opening is above/below the section plane and should therefor NOT interact with the hatching/filling of the parent architectural element in ground view. Defaults to False.
- `PlaneReferences` (PlaneReferences, get/set) — Get the plane references

## DoorSwingProperties (class)

Implementation of the door swing properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorSwingProperties/)

### Constructors
- `DoorSwingProperties(element: DoorSwingProperties)` — Copy constructor

### Properties
- `Angle` (float, get/set) — Get the Angle
- `BasePointIndex` (int, get/set) — Get the base point index
- `LeafThickness` (float, get/set) — Get the leaf thickness
- `Type` (DoorSwingType, get/set) — Get the swing type

## DoorSwingType (enum)

Type of the door swing

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/DoorSwingType/)

### Values
- `eBiFold` = `19`
- `eDoubleOppositeSwingCircular` = `5` — Revolving door, two leaves, opposite (arc)
- `eDoubleOppositeSwingLinear` = `6` — Revolving door, two leaves, opposite (diagonal)
- `eDoubleSwingCircular` = `3` — Revolving door, two leaves (arc)
- `eDoubleSwingLinear` = `4` — Revolving door, two leaves (diagonal)
- `eFolding` = `18` — Folding door
- `eLifting` = `27` — Lifting door
- `eLiftingSingleSwingCircular` = `11` — Lifting revolving door (arc)
- `eLiftingSingleSwingLinear` = `12` — Lifting revolving door (diagonal)
- `eLiftingSliding` = `16` — Lift and slide door
- `eNone` = `0` — None
- `eOneSidedDoubleRevolving` = `23` — Double revolving door, single-sided
- `eOneSidedRevolving` = `21` — Revolving door, single-sided
- `eOneSidedSwingOptional` = `25` — Up and over door, single-sided (optional)
- `ePendulumDoubleSwingCircular` = `9` — Swing door, two leaves (arc)
- `ePendulumDoubleSwingLinear` = `10` — Swing door, two leaves (diagonal)
- `ePendulumSingleSwingCircular` = `7` — Swing door, one leaf (arc)
- `ePendulumSingleSwingLinear` = `8` — Swing door, one leaf (diagonal)
- `eRevolving` = `17` — Turnstile door
- `eSingleSwingCircular` = `1` — Revolving door, one leaf (arc)
- `eSingleSwingLinear` = `2` — Revolving door, one leaf (diagonal)
- `eSliding` = `15`
- `eSlidingDoubleSwing` = `14` — Sliding door, two leaves
- `eSlidingSingleSwing` = `13` — Sliding door, one leaf
- `eSwing` = `20` — Up and over door
- `eTwoSidedDoubleRevolving` = `24` — Double revolving door, double-sided
- `eTwoSidedFolding` = `28` — Folding door double-sided
- `eTwoSidedRevolving` = `22` — Revolving door, double-sided
- `eTwoSidedSwingOptional` = `26` — Up and over door, double-sided (optional)

## ElementConverter (class)

Utility providing methods for conversion of architectural components

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ElementConverter/)

### Constructors
- `ElementConverter()` — Initialize

### Methods
#### `ConvertToUDElement(elements: BaseElementAdapterList) -> BaseElementAdapterList`

Create user defined elements (U-D element) from 3D objects

**Remarks:** Create user defined elements (U-D element) from 3D objects

**Parameters:**
- `elements` (BaseElementAdapterList) — Element adapters pointing to the 3D objects to convert

**Returns:** `BaseElementAdapterList` — Element adapters pointing to the converted 3D objects

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ElementConverter/#NemAll_Python_ArchElements.ElementConverter.ConvertToUDElement)

## FlushPierElement (class)

Implementation of the FlushPier element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierElement/)

### Constructors
- `FlushPierElement() | FlushPierElement( flushPierProp: FlushPierProperties, generalEle: BaseElementAdapter, startPnt: Point2D, endPnt: Point2D, drawPlacementPreview: bool, ) | FlushPierElement(element: FlushPierElement)` — Initialize

### Methods
#### `GetProperties() -> FlushPierProperties`

Get the FlushPier properties

**Remarks:** Get the FlushPier properties

**Returns:** `FlushPierProperties` — FlushPier properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierElement/#NemAll_Python_ArchElements.FlushPierElement.GetProperties)

#### `SetProperties(FlushPierProp: FlushPierProperties)`

Set the FlushPier properties

**Remarks:** Set the FlushPier properties

**Parameters:**
- `FlushPierProp` (FlushPierProperties) — FlushPier properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierElement/#NemAll_Python_ArchElements.FlushPierElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierElement/#NemAll_Python_ArchElements.FlushPierElement.__repr__)

### Properties
- `Properties` (FlushPierProperties, get/set) — Get the FlushPier properties

## FlushPierProperties (class)

Implementation of the FlushPier properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierProperties/)

### Constructors
- `FlushPierProperties() | FlushPierProperties(flushPierProp: FlushPierProperties)` — Initialize

### Methods
#### `GetWidth() -> float`

Get the width of the Flush Pier

**Remarks:** Get the width of the Flush Pier

**Returns:** `float` — Width of the Flush Pier

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierProperties/#NemAll_Python_ArchElements.FlushPierProperties.GetWidth)

#### `SetWidth(width: float)`

Set the width of the Flush Pier

**Remarks:** Set the width of the Flush Pier

**Parameters:**
- `width` (float) — Width of the Flush Pier

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/FlushPierProperties/#NemAll_Python_ArchElements.FlushPierProperties.SetWidth)

### Properties
- `Width` (float, get/set) — Get the width of the Flush Pier

## Functions (static-class)

Module-level functions of NemAll_Python_ArchElements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/)

## GeneralOpeningElement (class)

Representation of (polygonal) niche, recess, slit or opening.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/GeneralOpeningElement/)

### Constructors
- `GeneralOpeningElement() | GeneralOpeningElement( wallOpeningProp: GeneralOpeningProperties, generalEle: BaseElementAdapter, startPnt: Point2D, endPnt: Point2D, drawPlacementPreview: bool, ) | GeneralOpeningElement( wallOpeningProp: GeneralOpeningProperties, generalEle: BaseElementAdapter, groundPlanePolygon: Polygon2D, drawPlacementPreview: bool, ) | GeneralOpeningElement(element: GeneralOpeningElement)` — Initialize with default values

### Methods
#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/GeneralOpeningElement/#NemAll_Python_ArchElements.GeneralOpeningElement.__repr__)

### Properties
- `Properties` (GeneralOpeningProperties, get/set) — Opening properties

## GeneralOpeningProperties (class)

Properties of a regular or polygonal niche, recess, slit or opening

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/GeneralOpeningProperties/)

### Constructors
- `GeneralOpeningProperties(openingType: OpeningType) | GeneralOpeningProperties(openingProp: GeneralOpeningProperties)` — Constructor

### Methods
#### `GetGeometryProperties() -> VerticalOpeningGeometryProperties`

Get a reference of the geometry properties

**Remarks:** Get a reference of the geometry properties

**Returns:** `VerticalOpeningGeometryProperties` — Geometry properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/GeneralOpeningProperties/#NemAll_Python_ArchElements.GeneralOpeningProperties.GetGeometryProperties)

#### `GetOpeningSymbolsProperties() -> OpeningSymbolsProperties`

Get a reference of the opening symbols properties

**Remarks:** Get a reference of the opening symbols properties

**Returns:** `OpeningSymbolsProperties` — Opening symbols properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/GeneralOpeningProperties/#NemAll_Python_ArchElements.GeneralOpeningProperties.GetOpeningSymbolsProperties)

#### `GetSillProperties() -> VerticalOpeningSillProperties`

Get a reference of the sill properties

**Remarks:** Get a reference of the sill properties

**Returns:** `VerticalOpeningSillProperties` — Sill properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/GeneralOpeningProperties/#NemAll_Python_ArchElements.GeneralOpeningProperties.GetSillProperties)

### Properties
- `Independent2DInteraction` (bool, get/set) — Set to True, when the opening is above/below the section plane and should therefor NOT interact with the hatching/filling of the parent architectural element in ground view. Defaults to False.
- `OpeningType` (OpeningType, get/set) — Get the type of the opening
- `PlaneReferences` (PlaneReferences, get/set) — Get the plane references
- `VisibleInViewSection3D` (bool, get/set) — When set to False, the opening does NOT cut out the 3D model element of the parent architectural component (hence opening is invisible in UVSs). Relevant for recess only! Defaults to True.

## JointElement (class)

Implementation of the General opening element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/JointElement/)

### Constructors
- `JointElement() | JointElement( jointProp: JointProperties, generalEle: BaseElementAdapter, startPnt: Point2D, endPnt: Point2D, drawPlacementPreview: bool, ) | JointElement(element: JointElement)` — Initialize

### Methods
#### `GetProperties() -> JointProperties`

Get the General opening properties

**Remarks:** Get the General opening properties

**Returns:** `JointProperties` — General opening properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/JointElement/#NemAll_Python_ArchElements.JointElement.GetProperties)

#### `SetProperties(JointProp: JointProperties)`

Set the GeneralOpening properties

**Remarks:** Set the GeneralOpening properties

**Parameters:**
- `JointProp` (JointProperties) — General opening properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/JointElement/#NemAll_Python_ArchElements.JointElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/JointElement/#NemAll_Python_ArchElements.JointElement.__repr__)

### Properties
- `Properties` (JointProperties, get/set) — Get the General opening properties

## JointProperties (class)

Implementation of the Joint properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/JointProperties/)

### Constructors
- `JointProperties() | JointProperties(jointProp: JointProperties)` — Initialize

### Properties
- `Depth` (float, get/set) — Get the depth of the joint
- `PlaneReferences` (PlaneReferences, get/set) — Get the plane references
- `Width` (float, get/set) — Get the width of the joint

## OpeningSide (enum)

Type of the opening side

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/OpeningSide/)

### Values
- `eInnerSide` = `0`
- `eOuterSide` = `1`

## OpeningSymbolsProperties (class)

Implementation of the opening symbols properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/OpeningSymbolsProperties/)

### Constructors
- `OpeningSymbolsProperties(element: OpeningSymbolsProperties)` — Copy constructor

### Properties
- `OpeningRefPntIndex` (int, get/set) — Get the opening reference point index
- `OpeningTierIndex` (int, get/set) — Get the opening tier index
- `SymbolNames` (list[str], get/set) — Get the symbol names

## OpeningType (enum)

Type of the opening

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/OpeningType/)

### Values
- `eNiche` = `0`
- `eRecess` = `1`

## PlaneReferences (class)

Implementation of the plane references

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/)

### Constructors
- `PlaneReferences(doc: DocumentAdapter, refElement: BaseElementAdapter) | PlaneReferences(element: PlaneReferences)` — Constructor

### Methods
#### `GetAbsBottomElevation() -> float`

Get the absolute bottom elevation

**Remarks:** Get the absolute bottom elevation

**Returns:** `float` — Absolute bottom elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetAbsBottomElevation)

#### `GetAbsTopElevation() -> float`

Get the absolute top elevation

**Remarks:** Get the absolute top elevation

**Returns:** `float` — Absolute top elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetAbsTopElevation)

#### `GetBottomDirection() -> Direction`

Get the bottom direction

**Remarks:** Get the bottom direction

**Returns:** `Direction` — Bottom direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomDirection)

#### `GetBottomElevation() -> float`

Get the bottom plane elevation relative to the bottom plane of the document

**Remarks:** Get the bottom plane elevation relative to the bottom plane of the document

**Returns:** `float` — Bottom plane elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomElevation)

#### `GetBottomOffset() -> float`

Get the offset relative to the defined plane for the bottom

**Remarks:** Get the offset relative to the defined plane for the bottom

**Returns:** `float` — Offset relative to the defined plane for the bottom

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomOffset)

#### `GetBottomPlaneDependency() -> PlaneReferenceDependency`

Get the bottom plane dependency

**Remarks:** Get the bottom plane dependency

**Returns:** `PlaneReferenceDependency` — Bottom plane dependency

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomPlaneDependency)

#### `GetBottomPlaneSurface() -> Any`

Get the bottom plane surface

**Remarks:** Get the bottom plane surface

**Returns:** `Any` — Bottom plane surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomPlaneSurface)

#### `GetBottomReferencePlane() -> ReferencePlaneID`

Get the bottom reference plane

**Remarks:** Get the bottom reference plane

**Returns:** `ReferencePlaneID` — Bottom reference plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetBottomReferencePlane)

#### `GetDocument() -> DocumentAdapter`

Get the document

**Remarks:** Get the document

**Returns:** `DocumentAdapter` — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetDocument)

#### `GetHeight() -> float`

Get the height

**Remarks:** Get the height

**Returns:** `float` — Height

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetHeight)

#### `GetMaximumHeight() -> float`

Get the maximum height

**Remarks:** Get the maximum height

**Returns:** `float` — Maximum height

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetMaximumHeight)

#### `GetReferenceElement() -> BaseElementAdapter`

Get the reference element for the plane

**Remarks:** Get the reference element for the plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetReferenceElement)

#### `GetTopDirection() -> Direction`

Get the top direction

**Remarks:** Get the top direction

**Returns:** `Direction` — Top direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopDirection)

#### `GetTopElevation() -> float`

Get the top plane elevation relative to the top plane of the document

**Remarks:** Get the top plane elevation relative to the top plane of the document

**Returns:** `float` — Top plane elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopElevation)

#### `GetTopOffset() -> float`

Get the offset relative to the defined plane for the top

**Remarks:** Get the offset relative to the defined plane for the top

**Returns:** `float` — Offset relative to the defined plane for the top

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopOffset)

#### `GetTopPlaneDependency() -> PlaneReferenceDependency`

Get the top plane dependency

**Remarks:** Get the top plane dependency

**Returns:** `PlaneReferenceDependency` — Top plane dependency

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopPlaneDependency)

#### `GetTopPlaneSurface() -> Any`

Get the top plane surface

**Remarks:** Get the top plane surface

**Returns:** `Any` — Top plane surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopPlaneSurface)

#### `GetTopReferencePlane() -> ReferencePlaneID`

Get the top reference plane

**Remarks:** Get the top reference plane

**Returns:** `ReferencePlaneID` — Top reference plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.GetTopReferencePlane)

#### `SetAbsBottomElevation(absElevation: float)`

Set the absolute bottom elevation

**Remarks:** Set the absolute bottom elevation

**Parameters:**
- `absElevation` (float) — Absolute bottom elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetAbsBottomElevation)

#### `SetAbsTopElevation(absElevation: float)`

Set the absolute top elevation

**Remarks:** Set the absolute top elevation

**Parameters:**
- `absElevation` (float) — Absolute top elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetAbsTopElevation)

#### `SetBottomDirection(direction: Direction)`

Set the bottom plane Direction

**Remarks:** Set the bottom plane Direction

**Parameters:**
- `direction` (Direction) — Direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomDirection)

#### `SetBottomElevation(elevation: float)`

Set the bottom plane elevation relative to the bottom plane of the document

**Remarks:** Set the bottom plane elevation relative to the bottom plane of the document

**Parameters:**
- `elevation` (float) — Elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomElevation)

#### `SetBottomOffset(offset: float)`

Set the offset relative to the defined plane for the bottom

**Remarks:** Set the offset relative to the defined plane for the bottom

**Parameters:**
- `offset` (float) — Offset relative to the defined plane for the bottom

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomOffset)

#### `SetBottomPlaneDependency(dependency: PlaneReferenceDependency)`

Set the bottom plane dependency

**Remarks:** Set the bottom plane dependency

**Parameters:**
- `dependency` (PlaneReferenceDependency) — Dependency

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomPlaneDependency)

#### `SetBottomPlaneSurface(bottomSurface: object)`

Set the bottom plane surface

**Remarks:** Set the bottom plane surface

**Parameters:**
- `bottomSurface` (object) — Bottom plane surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomPlaneSurface)

#### `SetBottomReferencePlane(bottomReferencePlane: ReferencePlaneID)`

Set the bottom reference plane

**Remarks:** Set the bottom reference plane

**Parameters:**
- `bottomReferencePlane` (ReferencePlaneID) — Bottom reference plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomReferencePlane)

#### `SetBottomSurfacePlaneElement(surfacePlane: BaseElementAdapter)`

Set the bottom surface plane

**Remarks:** Set the bottom surface plane

**Parameters:**
- `surfacePlane` (BaseElementAdapter) — Bottom surface plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomSurfacePlaneElement)

#### `SetBottomToBottom(planeRef: PlaneReferences)`

Set the bottom level to the bottom level of the source plane reference

**Remarks:** Set the bottom level to the bottom level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomToBottom)

#### `SetBottomToTop(planeRef: PlaneReferences)`

Set the bottom level to the top level of the source plane reference

**Remarks:** Set the bottom level to the top level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetBottomToTop)

#### `SetDocument(doc: DocumentAdapter)`

Set the document

**Remarks:** Set the document

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetDocument)

#### `SetElementToPlaneModeling(elementToPlaneModeling: ElementToPlaneModeling)`

Set element to plane modeling

**Remarks:** Set element to plane modeling

**Parameters:**
- `elementToPlaneModeling` (ElementToPlaneModeling) — ElementToPlaneModeling

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetElementToPlaneModeling)

#### `SetHeight(height: float)`

Set the height

**Remarks:** Set the height

**Parameters:**
- `height` (float) — Height

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetHeight)

#### `SetMaximumHeight(maximumHeight: float)`

Set maximum height

**Remarks:** Set maximum height

**Parameters:**
- `maximumHeight` (float) — double

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetMaximumHeight)

#### `SetReferenceElement(refElement: BaseElementAdapter)`

Set the reference element for the plane

**Remarks:** Set the reference element for the plane

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetReferenceElement)

#### `SetTopDirection(direction: Direction)`

Set the top plane Direction

**Remarks:** Set the top plane Direction

**Parameters:**
- `direction` (Direction) — Direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopDirection)

#### `SetTopElevation(elevation: float)`

Set the top plane elevation relative to the top plane of the document

**Remarks:** Set the top plane elevation relative to the top plane of the document

**Parameters:**
- `elevation` (float) — Elevation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopElevation)

#### `SetTopOffset(offset: float)`

Set the offset relative to the defined plane for the top

**Remarks:** Set the offset relative to the defined plane for the top

**Parameters:**
- `offset` (float) — Offset relative to the defined plane for the top

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopOffset)

#### `SetTopPlaneDependency(dependency: PlaneReferenceDependency)`

Set the top plane dependency

**Remarks:** Set the top plane dependency

**Parameters:**
- `dependency` (PlaneReferenceDependency) — Dependency

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopPlaneDependency)

#### `SetTopPlaneSurface(topPlaneSurface: object)`

Set the top plane surface

**Remarks:** Set the top plane surface

**Parameters:**
- `topPlaneSurface` (object) — Top plane surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopPlaneSurface)

#### `SetTopReferencePlane(topReferencePlane: ReferencePlaneID)`

Set the top reference plane

**Remarks:** Set the top reference plane

**Parameters:**
- `topReferencePlane` (ReferencePlaneID) — Top reference plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopReferencePlane)

#### `SetTopSurfacePlaneElement(surfacePlane: BaseElementAdapter)`

Set the top surface plane

**Remarks:** Set the top surface plane

**Parameters:**
- `surfacePlane` (BaseElementAdapter) — Top surface plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopSurfacePlaneElement)

#### `SetTopToBottom(planeRef: PlaneReferences)`

Set the top level to the bottom level of the source plane reference

**Remarks:** Set the top level to the bottom level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopToBottom)

#### `SetTopToTop(planeRef: PlaneReferences)`

Set the top level to the top level of the source plane reference

**Remarks:** Set the top level to the top level of the source plane reference

**Parameters:**
- `planeRef` (PlaneReferences) — Source plane reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.SetTopToTop)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PlaneReferences/#NemAll_Python_ArchElements.PlaneReferences.__repr__)

### Properties
- `AbsBottomElevation` (float, get/set) — Get the absolute bottom elevation
- `AbsTopElevation` (float, get/set) — Get the absolute top elevation
- `BottomDirection` (Direction, get/set) — Get the bottom direction
- `BottomElevation` (float, get/set) — Get the bottom plane elevation relative to the bottom plane of the document
- `BottomOffset` (float, get/set) — Offset between the reference plane and the bottom edge of an architectural component. If the property BottomPlaneDependency is set to eAbsElevation, this is the absolute elevation of the bottom edge.
- `BottomPlaneDependency` (PlaneReferenceDependency, get/set) — Type of dependency of the bottom edge of the architectural component
- `BottomPlaneSurface` (Any, get/set) — Get the bottom plane surface
- `BottomReferencePlane` (ReferencePlaneID, get/set) — Get the bottom reference plane
- `Document` (DocumentAdapter, get/set) — Get the document
- `Height` (float, get/set) — Get the height
- `MaximumHeight` (float, get/set) — Get the maximum height
- `ReferenceElement` (BaseElementAdapter, get/set) — Get the reference element for the plane
- `TopDirection` (Direction, get/set) — Get the top direction
- `TopElevation` (float, get/set) — Get the top plane elevation relative to the top plane of the document
- `TopOffset` (float, get/set) — Offset between the reference plane and the top edge of an architectural component. If the property TopPlaneDependency is set to eAbsElevation, this is the absolute elevation of the top edge.
- `TopPlaneDependency` (PlaneReferenceDependency, get/set) — Type of dependency of the top edge of the architectural component
- `TopPlaneSurface` (Any, get/set) — Get the top plane surface
- `TopReferencePlane` (ReferencePlaneID, get/set) — Get the top reference plane

## ProfileCatalogService (class)

Service providing methods returning profiles as geometrical objects (Path2D, Polyline2D) based on path pointing to a symbol (.sym file) containing a closed profile.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/)

### Methods
#### `GetDoubleProfileGap(fullProfileName: str) -> float`

Gets double profile gap

**Remarks:** Gets double profile gap

**Parameters:**
- `fullProfileName` (str) — Profile name with path

**Returns:** `float` — Double profile gap

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetDoubleProfileGap)

#### `GetFullProfileBoundaryPaths( fullProfileName: str, overrideDefaultGap: bool = False, overrideGap: float = 0.0, ) -> Path2DList`

Get the boundary path of the full profile (e.g. in case of double profile).

**Remarks:** Get the boundary path of the full profile (e.g. in case of double profile).

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Path2DList` — Profile boundary paths

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetFullProfileBoundaryPaths)

#### `GetFullProfileBoundaryPolylines( fullProfileName: str, overrideDefaultGap: bool = False, overrideGap: float = 0.0, ) -> Polyline2DList`

Get the boundary polylines of the full profile (e.g. in case of double profile).

**Remarks:** Get the boundary polylines of the full profile (e.g. in case of double profile).

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Polyline2DList` — Profile boundary polylines

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetFullProfileBoundaryPolylines)

#### `GetProfileAttributes(fullProfileName: str, doc: DocumentAdapter) -> list`

Get the profile attributes

**Remarks:** Get the profile attributes

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `doc` (DocumentAdapter) — Document

**Returns:** `list` — Attributes

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileAttributes)

#### `GetProfileBoundaryPath( fullProfileName: str, overrideDefaultGap: bool = False, overrideGap: float = 0.0, ) -> Path2D`

Get the boundary path of the single profile

**Remarks:** Get the boundary path of the single profile

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Path2D` — Profile boundary path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileBoundaryPath)

#### `GetProfileBoundaryPolyline( fullProfileName: str, overrideDefaultGap: bool = False, overrideGap: float = 0.0, ) -> Polyline2D`

Get the boundary polyline of the single profile

**Remarks:** Get the boundary polyline of the single profile

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Polyline2D` — Profile boundary polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileBoundaryPolyline)

#### `GetProfileGeometry( fullProfileName: str, overrideDefaultGap: bool = False, overrideGap: float = 0.0, ) -> BRep3D`

Get the profile geometry

**Remarks:** Get the profile geometry

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `BRep3D` — Profile geometry as BRep3D

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfileGeometry)

#### `GetProfilePlacementPoint( fullProfileName: str, overrideDefaultGap: bool = False, overrideGap: float = 0.0, ) -> Point3D`

Get the profile placement point

**Remarks:** Get the profile placement point

**Parameters:**
- `fullProfileName` (str) — Profile name with path
- `overrideDefaultGap` (bool) — Override default gap for double profiles
- `overrideGap` (float) — Override gap for double profiles

**Returns:** `Point3D` — Profile placement point as Point3D

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileCatalogService/#NemAll_Python_ArchElements.ProfileCatalogService.GetProfilePlacementPoint)

## ProfileShape (class)

Representation of profile cross section of a structural framing element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ProfileShape/)

### Constructors
- `ProfileShape() | ProfileShape(value: ProfileShape)` — Initialize

### Properties
- `ProfilePath` (None, get) — Full path to the cross section profile

## PropertyDialogs (class)

Utility class providing methods related to architectural properties such as plane references or trade.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/)

### Methods
#### `GetLastSymbolPath() -> str`

Get the last symbol path

**Remarks:** Get the last symbol path

**Returns:** `str` — Last symbol path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.GetLastSymbolPath)

#### `GetSymbolName(symbolPath: str) -> str`

Get the symbol name

**Remarks:** Get the symbol name

**Parameters:**
- `symbolPath` (str) — Symbol path

**Returns:** `str` — Symbol name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.GetSymbolName)

#### `GetTradeDescription(tradeID: int) -> str`

Get the trade description

**Remarks:** Get the trade description

**Parameters:**
- `tradeID` (int) — Trade ID

**Returns:** `str` — Trade description

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.GetTradeDescription)

#### `OpenBottomPlaneReferenceDialog( refElement: BaseElementAdapter, doc: DocumentAdapter, planeRefs: PlaneReferences, ) -> float`

Open the plane references dialog

**Remarks:** Open the plane references dialog

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeRefs` (PlaneReferences) — Plane references

**Returns:** `float` — bottom height

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenBottomPlaneReferenceDialog)

#### `OpenPlaneReferencesDialog( refElement: BaseElementAdapter, doc: DocumentAdapter, planeRefs: PlaneReferences, )`

Open the plane references dialog

**Remarks:** Open the plane references dialog

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeRefs` (PlaneReferences) — Plane references

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenPlaneReferencesDialog)

#### `OpenSmartSymbolPartDialog( smartSymbolPath: str, showSmartSymbols: bool, showSmartParts: bool ) -> str`

Open the library dialog for SmartSymbols and SmartParts

**Remarks:** Open the library dialog for SmartSymbols and SmartParts

**Parameters:**
- `smartSymbolPath` (str) — Path to file
- `showSmartSymbols` (bool) — Show smart symbols state
- `showSmartParts` (bool) — Show SmartParts state

**Returns:** `str` — result path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenSmartSymbolPartDialog)

#### `OpenSymbolDialog(symbolPath: str) -> str`

Open the symbol library dialog

**Remarks:** Open the symbol library dialog

**Parameters:**
- `symbolPath` (str) — Path to .sym file

**Returns:** `str` — result path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenSymbolDialog)

#### `OpenTopPlaneReferenceDialog( refElement: BaseElementAdapter, doc: DocumentAdapter, planeRefs: PlaneReferences, ) -> float`

Open the plane references dialog

**Remarks:** Open the plane references dialog

**Parameters:**
- `refElement` (BaseElementAdapter) — Reference element (empty element if not exist)
- `doc` (DocumentAdapter) — Document
- `planeRefs` (PlaneReferences) — Plane references

**Returns:** `float` — top height

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenTopPlaneReferenceDialog)

#### `OpenTradeDialog(doc: DocumentAdapter, tradeID: int) -> int`

Open the trade dialog

**Remarks:** Open the trade dialog

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `tradeID` (int) — Current trade ID

**Returns:** `int` — Trade ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/PropertyDialogs/#NemAll_Python_ArchElements.PropertyDialogs.OpenTradeDialog)

## RectangularShape (class)

Representation of rectangular cross section of a structural framing element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RectangularShape/)

### Constructors
- `RectangularShape() | RectangularShape(value: RectangularShape)` — Initialize

### Properties
- `Thickness` (None, get) — Thickness of rectangular cross section
- `Width` (None, get) — Width of rectangular cross section

## ReferencePlaneID (class)

struct for handling reference plane IDs

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/)

### Constructors
- `ReferencePlaneID() | ReferencePlaneID(modelGuid: GUID, planeId: int) | ReferencePlaneID(element: ReferencePlaneID)` — Initialize

### Methods
#### `Invalidate()`

Set values to undefined state

**Remarks:** Set values to undefined state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.Invalidate)

#### `IsDefaultLowerPlane() -> bool`

Find out if reference plane is default lower reference plane

**Remarks:** Find out if reference plane is default lower reference plane

**Returns:** `bool` — true if ref plane is default lower ref plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsDefaultLowerPlane)

#### `IsDefaultUpperPlane() -> bool`

Find out if reference plane is default upper reference plane

**Remarks:** Find out if reference plane is default upper reference plane

**Returns:** `bool` — true if ref plane is default upper ref plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsDefaultUpperPlane)

#### `IsDocumentRefSurface() -> bool`

Find out if reference surface is from document

**Remarks:** Find out if reference surface is from document

**Returns:** `bool` — true is reference surface from document (not in model)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsDocumentRefSurface)

#### `IsInModel() -> bool`

Find out if reference plane is in model

**Remarks:** Find out if reference plane is in model

**Returns:** `bool` — true if ref plane is in model

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsInModel)

#### `IsValid() -> bool`

Get validity of ref plane

**Remarks:** Get validity of ref plane

**Returns:** `bool` — true if ref plane is valid

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.IsValid)

#### `SetCustomLowerPlane()`

Set values to custom lower state

**Remarks:** Set values to custom lower state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.SetCustomLowerPlane)

#### `SetCustomUpperPlane()`

Set values to custom upper state

**Remarks:** Set values to custom upper state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.SetCustomUpperPlane)

#### `__eq__(other: ReferencePlaneID) -> bool`

Equal operator

**Remarks:** Equal operator

**Parameters:**
- `other` (ReferencePlaneID) — Reference plane ID to compare

**Returns:** `bool` — Reference plane IDs are equal

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__eq__)

#### `__lt__(other: ReferencePlaneID) -> bool`

Less operator

**Remarks:** Less operator

**Parameters:**
- `other` (ReferencePlaneID) — Reference plane ID to compare

**Returns:** `bool` — Reference plane is lees then other

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__lt__)

#### `__ne__(other: ReferencePlaneID) -> bool`

Unequal operator

**Remarks:** Unequal operator

**Parameters:**
- `other` (ReferencePlaneID) — Reference plane ID to compare

**Returns:** `bool` — Reference plane IDs are not equal

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__ne__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ReferencePlaneID/#NemAll_Python_ArchElements.ReferencePlaneID.__repr__)

### Properties
- `ModelGuid` (GUID, get/set) — Get model change model id to another
- `PlaneId` (int, get) — Get plane ID

## RoomElement (class)

Implementation of the Room element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/)

### Constructors
- `RoomElement() | RoomElement(RoomProp: RoomProperties, RoomPolygon: Polygon2D) | RoomElement(element: RoomElement)` — Initialize

### Methods
#### `GetProperties() -> RoomProperties`

Get the Room properties

**Remarks:** Get the Room properties

**Returns:** `RoomProperties` — Room properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.GetProperties)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.SetCommonProperties)

#### `SetProperties(RoomProp: RoomProperties)`

Set the Room properties

**Remarks:** Set the Room properties

**Parameters:**
- `RoomProp` (RoomProperties) — Room properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomElement/#NemAll_Python_ArchElements.RoomElement.__repr__)

### Properties
- `Properties` (RoomProperties, get/set) — Get the Room properties

## RoomProperties (class)

Implementation of the Room properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/)

### Constructors
- `RoomProperties() | RoomProperties(RoomProp: RoomProperties)` — Initialize

### Methods
#### `GetAttributes( doc: DocumentAdapter, onlyModifiable: bool ) -> list[int | float | str]`

Get the room attributes

**Remarks:** Get the room attributes

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `onlyModifiable` (bool) — when true, gets only modifiable attributes, otherwise gets all attributes

**Returns:** `list[int | float | str]` — List with the attributes represented with tuples like (attributeID, attributeValue)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetAttributes)

#### `GetFunction() -> str`

Get the function

**Remarks:** Get the function

**Returns:** `str` — Function

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetFunction)

#### `GetStoreyCode() -> str`

Get the storey code

**Remarks:** Get the storey code

**Returns:** `str` — Storey code

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetStoreyCode)

#### `GetText(number: int) -> str`

Get the value of the attributes Text1 to Text5

**Remarks:** Get the value of the attributes Text1 to Text5

**Parameters:**
- `number` (int) — Number (1-5 allowed)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.GetText)

#### `SetAttribute( attrib: ( AttributeInteger | AttributeDouble | AttributeString | AttributeEnum ), )`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (AttributeInteger | AttributeDouble | AttributeString | AttributeEnum) — Attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetAttribute)

#### `SetFunction(name: str)`

Set the function

**Remarks:** Set the function

**Parameters:**
- `name` (str) — Function

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetFunction)

#### `SetStoreyCode(storeyCode: str)`

Set the storey code

**Remarks:** Set the storey code

**Parameters:**
- `storeyCode` (str) — Storey code

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetStoreyCode)

#### `SetText(text: str, number: int)`

Set a value to the attributes Text1 to Text5

**Remarks:** Set a value to the attributes Text1 to Text5

**Parameters:**
- `text` (str) — Desired value of the attribute
- `number` (int) — Number (1-5 allowed)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/RoomProperties/#NemAll_Python_ArchElements.RoomProperties.SetText)

### Properties
- `Function` (str, get/set) — Get the function
- `StoreyCode` (str, get/set) — Get the storey code

## ShapeType (enum)

Types of cross sections of linear architectural objects, like beams or columns.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/ShapeType/)

### Values
- `eArbitrary` = `6`
- `eCircular` = `1` — Round cross section
- `eConical` = `4`
- `ePolygonal` = `5` — Cross section bounded with a free, closed polygon
- `eProfile` = `6` — Cross section defined with a symbol from the library containing a closed profile
- `eRectangular` = `0` — Rectangular cross section
- `eRegularPolygonCircumscribed` = `3` — Cross section in a shape of a regular polygon circumscribed on a circle
- `eRegularPolygonInscribed` = `2` — Cross section in a shape of a regular polygon inscribed in a circle
- `eRiseBottomTop` = `7`
- `eUnknown` = `8`

## SlabElement (class)

Implementation of the Slab element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/)

### Constructors
- `SlabElement() | SlabElement(slabProp: SlabProperties, slabPolygon: Polygon2D) | SlabElement(element: SlabElement)` — Initialize

### Methods
#### `GetProperties() -> SlabProperties`

Get the Slab properties

**Remarks:** Get the Slab properties

**Returns:** `SlabProperties` — Slab properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.GetProperties)

#### `SetProperties(SlabProp: SlabProperties)`

Set the Slab properties

**Remarks:** Set the Slab properties

**Parameters:**
- `SlabProp` (SlabProperties) — Slab properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabElement/#NemAll_Python_ArchElements.SlabElement.__repr__)

### Properties
- `Properties` (SlabProperties, get/set) — Get the Slab properties

## SlabOpeningElement (class)

Representation of recess or opening in slab

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/)

### Constructors
- `SlabOpeningElement() | SlabOpeningElement( slabOpeningProp: SlabOpeningProperties, placementPoint: Point3D, slabConnectionUUID: GUID, ) | SlabOpeningElement( slabOpeningProp: SlabOpeningProperties, placementPoint: Point2D, slabConnectionUUID: GUID, ) | SlabOpeningElement(element: SlabOpeningElement)` — Initialize

### Methods
#### `GetPlacementPoint() -> Point2D`

Get the placement point

**Remarks:** Get the placement point

**Returns:** `Point2D` — Placement point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.GetPlacementPoint)

#### `GetProperties() -> SlabOpeningProperties`

Get the slab opening properties

**Remarks:** Get the slab opening properties

**Returns:** `SlabOpeningProperties` — Slab opening properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.GetProperties)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.SetCommonProperties)

#### `SetPlacementPoint(placementPoint: Point2D)`

Set the placement point

**Remarks:** Set the placement point

**Parameters:**
- `placementPoint` (Point2D) — Placement point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.SetPlacementPoint)

#### `SetProperties(slabOpeningProp: SlabOpeningProperties)`

Set the SlabOpening properties

**Remarks:** Set the SlabOpening properties

**Parameters:**
- `slabOpeningProp` (SlabOpeningProperties) — Slab opening properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningElement/#NemAll_Python_ArchElements.SlabOpeningElement.__repr__)

### Properties
- `PlacementPoint` (Point2D, get/set) — Get the placement point
- `Properties` (SlabOpeningProperties, get/set) — Get the slab opening properties

## SlabOpeningProperties (class)

Implementation of the slab opening properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningProperties/)

### Constructors
- `SlabOpeningProperties(openingType: SlabOpeningType = eOpening) | SlabOpeningProperties(openingProps: SlabOpeningProperties)` — Constructor

### Methods
#### `GetOpeningSymbolsProperties() -> OpeningSymbolsProperties`

Get a reference of the opening symbols properties

**Remarks:** Get a reference of the opening symbols properties

**Returns:** `OpeningSymbolsProperties` — Opening symbols properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningProperties/#NemAll_Python_ArchElements.SlabOpeningProperties.GetOpeningSymbolsProperties)

#### `GetOpeningType() -> SlabOpeningType`

Get the opening type

**Remarks:** Get the opening type

**Returns:** `SlabOpeningType` — Opening type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningProperties/#NemAll_Python_ArchElements.SlabOpeningProperties.GetOpeningType)

### Properties
- `Independent2DInteraction` (bool, get/set) — Get the independent 2D interaction state

## SlabOpeningType (enum)

eOpening: eRecess :

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabOpeningType/)

### Values
- `eOpening` = `0`
- `eRecess` = `1`

## SlabProperties (class)

Implementation of the Slab properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabProperties/)

### Constructors
- `SlabProperties() | SlabProperties(slabProp: SlabProperties)` — Initialize

### Methods
#### `SetAttribute(attrib: object)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SlabProperties/#NemAll_Python_ArchElements.SlabProperties.SetAttribute)

## SolidElementTruncationType (enum)

Type of cutting/stretching method of a structural framing element at its start or end

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/SolidElementTruncationType/)

### Values
- `FreePlane` = `3` — Cutting with any plane
- `Horizontal` = `1` — Horizontal cutting
- `NormalToBodyAxis` = `0` — Perpendicular to the element axis
- `Vertical` = `2` — Vertical cutting

## StructuralBeamElement (class)

Representation of the structural framing beam

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/)

### Constructors
- `StructuralBeamElement() | StructuralBeamElement(properties: StructuralBeamProperties) | StructuralBeamElement(element: StructuralBeamElement)` — Initialize

### Methods
#### `GetStructuralBeamProperties() -> StructuralBeamProperties`

Get the Structural Beam properties

**Remarks:** Get the Structural Beam properties

**Returns:** `StructuralBeamProperties` — Beam properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.GetStructuralBeamProperties)

#### `SetAxisVisibility(visibility: bool)`

Set the Visibility of element axis

**Remarks:** Set the Visibility of element axis

**Parameters:**
- `visibility` (bool) — Bool if axis of element should be visible

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.SetAxisVisibility)

#### `SetStructuralBeamProperties(properties: StructuralBeamProperties)`

Set the Structural Beam properties

**Remarks:** Set the Structural Beam properties

**Parameters:**
- `properties` (StructuralBeamProperties) — Beam properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.SetStructuralBeamProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamElement/#NemAll_Python_ArchElements.StructuralBeamElement.__repr__)

### Properties
- `StructuralBeamProperties` (StructuralBeamProperties, get/set) — Get the Structural Beam properties

## StructuralBeamProperties (class)

Representation of the properties of the structural framing beam

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/)

### Constructors
- `StructuralBeamProperties() | StructuralBeamProperties(value: StructuralBeamProperties)` — Initialize

### Methods
#### `GetEndPoint() -> Point3D`

Get the end point of the element in world coordinate system

**Remarks:** Get the end point of the element in world coordinate system

**Returns:** `Point3D` — End point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.GetEndPoint)

#### `GetStartPoint() -> Point3D`

Get the start point of the element in world coordinate system

**Remarks:** Get the start point of the element in world coordinate system

**Returns:** `Point3D` — Start point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.GetStartPoint)

#### `SetAnchorPointProperties( anchorPointStart: int, offsetStart: Point2D, anchorPointEnd: int, offsetEnd: Point2D, twoAnchorPoints: bool, )`

Set the properties of the beam anchor points

**Remarks:** Set the properties of the beam anchor points

**Parameters:**
- `anchorPointStart` (int) — anchor point position
- `offsetStart` (Point2D) — offset from start anchor point
- `anchorPointEnd` (int) — anchor point position
- `offsetEnd` (Point2D) — offset from end anchor point
- `twoAnchorPoints` (bool) — true when both anchor points are used

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetAnchorPointProperties)

#### `SetEndPoint(x: float, y: float, z: float)`

Set the end point of the element in world coordinate system

**Remarks:** Set the end point of the element in world coordinate system

**Parameters:**
- `x` (float) — x coordinate
- `y` (float) — y coordinate
- `z` (float) — z coordinate

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetEndPoint)

#### `SetHeightProperties(doc: DocumentAdapter, planeReferences: PlaneReferences)`

Set height properties

**Remarks:** Set height properties

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `planeReferences` (PlaneReferences) — Height properties as PlaneReferences object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetHeightProperties)

#### `SetHeightPropsByPlacementPoints( doc: DocumentAdapter, value: PlaneReferences, startPointZcoord: float, endPointZcoord: float, )`

Set HeightProperties by the placement points

**Remarks:** Set HeightProperties by the placement points

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `value` (PlaneReferences) — Height properties
- `startPointZcoord` (float) — Z coordinate of the start point
- `endPointZcoord` (float) — Z coordinate of the end point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetHeightPropsByPlacementPoints)

#### `SetStartPoint(x: float, y: float, z: float)`

Set the start point of the element in world coordinate system

**Remarks:** Set the start point of the element in world coordinate system

**Parameters:**
- `x` (float) — x coordinate
- `y` (float) — y coordinate
- `z` (float) — z coordinate

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBeamProperties/#NemAll_Python_ArchElements.StructuralBeamProperties.SetStartPoint)

## StructuralBraceElement (class)

Representation of the structural framing bracing

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/)

### Constructors
- `StructuralBraceElement() | StructuralBraceElement(properties: StructuralBraceProperties) | StructuralBraceElement(element: StructuralBraceElement)` — Initialize

### Methods
#### `GetStructuralBraceProperties() -> StructuralBraceProperties`

Get the Structural Brace properties

**Remarks:** Get the Structural Brace properties

**Returns:** `StructuralBraceProperties` — Brace properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.GetStructuralBraceProperties)

#### `SetAxisVisibility(visibility: bool)`

Set the Visibility of element axis

**Remarks:** Set the Visibility of element axis

**Parameters:**
- `visibility` (bool) — Bool if axis of element should be visible

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.SetAxisVisibility)

#### `SetStructuralBraceProperties(properties: StructuralBraceProperties)`

Set the Structural Brace properties

**Remarks:** Set the Structural Brace properties

**Parameters:**
- `properties` (StructuralBraceProperties) — Brace properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.SetStructuralBraceProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceElement/#NemAll_Python_ArchElements.StructuralBraceElement.__repr__)

### Properties
- `StructuralBraceProperties` (StructuralBraceProperties, get/set) — Get the Structural Brace properties

## StructuralBraceProperties (class)

Representation of the properties of the structural framing bracing

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/)

### Constructors
- `StructuralBraceProperties() | StructuralBraceProperties(value: StructuralBraceProperties)` — Initialize

### Methods
#### `GetEndPoint() -> Point3D`

Get the end point of the element in world coordinate system

**Remarks:** Get the end point of the element in world coordinate system

**Returns:** `Point3D` — End point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.GetEndPoint)

#### `GetStartPoint() -> Point3D`

Get the start point of the element in world coordinate system

**Remarks:** Get the start point of the element in world coordinate system

**Returns:** `Point3D` — Start point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.GetStartPoint)

#### `SetAnchorPointProperties( anchorPointStart: int, offsetStart: Point2D, anchorPointEnd: int, offsetEnd: Point2D, twoAnchorPoints: bool, )`

Set the Column Anchor Point Properties 10 - CenterOfGravity

**Remarks:** Set the Column Anchor Point Properties 10 - CenterOfGravity

**Parameters:**
- `anchorPointStart` (int) — Anchor point position
- `offsetStart` (Point2D) — Offset from start anchor point
- `anchorPointEnd` (int) — Anchor point position
- `offsetEnd` (Point2D) — Offset from end anchor point
- `twoAnchorPoints` (bool) — true when both anchor points are used

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetAnchorPointProperties)

#### `SetEndPoint(x: float, y: float, z: float)`

Set the end point of the element in world coordinate system

**Remarks:** Set the end point of the element in world coordinate system

**Parameters:**
- `x` (float) — x coordinate
- `y` (float) — y coordinate
- `z` (float) — z coordinate

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetEndPoint)

#### `SetHeightProperties(doc: DocumentAdapter, planeReferences: BasePlaneReferences)`

Set height properties

**Remarks:** Set height properties

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `planeReferences` (BasePlaneReferences) — Height properties as PlaneReferences object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetHeightProperties)

#### `SetHeightPropsByPlacementPoints( doc: DocumentAdapter, value: BasePlaneReferences, startPointZcoord: float, endPointZcoord: float, )`

Set HeightProperties by the placement points

**Remarks:** Set HeightProperties by the placement points

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `value` (BasePlaneReferences) — Height properties
- `startPointZcoord` (float) — Z coordinate of the start point
- `endPointZcoord` (float) — Z coordinate of the end point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetHeightPropsByPlacementPoints)

#### `SetStartPoint(x: float, y: float, z: float)`

Set the start point of the element in world coordinate system

**Remarks:** Set the start point of the element in world coordinate system

**Parameters:**
- `x` (float) — x coordinate
- `y` (float) — y coordinate
- `z` (float) — z coordinate

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralBraceProperties/#NemAll_Python_ArchElements.StructuralBraceProperties.SetStartPoint)

## StructuralColumnElement (class)

Representation of the structural framing column

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/)

### Constructors
- `StructuralColumnElement() | StructuralColumnElement(structuralColumnProp: StructuralColumnProperties) | StructuralColumnElement(element: StructuralColumnElement)` — Initialize

### Methods
#### `GetStructuralColumnProperties() -> StructuralColumnProperties`

Get the Structural Column properties

**Remarks:** Get the Structural Column properties

**Returns:** `StructuralColumnProperties` — Column properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.GetStructuralColumnProperties)

#### `SetAxisVisibility(visibility: bool)`

Set the Visibility of element axis

**Remarks:** Set the Visibility of element axis

**Parameters:**
- `visibility` (bool) — Bool if axis of element should be visible

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.SetAxisVisibility)

#### `SetStructuralColumnProperties(structuralColumnProp: StructuralColumnProperties)`

Set the Structural Column properties

**Remarks:** Set the Structural Column properties

**Parameters:**
- `structuralColumnProp` (StructuralColumnProperties) — Column properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.SetStructuralColumnProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnElement/#NemAll_Python_ArchElements.StructuralColumnElement.__repr__)

### Properties
- `StructuralColumnProperties` (StructuralColumnProperties, get/set) — Get the Structural Column properties

## StructuralColumnProperties (class)

Representation of the properties of the structural framing column

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/)

### Constructors
- `StructuralColumnProperties() | StructuralColumnProperties(value: StructuralColumnProperties)` — Initialize

### Methods
#### `GetHeight() -> float`

Get the Column Height

**Remarks:** Get the Column Height

**Returns:** `float` — Column Height

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.GetHeight)

#### `GetPosition() -> Point3D`

Get the Column position in world coordinate system

**Remarks:** Get the Column position in world coordinate system

**Returns:** `Point3D` — Column position in world coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.GetPosition)

#### `SetAnchorPointProperties(anchorPoint: int, offset: Point2D)`

Set the Column Anchor Point Properties 10 - CenterOfGravity

**Remarks:** Set the Column Anchor Point Properties 10 - CenterOfGravity

**Parameters:**
- `anchorPoint` (int) — Anchor point position
- `offset` (Point2D) — Offset from start anchor point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetAnchorPointProperties)

#### `SetHeight( height: float, doc: DocumentAdapter, heightRefereces: PlaneReferences )`

Set Column Height

**Remarks:** Set Column Height

**Parameters:**
- `height` (float) — Height
- `doc` (DocumentAdapter) — Document
- `heightRefereces` (PlaneReferences) — Height Properties of Column, as PlaneReferences type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetHeight)

#### `SetHeightProperties(doc: DocumentAdapter, value: PlaneReferences)`

Set height properties

**Remarks:** Set height properties

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `planeReferences` (object) — Height properties as PlaneReferences object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetHeightProperties)

#### `SetHeightPropsByPlacementPoint( doc: DocumentAdapter, value: PlaneReferences, startPointZcoord: float )`

Set HeightProperties by a placement point

**Remarks:** Set HeightProperties by a placement point

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `value` (PlaneReferences) — Height Properties of Column, as PlaneReferences type
- `startPointZcoord` (float) — Z coordinate start point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetHeightPropsByPlacementPoint)

#### `SetPosition(x: float, y: float, z: float)`

Set the position of the column in world coordinate system

**Remarks:** Set the position of the column in world coordinate system

**Parameters:**
- `x` (float) — x coordinate
- `y` (float) — y coordinate
- `z` (float) — z coordinate

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralColumnProperties/#NemAll_Python_ArchElements.StructuralColumnProperties.SetPosition)

## StructuralElementProperties (class)

Representation of the properties of structural framing element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/)

### Constructors
- `StructuralElementProperties() | StructuralElementProperties(value: StructuralElementProperties)` — Initialize

### Methods
#### `AddHole(holeGeometry: BRep3D) -> int`

Adds a hole to the list of holes for the Structural Element

**Remarks:** Adds a hole to the list of holes for the Structural Element

**Parameters:**
- `holeGeometry` (BRep3D) — 3D BRep Geometry of the hole

**Returns:** `int` — Unique ID for the created hole

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.AddHole)

#### `GetAnglesAtEnd() -> tuple`

Get angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its end

**Remarks:** Get angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its end

**Returns:** `tuple` — angle in local X direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetAnglesAtEnd)

#### `GetAnglesAtStart() -> tuple`

Get angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its start

**Remarks:** Get angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its start

**Returns:** `tuple` — angle in local X direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetAnglesAtStart)

#### `GetBodyAxis() -> Line3D`

Get the Axis of the structural framing element calculated in the cross section's center of gravity

**Remarks:** Get the Axis of the structural framing element calculated in the cross section's center of gravity

**Returns:** `Line3D` — Axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetBodyAxis)

#### `GetCenterAxis() -> Line3D`

Get the axis of the structural framing element calculated in the center of the cross section outline

**Remarks:** Get the axis of the structural framing element calculated in the center of the cross section outline

**Returns:** `Line3D` — Axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetCenterAxis)

#### `GetConnectionAxis() -> Line3D`

Get Connection Axis as a Line3D

**Remarks:** Get Connection Axis as a Line3D

**Returns:** `Line3D` — Line3D which contains information about start and end point coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetConnectionAxis)

#### `GetDoubleProfileGap() -> float`

Gets double profile gap

**Remarks:** Gets double profile gap

**Returns:** `float` — Double profile gap

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetDoubleProfileGap)

#### `GetEndPointAdditionalOffset() -> Vector3D`

Get the end point distance dx/dy

**Remarks:** Get the end point distance dx/dy

**Returns:** `Vector3D` — Vector3D with offsets in x and y like Vector3D(distance_dx, distance dy, 0)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetEndPointAdditionalOffset)

#### `GetEndPointZOffset() -> float`

Get the the offset between the anchor end point and the element end point of a structural framing element

**Remarks:** Get the the offset between the anchor end point and the element end point of a structural framing element

**Returns:** `float` — Offset value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetEndPointZOffset)

#### `GetEndReferencePointType() -> CustomBoxPoint`

Get type the reference point at end

**Remarks:** Get type the reference point at end

**Returns:** `CustomBoxPoint` — Type the reference point at end

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetEndReferencePointType)

#### `GetHeightProperties() -> PlaneReferences`

Get HeightProperties

**Remarks:** Get HeightProperties

**Returns:** `PlaneReferences` — HeightProperties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetHeightProperties)

#### `GetHoles() -> dict`

Returns existing holes as python dictionary, where key is hole ID, value is its geometry

**Remarks:** Returns existing holes as python dictionary, where key is hole ID, value is its geometry

**Returns:** `dict` — Existing holes as python dictionary, where key is hole ID, value is its geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetHoles)

#### `GetProfileAngle() -> Angle`

Get rotation angle of cross section profile around the axis of a structural framing element

**Remarks:** Get rotation angle of cross section profile around the axis of a structural framing element

**Returns:** `Angle` — Rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetProfileAngle)

#### `GetProfileShapeProperties() -> object`

Get the shape properties of the cross section

**Remarks:** Get the shape properties of the cross section

**Returns:** `object` — Cross section properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetProfileShapeProperties)

#### `GetShapeTypeAtEnd() -> SolidElementTruncationType`

Get truncation type at the element's end point

**Remarks:** Get truncation type at the element's end point

**Returns:** `SolidElementTruncationType` — truncation type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetShapeTypeAtEnd)

#### `GetShapeTypeAtStart() -> SolidElementTruncationType`

Get truncation type at the element's start point

**Remarks:** Get truncation type at the element's start point

**Returns:** `SolidElementTruncationType` — truncation type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetShapeTypeAtStart)

#### `GetSkeletonSolidElementProperties() -> object`

Get SkeletonSolidElementProperties

**Remarks:** Get SkeletonSolidElementProperties

**Returns:** `object` — SkeletonSolidElementProperties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetSkeletonSolidElementProperties)

#### `GetStartPointAdditionalOffset() -> Vector3D`

Get the start point distance dx/dy

**Remarks:** Get the start point distance dx/dy

**Returns:** `Vector3D` — Vector3D with offsets in x and y like Vector3D(distance_dx, distance dy, 0)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetStartPointAdditionalOffset)

#### `GetStartPointZOffset() -> float`

Get the the offset between the anchor start point and the element start point of a structural framing element

**Remarks:** Get the the offset between the anchor start point and the element start point of a structural framing element

**Returns:** `float` — Offset value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetStartPointZOffset)

#### `GetStartReferencePointType() -> CustomBoxPoint`

Get type of the reference point at start

**Remarks:** Get type of the reference point at start

**Returns:** `CustomBoxPoint` — Type of the reference point at start

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetStartReferencePointType)

#### `GetZCoordsVisibility() -> bool`

Get visibility of Z coordinates of insertion point

**Remarks:** Get visibility of Z coordinates of insertion point

**Returns:** `bool` — True if Z coordinates visible, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.GetZCoordsVisibility)

#### `HasTwoAnchorPoints() -> bool`

Get the two anchor point state

**Remarks:** Get the two anchor point state

**Returns:** `bool` — Two anchor point state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.HasTwoAnchorPoints)

#### `RemoveHole(holeID: int)`

Removes a hole to the list of holes for the Structural Element

**Remarks:** Removes a hole to the list of holes for the Structural Element

**Parameters:**
- `holeID` (int) — ID of the hole to remove. This ID is obtained by AddHole operation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.RemoveHole)

#### `SetAnglesAtEnd(angleX: Angle, angleY: Angle)`

Set angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its end

**Remarks:** Set angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its end

**Parameters:**
- `angleX` (Angle) — angle in local X direction
- `angleY` (Angle) — angle in local Y direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetAnglesAtEnd)

#### `SetAnglesAtStart(angleX: Angle, angleY: Angle)`

Set angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its start

**Remarks:** Set angles between the cross-sectional area and the plane perpendicular to the axis of the structural framing element at its start

**Parameters:**
- `angleX` (Angle) — angle in local X direction
- `angleY` (Angle) — angle in local Y direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetAnglesAtStart)

#### `SetAttribute(attrib: object)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetAttribute)

#### `SetCommonProperties(comProp: CommonProperties)`

Set the Common properties

**Remarks:** Set the Common properties

**Parameters:**
- `comProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetCommonProperties)

#### `SetDoubleProfileGap(desiredGap: float) -> bool`

Set the gap of double profile

**Remarks:** Set the gap of double profile

**Parameters:**
- `desiredGap` (float) — Desired gap

**Returns:** `bool` — true if attribute Gap was correctly set and exists / false if given profile doesn't have attribute Gap

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetDoubleProfileGap)

#### `SetEndPointZOffset(desiredOffset: float)`

Set the offset between the anchor end point and the element end point of a structural framing element

**Remarks:** Set the offset between the anchor end point and the element end point of a structural framing element

**Parameters:**
- `desiredOffset` (float) — offset value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetEndPointZOffset)

#### `SetProfileAngle(value: Angle)`

Set rotation angle of cross section profile around the axis of a structural framing element

**Remarks:** Set rotation angle of cross section profile around the axis of a structural framing element

**Parameters:**
- `value` (Angle) — Rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetProfileAngle)

#### `SetProfileShapeProperties(value: object)`

Set cross-section shape properties

**Remarks:** Set cross-section shape properties

**Parameters:**
- `value` (object) — Profile Shape Properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetProfileShapeProperties)

#### `SetShapeTypeAtEnd(endTruncationType: SolidElementTruncationType)`

Set truncation type at the element's end point

**Remarks:** Set truncation type at the element's end point

**Parameters:**
- `endTruncationType` (SolidElementTruncationType) — truncation type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetShapeTypeAtEnd)

#### `SetShapeTypeAtStart(startTruncationType: SolidElementTruncationType)`

Set truncation type at the element's start point

**Remarks:** Set truncation type at the element's start point

**Parameters:**
- `startTruncationType` (SolidElementTruncationType) — truncation type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetShapeTypeAtStart)

#### `SetStartPointZOffset(desiredOffset: float)`

Set the offset between the anchor start point and the element start point of a structural framing element

**Remarks:** Set the offset between the anchor start point and the element start point of a structural framing element

**Parameters:**
- `desiredOffset` (float) — offset value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetStartPointZOffset)

#### `SetTwoAnchorPoints(areTwoAnchorPoint: bool)`

Whether the structural framing element should have two different anchor points at beginning and at end

**Remarks:** Whether the structural framing element should have two different anchor points at beginning and at end

**Parameters:**
- `areTwoAnchorPoint` (bool) — True to make the element have two different anchor points, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/StructuralElementProperties/#NemAll_Python_ArchElements.StructuralElementProperties.SetTwoAnchorPoints)

### Properties
- `DoubleProfileGap` (float, get/set) — Gets double profile gap
- `EndPointZOffset` (float, get/set) — Get the value of Z offset of the anchor point at end the point of SF element
- `ProfileAngle` (Angle, get/set) — Get the rotation angle of element profile
- `ProfileShapeProperties` (object, get/set) — Get ProfileShapeProperties
- `ShapeTypeAtEnd` (SolidElementTruncationType, get/set) — Get truncation type at the start of the Structural Element
- `ShapeTypeAtStart` (SolidElementTruncationType, get/set) — Get truncation type at the end of the Structural Element
- `StartPointZOffset` (float, get/set) — Get the value of Z offset of the anchor point at start the point of SF element

## VerticalElementProperties (class)

Base class representing properties of vertical architectural components, such as columns.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/)

### Constructors
- `VerticalElementProperties(element: VerticalElementProperties)` — Copy constructor

### Methods
#### `GetAngle() -> Angle`

Get the angle

**Remarks:** Get the angle

**Returns:** `Angle` — Get angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetAngle)

#### `GetDepth() -> float`

Get the depth

**Remarks:** Get the depth

**Returns:** `float` — Depth

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetDepth)

#### `GetProfileFullName() -> str`

Get the full name of the profile

**Remarks:** Get the full name of the profile

**Returns:** `str` — Path and name of the profile

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetProfileFullName)

#### `GetRadius() -> float`

Get the radius

**Remarks:** Get the radius

**Returns:** `float` — Radius

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetRadius)

#### `GetShapePolygon() -> Polygon2D`

Get the shape polygon

**Remarks:** Get the shape polygon

**Returns:** `Polygon2D` — Shape polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetShapePolygon)

#### `GetShapeType() -> ShapeType`

Get the shape type

**Remarks:** Get the shape type

**Returns:** `ShapeType` — Shape type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetShapeType)

#### `GetWidth() -> float`

Get the width

**Remarks:** Get the width

**Returns:** `float` — Width

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.GetWidth)

#### `SetAngle(angle: Angle)`

Set the angle

**Remarks:** Set the angle

**Parameters:**
- `angle` (Angle) — Angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetAngle)

#### `SetAttribute(attrib: object)`

Set the attribute

**Remarks:** Set the attribute

**Parameters:**
- `attrib` (object) — Attribute

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetAttribute)

#### `SetCornerRadius(radius: float)`

Set the corner radius

**Remarks:** Set the corner radius

**Parameters:**
- `radius` (float) — Corner radius

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetCornerRadius)

#### `SetDepth(depth: float)`

Set the depth

**Remarks:** Set the depth

**Parameters:**
- `depth` (float) — Depth

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetDepth)

#### `SetProfileFullName(fullName: str)`

Set the full name of the profile

**Remarks:** Set the full name of the profile

**Parameters:**
- `fullName` (str) — Path and name of the profile

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetProfileFullName)

#### `SetRadius(radius: float)`

Set the radius

**Remarks:** Set the radius

**Parameters:**
- `radius` (float) — Radius

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetRadius)

#### `SetShapePolygon(shapePol: Polygon2D)`

Set the shape polygon

**Remarks:** Set the shape polygon

**Parameters:**
- `shapePol` (Polygon2D) — Shape polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetShapePolygon)

#### `SetShapeType(shapeType: ShapeType)`

Set the type of the shape

**Remarks:** Set the type of the shape

**Parameters:**
- `shapeType` (ShapeType) — Shape type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetShapeType)

#### `SetSize(width: float, depth: float)`

Set the size of the element

**Remarks:** Set the size of the element

**Parameters:**
- `width` (float) — Width
- `depth` (float) — Depth

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetSize)

#### `SetWidth(width: float)`

Set the width

**Remarks:** Set the width

**Parameters:**
- `width` (float) — Width

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalElementProperties/#NemAll_Python_ArchElements.VerticalElementProperties.SetWidth)

### Properties
- `Angle` (Angle, get/set) — Get the angle
- `Depth` (float, get/set) — Get the depth
- `ProfileFullName` (str, get/set) — Get the full name of the profile
- `Radius` (float, get/set) — Get the radius
- `ShapePolygon` (Polygon2D, get/set) — Get the shape polygon
- `ShapeType` (ShapeType, get/set) — Get the shape type
- `Width` (float, get/set) — Get the width

## VerticalOpeningFacingProperties (class)

Implementation of the vertical opening facing properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningFacingProperties/)

### Constructors
- `VerticalOpeningFacingProperties(element: VerticalOpeningFacingProperties)` — Copy constructor

### Properties
- `LeftDepth` (float, get/set) — Get the left depth
- `LeftFacing` (bool, get/set) — Get the left facing state
- `LeftWidth` (float, get/set) — Get the left width
- `OpeningSide` (OpeningSide, get/set) — Get the opening side
- `RightDepth` (float, get/set) — Get the right depth
- `RightFacing` (bool, get/set) — Get the right facing state
- `RightWidth` (float, get/set) — Get the right width
- `TopDepth` (float, get/set) — Get the top depth
- `TopFacing` (bool, get/set) — Get the top facing state
- `TopWidth` (float, get/set) — Get the top width

## VerticalOpeningGeometryProperties (class)

Implementation of the vertical opening geometry properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningGeometryProperties/)

### Constructors
- `VerticalOpeningGeometryProperties(element: VerticalOpeningGeometryProperties)` — Copy constructor

### Properties
- `Depth` (float, get/set) — Get the depth
- `ProfilePath` (str, get/set) — Get the full name of the profile
- `RiseAtBottom` (float, get/set) — Get the rise at the bottom
- `RiseAtTop` (float, get/set) — Get the rise at the top
- `SegmentsAtBottom` (int, get/set) — Get the segments of rise at bottom
- `SegmentsAtTop` (int, get/set) — Get the segments of rise at top
- `Shape` (VerticalOpeningShapeType, get/set) — Get the opening shape type
- `ShapePolygon` (Polygon2D, get/set) — Get the shape polygon
- `Width` (float, get/set) — Get the width

## VerticalOpeningRevealProperties (class)

Properties of the door/window reveal

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningRevealProperties/)

### Constructors
- `VerticalOpeningRevealProperties(element: VerticalOpeningRevealProperties)` — Copy constructor

### Properties
- `Depth` (float, get/set) — Door/Window depth
- `InnerOffset` (float, get/set) — Offset to inner face (in case of inner reveal)
- `OuterOffset` (float, get/set) — Offset to outer face (in case of inner reveal)
- `SideOffset` (float, get/set) — Side offset (aka projection, in case of outer reveal)
- `Type` (VerticalOpeningRevealType, get/set) — Get the reveal Type

## VerticalOpeningRevealType (enum)

Type of the reveal

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningRevealType/)

### Values
- `eEmbedded` = `1` — Inner reveal object
- `eExterior` = `3` — Outer reveal object - exterior reveal^
- `eInterior` = `2` — Outer reveal object - interior reveal
- `eNone` = `0` — no reveal

## VerticalOpeningShapeType (enum)

Shape of vertical opening (door or window)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningShapeType/)

### Values
- `eArbitrary` = `6` — Arbitrary shape
- `eCircle` = `2` — Circle
- `eDiamond` = `1` — Diamond
- `eRectangle` = `0` — Rectangle
- `eRiseBottomTop` = `5` — Rise at bottom and/or top
- `eSemiCircle` = `4` — Semi circle
- `eSemiDiamond` = `3` — Semi diamond

## VerticalOpeningSillProperties (class)

Implementation of the vertical opening properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningSillProperties/)

### Constructors
- `VerticalOpeningSillProperties(element: VerticalOpeningSillProperties)` — Copy constructor

### Properties
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `Type` (VerticalOpeningSillType, get/set) — Get the sill Type

## VerticalOpeningSillType (enum)

Type of the sill

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningSillType/)

### Values
- `eBothsides` = `3`
- `eInner` = `2`
- `eNone` = `0`
- `eOuter` = `1`

## VerticalOpeningTierOffsetProperties (class)

Implementation of the vertical opening tier offset properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningTierOffsetProperties/)

### Constructors
- `VerticalOpeningTierOffsetProperties(element: VerticalOpeningTierOffsetProperties)` — Copy constructor

### Methods
#### `GetFacingProperties() -> VerticalOpeningFacingProperties`

Get the facing properties

**Remarks:** Get the facing properties

**Returns:** `VerticalOpeningFacingProperties` — Facing properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningTierOffsetProperties/#NemAll_Python_ArchElements.VerticalOpeningTierOffsetProperties.GetFacingProperties)

### Properties
- `BottomOffset` (float, get/set) — Get the bottom offset
- `BottomOffsets` (list[float], get/set) — Get the bottom offsets
- `LeftOffset` (float, get/set) — Get the left offset
- `LeftOffsets` (list[float], get/set) — Get the left offset
- `RightOffset` (float, get/set) — Get the right offset
- `RightOffsets` (list[float], get/set) — Get the right offsets
- `TopOffset` (float, get/set) — Get the top offset
- `TopOffsets` (list[float], get/set) — Get the top offsets
- `Type` (VerticalOpeningTierOffsetType, get/set) — Get the tier offset type

## VerticalOpeningTierOffsetType (enum)

Type of offset settings

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/VerticalOpeningTierOffsetType/)

### Values
- `eAdvanced` = `3` — Advanced offset
- `eInnerSide` = `2` — Offset at inner side
- `eNone` = `0` — No offset
- `eOuterSide` = `1` — Offset at outer side
- `eUnsupported` = `6` — Unsupported
- `eWithInnerFacing` = `5` — With inner facing
- `eWithOuterFacing` = `4` — With outer facing

## WallAxisPosition (enum)

Position of the axis relative to the architectural component (any linear component, not only wall as the name might suggest).

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallAxisPosition/)

### Values
- `eCenter` = `2` — Axis is in the center of the architectural component. Set the Distance property to the half of the component width!
- `eFree` = `8` — Position is defined by in the Distance property
- `eLeft` = `1` — Axis is on the left side of the component (on the +Y side). Set the Distance property to 0!
- `eRight` = `4` — Axis is on the right side of the component (on the -Y side). Set the Distance property to the width of the component!
- `eUnknown` = `0`

## WallElement (class)

Representation of a multilayer wall

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/)

### Constructors
- `WallElement() | WallElement(wallProp: WallProperties, axis: object) | WallElement(element: WallElement)` — Initialize

### Methods
#### `GetProperties() -> WallProperties`

Get the wall properties

**Remarks:** Get the wall properties

**Returns:** `WallProperties` — Wall properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.GetProperties)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.SetCommonProperties)

#### `SetProperties(wallProp: WallProperties)`

Set the wall properties

**Remarks:** Set the wall properties

**Parameters:**
- `wallProp` (WallProperties) — Wall properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.SetProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallElement/#NemAll_Python_ArchElements.WallElement.__repr__)

### Properties
- `Properties` (WallProperties, get/set) — Properties of the wall

## WallProperties (class)

Representation of properties of a multi layer wall

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/)

### Constructors
- `WallProperties() | WallProperties(wallProp: WallProperties)` — Initialize

### Methods
#### `GetAxis() -> AxisProperties`

Get the axis properties

**Remarks:** Get the axis properties

**Returns:** `AxisProperties` — Axis properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetAxis)

#### `GetPathElementAxisType() -> int`

Get the axis type of the path element

**Remarks:** Get the axis type of the path element

**Returns:** `int` — Axis type of the path element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetPathElementAxisType)

#### `GetThickness() -> float`

Get the thickness

**Remarks:** Get the thickness

**Returns:** `float` — Thickness

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetThickness)

#### `GetTierCount() -> int`

Get the tier count

**Remarks:** Get the tier count

**Returns:** `int` — Tier count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetTierCount)

#### `GetWallTierProperties(tierIndex: int) -> WallTierProperties`

Get the properties of a specified wall tier

**Remarks:** Get the properties of a specified wall tier

**Parameters:**
- `tierIndex` (int) — Tier index. First tier has the index 1!

**Returns:** `WallTierProperties` — Wall tier properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.GetWallTierProperties)

#### `LoadFromFavoriteFile(doc: DocumentAdapter)`

Load the properties from the favorite file

**Remarks:** Load the properties from the favorite file

**Parameters:**
- `doc` (DocumentAdapter) — Document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.LoadFromFavoriteFile)

#### `SetAxis(axis: AxisProperties)`

Set the axis

**Remarks:** Set the axis

**Parameters:**
- `axis` (AxisProperties) — Axis properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.SetAxis)

#### `SetPathElementAxisType(pathEleAxisType: int)`

Set the axis type of the path element

**Remarks:** Set the axis type of the path element

**Parameters:**
- `pathEleAxisType` (int) — Axis type of the path element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.SetPathElementAxisType)

#### `SetTierCount(tierCount: int)`

Set the tier count

**Remarks:** Set the tier count

**Parameters:**
- `tierCount` (int) — Tier count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.SetTierCount)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallProperties/#NemAll_Python_ArchElements.WallProperties.__repr__)

### Properties
- `Axis` (AxisProperties, get/set) — Get the axis properties
- `PathElementAxisType` (int, get/set) — Get the axis type of the path element
- `StartNewJoinedWallGroup` (bool, get/set) — Get the state for starting a new joined wall group All walls in a group are used for to create a continues join between the walls.
- `TierCount` (int, get/set) — Get the tier count

## WallTierProperties (class)

Representation of the properties of a single layer of a multi layer wall

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/)

### Constructors
- `WallTierProperties(tierProp: WallTierProperties)` — Copy constructor

### Methods
#### `GetThickness() -> float`

Get the wall layer thickness

**Remarks:** Get the wall layer thickness

**Returns:** `float` — Thickness

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/#NemAll_Python_ArchElements.WallTierProperties.GetThickness)

#### `SetThickness(thickness: float)`

Set the wall layer thickness

**Remarks:** Set the wall layer thickness

**Parameters:**
- `thickness` (float) — Thickness

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/#NemAll_Python_ArchElements.WallTierProperties.SetThickness)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WallTierProperties/#NemAll_Python_ArchElements.WallTierProperties.__repr__)

### Properties
- `Thickness` (float, get/set) — Tier thickness

## WindowOpeningElement (class)

Representation of window opening

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningElement/)

### Constructors
- `WindowOpeningElement() | WindowOpeningElement( openingProp: WindowOpeningProperties, generalEle: BaseElementAdapter, startPnt: Point2D, endPnt: Point2D, drawPlacementPreview: bool, ) | WindowOpeningElement(element: WindowOpeningElement)` — Initialize

### Methods
#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningElement/#NemAll_Python_ArchElements.WindowOpeningElement.__repr__)

### Properties
- `Properties` (WindowOpeningProperties, get/set) — Window opening properties

## WindowOpeningProperties (class)

Properties of window opening

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningProperties/)

### Constructors
- `WindowOpeningProperties() | WindowOpeningProperties(windowProp: WindowOpeningProperties)` — Initialize

### Methods
#### `GetGeometryProperties() -> VerticalOpeningGeometryProperties`

Get a reference of the geometry properties

**Remarks:** Get a reference of the geometry properties

**Returns:** `VerticalOpeningGeometryProperties` — Geometry properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningProperties/#NemAll_Python_ArchElements.WindowOpeningProperties.GetGeometryProperties)

#### `GetOpeningSymbolsProperties() -> OpeningSymbolsProperties`

Get a reference of the opening symbols properties

**Remarks:** Get a reference of the opening symbols properties

**Returns:** `OpeningSymbolsProperties` — Opening symbols properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningProperties/#NemAll_Python_ArchElements.WindowOpeningProperties.GetOpeningSymbolsProperties)

#### `GetRevealProperties() -> VerticalOpeningRevealProperties`

Get a reference of the reveal properties

**Remarks:** Get a reference of the reveal properties

**Returns:** `VerticalOpeningRevealProperties` — Reveal properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningProperties/#NemAll_Python_ArchElements.WindowOpeningProperties.GetRevealProperties)

#### `GetSillProperties() -> VerticalOpeningSillProperties`

Get a reference of the sill properties

**Remarks:** Get a reference of the sill properties

**Returns:** `VerticalOpeningSillProperties` — Sill properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningProperties/#NemAll_Python_ArchElements.WindowOpeningProperties.GetSillProperties)

#### `GetTierOffsetProperties() -> VerticalOpeningTierOffsetProperties`

Get a reference of the tier offset properties

**Remarks:** Get a reference of the tier offset properties

**Returns:** `VerticalOpeningTierOffsetProperties` — Tier offset properties reference

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_ArchElements/WindowOpeningProperties/#NemAll_Python_ArchElements.WindowOpeningProperties.GetTierOffsetProperties)

### Properties
- `Independent2DInteraction` (bool, get/set) — Set to True, when the opening is above/below the section plane and should therefor NOT interact with the hatching/filling of the parent architectural element in ground view. Defaults to False.
- `PlaneReferences` (PlaneReferences, get/set) — Get the plane references

