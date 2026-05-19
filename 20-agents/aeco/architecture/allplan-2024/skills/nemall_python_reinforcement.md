---
name: allplan-nemall_python_reinforcement
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_Reinforcement namespace — 44 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnchorageLengthService, AllplanElement, AnchorageType, BarAreaPlacementService, BarAreaPlacementProperties, BarPlacementSection, BarPlacement, BarPositionData, and 36 more types.
---

# NemAll_Python_Reinforcement

Auto-generated from vendor docs for allplan 2024.0. 44 types in this namespace.

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/)

### Methods
#### `GetAttributes()`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetAttributes)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetCommonProperties)

#### `GetGeometryObject()`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetGeometryObject)

#### `GetLabelElements()`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetLabelElements)

#### `GetSubElementID()`

Get the sub element ID

**Remarks:** Get the sub element ID

**Returns:** `type` — Sub Element ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AllplanElement/#NemAll_Python_Reinforcement.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## AnchorageLengthService (class)

Service class for the anchorage length calculation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/)

### Constructors
- `AnchorageLengthService()` — Initialize

### Methods
#### `Calculate(concreteGrade, steelGrade, diameter, asMesh, bDoubleBar, meshBarDistCross, bMesh, barDistance, roundLength)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.Calculate)

#### `CalculateBar(concreteGrade, steelGrade, diameter, bDoubleBar, barDistance, roundLength)`

Calculation of the anchorage length for a bar

**Remarks:** Calculation of the anchorage length for a bar

**Parameters:**
- `concreteGrade` (int) — Concrete grade index (starting from 1)
- `steelGrade` (int) — Steel grade
- `diameter` (float) — Diameter
- `bDoubleBar` (bool) — Double bar
- `barDistance` (float) — Bar distance
- `roundLength` (float) — Rounding length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.CalculateBar)

#### `GetAnchorageLength()`

Get the anchorage length

**Remarks:** Get the anchorage length

**Returns:** `float` — Anchorage length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetAnchorageLength)

#### `GetAnchorageType()`

Get the anchorage type

**Remarks:** Get the anchorage type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetAnchorageType)

#### `GetAsFactor()`

Get the as factor required / available

**Remarks:** Get the as factor required / available

**Returns:** `float` — As mesh factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetAsFactor)

#### `GetCompositionZone()`

Get the composition zone

**Remarks:** Get the composition zone

**Returns:** `int` — Composition zone

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetCompositionZone)

#### `GetHookAngle()`

Get the hook angle

**Remarks:** Get the hook angle

**Returns:** `float` — Hook angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetHookAngle)

#### `GetL1()`

Get length L1

**Remarks:** Get length L1

**Returns:** `float` — Length L1

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetL1)

#### `GetL2()`

Get length L2

**Remarks:** Get length L2

**Returns:** `float` — Length L2

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetL2)

#### `GetL3()`

Get length L3

**Remarks:** Get length L3

**Returns:** `float` — Length L3

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetL3)

#### `GetLongitudinalOffset()`

Get the longitudinal offset

**Remarks:** Get the longitudinal offset

**Returns:** `float` — Longitudinal offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetLongitudinalOffset)

#### `GetOverlapLength()`

Get the overlap length

**Remarks:** Get the overlap length

**Returns:** `float` — Overlap length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.GetOverlapLength)

#### `IsCompressionBar()`

Get the compression bar state

**Remarks:** Get the compression bar state

**Returns:** `bool` — Compression bar: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.IsCompressionBar)

#### `SetAnchorageType(anchorageType)`

Set the anchorage type

**Remarks:** Set the anchorage type

**Parameters:**
- `anchorageType` (AnchorageType) — Anchorage type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetAnchorageType)

#### `SetAsFactor(AsFactor)`

Set the as factor required / available

**Remarks:** Set the as factor required / available

**Parameters:**
- `AsFactor` (float) — As facto required / availabler

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetAsFactor)

#### `SetCompositionZone(compositionZone)`

Set the composition zone

**Remarks:** Set the composition zone

**Parameters:**
- `compositionZone` (int) — Composition zone

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetCompositionZone)

#### `SetCompressionBar(bCompressionBar)`

Set the compression bar state

**Remarks:** Set the compression bar state

**Parameters:**
- `bCompressionBar` (bool) — Compression bar: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetCompressionBar)

#### `SetHookAngle(hookAngle)`

Set the hook angle

**Remarks:** Set the hook angle

**Parameters:**
- `hookAngle` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetHookAngle)

#### `SetLongitudinalOffset(longitudinalOffset)`

Set the longitudinal offset

**Remarks:** Set the longitudinal offset

**Parameters:**
- `longitudinalOffset` (float) — longitudinal offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageLengthService/#NemAll_Python_Reinforcement.AnchorageLengthService.SetLongitudinalOffset)

## AnchorageType (enum)

Types of the anchorage

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `AnchorageType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/AnchorageType/#NemAll_Python_Reinforcement.AnchorageType.__getitem__)

### Values
- `eAnchorageHook` = `2`
- `eAnchorageHookOneCrossBar` = `4`
- `eAnchorageStraight` = `1`
- `eAnchorageStraightOneCrossBar` = `3`
- `eAnchorageStraightTwoCrossBars` = `5`

## BarAreaPlacementProperties (enum)

(No description provided in vendor docs for NemAll_Python_Reinforcement.BarAreaPlacementProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementProperties/)

### Constructors
- `BarAreaPlacementProperties() | BarAreaPlacementProperties(diameter, distance, overlapping, isMoveOverlapping, maxBarLength, startBarLength, maxPlacementLength, firstBarEdgeDistance, placementStrategy, benching, benchingLength, isPolygonalPlacement)` — Initialize

## BarAreaPlacementService (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.BarAreaPlacementService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/)

### Constructors
- `BarAreaPlacementService()` — Initialize

### Methods
#### `AddOpeningPolygon(arg2, openingPol)`

Add an opening polygon

**Remarks:** Add an opening polygon

**Parameters:**
- `openingPol` (float) — Opening polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/#NemAll_Python_Reinforcement.BarAreaPlacementService.AddOpeningPolygon)

#### `Calculate(doc, barPlacementProp, placementMatrix, concreteCoverZDir)`

Calculate the meshes

**Remarks:** Calculate the meshes

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `barPlacementProp` (BarAreaPlacementProperties) — Placement properties
- `placementMatrix` (Matrix3D) — Placement matrix
- `concreteCoverZDir` (float) — Concrete cover in the local z direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/#NemAll_Python_Reinforcement.BarAreaPlacementService.Calculate)

#### `SetOuterPolygon(arg2, placementPol)`

Constructor

**Remarks:** Constructor

**Parameters:**
- `placementPol` (float) — Placement polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarAreaPlacementService/#NemAll_Python_Reinforcement.BarAreaPlacementService.SetOuterPolygon)

## BarPlacement (class)

Implementation of the bar placement element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/)

### Constructors
- `BarPlacement() | BarPlacement(placement) | BarPlacement(positionNumber, barCount, distVec, startPnt, endPnt, bendingShape) | BarPlacement(positionNumber, barCount, startBendingShape, endBendingShape) | BarPlacement(positionNumber, barCount, rotationAxis, rotationAngle, bendingShape)` — Initialize

### Methods
#### `GetBarCount()`

Get the bar count

**Remarks:** Get the bar count

**Returns:** `int` — Bar count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetBarCount)

#### `GetBendingShape()`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `BendingShape` — Shape polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetBendingShape)

#### `GetBendingShapeMatrix()`

Get the bending shape matrix

**Remarks:** Get the bending shape matrix

**Returns:** `Matrix3D` — Bending shape matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetBendingShapeMatrix)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetCommonProperties)

#### `GetDistanceVector()`

Get the distance vector

**Remarks:** Get the distance vector

**Returns:** `Vector3D` — Distance vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetDistanceVector)

#### `GetEndBendingShape()`

Get the shape polyline at the end of a polygonal placement

**Remarks:** Get the shape polyline at the end of a polygonal placement

**Returns:** `BendingShape` — Shape polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetEndBendingShape)

#### `GetEndPoint()`

Get the end point of the placement at the placement line

**Remarks:** Get the end point of the placement at the placement line

**Returns:** `Point3D` — End point of the placement at the placement line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetEndPoint)

#### `GetLabel()`

Get the label

**Remarks:** Get the label

**Returns:** `ReinforcementLabel` — Label

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetLabel)

#### `GetLengthFactor()`

Get the length factor

**Remarks:** Get the length factor

**Returns:** `float` — Length factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetLengthFactor)

#### `GetPositionNumber()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetPositionNumber)

#### `GetRotationAngle()`

Get the rotation angle

**Remarks:** Get the rotation angle

**Returns:** `Angle` — Rotation angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetRotationAngle)

#### `GetRotationAxis()`

Get the rotation axis

**Remarks:** Get the rotation axis

**Returns:** `Line3D` — Rotation axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetRotationAxis)

#### `GetStartPoint()`

Get start point of the placement at the placement line

**Remarks:** Get start point of the placement at the placement line

**Returns:** `Point3D` — Start point of the placement at the placement line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.GetStartPoint)

#### `IsPlacePerLinearMeter()`

Get the place per linear meter state

**Remarks:** Get the place per linear meter state

**Returns:** `bool` — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.IsPlacePerLinearMeter)

#### `IsPolygonalPlacement()`

Get the polygonal placement state

**Remarks:** Get the polygonal placement state

**Returns:** `bool` — Polygonal placement: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.IsPolygonalPlacement)

#### `IsRotationalPlacement()`

Get the rotational placement state

**Remarks:** Get the rotational placement state

**Returns:** `bool` — Rotational placement: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.IsRotationalPlacement)

#### `Move(transVec)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.Move)

#### `SetBendingShape(shape)`

Set the reinforcement shape

**Remarks:** Set the reinforcement shape

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetBendingShape)

#### `SetBendingShapeMatrix(bendingShapeMat)`

Set the bending shape matrix

**Remarks:** Set the bending shape matrix

**Parameters:**
- `bendingShapeMat` (Matrix3D) — Bending shape matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetBendingShapeMatrix)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetCommonProperties)

#### `SetDistanceVector(distVec)`

Set the distance vector

**Remarks:** Set the distance vector

**Parameters:**
- `distVec` (Vector3D) — Distance vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetDistanceVector)

#### `SetEndBendingShape(shape)`

Set the reinforcement shape at the end

**Remarks:** Set the reinforcement shape at the end

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetEndBendingShape)

#### `SetLabel(label, labelAssocView)`

Set the label

**Remarks:** Set the label

**Parameters:**
- `label` (ReinforcementLabel) — Label
- `labelAssocView` (AssocViewElementAdapter) — Associative view for the label

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetLabel)

#### `SetLengthFactor(lengthFactor)`

Set the length factor

**Remarks:** Set the length factor

**Parameters:**
- `lengthFactor` (float) — Length factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetLengthFactor)

#### `SetPlacePerLinearMeter(bPlacePerLinearMeter)`

Set the place per linear meter state

**Remarks:** Set the place per linear meter state

**Parameters:**
- `bPlacePerLinearMeter` (bool) — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetPlacePerLinearMeter)

#### `SetPositionNumber(positionNumber)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetPositionNumber)

#### `SetRotationAxis(rotationAxis)`

Set the rotation axis

**Remarks:** Set the rotation axis

**Parameters:**
- `rotationAxis` (Line3D) — Rotation axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.SetRotationAxis)

#### `Transform(transMat)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.Transform)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacement/#NemAll_Python_Reinforcement.BarPlacement.__repr__)

### Properties
- `BendingShape` (BendingShape, get/set) — Get the shape polyline
- `BendingShapeMatrix` (NemAll_Python_Geometry.Matrix3D, get/set) — Get the bending shape matrix
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `DistanceVector` (NemAll_Python_Geometry.Vector3D, get/set) — Get the distance vector
- `EndBendingShape` (BendingShape, get/set) — Get the shape polyline at the end of a polygonal placement
- `LengthFactor` (float, get/set) — Get the length factor
- `PlacePerLinearMeter` (bool, get/set) — Get the place per linear meter state
- `PositionNumber` (int, get/set) — Get the position number
- `RotationAxis` (NemAll_Python_Geometry.Line3D, get/set) — Get the rotation axis

## BarPlacementSection (class)

Implementation of the bar placement section class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/)

### Constructors
- `BarPlacementSection(isEnabled, length, distance) | BarPlacementSection(element)` — Constructor

### Methods
#### `GetDistance()`

Get the distance

**Remarks:** Get the distance

**Returns:** `float` — Distance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/#NemAll_Python_Reinforcement.BarPlacementSection.GetDistance)

#### `GetLength()`

Get the length

**Remarks:** Get the length

**Returns:** `float` — Length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/#NemAll_Python_Reinforcement.BarPlacementSection.GetLength)

#### `IsEnabled()`

Get the enabled state

**Remarks:** Get the enabled state

**Returns:** `bool` — Enable state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPlacementSection/#NemAll_Python_Reinforcement.BarPlacementSection.IsEnabled)

## BarPositionData (class)

Implementation of the bar position data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/)

### Constructors
- `BarPositionData(element)` — Copy constructor

### Methods
#### `GetCount()`

Get the count

**Remarks:** Get the count

**Returns:** `int` — Count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetCount)

#### `GetDiameter()`

Get the diameter

**Remarks:** Get the diameter

**Returns:** `float` — diameter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetDiameter)

#### `GetLength()`

Get the length

**Remarks:** Get the length

**Returns:** `float` — length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetLength)

#### `GetPosition()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetPosition)

#### `GetSteelGrade()`

Get the steel grade

**Remarks:** Get the steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.GetSteelGrade)

#### `SetCount(count)`

Set the count

**Remarks:** Set the count

**Parameters:**
- `count` (int) — Count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetCount)

#### `SetDiameter(diameter)`

Set the diameter

**Remarks:** Set the diameter

**Parameters:**
- `diameter` (float) — diameter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetDiameter)

#### `SetLength(length)`

Set the length

**Remarks:** Set the length

**Parameters:**
- `length` (float) — length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetLength)

#### `SetPosition(position)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `position` (int) — Position

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetPosition)

#### `SetSteelGrade(steelGrade)`

Set the steel grade

**Remarks:** Set the steel grade

**Parameters:**
- `steelGrade` (int) — steel grade

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.SetSteelGrade)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarPositionData/#NemAll_Python_Reinforcement.BarPositionData.__repr__)

### Properties
- `Count` (int, get/set) — Get the count
- `Diameter` (float, get/set) — Get the diameter
- `Length` (float, get/set) — Get the length
- `Position` (int, get/set) — Get the position number
- `SteelGrade` (int, get/set) — Get the steel grade

## BarsOperations (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.BarsOperations.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarsOperations/)

### Constructors
- `BarsOperations()` — Initialize

### Methods
#### `DivideBarsPlacement(placement, divisionPolyline)`

Divide the bars placement

**Remarks:** Divide the bars placement

**Parameters:**
- `placement` (DivideBarsParameters) — BaseElementAdapter with the placement
- `divideBarsParameter` (object) — Divide bars parameters
- `divisionPolyline` (Polyline2D) — Divison polyline

**Returns:** `str` — Result message

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarsOperations/#NemAll_Python_Reinforcement.BarsOperations.DivideBarsPlacement)

#### `JoinBarsPlacements(placement, fillEdges)`

Join the bars placements

**Remarks:** Join the bars placements

**Parameters:**
- `placements` (object) — BaseElementAdapterList with the placements
- `fillEdges` (bool) — Fill the edges: True/False

**Returns:** `str` — Result message

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BarsOperations/#NemAll_Python_Reinforcement.BarsOperations.JoinBarsPlacements)

## BendingRollerService (class)

Service class for the bending roller calculation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/)

### Methods
#### `GetBendBendingRollerFactor(diameter, steelGrade, concreteGrade)`

Get the bend bending roller factor

**Remarks:** Get the bend bending roller factor

**Parameters:**
- `diameter` (float) — Diameter
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade

**Returns:** `float` — Bending roller factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetBendBendingRollerFactor)

#### `GetBendingRoller(diameter, steelGrade, concreteGrade, bStirrup)`

Get the bending roller

**Remarks:** Get the bending roller

**Parameters:**
- `diameter` (float) — Diameter
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade
- `bStirrup` (bool) — Shape is a stirrup: true/false

**Returns:** `float` — Bending roller factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetBendingRoller)

#### `GetBendingRollerFactor(diameter, steelGrade, concreteGrade, bStirrup)`

Get the bending roller factor

**Remarks:** Get the bending roller factor

**Parameters:**
- `diameter` (float) — Diameter
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade
- `bStirrup` (bool) — Shape is a stirrup: true/false

**Returns:** `float` — Bending roller factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetBendingRollerFactor)

#### `GetDefaultBendingRollers(norm)`

Get the default bending rollers

**Remarks:** Get the default bending rollers

**Parameters:**
- `norm` (NormType) — Norm

**Returns:** `VecDoubleList` — Default bending rollers

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingRollerService/#NemAll_Python_Reinforcement.BendingRollerService.GetDefaultBendingRollers)

## BendingShape (class)

Implementation of the reinforcement shape

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/)

### Constructors
- `BendingShape() | BendingShape(shapePol, bendingRoller, diameter, steelGrade, concreteGrade, bendingShapeType) | BendingShape(shapePoint, diameter, steelGrade, concreteGrade) | BendingShape(shapePol, bendingRoller, meshType, meshBendingDirection, steelGrade, concreteGrade, bendingShapeType) | BendingShape(shapePol, bendingRoller, diameter, steelGrade, concreteGrade, bendingShapeType, hookLengthStart, hookAngleStart, hookTypeStart, hookLengthEnd, hookAngleEnd, hookTypeEnd) | BendingShape(shapePol, bendingRoller, meshType, meshBendingDirection, steelGrade, concreteGrade, bendingShapeType, hookLengthStart, hookAngleStart, hookTypeStart, hookLengthEnd, hookAngleEnd, hookTypeEnd) | BendingShape(element)` — Initialize

### Methods
#### `GetBendingRoller()`

Get the bending roller

**Remarks:** Get the bending roller

**Returns:** `VecDoubleList` — Bending roller

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetBendingRoller)

#### `GetBendingShapeType()`

Get the bending shape type

**Remarks:** Get the bending shape type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetBendingShapeType)

#### `GetConcreteGrade()`

Get the concrete grade

**Remarks:** Get the concrete grade

**Returns:** `int` — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetConcreteGrade)

#### `GetDiameter()`

Get the diameter

**Remarks:** Get the diameter

**Returns:** `float` — Diameter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetDiameter)

#### `GetHookAngleEnd()`

Get the hook angle a the end of the shape

**Remarks:** Get the hook angle a the end of the shape

**Returns:** `float` — Hook angle at the end of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookAngleEnd)

#### `GetHookAngleStart()`

Get the hook angle a the start of the shape

**Remarks:** Get the hook angle a the start of the shape

**Returns:** `float` — Hook angle at the start of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookAngleStart)

#### `GetHookLengthEnd()`

Get the hook length a the end of the shape

**Remarks:** Get the hook length a the end of the shape

**Returns:** `float` — Hook length at the end of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookLengthEnd)

#### `GetHookLengthStart()`

Get the hook length a the start of the shape

**Remarks:** Get the hook length a the start of the shape

**Returns:** `float` — Hook length at the start of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookLengthStart)

#### `GetHookTypeEnd()`

Get the hook type a the end of the shape

**Remarks:** Get the hook type a the end of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookTypeEnd)

#### `GetHookTypeStart()`

Get the hook type a the start of the shape

**Remarks:** Get the hook type a the start of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetHookTypeStart)

#### `GetMeshBendingDirection()`

Get the mesh bending direction

**Remarks:** Get the mesh bending direction

**Returns:** `MeshBendingDirection` — Mesh bending direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetMeshBendingDirection)

#### `GetMeshType()`

Get the mesh type

**Remarks:** Get the mesh type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetMeshType)

#### `GetShapePolyline()`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `Polyline3D` — Shape polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetShapePolyline)

#### `GetSteelGrade()`

Get the steel grade

**Remarks:** Get the steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.GetSteelGrade)

#### `IsValid()`

Get the state of the shape

**Remarks:** Get the state of the shape

**Returns:** `bool` — Shape is valid: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.IsValid)

#### `Move(transVec)`

Move the shape

**Remarks:** Move the shape

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.Move)

#### `Rotate(modelAngles, refPnt) | Rotate(modelAngles)`

Rotate the shape

**Remarks:** Rotate the shape

**Parameters:**
- `modelAngles` (object) — Model angles
- `refPnt` (Point3D) — Reference point of the rotation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.Rotate)

#### `SetBendingRoller(bendingRoller)`

Set the bending roller

**Remarks:** Set the bending roller

**Parameters:**
- `bendingRoller` (VecDoubleList) — Bending roller

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetBendingRoller)

#### `SetHookLengthEnd(hookLengthEnd)`

Set the end length of the hook

**Remarks:** Set the end length of the hook

**Parameters:**
- `hookLengthEnd` (float) — End length of the hook

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookLengthEnd)

#### `SetHookLengthStart(hookLengthStart)`

Set the start length of the hook

**Remarks:** Set the start length of the hook

**Parameters:**
- `hookLengthStart` (float) — Start length of the hook

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetHookLengthStart)

#### `SetShapePolyline(shapePol)`

Set the shape polyline

**Remarks:** Set the shape polyline

**Parameters:**
- `shapePol` (Polyline3D) — Shape polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.SetShapePolyline)

#### `Transform(transMat)`

Transform the shape

**Remarks:** Transform the shape

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.Transform)

#### `__eq__(shape)`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `shape` (BendingShape) — Shape to compare

**Returns:** `bool` — Shapes are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShape/#NemAll_Python_Reinforcement.BendingShape.__repr__)

### Properties
- `BendingRoller` (list[float], get/set) — Get the bending roller
- `HookLengthEnd` (float, get/set) — Get the hook length a the end of the shape
- `HookLengthStart` (float, get/set) — Get the hook length a the start of the shape
- `ShapePolyline` (NemAll_Python_Geometry.Polyline3D, get/set) — Get the shape polyline

## BendingShapeList (class)

List for BendingShape objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/)

### Constructors
- `BendingShapeList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (BendingShape) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (BendingShape) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `BendingShape` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (BendingShape) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (BendingShape) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (BendingShapeList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeList/#NemAll_Python_Reinforcement.BendingShapeList.extend)

## BendingShapeType (enum)

Type of the bending shape

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `BendingShapeType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/BendingShapeType/#NemAll_Python_Reinforcement.BendingShapeType.__getitem__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/)

### Constructors
- `CircularAreaElement() | CircularAreaElement(positionNumber, diameter, steelGrade, concreteGrade, rotationAxis, contourPoints, outerAngleStart, outerAngleEnd, innerAngleStart, innerAngleEnd, concreteCoverStart, concreteCoverEnd, concreteCoverContour) | CircularAreaElement(element)` — Initialize

### Methods
#### `GetConcreteCoverContour()`

Get the concrete cover from the contour

**Remarks:** Get the concrete cover from the contour

**Returns:** `float` — Concrete cover from the contour

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteCoverContour)

#### `GetConcreteCoverEnd()`

Get the concrete cover from the end

**Remarks:** Get the concrete cover from the end

**Returns:** `float` — Concrete cover from the end

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteCoverEnd)

#### `GetConcreteCoverStart()`

Get the concrete cover from the start

**Remarks:** Get the concrete cover from the start

**Returns:** `float` — Concrete cover from the start

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteCoverStart)

#### `GetConcreteGrade()`

Get the concrete grade

**Remarks:** Get the concrete grade

**Returns:** `int` — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetConcreteGrade)

#### `GetContourPoints()`

Get the contour points

**Remarks:** Get the contour points

**Returns:** `Polyline3D` — Contour points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetContourPoints)

#### `GetDiameter()`

Get the diameter

**Remarks:** Get the diameter

**Returns:** `float` — Diameter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetDiameter)

#### `GetDistance()`

Get the distance

**Remarks:** Get the distance

**Returns:** `float` — Distance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetDistance)

#### `GetEvenFirstLength()`

Get the first length for the even ring number

**Remarks:** Get the first length for the even ring number

**Returns:** `float` — First length for the even ring number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetEvenFirstLength)

#### `GetEvenOverlapEnd()`

Get the overlap length at the end for the even ring number

**Remarks:** Get the overlap length at the end for the even ring number

**Returns:** `float` — Overlap length at the end for the even ring number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetEvenOverlapEnd)

#### `GetEvenOverlapStart()`

Get the overlap length at the start for the even ring number

**Remarks:** Get the overlap length at the start for the even ring number

**Returns:** `float` — Overlap length at the start for the even ring number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetEvenOverlapStart)

#### `GetLengthFactor()`

Get the length factor

**Remarks:** Get the length factor

**Returns:** `float` — Length factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetLengthFactor)

#### `GetMaxBarLength()`

Get the maximal bar length

**Remarks:** Get the maximal bar length

**Returns:** `float` — Maximal bar length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMaxBarLength)

#### `GetMaxBarRise()`

Get the maximal bar radius

**Remarks:** Get the maximal bar radius

**Returns:** `float` — Maximal bar radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMaxBarRise)

#### `GetMinBarLength()`

Get the minimal bar length

**Remarks:** Get the minimal bar length

**Returns:** `float` — Minimal bar length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMinBarLength)

#### `GetMinBarRadius()`

Get the minimal bar radius

**Remarks:** Get the minimal bar radius

**Returns:** `float` — Minimal bar radius

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetMinBarRadius)

#### `GetOddFirstLength()`

Get the first length for the odd ring number

**Remarks:** Get the first length for the odd ring number

**Returns:** `float` — First length for the odd ring number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOddFirstLength)

#### `GetOddOverlapEnd()`

Get the overlap length at the end for the odd ring number

**Remarks:** Get the overlap length at the end for the odd ring number

**Returns:** `float` — Overlap length at the end for the odd ring number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOddOverlapEnd)

#### `GetOddOverlapStart()`

Get the overlap length at the start for the even ring number

**Remarks:** Get the overlap length at the start for the even ring number

**Returns:** `float` — Overlap length at the start for the odd ring number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOddOverlapStart)

#### `GetOuterAngleEnd()`

Get the outer angle at the end

**Remarks:** Get the outer angle at the end

**Returns:** `float` — Outer angle at the end

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOuterAngleEnd)

#### `GetOuterAngleStart()`

Get the outer angle at the start

**Remarks:** Get the outer angle at the start

**Returns:** `float` — Outer angle at the start

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOuterAngleStart)

#### `GetOverlapLength()`

Get the overlap length

**Remarks:** Get the overlap length

**Returns:** `float` — Overlap length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetOverlapLength)

#### `GetPlacementRule()`

Get the placement rule

**Remarks:** Get the placement rule

**Returns:** `int` — Placement rule

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetPlacementRule)

#### `GetPositionNumber()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetPositionNumber)

#### `GetRotationAxis()`

Get the rotation axis

**Remarks:** Get the rotation axis

**Returns:** `Line3D` — Rotation axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetRotationAxis)

#### `GetSteelGrade()`

Get the steel grade

**Remarks:** Get the steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetSteelGrade)

#### `GetinnerAngleEnd()`

Get the inner angle at the end

**Remarks:** Get the inner angle at the end

**Returns:** `float` — Inner angle at the end

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetinnerAngleEnd)

#### `GetinnerAngleStart()`

Get the inner angle at the start

**Remarks:** Get the inner angle at the start

**Returns:** `float` — Inner angle at the start

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.GetinnerAngleStart)

#### `IsPlacePerLinearMeter()`

Get the place per linear meter state

**Remarks:** Get the place per linear meter state

**Returns:** `bool` — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.IsPlacePerLinearMeter)

#### `IsbOverlapEndAsCircle()`

Get the overlap state at the end

**Remarks:** Get the overlap state at the end

**Returns:** `bool` — Overlap length at the end as circle = true, as tangent = false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.IsbOverlapEndAsCircle)

#### `IsbOverlapStartAsCircle()`

Get the overlap state at the start

**Remarks:** Get the overlap state at the start

**Returns:** `bool` — Overlap length at the start as circle = true, as tangent = false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.IsbOverlapStartAsCircle)

#### `SetBarProperties(distance, maxBarLength, minBarLength, placementRule, oddFirstLength, evenFirstLength, minBarRadius, maxBarRise)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetBarProperties)

#### `SetLengthFactor(lengthFactor)`

Set the length factor

**Remarks:** Set the length factor

**Parameters:**
- `lengthFactor` (float) — Length factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetLengthFactor)

#### `SetOverlap(oddOverlapStart, evenOverlapStart, bOverlapStartAsCircle, oddOverlapEnd, evenOverlapEnd, bOverlapEndAsCircle, overlapLength)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetOverlap)

#### `SetPlacePerLinearMeter(bPlacePerLinearMeter)`

Set the place per linear meter state

**Remarks:** Set the place per linear meter state

**Parameters:**
- `bPlacePerLinearMeter` (bool) — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.SetPlacePerLinearMeter)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/CircularAreaElement/#NemAll_Python_Reinforcement.CircularAreaElement.__repr__)

### Properties
- `LengthFactor` (float, get/set) — Get the length factor
- `PlacePerLinearMeter` (bool, get/set) — Get the place per linear meter state

## DivideBarsParameters (enum)

Parameters for dividing engineering geometry

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/DivideBarsParameters/)

### Constructors
- `DivideBarsParameters() | DivideBarsParameters(DivideMode, OverlapPosition, OverlapLength, GapPosition, GapLength) | DivideBarsParameters(element)` — Initialize

### Methods
#### `GetTrimLens()`

Get necessary length to lengthen/shorten bar parts

**Remarks:** Get necessary length to lengthen/shorten bar parts

**Returns:** `float` — tuple(lengthen/shorten left bar part,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/DivideBarsParameters/#NemAll_Python_Reinforcement.DivideBarsParameters.GetTrimLens)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/DivideBarsParameters/#NemAll_Python_Reinforcement.DivideBarsParameters.__repr__)

### Properties
- `DivideMode` (DivideBarsParameters.eDivideMode, get/set) — Get the mode of division
- `GapLength` (float, get/set) — Get the gap length
- `GapPosition` (DivideBarsParameters.eLengthPosition, get/set) — Get the position of gap
- `OverlapLength` (float, get/set) — Get the overlap length
- `OverlapPosition` (DivideBarsParameters.eLengthPosition, get/set) — Get the position of overlap

## ExtrudeBarPlacement (class)

Implementation of the extrude bar placement element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/)

### Constructors
- `ExtrudeBarPlacement() | ExtrudeBarPlacement(placement) | ExtrudeBarPlacement(positionNumber, path, profileRotation, breakElimination, maxBreakAngle, crossBarDistance, concreteCoverStart, concreteCoverEnd, edgeOffsetType, edgeOffsetStart, edgeOffsetEnd, barOffset, bendingShapeViewVector)` — Initialize

### Methods
#### `AddCrossBendingShape(shape)`

Add a cross bending shape

**Remarks:** Add a cross bending shape

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.AddCrossBendingShape)

#### `AddLongitudinalBarProp(longitudinalBarProp)`

Add the longitudinal bar properties

**Remarks:** Add the longitudinal bar properties

**Parameters:**
- `longitudinalBarProp` (LongitudinalBarProperties) — longitudinal bar properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.AddLongitudinalBarProp)

#### `AddPlacementSection(placementSection)`

Add a placement section

**Remarks:** Add a placement section

**Parameters:**
- `placementSection` (BarPlacementSection) — Section

**Returns:** `bool` — Section is added: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.AddPlacementSection)

#### `Extrude()`

Extrude the bars

**Remarks:** Extrude the bars

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.Extrude)

#### `GetBarOffset()`

Get the bar offset

**Remarks:** Get the bar offset

**Returns:** `float` — Bar offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetBarOffset)

#### `GetBendingShapeViewVector()`

Get the view vector of the bending shape

**Remarks:** Get the view vector of the bending shape

**Returns:** `Vector3D` — View vector of the bending shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetBendingShapeViewVector)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetCommonProperties)

#### `GetConcreteCoverEnd()`

Get the concrete cover at the end of the path

**Remarks:** Get the concrete cover at the end of the path

**Returns:** `float` — Concrete cover at the end of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetConcreteCoverEnd)

#### `GetConcreteCoverStart()`

Get the concrete cover at the start of the path

**Remarks:** Get the concrete cover at the start of the path

**Returns:** `float` — Concrete cover at the start of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetConcreteCoverStart)

#### `GetCrossBarDistance()`

Get the cross bar distance

**Remarks:** Get the cross bar distance

**Returns:** `float` — Cross bar distance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetCrossBarDistance)

#### `GetCrossBendingShapes()`

Get the cross bending shapes

**Remarks:** Get the cross bending shapes

**Returns:** `BendingShapeList` — Cross bending shapes

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetCrossBendingShapes)

#### `GetEdgeOffsetEnd()`

Get the edge offset at the end of the path

**Remarks:** Get the edge offset at the end of the path

**Returns:** `float` — Edge offset at the end of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsetEnd)

#### `GetEdgeOffsetStart()`

Get the edge offset at the start of the path

**Remarks:** Get the edge offset at the start of the path

**Returns:** `float` — Edge offset at the start of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsetStart)

#### `GetEdgeOffsetType()`

Get the edge offset type

**Remarks:** Get the edge offset type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsetType)

#### `GetEdgeOffsets()`

Get the edge offsets

**Remarks:** Get the edge offsets

**Returns:** `tuple` — Edge offsets

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetEdgeOffsets)

#### `GetMaxBreakAngle()`

Get the maximal break angle

**Remarks:** Get the maximal break angle

**Returns:** `float` — Maximal break angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetMaxBreakAngle)

#### `GetPlacementPath()`

Get the placement path

**Remarks:** Get the placement path

**Returns:** `Path3D` — Placement path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetPlacementPath)

#### `GetPlacementSections()`

Get the placement sections

**Remarks:** Get the placement sections

**Returns:** `object` — Placement sections

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetPlacementSections)

#### `GetPositionNumber()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetPositionNumber)

#### `GetProfileRoation()`

Get the profile rotation

**Remarks:** Get the profile rotation

**Returns:** `eProfileRotation` — Profile rotation

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.GetProfileRoation)

#### `IsBreakElimination()`

Get the break eliminination state

**Remarks:** Get the break eliminination state

**Returns:** `bool` — Break elimination state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.IsBreakElimination)

#### `Move(transVec)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.Move)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.SetCommonProperties)

#### `Transform(transMat)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.Transform)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ExtrudeBarPlacement/#NemAll_Python_Reinforcement.ExtrudeBarPlacement.__repr__)

### Properties
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `PositionNumber` (int, get/set) — Get the position number

## Functions (static-class)

Module-level functions of NemAll_Python_Reinforcement

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/_functions/)

### Methods
#### `CreateReinforcementLabeling(doc, insertionMat, labelList, viewProj, undoRedoService=None) | CreateReinforcementLabeling(doc, insertionMat, labelList, viewProj)`

Create the reinforcement labels

**Remarks:** Create the reinforcement labels

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Insertion matrix
- `modelEleList` (object) — List with the model elements
- `viewProj` (ViewWorldProjection) — View projection
- `undoRedoService` (Optional[object]) — Undo redo service

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/_functions/#NemAll_Python_Reinforcement.CreateReinforcementLabeling)

#### `InitApplicationtest()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/_functions/#NemAll_Python_Reinforcement.InitApplicationtest)

#### `InitUnitTest()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/_functions/#NemAll_Python_Reinforcement.InitUnitTest)

## GeometryExpansionUtil (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.GeometryExpansionUtil.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/)

### Constructors
- `GeometryExpansionUtil() | GeometryExpansionUtil(pMsgInfo, use3DGeometry)` — Initialize

### Methods
#### `GetLineAbove(arg2, arg3, arg4, arg5)`

Get the line above the base line and the placement point

**Remarks:** Get the line above the base line and the placement point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineAbove)

#### `GetLineAtPoint(arg2, arg3, arg4, arg5)`

Get the line at the defined point of the reference line

**Remarks:** Get the line at the defined point of the reference line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineAtPoint)

#### `GetLineFromPoint(arg2, arg3, arg4, arg5)`

Get the line near to the input point

**Remarks:** Get the line near to the input point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineFromPoint)

#### `GetLineLeft(arg2, arg3, arg4, arg5)`

Get the line left from the base line and the placement point

**Remarks:** Get the line left from the base line and the placement point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/GeometryExpansionUtil/#NemAll_Python_Reinforcement.GeometryExpansionUtil.GetLineLeft)

## HookLengthService (class)

Service class for the hook length calculation

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/)

### Constructors
- `HookLengthService(norm, concreteGrade, steelGrade, bExactLength)` — Constructor

### Methods
#### `GetHookLength(hookAngle, hookType, diameter)`

Calculate the hook length

**Remarks:** Calculate the hook length

**Parameters:**
- `hookAngle` (float) — Hook angle
- `hookType` (HookType) — Hook type
- `diameter` (float) — Diameter

**Returns:** `float` — Hook length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetHookLength)

#### `GetHookLengthPartFromBendingRoller(hookAngle, hookType, diameter)`

Calculate the hook length part from the beginning of the bending roller

**Remarks:** Calculate the hook length part from the beginning of the bending roller

**Parameters:**
- `hookAngle` (float) — Hook angle
- `hookType` (HookType) — Hook type
- `diameter` (float) — Diameter

**Returns:** `float` — Hook length part from the beginning of the bending roller

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetHookLengthPartFromBendingRoller)

#### `GetHookLengthPartOfBendingRoller(hookAngle, hookType, diameter)`

Calculate the hook length part of the bending roller

**Remarks:** Calculate the hook length part of the bending roller

**Parameters:**
- `hookAngle` (float) — Hook angle
- `hookType` (HookType) — Hook type
- `diameter` (float) — Diameter

**Returns:** `float` — Hook length part of the bending roller

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetHookLengthPartOfBendingRoller)

#### `GetStandardAnchorageHookLength(diameter)`

Calculate the standard anchorage hook length

**Remarks:** Calculate the standard anchorage hook length

**Parameters:**
- `diameter` (float) — Diameter

**Returns:** `float` — Standard anchorage hook length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookLengthService/#NemAll_Python_Reinforcement.HookLengthService.GetStandardAnchorageHookLength)

## HookType (enum)

Types of the hooks

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `HookType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/HookType/#NemAll_Python_Reinforcement.HookType.__getitem__)

### Values
- `eAnchorage` = `3`
- `eAngle` = `2`
- `eStirrup` = `1`

## LabelType (enum)

Types of the label

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LabelType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `LabelType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LabelType/#NemAll_Python_Reinforcement.LabelType.__getitem__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/)

### Constructors
- `LongitudinalBarProperties() | LongitudinalBarProperties(shape, overlappingAtStartTurnedOn, overlappingAtStart, overlappingAtEndTurnedOn, overlappingAtEnd, overlappingLength, minBarDistance, deliveryShapeType, insideBarsState, startLength)` — Initialize

### Methods
#### `GetBendingShape()`

Get the bending shape

**Remarks:** Get the bending shape

**Returns:** `BendingShape` — Bending shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetBendingShape)

#### `GetDeliveryShapeType()`

Get the delivery shape type

**Remarks:** Get the delivery shape type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetDeliveryShapeType)

#### `GetInsideBarsState()`

Get the insid bars state

**Remarks:** Get the insid bars state

**Returns:** `eInsideBarsState` — Inside bars state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetInsideBarsState)

#### `GetMinBarDistance()`

Get the minimal bar distance

**Remarks:** Get the minimal bar distance

**Returns:** `float` — Minimal bar distance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetMinBarDistance)

#### `GetOverlappingAtEnd()`

Get the overlapping at end

**Remarks:** Get the overlapping at end

**Returns:** `float` — Overlapping at end

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetOverlappingAtEnd)

#### `GetOverlappingAtStart()`

Get the overlapping at start

**Remarks:** Get the overlapping at start

**Returns:** `float` — Overlapping at start

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetOverlappingAtStart)

#### `GetOverlappingLength()`

Get the overlapping length

**Remarks:** Get the overlapping length

**Returns:** `float` — Overlapping length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetOverlappingLength)

#### `GetStartLength()`

Get the start length

**Remarks:** Get the start length

**Returns:** `float` — Start length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.GetStartLength)

#### `IsOverlappingAtEndTurnedOn()`

Get the overlapping at end state

**Remarks:** Get the overlapping at end state

**Returns:** `bool` — Overlapping at end state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.IsOverlappingAtEndTurnedOn)

#### `IsOverlappingAtStartTurnedOn()`

Get the overlapping at start state

**Remarks:** Get the overlapping at start state

**Returns:** `bool` — Overlapping at start state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.IsOverlappingAtStartTurnedOn)

#### `SetBendingShape(shape)`

Set the bending shape

**Remarks:** Set the bending shape

**Parameters:**
- `shape` (BendingShape) — Shape

**Returns:** `object` — Bending shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.SetBendingShape)

#### `__eq__(LongitudinalBarProperties)`

Compare operator Args:

**Remarks:** Compare operator Args:

**Returns:** `bool` — Bars are equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarProperties/#NemAll_Python_Reinforcement.LongitudinalBarProperties.__eq__)

### Properties
- `BendingShape` (BendingShape, get/set) — Get the bending shape

## LongitudinalBarPropertiesList (class)

List for LongitudinalBarProperties objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/)

### Constructors
- `LongitudinalBarPropertiesList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (LongitudinalBarProperties) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (LongitudinalBarProperties) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `LongitudinalBarProperties` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__len__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (LongitudinalBarProperties) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (LongitudinalBarProperties) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (LongitudinalBarPropertiesList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/LongitudinalBarPropertiesList/#NemAll_Python_Reinforcement.LongitudinalBarPropertiesList.extend)

## MeshAreaPlacementProperties (enum)

(No description provided in vendor docs for NemAll_Python_Reinforcement.MeshAreaPlacementProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementProperties/)

### Constructors
- `MeshAreaPlacementProperties()` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementProperties/#NemAll_Python_Reinforcement.MeshAreaPlacementProperties.__repr__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/)

### Constructors
- `MeshAreaPlacementService()` — Initialize

### Methods
#### `AddOpeningPolygon(arg2, openingPol)`

Add an opening polygon

**Remarks:** Add an opening polygon

**Parameters:**
- `openingPol` (float) — Opening polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/#NemAll_Python_Reinforcement.MeshAreaPlacementService.AddOpeningPolygon)

#### `Calculate(arg2, doc, mesh, placementMatrix, startPositionNumber, concreteCoverZDir)`

Calculate the meshes

**Remarks:** Calculate the meshes

**Parameters:**
- `doc` (MeshData) — Document
- `mesh` (MeshAreaPlacementProperties) — Mesh data
- `placementMatrix` (Matrix3D) — Placement matrix
- `startPositionNumber` (int) — Start position number
- `concreteCoverZDir` (float) — Concrete cover in the local z direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/#NemAll_Python_Reinforcement.MeshAreaPlacementService.Calculate)

#### `SetOuterPolygon(arg2, placementPol)`

Constructor

**Remarks:** Constructor

**Parameters:**
- `placementPol` (float) — Placement polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshAreaPlacementService/#NemAll_Python_Reinforcement.MeshAreaPlacementService.SetOuterPolygon)

## MeshBendingDirection (enum)

Types of the mesh bending direction

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshBendingDirection/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `MeshBendingDirection` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshBendingDirection/#NemAll_Python_Reinforcement.MeshBendingDirection.__getitem__)

### Values
- `CrossBars` = `0`
- `LongitudinalBars` = `1`

## MeshData (class)

Implementation of the mesh data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/)

### Constructors
- `MeshData() | MeshData(type, length, width, diameterLongitudinal, diameterCross, asLongitudinal, asCross, distanceLongitudinal, distanceCross, bDoubleBarLongitudinal, bDoubleBarCross, overlapLongitudinal, overlapCross, weight) | MeshData(element)` — Initialize

### Methods
#### `CreateLabel()`

Create the label

**Remarks:** Create the label

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.CreateLabel)

#### `Format(type, length, width)`

Get the mesh text

**Remarks:** Get the mesh text

**Parameters:**
- `type` (str) — Mesh type
- `length` (float) — Mesh length
- `width` (float) — Mesh width

**Returns:** `str` — Mesh text

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.Format)

#### `GetAsBendingDirection(bendingDirection)`

Get the as in bending direction

**Remarks:** Get the as in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — As in bending direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetAsBendingDirection)

#### `GetDiameterBendingDirection(bendingDirection)`

Get the diameter in bending direction

**Remarks:** Get the diameter in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — tuple(Diameter in bending direction,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetDiameterBendingDirection)

#### `GetDimensions()`

Get the mesh dimensions

**Remarks:** Get the mesh dimensions

**Returns:** `float` — tuple(Mesh length,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetDimensions)

#### `GetDistanceBendingDirection(bendingDirection)`

Get the distance in bending direction

**Remarks:** Get the distance in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — Distance in bending direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetDistanceBendingDirection)

#### `GetOverlapBendingDirection(bendingDirection)`

Get the overlap in bending direction

**Remarks:** Get the overlap in bending direction

**Parameters:**
- `bendingDirection` (MeshBendingDirection) — Bending direction

**Returns:** `float` — Overlap in bending direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.GetOverlapBendingDirection)

#### `SetType(type)`

Set the mesh type

**Remarks:** Set the mesh type

**Parameters:**
- `type` (str) — Mesh type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshData/#NemAll_Python_Reinforcement.MeshData.SetType)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshOperations/)

### Constructors
- `MeshOperations()` — Initialize

### Methods
#### `CutMesh(placements, divisionLine)`

Divide the bars placement

**Remarks:** Divide the bars placement

**Parameters:**
- `placement` (object) — BaseElementAdapter with the placement
- `cutPolygon` (object) — Cut polygon

**Returns:** `str` — Result message

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshOperations/#NemAll_Python_Reinforcement.MeshOperations.CutMesh)

## MeshPlacement (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.MeshPlacement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/)

### Constructors
- `MeshPlacement() | MeshPlacement(positionNumber, widthVec, bendingShape)` — Initialize

### Methods
#### `GetBendingShape()`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `BendingShape` — Shape polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.GetBendingShape)

#### `GetPositionNumber()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.GetPositionNumber)

#### `GetWidthVector()`

Get the width vector

**Remarks:** Get the width vector

**Returns:** `Vector3D` — Width vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.GetWidthVector)

#### `Move(transVec)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.Move)

#### `SetBendingShape(shape)`

Set the reinforcement shape

**Remarks:** Set the reinforcement shape

**Parameters:**
- `shape` (BendingShape) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.SetBendingShape)

#### `SetPositionNumber(positionNumber)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.SetPositionNumber)

#### `SetWidthVector(widthVec)`

Set the width vector

**Remarks:** Set the width vector

**Parameters:**
- `widthVec` (Vector3D) — Width vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.SetWidthVector)

#### `Transform(transMat)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.Transform)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/MeshPlacement/#NemAll_Python_Reinforcement.MeshPlacement.__repr__)

## NormType (enum)

Types of the norms

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/NormType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `NormType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/NormType/#NemAll_Python_Reinforcement.NormType.__getitem__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/)

### Constructors
- `PlaneMeshPlacement() | PlaneMeshPlacement(placement) | PlaneMeshPlacement(positionNumber, meshData, meshLength, meshWidth, meshPolygon)` — Initialize

### Methods
#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetCommonProperties)

#### `GetMeshData()`

Get the mesh data

**Remarks:** Get the mesh data

**Returns:** `MeshData` — Mesh data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshData)

#### `GetMeshLength()`

Get the mesh length

**Remarks:** Get the mesh length

**Returns:** `float` — Mesh length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshLength)

#### `GetMeshPolygon()`

Get the shape polyline

**Remarks:** Get the shape polyline

**Returns:** `Polygon3D` — Shape polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshPolygon)

#### `GetMeshWidth()`

Get the mesh width

**Remarks:** Get the mesh width

**Returns:** `float` — Mesh width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetMeshWidth)

#### `GetPositionNumber()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.GetPositionNumber)

#### `IsValid()`

Get the state of the shape

**Remarks:** Get the state of the shape

**Returns:** `bool` — Shape is valid: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.IsValid)

#### `Move(transVec)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.Move)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.SetCommonProperties)

#### `SetMeshPolygon(shape)`

Set the reinforcement shape

**Remarks:** Set the reinforcement shape

**Parameters:**
- `shape` (Polygon3D) — Reinforcement shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.SetMeshPolygon)

#### `SetPositionNumber(positionNumber)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.SetPositionNumber)

#### `Transform(transMat)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.Transform)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/PlaneMeshPlacement/#NemAll_Python_Reinforcement.PlaneMeshPlacement.__repr__)

### Properties
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `MeshPolygon` (NemAll_Python_Geometry.Polygon3D, get/set) — Get the shape polyline
- `PositionNumber` (int, get/set) — Get the position number

## ReinfElement (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinfElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinfElement/)

## ReinforcementLabel (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabel.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/)

### Constructors
- `ReinforcementLabel() | ReinforcementLabel(reinforcementType, type, positionNumber, labelProp, labelPoint, angle) | ReinforcementLabel(reinforcementType, type, positionNumber, labelProp, shapeSide, shapeSideFactor, labelOffset, angle) | ReinforcementLabel(reinforcementType, type, positionNumber, labelProp, bDimLineAtShapeStart, dimLineOffset) | ReinforcementLabel(reinforcementType, type, positionNumber, labelProp, pointerProp, bDimLineAtShapeStart, dimLineOffset)` — Initialize

### Methods
#### `SetAdditionalText(additionalText)`

Set the additional text

**Remarks:** Set the additional text

**Parameters:**
- `additionalText` (str) — Additional text

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetAdditionalText)

#### `SetLabelOffset(labelOffset)`

Set the label offset

**Remarks:** Set the label offset

**Parameters:**
- `labelOffset` (Vector2D) — Label offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetLabelOffset)

#### `SetPointerStartPoint(pointerStartPoint)`

Set the start pointer of the text pointer

**Remarks:** Set the start pointer of the text pointer

**Parameters:**
- `pointerStartPoint` (Point2D) — Start point of the text pointer

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetPointerStartPoint)

#### `SetShowTextPointer(showTextPointer)`

Set the state for showing the text pointer

**Remarks:** Set the state for showing the text pointer

**Parameters:**
- `showTextPointer` (bool) — Show the text pointer: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetShowTextPointer)

#### `SetShowTextPointerEndSymbol(showTextPointerEndSymbol)`

Set the state for showing the text pointer end symbol

**Remarks:** Set the state for showing the text pointer end symbol

**Parameters:**
- `showTextPointerEndSymbol` (bool) — Show the text pointer end symbol: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetShowTextPointerEndSymbol)

#### `SetTextProperties(textProperties)`

Set the text properties

**Remarks:** Set the text properties

**Parameters:**
- `textProperties` (TextProperties) — Text properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetTextProperties)

#### `SetVisibleBars(visibleBars)`

Set the vector with the visible bars

**Remarks:** Set the vector with the visible bars

**Parameters:**
- `visibleBars` ([list[int] | VecIntList]) — Vector with the visible bars: 1, 2, 3, .. index from left; -1, -2, -3, ... index from right, 0 = center

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.SetVisibleBars)

#### `ShowAllBars(bShowAllBars)`

Set the all bars inside the dimension line, ... state

**Remarks:** Set the all bars inside the dimension line, ... state

**Parameters:**
- `bShowAllBars` (bool) — Show all bars in the dimension lines, ...: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabel/#NemAll_Python_Reinforcement.ReinforcementLabel.ShowAllBars)

## ReinforcementLabelList (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabelList.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/)

### Constructors
- `ReinforcementLabelList()` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (ReinforcementLabel) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (ReinforcementLabel) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__delitem__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `ReinforcementLabel` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__getitem__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__len__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (ReinforcementLabel) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (ReinforcementLabel) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.append)

#### `extend(iterable)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (ReinforcementLabelList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelList/#NemAll_Python_Reinforcement.ReinforcementLabelList.extend)

## ReinforcementLabelPointerProperties (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabelPointerProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelPointerProperties/)

### Constructors
- `ReinforcementLabelPointerProperties() | ReinforcementLabelPointerProperties(combLineAngle, bCombLineByLength, combLineValue)` — Initialize

## ReinforcementLabelProperties (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementLabelProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelProperties/)

### Constructors
- `ReinforcementLabelProperties() | ReinforcementLabelProperties(prop)` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementLabelProperties/#NemAll_Python_Reinforcement.ReinforcementLabelProperties.__repr__)

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

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/)

### Methods
#### `GetACIBarMark(barsDefinitionElement, showIndex)`

Get the bar mark

**Remarks:** Get the bar mark

**Parameters:**
- `barsDefinitionElement` (BaseElementAdapter) — Bars definition element
- `showIndex` (bool) — Show the index

**Returns:** `list` — bar mark for ACI

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetACIBarMark)

#### `GetACIPlacementBarMark(barsPlacementElement, showIndex)`

Get the bar mark for a placement

**Remarks:** Get the bar mark for a placement

**Parameters:**
- `barsPlacementElement` (BaseElementAdapter) — Bars placement element
- `showIndex` (bool) — Show the index

**Returns:** `tuple` — tuple(bar mark for ACI, bar count)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetACIPlacementBarMark)

#### `GetBarPositionData(elementVec)`

Get the bar position data

**Remarks:** Get the bar position data

**Parameters:**
- `elementVec` (BaseElementAdapterList) — GUID vector with the elements

**Returns:** `list` — List with the bar position data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetBarPositionData)

#### `GetBarShapeCode(barsDefinitionElement)`

Get the bar shape code

**Remarks:** Get the bar shape code

**Parameters:**
- `barsDefinitionElement` (BarShapeCodeStandard) — Bars definition element
- `barShapeCopdeStandard` (object) — Standard for the bar shape code

**Returns:** `tuple` — shape code count, list of (shape codes, bar length), list of (segment name, segment lengths)) for ACI

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementService/#NemAll_Python_Reinforcement.ReinforcementService.GetBarShapeCode)

## ReinforcementSettings (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementSettings.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/)

### Constructors
- `ReinforcementSettings()` — Initialize

### Methods
#### `CheckBarDiameter()`

Check, whether the diameter is included in the diameter list of the current steel grade

**Remarks:** Check, whether the diameter is included in the diameter list of the current steel grade

**Returns:** `float` — Bar diameter (original or nearest value)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.CheckBarDiameter)

#### `CheckMeshGroup()`

Check, whether the mesh group is included in the group list

**Remarks:** Check, whether the mesh group is included in the group list

**Returns:** `int` — Mesh group (original or first)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.CheckMeshGroup)

#### `GetBarDiameter()`

Get the current bar diameter

**Remarks:** Get the current bar diameter

**Returns:** `float` — Bar diameter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetBarDiameter)

#### `GetBarWeight(arg2)`

Get the weight for a bar diameter

**Remarks:** Get the weight for a bar diameter

**Returns:** `float` — Weight for a bar diameter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetBarWeight)

#### `GetBendingRoller()`

Get the current bending roller

**Remarks:** Get the current bending roller

**Returns:** `float` — Bending roller

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetBendingRoller)

#### `GetConcreteGrade()`

Get the current concrete grade

**Remarks:** Get the current concrete grade

**Returns:** `int` — Concrete grade

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetConcreteGrade)

#### `GetMaxBarLength()`

Get the maximal bar length

**Remarks:** Get the maximal bar length

**Returns:** `float` — Maximal bar length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetMaxBarLength)

#### `GetMeshGroup()`

Get the current mesh group

**Remarks:** Get the current mesh group

**Returns:** `int` — Mesh group

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetMeshGroup)

#### `GetMeshType()`

Get the current mesh type

**Remarks:** Get the current mesh type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetMeshType)

#### `GetNorm()`

Get the current norm

**Remarks:** Get the current norm

**Returns:** `int` — Norm

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetNorm)

#### `GetSteelGrade()`

Get the current steel grade

**Remarks:** Get the current steel grade

**Returns:** `int` — Steel grade

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.GetSteelGrade)

#### `Is3DReinforcement()`

Get the 3D-Reinforcement state

**Remarks:** Get the 3D-Reinforcement state

**Returns:** `bool` — Create 3D-Reinforcement: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementSettings/#NemAll_Python_Reinforcement.ReinforcementSettings.Is3DReinforcement)

## ReinforcementShapeBuilder (class)

Implementation of the reinforcement shape builder

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/)

### Constructors
- `ReinforcementShapeBuilder() | ReinforcementShapeBuilder(shapePlaneMatrix) | ReinforcementShapeBuilder(shapePlaneMatrix, create3DShape, localZCoverFront, localZCoverBack) | ReinforcementShapeBuilder(element)` — Initialize

### Methods
#### `AddPoint(pnt, concreteCover, bendingRoller, zCoordBar=0) | AddPoint(pnt, concreteCover, bendingRoller)`

Add an end point of a geometry side

**Remarks:** Add an end point of a geometry side

**Parameters:**
- `pnt` (Point2D) — End point of the side
- `concreteCover` (float) — Concrete cover
- `bendingRoller` (float) — Bending roller
- `zCoordBar` (float) — Bar coordinate in z direction of the local shape coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddPoint)

#### `AddPoints(pointList)`

Add the shape geometry points

**Remarks:** Add the shape geometry points

**Parameters:**
- `pointList` (object) — Point list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddPoints)

#### `AddSide(startPnt, endPnt, concreteCover, bendingRoller, zCoordBar=0) | AddSide(startPnt, endPnt, concreteCover, bendingRoller)`

Add a geometry side of the shape

**Remarks:** Add a geometry side of the shape

**Parameters:**
- `startPnt` (Point2D) — Start point of the geometry side
- `endPnt` (Point2D) — End point of the geometry side
- `concreteCover` (float) — Concrete cover
- `bendingRoller` (float) — Bending roller between the last and current side
- `zCoordBar` (float) — Bar coordinate in z direction of the local shape coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddSide)

#### `AddSides(sideList)`

Add the geometry sides of a shape

**Remarks:** Add the geometry sides of a shape

**Parameters:**
- `sideList` (object) — Side list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.AddSides)

#### `CreateShape(diameter, bendingRoller, steelGrade, concreteGrade, bendingShapeType) | CreateShape(meshType, meshBendingDirection, bendingRoller, steelGrade, concreteGrade, bendingShapeType) | CreateShape(shapeProps)`

Create the shape

**Remarks:** Create the shape

**Parameters:**
- `diameter` (float) — Diameter
- `bendingRoller` (float) — Default bending roller
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)
- `bendingShapeType` (BendingShapeType) — Bending shape type

**Returns:** `BendingShape` — Creation successful: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.CreateShape)

#### `CreateStirrup(diameter, bendingRoller, steelGrade, concreteGrade, stirrupType) | CreateStirrup(meshType, meshBendingDirection, bendingRoller, steelGrade, concreteGrade, stirrupType) | CreateStirrup(shapeProps, stirrupType)`

Create the stirrup shape

**Remarks:** Create the stirrup shape

**Parameters:**
- `diameter` (float) — Diameter
- `bendingRoller` (float) — Default bending roller
- `steelGrade` (int) — Steel grade
- `concreteGrade` (int) — Concrete grade (index of the global list starting from 0, -1 = use global value from the Allplan settings)
- `stirrupType` (StirrupType) — Type of the stirrup

**Returns:** `BendingShape` — Creation successful: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.CreateStirrup)

#### `GetMeshData(meshType)`

Get the mesh data

**Remarks:** Get the mesh data

**Parameters:**
- `meshType` (str) — Mesh type

**Returns:** `MeshData` — Mesh data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.GetMeshData)

#### `SetAnchorageHookEnd(angle)`

Set an anchorage hook at the end of the shape

**Remarks:** Set an anchorage hook at the end of the shape

**Parameters:**
- `angle` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookEnd)

#### `SetAnchorageHookEndFromSide()`

Set an anchorage hook at the end of the shape, get the angle from the side

**Remarks:** Set an anchorage hook at the end of the shape, get the angle from the side

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookEndFromSide)

#### `SetAnchorageHookStart(angle)`

Set an anchorage hook at the start of the shape

**Remarks:** Set an anchorage hook at the start of the shape

**Parameters:**
- `angle` (float) — Hook angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookStart)

#### `SetAnchorageHookStartFromSide()`

Set an anchorage hook at the start of the shape, get the angle from the side

**Remarks:** Set an anchorage hook at the start of the shape, get the angle from the side

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageHookStartFromSide)

#### `SetAnchorageLengthEnd(anchorageLength)`

Set the anchorage length at the end of the shape

**Remarks:** Set the anchorage length at the end of the shape

**Parameters:**
- `anchorageLength` (float) — Anchorage length at the end of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageLengthEnd)

#### `SetAnchorageLengthStart(anchorageLength)`

Set the anchorage length at the start of the shape

**Remarks:** Set the anchorage length at the start of the shape

**Parameters:**
- `anchorageLength` (float) — Anchorage length at the start of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetAnchorageLengthStart)

#### `SetConcreteCoverEnd(concreteCover)`

Set the concrete cover at the end of the shape

**Remarks:** Set the concrete cover at the end of the shape

**Parameters:**
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverEnd)

#### `SetConcreteCoverLineEnd(startPnt, endPnt, concreteCover) | SetConcreteCoverLineEnd(startPnt, endPnt, concreteCover)`

Set the concrete cover line at the end of the shape

**Remarks:** Set the concrete cover line at the end of the shape

**Parameters:**
- `startPnt` (Point2D) — Start point of the concrete cover line at the end of the shape
- `endPnt` (Point2D) — Endpoint of the concrete cover line at the end of the shape
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverLineEnd)

#### `SetConcreteCoverLineStart(startPnt, endPnt, concreteCover) | SetConcreteCoverLineStart(startPnt, endPnt, concreteCover)`

Set the concrete cover line at the start of the shape

**Remarks:** Set the concrete cover line at the start of the shape

**Parameters:**
- `startPnt` (Point2D) — Start point of the concrete cover line at the start of the shape
- `endPnt` (Point2D) — Endpoint of the concrete cover line at the start of the shape
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverLineStart)

#### `SetConcreteCoverStart(concreteCover)`

Set the concrete cover at the start of the shape

**Remarks:** Set the concrete cover at the start of the shape

**Parameters:**
- `concreteCover` (float) — Concrete cover

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetConcreteCoverStart)

#### `SetFullCircleOverlap(fullCircleOverlap)`

Set the overlap length for the full circle stirrup

**Remarks:** Set the overlap length for the full circle stirrup

**Parameters:**
- `fullCircleOverlap` (float) — Overlap length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetFullCircleOverlap)

#### `SetHookEnd(length, angle, type)`

Set the hook at the end of the shape

**Remarks:** Set the hook at the end of the shape

**Parameters:**
- `length` (float) — Hook length (0 = calculate)
- `angle` (float) — Hook angle
- `type` (HookType) — Hook type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetHookEnd)

#### `SetHookStart(length, angle, type)`

Set the hook at the start of the shape

**Remarks:** Set the hook at the start of the shape

**Parameters:**
- `length` (float) — Hook length (0 = calculate)
- `angle` (float) — Hook angle
- `type` (HookType) — Hook type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetHookStart)

#### `SetOverlapLengthEnd()`

Set an overlap length a the end of the shape

**Remarks:** Set an overlap length a the end of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetOverlapLengthEnd)

#### `SetOverlapLengthStart()`

Set an overlap length a the start of the shape

**Remarks:** Set an overlap length a the start of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetOverlapLengthStart)

#### `SetSideLengthEnd(sideLength)`

Set the side length at the end of the shape

**Remarks:** Set the side length at the end of the shape

**Parameters:**
- `sideLength` (float) — Side length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetSideLengthEnd)

#### `SetSideLengthStart(sideLength)`

Set the side length at the start of the shape

**Remarks:** Set the side length at the start of the shape

**Parameters:**
- `sideLength` (float) — Side length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetSideLengthStart)

#### `SetStartPoint(startPnt) | SetStartPoint(startPnt)`

Set a start point of a geometry side

**Remarks:** Set a start point of a geometry side

**Parameters:**
- `startPnt` (Point2D) — Start point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementShapeBuilder/#NemAll_Python_Reinforcement.ReinforcementShapeBuilder.SetStartPoint)

## ReinforcementType (enum)

Types of the reinforcement

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ReinforcementType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementType/#NemAll_Python_Reinforcement.ReinforcementType.__getitem__)

### Values
- `Bar` = `0`
- `Mesh` = `1`

## ReinforcementUtil (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.ReinforcementUtil.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/)

### Constructors
- `ReinforcementUtil()` — Initialize

### Methods
#### `GetNextBarPositionNumber(doc)`

Get the the next bar position number

**Remarks:** Get the the next bar position number

**Returns:** `int` — Next bar position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/#NemAll_Python_Reinforcement.ReinforcementUtil.GetNextBarPositionNumber)

#### `GetNextMeshPositionNumber(doc)`

Get the the next mesh position number

**Remarks:** Get the the next mesh position number

**Returns:** `int` — Next mesh position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/#NemAll_Python_Reinforcement.ReinforcementUtil.GetNextMeshPositionNumber)

#### `Rearrange(doc, fromBarPosition=1, fromMeshPosition=1, toBarPosition=99999, toMeshPosition=99999, afterBarPosition=1, aftgerMeshPosition=1, tolerance=1.0, rearrangedLock=False, identicalShapes=False, identicalPrefix=False, createUndoStep=True)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/ReinforcementUtil/#NemAll_Python_Reinforcement.ReinforcementUtil.Rearrange)

## SpiralElement (class)

(No description provided in vendor docs for NemAll_Python_Reinforcement.SpiralElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/)

### Constructors
- `SpiralElement() | SpiralElement(positionNumber, diameter, steelGrade, concreteGrade, rotationAxis, contourPoints, pitch, hookLengthStart, hookAngleStart, hookLengthEnd, hookAngleEnd, concreteCoverStart, concreteCoverEnd, concreteCoverContour)` — Initialize

### Methods
#### `SetLengthFactor(arg2)`

Get the length factor

**Remarks:** Get the length factor

**Returns:** `object` — Length factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetLengthFactor)

#### `SetNumberLoopsEnd(arg2)`

Set the loops at the end

**Remarks:** Set the loops at the end

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetNumberLoopsEnd)

#### `SetNumberLoopsStart(arg2)`

Set the loops at the start

**Remarks:** Set the loops at the start

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetNumberLoopsStart)

#### `SetPitchSections(pitch1, length1, pitch2, length2, pitch3, length3, pitch4, length4)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetPitchSections)

#### `SetPlacePerLinearMeter(bPlacePerMeter)`

Set the place per linear meter state

**Remarks:** Set the place per linear meter state

**Parameters:**
- `bPlacePerMeter` (bool) — Place per linear meter: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SpiralElement/#NemAll_Python_Reinforcement.SpiralElement.SetPlacePerLinearMeter)

## StirrupType (enum)

Types of the stirrups

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/StirrupType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `StirrupType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/StirrupType/#NemAll_Python_Reinforcement.StirrupType.__getitem__)

### Values
- `Column` = `3`
- `Diamond` = `4`
- `FullCircle` = `5`
- `Normal` = `1`
- `Torsion` = `2`

## SweepBarPlacement (class)

Implementation of the sweep bar placement element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/)

### Constructors
- `SweepBarPlacement() | SweepBarPlacement(placement) | SweepBarPlacement(positionNumber, sweepPaths, rotation, firstPathIsSweepPath, interpolation, interpolationOfAllPoints, crossBarDistance, concreteCoverStart, concreteCoverEnd, edgeOffsetType, edgeOffsetStart, edgeOffsetEnd, barOffset, benchingLength, benchingAngle)` — Initialize

### Methods
#### `AddPlacementSection(placementSection)`

Add a placement section

**Remarks:** Add a placement section

**Parameters:**
- `placementSection` (BarPlacementSection) — Section

**Returns:** `bool` — Section is added: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.AddPlacementSection)

#### `AddSectionBars(bendingShapes, sectionsLongitudinalBarsProp, sectionPlane)`

Add the bars and the section plane for a section bendingShapeViewVector View vector of the bending shape

**Remarks:** Add the bars and the section plane for a section bendingShapeViewVector View vector of the bending shape

**Parameters:**
- `bendingShapes` (BendingShapeList) — Bending shapes of the section
- `sectionsLongitudinalBarsProp` (LongitudinalBarPropertiesList) — Longitudinal bars properties
- `sectionPlane` (Plane3D) — Section plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.AddSectionBars)

#### `GetBarOffset()`

Get the bar offset

**Remarks:** Get the bar offset

**Returns:** `float` — Bar offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetBarOffset)

#### `GetBenchingAngle()`

Get the benching angle

**Remarks:** Get the benching angle

**Returns:** `float` — Benching angle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetBenchingAngle)

#### `GetBenchingLength()`

Get the benching length

**Remarks:** Get the benching length

**Returns:** `float` — Benching length

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetBenchingLength)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetCommonProperties)

#### `GetConcreteCoverEnd()`

Get the concrete cover at the end of the path

**Remarks:** Get the concrete cover at the end of the path

**Returns:** `float` — Concrete cover at the end of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetConcreteCoverEnd)

#### `GetConcreteCoverStart()`

Get the concrete cover at the start of the path

**Remarks:** Get the concrete cover at the start of the path

**Returns:** `float` — Concrete cover at the start of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetConcreteCoverStart)

#### `GetCrossBarDistance()`

Get the cross bar distance

**Remarks:** Get the cross bar distance

**Returns:** `float` — Cross bar distance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetCrossBarDistance)

#### `GetEdgeOffsetEnd()`

Get the edge offset at the end of the path

**Remarks:** Get the edge offset at the end of the path

**Returns:** `float` — Edge offset at the end of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsetEnd)

#### `GetEdgeOffsetStart()`

Get the edge offset at the start of the path

**Remarks:** Get the edge offset at the start of the path

**Returns:** `float` — Edge offset at the start of the path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsetStart)

#### `GetEdgeOffsetType()`

Get the edge offset type

**Remarks:** Get the edge offset type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsetType)

#### `GetEdgeOffsets()`

Get the edge offsets

**Remarks:** Get the edge offsets

**Returns:** `tuple` — Edge offsets

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetEdgeOffsets)

#### `GetPlacementSections()`

Get the placement sections

**Remarks:** Get the placement sections

**Returns:** `object` — Placement sections

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetPlacementSections)

#### `GetPositionNumber()`

Get the position number

**Remarks:** Get the position number

**Returns:** `int` — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.GetPositionNumber)

#### `IsFirstPathIsSweepPath()`

Get the first path is sweep path state

**Remarks:** Get the first path is sweep path state

**Returns:** `bool` — First path is sweep path state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsFirstPathIsSweepPath)

#### `IsInterpolation()`

Get the interpolation state

**Remarks:** Get the interpolation state

**Returns:** `bool` — Interpolation state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsInterpolation)

#### `IsInterpolationOfAllPoints()`

Get the interpolation of all points state

**Remarks:** Get the interpolation of all points state

**Returns:** `bool` — Interpolation of all points state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsInterpolationOfAllPoints)

#### `IsRoation()`

Get the rotation state

**Remarks:** Get the rotation state

**Returns:** `bool` — Rotation state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.IsRoation)

#### `Move(transVec)`

Move the placement

**Remarks:** Move the placement

**Parameters:**
- `transVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.Move)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.SetCommonProperties)

#### `SetPositionNumber(positionNumber)`

Set the position number

**Remarks:** Set the position number

**Parameters:**
- `positionNumber` (int) — Position number

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.SetPositionNumber)

#### `Sweep()`

Sweep the bars

**Remarks:** Sweep the bars

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.Sweep)

#### `Transform(transMat)`

Transform the placement

**Remarks:** Transform the placement

**Parameters:**
- `transMat` (Matrix3D) — Transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.Transform)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_Reinforcement/SweepBarPlacement/#NemAll_Python_Reinforcement.SweepBarPlacement.__repr__)

### Properties
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `PositionNumber` (int, get/set) — Get the position number

