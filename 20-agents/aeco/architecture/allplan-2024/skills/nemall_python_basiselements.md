---
name: allplan-nemall_python_basiselements
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_BasisElements namespace — 94 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ARGB, AllplanElement, AssociativeViewElement, AssociativeViewProperties, AssociativeViewElementRepresentation, BasisElement, AttributeContainer, BasisPropertyDialogs, and 86 more types.
---

# NemAll_Python_BasisElements

Auto-generated from vendor docs for allplan 2024.0. 94 types in this namespace.

## ARGB (class)

ARGB class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/)

### Constructors
- `ARGB() | ARGB(red, green, blue, alpha) | ARGB(argb) | ARGB(argb)` — Initialize

### Methods
#### `GetARGB()`

returns color as unsigned long

**Remarks:** returns color as unsigned long

**Returns:** `int` — color as long

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/#NemAll_Python_BasisElements.ARGB.GetARGB)

#### `__eq__(argb)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `argb` (ARGB) — color to compare

**Returns:** `bool` — true if they are equal

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/#NemAll_Python_BasisElements.ARGB.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/#NemAll_Python_BasisElements.ARGB.__repr__)

### Properties
- `Alpha` (None, get) — Property for alpha channel :type: None
- `Blue` (None, get) — Property for blue channel :type: None
- `Green` (None, get) — Property for green channel :type: None
- `Red` (None, get) — Property for red channel :type: None

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/)

### Methods
#### `GetAttributes()`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetAttributes)

#### `GetCommonProperties()`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetCommonProperties)

#### `GetGeometryObject()`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetGeometryObject)

#### `GetLabelElements()`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetLabelElements)

#### `GetSubElementID()`

Get the sub element ID

**Remarks:** Get the sub element ID

**Returns:** `type` — Sub Element ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (NemAll_Python_BaseElements.CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## AssociativeViewElement (class)

AssociativeViewElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElement/)

### Constructors
- `AssociativeViewElement()` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElement/#NemAll_Python_BasisElements.AssociativeViewElement.__repr__)

### Properties
- `ClippingPathProperties` (None, get) — Property for the clipping path properties :type: None
- `DimensionElements` (None, get) — Property for the dimension elements :type: None
- `PlacementAngle` (None, get) — Property for the placement angle :type: None
- `PlacementPoint` (None, get) — Property for the placement point :type: None
- `ReinforcementLabels` (None, get) — Property for the reinforcement labels :type: None
- `SectionAngle` (None, get) — Property for the section angle :type: None
- `SectionPolyhedron` (None, get) — Property for the section polyhedron :type: None
- `TextElements` (None, get) — Property for the text elements :type: None
- `ViewMatrix` (None, get) — Property for the view matrix :type: None
- `ViewProperties` (None, get) — Property for the view properties :type: None

## AssociativeViewElementRepresentation (enum)

Element representation of the associative view

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElementRepresentation/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `AssociativeViewElementRepresentation` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElementRepresentation/#NemAll_Python_BasisElements.AssociativeViewElementRepresentation.__getitem__)

### Values
- `AssociativeViewAllElements` = `0`
- `AssociativeViewConcreteShape` = `3`

## AssociativeViewProperties (class)

(No description provided in vendor docs for NemAll_Python_BasisElements.AssociativeViewProperties.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewProperties/)

### Constructors
- `AssociativeViewProperties() | AssociativeViewProperties(arg2) | AssociativeViewProperties(arg2)` — Initialize

### Properties
- `AdjacentEdges` (None, get) — Set/get the adjacent edges state :type: None
- `ColorHiddenEdge` (None, get) — Set/get the color of the hidden edges :type: None
- `ColorVisibleEdge` (None, get) — Set/get the color of the visible edges :type: None
- `ConvertTo2D` (None, get) — Set/get the convert to 2D state :type: None
- `ElementRepresentation` (None, get) — Set/get the element representation :type: None
- `Hidden` (None, get) — Set/get the hidden state :type: None
- `LayerBoundaryLine` (None, get) — Set/get the layer of the boundary line :type: None
- `LayerFinishLine` (None, get) — Set/get the layer of the finish line :type: None
- `LayerHiddenEdge` (None, get) — Set/get the layer of the hidden edges :type: None
- `LayerHiddenSectionLine` (None, get) — Set/get the layer of the hidden section line :type: None
- `LayerSectionLine` (None, get) — Set/get the layer of the section line :type: None
- `LayerVisibleEdge` (None, get) — Set/get the layer of the visible edges :type: None
- `PenHiddenEdge` (None, get) — Set/get the pen of the hidden edges :type: None
- `PenVisibleEdge` (None, get) — Set/get the pen of the visible edges :type: None
- `RemoveAdjacentEdges` (None, get) — Set/get the remove adjacent edges state :type: None
- `ShowHiddenEdges` (None, get) — Set/get the show hidden edges state :type: None
- `ShowSectionBody` (None, get) — :type: None
- `ShowVisibleEdges` (None, get) — Set/get the show visible edges state :type: None
- `StrokeHiddenEdge` (None, get) — Set/get the stroke of the hidden edges :type: None
- `StrokeVisibleEdge` (None, get) — Set/get the stroke of the visible edges :type: None

## AttributeContainer (class)

AttributeContainer class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AttributeContainer/)

### Constructors
- `AttributeContainer() | AttributeContainer(attributesObject)` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AttributeContainer/#NemAll_Python_BasisElements.AttributeContainer.__repr__)

## BasisElement (class)

(No description provided in vendor docs for NemAll_Python_BasisElements.BasisElement.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisElement/)

## BasisPropertyDialogs (class)

(No description provided in vendor docs for NemAll_Python_BasisElements.BasisPropertyDialogs.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisPropertyDialogs/)

### Methods
#### `OpenBitmapResourceDialog(doc, bitmapPath)`

Open the bitmap resource dialog

**Remarks:** Open the bitmap resource dialog

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `bitmapPath` (str) — Initial path

**Returns:** `str` — Selected bitmap path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisPropertyDialogs/#NemAll_Python_BasisElements.BasisPropertyDialogs.OpenBitmapResourceDialog)

#### `OpenRGBColorDialog(doc, color)`

Open the RGB color dialog

**Remarks:** Open the RGB color dialog

**Parameters:**
- `doc` (object) — Document
- `color` (int) — Current color

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisPropertyDialogs/#NemAll_Python_BasisElements.BasisPropertyDialogs.OpenRGBColorDialog)

## BitmapAreaElement (class)

BitmapAreaElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/)

### Constructors
- `BitmapAreaElement() | BitmapAreaElement(commonProp, bitmapAreaProp, geometryObject) | BitmapAreaElement(BitmapAreaElement)` — Initialize

### Methods
#### `GetBitmapAreaProperties()`

Get the BitmapArea properties

**Remarks:** Get the BitmapArea properties

**Returns:** `BitmapAreaProperties` — BitmapArea properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/#NemAll_Python_BasisElements.BitmapAreaElement.GetBitmapAreaProperties)

#### `SetBitmapAreaProperties(bitmapAreaProp)`

Set the bitmapArea properties

**Remarks:** Set the bitmapArea properties

**Parameters:**
- `bitmapAreaProp` (BitmapAreaProperties) — BitmapArea properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/#NemAll_Python_BasisElements.BitmapAreaElement.SetBitmapAreaProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/#NemAll_Python_BasisElements.BitmapAreaElement.__repr__)

## BitmapAreaProperties (class)

BitmapAreaProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaProperties/)

### Constructors
- `BitmapAreaProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (BitmapAreaProperties) — BitmapProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaProperties/#NemAll_Python_BasisElements.BitmapAreaProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaProperties/#NemAll_Python_BasisElements.BitmapAreaProperties.__repr__)

### Properties
- `BitmapName` (None, get) — Property for bitmap name of the bitmap area :type: None
- `DirectionToReferenceLine` (None, get) — Property for direction to reference line :type: None
- `ReferencePoint` (None, get) — Property for reference point :type: None
- `RotationAngle` (None, get) — Property for rotation angle :type: None
- `TransparentColor` (None, get) — Property for transparent color :type: None
- `TransparentColorTolerance` (None, get) — Property for transparent color tolerance :type: None
- `UseDirectionToReferenceLine` (None, get) — Property for usage of direction to reference line :type: None
- `UseMetricalValues` (None, get) — Property for metrical values :type: None
- `UsePixelMask` (None, get) — Property for usage of mask black pixels :type: None
- `UseReferencePoint` (None, get) — Property for usage of reference point :type: None
- `UseRepeatTile` (None, get) — Property for repeat tile :type: None
- `XOffset` (None, get) — Property for X offset value :type: None
- `XScalingFactor` (None, get) — Property for X scaling factor :type: None
- `YOffset` (None, get) — Property for Y offset value :type: None
- `YScalingFactor` (None, get) — Property for Y scaling factor :type: None

## BitmapDefinition (class)

(No description provided in vendor docs for NemAll_Python_BasisElements.BitmapDefinition.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/)

### Methods
#### `Create(bitmapName)`

Create a bitmap definition

**Remarks:** Create a bitmap definition

**Parameters:**
- `bitmapName` (str) — Path and name of the bitmap

**Returns:** `BitmapDefinition` — Bitmap definition

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.Create)

#### `Dump(deep)`

Args: deep:

**Remarks:** Args: deep:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.Dump)

#### `GetBitmapName()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetBitmapName)

#### `GetHeight()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetHeight)

#### `GetPixel(x, y)`

Args: x: y:

**Remarks:** Args: x: y:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetPixel)

#### `GetWidth()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetWidth)

#### `IsHDRI()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.IsHDRI)

#### `LoadBitmap()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.LoadBitmap)

#### `setName(resourceName)`

Args: resourceName:

**Remarks:** Args: resourceName:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.setName)

### Properties
- `AbsolutePath` (str, get/set) — 
- `RelativeName` (str, get/set) — 

## ClippingPathProperties (enum)

Clipping properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ClippingPathProperties/)

### Constructors
- `ClippingPathProperties()` — Initialize

### Properties
- `BottomLevel` (None, get) — Get/set the bottom level :type: None
- `IsClippingLineOn` (None, get) — Get/set the clipping line state :type: None
- `IsHeightFromElementOn` (None, get) — Get/set the height from element state :type: None
- `SectionIdentifier` (None, get) — Get/set the section identifier :type: None
- `SectionType` (None, get) — Property for type of section :type: None
- `TopLevel` (None, get) — Get/set the top level :type: None

## CombinationType (enum)

Definition of the combination type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/CombinationType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `CombinationType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/CombinationType/#NemAll_Python_BasisElements.CombinationType.__getitem__)

### Values
- `eVx` = `1`
- `eVy` = `2`
- `eVz` = `3`

## ConsiderType (enum)

Definition of consider type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ConsiderType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ConsiderType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ConsiderType/#NemAll_Python_BasisElements.ConsiderType.__getitem__)

### Values
- `eConsiderAutomatic` = `3`
- `eConsiderCeilingOpening` = `8`
- `eConsiderCeilingRecess` = `9`
- `eConsiderCeilingSurface` = `2`
- `eConsiderDoorOpening` = `5`
- `eConsiderFloorSurface` = `1`
- `eConsiderNiche` = `6`
- `eConsiderNothing` = `0`
- `eConsiderRecess` = `7`
- `eConsiderWindowOpening` = `4`

## DimensionLineElement (class)

DimensionLineElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/)

### Constructors
- `DimensionLineElement() | DimensionLineElement(arg2, dimensionPoints, placementPoint, directionVector)` — Initialize

### Methods
#### `GetDimensionPoints()`

Get the dimension points

**Remarks:** Get the dimension points

**Returns:** `Point3DList` — Dimension points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.GetDimensionPoints)

#### `GetDirectionVector()`

Get the direction vector

**Remarks:** Get the direction vector

**Returns:** `Vector2D` — Direction vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.GetDirectionVector)

#### `GetPlacementVector()`

Get the placement vector

**Remarks:** Get the placement vector

**Returns:** `Vector2D` — Placement vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.GetPlacementVector)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.__repr__)

## DimensionProperties (class)

DimensionProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionProperties/)

### Constructors
- `DimensionProperties() | DimensionProperties(arg2, arg3) | DimensionProperties(arg2)` — Initialize

### Properties
- `ElevationBaseOffset` (None, get) — Property for the elevation base offset :type: None
- `FontIDDimensionNumber` (None, get) — Property for the font ID of the dimension number :type: None
- `IsAbsoluteElevation` (None, get) — Property for the elevation base offset :type: None
- `LeadingCharacter` (None, get) — Property for the leading characters :type: None
- `PointSymbol` (None, get) — Property for the point symbol of the elevation :type: None
- `PointSymbolEnd` (None, get) — Property for the point symbol at the start of the dimension line :type: None
- `PointSymbolStart` (None, get) — Property for the point symbol at the start of the dimension line :type: None
- `TailingCharacters` (None, get) — Property for the tailing characters :type: None
- `TextHeightDimensionNumber` (None, get) — Property for the height of the dimension number :type: None
- `TextLocation` (None, get) — Property for the elevation base offset :type: None
- `TextOffset` (None, get) — Offset of text from dimension line :type: None

## Dimensioning (enum)

Type of the dimensioning

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Dimensioning/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `Dimensioning` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Dimensioning/#NemAll_Python_BasisElements.Dimensioning.__getitem__)

### Values
- `eDimensionLine` = `1`
- `eElevation` = `2`

## ElementGroupElement (class)

ElementGroupElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/)

### Constructors
- `ElementGroupElement() | ElementGroupElement(commonProp, elementGroupProp, elementGroupObjectList)` — Initialize

### Methods
#### `GetElementGroupProperties()`

Get the ElementGroup properties

**Remarks:** Get the ElementGroup properties

**Returns:** `ElementGroupProperties` — ElementGroup properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.GetElementGroupProperties)

#### `GetObjectList()`

Get the list of element group objects

**Remarks:** Get the list of element group objects

**Returns:** `list` — Element group object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.GetObjectList)

#### `SetElementGroupProperties(ElementGroupProp)`

Set the ElementGroup properties

**Remarks:** Set the ElementGroup properties

**Parameters:**
- `ElementGroupProp` (ElementGroupProperties) — ElementGroup properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.SetElementGroupProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.__repr__)

## ElementGroupProperties (class)

ElementGroupProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupProperties/)

### Constructors
- `ElementGroupProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (ElementGroupProperties) — ElementGroupProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupProperties/#NemAll_Python_BasisElements.ElementGroupProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupProperties/#NemAll_Python_BasisElements.ElementGroupProperties.__repr__)

### Properties
- `ModifiableFlag` (None, get) — Property for modifiable flag :type: None
- `Name` (None, get) — Property for name of element group :type: None
- `SubType` (None, get) — Property for macro sub type :type: None

## ElementNodeElement (class)

ElementNodeElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/)

### Constructors
- `ElementNodeElement() | ElementNodeElement(commonProp, elementNodeId, elementNodeObjectList)` — Initialize

### Methods
#### `GetObjectList()`

Get the list of element node objects

**Remarks:** Get the list of element node objects

**Returns:** `list` — Element node object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.GetObjectList)

#### `GetParentID()`

Get parent ID

**Remarks:** Get parent ID

**Returns:** `type` — UUID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.GetParentID)

#### `SetParentID(ParentID)`

Set the parent element id to this object

**Remarks:** Set the parent element id to this object

**Parameters:**
- `ParentID` (type) — (UUID) sub element id

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.SetParentID)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.__repr__)

## ElevationElement (class)

ElevationElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElevationElement/)

### Constructors
- `ElevationElement() | ElevationElement(dimensionPoints, placementVector, directionVector, settings)` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElevationElement/#NemAll_Python_BasisElements.ElevationElement.__repr__)

## EndSymbolsProperties (class)

EndSymbolsProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/EndSymbolsProperties/)

### Constructors
- `EndSymbolsProperties() | EndSymbolsProperties(startID, startSize, endID, endSize)` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (EndSymbolsProperties) — EndSymbolsProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/EndSymbolsProperties/#NemAll_Python_BasisElements.EndSymbolsProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/EndSymbolsProperties/#NemAll_Python_BasisElements.EndSymbolsProperties.__repr__)

### Properties
- `EndID` (None, get) — Property for end symbol ID :type: None
- `EndSize` (None, get) — Property for end symbol size :type: None
- `StartID` (None, get) — Property for start symbol ID :type: None
- `StartSize` (None, get) — Property for start symbol size :type: None

## FaceStyleElement (class)

FaceStyleElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/)

### Constructors
- `FaceStyleElement() | FaceStyleElement(commonProp, faceStyleProp, geometryObject) | FaceStyleElement(FaceStyleElement)` — Initialize

### Methods
#### `GetFaceStyleProperties()`

Get the face style properties

**Remarks:** Get the face style properties

**Returns:** `FaceStyleProperties` — face style properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/#NemAll_Python_BasisElements.FaceStyleElement.GetFaceStyleProperties)

#### `SetFaceStyleProperties(faceStyleProp)`

Set the FaceStyle properties

**Remarks:** Set the FaceStyle properties

**Parameters:**
- `faceStyleProp` (FaceStyleProperties) — face style properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/#NemAll_Python_BasisElements.FaceStyleElement.SetFaceStyleProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/#NemAll_Python_BasisElements.FaceStyleElement.__repr__)

## FaceStyleProperties (class)

FaceStyleProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleProperties/)

### Constructors
- `FaceStyleProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FaceStyleProperties) — FaceStyleProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleProperties/#NemAll_Python_BasisElements.FaceStyleProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleProperties/#NemAll_Python_BasisElements.FaceStyleProperties.__repr__)

### Properties
- `DirectionToReferenceLine` (None, get) — Property for direction to reference line :type: None
- `FaceStyleID` (None, get) — Property for face style ID :type: None
- `ReferencePoint` (None, get) — Property for reference point :type: None
- `RotationAngle` (None, get) — Property for rotation angle :type: None
- `UseDirectionToReferenceLine` (None, get) — Property for usage of direction to reference line :type: None
- `UseReferencePoint` (None, get) — Property for usage of reference point :type: None

## FillingElement (class)

FillingElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/)

### Constructors
- `FillingElement() | FillingElement(commonProp, gradientFillingProp, geometryObject) | FillingElement(FillingElement)` — Initialize

### Methods
#### `GetFillingProperties()`

Get the gradient filling properties

**Remarks:** Get the gradient filling properties

**Returns:** `FillingProperties` — GradientFillingProperties properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/#NemAll_Python_BasisElements.FillingElement.GetFillingProperties)

#### `SetFillingProperties(gradientFillingProp)`

Set the gradient filling properties

**Remarks:** Set the gradient filling properties

**Parameters:**
- `gradientFillingProp` (FillingProperties) — GradientFillingProperties properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/#NemAll_Python_BasisElements.FillingElement.SetFillingProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/#NemAll_Python_BasisElements.FillingElement.__repr__)

## FillingProperties (class)

FillingProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingProperties/)

### Constructors
- `FillingProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FillingProperties) — GradientFillingProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingProperties/#NemAll_Python_BasisElements.FillingProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingProperties/#NemAll_Python_BasisElements.FillingProperties.__repr__)

### Properties
- `DirectionToReferenceLine` (None, get) — Property for direction to reference line :type: None
- `FirstColor` (None, get) — Property for first color :type: None
- `RotationAngle` (None, get) — Property for rotation angle :type: None
- `SecondColor` (None, get) — Property for first color :type: None
- `ShadingType` (None, get) — Property for shading type :type: None
- `TranslationType` (None, get) — Property for color type :type: None
- `UseDirectionToReferenceLine` (None, get) — Property for usage of direction to reference line :type: None
- `UseGradientFilling` (None, get) — Property for usage of gradient filling type :type: None
- `VariantType` (None, get) — Property for variant type :type: None

## HatchingElement (class)

Implementation of the hatching element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/)

### Constructors
- `HatchingElement() | HatchingElement(commonProp, hatchingProp, polygon) | HatchingElement(element)` — Initialize

### Methods
#### `GetHatchingProperties()`

Get the hatching properties

**Remarks:** Get the hatching properties

**Returns:** `HatchingProperties` — Hatching properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/#NemAll_Python_BasisElements.HatchingElement.GetHatchingProperties)

#### `SetHatchingProperties(hatchingProp)`

Set the hatching properties

**Remarks:** Set the hatching properties

**Parameters:**
- `hatchingProp` (HatchingProperties) — Hatching properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/#NemAll_Python_BasisElements.HatchingElement.SetHatchingProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/#NemAll_Python_BasisElements.HatchingElement.__repr__)

### Properties
- `HatchingProperties` (HatchingProperties, get/set) — Get the hatching properties

## HatchingProperties (class)

Implementation of the hatching properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/)

### Constructors
- `HatchingProperties() | HatchingProperties(element)` — Initialize

### Methods
#### `SetBackgroundColorBGR(value)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as BGR

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.SetBackgroundColorBGR)

#### `SetBackgroundColorIRGB(value)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as IRGB

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.SetBackgroundColorIRGB)

#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (HatchingProperties) — HatchingProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.__repr__)

### Properties
- `BackgroundColor` (ARGB, get/set) — Get the background color
- `DirectionToReferenceLine` (int, get/set) — Get the index of the direction line
- `ExistAlignment` (bool, get/set) — Get the alignment state
- `HatchID` (int, get/set) — Get the hatch ID
- `IsScaleDependent` (bool, get/set) — Get the scale dependent state
- `ReferencePoint` (NemAll_Python_Geometry.Point2D, get/set) — Get the reference point
- `RotationAngle` (NemAll_Python_Geometry.Angle, get/set) — Get the rotation angle
- `UseBackgroundColor` (bool, get/set) — Get the use background color state
- `UseReferencePoint` (bool, get/set) — Get the use reference point state

## HeightDefinitionType (enum)

Height definition types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HeightDefinitionType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `HeightDefinitionType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HeightDefinitionType/#NemAll_Python_BasisElements.HeightDefinitionType.__getitem__)

### Values
- `eAverage` = `3`
- `eComponent` = `2`
- `eMacro` = `1`
- `eNone` = `0`

## HiddenSectionLinesProperties (class)

Visible hidden edges properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HiddenSectionLinesProperties/)

### Constructors
- `HiddenSectionLinesProperties()` — Initialize

### Properties
- `HiddenSectionLinesColor` (None, get) — Property for color for hidden section lines :type: None
- `HiddenSectionLinesLayer` (None, get) — Property for layer for hidden section lines :type: None
- `HiddenSectionLinesLineType` (None, get) — Property for line type for hidden section lines :type: None
- `HiddenSectionLinesPen` (None, get) — Property for pen for hidden section lines :type: None
- `IsHiddenSectionLinesColorFromLayer` (None, get) — Property for hidden section lines color from layer :type: None
- `IsHiddenSectionLinesLineTypeFromLayer` (None, get) — Property for hidden section lines line type from layer :type: None
- `IsHiddenSectionLinesOn` (None, get) — Property for the drawing of the hidden section lines :type: None
- `IsHiddenSectionLinesPenFromLayer` (None, get) — Property for hidden section lines pen from layer :type: None

## LabelElement (class)

Implementation of the Label element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/)

### Constructors
- `LabelElement() | LabelElement(text, labelType) | LabelElement(textElements, labelType) | LabelElement(element)` — Initialize

### Methods
#### `AddTextElement(text)`

Add a text element

**Remarks:** Add a text element

**Parameters:**
- `text` (TextElement) — Text element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/#NemAll_Python_BasisElements.LabelElement.AddTextElement)

#### `SetLabeledElement(labeledElement)`

Set the labeled element

**Remarks:** Set the labeled element

**Parameters:**
- `labeledElement` (BaseElementAdapter) — Labeled element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/#NemAll_Python_BasisElements.LabelElement.SetLabeledElement)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/#NemAll_Python_BasisElements.LabelElement.__repr__)

## LabelType (enum)

Type of the label

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `LabelType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelType/#NemAll_Python_BasisElements.LabelType.__getitem__)

### Values
- `eLabelArchDimensionLine` = `3`
- `eLabelBftSlabElementation` = `5`
- `eLabelBftWallElementation` = `4`
- `eLabelEng3DBarReinforcement` = `2`
- `eLabelNoText` = `7`
- `eLabelNormalText` = `0`
- `eLabelVariableText` = `1`

## LabelingProperties (class)

Labeling properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelingProperties/)

### Constructors
- `LabelingProperties()` — Initialize

### Properties
- `AddProjectionName` (None, get) — Property for adding the projection name :type: None
- `HeadingOn` (None, get) — Property for the heading on :type: None
- `HeadingText` (None, get) — Property for the heading text :type: None
- `IsScaleOn` (None, get) — Property for the scal on :type: None

## LibraryElement (class)

Implementation of the library element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/)

### Constructors
- `LibraryElement() | LibraryElement(libEleProp) | LibraryElement(libEle)` — Initialize

### Methods
#### `GetCount()`

Get the element count

**Remarks:** Get the element count

**Returns:** `int` — Element count

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetCount)

#### `GetGeometryElements(doc)`

Get the geometry elements

**Remarks:** Get the geometry elements

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `list` — Geometry elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetGeometryElements)

#### `GetMinMax(doc)`

Get the min/max box of the element

**Remarks:** Get the min/max box of the element

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `MinMax3D` — Min/max box of the element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetMinMax)

#### `GetProperties()`

Get the properties

**Remarks:** Get the properties

**Returns:** `LibraryElementProperties` — Properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetProperties)

#### `GetReferencePoint(doc)`

Get the reference point

**Remarks:** Get the reference point

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `Point3D` — Reference point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetReferencePoint)

#### `Move(moveVec)`

Move the element

**Remarks:** Move the element

**Parameters:**
- `moveVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.Move)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.__repr__)

## LibraryElementProperties (class)

Implementation of the library element properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/)

### Constructors
- `LibraryElementProperties() | LibraryElementProperties(fullPathName, elementType, placementMatrix) | LibraryElementProperties(fullPathName, elementType, placementMatrices) | LibraryElementProperties(path, group, element, elementType, placementMatrix) | LibraryElementProperties(path, group, element, elementType, placementMatrices) | LibraryElementProperties(arg2, path, group, element, elementType, placementMatrix) | LibraryElementProperties(arg2, path, group, element, elementType, placementMatrices)` — Initialize

### Methods
#### `GetElement()`

Get the element

**Remarks:** Get the element

**Returns:** `str` — Element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetElement)

#### `GetElementType()`

Get the element type

**Remarks:** Get the element type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetElementType)

#### `GetGroup()`

Get the group

**Remarks:** Get the group

**Returns:** `str` — Group

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetGroup)

#### `GetPath()`

Get the path

**Remarks:** Get the path

**Returns:** `str` — Path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPath)

#### `GetPlacementMatrices()`

Get the placement matrices

**Remarks:** Get the placement matrices

**Returns:** `Matrix3DList` — Placement matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPlacementMatrices)

#### `GetPlacementMatrix()`

Get the placement matrix

**Remarks:** Get the placement matrix

**Returns:** `Matrix3D` — Placement matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPlacementMatrix)

#### `GetPolyline()`

Get the polygon points in case of a line fixture

**Remarks:** Get the polygon points in case of a line fixture

**Returns:** `Polyline3D` — Polyline points of the line fixture

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPolyline)

#### `GetProducer()`

Get the producer

**Remarks:** Get the producer

**Returns:** `str` — Producer

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetProducer)

#### `GetSingleFilePath()`

Get the Single File Path

**Remarks:** Get the Single File Path

**Returns:** `str` — SingleFilePath

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetSingleFilePath)

#### `SetPlacementMatrices(placementMatrices)`

Set the placement matrices

**Remarks:** Set the placement matrices

**Parameters:**
- `placementMatrices` (Matrix3DList) — Placement matrices

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetPlacementMatrices)

#### `SetPlacementMatrix(placementMatrix)`

Set the placement matrix

**Remarks:** Set the placement matrix

**Parameters:**
- `placementMatrix` (Matrix3D) — Placement matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetPlacementMatrix)

#### `SetPolyline(polyline)`

Set the polygon points in case of a line fixture

**Remarks:** Set the polygon points in case of a line fixture

**Parameters:**
- `polyline` (Polyline3D) — Polyline of a line fixture

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetPolyline)

#### `SetProducer(producer)`

Set the producer

**Remarks:** Set the producer

**Parameters:**
- `producer` (str) — Producer

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetProducer)

### Properties
- `PlacementMatrices` (NemAll_Python_Geometry.Matrix3DList, get/set) — Get the placement matrices
- `PlacementMatrix` (NemAll_Python_Geometry.Matrix3D, get/set) — Get the placement matrix
- `Polyline` (NemAll_Python_Geometry.Polyline3D, get/set) — Get the polygon points in case of a line fixture
- `Producer` (str, get/set) — Get the producer

## LibraryElementType (enum)

(No description provided in vendor docs for NemAll_Python_BasisElements.LibraryElementType.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `LibraryElementType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementType/#NemAll_Python_BasisElements.LibraryElementType.__getitem__)

### Values
- `eFixture` = `1`
- `eFixtureSingleFile` = `3`
- `eSmartSymbol` = `0`
- `eSymbol` = `2`

## LinkType (enum)

Definition of link type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LinkType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `LinkType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LinkType/#NemAll_Python_BasisElements.LinkType.__getitem__)

### Values
- `eLinkNothing` = `0`
- `eLinkToCeilingSurface` = `3`
- `eLinkToFloorSurface` = `4`
- `eLinkToRoofSlab` = `2`
- `eLinkToRoom` = `1`

## MacroElement (class)

MacroElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/)

### Constructors
- `MacroElement() | MacroElement(macroProp, slideList)` — Initialize

### Methods
#### `GetHash()`

Get the hash value

**Remarks:** Get the hash value

**Returns:** `str` — Hash value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetHash)

#### `GetMacroProperties()`

Get the Macro properties

**Remarks:** Get the Macro properties

**Returns:** `MacroProperties` — Macro properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetMacroProperties)

#### `GetSlideList()`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetSlideList)

#### `SetHash(hash)`

Set the hash value

**Remarks:** Set the hash value

**Parameters:**
- `hash` (str) — Hash value

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.SetHash)

#### `SetMacroProperties(MacroProp)`

Set the Macro properties

**Remarks:** Set the Macro properties

**Parameters:**
- `MacroProp` (MacroProperties) — Macro properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.SetMacroProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.__repr__)

## MacroGroupElement (class)

Implementation of the macro group element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/)

### Constructors
- `MacroGroupElement() | MacroGroupElement(macroGroupProp, placementList) | MacroGroupElement(commonProp, macroGroupProp, placementList) | MacroGroupElement(element)` — Initialize

### Methods
#### `GetMacroGroupProperties()`

Get the macro group properties

**Remarks:** Get the macro group properties

**Returns:** `MacroGroupProperties` — MacroGroup properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.GetMacroGroupProperties)

#### `GetPlacementList()`

Get the placement list

**Remarks:** Get the placement list

**Returns:** `List[MacroPlacementElement]` — Placements of macro group

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.GetPlacementList)

#### `SetMacroGroupProperties(macroGroupProp)`

Set the macro group properties

**Remarks:** Set the macro group properties

**Parameters:**
- `macroGroupProp` (MacroGroupProperties) — MacroGroup properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.SetMacroGroupProperties)

#### `TransformElement(transMat)`

Args: transMat:

**Remarks:** Args: transMat:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.TransformElement)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.__repr__)

### Properties
- `MacroGroupProperties` (MacroGroupProperties, get/set) — Get the macro group properties

## MacroGroupProperties (class)

Implementation of the macro group element properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupProperties/)

### Constructors
- `MacroGroupProperties() | MacroGroupProperties(element)` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroGroupProperties) — MacroGroupProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupProperties/#NemAll_Python_BasisElements.MacroGroupProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupProperties/#NemAll_Python_BasisElements.MacroGroupProperties.__repr__)

### Properties
- `Name` (str, get/set) — Get the Name

## MacroPlacementElement (class)

Implementation of the macro placement element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/)

### Constructors
- `MacroPlacementElement() | MacroPlacementElement(commonProp, macroPlacementProp, macro, reinforcementList, libraryElementList=[], architectureElementsList=[], fixtureElementsList=[]) | MacroPlacementElement(placement)` — Initialize

### Methods
#### `GetArchitectureElementsList()`

Get the architecture elements

**Remarks:** Get the architecture elements

**Returns:** `list` — Architecture elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetArchitectureElementsList)

#### `GetAttributesList()`

Get the attributes list

**Remarks:** Get the attributes list

**Returns:** `List[AttributeSet]` — Attributes list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetAttributesList)

#### `GetFixtureElementsList()`

Get the fixture elements

**Remarks:** Get the fixture elements

**Returns:** `list` — Fixture elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetFixtureElementsList)

#### `GetLibraryElementsList()`

Get the library elements

**Remarks:** Get the library elements

**Returns:** `list` — Library elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetLibraryElementsList)

#### `GetMacro()`

Get the corresponding macro definition

**Remarks:** Get the corresponding macro definition

**Returns:** `MacroElement` — Macro definition element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetMacro)

#### `GetMacroPlacementProperties()`

Get the macro placement properties

**Remarks:** Get the macro placement properties

**Returns:** `MacroPlacementProperties` — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetMacroPlacementProperties)

#### `GetReinforcementList()`

Get the reinforcement elements

**Remarks:** Get the reinforcement elements

**Returns:** `list` — Reinforcement elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetReinforcementList)

#### `SetMacroPlacementProperties(macroPlacementProp)`

Set the macro placement properties

**Remarks:** Set the macro placement properties

**Parameters:**
- `macroPlacementProp` (MacroPlacementProperties) — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.SetMacroPlacementProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.__repr__)

### Properties
- `MacroPlacementProperties` (MacroPlacementProperties, get/set) — Get the macro placement properties

## MacroPlacementProperties (class)

Implementation of the macro placement element properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementProperties/)

### Constructors
- `MacroPlacementProperties() | MacroPlacementProperties(element)` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroPlacementProperties) — MacroPlacementProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementProperties/#NemAll_Python_BasisElements.MacroPlacementProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementProperties/#NemAll_Python_BasisElements.MacroPlacementProperties.__repr__)

### Properties
- `BillingCategory` (int, get/set) — Get the Billing category
- `ConsiderType` (ConsiderType, get/set) — Get the Consider type
- `Craft` (int, get/set) — Get the Craft
- `DistortionState` (bool, get/set) — Get the Distortion state
- `DomainType` (int, get/set) — Get the Domain type?
- `HasParentModificationBehaviour` (bool, get/set) — Get the Specific behavior for modification
- `HeightDefinitionType` (NemAll_Python_Precast.HeightDefinitionType, get/set) — Get the Height definition type
- `InOpeningState` (bool, get/set) — Get the Is the macro placement inside opening ?
- `LeadingMacro` (bool, get/set) — Get the leading macro (post activation flag for connected elements - iseg1)
- `LinkType` (LinkType, get/set) — Get the Link type
- `Mass_V6` (float, get/set) — Get the Mass attribute V6
- `Mass_V7` (float, get/set) — Get the Mass attribute V7
- `Mass_V8` (float, get/set) — Get the Mass attribute V8
- `Mass_V9` (float, get/set) — Get the Mass attribute V9
- `Matrix` (NemAll_Python_Geometry.Matrix3D, get/set) — Get the Matrix for location in world coordinate system
- `MirrorState` (bool, get/set) — Get the Was the macro placement mirrored ?
- `Name` (str, get/set) — Get the Name
- `PositionNr` (int, get/set) — Get the Unit factor
- `SubType` (int, get/set) — Get the Subtype?
- `Type` (int, get/set) — Get the Type?
- `UnitFactor` (float, get/set) — Get the Unit factor
- `UseAlways2DRepInGroundView` (bool, get/set) — Get the Use always 2D representation in ground view
- `UseDrawOrder` (bool, get/set) — Get the Uses the draw order setting of the placement or the elements of the macro ?
- `UseFormat` (bool, get/set) — Get the Uses the format setting (pen, stroke, color) of the placement or the elements of the macro ?

## MacroProperties (class)

Implementation of the macro definition element properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroProperties/)

### Constructors
- `MacroProperties() | MacroProperties(element)` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroProperties) — MacroProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroProperties/#NemAll_Python_BasisElements.MacroProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroProperties/#NemAll_Python_BasisElements.MacroProperties.__repr__)

### Properties
- `CatalogName` (str, get/set) — Get the Catalog name
- `DomainType` (int, get/set) — Get the Domain type
- `InsertionPoint` (NemAll_Python_Geometry.Point3D, get/set) — Get the Insertion point
- `IsScaleDependent` (bool, get/set) — Get the Is macro scale dependent ?
- `Name` (str, get/set) — Get the Name
- `PositionNr` (int, get/set) — Get the Position number
- `SubType` (int, get/set) — Get the Sub type
- `UnitFactor` (float, get/set) — Get the Unit factor

## MacroSlideElement (class)

MacroSlideElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/)

### Constructors
- `MacroSlideElement() | MacroSlideElement(macroSlideProp, objectList)` — Initialize

### Methods
#### `GetMacroSlideProperties()`

Get the MacroSlide properties

**Remarks:** Get the MacroSlide properties

**Returns:** `MacroSlideProperties` — MacroSlide properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.GetMacroSlideProperties)

#### `GetObjectList()`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.GetObjectList)

#### `SetMacroSlideProperties(MacroSlideProp)`

Set the MacroSlide properties

**Remarks:** Set the MacroSlide properties

**Parameters:**
- `MacroSlideProp` (MacroSlideProperties) — MacroSlide properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.SetMacroSlideProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.__repr__)

## MacroSlideProperties (class)

Implementation of the macro slide element properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideProperties/)

### Constructors
- `MacroSlideProperties() | MacroSlideProperties(element)` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroSlideProperties) — MacroSlideProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideProperties/#NemAll_Python_BasisElements.MacroSlideProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideProperties/#NemAll_Python_BasisElements.MacroSlideProperties.__repr__)

### Properties
- `EndScaleRange` (float, get/set) — Get the Start reference scale of slide
- `OffsetOfReferencePoint1` (NemAll_Python_Geometry.Vector3D, get/set) — Get the First offset value to reference point
- `OffsetOfReferencePoint2` (NemAll_Python_Geometry.Vector3D, get/set) — Get the Second offset value to reference point
- `ReferencePoint` (NemAll_Python_Geometry.Point3D, get/set) — Get the Reference point of this slide
- `ResizeSettingVx` (CombinationType, get/set) — Get the Resize setting for x direction
- `ResizeSettingVy` (CombinationType, get/set) — Get the Resize setting for y direction
- `ResizeSettingVz` (CombinationType, get/set) — Get the Resize setting for z direction
- `StartScaleRange` (float, get/set) — Get the Start reference scale of slide
- `Type` (MacroSlideType, get/set) — Get the Type of macro slide
- `VisibilityGeo2D` (bool, get/set) — Get the Geometry 2D visibility of slide
- `VisibilityGeo3D` (bool, get/set) — Get the Geometry 3D visibility of slide
- `VisibilityLayerA` (bool, get/set) — Get the Layer A visibility of slide
- `VisibilityLayerB` (bool, get/set) — Get the Layer B visibility of slide
- `VisibilityLayerC` (bool, get/set) — Get the Layer C visibility of slide

## MacroSlideType (enum)

Definition of the macro slide type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `MacroSlideType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideType/#NemAll_Python_BasisElements.MacroSlideType.__getitem__)

### Values
- `eCode` = `1`
- `eExtension` = `5`
- `eGeometry` = `0`
- `eReinforcement` = `2`
- `eReport` = `3`
- `eUndergroundCadaster` = `4`

## ModelElement2D (class)

Implementation of the 2D model element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/)

### Constructors
- `ModelElement2D() | ModelElement2D(commonProp, geometryObject) | ModelElement2D(commonProp, patternCurveProp, endSymbolProp, geometryObject) | ModelElement2D(element)` — Initialize

### Methods
#### `GetEndSymbolsProperties()`

Get the end symbols properties

**Remarks:** Get the end symbols properties

**Returns:** `EndSymbolsProperties` — End symbols properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.GetEndSymbolsProperties)

#### `GetPatternCurveProperties()`

Get the pattern curve properties

**Remarks:** Get the pattern curve properties

**Returns:** `PatternCurveProperties` — Pattern curve properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.GetPatternCurveProperties)

#### `GetTransformationList()`

Get transformation list

**Remarks:** Get transformation list

**Returns:** `list` — List with the transformations

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.GetTransformationList)

#### `SetEndSymbolsProperties(endSymbolsProp)`

Set the end symbols properties

**Remarks:** Set the end symbols properties

**Parameters:**
- `endSymbolsProp` (EndSymbolsProperties) — End symbols properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.SetEndSymbolsProperties)

#### `SetPatternCurveProperties(patternCurveProp)`

Set the pattern curve properties

**Remarks:** Set the pattern curve properties

**Parameters:**
- `patternCurveProp` (PatternCurveProperties) — Pattern curve properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.SetPatternCurveProperties)

#### `SetTransformationList(transformationList)`

Set the transformation list

**Remarks:** Set the transformation list

**Parameters:**
- `transformationList` (list) — List with the transformations

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.SetTransformationList)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.__repr__)

### Properties
- `EndSymbolsProperties` (EndSymbolsProperties, get/set) — Get the end symbols properties
- `PatternCurveProperties` (PatternCurveProperties, get/set) — Get the pattern curve properties
- `TransformationList` (list, get/set) — Get transformation list

## ModelElement3D (class)

Implementation of the 3D model element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/)

### Constructors
- `ModelElement3D() | ModelElement3D(commonProp, geometryObject) | ModelElement3D(commonProp, BrepIsoLinesU, BrepIsoLinesV, geometryObject) | ModelElement3D(commonProp, textureDefinition, geometryObject) | ModelElement3D(commonProp, textureDefinition, BrepIsoLinesU, BrepIsoLinesV, geometryObject) | ModelElement3D(commonProp, textureDefinition, textureMapping, geometryObject) | ModelElement3D(commonProp, textureDefinition, textureMapping, BrepIsoLinesU, BrepIsoLinesV, geometryObject) | ModelElement3D(element)` — Initialize

### Methods
#### `GetTextureDefinition()`

Get the texture definition

**Remarks:** Get the texture definition

**Returns:** `TextureDefinition` — Texture definition (surface filename)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.GetTextureDefinition)

#### `GetTextureMapping()`

Get the texture mapping

**Remarks:** Get the texture mapping

**Returns:** `TextureMapping` — Texture mapping properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.GetTextureMapping)

#### `GetTransformationList()`

Get transformation list

**Remarks:** Get transformation list

**Returns:** `list` — List with the transformations

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.GetTransformationList)

#### `IsValidateGeometry()`

Get the validate geometry state

**Remarks:** Get the validate geometry state

**Returns:** `bool` — Validate the geometry state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.IsValidateGeometry)

#### `SetTextureDefinition(textureDefinition)`

Set the texture definition

**Remarks:** Set the texture definition

**Parameters:**
- `textureDefinition` (TextureDefinition) — Texture definition (surface filename)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetTextureDefinition)

#### `SetTextureMapping(textureMapping)`

Set the texture mapping

**Remarks:** Set the texture mapping

**Parameters:**
- `textureMapping` (TextureMapping) — Texture mapping properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetTextureMapping)

#### `SetTransformationList(transformationList)`

Set the transformation list

**Remarks:** Set the transformation list

**Parameters:**
- `transformationList` (list) — List with the transformations

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetTransformationList)

#### `SetValidateGeometry(validateGeometry)`

Set the validate geometry state

**Remarks:** Set the validate geometry state

**Parameters:**
- `validateGeometry` (bool) — Validate the geometry

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetValidateGeometry)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.__repr__)

### Properties
- `TextureDefinition` (TextureDefinition, get/set) — Get the texture definition
- `TextureMapping` (TextureMappingProperties, get/set) — Get the texture mapping
- `TransformationList` (list, get/set) — Get transformation list

## PatternCurveAlignment (enum)

Pattern curve alignment types of the pattern curve property

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveAlignment/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PatternCurveAlignment` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveAlignment/#NemAll_Python_BasisElements.PatternCurveAlignment.__getitem__)

### Values
- `eCenter` = `1`
- `eLeft` = `0`
- `eRight` = `2`

## PatternCurveIntersectionType (enum)

Pattern intersection types of the pattern curve property

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveIntersectionType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PatternCurveIntersectionType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveIntersectionType/#NemAll_Python_BasisElements.PatternCurveIntersectionType.__getitem__)

### Values
- `eDisabled` = `0`
- `eJoint` = `2`
- `eMiter` = `1`
- `eSeamless` = `3`

## PatternCurveProperties (class)

PatternCurveProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveProperties/)

### Constructors
- `PatternCurveProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PatternCurveProperties) — PatternCurveProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveProperties/#NemAll_Python_BasisElements.PatternCurveProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveProperties/#NemAll_Python_BasisElements.PatternCurveProperties.__repr__)

### Properties
- `AlignmentType` (None, get) — Property for alignment type :type: None
- `Height` (None, get) — Property for height of pattern :type: None
- `IntersectionType` (None, get) — Property for intersection type :type: None
- `IsDrawReferenceCurve` (None, get) — Property for draw reference curve state :type: None
- `IsLockedToCorner` (None, get) — Property for locked to corner state :type: None
- `IsMirrorPattern` (None, get) — Property for mirror pattern state :type: None
- `IsScaleDependent` (None, get) — Property for scale dependent :type: None
- `PatternID` (None, get) — Property for pattern ID :type: None
- `Width` (None, get) — Property for width of pattern :type: None

## PatternElement (class)

Implementation of the Pattern element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/)

### Constructors
- `PatternElement() | PatternElement(commonProp, patternProp, polygon) | PatternElement(element)` — Initialize

### Methods
#### `GetPatternProperties()`

Get the Pattern properties

**Remarks:** Get the Pattern properties

**Returns:** `PatternProperties` — Pattern properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/#NemAll_Python_BasisElements.PatternElement.GetPatternProperties)

#### `SetPatternProperties(patternProp)`

Set the Pattern properties

**Remarks:** Set the Pattern properties

**Parameters:**
- `patternProp` (PatternProperties) — Pattern properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/#NemAll_Python_BasisElements.PatternElement.SetPatternProperties)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/#NemAll_Python_BasisElements.PatternElement.__repr__)

### Properties
- `PatternProperties` (PatternProperties, get/set) — Get the Pattern properties

## PatternProperties (class)

Implementation of the pattern properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/)

### Constructors
- `PatternProperties() | PatternProperties(element)` — Initialize

### Methods
#### `SetBackgroundColorBGR(value)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as BGR

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.SetBackgroundColorBGR)

#### `SetBackgroundColorIRGB(value)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as IRGB

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.SetBackgroundColorIRGB)

#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PatternProperties) — PatternProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.__repr__)

### Properties
- `BackgroundColor` (ARGB, get/set) — Get the background color
- `IsScaleDependent` (bool, get/set) — Get the scale dependent state
- `PatternID` (int, get/set) — Get the pattern ID
- `PlacementType` (PlacementType, get/set) — Get the placement type
- `ReferencePoint` (NemAll_Python_Geometry.Point2D, get/set) — Get the reference point
- `RotationAngle` (NemAll_Python_Geometry.Angle, get/set) — Get the rotation angle
- `UseBackgroundColor` (bool, get/set) — Get the use background color state
- `UseReferencePoint` (bool, get/set) — Get the use reference point state
- `XScalingFactor` (float, get/set) — Get the scaling factor in x direction
- `YScalingFactor` (float, get/set) — Get the scaling factor in y direction

## PlacementType (enum)

Definition of the pattern placement type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PlacementType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `PlacementType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PlacementType/#NemAll_Python_BasisElements.PlacementType.__getitem__)

### Values
- `eFitting` = `1`
- `eInsideFitting` = `2`
- `eOutsideFitting` = `0`

## SectionAlongPathClippingPathProperties (class)

Clipping path properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/)

### Constructors
- `SectionAlongPathClippingPathProperties(bInitFromSTW=False) | SectionAlongPathClippingPathProperties(B)` — constructor

### Methods
#### `GetClippingPathGeometryTolerance()`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — ClippingPathGeometryTolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.GetClippingPathGeometryTolerance)

#### `GetPathLength()`

Get length of path

**Remarks:** Get length of path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.GetPathLength)

#### `GetResultPathLength()`

Get length of the part of path limited by EndCoord and StartCoord

**Remarks:** Get length of the part of path limited by EndCoord and StartCoord

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.GetResultPathLength)

#### `RestrictEmptySectionId()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.RestrictEmptySectionId)

### Properties
- `BottomLevel` (float, get/set) — Get the BottomLevel for section Vertical or Horizontal_From_Top
- `ClippingPathViewProperties` (SectionAlongPathClippingPathViewProperties, get/set) — Get the SectionAlongPathClippingPathViewProperties
- `EndCoord` (float, get/set) — Get the EndCoord value
- `IsChangeViewDirectionOn` (bool, get/set) — Get the IsChangeViewDirectionOn status : ON/OFF
- `IsClippingLineOn` (bool, get/set) — Get the Is clipping path line on
- `IsHeightFromElementOn` (bool, get/set) — Get the IsHeightFromElementOn status : ON/OFF
- `SectionIdentifier` (str, get/set) — Get the Section identifier
- `StartCoord` (float, get/set) — Get the StartCoord value
- `StationingEnd` (float, get/set) — Get end stationing
- `StationingStart` (float, get/set) — Get start stationing
- `TopLevel` (float, get/set) — Get the TopLevel for section Vertical or Horizontal_From_Bottom

## SectionAlongPathClippingPathViewProperties (class)

Clipping path view properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/)

### Constructors
- `SectionAlongPathClippingPathViewProperties(bInitFromSTW=False) | SectionAlongPathClippingPathViewProperties(B)` — constructor

### Methods
#### `ConvertDirectionSymbolNumberFromViewModel(arg2, iPaletteIconNum)`

Convert icon number used in WPF palette to symbol number used in Allplan for the same graphical representation

**Remarks:** Convert icon number used in WPF palette to symbol number used in Allplan for the same graphical representation

**Parameters:**
- `iPaletteIconNum` (bool) — number of icon from palette (view model)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathViewProperties.ConvertDirectionSymbolNumberFromViewModel)

#### `ConvertDirectionSymbolNumberToViewModel(arg2)`

Convert symbol number used in Allplan to icon number used in WPF palette for the same graphical representation

**Remarks:** Convert symbol number used in Allplan to icon number used in WPF palette for the same graphical representation

**Returns:** `int` — icon number used in WPF

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathViewProperties.ConvertDirectionSymbolNumberToViewModel)

#### `GetDirectionSymbolHeightTolerance()`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — symbol height tolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathViewProperties.GetDirectionSymbolHeightTolerance)

### Properties
- `DirectionSymbolHeight` (float, get/set) — Get the DirectionSymbolHeight
- `DirectionSymbolNr` (int, get/set) — Get the DirectionSymbolNr
- `IsDirectionSymbolOn` (bool, get/set) — Get the IsDirectionSymbolOn
- `TextParameterProperties` (SectionAlongPathTextParameterProperties, get/set) — Access text params Get the SectionAlongPathTextParameterProperties

## SectionAlongPathElement (class)

Implementation of the section along path element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathElement/)

### Constructors
- `SectionAlongPathElement(sectionAlongPathProperties, sectionPathElement) | SectionAlongPathElement(sectionAlongPathProperties, sectionPath, comProp) | SectionAlongPathElement(element)` — Default constructor

## SectionAlongPathElevationSpecifications (class)

Elevation specifications

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathElevationSpecifications/)

### Constructors
- `SectionAlongPathElevationSpecifications(bInitFromSTW=False) | SectionAlongPathElevationSpecifications(B)` — constructor

### Properties
- `DecimalPlaces` (int, get/set) — Get the number of places after decimal point
- `FontHeight` (float, get/set) — Get the Height of the character set
- `FontNameStrg` (str, get/set) — Get the Name the the character set
- `HeightWidthFactor` (float, get/set) — Get the height/width ratio
- `IsAbsoluteElevation` (bool, get/set) — Get the Absolute elevation
- `IsDimensionLineVisible` (bool, get/set) — Get the visibility of dimension line
- `IsDimensionNumberVisible` (bool, get/set) — Get the Dimension number is visible
- `IsFixFraction` (bool, get/set) — Get the Fix fraction
- `IsFontBold` (bool, get/set) — Get the character set Bold
- `IsFontItalic` (bool, get/set) — Get the character set Italic
- `IsFontUnderline` (bool, get/set) — Get the character set Underline
- `IsFreeElevation` (bool, get/set) — Get the Free elevation
- `IsLineColorFromLayer` (bool, get/set) — Get the settings from layers three checkboxes on third page in layer dialog like in BBasisDimSettings.h (do not know, why is it is doubled)
- `IsLineColorSameForAll` (bool, get/set) — Get the the same line color for DMline,auxiliary line and number/texts
- `IsLineColorTake` (bool, get/set) — Get the settings from layers three checkboxes on third page in layer dialog
- `IsModifiedIsTextOpaque` (bool, get/set) — Get the is text opaque
- `IsPenFromLayer` (bool, get/set) — Get the settings from layers three checkboxes on third page in layer dialog like in BasisDimSettings.h (do not know, why is it is doubled)
- `IsPenSameForAll` (bool, get/set) — Get the the same pen for DMline,auxiliary line and number/texts
- `IsPenSymbolFlag` (bool, get/set) — Get the when true -> pen for symbol as help construction line
- `IsPenTake` (bool, get/set) — Get the settings from layers three checkboxes on third page in layer dialog
- `IsPlusMinusSign` (bool, get/set) — Get the Draw +- sign before zero number
- `IsPositiveSign` (bool, get/set) — Get the Draw +s ign before positive number
- `IsTextSymbolFlag` (bool, get/set) — Get the when true -> pen for text as help construction line
- `LayerId` (int, get/set) — Get the Layer Number
- `LeadingCharacters` (str, get/set) — Get the prefix of dimension number/additional number(16)
- `MeasuredValueUnit` (int, get/set) — Get the Unit of the measured value
- `PointSymbol` (int, get/set) — Get the Point symbol
- `PointSymbolSize` (float, get/set) — Get the symbol size in mm/inch
- `RoundOffInch` (float, get/set) — Get the round of number inch units
- `RoundOffNormal` (float, get/set) — Get the round of number metric units
- `SymbolColorId` (int, get/set) — Get the Symbol color
- `SymbolPenId` (int, get/set) — Get the Symbol pen
- `TailingCharacters` (str, get/set) — Get the suffix of dimension number/additional number(16)
- `TailingZeros` (int, get/set) — Get the number of zeros after decimal point
- `TextColorId` (int, get/set) — Get the Text color
- `TextLocation` (int, get/set) — Get the text location from position / location of texts (up to enum eTextLocation)
- `TextOffset` (float, get/set) — Get the distance -> offset of texts from dimension line
- `TextPenId` (int, get/set) — Get the Text pen

## SectionAlongPathFilterProperties (class)

Filter properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFilterProperties/)

### Constructors
- `SectionAlongPathFilterProperties(bInitFromSTW=False) | SectionAlongPathFilterProperties(B)` — constructor

### Properties
- `DrawingFilesProperties` (SectionDrawingFilesProperties, get/set) — Access drawing file params Get the SectionDrawingFilesProperties
- `IsAssociativityOn` (bool, get/set) — Get the When the section is frozen / associativity is OFF -, no updates are available for it
- `IsSelectionFilterOn` (bool, get/set) — Get the when == true - only selected elements are included to UVS model / when == false (default value) - selected elements are excluded from UVS model
- `LayerProperties` (SectionLayerProperties, get/set) — Access layer params Get the SectionLayerProperties

## SectionAlongPathFormatProperties (class)

Format properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFormatProperties/)

### Constructors
- `SectionAlongPathFormatProperties(bInitFromSTW=False) | SectionAlongPathFormatProperties(B)` — constructor

### Methods
#### `GetEliminationAngleTolerance()`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — elimination angle tolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFormatProperties/#NemAll_Python_BasisElements.SectionAlongPathFormatProperties.GetEliminationAngleTolerance)

#### `GetOverhangTolerance()`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — overhang tolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFormatProperties/#NemAll_Python_BasisElements.SectionAlongPathFormatProperties.GetOverhangTolerance)

### Properties
- `AdjacentEdgeTypeSwitcher` (int, get/set) — Get the 0=material, 1=surface, we set a tooltip and description in palette
- `BoundaryLineColor` (int, get/set) — Get the Border: Boundary Line color index
- `BoundaryLineLayer` (int, get/set) — Get the Border: Boundary Line Layer number
- `BoundaryLineLayerKenner` (int, get/set) — Gets layer kenner for Boundary Line Sets layer kenner for Boundary Line
- `BoundaryLinePen` (int, get/set) — Get the Border: Boundary Line pen index (0== not used)
- `BoundaryLineType` (int, get/set) — Get the Border: Boundary Line stroke index
- `EdgesVisibility` (bool, get/set) — Get the Outer edges/All edges Visible 0, 1
- `EliminationAngle` (float, get/set) — Get the Angle of the Adjacent Edge elimination, //always in degree
- `IsBetweenDifferentSurfacesOn` (bool, get/set) — Get the Adjacent Edge Between Different Surfaces
- `IsBoundaryLineColorFromLayer` (bool, get/set) — Get the Border: IsBoundaryLineColorFromLayer
- `IsBoundaryLinePenFromLayer` (bool, get/set) — Get the Border: IsBoundaryLinePenFromLayer
- `IsBoundaryLineTypeFromLayer` (bool, get/set) — Get the Border: IsBoundaryLineTypeFromLayer
- `IsBoundaryLineVisible` (bool, get/set) — Get the BoundaryLine visibility
- `IsEliminationOn` (bool, get/set) — Get the Adjacent Edge elimination
- `IsSectionLineColorFromLayer` (bool, get/set) — Get the Section: IsSectionLineColorFromLayer
- `IsSectionLinePenFromLayer` (bool, get/set) — Get the Section: IsSectionLinePenFromLayer
- `IsSectionLineThicknessVisible` (bool, get/set) — Get the SectionLineThickness visibility
- `IsSectionLineTypeFromLayer` (bool, get/set) — Get the Section: IsSectionLineTypeFromLayer
- `IsSurfaceFromObjectOn` (bool, get/set) — Get the Display Surface From Object
- `Overhang` (float, get/set) — Get the Overhang
- `SectionLineColor` (int, get/set) — Get the Section: Boundary Line color index
- `SectionLineLayer` (int, get/set) — Get the Section: Section Line Layer number
- `SectionLineLayerKenner` (int, get/set) — Gets layer kenner for Section Line Sets layer kenner for Section Line
- `SectionLinePen` (int, get/set) — Get the Section: Boundary Line pen index (0== not used)
- `SectionLineType` (int, get/set) — Get the Section: Boundary Line stroke index

## SectionAlongPathGeneralSectionProperties (class)

General section properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/)

### Constructors
- `SectionAlongPathGeneralSectionProperties(bInitFromSTW=False) | SectionAlongPathGeneralSectionProperties(B)` — constructor

### Methods
#### `GetOffset()`

Get offset

**Remarks:** Get offset

**Returns:** `bool` — tuple(true if it was read from 55th element,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/#NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties.GetOffset)

#### `IsOffsetValid()`

Get offset state

**Remarks:** Get offset state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/#NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties.IsOffsetValid)

#### `SetOffset(dOffset_X, dOffset_Y)`

Set offset

**Remarks:** Set offset

**Parameters:**
- `dOffset_X` (float) — X coordinate of section's left bottom corner
- `dOffset_Y` (float) — Y coordinate of section's left bottom corner

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/#NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties.SetOffset)

### Properties
- `ElevationSpecifications` (SectionAlongPathElevationSpecifications, get/set) — Get the SectionAlongPathElevationSpecifications
- `FormatProperties` (SectionAlongPathFormatProperties, get/set) — Get the SectionAlongPathFormatProperties
- `LabelingStripSetting` (SectionAlongPathLabelingStripSetting, get/set) — Get the SectionAlongPathLabelingStripSetting
- `Offset_X` (float, get/set) — Get the X coordinate of left bottom corner
- `Offset_Y` (float, get/set) — Get the Y coordinate of left bottom corner
- `SectionLabelingProperties` (SectionAlongPathSectionLabelingProperties, get/set) — Get the SectionAlongPathSectionLabelingProperties
- `SectionViewProperties` (SectionAlongPathSectionViewProperties, get/set) — Get the SectionAlongPathSectionViewProperties

## SectionAlongPathLabelingStripSetting (class)

Labeling strip settings

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathLabelingStripSetting/)

### Constructors
- `SectionAlongPathLabelingStripSetting(bInitFromSTW=False) | SectionAlongPathLabelingStripSetting(B)` — constructor

### Properties
- `Distance_1` (float, get/set) — Get the Distance 1 in mm
- `Distance_2` (float, get/set) — Get the Distance 2 in mm
- `Distance_3` (float, get/set) — Get the Distance 3 in mm
- `Distance_4` (float, get/set) — Get the Distance 4 in mm
- `Distance_5` (float, get/set) — Get the Distance 5 in mm
- `Distance_6` (float, get/set) — Get the Distance 6 in mm
- `Distance_7` (float, get/set) — Get the Distance 7 in mm
- `HeightLines_Color` (int, get/set) — Get the Height lines color
- `HeightLines_HeightZone` (int, get/set) — Get the Height lines height zone
- `HeightLines_LineType` (int, get/set) — Get the Height lines line type
- `HeightLines_Pen` (int, get/set) — Get the Height lines pen
- `HeightLines_ProfileZone` (int, get/set) — Get the Height lines profile zone
- `HeightLines_Splay` (int, get/set) — Get the Height lines splay
- `HeightLines_StationZone` (int, get/set) — Get the Height lines station zone
- `HeightText_Color` (int, get/set) — Get the Height text color
- `HeightText_Distance` (float, get/set) — Get the Height text distance in mm
- `HeightText_Height` (float, get/set) — Get the Height text height in mm
- `HeightText_Pen` (int, get/set) — Get the Height text pen
- `HeightText_Precision` (int, get/set) — Get the Height text decimals
- `HeightText_Width` (float, get/set) — Get the Height text width in mm
- `ProfileAx_Color` (int, get/set) — Get the Profile axis color
- `ProfileAx_LineType` (int, get/set) — Get the Profile axis line type
- `ProfileAx_Pen` (int, get/set) — Get the Profile axis pen
- `ProfileFrame_Color` (int, get/set) — Get the Profile frame color
- `ProfileFrame_LineType` (int, get/set) — Get the Profile frame line type
- `ProfileFrame_Pen` (int, get/set) — Get the Profile frame pen
- `ProfileLine_Color` (int, get/set) — Get the Profile line color
- `ProfileLine_LineType` (int, get/set) — Get the Profile line line type
- `ProfileLine_Muster` (int, get/set) — Get the Profile line pattern
- `ProfileLine_Pen` (int, get/set) — Get the Profile line pen
- `RefererenzeHeight_Distance` (float, get/set) — Get the Reference height text distance in mm
- `RefererenzeHeight_Height` (float, get/set) — Get the Reference height text height in mm
- `RefererenzeHeight_TextColor` (int, get/set) — Get the Reference height text color
- `RefererenzeHeight_TextPen` (int, get/set) — Get the Reference height text pen
- `RefererenzeHeight_TextPrecision` (int, get/set) — Get the Reference height text decimals
- `RefererenzeHeight_Width` (float, get/set) — Get the Reference height text width in mm
- `StationText_Color` (int, get/set) — Get the Station text color
- `StationText_Distance` (float, get/set) — Get the Station text distance in mm
- `StationText_Height` (float, get/set) — Get the Station text height in mm
- `StationText_Pen` (int, get/set) — Get the Station text pen
- `StationText_Precision` (int, get/set) — Get the Station text decimals
- `StationText_Width` (float, get/set) — Get the Station text width in mm

## SectionAlongPathProperties (class)

Section along path properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathProperties/)

### Constructors
- `SectionAlongPathProperties(bInitFromSTW=False) | SectionAlongPathProperties(B)` — constructor

### Properties
- `ClippingPathProperties` (SectionAlongPathClippingPathProperties, get/set) — Get the SectionAlongPathClippingPathProperties
- `FilterProperties` (SectionAlongPathFilterProperties, get/set) — Get the SectionAlongPathFilterProperties
- `GeneralSectionProperties` (SectionAlongPathGeneralSectionProperties, get/set) — Get the SectionAlongPathGeneralSectionProperties
- `ScaleProperties` (SectionAlongPathScaleProperties, get/set) — Get the SectionAlongPathScaleProperties

## SectionAlongPathScaleProperties (class)

Scale properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathScaleProperties/)

### Constructors
- `SectionAlongPathScaleProperties(bInitFromSTW=False) | SectionAlongPathScaleProperties(B)` — constructor

### Methods
#### `GetScaleFactorTolerance()`

Access tolerance

**Remarks:** Access tolerance

**Returns:** `float` — scale factor tolerance

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathScaleProperties/#NemAll_Python_BasisElements.SectionAlongPathScaleProperties.GetScaleFactorTolerance)

### Properties
- `ScaleFactorX` (float, get/set) — Get the resizing factor X in Longitudinal direction
- `ScaleFactorY` (float, get/set) — Get the resizing factor Y in Transversal direction

## SectionAlongPathSectionLabelingProperties (enum)

Section labeling properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionLabelingProperties/)

### Constructors
- `SectionAlongPathSectionLabelingProperties(bInitFromSTW=False) | SectionAlongPathSectionLabelingProperties(B)` — constructor

### Properties
- `HeadingText` (str, get/set) — Get the HeadingText "longitudinal profile", "transversal profile", "elevation", "section", "Unfold", . and "none" (expandable)
- `HeadingTextParameters` (SectionAlongPathTextParameterProperties, get/set) — Get the SectionAlongPathTextParameterProperties
- `IsHeadingOn` (bool, get/set) — Get the IsHeadingOn
- `IsScaleOn` (bool, get/set) — Get the IsScaleOn
- `LabelingOffset` (float, get/set) — Get the Distance of the labeling from its content's min/max box
- `LabelingPosition` (int, get/set) — Get the Position of the labeling with respect to its content's min/max box
- `LabelingsDistance` (float, get/set) — Get the Distance between scale text and heading text
- `ScaleTextParameters` (SectionAlongPathTextParameterProperties, get/set) — Get the SectionAlongPathTextParameterProperties

## SectionAlongPathSectionViewProperties (class)

Section view properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionViewProperties/)

### Constructors
- `SectionAlongPathSectionViewProperties(bInitFromSTW=False) | SectionAlongPathSectionViewProperties(B)` — constructor

### Methods
#### `GetHorizontTolerance()`

Access tolerances

**Remarks:** Access tolerances

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionViewProperties/#NemAll_Python_BasisElements.SectionAlongPathSectionViewProperties.GetHorizontTolerance)

#### `GetOffsetTolerance()`

Access tolerances

**Remarks:** Access tolerances

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionViewProperties/#NemAll_Python_BasisElements.SectionAlongPathSectionViewProperties.GetOffsetTolerance)

### Properties
- `Horizont` (float, get/set) — Get the Horizon
- `IsElementedgesDisplayed` (bool, get/set) — Get the Is element edges displayed
- `IsElementedges_BottomLabeling` (bool, get/set) — Get the Is element edges bottom labeling
- `IsElementedges_MiddleLabeling` (bool, get/set) — Get the Is element edges middle labeling
- `IsElementedges_TopLabeling` (bool, get/set) — Get the Is element edges top labeling
- `IsHorizontalPositionPointDisplayed` (bool, get/set) — Get the Is horizontal position point displayed
- `IsHorizontalPositionPoint_BottomLabeling` (bool, get/set) — Get the Is horizontal position point bottom labeling
- `IsHorizontalPositionPoint_MiddleLabeling` (bool, get/set) — Get the Is horizontalPositionPoint_Middle labeling
- `IsHorizontalPositionPoint_TopLabeling` (bool, get/set) — Get the Is horizontal position point top labeling
- `IsStationPointDisplayed` (bool, get/set) — Get the Is station point displayed
- `IsStationPoint_BottomLabeling` (bool, get/set) — Get the Is station point bottom labeling
- `IsStationPoint_MiddleLabeling` (bool, get/set) — Get the Is station point middle labeling
- `IsStationPoint_TopLabeling` (bool, get/set) — Get the Is station point_Top labeling
- `IsZeroAxDisplayed` (bool, get/set) — Get the Is zero axis displayed
- `IsZeroAx_BottomLabeling` (bool, get/set) — Get the Is zero Axis bottom labeling
- `IsZeroAx_MiddleLabeling` (bool, get/set) — Get the Is zero axis middle labeling
- `IsZeroAx_TopLabeling` (bool, get/set) — Get the Is zero Axis top labeling
- `LabelingType` (int, get/set) — Get the LabelingType
- `Offset` (float, get/set) — Get the Offset

## SectionAlongPathTextParameterProperties (enum)

Text parameter properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathTextParameterProperties/)

### Constructors
- `SectionAlongPathTextParameterProperties() | SectionAlongPathTextParameterProperties(iTextParameterID, bInitFromSTW=False) | SectionAlongPathTextParameterProperties(bInitFromSTW) | SectionAlongPathTextParameterProperties(A)` — Initialize

### Methods
#### `DetectBackgroundColorType(bHasBgColor, bgColor, backgroundIsTransparent)`

Args: bHasBgColor: bgColor: backgroundIsTransparent:

**Remarks:** Args: bHasBgColor: bgColor: backgroundIsTransparent:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathTextParameterProperties/#NemAll_Python_BasisElements.SectionAlongPathTextParameterProperties.DetectBackgroundColorType)

#### `FillPropertiesFromAllplanTextParameters(richTextFlags, layerId, byLayerFlags)`

Args: richTextFlags: layerId: byLayerFlags:

**Remarks:** Args: richTextFlags: layerId: byLayerFlags:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathTextParameterProperties/#NemAll_Python_BasisElements.SectionAlongPathTextParameterProperties.FillPropertiesFromAllplanTextParameters)

### Properties
- `BackgroundColorType` (int, get/set) — Get the when =0 : no background color (transparency), when = 1 color is according to Allplan's view, when = 2, custom defined in CustomBackgroundColor
- `BorderColor` (int, get/set) — Get the rotation of the text in radians, never serialized, not in standard values, it is a working property only
- `BorderLineType` (int, get/set) — Get the rotation of the text in radians, never serialized, not in standard values, it is a working property only
- `BorderOffset` (int, get/set) — Get the BorderOffset 0 it represents value -1; 4 repres. -0.5; 2 repres. 0.0; 1 repres.1.0; 5 repres. 2.0
- `BorderThickness` (int, get/set) — Get the rotation of the text in radians, never serialized, not in standard values, it is a working property only
- `ColumnAngle` (float, get/set) — Get the ColumnAngle
- `CustomBackgroundColor` (int, get/set) — Get the when =0 : no background color (transparency), when = 1 color is according to Allplan's view, when = 2, custom defined in CustomBackgroundColor
- `FontAngle` (float, get/set) — Get the FontAngle //ONLY for italic
- `FontColor` (int, get/set) — Get the ColumnAngle
- `FontEmphasis` (int, get/set) — Get the Font emphasis
- `FontHeight` (float, get/set) — Get the FontHeight
- `FontID` (int, get/set) — Get the FontType
- `FontLayer` (int, get/set) — Get the ColumnAngle
- `FontThickness` (int, get/set) — Get the pen of the text, never serialized, not in standard values, it is a working property only
- `FontWidth` (float, get/set) — Get the FontWidth
- `IsFontColorFromLayer` (bool, get/set) — Get the When False: it means scale dependent
- `IsFontLineTypeFromLayer` (bool, get/set) — Get the When False: it means scale dependent
- `IsFontPenFromLayer` (bool, get/set) — Get the When False: it means scale dependent
- `RowDistance` (float, get/set) — Get the RowDistance //Line spacing
- `TextAngle` (float, get/set) — Get the rotation of the text in radians, never serialized, not in standard values, it is a working property only
- `TextParameterID` (TextPropertiesSource_Enum, get/set) — Initialize the class with Usable Values
- `TextParameterId` (int, get/set) — Get the Text parameter ID
- `TextPlacementPointType` (int, get/set) — Get the text placement point, see in NA_Data_TextDefines.h
- `UseBorderAroundTheText` (bool, get/set) — Get the rotation of the text in radians, never serialized, not in standard values, it is a working property only
- `UseConstantSizeInLayout` (bool, get/set) — Get the When False: it means scale dependent

## SectionDefinitionData (class)

Section definition data

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDefinitionData/)

### Constructors
- `SectionDefinitionData()` — Initialize

### Properties
- `ClippingPath` (None, get) — Set/get the clipping path :type: None
- `DefinitionProperties` (None, get) — Set/Get the definition properties :type: None
- `DirectionVector` (None, get) — Set/get the direction vector :type: None
- `HeightDirection` (None, get) — Set/get the height direction :type: None
- `SectionBody` (None, get) — Set/get the section body :type: None

## SectionDefinitionProperties (enum)

Section definition properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDefinitionProperties/)

### Constructors
- `SectionDefinitionProperties()` — Initialize

### Properties
- `ClippingPathProperties` (None, get) — Set/Get the clipping path properties :type: None
- `IsAdvancedOn` (None, get) — Property for type of section architecture= OFF /Engineering = ON :type: None
- `IsSectionBodyOn` (None, get) — Property for show the section body :type: None
- `RefMode` (None, get) — Property for the reference mode :type: None
- `ViewInputType` (None, get) — Property for the view input type :type: None

## SectionDrawingFilesProperties (class)

Drawing files properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDrawingFilesProperties/)

### Constructors
- `SectionDrawingFilesProperties() | SectionDrawingFilesProperties(A)` — Initialize

### Methods
#### `__eq__(A)`

operator ==

**Remarks:** operator ==

**Parameters:**
- `A` (SectionDrawingFilesProperties)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDrawingFilesProperties/#NemAll_Python_BasisElements.SectionDrawingFilesProperties.__eq__)

### Properties
- `DrawingNumbers` ([list[int] | VecIntList], get/set) — Get the drawing file numbers

## SectionFilterProperties (class)

Section filter properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionFilterProperties/)

### Constructors
- `SectionFilterProperties()` — Initialize

### Properties
- `DrawingFilesProperties` (None, get) — Property for the drawing files properties :type: None
- `IsAssociativityOn` (None, get) — Property for the associativity :type: None
- `LayerProperties` (None, get) — Property for the drawing files properties :type: None

## SectionFormatProperties (class)

Section format properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionFormatProperties/)

### Constructors
- `SectionFormatProperties() | SectionFormatProperties(bInitFromSTW)` — Initialize

### Properties
- `EliminationAngle` (None, get) — Property for the adjacent edge elimination angle :type: None
- `IsEliminationOn` (None, get) — Property for the adjacent edge elimination :type: None

## SectionGeneralProperties (enum)

General section properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionGeneralProperties/)

### Constructors
- `SectionGeneralProperties() | SectionGeneralProperties(bInitFromSTW)` — Initialize

### Properties
- `FilterProperties` (None, get) — Property for the filter properties :type: None
- `FormatProperties` (None, get) — Property for the format properties :type: None
- `HiddenSectionLinesProperties` (None, get) — Property for the hidden section line properties :type: None
- `LabelingProperties` (None, get) — Property for the labeling properties :type: None
- `PlacementAngle` (None, get) — Property for the placement angle :type: None
- `PlacementPoint` (None, get) — Property for the placement point :type: None
- `PlacementPointType` (None, get) — Property for the placement point :type: None
- `ShowSectionBody` (None, get) — Property for the showing of the section body :type: None
- `Status` (None, get) — Property for the drawing state :type: None
- `VisibleHiddenEdgesProperties` (None, get) — Property for the visible and hidden edge properties :type: None

## SectionLayerProperties (enum)

Section layer properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionLayerProperties/)

### Constructors
- `SectionLayerProperties()` — Initialize

### Methods
#### `SetLayerProperties(iVisibilityFilterMode, bAreInvisibleLyersStored, layerIdVector)`

Set the layer properties

**Remarks:** Set the layer properties

**Parameters:**
- `iVisibilityFilterMode` (int) — Type of selected filter mode
- `bAreInvisibleLyersStored` (bool) — true -> Elements in excludedLayerList are filtered out
- `false` (object) — -> Elements in excludedLayerList are taken into consideration
- `layerIdVector` (VecUShortList) — Elements belonging to these Layers should be filtered (excluded) from source up to flag bAreInvisibleLyersStored

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionLayerProperties/#NemAll_Python_BasisElements.SectionLayerProperties.SetLayerProperties)

## ShadingType (enum)

Shading types of the filling property

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ShadingType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `ShadingType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ShadingType/#NemAll_Python_BasisElements.ShadingType.__getitem__)

### Values
- `eFromCenter` = `2`
- `eFromCorner` = `1`
- `eLinear` = `0`
- `eRound` = `3`

## SubType (enum)

Sub types of the element group property

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SubType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `SubType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SubType/#NemAll_Python_BasisElements.SubType.__getitem__)

### Values
- `eMultiLine3D` = `1`
- `eMultiLine3D_Group` = `2`
- `eUseNoSpecialSubType` = `0`

## SurfaceDefinition (enum)

Surface resource definition

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/)

### Methods
#### `CompareData(ldef)`

compare the surface properties

**Remarks:** compare the surface properties

**Parameters:**
- `ldef` (SurfaceDefinition)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.CompareData)

#### `CopyData(pSrcSurface, bCopyName=True)`

copy the data from an other surface

**Remarks:** copy the data from an other surface

**Parameters:**
- `pSrcSurface` (SurfaceDefinition)
- `bCopyName` (bool)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.CopyData)

#### `Create()`

Create a surface definition

**Remarks:** Create a surface definition

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.Create)

#### `Dump(deep)`

Dumps content of the resource

**Remarks:** Dumps content of the resource

**Parameters:**
- `deep` (bool)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.Dump)

#### `GetDiffuseFactor()`

get the diffuse factor calculated from diffuse color intensity, reflection intensity and the transparency intensity

**Remarks:** get the diffuse factor calculated from diffuse color intensity, reflection intensity and the transparency intensity

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetDiffuseFactor)

#### `GetHash()`

get the hash of the surface definition data

**Remarks:** get the hash of the surface definition data

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetHash)

#### `GetReflectionFactor()`

calculate the reflection factor from transparency and reflection intensity

**Remarks:** calculate the reflection factor from transparency and reflection intensity

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetReflectionFactor)

#### `GetScaleU()`

get the texture u scale factor

**Remarks:** get the texture u scale factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetScaleU)

#### `GetScaleV()`

get the texture v scale factor

**Remarks:** get the texture v scale factor

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetScaleV)

#### `GetSeasonStatus()`

get the surface season flag , true - not all color texture are the same

**Remarks:** get the surface season flag , true - not all color texture are the same

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetSeasonStatus)

#### `GetSurfaceName()`

get the surface name

**Remarks:** get the surface name

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetSurfaceName)

#### `GetTextureID(tex)`

get the bitmap id of the texture

**Remarks:** get the bitmap id of the texture

**Parameters:**
- `tex` (eSurfaceTextureID)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetTextureID)

#### `GetTranslateU()`

get the texture u offset

**Remarks:** get the texture u offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetTranslateU)

#### `GetTranslateV()`

get the texture v offset

**Remarks:** get the texture v offset

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetTranslateV)

#### `IsDefault()`

check whether the surface properties has default values

**Remarks:** check whether the surface properties has default values

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.IsDefault)

#### `LoadSurfaceData(surfacePath)`

load surface properties only

**Remarks:** load surface properties only

**Parameters:**
- `surfacePath` (str)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.LoadSurfaceData)

#### `PreviewCalculationSkipped()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.PreviewCalculationSkipped)

#### `Save(surfacePath, bitmapDict)`

Create a surface definition

**Remarks:** Create a surface definition

**Parameters:**
- `surfacePath` (str) — Path of the bitmap
- `bitmapDefinitions` (object) — Dict with the bitmap definitions

**Returns:** `bool` — Creation successful state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.Save)

#### `SetScale(u, v)`

set the texture scaling factors -10000 to 10000

**Remarks:** set the texture scaling factors -10000 to 10000

**Parameters:**
- `u` (float)
- `v` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.SetScale)

#### `SetTextureID(tex, bitmapID)`

set the bitmapID for one texture

**Remarks:** set the bitmapID for one texture

**Parameters:**
- `tex` (eSurfaceTextureID)
- `bitmapID` (int)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.SetTextureID)

#### `SetTranslate(u, v)`

set the texture offset -INT_MAX to INT_MAX

**Remarks:** set the texture offset -INT_MAX to INT_MAX

**Parameters:**
- `u` (float)
- `v` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.SetTranslate)

### Properties
- `AbsolutePath` (str, get/set) — get the absolute path of the surface, outside from the design path
- `AlphaFromTextureStatus` (bool, get/set) — get the flag for using alpha channel from the color bitmap
- `BumpAmplitude` (float, get/set) — get bump mapping amplitude
- `ColorKey` (ARGB, get/set) — get the color key color
- `ColorKeyStatus` (bool, get/set) — get the flag for using color key
- `ColorKeyTolerance` (int, get/set) — get the color key tolerance 0-255
- `ColorMixingFactor` (int, get/set) — get the color mixing factor, 0 - 100
- `ColorMixingMode` (int, get/set) — get the color mixing mode.
- `DiffuseColor` (ARGB, get/set) — get the diffuse color
- `DiffuseReflectivity` (int, get/set) — get the diffuse color intensity
- `Emission` (float, get/set) — get the color emission intensity
- `MetricStatus` (bool, get/set) — get the flag for texture scale dependency from the object size. true - texture is scaled independently from the object size
- `MultiToneFactor` (float, get/set) — get the coating reflection intensity
- `NormalMapStatus` (bool, get/set) — get the flag for using normal map texture in bump map, otherwise it will be height map
- `NotFound` (bool, get/set) — get the flag for not found surface file
- `ParallaxOffset` (float, get/set) — get the parallax offset
- `ParallaxSamples` (int, get/set) — get parallax samples
- `Reflection` (int, get/set) — get the reflection amplitude
- `Refraction` (float, get/set) — get the IOR
- `RelativeName` (str, get/set) — get the relative path and name of the surface, under the design folder
- `RepeatStatus` (bool, get/set) — get the flag for texture repeating
- `Rotation` (float, get/set) — get the texture rotation angle in degrees
- `Roughness` (float, get/set) — get the roughness amplitude
- `Transparency` (int, get/set) — get the transparency intensity

## Symbol2DElement (class)

Symbol2DElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/)

### Constructors
- `Symbol2DElement() | Symbol2DElement(commonProp, Symbol2DProp, geometryObject)` — Initialize

### Methods
#### `GetSymbol2DProperties()`

Get the Symbol2D properties

**Remarks:** Get the Symbol2D properties

**Returns:** `Symbol2DProperties` — Symbol2D properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/#NemAll_Python_BasisElements.Symbol2DElement.GetSymbol2DProperties)

#### `SetSymbol2DProperties(symbol2DProp)`

Set the Symbol2D properties

**Remarks:** Set the Symbol2D properties

**Parameters:**
- `symbol2DProp` (Symbol2DProperties) — Symbol2D properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/#NemAll_Python_BasisElements.Symbol2DElement.SetSymbol2DProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/#NemAll_Python_BasisElements.Symbol2DElement.__repr__)

## Symbol2DProperties (class)

Symbol2DProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DProperties/)

### Constructors
- `Symbol2DProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (Symbol2DProperties) — Symbol2DProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DProperties/#NemAll_Python_BasisElements.Symbol2DProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DProperties/#NemAll_Python_BasisElements.Symbol2DProperties.__repr__)

### Properties
- `Height` (None, get) — Property for height :type: None
- `IsScaleDependent` (None, get) — Property for scale dependency :type: None
- `PrimaryPointNumber` (None, get) — Property for primary point number :type: None
- `RotationAngle` (None, get) — Property for rotation angle :type: None
- `SecondaryPointNumber` (None, get) — Property for secondary point number :type: None
- `SymbolID` (None, get) — Property for symbol ID :type: None
- `Width` (None, get) — Property for width :type: None

## Symbol3DElement (class)

Symbol3DElement class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/)

### Constructors
- `Symbol3DElement() | Symbol3DElement(commonProp, Symbol3DProp, geometryObject)` — Initialize

### Methods
#### `GetSymbol3DProperties()`

Get the Symbol3D properties

**Remarks:** Get the Symbol3D properties

**Returns:** `Symbol3DProperties` — Symbol3D properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/#NemAll_Python_BasisElements.Symbol3DElement.GetSymbol3DProperties)

#### `SetSymbol3DProperties(symbol3DProp)`

Set the Symbol3D properties

**Remarks:** Set the Symbol3D properties

**Parameters:**
- `symbol3DProp` (Symbol3DProperties) — Symbol3D properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/#NemAll_Python_BasisElements.Symbol3DElement.SetSymbol3DProperties)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/#NemAll_Python_BasisElements.Symbol3DElement.__repr__)

## Symbol3DProperties (class)

Symbol3DProperties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DProperties/)

### Constructors
- `Symbol3DProperties()` — Initialize

### Methods
#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (Symbol3DProperties) — Symbol3DProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DProperties/#NemAll_Python_BasisElements.Symbol3DProperties.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DProperties/#NemAll_Python_BasisElements.Symbol3DProperties.__repr__)

### Properties
- `ControlPointOffset` (None, get) — Property for control point offset :type: None
- `DescriptionText` (None, get) — Property for description text :type: None
- `Height` (None, get) — Property for height :type: None
- `IsScaleDependent` (None, get) — Property for scale dependency :type: None
- `IsStation` (None, get) — Property for station :type: None
- `PrimaryPointNumber` (None, get) — Property for primary point number :type: None
- `RotationAngle` (None, get) — Property for rotation angle :type: None
- `SecondaryPointNumber` (None, get) — Property for secondary point number :type: None
- `StationCode` (None, get) — Property for station code :type: None
- `SymbolID` (None, get) — Property for symbol ID :type: None
- `Width` (None, get) — Property for width :type: None

## TextAlignment (enum)

Text alignment types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextAlignment/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `TextAlignment` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextAlignment/#NemAll_Python_BasisElements.TextAlignment.__getitem__)

### Values
- `eLeftBottom` = `1`
- `eLeftMiddle` = `9`
- `eLeftTop` = `4`
- `eMiddleBottom` = `6`
- `eMiddleMiddle` = `5`
- `eMiddleTop` = `8`
- `eRightBottom` = `2`
- `eRightMiddle` = `7`
- `eRightTop` = `3`

## TextElement (class)

Implementation of the Text element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/)

### Constructors
- `TextElement() | TextElement(commonProp, textProp, text, textPnt) | TextElement(element)` — Initialize

### Methods
#### `GetDimensions(arg2)`

Get the text size

**Remarks:** Get the text size

**Parameters:**
- `doc` (object) — Document

**Returns:** `Vector2D` — Text size

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetDimensions)

#### `GetText()`

Get the text string

**Remarks:** Get the text string

**Returns:** `str` — Text string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetText)

#### `GetTextPoints(arg2, arg3)`

Get the text points

**Remarks:** Get the text points

**Parameters:**
- `doc` (object) — Document
- `refPnt` (object) — Reference point of the text

**Returns:** `Point2DList` — List with the text points

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetTextPoints)

#### `GetTextProperties()`

Get the Text properties

**Remarks:** Get the Text properties

**Returns:** `TextProperties` — Text properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetTextProperties)

#### `SetText(text)`

Set the text string

**Remarks:** Set the text string

**Parameters:**
- `text` (str) — Text string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.SetText)

#### `SetTextProperties(textProp)`

Set the Text properties

**Remarks:** Set the Text properties

**Parameters:**
- `textProp` (TextProperties) — Text properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.SetTextProperties)

#### `__eq__(textEle)`

Comparison operator

**Remarks:** Comparison operator

**Parameters:**
- `textEle` (TextElement) — Text element to compare

**Returns:** `bool` — Comparison state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.__repr__)

### Properties
- `Text` (str, get/set) — Get the text string
- `TextProperties` (TextProperties, get/set) — Get the Text properties

## TextElementList (class)

List for TextElement objects

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/)

### Constructors
- `TextElementList() | TextElementList(ele) | TextElementList(eleList)` — Initialize

### Methods
#### `__contains__(value)`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (object) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__contains__)

#### `__delitem__(value)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (object) — Value to delete

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__delitem__)

#### `__eq__(compare_list)`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (TextElementList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__eq__)

#### `__getitem__(index)`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `object` — Value for the index

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__getitem__)

#### `__iadd__(eleList)`

Add a list

**Remarks:** Add a list

**Parameters:**
- `eleList` (list) — TextElement list

**Returns:** `TextElementList` — Lists with the added elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__iadd__)

#### `__iter__()`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__iter__)

#### `__len__()`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__len__)

#### `__repr__()`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__repr__)

#### `__setitem__(index, value)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (object) — Value to item

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__setitem__)

#### `append(value)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (object) — Value to append

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.append)

#### `extend(iterable) | extend(eleList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (TextElementList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.extend)

## TextLocation (enum)

Location of the dimension text

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextLocation/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `TextLocation` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextLocation/#NemAll_Python_BasisElements.TextLocation.__getitem__)

### Values
- `eBASIS_DIM_BOTTOM_CENTER` = `-2`
- `eBASIS_DIM_BOTTOM_LEFT` = `-1`
- `eBASIS_DIM_BOTTOM_RIGHT` = `-3`
- `eBASIS_DIM_CENTER` = `0`
- `eBASIS_DIM_CENTER_LEFT` = `-4`
- `eBASIS_DIM_CENTER_RIGHT` = `-5`
- `eBASIS_DIM_NONE` = `10`
- `eBASIS_DIM_TOP_CENTER` = `2`
- `eBASIS_DIM_TOP_LEFT` = `1`
- `eBASIS_DIM_TOP_RIGHT` = `3`

## TextProperties (class)

Implementation of the text properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/)

### Constructors
- `TextProperties() | TextProperties(element)` — Initialize

### Methods
#### `GetRatio()`

Get ration height/width

**Remarks:** Get ration height/width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.GetRatio)

#### `Init()`

Init section properties with default values

**Remarks:** Init section properties with default values

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.Init)

#### `IsDraftText()`

check eDraftText property flag return true if eDraftText is set otherwise return false

**Remarks:** check eDraftText property flag return true if eDraftText is set otherwise return false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.IsDraftText)

#### `SetHasBackgroundColorAndTransparentBackgroundColor(value)`

Args: value:

**Remarks:** Args: value:

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.SetHasBackgroundColorAndTransparentBackgroundColor)

#### `__eq__(prop)`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (TextProperties) — TextProperties to compare

**Returns:** `object` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.__repr__)

### Properties
- `Alignment` (eTextAlignment, get/set) — Get the Alignment of text
- `BackgroundColor` (int, get/set) — Get the Background color of text
- `ColumnSlopeAngle` (NemAll_Python_Geometry.Angle, get/set) — Get the Angle of columns in [rad]
- `Expansion` (NEM_UBYTE_t, get/set) — Get the Background color of text
- `Font` (int, get/set) — Get the font ID
- `FontAngle` (NemAll_Python_Geometry.Angle, get/set) — Get the Angle of characters (italic) in [rad]
- `HasBackgroundColor` (bool, get/set) — check eHasTextBgColor property flag return true if eHasTextBgColor is set otherwise return false
- `HasReferencePoint` (bool, get/set) — check eHasReferencePoint property flag return true if eHasReferencePoint is set otherwise return false
- `HasTextFrame` (bool, get/set) — check text frame active return true if text frame is active otherwise return false
- `HasTransparentBackground` (bool, get/set) — check eHasTransparentBackground property flag return true if eHasTransparentBackground is set otherwise return false
- `Height` (float, get/set) — Get the Height of text in [mm]
- `IsScaleDependent` (bool, get/set) — check eIsScaleDependent property flag return true if eIsScaleDependent is set otherwise return false
- `IsUserModifiable` (bool, get/set) — Check if user can modify text return true if user can modify text
- `LineFeed` (float, get/set) — Get the Line feed of text in [mm]
- `TextAngle` (NemAll_Python_Geometry.Angle, get/set) — Get the Angle of hole text in [rad]
- `TextFrame` (TextFrameStruct, get/set) — Get the Text frame pen, stroke color settings
- `TextFrameColor` (int, get/set) — Get text frame color value
- `TextFramePen` (int, get/set) — Get text frame pen value
- `TextFrameStroke` (int, get/set) — Get text frame stroke value
- `Type` (eTextType, get/set) — Get the Text type (0 - normal text, >1 variables textbild (itv))
- `Width` (float, get/set) — Get the Width of text in [mm]
- `WrappedText` (bool, get/set) — Property for text wrapping returns true if the text is wrapped

## TextType (enum)

Text types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `TextType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextType/#NemAll_Python_BasisElements.TextType.__getitem__)

### Values
- `eFormularText` = `1`
- `eNormalText` = `0`
- `eVariableText` = `2`

## TextureDefinition (class)

TextureDefinition class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureDefinition/)

### Constructors
- `TextureDefinition() | TextureDefinition(surfacePath)` — Initialize

### Methods
#### `__eq__(props)`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (TextureDefinition) — Properties to compare

**Returns:** `bool` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureDefinition/#NemAll_Python_BasisElements.TextureDefinition.__eq__)

#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureDefinition/#NemAll_Python_BasisElements.TextureDefinition.__repr__)

### Properties
- `SurfacePath` (None, get) — Property for surface path :type: None

## TextureMapping (class)

Texture mapping property class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/)

### Constructors
- `TextureMapping() | TextureMapping(mappingType, mappingAngle, scaleX, scaleY, offsetX, offsetY, phongAngle, refFace, refEdge) | TextureMapping(mappingType, mappingAngle, scaleX, scaleY, offsetX, offsetY, phongAngle, refFace, refEdge, uvCoords) | TextureMapping(uvCoords) | TextureMapping(element)` — Initialize

### Methods
#### `FromSurfaceMapping(mapping)`

Get the properties from the surface mapping object

**Remarks:** Get the properties from the surface mapping object

**Parameters:**
- `mapping` (object) — Surface mapping

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/#NemAll_Python_BasisElements.TextureMapping.FromSurfaceMapping)

#### `__eq__(props)`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (TextureMapping) — Properties to compare

**Returns:** `bool` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/#NemAll_Python_BasisElements.TextureMapping.__eq__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/#NemAll_Python_BasisElements.TextureMapping.__repr__)

### Properties
- `MappingAngle` (float, get/set) — Get mapping angle
- `MappingType` (eMappingType, get/set) — Get mapping type
- `PhongAngle` (float, get/set) — Get angle for Phong light model
- `ReferenceEdge` (int, get/set) — Get reference edge
- `ReferenceFace` (int, get/set) — Get reference face
- `UVCoordinates` (list[float], get/set) — Get UV texture mapping coordinates
- `UValueForIsoLines` (int, get/set) — Get U count of isolines
- `VValueForIsoLines` (int, get/set) — Get V count of isolines
- `XOffset` (float, get/set) — Get X offset
- `XScale` (float, get/set) — Get mapping X scale
- `YOffset` (float, get/set) — Get Y offset
- `YScale` (float, get/set) — Get mapping Y scale

## TextureMappingType (enum)

eCube - from each side eWall - mainly from the front view eRoof - mainly from the top view eGround - only from the top view eCylinder - Cylindrical mapping eSphere - Spherical mapping eUV - UV mapping

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMappingType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `TextureMappingType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMappingType/#NemAll_Python_BasisElements.TextureMappingType.__getitem__)

### Values
- `eCube` = `0`
- `eCylinder` = `4`
- `eGround` = `3`
- `eRoof` = `2`
- `eSphere` = `5`
- `eUV` = `6`
- `eWall` = `1`

## TransitionType (enum)

Transition types of the filling property

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TransitionType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `TransitionType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TransitionType/#NemAll_Python_BasisElements.TransitionType.__getitem__)

### Values
- `eNoTransition` = `0`
- `eOneColorTransition` = `1`
- `eTwoColorTransition` = `2`

## VariantType (enum)

Variant types of the filling property

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/VariantType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `VariantType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/VariantType/#NemAll_Python_BasisElements.VariantType.__getitem__)

### Values
- `eVariant1` = `0`
- `eVariant2` = `1`
- `eVariant3` = `2`
- `eVariant4` = `3`

## ViewSectionElement (class)

Implementation of the view section element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/)

### Constructors
- `ViewSectionElement() | ViewSectionElement(param)` — Initialize

### Methods
#### `CreateSectionBody(doc, insertionMat, undoRedoService=None)`

Create the section body

**Remarks:** Create the section body

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Placement matrix
- `undoRedoService` (Optional[object])

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/#NemAll_Python_BasisElements.ViewSectionElement.CreateSectionBody)

#### `DrawElement(modelElements, insertionMat, docID, asStaticPreview)`

Draw eLement preview in Allplan

**Remarks:** Draw eLement preview in Allplan

**Parameters:**
- `modelElements` (object) — Python object which should be drawn
- `insertionMat` (Matrix3D) — insertion matrix
- `docID` (int) — Document ID
- `asStaticPreview` (bool)

**Returns:** `MinMax3D` — Min/max box of the preview

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/#NemAll_Python_BasisElements.ViewSectionElement.DrawElement)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/#NemAll_Python_BasisElements.ViewSectionElement.__repr__)

### Properties
- `DimensionElements` (list, get/set) — 
- `GeneralSectionProperties` (SectionGeneralProperties, get/set) — 
- `ReinforcementLabels` (NemAll_Python_Reinforcement.ReinforcementLabel, get/set) — 
- `SectionDefinitionData` (SectionDefinitionData, get/set) — 
- `TextElements` (list, get/set) — 
- `ViewMatrix` (NemAll_Python_Geometry.Matrix3D, get/set) — 

## VisibleHiddenEdgesProperties (class)

Visible hidden edges properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_BasisElements/VisibleHiddenEdgesProperties/)

### Constructors
- `VisibleHiddenEdgesProperties()` — Initialize

### Properties
- `HiddenEdgesColor` (None, get) — Property for color for hidden edges :type: None
- `HiddenEdgesLayer` (None, get) — Property for layer for hidden edges :type: None
- `HiddenEdgesLineType` (None, get) — Property for line type for hidden edges :type: None
- `HiddenEdgesPen` (None, get) — Property for pen for hidden edges :type: None
- `IsHiddenEdgesColorFromLayer` (None, get) — Property for hidden edges color from layer :type: None
- `IsHiddenEdgesColorUsed` (None, get) — Property for custom color used for hidden edges :type: None
- `IsHiddenEdgesLayerUsed` (None, get) — Property for custom layer used for hidden edges :type: None
- `IsHiddenEdgesLineTypeFromLayer` (None, get) — Property for hidden edges line type from layer :type: None
- `IsHiddenEdgesLineTypeUsed` (None, get) — Property for custom line type used for hidden edges :type: None
- `IsHiddenEdgesOn` (None, get) — Property for the drawing of the hidden edges :type: None
- `IsHiddenEdgesPenFromLayer` (None, get) — Property for hidden edges pen from layer :type: None
- `IsHiddenEdgesPenUsed` (None, get) — Property for custom pen used for hidden edges :type: None
- `IsVisibleEdgesColorFromLayer` (None, get) — Property for visible edges color from layer :type: None
- `IsVisibleEdgesColorUsed` (None, get) — Property for custom color used for visible edges :type: None
- `IsVisibleEdgesLayerUsed` (None, get) — Property for custom layer used for visible edges :type: None
- `IsVisibleEdgesLineTypeFromLayer` (None, get) — Property for visible edges line type from layer :type: None
- `IsVisibleEdgesLineTypeUsed` (None, get) — Property for custom line type used for visible edges :type: None
- `IsVisibleEdgesOn` (None, get) — Property for the drawing of the visible edges :type: None
- `IsVisibleEdgesPenFromLayer` (None, get) — Property for visible edges pen from layer :type: None
- `IsVisibleEdgesPenUsed` (None, get) — Property for custom pen used for visible edges :type: None
- `VisibleEdgesColor` (None, get) — Property for color for visible edges :type: None
- `VisibleEdgesLayer` (None, get) — Property for layer for visible edges :type: None
- `VisibleEdgesLineType` (None, get) — Property for line type for visible edges :type: None
- `VisibleEdgesPen` (None, get) — Property for pen for visible edges :type: None

