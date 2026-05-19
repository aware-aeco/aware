---
name: allplan-nemall_python_reinforcement
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_Reinforcement namespace — 44 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, AllplanElement, BarAreaPlacementProperties, AnchorageType, AnchorageLengthService, BarAreaPlacementService, BarPlacement, BarPositionData, and 36 more types.
---

# NemAll_Python_Reinforcement

Auto-generated from vendor docs for allplan 2025.0. 44 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/)

### Methods
#### `GetAttributes() -> object`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetAttributes)

#### `GetBaseElementAdapter() -> BaseElementAdapter`

Get the model element

**Remarks:** Get the model element

**Returns:** `BaseElementAdapter` — Model element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetBaseElementAdapter)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetCommonProperties)

#### `GetGeometryObject() -> object`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetGeometryObject)

#### `GetLabelElements() -> list`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetLabelElements)

#### `GetSubElementID() -> type`

Get the SubElementID

**Remarks:** Get the SubElementID

**Returns:** `type` — SubElementID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer: object)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey: str)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject: object)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements: list)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## AnchorageLengthService (class)

Service class for the anchorage length calculation

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/)

### Constructors
- `AnchorageLengthService()` — Initialize

### Methods
#### `Calculate( concreteGrade: int, steelGrade: int, diameter: float, asMesh: float, bDoubleBar: bool, meshBarDistCross: float, bMesh: bool, barDistance: float, roundLength: float, )`

Calculation of the anchorage length

**Remarks:** Calculation of the anchorage length

**Parameters:**
- `concreteGrade` (int) — Concrete grade index (starting from 1)
- `steelGrade` (int) — Steel grade
- `diameter` (float) — Diameter
- `asMesh` (float) — asMesh of the mesh
- `bDoubleBar` (bool) — Double bar
- `meshBarDistCross` (float) — Distance of the mesh bars cross to the anchorage direction
- `bMesh` (bool) — Anchorage for a mesh
- `barDistance` (float) — Bar distance
- `roundLength` (float) — Rounding length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.Calculate)

#### `CalculateBar( concreteGrade: int, steelGrade: int, diameter: float, bDoubleBar: bool, barDistance: float, roundLength: float, )`

Calculation of the anchorage length for a bar

**Remarks:** Calculation of the anchorage length for a bar

**Parameters:**
- `concreteGrade` (int) — Concrete grade index (starting from 1)
- `steelGrade` (int) — Steel grade
- `diameter` (float) — Diameter
- `bDoubleBar` (bool) — Double bar
- `barDistance` (float) — Bar distance
- `roundLength` (float) — Rounding length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.CalculateBar)

#### `GetAnchorageLength() -> float`

Get the anchorage length

**Remarks:** Get the anchorage length

**Returns:** `float` — Anchorage length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetAnchorageLength)

#### `GetAnchorageType() -> AnchorageType`

Get the anchorage type

**Remarks:** Get the anchorage type

**Returns:** `AnchorageType` — Anchorage type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetAnchorageType)

#### `GetAsFactor() -> float`

Get the as factor required / available

**Remarks:** Get the as factor required / available

**Returns:** `float` — As mesh factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetAsFactor)

#### `GetCompositionZone() -> int`

Get the composition zone

**Remarks:** Get the composition zone

**Returns:** `int` — Composition zone

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetCompositionZone)

#### `GetHookAngle() -> float`

Get the hook angle

**Remarks:** Get the hook angle

**Returns:** `float` — Hook angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetHookAngle)

#### `GetL1() -> float`

Get length L1

**Remarks:** Get length L1

**Returns:** `float` — Length L1

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetL1)

#### `GetL2() -> float`

Get length L2

**Remarks:** Get length L2

**Returns:** `float` — Length L2

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetL2)

#### `GetL3() -> float`

Get length L3

**Remarks:** Get length L3

**Returns:** `float` — Length L3

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetL3)

#### `GetLongitudinalOffset() -> float`

Get the longitudinal offset

**Remarks:** Get the longitudinal offset

**Returns:** `float` — Longitudinal offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetLongitudinalOffset)

#### `GetOverlapLength() -> float`

Get the overlap length

**Remarks:** Get the overlap length

**Returns:** `float` — Overlap length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetOverlapLength)

#### `IsCompressionBar() -> bool`

Get the compression bar state

**Remarks:** Get the compression bar state

**Returns:** `bool` — Compression bar: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.IsCompressionBar)

#### `SetAnchorageType(anchorageType: AnchorageType)`

Set the anchorage type

**Remarks:** Set the anchorage type

**Parameters:**
- `anchorageType` (AnchorageType) — Anchorage type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetAnchorageType)

#### `SetAsFactor(AsFactor: float)`

Set the as factor required / available

**Remarks:** Set the as factor required / available

**Parameters:**
- `AsFactor` (float) — As facto required / availabler

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetAsFactor)

#### `SetCompositionZone(compositionZone: int)`

Set the composition zone

**Remarks:** Set the composition zone

**Parameters:**
- `compositionZone` (int) — Composition zone

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetCompositionZone)

#### `SetCompressionBar(bCompressionBar: bool)`

Set the compression bar state

**Remarks:** Set the compression bar state

**Parameters:**
- `bCompressionBar` (bool) — Compression bar: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetCompressionBar)

#### `SetHookAngle(hookAngle: float)`

Set the hook angle

**Remarks:** Set the hook angle

**Parameters:**
- `hookAngle` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetHookAngle)

#### `SetLongitudinalOffset(longitudinalOffset: float)`

Set the longitudinal offset

**Remarks:** Set the longitudinal offset

**Parameters:**
- `longitudinalOffset` (float) — longitudinal offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetLongitudinalOffset)

## AnchorageType (enum)

Types of the anchorage

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageType/)

### Values
- `eAnchorageHook` = `2`
- `eAnchorageHookOneCrossBar` = `4`
- `eAnchorageStraight` = `1`
- `eAnchorageStraightOneCrossBar` = `3`
- `eAnchorageStraightTwoCrossBars` = `5`

## BarAreaPlacementProperties (enum)

(No description provided in vendor docs for NemAll_Python_Reinforcement.BarAreaPlacementProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementProperties/)

### Constructors
- `BarAreaPlacementProperties() | BarAreaPlacementProperties( diameter: float, distance: float, overlapping: float, isMoveOverlapping: bool, maxBarLength: float, startBarLength: float, maxPlacementLength: float, firstBarEdgeDistance: float, placementStrategy: PlacementStrategy, benching: Benching, benchingLength: float, isPolygonalPlacement: bool, )` — Initialize

## BarAreaPlacementService (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.BarAreaPlacementService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/)

### Constructors
- `BarAreaPlacementService()` — Initialize

### Methods
#### `AddOpeningPolygon(arg2: Polygon3D, openingPol: float)`

Add an opening polygon

**Remarks:** Add an opening polygon

**Parameters:**
- `openingPol` (float) — Opening polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/#NemAll_Python_Reinforcement.BarAreaPlacementService.AddOpeningPolygon)

#### `Calculate( doc: DocumentAdapter, barPlacementProp: BarAreaPlacementProperties, placementMatrix: Matrix3D, concreteCoverZDir: float, ) -> list`

Calculate the meshes

**Remarks:** Calculate the meshes

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `barPlacementProp` (BarAreaPlacementProperties) — Placement properties
- `placementMatrix` (Matrix3D) — Placement matrix
- `concreteCoverZDir` (float) — Concrete cover in the local z direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/#NemAll_Python_Reinforcement.BarAreaPlacementService.Calculate)

#### `SetOuterPolygon(arg2: Polygon3D, placementPol: float)`

Constructor

**Remarks:** Constructor

**Parameters:**
- `placementPol` (float) — Placement polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/#NemAll_Python_Reinforcement.BarAreaPlacementService.SetOuterPolygon)

## BarPlacement (class)

Implementation of the bar placement element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/)

### Constructors
- `BarPlacement() | BarPlacement(placement: BarPlacement) | BarPlacement( positionNumber: int, barCount: int, distVec: Vector3D, startPnt: Point3D, endPnt: Point3D, bendingShape: BendingShape, ) | BarPlacement( positionNumber: int, barCount: int, startBendingShape: BendingShape, endBendingShape: BendingShape, ) | BarPlacement( positionNumber: int, barCount: int, rotationAxis: Line3D, rotationAngle: Angle, bendingShape: BendingShape, )` — Initialize

### Methods
#### `GetBarCount() -> int`

Get the bar count

**Remarks:** Get the bar count

**Returns:** `int` — Bar count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetBarCount)

#### `GetBendingShape() -> BendingShape`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `BendingShape` — Shape polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetBendingShape)

#### `GetBendingShapeMatrix() -> Matrix3D`

Get the bending shape matrix

**Remarks:** Get the bending shape matrix

**Returns:** `Matrix3D` — Bending shape matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetBendingShapeMatrix)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetCommonProperties)

#### `GetDistanceVector() -> Vector3D`

Get the distance vector

**Remarks:** Get the distance vector

**Returns:** `Vector3D` — Distance vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetDistanceVector)

#### `GetEndBendingShape() -> BendingShape`

Get the shape polyline at the end of a polygonal placement

**Remarks:** Get the shape polyline at the end of a polygonal placement

**Returns:** `BendingShape` — Shape polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetEndBendingShape)

#### `GetEndPoint() -> Point3D`

Get the end point of the placement at the placement line

**Remarks:** Get the end point of the placement at the placement line

**Returns:** `Point3D` — End point of the placement at the placement line

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetEndPoint)

#### `GetLabel() -> ReinforcementLabel`

Get the label

**Remarks:** Get the label

**Returns:** `ReinforcementLabel` — Label

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetLabel)

#### `GetLengthFactor() -> float`

Get the length factor

**Remarks:** Get the length factor

**Returns:** `float` — Length factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetLengthFactor)

#### `GetPlacementMatrix() -> Matrix3D`

Get the placement matrix of the first bar

**Remarks:** Get the placement matrix of the first bar

**Returns:** `Matrix3D` — Placement matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetPlacementMatrix)

#### `GetPositionNumber() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetPositionNumber)

#### `GetRotationAngle() -> Angle`

Get the rotation angle

**Remarks:** Get the rotation angle

**Returns:** `Angle` — Rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetRotationAngle)

#### `GetRotationAxis() -> Line3D`

Get the rotation axis

**Remarks:** Get the rotation axis

**Returns:** `Line3D` — Rotation axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetRotationAxis)

#### `GetStartPoint() -> Point3D`

Get start point of the placement at the placement line

**Remarks:** Get start point of the placement at the placement line

**Returns:** `Point3D` — Start point of the placement at the placement line

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetStartPoint)

#### `IsPlacePerLinearMeter() -> bool`

Get the place per linear meter state

**Remarks:** Get the place per linear meter state

**Returns:** `bool` — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.IsPlacePerLinearMeter)

#### `IsPolygonalPlacement() -> bool`

Get the polygonal placement state

**Remarks:** Get the polygonal placement state

**Returns:** `bool` — Polygonal placement: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.IsPolygonalPlacement)

#### `IsRotationalPlacement() -> bool`

Get the rotational placement state

**Remarks:** Get the rotational placement state

**Returns:** `bool` — Rotational placement: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.IsRotationalPlacement)

#### `Move(transVec: Vector3D)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.Move)

#### `SetBarCount(barCount: int)`

Set the bar count

**Remarks:** Set the bar count

**Parameters:**
- `barCount` (int) — Bar count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetBarCount)

#### `SetBendingShape(shape: BendingShape)`

Set the reinforcement shape

**Remarks:** Set the reinforcement shape

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetBendingShape)

#### `SetBendingShapeMatrix(bendingShapeMat: Matrix3D)`

Set the bending shape matrix

**Remarks:** Set the bending shape matrix

**Parameters:**
- `bendingShapeMat` (Matrix3D) — Bending shape matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetBendingShapeMatrix)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetCommonProperties)

#### `SetDistanceVector(distVec: Vector3D)`

Set the distance vector

**Remarks:** Set the distance vector

**Parameters:**
- `distVec` (Vector3D) — Distance vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetDistanceVector)

#### `SetEndBendingShape(shape: BendingShape)`

Set the reinforcement shape at the end

**Remarks:** Set the reinforcement shape at the end

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetEndBendingShape)

#### `SetLabel(label: ReinforcementLabel, labelAssocView: AssocViewElementAdapter)`

Set the label

**Remarks:** Set the label

**Parameters:**
- `label` (ReinforcementLabel) — Label
- `labelAssocView` (AssocViewElementAdapter) — Associative view for the label

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetLabel)

#### `SetLengthFactor(lengthFactor: float)`

Set the length factor

**Remarks:** Set the length factor

**Parameters:**
- `lengthFactor` (float) — Length factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetLengthFactor)

#### `SetPlacePerLinearMeter(bPlacePerLinearMeter: bool)`

Set the place per linear meter state

**Remarks:** Set the place per linear meter state

**Parameters:**
- `bPlacePerLinearMeter` (bool) — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetPlacePerLinearMeter)

#### `SetPositionNumber(positionNumber: int)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetPositionNumber)

#### `SetRotationAngle(rotationAngle: float)`

Set the rotation angle

**Remarks:** Set the rotation angle

**Parameters:**
- `rotationAngle` (float) — Rotation angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetRotationAngle)

#### `SetRotationAxis(rotationAxis: Line3D)`

Set the rotation axis

**Remarks:** Set the rotation axis

**Parameters:**
- `rotationAxis` (Line3D) — Rotation axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetRotationAxis)

#### `SetRotationalPlacement(bRotationalPlacement: bool)`

Set the rotational placement state

**Remarks:** Set the rotational placement state

**Parameters:**
- `bRotationalPlacement` (bool) — Rotational placement state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetRotationalPlacement)

#### `Transform(transMat: Matrix3D)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.Transform)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.__repr__)

### Properties
- `BarCount` (int, get/set) — Get the bar count
- `BendingShape` (BendingShape, get/set) — Get the shape polyline
- `BendingShapeMatrix` (Matrix3D, get/set) — Get the bending shape matrix
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `DistanceVector` (Vector3D, get/set) — Get the distance vector
- `EndBendingShape` (BendingShape, get/set) — Get the shape polyline at the end of a polygonal placement
- `LengthFactor` (float, get/set) — Get the length factor
- `PlacePerLinearMeter` (bool, get/set) — Get the place per linear meter state
- `PositionNumber` (int, get/set) — Get the position number
- `RotationAngle` (Angle, get/set) — Get the rotation angle
- `RotationAxis` (Line3D, get/set) — Get the rotation axis
- `RotationalPlacement` (bool, get/set) — Get the rotational placement state

## BarPlacementSection (class)

Implementation of the bar placement section class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/)

### Constructors
- `BarPlacementSection(isEnabled: bool, length: float, distance: float) | BarPlacementSection(element: BarPlacementSection)` — Constructor

### Methods
#### `GetDistance() -> float`

Get the distance

**Remarks:** Get the distance

**Returns:** `float` — Distance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/#NemAll_Python_Reinforcement.BarPlacementSection.GetDistance)

#### `GetLength() -> float`

Get the length

**Remarks:** Get the length

**Returns:** `float` — Length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/#NemAll_Python_Reinforcement.BarPlacementSection.GetLength)

#### `IsEnabled() -> bool`

Get the enabled state

**Remarks:** Get the enabled state

**Returns:** `bool` — Enable state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/#NemAll_Python_Reinforcement.BarPlacementSection.IsEnabled)

## BarPositionData (class)

Implementation of the bar position data

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/)

### Constructors
- `BarPositionData() | BarPositionData(barElement: BaseElementAdapter) | BarPositionData(param: BarPositionData) | BarPositionData(bendingShape: BendingShape)` — Initialize

### Methods
#### `GetCount() -> int`

Get the count

**Remarks:** Get the count

**Returns:** `int` — Count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetCount)

#### `GetLength() -> float`

Get the length

**Remarks:** Get the length

**Returns:** `float` — length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetLength)

#### `GetPosition() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetPosition)

#### `SetCount(count: int)`

Set the count

**Remarks:** Set the count

**Parameters:**
- `count` (int) — Count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetCount)

#### `SetLength(length: float)`

Set the length

**Remarks:** Set the length

**Parameters:**
- `length` (float) — length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetLength)

#### `SetPosition(position: int)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `position` (int) — Position

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetPosition)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.__repr__)

### Properties
- `Count` (int, get/set) — Get the count
- `Length` (float, get/set) — Get the length
- `Position` (int, get/set) — Get the position number

## BarsOperations (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.BarsOperations.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarsOperations/)

### Constructors
- `BarsOperations()` — Initialize

### Methods
#### `DivideBarsPlacement( placement: DivideBarsParameters, divisionPolyline: Polyline2D ) -> str`

Divide the bars placement

**Remarks:** Divide the bars placement

**Parameters:**
- `placement` (DivideBarsParameters) — BaseElementAdapter with the placement
- `divideBarsParameter` (object) — Divide bars parameters
- `divisionPolyline` (Polyline2D) — Divison polyline

**Returns:** `str` — Result message

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarsOperations/#NemAll_Python_Reinforcement.BarsOperations.DivideBarsPlacement)

#### `JoinBarsPlacements(placement: BaseElementAdapterList, fillEdges: bool) -> str`

Join the bars placements

**Remarks:** Join the bars placements

**Parameters:**
- `placements` (object) — BaseElementAdapterList with the placements
- `fillEdges` (bool) — Fill the edges: True/False

**Returns:** `str` — Result message

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarsOperations/#NemAll_Python_Reinforcement.BarsOperations.JoinBarsPlacements)

## BendingRollerService (class)

Service class for the bending roller calculation

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/)

### Methods
#### `GetBendBendingRollerFactor( diameter: float, steelGrade: int, concreteGrade: int ) -> float`

Get the bend bending roller factor

**Remarks:** Get the bend bending roller factor

**Parameters:**
- `diameter` (float) — Diameter
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade

**Returns:** `float` — Bending roller factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetBendBendingRollerFactor)

#### `GetBendingRoller( diameter: float, steelGrade: int, concreteGrade: int, bStirrup: bool ) -> float`

Get the bending roller

**Remarks:** Get the bending roller

**Parameters:**
- `diameter` (float) — Diameter
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade
- `bStirrup` (bool) — Shape is a stirrup: true/false

**Returns:** `float` — Bending roller factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetBendingRoller)

#### `GetBendingRollerFactor( diameter: float, steelGrade: int, concreteGrade: int, bStirrup: bool ) -> float`

Get the bending roller factor

**Remarks:** Get the bending roller factor

**Parameters:**
- `diameter` (float) — Diameter
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade
- `bStirrup` (bool) — Shape is a stirrup: true/false

**Returns:** `float` — Bending roller factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetBendingRollerFactor)

#### `GetDefaultBendingRollers(norm: NormType) -> VecDoubleList`

Get the default bending rollers

**Remarks:** Get the default bending rollers

**Parameters:**
- `norm` (NormType) — Norm

**Returns:** `VecDoubleList` — Default bending rollers

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetDefaultBendingRollers)

## BendingShape (class)

Implementation of the reinforcement shape

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/)

### Constructors
- `BendingShape() | BendingShape( shapePol: Polyline3D, bendingRoller: VecDoubleList, diameter: float, steelGrade: int, concreteGrade: int, bendingShapeType: BendingShapeType, ) | BendingShape( shapePoint: Point3D, diameter: float, steelGrade: int, concreteGrade: int ) | BendingShape( shapePol: Polyline3D, bendingRoller: VecDoubleList, meshType: str, meshBendingDirection: MeshBendingDirection, steelGrade: int, concreteGrade: int, bendingShapeType: BendingShapeType, ) | BendingShape( shapePol: Polyline3D, bendingRoller: VecDoubleList, diameter: float, steelGrade: int, concreteGrade: int, bendingShapeType: BendingShapeType, hookLengthStart: float, hookAngleStart: float, hookTypeStart: HookType, hookLengthEnd: float, hookAngleEnd: float, hookTypeEnd: HookType, ) | BendingShape( shapePol: Polyline3D, bendingRoller: VecDoubleList, meshType: str, meshBendingDirection: MeshBendingDirection, steelGrade: int, concreteGrade: int, bendingShapeType: BendingShapeType, hookLengthStart: float, hookAngleStart: float, hookTypeStart: HookType, hookLengthEnd: float, hookAngleEnd: float, hookTypeEnd: HookType, ) | BendingShape(element: BendingShape)` — Initialize

### Methods
#### `GetBendingRoller() -> VecDoubleList`

Get the bending roller

**Remarks:** Get the bending roller

**Returns:** `VecDoubleList` — Bending roller

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetBendingRoller)

#### `GetBendingShapeType() -> BendingShapeType`

Get the bending shape type

**Remarks:** Get the bending shape type

**Returns:** `BendingShapeType` — Bending shape type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetBendingShapeType)

#### `GetConcreteGrade() -> int`

Get the concrete grade

**Remarks:** Get the concrete grade

**Returns:** `int` — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetConcreteGrade)

#### `GetDiameter() -> float`

Get the diameter

**Remarks:** Get the diameter

**Returns:** `float` — Diameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetDiameter)

#### `GetHookAngleEnd() -> float`

Get the hook angle a the end of the shape

**Remarks:** Get the hook angle a the end of the shape

**Returns:** `float` — Hook angle at the end of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookAngleEnd)

#### `GetHookAngleStart() -> float`

Get the hook angle a the start of the shape

**Remarks:** Get the hook angle a the start of the shape

**Returns:** `float` — Hook angle at the start of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookAngleStart)

#### `GetHookLengthEnd() -> float`

Get the hook length a the end of the shape

**Remarks:** Get the hook length a the end of the shape

**Returns:** `float` — Hook length at the end of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookLengthEnd)

#### `GetHookLengthStart() -> float`

Get the hook length a the start of the shape

**Remarks:** Get the hook length a the start of the shape

**Returns:** `float` — Hook length at the start of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookLengthStart)

#### `GetHookTypeEnd() -> HookType`

Get the hook type a the end of the shape

**Remarks:** Get the hook type a the end of the shape

**Returns:** `HookType` — Hook type a the end of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookTypeEnd)

#### `GetHookTypeStart() -> HookType`

Get the hook type a the start of the shape

**Remarks:** Get the hook type a the start of the shape

**Returns:** `HookType` — Hook type a the start of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookTypeStart)

#### `GetMeshBendingDirection() -> MeshBendingDirection`

Get the mesh bending direction

**Remarks:** Get the mesh bending direction

**Returns:** `MeshBendingDirection` — Mesh bending direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetMeshBendingDirection)

#### `GetMeshType() -> str`

Get the mesh type

**Remarks:** Get the mesh type

**Returns:** `str` — Mesh type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetMeshType)

#### `GetShapePolyline() -> Polyline3D`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `Polyline3D` — Shape polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetShapePolyline)

#### `GetSteelGrade() -> int`

Get the steel grade

**Remarks:** Get the steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetSteelGrade)

#### `IsValid() -> bool`

Get the state of the shape

**Remarks:** Get the state of the shape

**Returns:** `bool` — Shape is valid: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.IsValid)

#### `Move(transVec: Vector3D)`

Move the shape

**Remarks:** Move the shape

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.Move)

#### `Rotate(modelAngles: object, refPnt: Point3D) | Rotate(modelAngles: object)`

Rotate the shape

**Remarks:** Rotate the shape

**Parameters:**
- `modelAngles` (object) — Model angles
- `refPnt` (Point3D) — Reference point of the rotation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.Rotate)

#### `SetBendingRoller(bendingRoller: VecDoubleList)`

Set the bending roller

**Remarks:** Set the bending roller

**Parameters:**
- `bendingRoller` (VecDoubleList) — Bending roller

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetBendingRoller)

#### `SetDiameter(diameter: float)`

Set the diameter

**Remarks:** Set the diameter

**Parameters:**
- `diameter` (float) — diameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetDiameter)

#### `SetHookAngleEnd(hookAngleEnd: float)`

Set the hook angle at the end of the shape

**Remarks:** Set the hook angle at the end of the shape

**Parameters:**
- `hookAngleEnd` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookAngleEnd)

#### `SetHookAngleStart(hookAngleStart: float)`

Set the hook angle at the start of the shape

**Remarks:** Set the hook angle at the start of the shape

**Parameters:**
- `hookAngleStart` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookAngleStart)

#### `SetHookLengthEnd(hookLengthEnd: float)`

Set the end length of the hook

**Remarks:** Set the end length of the hook

**Parameters:**
- `hookLengthEnd` (float) — End length of the hook

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookLengthEnd)

#### `SetHookLengthStart(hookLengthStart: float)`

Set the start length of the hook

**Remarks:** Set the start length of the hook

**Parameters:**
- `hookLengthStart` (float) — Start length of the hook

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookLengthStart)

#### `SetHookTypeEnd(hookTypeEnd: HookType)`

Set the hook type at the end of the shape

**Remarks:** Set the hook type at the end of the shape

**Parameters:**
- `hookTypeEnd` (HookType) — Hook type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookTypeEnd)

#### `SetHookTypeStart(hookTypeStart: HookType)`

Set the hook type at the start of the shape

**Remarks:** Set the hook type at the start of the shape

**Parameters:**
- `hookTypeStart` (HookType) — Hook type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookTypeStart)

#### `SetShapePolyline(shapePol: Polyline3D)`

Set the shape polyline

**Remarks:** Set the shape polyline

**Parameters:**
- `shapePol` (Polyline3D) — Shape polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetShapePolyline)

#### `SetSteelGrade(steelGrade: int)`

Set the steel grade

**Remarks:** Set the steel grade

**Parameters:**
- `steelGrade` (int) — steel grade

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetSteelGrade)

#### `Transform(transMat: Matrix3D)`

Transform the shape

**Remarks:** Transform the shape

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.Transform)

#### `__eq__(shape: BendingShape) -> bool`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `shape` (BendingShape) — Shape to compare

**Returns:** `bool` — Shapes are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.__repr__)

### Properties
- `BendingRoller` (list[float], get/set) — Get the bending roller
- `Diameter` (float, get/set) — Get the diameter
- `HookAngleEnd` (float, get/set) — Get the hook angle a the end of the shape
- `HookAngleStart` (float, get/set) — Get the hook angle a the start of the shape
- `HookLengthEnd` (float, get/set) — Get the hook length a the end of the shape
- `HookLengthStart` (float, get/set) — Get the hook length a the start of the shape
- `HookTypeEnd` (HookType, get/set) — Get the hook type a the end of the shape
- `HookTypeStart` (HookType, get/set) — Get the hook type a the start of the shape
- `ShapePolyline` (Polyline3D, get/set) — Get the shape polyline
- `SteelGrade` (int, get/set) — Get the steel grade

## BendingShapeList (class)

List for BendingShape objects

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/)

### Constructors
- `BendingShapeList() | BendingShapeList(ele: BendingShape) | BendingShapeList(eleList: list)` — Initialize

### Methods
#### `__contains__(value: BendingShape) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BendingShape) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__contains__)

#### `__delitem__(value: BendingShape)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BendingShape) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__delitem__)

#### `__eq__(compare_list: BendingShapeList) -> bool`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (BendingShapeList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__eq__)

#### `__getitem__(index: int) -> BendingShape`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BendingShape` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__getitem__)

#### `__iadd__(eleList: list) -> BendingShapeList`

Add a list

**Remarks:** Add a list

**Parameters:**
- `eleList` (list) — BendingShape list

**Returns:** `BendingShapeList` — Lists with the added elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__iadd__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__len__)

#### `__repr__() -> str`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__repr__)

#### `__setitem__(index: int | slice, value: BendingShape)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BendingShape) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__setitem__)

#### `append(value: BendingShape)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BendingShape) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.append)

#### `extend(iterable: BendingShapeList) | extend(eleList: list)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BendingShapeList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.extend)

## BendingShapeType (enum)

Type of the bending shape

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeType/)

### Values
- `BarSpacer` = `113`
- `BarWithArc` = `115`
- `CircleStirrup` = `67`
- `ColumnStirrup` = `110`
- `Freeform` = `99`
- `LShapedBar` = `11`
- `LongitudinalBar` = `0`
- `LongitudinalBarDoubleBentOff` = `26`
- `LongitudinalBarFourTimesBentOff` = `44`
- `LongitudinalBarSingleBentOff` = `15`
- `OpenStirrup` = `21`
- `SHook` = `112`
- `Stirrup` = `51`
- `TorsionStirrup` = `111`

## CircularAreaElement (class)

Implementation of the bar placement element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/)

### Constructors
- `CircularAreaElement() | CircularAreaElement( positionNumber: int, diameter: float, steelGrade: int, concreteGrade: int, rotationAxis: Line3D, contourPoints: Polyline3D, outerAngleStart: float, outerAngleEnd: float, innerAngleStart: float, innerAngleEnd: float, concreteCoverStart: float, concreteCoverEnd: float, concreteCoverContour: float, ) | CircularAreaElement(element: CircularAreaElement)` — Initialize

### Methods
#### `GetConcreteCoverContour() -> float`

Get the concrete cover from the contour

**Remarks:** Get the concrete cover from the contour

**Returns:** `float` — Concrete cover from the contour

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteCoverContour)

#### `GetConcreteCoverEnd() -> float`

Get the concrete cover from the end

**Remarks:** Get the concrete cover from the end

**Returns:** `float` — Concrete cover from the end

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteCoverEnd)

#### `GetConcreteCoverStart() -> float`

Get the concrete cover from the start

**Remarks:** Get the concrete cover from the start

**Returns:** `float` — Concrete cover from the start

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteCoverStart)

#### `GetConcreteGrade() -> int`

Get the concrete grade

**Remarks:** Get the concrete grade

**Returns:** `int` — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteGrade)

#### `GetContourPoints() -> Polyline3D`

Get the contour points

**Remarks:** Get the contour points

**Returns:** `Polyline3D` — Contour points

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetContourPoints)

#### `GetDiameter() -> float`

Get the diameter

**Remarks:** Get the diameter

**Returns:** `float` — Diameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetDiameter)

#### `GetDistance() -> float`

Get the distance

**Remarks:** Get the distance

**Returns:** `float` — Distance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetDistance)

#### `GetEvenFirstLength() -> float`

Get the first length for the even ring number

**Remarks:** Get the first length for the even ring number

**Returns:** `float` — First length for the even ring number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetEvenFirstLength)

#### `GetEvenOverlapEnd() -> float`

Get the overlap length at the end for the even ring number

**Remarks:** Get the overlap length at the end for the even ring number

**Returns:** `float` — Overlap length at the end for the even ring number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetEvenOverlapEnd)

#### `GetEvenOverlapStart() -> float`

Get the overlap length at the start for the even ring number

**Remarks:** Get the overlap length at the start for the even ring number

**Returns:** `float` — Overlap length at the start for the even ring number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetEvenOverlapStart)

#### `GetLengthFactor() -> float`

Get the length factor

**Remarks:** Get the length factor

**Returns:** `float` — Length factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetLengthFactor)

#### `GetMaxBarLength() -> float`

Get the maximal bar length

**Remarks:** Get the maximal bar length

**Returns:** `float` — Maximal bar length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMaxBarLength)

#### `GetMaxBarRise() -> float`

Get the maximal bar radius

**Remarks:** Get the maximal bar radius

**Returns:** `float` — Maximal bar radius

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMaxBarRise)

#### `GetMinBarLength() -> float`

Get the minimal bar length

**Remarks:** Get the minimal bar length

**Returns:** `float` — Minimal bar length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMinBarLength)

#### `GetMinBarRadius() -> float`

Get the minimal bar radius

**Remarks:** Get the minimal bar radius

**Returns:** `float` — Minimal bar radius

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMinBarRadius)

#### `GetOddFirstLength() -> float`

Get the first length for the odd ring number

**Remarks:** Get the first length for the odd ring number

**Returns:** `float` — First length for the odd ring number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOddFirstLength)

#### `GetOddOverlapEnd() -> float`

Get the overlap length at the end for the odd ring number

**Remarks:** Get the overlap length at the end for the odd ring number

**Returns:** `float` — Overlap length at the end for the odd ring number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOddOverlapEnd)

#### `GetOddOverlapStart() -> float`

Get the overlap length at the start for the even ring number

**Remarks:** Get the overlap length at the start for the even ring number

**Returns:** `float` — Overlap length at the start for the odd ring number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOddOverlapStart)

#### `GetOuterAngleEnd() -> float`

Get the outer angle at the end

**Remarks:** Get the outer angle at the end

**Returns:** `float` — Outer angle at the end

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOuterAngleEnd)

#### `GetOuterAngleStart() -> float`

Get the outer angle at the start

**Remarks:** Get the outer angle at the start

**Returns:** `float` — Outer angle at the start

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOuterAngleStart)

#### `GetOverlapLength() -> float`

Get the overlap length

**Remarks:** Get the overlap length

**Returns:** `float` — Overlap length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOverlapLength)

#### `GetPlacementRule() -> int`

Get the placement rule

**Remarks:** Get the placement rule

**Returns:** `int` — Placement rule

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetPlacementRule)

#### `GetPositionNumber() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetPositionNumber)

#### `GetRotationAxis() -> Line3D`

Get the rotation axis

**Remarks:** Get the rotation axis

**Returns:** `Line3D` — Rotation axis

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetRotationAxis)

#### `GetSteelGrade() -> int`

Get the steel grade

**Remarks:** Get the steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetSteelGrade)

#### `GetinnerAngleEnd() -> float`

Get the inner angle at the end

**Remarks:** Get the inner angle at the end

**Returns:** `float` — Inner angle at the end

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetinnerAngleEnd)

#### `GetinnerAngleStart() -> float`

Get the inner angle at the start

**Remarks:** Get the inner angle at the start

**Returns:** `float` — Inner angle at the start

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetinnerAngleStart)

#### `IsPlacePerLinearMeter() -> bool`

Get the place per linear meter state

**Remarks:** Get the place per linear meter state

**Returns:** `bool` — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.IsPlacePerLinearMeter)

#### `IsbOverlapEndAsCircle() -> bool`

Get the overlap state at the end

**Remarks:** Get the overlap state at the end

**Returns:** `bool` — Overlap length at the end as circle = true, as tangent = false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.IsbOverlapEndAsCircle)

#### `IsbOverlapStartAsCircle() -> bool`

Get the overlap state at the start

**Remarks:** Get the overlap state at the start

**Returns:** `bool` — Overlap length at the start as circle = true, as tangent = false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.IsbOverlapStartAsCircle)

#### `SetBarProperties( distance: float, maxBarLength: float, minBarLength: float, placementRule: int, oddFirstLength: float, evenFirstLength: float, minBarRadius: float, maxBarRise: float, )`

Set the bar properties

**Remarks:** Set the bar properties

**Parameters:**
- `distance` (float) — Distance
- `maxBarLength` (float) — Maximal bar length
- `minBarLength` (float) — Minimal bar length
- `placementRule` (int) — Placement rule
- `oddFirstLength` (float) — First length for the odd ring number
- `evenFirstLength` (float) — First bar length for the event ring number
- `minBarRadius` (float) — Minimal bar radius
- `maxBarRise` (float) — Maximal bar rise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetBarProperties)

#### `SetLengthFactor(lengthFactor: float)`

Set the length factor

**Remarks:** Set the length factor

**Parameters:**
- `lengthFactor` (float) — Length factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetLengthFactor)

#### `SetOverlap( oddOverlapStart: float, evenOverlapStart: float, bOverlapStartAsCircle: bool, oddOverlapEnd: float, evenOverlapEnd: float, bOverlapEndAsCircle: bool, overlapLength: float, )`

Set the overlap

**Remarks:** Set the overlap

**Parameters:**
- `oddOverlapStart` (float) — Overlap length at the start for the odd ring number
- `evenOverlapStart` (float) — Overlap length at the start for the even ring number
- `bOverlapStartAsCircle` (bool) — Overlap length at the start as circle = true, as tangent = false
- `oddOverlapEnd` (float) — Overlap length at the end for the odd ring number
- `evenOverlapEnd` (float) — Overlap length at the end for the even ring number
- `bOverlapEndAsCircle` (bool) — Overlap length at the end as circle = true, as tangent = false
- `overlapLength` (float) — Overlap length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetOverlap)

#### `SetPlacePerLinearMeter(bPlacePerLinearMeter: bool)`

Set the place per linear meter state

**Remarks:** Set the place per linear meter state

**Parameters:**
- `bPlacePerLinearMeter` (bool) — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetPlacePerLinearMeter)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.__repr__)

### Properties
- `LengthFactor` (float, get/set) — Get the length factor
- `PlacePerLinearMeter` (bool, get/set) — Get the place per linear meter state

## DivideBarsParameters (enum)

Parameters for dividing engineering geometry

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/DivideBarsParameters/)

### Constructors
- `DivideBarsParameters() | DivideBarsParameters( DivideMode: eDivideMode, OverlapPosition: eLengthPosition, OverlapLength: float, GapPosition: eLengthPosition, GapLength: float, ) | DivideBarsParameters(element: DivideBarsParameters)` — Initialize

### Methods
#### `GetTrimLens() -> tuple[float, float]`

Get necessary length to lengthen/shorten bar parts

**Remarks:** Get necessary length to lengthen/shorten bar parts

**Returns:** `tuple[float, float]` — tuple(lengthen/shorten left bar part, lengthen/shorten right bar part)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/DivideBarsParameters/#NemAll_Python_Reinforcement.DivideBarsParameters.GetTrimLens)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/DivideBarsParameters/#NemAll_Python_Reinforcement.DivideBarsParameters.__repr__)

### Properties
- `DivideMode` (eDivideMode, get/set) — Get the mode of division
- `GapLength` (float, get/set) — Get the gap length
- `GapPosition` (eLengthPosition, get/set) — Get the position of gap
- `OverlapLength` (float, get/set) — Get the overlap length
- `OverlapPosition` (eLengthPosition, get/set) — Get the position of overlap

## ExtrudeBarPlacement (class)

Implementation of the extrude bar placement element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/)

### Constructors
- `ExtrudeBarPlacement() | ExtrudeBarPlacement(placement: ExtrudeBarPlacement) | ExtrudeBarPlacement( positionNumber: int, path: Path3D, profileRotation: eProfileRotation, breakElimination: bool, maxBreakAngle: float, crossBarDistance: float, concreteCoverStart: float, concreteCoverEnd: float, edgeOffsetType: eEdgeOffsetType, edgeOffsetStart: float, edgeOffsetEnd: float, barOffset: float, bendingShapeViewVector: Vector3D, )` — Initialize

### Methods
#### `AddCrossBendingShape(shape: BendingShape)`

Add a cross bending shape

**Remarks:** Add a cross bending shape

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.AddCrossBendingShape)

#### `AddLongitudinalBarProp(longitudinalBarProp: LongitudinalBarProperties)`

Add the longitudinal bar properties

**Remarks:** Add the longitudinal bar properties

**Parameters:**
- `longitudinalBarProp` (LongitudinalBarProperties) — longitudinal bar properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.AddLongitudinalBarProp)

#### `AddPlacementSection(placementSection: BarPlacementSection) -> bool`

Add a placement section

**Remarks:** Add a placement section

**Parameters:**
- `placementSection` (BarPlacementSection) — Section

**Returns:** `bool` — Section is added: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.AddPlacementSection)

#### `Extrude()`

Extrude the bars

**Remarks:** Extrude the bars

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.Extrude)

#### `GetBarOffset() -> float`

Get the bar offset

**Remarks:** Get the bar offset

**Returns:** `float` — Bar offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetBarOffset)

#### `GetBendingShapeViewVector() -> Vector3D`

Get the view vector of the bending shape

**Remarks:** Get the view vector of the bending shape

**Returns:** `Vector3D` — View vector of the bending shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetBendingShapeViewVector)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetCommonProperties)

#### `GetConcreteCoverEnd() -> float`

Get the concrete cover at the end of the path

**Remarks:** Get the concrete cover at the end of the path

**Returns:** `float` — Concrete cover at the end of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetConcreteCoverEnd)

#### `GetConcreteCoverStart() -> float`

Get the concrete cover at the start of the path

**Remarks:** Get the concrete cover at the start of the path

**Returns:** `float` — Concrete cover at the start of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetConcreteCoverStart)

#### `GetCrossBarDistance() -> float`

Get the cross bar distance

**Remarks:** Get the cross bar distance

**Returns:** `float` — Cross bar distance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetCrossBarDistance)

#### `GetCrossBendingShapes() -> BendingShapeList`

Get the cross bending shapes

**Remarks:** Get the cross bending shapes

**Returns:** `BendingShapeList` — Cross bending shapes

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetCrossBendingShapes)

#### `GetEdgeOffsetEnd() -> float`

Get the edge offset at the end of the path

**Remarks:** Get the edge offset at the end of the path

**Returns:** `float` — Edge offset at the end of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsetEnd)

#### `GetEdgeOffsetStart() -> float`

Get the edge offset at the start of the path

**Remarks:** Get the edge offset at the start of the path

**Returns:** `float` — Edge offset at the start of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsetStart)

#### `GetEdgeOffsetType() -> eEdgeOffsetType`

Get the edge offset type

**Remarks:** Get the edge offset type

**Returns:** `eEdgeOffsetType` — Edge offset type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsetType)

#### `GetEdgeOffsets() -> tuple`

Get the edge offsets

**Remarks:** Get the edge offsets

**Returns:** `tuple` — Edge offsets

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsets)

#### `GetMaxBreakAngle() -> float`

Get the maximal break angle

**Remarks:** Get the maximal break angle

**Returns:** `float` — Maximal break angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetMaxBreakAngle)

#### `GetPlacementPath() -> Path3D`

Get the placement path

**Remarks:** Get the placement path

**Returns:** `Path3D` — Placement path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetPlacementPath)

#### `GetPlacementSections() -> object`

Get the placement sections

**Remarks:** Get the placement sections

**Returns:** `object` — Placement sections

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetPlacementSections)

#### `GetPositionNumber() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetPositionNumber)

#### `GetProfileRoation() -> eProfileRotation`

Get the profile rotation

**Remarks:** Get the profile rotation

**Returns:** `eProfileRotation` — Profile rotation

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetProfileRoation)

#### `IsBreakElimination() -> bool`

Get the break eliminination state

**Remarks:** Get the break eliminination state

**Returns:** `bool` — Break elimination state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.IsBreakElimination)

#### `Move(transVec: Vector3D)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.Move)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.SetCommonProperties)

#### `Transform(transMat: Matrix3D)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.Transform)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.__repr__)

### Properties
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `PositionNumber` (int, get/set) — Get the position number

## Functions (static-class)

Module-level functions of NemAll_Python_Reinforcement

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/)

### Methods
#### `CreateReinforcementLabeling( doc: DocumentAdapter, insertionMat: Matrix3D, labelList: list, viewProj: ViewWorldProjection, undoRedoService: object | None = None, ) | CreateReinforcementLabeling( doc: DocumentAdapter, insertionMat: Matrix3D, labelList: ReinforcementLabelList, viewProj: ViewWorldProjection, )`

Create the reinforcement labels

**Remarks:** Create the reinforcement labels

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Insertion matrix
- `modelEleList` (object) — List with the model elements
- `viewProj` (ViewWorldProjection) — View projection
- `undoRedoService` (object | None) — Undo redo service

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/#NemAll_Python_Reinforcement.CreateReinforcementLabeling)

#### `InitApplicationtest() -> DocumentAdapter`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/#NemAll_Python_Reinforcement.InitApplicationtest)

#### `InitUnitTest()`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/#NemAll_Python_Reinforcement.InitUnitTest)

## GeometryExpansionUtil (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.GeometryExpansionUtil.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/)

### Constructors
- `GeometryExpansionUtil() | GeometryExpansionUtil(pMsgInfo: AddMsgInfo, use3DGeometry: bool)` — Initialize

### Methods
#### `GetLineAbove(arg2: Point2D, arg3: Line2D, arg4: bool, arg5: int) -> tuple`

Get the line above the base line and the placement point

**Remarks:** Get the line above the base line and the placement point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineAbove)

#### `GetLineAtPoint(arg2: Point2D, arg3: Vector2D, arg4: bool, arg5: float) -> tuple`

Get the line at the defined point of the reference line

**Remarks:** Get the line at the defined point of the reference line

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineAtPoint)

#### `GetLineFromPoint( arg2: DocumentAdapter, arg3: Point2D, arg4: ViewWorldProjection, arg5: bool ) -> tuple`

Get the line near to the input point

**Remarks:** Get the line near to the input point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineFromPoint)

#### `GetLineLeft(arg2: Point2D, arg3: Line2D, arg4: bool, arg5: int) -> tuple`

Get the line left from the base line and the placement point

**Remarks:** Get the line left from the base line and the placement point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineLeft)

## HookLengthService (class)

Service class for the hook length calculation

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/)

### Constructors
- `HookLengthService(norm: int, concreteGrade: int, steelGrade: int, bExactLength: bool)` — Constructor

### Methods
#### `GetHookLength(hookAngle: float, hookType: HookType, diameter: float) -> float`

Calculate the hook length

**Remarks:** Calculate the hook length

**Parameters:**
- `hookAngle` (float) — Hook angle
- `hookType` (HookType) — Hook type
- `diameter` (float) — Diameter

**Returns:** `float` — Hook length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetHookLength)

#### `GetHookLengthPartFromBendingRoller( hookAngle: float, hookType: HookType, diameter: float ) -> float`

Calculate the hook length part from the beginning of the bending roller

**Remarks:** Calculate the hook length part from the beginning of the bending roller

**Parameters:**
- `hookAngle` (float) — Hook angle
- `hookType` (HookType) — Hook type
- `diameter` (float) — Diameter

**Returns:** `float` — Hook length part from the beginning of the bending roller

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetHookLengthPartFromBendingRoller)

#### `GetHookLengthPartOfBendingRoller( hookAngle: float, hookType: HookType, diameter: float ) -> float`

Calculate the hook length part of the bending roller

**Remarks:** Calculate the hook length part of the bending roller

**Parameters:**
- `hookAngle` (float) — Hook angle
- `hookType` (HookType) — Hook type
- `diameter` (float) — Diameter

**Returns:** `float` — Hook length part of the bending roller

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetHookLengthPartOfBendingRoller)

#### `GetStandardAnchorageHookLength(diameter: float) -> float`

Calculate the standard anchorage hook length

**Remarks:** Calculate the standard anchorage hook length

**Parameters:**
- `diameter` (float) — Diameter

**Returns:** `float` — Standard anchorage hook length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetStandardAnchorageHookLength)

## HookType (enum)

Types of the hooks

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookType/)

### Values
- `eAnchorage` = `3`
- `eAngle` = `2`
- `eStirrup` = `1`

## LabelType (enum)

Types of the label

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LabelType/)

### Values
- `LabelWithComb` = `2`
- `LabelWithComb2Pointer` = `3`
- `LabelWithComb3Pointer` = `4`
- `LabelWithDimensionLine` = `1`
- `LabelWithFan` = `5`
- `LabelWithFanStartCenterEnd` = `7`
- `LabelWithFanStartEnd` = `6`
- `LabelWithPointer` = `0`

## LongitudinalBarProperties (enum)

Implementation of the longitudinal bar properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/)

### Constructors
- `LongitudinalBarProperties() | LongitudinalBarProperties( shape: BendingShape, overlappingAtStartTurnedOn: bool, overlappingAtStart: float, overlappingAtEndTurnedOn: bool, overlappingAtEnd: float, overlappingLength: float, minBarDistance: float, deliveryShapeType: eDeliveryShapeType, insideBarsState: eInsideBarsState, startLength: float, )` — Initialize

### Methods
#### `GetBendingShape() -> BendingShape`

Get the bending shape

**Remarks:** Get the bending shape

**Returns:** `BendingShape` — Bending shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetBendingShape)

#### `GetDeliveryShapeType() -> eDeliveryShapeType`

Get the delivery shape type

**Remarks:** Get the delivery shape type

**Returns:** `eDeliveryShapeType` — Delivery shape type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetDeliveryShapeType)

#### `GetInsideBarsState() -> eInsideBarsState`

Get the insid bars state

**Remarks:** Get the insid bars state

**Returns:** `eInsideBarsState` — Inside bars state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetInsideBarsState)

#### `GetMinBarDistance() -> float`

Get the minimal bar distance

**Remarks:** Get the minimal bar distance

**Returns:** `float` — Minimal bar distance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetMinBarDistance)

#### `GetOverlappingAtEnd() -> float`

Get the overlapping at end

**Remarks:** Get the overlapping at end

**Returns:** `float` — Overlapping at end

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetOverlappingAtEnd)

#### `GetOverlappingAtStart() -> float`

Get the overlapping at start

**Remarks:** Get the overlapping at start

**Returns:** `float` — Overlapping at start

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetOverlappingAtStart)

#### `GetOverlappingLength() -> float`

Get the overlapping length

**Remarks:** Get the overlapping length

**Returns:** `float` — Overlapping length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetOverlappingLength)

#### `GetStartLength() -> float`

Get the start length

**Remarks:** Get the start length

**Returns:** `float` — Start length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetStartLength)

#### `IsOverlappingAtEndTurnedOn() -> bool`

Get the overlapping at end state

**Remarks:** Get the overlapping at end state

**Returns:** `bool` — Overlapping at end state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.IsOverlappingAtEndTurnedOn)

#### `IsOverlappingAtStartTurnedOn() -> bool`

Get the overlapping at start state

**Remarks:** Get the overlapping at start state

**Returns:** `bool` — Overlapping at start state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.IsOverlappingAtStartTurnedOn)

#### `SetBendingShape(shape: BendingShape)`

Set the bending shape

**Remarks:** Set the bending shape

**Parameters:**
- `shape` (BendingShape) — Shape

**Returns:** `object` — Bending shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.SetBendingShape)

#### `__eq__(bar_props: LongitudinalBarProperties) -> bool`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `bar_props` (LongitudinalBarProperties) — Bar properties to compare

**Returns:** `bool` — Bars are equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.__eq__)

### Properties
- `BendingShape` (BendingShape, get/set) — Get the bending shape

## LongitudinalBarPropertiesList (class)

List for LongitudinalBarProperties objects

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/)

### Constructors
- `LongitudinalBarPropertiesList()` — Initialize

### Methods
#### `__contains__(value: LongitudinalBarProperties) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (LongitudinalBarProperties) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__contains__)

#### `__delitem__(value: LongitudinalBarProperties)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (LongitudinalBarProperties) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__delitem__)

#### `__eq__(compare_list: LongitudinalBarPropertiesList) -> bool`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__eq__)

#### `__getitem__(index: int) -> LongitudinalBarProperties`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `LongitudinalBarProperties` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__len__)

#### `__setitem__(index: int | slice, value: LongitudinalBarProperties)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (LongitudinalBarProperties) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__setitem__)

#### `append(value: LongitudinalBarProperties)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (LongitudinalBarProperties) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.append)

#### `extend(iterable: LongitudinalBarPropertiesList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (LongitudinalBarPropertiesList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.extend)

## MeshAreaPlacementProperties (enum)

(No description provided in vendor docs for NemAll_Python_Reinforcement.MeshAreaPlacementProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementProperties/)

### Constructors
- `MeshAreaPlacementProperties()` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementProperties/#NemAll_Python_Reinforcement.MeshAreaPlacementProperties.__repr__)

### Properties
- `LapJointOffset` (None, get) — Get/set the lap joint offset state :type: None
- `MeshSizeRound` (None, get) — Get/set the mesh size round state :type: None
- `OverlapCross` (None, get) — Get/set the cross overlap :type: None
- `OverlapLongitudinal` (None, get) — Get/set the longitudinal overlap :type: None
- `PlacementDirection` (None, get) — Get/set the placement direction :type: None
- `PlacementEndJustified` (None, get) — Get/set the placement end justified state :type: None
- `PlacementStartChange` (None, get) — Get/set the placement start change state :type: None
- `StartLength` (None, get) — Get/set the start length :type: None
- `StartWidth` (None, get) — Get/set the start width :type: None

## MeshAreaPlacementService (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.MeshAreaPlacementService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/)

### Constructors
- `MeshAreaPlacementService()` — Initialize

### Methods
#### `AddOpeningPolygon(arg2: Polygon3D, openingPol: float)`

Add an opening polygon

**Remarks:** Add an opening polygon

**Parameters:**
- `openingPol` (float) — Opening polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/#NemAll_Python_Reinforcement.MeshAreaPlacementService.AddOpeningPolygon)

#### `Calculate( arg2: DocumentAdapter, doc: MeshData, mesh: MeshAreaPlacementProperties, placementMatrix: Matrix3D, startPositionNumber: int, concreteCoverZDir: float, ) -> list`

Calculate the meshes

**Remarks:** Calculate the meshes

**Parameters:**
- `doc` (MeshData) — Document
- `mesh` (MeshAreaPlacementProperties) — Mesh data
- `placementMatrix` (Matrix3D) — Placement matrix
- `startPositionNumber` (int) — Start position number
- `concreteCoverZDir` (float) — Concrete cover in the local z direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/#NemAll_Python_Reinforcement.MeshAreaPlacementService.Calculate)

#### `SetOuterPolygon(arg2: Polygon3D, placementPol: float)`

Constructor

**Remarks:** Constructor

**Parameters:**
- `placementPol` (float) — Placement polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/#NemAll_Python_Reinforcement.MeshAreaPlacementService.SetOuterPolygon)

## MeshBendingDirection (enum)

Types of the mesh bending direction

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshBendingDirection/)

### Values
- `CrossBars` = `0`
- `LongitudinalBars` = `1`

## MeshData (class)

Implementation of the mesh data

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/)

### Constructors
- `MeshData() | MeshData( type: str, length: float, width: float, diameterLongitudinal: float, diameterCross: float, asLongitudinal: float, asCross: float, distanceLongitudinal: float, distanceCross: float, bDoubleBarLongitudinal: bool, bDoubleBarCross: bool, overlapLongitudinal: float, overlapCross: float, weight: float, ) | MeshData(element: MeshData)` — Initialize

### Methods
#### `CreateLabel()`

Create the label

**Remarks:** Create the label

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.CreateLabel)

#### `Format(type: str, length: float, width: float) -> str`

Get the mesh text

**Remarks:** Get the mesh text

**Parameters:**
- `type` (str) — Mesh type
- `length` (float) — Mesh length
- `width` (float) — Mesh width

**Returns:** `str` — Mesh text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.Format)

#### `GetAsBendingDirection(bendingDirection: MeshBendingDirection) -> float`

Get the as in bending direction

**Remarks:** Get the as in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — As in bending direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetAsBendingDirection)

#### `GetDiameterBendingDirection( bendingDirection: MeshBendingDirection, ) -> tuple[float, bool]`

Get the diameter in bending direction

**Remarks:** Get the diameter in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `tuple[float, bool]` — tuple(Diameter in bending direction, Double bar state)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetDiameterBendingDirection)

#### `GetDimensions() -> tuple[float, float]`

Get the mesh dimensions

**Remarks:** Get the mesh dimensions

**Returns:** `tuple[float, float]` — tuple(Mesh length, Mesh width)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetDimensions)

#### `GetDistanceBendingDirection(bendingDirection: MeshBendingDirection) -> float`

Get the distance in bending direction

**Remarks:** Get the distance in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — Distance in bending direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetDistanceBendingDirection)

#### `GetOverlapBendingDirection(bendingDirection: MeshBendingDirection) -> float`

Get the overlap in bending direction

**Remarks:** Get the overlap in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — Overlap in bending direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetOverlapBendingDirection)

#### `SetType(type: str)`

Set the mesh type

**Remarks:** Set the mesh type

**Parameters:**
- `type` (str) — Mesh type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.SetType)

### Properties
- `AsBendingDirection` (float, get) — Get the as in bending direction
- `AsCross` (float, get) — Get the as in cross direction
- `AsLongitudinal` (float, get) — Get the as in longitudinal direction
- `DiameterCross` (float, get) — Get the diameter in cross direction
- `DiameterLongitudinal` (float, get) — Get the diameter in longitudinal direction
- `DistanceBendingDirection` (float, get) — Get the distance in bending direction
- `DistanceCross` (float, get) — Get the distance in cross direction
- `DistanceLongitudinal` (float, get) — Get the distance in longitudinal direction
- `IsDoubleBarCross` (bool, get) — Get the double bar state in cross direction
- `IsDoubleBarLongitudinal` (bool, get) — Get the double bar state in longitudinal direction
- `Label` (str, get) — Get the mesh label
- `Length` (float, get) — Get the mesh length
- `OverlapBendingDirection` (float, get) — Get the overlap in bending direction
- `OverlapCross` (float, get) — Get the overlap in cross direction
- `OverlapLongitudinal` (float, get) — Get the overlap in longitudinal direction
- `Type` (str, get/set) — Get the mesh type
- `Weight` (float, get) — Get the mesh weight
- `Width` (float, get) — Get the mesh width

## MeshOperations (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.MeshOperations.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshOperations/)

### Constructors
- `MeshOperations()` — Initialize

### Methods
#### `CutMesh(placements: BaseElementAdapter, divisionLine: Polygon2D) -> str`

Divide the bars placement

**Remarks:** Divide the bars placement

**Parameters:**
- `placement` (object) — BaseElementAdapter with the placement
- `cutPolygon` (object) — Cut polygon

**Returns:** `str` — Result message

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshOperations/#NemAll_Python_Reinforcement.MeshOperations.CutMesh)

## MeshPlacement (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.MeshPlacement.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/)

### Constructors
- `MeshPlacement() | MeshPlacement(positionNumber: int, widthVec: Vector3D, bendingShape: BendingShape)` — Initialize

### Methods
#### `GetBendingShape() -> BendingShape`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `BendingShape` — Shape polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.GetBendingShape)

#### `GetPositionNumber() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.GetPositionNumber)

#### `GetWidthVector() -> Vector3D`

Get the width vector

**Remarks:** Get the width vector

**Returns:** `Vector3D` — Width vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.GetWidthVector)

#### `Move(transVec: Vector3D)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.Move)

#### `SetBendingShape(shape: BendingShape)`

Set the reinforcement shape

**Remarks:** Set the reinforcement shape

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.SetBendingShape)

#### `SetPositionNumber(positionNumber: int)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.SetPositionNumber)

#### `SetWidthVector(widthVec: Vector3D)`

Set the width vector

**Remarks:** Set the width vector

**Parameters:**
- `widthVec` (Vector3D) — Width vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.SetWidthVector)

#### `Transform(transMat: Matrix3D)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.Transform)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.__repr__)

## NormType (enum)

Types of the norms

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/NormType/)

### Values
- `eNORM_AS` = `8`
- `eNORM_BS` = `5`
- `eNORM_DIN` = `0`
- `eNORM_DIN_1` = `10`
- `eNORM_DIN_H` = `4`
- `eNORM_EC2` = `6`
- `eNORM_EHE` = `7`
- `eNORM_NEN` = `9`
- `eNORM_NF` = `3`
- `eNORM_OE` = `2`
- `eNORM_SIA` = `1`
- `eNORM_SNIP` = `11`
- `eNORM_SNIP2003` = `12`
- `eNormNo` = `-1`

## PlaneMeshPlacement (class)

Implementation of the mesh placement element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/)

### Constructors
- `PlaneMeshPlacement() | PlaneMeshPlacement(placement: PlaneMeshPlacement) | PlaneMeshPlacement( positionNumber: int, meshData: MeshData, meshLength: float, meshWidth: float, meshPolygon: Polygon3D, )` — Initialize

### Methods
#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetCommonProperties)

#### `GetMeshData() -> MeshData`

Get the mesh data

**Remarks:** Get the mesh data

**Returns:** `MeshData` — Mesh data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshData)

#### `GetMeshLength() -> float`

Get the mesh length

**Remarks:** Get the mesh length

**Returns:** `float` — Mesh length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshLength)

#### `GetMeshPolygon() -> Polygon3D`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `Polygon3D` — Shape polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshPolygon)

#### `GetMeshWidth() -> float`

Get the mesh width

**Remarks:** Get the mesh width

**Returns:** `float` — Mesh width

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshWidth)

#### `GetPositionNumber() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetPositionNumber)

#### `IsValid() -> bool`

Get the state of the shape

**Remarks:** Get the state of the shape

**Returns:** `bool` — Shape is valid: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.IsValid)

#### `Move(transVec: Vector3D)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.Move)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.SetCommonProperties)

#### `SetMeshPolygon(shape: Polygon3D)`

Set the reinforcement shape

**Remarks:** Set the reinforcement shape

**Parameters:**
- `shape` (Polygon3D) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.SetMeshPolygon)

#### `SetPositionNumber(positionNumber: int)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.SetPositionNumber)

#### `Transform(transMat: Matrix3D)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.Transform)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.__repr__)

### Properties
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `MeshPolygon` (Polygon3D, get/set) — Get the shape polyline
- `PositionNumber` (int, get/set) — Get the position number

## ReinfElement (class)

Abstract class representing all reinforcement placement elements, such as linear bar placement or mesh placement

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinfElement/)

## ReinforcementLabel (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabel.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/)

### Constructors
- `ReinforcementLabel() | ReinforcementLabel( reinforcementType: ReinforcementType, type: LabelType, positionNumber: int, labelProp: ReinforcementLabelProperties, labelPoint: Point2D, angle: Angle, ) | ReinforcementLabel( reinforcementType: ReinforcementType, type: LabelType, positionNumber: int, labelProp: ReinforcementLabelProperties, shapeSide: int, shapeSideFactor: float, labelOffset: Vector2D, angle: Angle, ) | ReinforcementLabel( reinforcementType: ReinforcementType, type: LabelType, positionNumber: int, labelProp: ReinforcementLabelProperties, bDimLineAtShapeStart: bool, dimLineOffset: float, ) | ReinforcementLabel( reinforcementType: ReinforcementType, type: LabelType, positionNumber: int, labelProp: ReinforcementLabelProperties, pointerProp: ReinforcementLabelPointerProperties, bDimLineAtShapeStart: bool, dimLineOffset: float, )` — Initialize

### Methods
#### `SetAdditionalText(additionalText: str)`

Set the additional text

**Remarks:** Set the additional text

**Parameters:**
- `additionalText` (str) — Additional text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetAdditionalText)

#### `SetLabelOffset(labelOffset: Vector2D)`

Set the label offset

**Remarks:** Set the label offset

**Parameters:**
- `labelOffset` (Vector2D) — Label offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetLabelOffset)

#### `SetPointerStartPoint(pointerStartPoint: Point2D)`

Set the start pointer of the text pointer

**Remarks:** Set the start pointer of the text pointer

**Parameters:**
- `pointerStartPoint` (Point2D) — Start point of the text pointer

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetPointerStartPoint)

#### `SetShowTextPointer(showTextPointer: bool)`

Set the state for showing the text pointer

**Remarks:** Set the state for showing the text pointer

**Parameters:**
- `showTextPointer` (bool) — Show the text pointer: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetShowTextPointer)

#### `SetShowTextPointerEndSymbol(showTextPointerEndSymbol: bool)`

Set the state for showing the text pointer end symbol

**Remarks:** Set the state for showing the text pointer end symbol

**Parameters:**
- `showTextPointerEndSymbol` (bool) — Show the text pointer end symbol: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetShowTextPointerEndSymbol)

#### `SetTextProperties(textProperties: TextProperties)`

Set the text properties

**Remarks:** Set the text properties

**Parameters:**
- `textProperties` (TextProperties) — Text properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetTextProperties)

#### `SetVisibleBars(visibleBars: list[int] | VecIntList)`

Set the vector with the visible bars

**Remarks:** Set the vector with the visible bars

**Parameters:**
- `visibleBars` (list[int] | VecIntList) — Vector with the visible bars: 1, 2, 3, .. index from left; -1, -2, -3, ... index from right, 0 = center

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetVisibleBars)

#### `ShowAllBars(bShowAllBars: bool)`

Set the all bars inside the dimension line, ... state

**Remarks:** Set the all bars inside the dimension line, ... state

**Parameters:**
- `bShowAllBars` (bool) — Show all bars in the dimension lines, ...: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.ShowAllBars)

## ReinforcementLabelList (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabelList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/)

### Constructors
- `ReinforcementLabelList()` — Initialize

### Methods
#### `__contains__(value: ReinforcementLabel) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ReinforcementLabel) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__contains__)

#### `__delitem__(value: ReinforcementLabel)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ReinforcementLabel) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__delitem__)

#### `__getitem__(index: int) -> ReinforcementLabel`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ReinforcementLabel` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__len__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__repr__)

#### `__setitem__(index: int | slice, value: ReinforcementLabel)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ReinforcementLabel) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__setitem__)

#### `append(value: ReinforcementLabel)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ReinforcementLabel) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.append)

#### `extend(iterable: ReinforcementLabelList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ReinforcementLabelList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.extend)

## ReinforcementLabelPointerProperties (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabelPointerProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelPointerProperties/)

### Constructors
- `ReinforcementLabelPointerProperties() | ReinforcementLabelPointerProperties(combLineAngle: float, bCombLineByLength: bool, combLineValue: float)` — Initialize

## ReinforcementLabelProperties (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabelProperties.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelProperties/)

### Constructors
- `ReinforcementLabelProperties() | ReinforcementLabelProperties(prop: ReinforcementLabelProperties)` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelProperties/#NemAll_Python_Reinforcement.ReinforcementLabelProperties.__repr__)

### Properties
- `ShowBarCount` (None, get) — Set/get the show state for the bar count :type: None
- `ShowBarDiameter` (None, get) — Set/get the show state for the bar diameter :type: None
- `ShowBarDistance` (None, get) — Set/get the show state for the bar distance :type: None
- `ShowBarLayer` (None, get) — Set/get the show state for the bar layer :type: None
- `ShowBarLength` (None, get) — Set/get the show state for the bar length :type: None
- `ShowBarPlace` (None, get) — Set/get the show state for the bar place :type: None
- `ShowBendingShape` (None, get) — Set/get the show state for the bending shape :type: None
- `ShowPositionAtEnd` (None, get) — Set/get the show state for the bar diameter :type: None
- `ShowPositionNumber` (None, get) — Set/get the show state for the position number :type: None
- `ShowSteelGrade` (None, get) — Set/get the show state for the steel grade :type: None
- `ShowTwoLineText` (None, get) — Set/get the show state for the two line text :type: None

## ReinforcementService (enum)

Reinforcement service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/)

### Methods
#### `GetACIBarMark( barsDefinitionElement: BaseElementAdapter, showIndex: bool ) -> list`

Get the bar mark

**Remarks:** Get the bar mark

**Parameters:**
- `barsDefinitionElement` (BaseElementAdapter) — Bars definition element
- `showIndex` (bool) — Show the index

**Returns:** `list` — bar mark for ACI

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetACIBarMark)

#### `GetACIPlacementBarMark( barsPlacementElement: BaseElementAdapter, showIndex: bool ) -> tuple`

Get the bar mark for a placement

**Remarks:** Get the bar mark for a placement

**Parameters:**
- `barsPlacementElement` (BaseElementAdapter) — Bars placement element
- `showIndex` (bool) — Show the index

**Returns:** `tuple` — tuple(bar mark for ACI, bar count)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetACIPlacementBarMark)

#### `GetBarPositionData(elements: BaseElementAdapterList) -> list`

Get the bar position data

**Remarks:** Get the bar position data

**Parameters:**
- `elements` (BaseElementAdapterList) — List with the elements

**Returns:** `list` — List with the bar position data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetBarPositionData)

#### `GetBarShapeCode( barsDefinitionElement: BaseElementAdapter, barShapeCopdeStandard: BarShapeCodeStandard, ) -> tuple`

Get the bar shape code

**Remarks:** Get the bar shape code

**Parameters:**
- `barsDefinitionElement` (BaseElementAdapter) — Bars definition element
- `barShapeCopdeStandard` (BarShapeCodeStandard) — Standard for the bar shape code

**Returns:** `tuple` — shape code count, list of (shape codes, bar length), list of (segment name, segment lengths)) for ACI

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetBarShapeCode)

## ReinforcementSettings (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementSettings.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/)

### Constructors
- `ReinforcementSettings()` — Initialize

### Methods
#### `CheckBarDiameter() -> float`

Check, whether the diameter is included in the diameter list of the current steel grade

**Remarks:** Check, whether the diameter is included in the diameter list of the current steel grade

**Returns:** `float` — Bar diameter (original or nearest value)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.CheckBarDiameter)

#### `CheckMeshGroup() -> int`

Check, whether the mesh group is included in the group list

**Remarks:** Check, whether the mesh group is included in the group list

**Returns:** `int` — Mesh group (original or first)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.CheckMeshGroup)

#### `GetBarDiameter() -> float`

Get the current bar diameter

**Remarks:** Get the current bar diameter

**Returns:** `float` — Bar diameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetBarDiameter)

#### `GetBarWeight(steelGrade: int, barDiameter: float) -> float`

Get the weight for a bar diameter

**Remarks:** Get the weight for a bar diameter

**Parameters:**
- `steelGrade` (int) — Steel grade
- `barDiameter` (float) — Bar diameter

**Returns:** `float` — Weight for a bar diameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetBarWeight)

#### `GetBendingRoller() -> float`

Get the current bending roller

**Remarks:** Get the current bending roller

**Returns:** `float` — Bending roller

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetBendingRoller)

#### `GetConcreteGrade() -> int`

Get the current concrete grade

**Remarks:** Get the current concrete grade

**Returns:** `int` — Concrete grade

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetConcreteGrade)

#### `GetMaxBarLength() -> float`

Get the maximal bar length

**Remarks:** Get the maximal bar length

**Returns:** `float` — Maximal bar length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetMaxBarLength)

#### `GetMeshGroup() -> int`

Get the current mesh group

**Remarks:** Get the current mesh group

**Returns:** `int` — Mesh group

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetMeshGroup)

#### `GetMeshType() -> str`

Get the current mesh type

**Remarks:** Get the current mesh type

**Returns:** `str` — Mesh type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetMeshType)

#### `GetNorm() -> int`

Get the current norm

**Remarks:** Get the current norm

**Returns:** `int` — Norm

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetNorm)

#### `GetSteelGrade() -> int`

Get the current steel grade

**Remarks:** Get the current steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetSteelGrade)

#### `Is3DReinforcement() -> bool`

Get the 3D-Reinforcement state

**Remarks:** Get the 3D-Reinforcement state

**Returns:** `bool` — Create 3D-Reinforcement: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.Is3DReinforcement)

## ReinforcementShapeBuilder (class)

Implementation of the reinforcement shape builder

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/)

### Constructors
- `ReinforcementShapeBuilder() | ReinforcementShapeBuilder(shapePlaneMatrix: Matrix3D) | ReinforcementShapeBuilder( shapePlaneMatrix: Matrix3D, create3DShape: bool, localZCoverFront: float, localZCoverBack: float, ) | ReinforcementShapeBuilder(element: ReinforcementShapeBuilder)` — Initialize

### Methods
#### `AddPoint( pnt: Point2D, concreteCover: float, bendingRoller: float, zCoordBar: float = 0, ) | AddPoint(pnt: Point3D, concreteCover: float, bendingRoller: float)`

Add an end point of a geometry side

**Remarks:** Add an end point of a geometry side

**Parameters:**
- `pnt` (Point2D) — End point of the side
- `concreteCover` (float) — Concrete cover
- `bendingRoller` (float) — Bending roller
- `zCoordBar` (float) — Bar coordinate in z direction of the local shape coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddPoint)

#### `AddPoints(pointList: object)`

Add the shape geometry points

**Remarks:** Add the shape geometry points

**Parameters:**
- `pointList` (object) — Point list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddPoints)

#### `AddSide( startPnt: Point2D, endPnt: Point2D, concreteCover: float, bendingRoller: float, zCoordBar: float = 0, ) | AddSide( startPnt: Point3D, endPnt: Point3D, concreteCover: float, bendingRoller: float, )`

Add a geometry side of the shape

**Remarks:** Add a geometry side of the shape

**Parameters:**
- `startPnt` (Point2D) — Start point of the geometry side
- `endPnt` (Point2D) — End point of the geometry side
- `concreteCover` (float) — Concrete cover
- `bendingRoller` (float) — Bending roller between the last and current side
- `zCoordBar` (float) — Bar coordinate in z direction of the local shape coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddSide)

#### `AddSides(sideList: object)`

Add the geometry sides of a shape

**Remarks:** Add the geometry sides of a shape

**Parameters:**
- `sideList` (object) — Side list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddSides)

#### `CreateShape( diameter: float, bendingRoller: float, steelGrade: int, concreteGrade: int, bendingShapeType: BendingShapeType, ) -> BendingShape | CreateShape( meshType: str, meshBendingDirection: MeshBendingDirection, bendingRoller: float, steelGrade: int, concreteGrade: int, bendingShapeType: BendingShapeType, ) -> BendingShape | CreateShape(shapeProps: object) -> BendingShape`

Create the shape

**Remarks:** Create the shape

**Parameters:**
- `diameter` (float) — Diameter
- `bendingRoller` (float) — Default bending roller
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)
- `bendingShapeType` (BendingShapeType) — Bending shape type

**Returns:** `BendingShape` — Creation successful: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.CreateShape)

#### `CreateStirrup( diameter: float, bendingRoller: float, steelGrade: int, concreteGrade: int, stirrupType: StirrupType, ) -> BendingShape | CreateStirrup( meshType: str, meshBendingDirection: MeshBendingDirection, bendingRoller: float, steelGrade: int, concreteGrade: int, stirrupType: StirrupType, ) -> BendingShape | CreateStirrup(shapeProps: object, stirrupType: StirrupType) -> BendingShape`

Create the stirrup shape

**Remarks:** Create the stirrup shape

**Parameters:**
- `diameter` (float) — Diameter
- `bendingRoller` (float) — Default bending roller
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)
- `stirrupType` (StirrupType) — Type of the stirrup

**Returns:** `BendingShape` — Creation successful: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.CreateStirrup)

#### `GetMeshData(meshType: str) -> MeshData`

Get the mesh data

**Remarks:** Get the mesh data

**Parameters:**
- `meshType` (str) — Mesh type

**Returns:** `MeshData` — Mesh data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.GetMeshData)

#### `SetAnchorageHookEnd(angle: float)`

Set an anchorage hook at the end of the shape

**Remarks:** Set an anchorage hook at the end of the shape

**Parameters:**
- `angle` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookEnd)

#### `SetAnchorageHookEndFromSide()`

Set an anchorage hook at the end of the shape, get the angle from the side

**Remarks:** Set an anchorage hook at the end of the shape, get the angle from the side

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookEndFromSide)

#### `SetAnchorageHookStart(angle: float)`

Set an anchorage hook at the start of the shape

**Remarks:** Set an anchorage hook at the start of the shape

**Parameters:**
- `angle` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookStart)

#### `SetAnchorageHookStartFromSide()`

Set an anchorage hook at the start of the shape, get the angle from the side

**Remarks:** Set an anchorage hook at the start of the shape, get the angle from the side

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookStartFromSide)

#### `SetAnchorageLengthEnd(anchorageLength: float)`

Set the anchorage length at the end of the shape

**Remarks:** Set the anchorage length at the end of the shape

**Parameters:**
- `anchorageLength` (float) — Anchorage length at the end of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageLengthEnd)

#### `SetAnchorageLengthStart(anchorageLength: float)`

Set the anchorage length at the start of the shape

**Remarks:** Set the anchorage length at the start of the shape

**Parameters:**
- `anchorageLength` (float) — Anchorage length at the start of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageLengthStart)

#### `SetConcreteCoverEnd(concreteCover: float)`

Set the concrete cover at the end of the shape

**Remarks:** Set the concrete cover at the end of the shape

**Parameters:**
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverEnd)

#### `SetConcreteCoverLineEnd( startPnt: Point2D, endPnt: Point2D, concreteCover: float ) | SetConcreteCoverLineEnd( startPnt: Point3D, endPnt: Point3D, concreteCover: float )`

Set the concrete cover line at the end of the shape

**Remarks:** Set the concrete cover line at the end of the shape

**Parameters:**
- `startPnt` (Point2D) — Start point of the concrete cover line at the end of the shape
- `endPnt` (Point2D) — Endpoint of the concrete cover line at the end of the shape
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverLineEnd)

#### `SetConcreteCoverLineStart( startPnt: Point2D, endPnt: Point2D, concreteCover: float ) | SetConcreteCoverLineStart( startPnt: Point3D, endPnt: Point3D, concreteCover: float )`

Set the concrete cover line at the start of the shape

**Remarks:** Set the concrete cover line at the start of the shape

**Parameters:**
- `startPnt` (Point2D) — Start point of the concrete cover line at the start of the shape
- `endPnt` (Point2D) — Endpoint of the concrete cover line at the start of the shape
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverLineStart)

#### `SetConcreteCoverStart(concreteCover: float)`

Set the concrete cover at the start of the shape

**Remarks:** Set the concrete cover at the start of the shape

**Parameters:**
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverStart)

#### `SetFullCircleOverlap(fullCircleOverlap: float)`

Set the overlap length for the full circle stirrup

**Remarks:** Set the overlap length for the full circle stirrup

**Parameters:**
- `fullCircleOverlap` (float) — Overlap length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetFullCircleOverlap)

#### `SetHookEnd(length: float, angle: float, type: HookType)`

Set the hook at the end of the shape

**Remarks:** Set the hook at the end of the shape

**Parameters:**
- `length` (float) — Hook length (0 = calculate)
- `angle` (float) — Hook angle
- `type` (HookType) — Hook type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetHookEnd)

#### `SetHookStart(length: float, angle: float, type: HookType)`

Set the hook at the start of the shape

**Remarks:** Set the hook at the start of the shape

**Parameters:**
- `length` (float) — Hook length (0 = calculate)
- `angle` (float) — Hook angle
- `type` (HookType) — Hook type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetHookStart)

#### `SetOverlapLengthEnd()`

Set an overlap length a the end of the shape

**Remarks:** Set an overlap length a the end of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetOverlapLengthEnd)

#### `SetOverlapLengthStart()`

Set an overlap length a the start of the shape

**Remarks:** Set an overlap length a the start of the shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetOverlapLengthStart)

#### `SetSideLengthEnd(sideLength: float)`

Set the side length at the end of the shape

**Remarks:** Set the side length at the end of the shape

**Parameters:**
- `sideLength` (float) — Side length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetSideLengthEnd)

#### `SetSideLengthStart(sideLength: float)`

Set the side length at the start of the shape

**Remarks:** Set the side length at the start of the shape

**Parameters:**
- `sideLength` (float) — Side length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetSideLengthStart)

#### `SetStartPoint(startPnt: Point2D) | SetStartPoint(startPnt: Point3D)`

Set a start point of a geometry side

**Remarks:** Set a start point of a geometry side

**Parameters:**
- `startPnt` (Point2D) — Start point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetStartPoint)

## ReinforcementType (enum)

Types of the reinforcement

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementType/)

### Values
- `Bar` = `0`
- `Mesh` = `1`

## ReinforcementUtil (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementUtil.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/)

### Constructors
- `ReinforcementUtil()` — Initialize

### Methods
#### `GetNextBarPositionNumber(doc: DocumentAdapter) -> int`

Get the the next bar position number

**Remarks:** Get the the next bar position number

**Returns:** `int` — Next bar position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/#NemAll_Python_Reinforcement.ReinforcementUtil.GetNextBarPositionNumber)

#### `GetNextMeshPositionNumber(doc: DocumentAdapter) -> int`

Get the the next mesh position number

**Remarks:** Get the the next mesh position number

**Returns:** `int` — Next mesh position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/#NemAll_Python_Reinforcement.ReinforcementUtil.GetNextMeshPositionNumber)

#### `Rearrange( doc: DocumentAdapter, fromBarPosition: int = 1, fromMeshPosition: int = 1, toBarPosition: int = 99999, toMeshPosition: int = 99999, afterBarPosition: int = 1, aftgerMeshPosition: int = 1, tolerance: float = 1.0, rearrangedLock: bool = False, identicalShapes: bool = False, identicalPrefix: bool = False, createUndoStep: bool = True, )`

Rearrange the reinforcement

**Remarks:** Rearrange the reinforcement

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `fromBarPosition` (int) — Rearrange from bar position
- `fromMeshPosition` (int) — Rearrange from mesh position
- `toBarPosition` (int) — Rearrange to bar position
- `toMeshPosition` (int) — Rearrange to mesh position
- `afterBarPosition` (int) — Rearrange after bar position
- `afterMeshPosition` (object) — Rearrange after mesh position
- `tolerance` (float) — Tolerance
- `rearrangedLock` (bool) — Rearranged positions are locked state
- `identicalShapes` (bool) — Rearrange identical shapes state
- `identicalPrefix` (bool) — Rearrange identical prefix state
- `createUndoStep` (bool) — Create the undo step state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/#NemAll_Python_Reinforcement.ReinforcementUtil.Rearrange)

## SpiralElement (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.SpiralElement.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/)

### Constructors
- `SpiralElement() | SpiralElement( positionNumber: int, diameter: float, steelGrade: int, concreteGrade: int, rotationAxis: Line3D, contourPoints: Polyline3D, pitch: float, hookLengthStart: float, hookAngleStart: float, hookLengthEnd: float, hookAngleEnd: float, concreteCoverStart: float, concreteCoverEnd: float, concreteCoverContour: float, )` — Initialize

### Methods
#### `SetLengthFactor(arg2: float)`

Get the length factor

**Remarks:** Get the length factor

**Returns:** `object` — Length factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetLengthFactor)

#### `SetNumberLoopsEnd(arg2: int)`

Set the loops at the end

**Remarks:** Set the loops at the end

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetNumberLoopsEnd)

#### `SetNumberLoopsStart(arg2: int)`

Set the loops at the start

**Remarks:** Set the loops at the start

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetNumberLoopsStart)

#### `SetPitchSections( pitch1: float, length1: float, pitch2: float, length2: float, pitch3: float, length3: float, pitch4: float, length4: float, )`

Set the pitch section

**Remarks:** Set the pitch section

**Parameters:**
- `pitch1` (float) — Pitch section 1
- `length1` (float) — Length section 1
- `pitch2` (float) — Pitch section 2
- `length2` (float) — Length section 2
- `pitch3` (float) — Pitch section 3
- `length3` (float) — Length section 3
- `pitch4` (float) — Pitch section 4
- `length4` (float) — Length section 4

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetPitchSections)

#### `SetPlacePerLinearMeter(bPlacePerMeter: bool)`

Set the place per linear meter state

**Remarks:** Set the place per linear meter state

**Parameters:**
- `bPlacePerMeter` (bool) — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetPlacePerLinearMeter)

## StirrupType (enum)

Types of the stirrups

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/StirrupType/)

### Values
- `Column` = `3`
- `Diamond` = `4`
- `FullCircle` = `5`
- `Normal` = `1`
- `Torsion` = `2`

## SweepBarPlacement (class)

Implementation of the sweep bar placement element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/)

### Constructors
- `SweepBarPlacement() | SweepBarPlacement(placement: SweepBarPlacement) | SweepBarPlacement( positionNumber: int, sweepPaths: Path3DList, rotation: bool, firstPathIsSweepPath: bool, interpolation: bool, interpolationOfAllPoints: bool, crossBarDistance: float, concreteCoverStart: float, concreteCoverEnd: float, edgeOffsetType: eEdgeOffsetType, edgeOffsetStart: float, edgeOffsetEnd: float, barOffset: float, benchingLength: float, benchingAngle: float, )` — Initialize

### Methods
#### `AddPlacementSection(placementSection: BarPlacementSection) -> bool`

Add a placement section

**Remarks:** Add a placement section

**Parameters:**
- `placementSection` (BarPlacementSection) — Section

**Returns:** `bool` — Section is added: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.AddPlacementSection)

#### `AddSectionBars( bendingShapes: BendingShapeList, sectionsLongitudinalBarsProp: LongitudinalBarPropertiesList, sectionPlane: Plane3D, )`

Add the bars and the section plane for a section bendingShapeViewVector View vector of the bending shape

**Remarks:** Add the bars and the section plane for a section bendingShapeViewVector View vector of the bending shape

**Parameters:**
- `bendingShapes` (BendingShapeList) — Bending shapes of the section
- `sectionsLongitudinalBarsProp` (LongitudinalBarPropertiesList) — Longitudinal bars properties
- `sectionPlane` (Plane3D) — Section plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.AddSectionBars)

#### `GetBarOffset() -> float`

Get the bar offset

**Remarks:** Get the bar offset

**Returns:** `float` — Bar offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetBarOffset)

#### `GetBenchingAngle() -> float`

Get the benching angle

**Remarks:** Get the benching angle

**Returns:** `float` — Benching angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetBenchingAngle)

#### `GetBenchingLength() -> float`

Get the benching length

**Remarks:** Get the benching length

**Returns:** `float` — Benching length

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetBenchingLength)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetCommonProperties)

#### `GetConcreteCoverEnd() -> float`

Get the concrete cover at the end of the path

**Remarks:** Get the concrete cover at the end of the path

**Returns:** `float` — Concrete cover at the end of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetConcreteCoverEnd)

#### `GetConcreteCoverStart() -> float`

Get the concrete cover at the start of the path

**Remarks:** Get the concrete cover at the start of the path

**Returns:** `float` — Concrete cover at the start of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetConcreteCoverStart)

#### `GetCrossBarDistance() -> float`

Get the cross bar distance

**Remarks:** Get the cross bar distance

**Returns:** `float` — Cross bar distance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetCrossBarDistance)

#### `GetEdgeOffsetEnd() -> float`

Get the edge offset at the end of the path

**Remarks:** Get the edge offset at the end of the path

**Returns:** `float` — Edge offset at the end of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsetEnd)

#### `GetEdgeOffsetStart() -> float`

Get the edge offset at the start of the path

**Remarks:** Get the edge offset at the start of the path

**Returns:** `float` — Edge offset at the start of the path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsetStart)

#### `GetEdgeOffsetType() -> eEdgeOffsetType`

Get the edge offset type

**Remarks:** Get the edge offset type

**Returns:** `eEdgeOffsetType` — Edge offset type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsetType)

#### `GetEdgeOffsets() -> tuple`

Get the edge offsets

**Remarks:** Get the edge offsets

**Returns:** `tuple` — Edge offsets

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsets)

#### `GetPlacementSections() -> object`

Get the placement sections

**Remarks:** Get the placement sections

**Returns:** `object` — Placement sections

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetPlacementSections)

#### `GetPositionNumber() -> int`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetPositionNumber)

#### `IsFirstPathIsSweepPath() -> bool`

Get the first path is sweep path state

**Remarks:** Get the first path is sweep path state

**Returns:** `bool` — First path is sweep path state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsFirstPathIsSweepPath)

#### `IsInterpolation() -> bool`

Get the interpolation state

**Remarks:** Get the interpolation state

**Returns:** `bool` — Interpolation state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsInterpolation)

#### `IsInterpolationOfAllPoints() -> bool`

Get the interpolation of all points state

**Remarks:** Get the interpolation of all points state

**Returns:** `bool` — Interpolation of all points state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsInterpolationOfAllPoints)

#### `IsRoation() -> bool`

Get the rotation state

**Remarks:** Get the rotation state

**Returns:** `bool` — Rotation state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsRoation)

#### `Move(transVec: Vector3D)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.Move)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.SetCommonProperties)

#### `SetPositionNumber(positionNumber: int)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.SetPositionNumber)

#### `Sweep()`

Sweep the bars

**Remarks:** Sweep the bars

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.Sweep)

#### `Transform(transMat: Matrix3D)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.Transform)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.__repr__)

### Properties
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `PositionNumber` (int, get/set) — Get the position number

