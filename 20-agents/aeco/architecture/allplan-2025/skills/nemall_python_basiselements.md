---
name: allplan-nemall_python_basiselements
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_BasisElements namespace — 95 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, ARGB, AllplanElement, AssociativeViewElementRepresentation, AssociativeViewElement, AssociativeViewProperties, AttributeContainer, BasisElement, and 87 more types.
---

# NemAll_Python_BasisElements

Auto-generated from vendor docs for allplan 2025.0. 95 types in this namespace.

## ARGB (class)

Represents true color with transparency

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/)

### Constructors
- `ARGB() | ARGB(argb: int) | ARGB(red: int, green: int, blue: int, alpha: int) | ARGB(argb: ARGB)` — Initialize

### Methods
#### `GetARGB() -> int`

returns color as unsigned long

**Remarks:** returns color as unsigned long

**Returns:** `int` — color as long

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/#NemAll_Python_BasisElements.ARGB.GetARGB)

#### `__eq__(argb: ARGB) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `argb` (ARGB) — color to compare

**Returns:** `bool` — true if they are equal

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/#NemAll_Python_BasisElements.ARGB.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ARGB/#NemAll_Python_BasisElements.ARGB.__repr__)

### Properties
- `ABGR` (int, get/set) — returns color as ABGR sets color from unsigned long ABGR value
- `ARGB` (int, get/set) — returns color as unsigned long sets color from ARGB
- `Alpha` (int, get/set) — returns alpha component
- `BGR` (int, get/set) — returns color as BGR sets only RGB values from unsigned long BGR format
- `Blue` (int, get/set) — returns blue component
- `Green` (int, get/set) — returns green component
- `IRGB` (int, get/set) — Get the indexed rgb color representation of the ARGB color If color not found in the index table, the rgb color converted to integer value is returned
- `Red` (int, get/set) — returns red component

## AllplanElement (class)

Implementation of the Allplan element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/)

### Methods
#### `GetAttributes() -> object`

Get the attributes object

**Remarks:** Get the attributes object

**Returns:** `object` — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetAttributes)

#### `GetBaseElementAdapter() -> BaseElementAdapter`

Get the model element

**Remarks:** Get the model element

**Returns:** `BaseElementAdapter` — Model element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetBaseElementAdapter)

#### `GetCommonProperties() -> CommonProperties`

Get the common properties

**Remarks:** Get the common properties

**Returns:** `CommonProperties` — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetCommonProperties)

#### `GetGeometryObject() -> object`

Get the geometry object

**Remarks:** Get the geometry object

**Returns:** `object` — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetGeometryObject)

#### `GetLabelElements() -> list`

Get the label elements

**Remarks:** Get the label elements

**Returns:** `list` — LabelElements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetLabelElements)

#### `GetSubElementID() -> type`

Get the SubElementID

**Remarks:** Get the SubElementID

**Returns:** `type` — SubElementID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.GetSubElementID)

#### `SetAttributes(attributeContainer: object)`

Set the attributes object

**Remarks:** Set the attributes object

**Parameters:**
- `attributeContainer` (object) — Attributes object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetAttributes)

#### `SetCommonProperties(commonProp: CommonProperties)`

Set the common properties

**Remarks:** Set the common properties

**Parameters:**
- `commonProp` (CommonProperties) — Common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetCommonProperties)

#### `SetDockingPointsKey(dockingPointsKey: str)`

Set the docking points key

**Remarks:** Set the docking points key

**Parameters:**
- `dockingPointsKey` (str) — Docking points key

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetDockingPointsKey)

#### `SetGeometryObject(geoObject: object)`

Set the geometry object

**Remarks:** Set the geometry object

**Parameters:**
- `geoObject` (object) — Geometry object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetGeometryObject)

#### `SetLabelElements(labelElements: list)`

Set the label elements

**Remarks:** Set the label elements

**Parameters:**
- `labelElements` (list) — Label elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AllplanElement/#NemAll_Python_BasisElements.AllplanElement.SetLabelElements)

### Properties
- `Attributes` (object, get/set) — Get the attributes object
- `CommonProperties` (CommonProperties, get/set) — Get the common properties
- `GeometryObject` (object, get/set) — Get the geometry object
- `LabelElements` (list, get/set) — Get the label elements

## AssociativeViewElement (class)

Representation of the associative views and sections

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElement/)

### Constructors
- `AssociativeViewElement()` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElement/#NemAll_Python_BasisElements.AssociativeViewElement.__repr__)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewElementRepresentation/)

### Values
- `AssociativeViewAllElements` = `0`
- `AssociativeViewConcreteShape` = `3`

## AssociativeViewProperties (class)

Properties of the associative views and sections

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AssociativeViewProperties/)

### Constructors
- `AssociativeViewProperties() | AssociativeViewProperties(arg2: DocumentAdapter) | AssociativeViewProperties(arg2: AssociativeViewProperties)` — Initialize

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AttributeContainer/)

### Constructors
- `AttributeContainer() | AttributeContainer(attributesObject: object)` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/AttributeContainer/#NemAll_Python_BasisElements.AttributeContainer.__repr__)

## BasisElement (class)

Abstract base class representing general elements in Allplan.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisElement/)

## BasisPropertyDialogs (class)

(No description provided in vendor docs for NemAll_Python_BasisElements.BasisPropertyDialogs.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisPropertyDialogs/)

### Methods
#### `OpenBitmapResourceDialog(doc: DocumentAdapter, bitmapPath: str) -> str`

Open the bitmap resource dialog

**Remarks:** Open the bitmap resource dialog

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `bitmapPath` (str) — Initial path

**Returns:** `str` — Selected bitmap path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisPropertyDialogs/#NemAll_Python_BasisElements.BasisPropertyDialogs.OpenBitmapResourceDialog)

#### `OpenRGBColorDialog(doc, color: int) -> int`

Open the RGB color dialog

**Remarks:** Open the RGB color dialog

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BasisPropertyDialogs/#NemAll_Python_BasisElements.BasisPropertyDialogs.OpenRGBColorDialog)

## BitmapAreaElement (class)

Representation of the bitmap area in Allplan.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/)

### Constructors
- `BitmapAreaElement() | BitmapAreaElement( commonProp: CommonProperties, bitmapAreaProp: BitmapAreaProperties, geometryObject: object, ) | BitmapAreaElement(BitmapAreaElement: BitmapAreaElement)` — Initialize

### Methods
#### `GetBitmapAreaProperties() -> BitmapAreaProperties`

Get the BitmapArea properties

**Remarks:** Get the BitmapArea properties

**Returns:** `BitmapAreaProperties` — BitmapArea properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/#NemAll_Python_BasisElements.BitmapAreaElement.GetBitmapAreaProperties)

#### `SetBitmapAreaProperties(bitmapAreaProp: BitmapAreaProperties)`

Set the bitmapArea properties

**Remarks:** Set the bitmapArea properties

**Parameters:**
- `bitmapAreaProp` (BitmapAreaProperties) — BitmapArea properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/#NemAll_Python_BasisElements.BitmapAreaElement.SetBitmapAreaProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaElement/#NemAll_Python_BasisElements.BitmapAreaElement.__repr__)

## BitmapAreaProperties (class)

Properties of the bitmap area.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaProperties/)

### Constructors
- `BitmapAreaProperties()` — Initialize

### Methods
#### `__eq__(prop: BitmapAreaProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (BitmapAreaProperties) — BitmapProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaProperties/#NemAll_Python_BasisElements.BitmapAreaProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapAreaProperties/#NemAll_Python_BasisElements.BitmapAreaProperties.__repr__)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/)

### Methods
#### `Create(bitmapName: str) -> BitmapDefinition`

Create a bitmap definition

**Remarks:** Create a bitmap definition

**Parameters:**
- `bitmapName` (str) — Path and name of the bitmap

**Returns:** `BitmapDefinition` — Bitmap definition

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.Create)

#### `Dump(deep: bool)`

Dump the bitmap definition

**Remarks:** Dump the bitmap definition

**Parameters:**
- `deep` (bool) — True for a deep dump

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.Dump)

#### `GetBitmapName() -> str`

Get the name

**Remarks:** Get the name

**Returns:** `str` — Name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetBitmapName)

#### `GetHeight() -> int`

Get bitmap height

**Remarks:** Get bitmap height

**Returns:** `int` — height in pixels

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetHeight)

#### `GetPixel(x: int, y: int) -> int`

Get the color of a specific pixel

**Remarks:** Get the color of a specific pixel

**Parameters:**
- `x` (int) — horizontal position
- `y` (int) — vertical position

**Returns:** `int` — Color as ARGB

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetPixel)

#### `GetWidth() -> int`

Get bitmap width

**Remarks:** Get bitmap width

**Returns:** `int` — Width in pixels

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.GetWidth)

#### `IsHDRI() -> bool`

Is High Dynamic Range Image

**Remarks:** Is High Dynamic Range Image

**Returns:** `bool` — Bitmap is a HDR image

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.IsHDRI)

#### `LoadBitmap() -> bool`

Load bitmap

**Remarks:** Load bitmap

**Returns:** `bool` — True, when loading was successful

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.LoadBitmap)

#### `setName(resourceName: str)`

Set the name of the bitmap

**Remarks:** Set the name of the bitmap

**Parameters:**
- `resourceName` (str) — name to assign

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/BitmapDefinition/#NemAll_Python_BasisElements.BitmapDefinition.setName)

### Properties
- `AbsolutePath` (str, get/set) — Absolute path of the bitmap
- `RelativeName` (str, get/set) — Relative name of the bitmap

## ClippingPathProperties (class)

Representation if the properties of a clipping path of a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ClippingPathProperties/)

### Constructors
- `ClippingPathProperties()` — Initialize

### Properties
- `BottomLevel` (None, get) — The bottom elevation of the section
- `IsClippingLineOn` (None, get) — Whether to draw the clipping line
- `IsHeightFromElementOn` (None, get) — Whether to get the section height from elements
- `SectionIdentifier` (None, get) — the section identifier
- `SectionType` (None, get) — Property for type of section :type: None
- `TopLevel` (None, get) — The top elevation of the section

## CombinationType (enum)

Definition of the combination type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/CombinationType/)

### Values
- `eVx` = `1`
- `eVy` = `2`
- `eVz` = `3`

## ConsiderType (enum)

Possible options for how a smart symbol (macro) placed in a room with finishing surfaces is displayed in animation and in sections

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ConsiderType/)

### Values
- `eConsiderAutomatic` = `3` — Automatic to nearest (floor or ceiling) surface - !!! don't use this value at the moment, it's only for internal usage
- `eConsiderCeilingOpening` = `8` — Adapts to ceiling opening
- `eConsiderCeilingRecess` = `9` — Adapts to ceiling recess
- `eConsiderCeilingSurface` = `2` — Adapts to the height of the ceiling in animation and sections.
- `eConsiderDoorOpening` = `5` — Adapts to door opening
- `eConsiderFloorSurface` = `1` — Adapts to the height of the floor
- `eConsiderNiche` = `6` — Adapts to niche opening
- `eConsiderNothing` = `0` — No adaptation
- `eConsiderRecess` = `7` — Adapts to recess opening
- `eConsiderWindowOpening` = `4` — Adapts to window opening

## DimensionLineElement (class)

Representation of the dimension line in Allplan.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/)

### Constructors
- `DimensionLineElement() | DimensionLineElement( arg2: Point3DList, dimensionPoints: Vector2D, placementPoint: Vector2D, directionVector: DimensionProperties, )` — Initialize

### Methods
#### `GetDimensionPoints() -> Point3DList`

Get the dimension points

**Remarks:** Get the dimension points

**Returns:** `Point3DList` — Dimension points

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.GetDimensionPoints)

#### `GetDirectionVector() -> Vector2D`

Get the direction vector

**Remarks:** Get the direction vector

**Returns:** `Vector2D` — Direction vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.GetDirectionVector)

#### `GetPlacementVector() -> Vector2D`

Get the placement vector

**Remarks:** Get the placement vector

**Returns:** `Vector2D` — Placement vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.GetPlacementVector)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionLineElement/#NemAll_Python_BasisElements.DimensionLineElement.__repr__)

## DimensionProperties (class)

Properties of the dimension line as well as the elevation points.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/DimensionProperties/)

### Constructors
- `DimensionProperties() | DimensionProperties(arg2: DocumentAdapter, arg3: Dimensioning) | DimensionProperties(arg2: DimensionProperties)` — Initialize

### Properties
- `ElevationBaseOffset` (None, get) — Property for the elevation base offset :type: None
- `FontIDDimensionNumber` (None, get) — Property for the font ID of the dimension number :type: None
- `IsAbsoluteElevation` (None, get) — Whether to use the absolute elevation values
- `LeadingCharacter` (None, get) — Property for the leading characters :type: None
- `PointSymbol` (None, get) — Property for the point symbol of the elevation :type: None
- `PointSymbolEnd` (None, get) — Property for the point symbol at the start of the dimension line :type: None
- `PointSymbolStart` (None, get) — Property for the point symbol at the start of the dimension line :type: None
- `TailingCharacters` (None, get) — Property for the tailing characters :type: None
- `TextHeightDimensionNumber` (None, get) — Property for the height of the dimension number :type: None
- `TextLocation` (None, get) — The location of the dimension number
- `TextOffset` (None, get) — Offset of text from dimension line :type: None

## Dimensioning (enum)

Type of the dimensioning

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Dimensioning/)

### Values
- `eDimensionLine` = `1` — As a dimension line
- `eElevation` = `2` — As elevation points

## ElementGroupElement (class)

Representation of the element group in Allplan.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/)

### Constructors
- `ElementGroupElement() | ElementGroupElement( commonProp: CommonProperties, elementGroupProp: ElementGroupProperties, elementGroupObjectList: list, )` — Initialize

### Methods
#### `GetElementGroupProperties() -> ElementGroupProperties`

Get the ElementGroup properties

**Remarks:** Get the ElementGroup properties

**Returns:** `ElementGroupProperties` — ElementGroup properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.GetElementGroupProperties)

#### `GetObjectList() -> list`

Get the list of element group objects

**Remarks:** Get the list of element group objects

**Returns:** `list` — Element group object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.GetObjectList)

#### `SetElementGroupProperties(ElementGroupProp: ElementGroupProperties)`

Set the ElementGroup properties

**Remarks:** Set the ElementGroup properties

**Parameters:**
- `ElementGroupProp` (ElementGroupProperties) — ElementGroup properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.SetElementGroupProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupElement/#NemAll_Python_BasisElements.ElementGroupElement.__repr__)

## ElementGroupProperties (class)

Representation of the elevation point in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupProperties/)

### Constructors
- `ElementGroupProperties()` — Initialize

### Methods
#### `__eq__(prop: ElementGroupProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (ElementGroupProperties) — ElementGroupProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupProperties/#NemAll_Python_BasisElements.ElementGroupProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementGroupProperties/#NemAll_Python_BasisElements.ElementGroupProperties.__repr__)

### Properties
- `ModifiableFlag` (None, get) — Property for modifiable flag :type: None
- `Name` (None, get) — Property for name of element group :type: None
- `SubType` (None, get) — Property for macro sub type :type: None

## ElementNodeElement (class)

ElementNodeElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/)

### Constructors
- `ElementNodeElement() | ElementNodeElement( commonProp: CommonProperties, elementNodeId: type, elementNodeObjectList: list, )` — Initialize

### Methods
#### `GetObjectList() -> list`

Get the list of element node objects

**Remarks:** Get the list of element node objects

**Returns:** `list` — Element node object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.GetObjectList)

#### `GetParentID() -> type`

Get parent ID

**Remarks:** Get parent ID

**Returns:** `type` — UUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.GetParentID)

#### `SetParentID(ParentID: type)`

Set the parent element id to this object

**Remarks:** Set the parent element id to this object

**Parameters:**
- `ParentID` (type) — (UUID) sub element id

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.SetParentID)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElementNodeElement/#NemAll_Python_BasisElements.ElementNodeElement.__repr__)

## ElevationElement (class)

ElevationElement class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElevationElement/)

### Constructors
- `ElevationElement() | ElevationElement( dimensionPoints: Point3DList, placementVector: Vector2D, directionVector: Vector2D, settings: DimensionProperties, )` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ElevationElement/#NemAll_Python_BasisElements.ElevationElement.__repr__)

## EndSymbolsProperties (class)

Properties of start an end symbols of a ModelElement2D (line, polyline, etc...)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/EndSymbolsProperties/)

### Constructors
- `EndSymbolsProperties() | EndSymbolsProperties(startID: int, startSize: float, endID: int, endSize: float)` — Initialize

### Methods
#### `__eq__(prop: EndSymbolsProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (EndSymbolsProperties) — EndSymbolsProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/EndSymbolsProperties/#NemAll_Python_BasisElements.EndSymbolsProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/EndSymbolsProperties/#NemAll_Python_BasisElements.EndSymbolsProperties.__repr__)

### Properties
- `EndID` (None, get) — Property for end symbol ID :type: None
- `EndSize` (None, get) — Property for end symbol size :type: None
- `StartID` (None, get) — Property for start symbol ID :type: None
- `StartSize` (None, get) — Property for start symbol size :type: None

## FaceStyleElement (class)

Representation of the style area in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/)

### Constructors
- `FaceStyleElement() | FaceStyleElement( commonProp: CommonProperties, faceStyleProp: FaceStyleProperties, geometryObject: object, ) | FaceStyleElement(FaceStyleElement: FaceStyleElement)` — Initialize

### Methods
#### `GetFaceStyleProperties() -> FaceStyleProperties`

Get the face style properties

**Remarks:** Get the face style properties

**Returns:** `FaceStyleProperties` — face style properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/#NemAll_Python_BasisElements.FaceStyleElement.GetFaceStyleProperties)

#### `SetFaceStyleProperties(faceStyleProp: FaceStyleProperties)`

Set the FaceStyle properties

**Remarks:** Set the FaceStyle properties

**Parameters:**
- `faceStyleProp` (FaceStyleProperties) — face style properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/#NemAll_Python_BasisElements.FaceStyleElement.SetFaceStyleProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleElement/#NemAll_Python_BasisElements.FaceStyleElement.__repr__)

## FaceStyleProperties (class)

Properties of the style area

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleProperties/)

### Constructors
- `FaceStyleProperties()` — Initialize

### Methods
#### `__eq__(prop: FaceStyleProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FaceStyleProperties) — FaceStyleProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleProperties/#NemAll_Python_BasisElements.FaceStyleProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FaceStyleProperties/#NemAll_Python_BasisElements.FaceStyleProperties.__repr__)

### Properties
- `DirectionToReferenceLine` (None, get) — Index of the edge of the outline polyline, to align the style area to, when the property UseDirectionToReferenceLine is set to True.
- `FaceStyleID` (None, get) — ID of style area
- `ReferencePoint` (None, get) — the point of origin, when property UseReferencePoint is set to True
- `RotationAngle` (None, get) — Rotation angle
- `UseDirectionToReferenceLine` (None, get) — Whether to align the direction of the style area to a certain edge. When set to true, the index of the edge is to be specified in the property DirectionToReferenceLine.
- `UseReferencePoint` (None, get) — Whether to use a specific point as the origin of the style area. When set to False, the origin is (0,0). When set to True, the point is to be specified in the property ReferencePoint.

## FillingElement (class)

Representation of the filling in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/)

### Constructors
- `FillingElement() | FillingElement( commonProp: CommonProperties, gradientFillingProp: FillingProperties, geometryObject: object, ) | FillingElement(FillingElement: FillingElement)` — Initialize

### Methods
#### `GetFillingProperties() -> FillingProperties`

Get the filling properties

**Remarks:** Get the filling properties

**Returns:** `FillingProperties` — Filling properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/#NemAll_Python_BasisElements.FillingElement.GetFillingProperties)

#### `SetFillingProperties(gradientFillingProp: FillingProperties)`

Set the filling properties

**Remarks:** Set the filling properties

**Parameters:**
- `gradientFillingProp` (FillingProperties) — Filling properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/#NemAll_Python_BasisElements.FillingElement.SetFillingProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingElement/#NemAll_Python_BasisElements.FillingElement.__repr__)

## FillingProperties (class)

Properties of the filling

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingProperties/)

### Constructors
- `FillingProperties()` — Initialize

### Methods
#### `__eq__(prop: FillingProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (FillingProperties) — GradientFillingProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingProperties/#NemAll_Python_BasisElements.FillingProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/FillingProperties/#NemAll_Python_BasisElements.FillingProperties.__repr__)

### Properties
- `DirectionToReferenceLine` (None, get) — The edge index of the outline polyline to align the filling to. Relevant, when the property UseDirectionToReferenceLine is set to true.
- `FirstColor` (None, get) — First color
- `RotationAngle` (None, get) — Rotation angle. Relevant, when UseGradientFilling is set to True.
- `SecondColor` (None, get) — Second color. Relevant, when UseGradientFilling is set to True.
- `ShadingType` (None, get) — Shading style. Relevant, when the UseGradientFilling is set to True.
- `TranslationType` (None, get) — Color gradients type. Relevant, when UseGradientFilling is set to True.
- `UseDirectionToReferenceLine` (None, get) — Whether to align the direction of the filling to a certain edge of the outline polyline. When set to True, the index of the edge is to be specified in DirectionToReferenceLine.
- `UseGradientFilling` (None, get) — Whether to use gradient filling.
- `VariantType` (None, get) — Gradient variant type. Relevant, when UseGradientFilling is set to True.

## Functions (static-class)

Module-level functions of NemAll_Python_BasisElements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/)

## HatchingElement (class)

Representation of the hatching in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/)

### Constructors
- `HatchingElement() | HatchingElement( commonProp: CommonProperties, hatchingProp: HatchingProperties, polygon: Polygon2D, ) | HatchingElement(element: HatchingElement)` — Initialize

### Methods
#### `GetHatchingProperties() -> HatchingProperties`

Get the hatching properties

**Remarks:** Get the hatching properties

**Returns:** `HatchingProperties` — Hatching properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/#NemAll_Python_BasisElements.HatchingElement.GetHatchingProperties)

#### `SetHatchingProperties(hatchingProp: HatchingProperties)`

Set the hatching properties

**Remarks:** Set the hatching properties

**Parameters:**
- `hatchingProp` (HatchingProperties) — Hatching properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/#NemAll_Python_BasisElements.HatchingElement.SetHatchingProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingElement/#NemAll_Python_BasisElements.HatchingElement.__repr__)

### Properties
- `HatchingProperties` (HatchingProperties, get/set) — Get the hatching properties

## HatchingProperties (class)

Properties of the hatching

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/)

### Constructors
- `HatchingProperties() | HatchingProperties(element: HatchingProperties)` — Initialize

### Methods
#### `SetBackgroundColorBGR(value: int)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as BGR

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.SetBackgroundColorBGR)

#### `SetBackgroundColorIRGB(value: int)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as IRGB

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.SetBackgroundColorIRGB)

#### `__eq__(prop: HatchingProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (HatchingProperties) — HatchingProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HatchingProperties/#NemAll_Python_BasisElements.HatchingProperties.__repr__)

### Properties
- `BackgroundColor` (ARGB, get/set) — Background color. Relevant, when UseBackgroundColor is set to True.
- `DirectionToReferenceLine` (int, get/set) — The edge index of the outline polyline to align the hatching to.
- `ExistAlignment` (bool, get/set) — Whether to align the hatching to a certain edge of the outline polyline. When set to True, the index of the edge is to be specified in DirectionToReferenceLine.
- `HatchID` (int, get/set) — ID of the hatch
- `IsScaleDependent` (bool, get/set) — Whether the hatch is to be scaled according to drawing file scale
- `ReferencePoint` (Point2D, get/set) — the point of origin, when property UseReferencePoint is set to True
- `RotationAngle` (Angle, get/set) — Rotation angle
- `UseBackgroundColor` (bool, get/set) — Whether to use background color
- `UseReferencePoint` (bool, get/set) — Whether to use a specific point as the origin of the hatching. When set to False, the origin is (0,0). When set to True, the point is to be specified in the property ReferencePoint.

## HeightDefinitionType (enum)

Enumeration of height definition types of a macro placement

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HeightDefinitionType/)

### Values
- `eAverage` = `3` — Average
- `eComponent` = `2` — Component
- `eMacro` = `1` — Macro
- `eNone` = `0` — Not defined

## HiddenSectionLinesProperties (class)

Properties of hidden section lines in a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/HiddenSectionLinesProperties/)

### Constructors
- `HiddenSectionLinesProperties()` — Initialize

### Properties
- `HiddenSectionLinesColor` (None, get) — Color ID of hidden section lines
- `HiddenSectionLinesLayer` (None, get) — Layer ID of hidden section lines
- `HiddenSectionLinesLineType` (None, get) — Stroke ID of hidden section lines
- `HiddenSectionLinesPen` (None, get) — Pen ID of hidden section lines
- `IsHiddenSectionLinesColorFromLayer` (None, get) — Whether to get the color ID from the layer definition
- `IsHiddenSectionLinesLineTypeFromLayer` (None, get) — Whether to get the stroke ID from the layer definition
- `IsHiddenSectionLinesOn` (None, get) — Whether to draw hidden section lines
- `IsHiddenSectionLinesPenFromLayer` (None, get) — Whether to get the pen ID from the layer definition

## LabelElement (class)

Representation of the label in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/)

### Constructors
- `LabelElement() | LabelElement(text: TextElement, labelType: LabelType) | LabelElement(textElements: TextElementList, labelType: LabelType) | LabelElement(element: LabelElement)` — Initialize

### Methods
#### `AddTextElement(text: TextElement)`

Add a text element

**Remarks:** Add a text element

**Parameters:**
- `text` (TextElement) — Text element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/#NemAll_Python_BasisElements.LabelElement.AddTextElement)

#### `SetLabeledElement(labeledElement: BaseElementAdapter)`

Set the labeled element

**Remarks:** Set the labeled element

**Parameters:**
- `labeledElement` (BaseElementAdapter) — Labeled element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/#NemAll_Python_BasisElements.LabelElement.SetLabeledElement)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelElement/#NemAll_Python_BasisElements.LabelElement.__repr__)

### Properties
- `TextElements` (TextElementList, get/set) — Get the text elements

## LabelType (enum)

Type of the label

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelType/)

### Values
- `eLabelArchDimensionLine` = `3` — Architectural dimension line
- `eLabelBftSlabElementation` = `5` — Precast slab elementation label
- `eLabelBftWallElementation` = `4` — Precast wall elementation label
- `eLabelEng3DBarReinforcement` = `2` — 3D reinforcement bar label
- `eLabelNoText` = `7` — Label with no text
- `eLabelNormalText` = `0` — Fixed text
- `eLabelVariableText` = `1` — Variable text

## LabelingProperties (class)

Representation of labeling settings of a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LabelingProperties/)

### Constructors
- `LabelingProperties()` — Initialize

### Properties
- `AddProjectionName` (None, get) — Whether to display the section identifier
- `HeadingOn` (None, get) — Whether to display the heading
- `HeadingText` (None, get) — Additional heading text, prior to the section identifier
- `IsScaleOn` (None, get) — Whether to display the UVS scale

## LibraryElement (class)

Representation of an element from the Allplan library

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/)

### Constructors
- `LibraryElement() | LibraryElement(libEleProp: LibraryElementProperties) | LibraryElement(libEle: LibraryElement)` — Initialize

### Methods
#### `GetCount() -> int`

Get the element count

**Remarks:** Get the element count

**Returns:** `int` — Element count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetCount)

#### `GetGeometryElements(doc: DocumentAdapter) -> list`

Get the geometry elements

**Remarks:** Get the geometry elements

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `list` — Geometry elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetGeometryElements)

#### `GetMinMax(doc: DocumentAdapter) -> MinMax3D`

Get the min/max box of the element

**Remarks:** Get the min/max box of the element

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `MinMax3D` — Min/max box of the element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetMinMax)

#### `GetProperties() -> LibraryElementProperties`

Get the properties

**Remarks:** Get the properties

**Returns:** `LibraryElementProperties` — Properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetProperties)

#### `GetReferencePoint(doc: DocumentAdapter) -> Point3D`

Get the reference point

**Remarks:** Get the reference point

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `Point3D` — Reference point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.GetReferencePoint)

#### `Move(moveVec: Vector3D)`

Move the element

**Remarks:** Move the element

**Parameters:**
- `moveVec` (Vector3D) — Move vector

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.Move)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElement/#NemAll_Python_BasisElements.LibraryElement.__repr__)

## LibraryElementProperties (class)

Properties of the library element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/)

### Constructors
- `LibraryElementProperties() | LibraryElementProperties( fullPathName: str, elementType: LibraryElementType, placementMatrix: Matrix3D, ) | LibraryElementProperties( fullPathName: str, elementType: LibraryElementType, placementMatrices: Matrix3DList, ) | LibraryElementProperties( path: str, group: str, element: str, elementType: LibraryElementType, placementMatrix: Matrix3D, ) | LibraryElementProperties( path: str, group: str, element: str, elementType: LibraryElementType, placementMatrices: Matrix3DList, ) | LibraryElementProperties( arg2: str, path: str, group: str, element: str, elementType: LibraryElementType, placementMatrix: Matrix3D, ) | LibraryElementProperties( arg2: str, path: str, group: str, element: str, elementType: LibraryElementType, placementMatrices: Matrix3DList, )` — Initialize

### Methods
#### `GetElement() -> str`

Get the element

**Remarks:** Get the element

**Returns:** `str` — Element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetElement)

#### `GetElementType() -> LibraryElementType`

Get the element type

**Remarks:** Get the element type

**Returns:** `LibraryElementType` — Element type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetElementType)

#### `GetGroup() -> str`

Get the group

**Remarks:** Get the group

**Returns:** `str` — Group

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetGroup)

#### `GetPath() -> str`

Get the path

**Remarks:** Get the path

**Returns:** `str` — Path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPath)

#### `GetPlacementMatrices() -> Matrix3DList`

Get the placement matrices

**Remarks:** Get the placement matrices

**Returns:** `Matrix3DList` — Placement matrices

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPlacementMatrices)

#### `GetPlacementMatrix() -> Matrix3D`

Get the placement matrix

**Remarks:** Get the placement matrix

**Returns:** `Matrix3D` — Placement matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPlacementMatrix)

#### `GetPolyline() -> Polyline3D`

Get the polygon points in case of a line fixture

**Remarks:** Get the polygon points in case of a line fixture

**Returns:** `Polyline3D` — Polyline points of the line fixture

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetPolyline)

#### `GetProducer() -> str`

Get the producer

**Remarks:** Get the producer

**Returns:** `str` — Producer

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetProducer)

#### `GetSingleFilePath() -> str`

Get the Single File Path

**Remarks:** Get the Single File Path

**Returns:** `str` — SingleFilePath

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.GetSingleFilePath)

#### `SetPlacementMatrices(placementMatrices: Matrix3DList)`

Set the placement matrices

**Remarks:** Set the placement matrices

**Parameters:**
- `placementMatrices` (Matrix3DList) — Placement matrices

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetPlacementMatrices)

#### `SetPlacementMatrix(placementMatrix: Matrix3D)`

Set the placement matrix

**Remarks:** Set the placement matrix

**Parameters:**
- `placementMatrix` (Matrix3D) — Placement matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetPlacementMatrix)

#### `SetPolyline(polyline: Polyline3D)`

Set the polygon points in case of a line fixture

**Remarks:** Set the polygon points in case of a line fixture

**Parameters:**
- `polyline` (Polyline3D) — Polyline of a line fixture

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetPolyline)

#### `SetProducer(producer: str)`

Set the producer

**Remarks:** Set the producer

**Parameters:**
- `producer` (str) — Producer

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementProperties/#NemAll_Python_BasisElements.LibraryElementProperties.SetProducer)

### Properties
- `PlacementMatrices` (Matrix3DList, get/set) — Get the placement matrices
- `PlacementMatrix` (Matrix3D, get/set) — Get the placement matrix
- `Polyline` (Polyline3D, get/set) — Get the polygon points in case of a line fixture
- `Producer` (str, get/set) — Get the producer

## LibraryElementType (enum)

Types of library elements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LibraryElementType/)

### Values
- `eFixture` = `1` — Fixture from fixture catalog
- `eFixtureSingleFile` = `3` — Fixture from Allplan library
- `eSmartSymbol` = `0` — Smart symbol
- `eSymbol` = `2` — Symbol

## LinkType (enum)

Definition of link type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/LinkType/)

### Values
- `eLinkNothing` = `0`
- `eLinkToCeilingSurface` = `3`
- `eLinkToFloorSurface` = `4`
- `eLinkToRoofSlab` = `2`
- `eLinkToRoom` = `1`

## MacroElement (class)

Representation of the macro definition (smart symbol) in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/)

### Constructors
- `MacroElement() | MacroElement(macroProp: MacroProperties, slideList: list)` — Initialize

### Methods
#### `GetAttributesForSubElementInStrucutredContainer(subElementID: type) -> object`

Get attributes for sub element in StructuredContainer

**Remarks:** Get attributes for sub element in StructuredContainer

**Parameters:**
- `subElementID` (type) — Sub element id

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetAttributesForSubElementInStrucutredContainer)

#### `GetHash() -> str`

Get the hash value

**Remarks:** Get the hash value

**Returns:** `str` — Hash value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetHash)

#### `GetMacroProperties() -> MacroProperties`

Get the Macro properties

**Remarks:** Get the Macro properties

**Returns:** `MacroProperties` — Macro properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetMacroProperties)

#### `GetSlideList() -> list`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.GetSlideList)

#### `SetAttributesForSubElementInStrucutredContainer( attributeContainer: object, subElementID: type )`

Set attributes to sub element in StructuredContainer

**Remarks:** Set attributes to sub element in StructuredContainer

**Parameters:**
- `attributeContainer` (object) — AttributeSet
- `subElementID` (type) — Sub element id

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.SetAttributesForSubElementInStrucutredContainer)

#### `SetHash(hash: str)`

Set the hash value

**Remarks:** Set the hash value

**Parameters:**
- `hash` (str) — Hash value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.SetHash)

#### `SetMacroProperties(MacroProp: MacroProperties)`

Set the Macro properties

**Remarks:** Set the Macro properties

**Parameters:**
- `MacroProp` (MacroProperties) — Macro properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.SetMacroProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroElement/#NemAll_Python_BasisElements.MacroElement.__repr__)

## MacroGroupElement (class)

Representation of the macro group (group of smart symbols) in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/)

### Constructors
- `MacroGroupElement() | MacroGroupElement(macroGroupProp: MacroGroupProperties, placementList: list) | MacroGroupElement( commonProp: CommonProperties, macroGroupProp: MacroGroupProperties, placementList: list, ) | MacroGroupElement(element: MacroGroupElement)` — Initialize

### Methods
#### `GetMacroGroupProperties() -> MacroGroupProperties`

Get the macro group properties

**Remarks:** Get the macro group properties

**Returns:** `MacroGroupProperties` — MacroGroup properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.GetMacroGroupProperties)

#### `GetPlacementList() -> List[MacroPlacementElement]`

Get the placement list

**Remarks:** Get the placement list

**Returns:** `List[MacroPlacementElement]` — Placements of macro group

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.GetPlacementList)

#### `SetGeometryParameterValueList(geometryParameterValueList: list)`

Set the geometry parameter value list

**Remarks:** Set the geometry parameter value list

**Parameters:**
- `geometryParameterValueList` (list) — Geometry parameter value list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.SetGeometryParameterValueList)

#### `SetMacroGroupProperties(macroGroupProp: MacroGroupProperties)`

Set the macro group properties

**Remarks:** Set the macro group properties

**Parameters:**
- `macroGroupProp` (MacroGroupProperties) — MacroGroup properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.SetMacroGroupProperties)

#### `TransformElement(transMat: Matrix3D)`

Args: transMat

**Remarks:** Args: transMat

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.TransformElement)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupElement/#NemAll_Python_BasisElements.MacroGroupElement.__repr__)

### Properties
- `MacroGroupProperties` (MacroGroupProperties, get/set) — Get the macro group properties

## MacroGroupProperties (class)

Properties of the macro group.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupProperties/)

### Constructors
- `MacroGroupProperties() | MacroGroupProperties(element: MacroGroupProperties)` — Initialize

### Methods
#### `__eq__(prop: MacroGroupProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroGroupProperties) — MacroGroupProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupProperties/#NemAll_Python_BasisElements.MacroGroupProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroGroupProperties/#NemAll_Python_BasisElements.MacroGroupProperties.__repr__)

### Properties
- `Name` (str, get/set) — Name

## MacroPlacementElement (class)

Representation of the macro placement. In Allplan referred to as instance of a smart symbol

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/)

### Constructors
- `MacroPlacementElement() | MacroPlacementElement( commonProp: CommonProperties, macroPlacementProp: MacroPlacementProperties, macro: object, reinforcementList: list, libraryElementList: list = [], architectureElementsList: list = [], fixtureElementsList: list = [], assemblyGroupList: list = [], precastMWSList: list = [], ) | MacroPlacementElement(placement: MacroPlacementElement)` — Initialize

### Methods
#### `GetArchitectureElementsList() -> list`

Get the architecture elements

**Remarks:** Get the architecture elements

**Returns:** `list` — Architecture elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetArchitectureElementsList)

#### `GetAssemblyGoupList() -> list`

Get the assembly group elements

**Remarks:** Get the assembly group elements

**Returns:** `list` — Assembly group elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetAssemblyGoupList)

#### `GetAttributesList() -> List[AttributeSet]`

Get the attributes list

**Remarks:** Get the attributes list

**Returns:** `List[AttributeSet]` — Attributes list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetAttributesList)

#### `GetFixtureElementsList() -> list`

Get the fixture elements

**Remarks:** Get the fixture elements

**Returns:** `list` — Fixture elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetFixtureElementsList)

#### `GetLibraryElementsList() -> list`

Get the library elements

**Remarks:** Get the library elements

**Returns:** `list` — Library elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetLibraryElementsList)

#### `GetMacro() -> MacroElement`

Get the corresponding macro definition

**Remarks:** Get the corresponding macro definition

**Returns:** `MacroElement` — Macro definition element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetMacro)

#### `GetMacroPlacementProperties() -> MacroPlacementProperties`

Get the macro placement properties

**Remarks:** Get the macro placement properties

**Returns:** `MacroPlacementProperties` — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetMacroPlacementProperties)

#### `GetPrecastMWSList() -> list`

Get the Precast MWS elements

**Remarks:** Get the Precast MWS elements

**Returns:** `list` — Precast MWS elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetPrecastMWSList)

#### `GetReinforcementList() -> list`

Get the reinforcement elements

**Remarks:** Get the reinforcement elements

**Returns:** `list` — Reinforcement elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.GetReinforcementList)

#### `SetGeometryParameterValueList(geometryParameterValueList: list)`

Set the geometry parameter value list

**Remarks:** Set the geometry parameter value list

**Parameters:**
- `geometryParameterValueList` (list) — Geometry parameter value list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.SetGeometryParameterValueList)

#### `SetMacroPlacementProperties(macroPlacementProp: MacroPlacementProperties)`

Set the macro placement properties

**Remarks:** Set the macro placement properties

**Parameters:**
- `macroPlacementProp` (MacroPlacementProperties) — MacroPlacement properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.SetMacroPlacementProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementElement/#NemAll_Python_BasisElements.MacroPlacementElement.__repr__)

### Properties
- `MacroPlacementProperties` (MacroPlacementProperties, get/set) — Properties of the macro placement

## MacroPlacementProperties (class)

Properties of the macro placement

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementProperties/)

### Constructors
- `MacroPlacementProperties() | MacroPlacementProperties(element: MacroPlacementProperties)` — Initialize

### Methods
#### `__eq__(prop: MacroPlacementProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroPlacementProperties) — MacroPlacementProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementProperties/#NemAll_Python_BasisElements.MacroPlacementProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroPlacementProperties/#NemAll_Python_BasisElements.MacroPlacementProperties.__repr__)

### Properties
- `BillingCategory` (int, get/set) — Get the Billing category
- `ConsiderType` (ConsiderType, get/set) — The type of adaptation behavior of the smart symbol relative to finishing surfaces
- `Craft` (int, get/set) — Get the Craft
- `DistortionState` (bool, get/set) — Whether to allow the resize of the smart symbol
- `DomainType` (int, get/set) — Get the Domain type?
- `HasParentModificationBehaviour` (bool, get/set) — Get the Specific behavior for modification
- `HeightDefinitionType` (HeightDefinitionType, get/set) — Get the Height definition type
- `InOpeningState` (bool, get/set) — Get the Is the macro placement inside opening ?
- `LeadingMacro` (bool, get/set) — Is the macro placement a leading macro
- `LinkType` (LinkType, get/set) — Get the Link type
- `Mass_V6` (float, get/set) — Get the Mass attribute V6
- `Mass_V7` (float, get/set) — Get the Mass attribute V7
- `Mass_V8` (float, get/set) — Get the Mass attribute V8
- `Mass_V9` (float, get/set) — Get the Mass attribute V9
- `Matrix` (Matrix3D, get/set) — Get the Matrix for location in world coordinate system
- `MirrorState` (bool, get/set) — Get the Was the macro placement mirrored ?
- `Name` (str, get/set) — Get the Name
- `PositionNr` (int, get/set) — Get the Unit factor
- `SubType` (int, get/set) — Get the Subtype?
- `Type` (int, get/set) — Get the Type?
- `UnitFactor` (float, get/set) — Get the Unit factor
- `UseAlways2DRepInGroundView` (bool, get/set) — Get the Use always 2D representation in ground view
- `UseDrawOrder` (bool, get/set) — Set to True to use the draw order (sequence) setting of the smart symbol's instance. Set to False to use the setting of elements included in the macro slide
- `UseFormat` (bool, get/set) — Set to True to use the format setting (pen, stroke, color) of the smart symbol's instance. Set to False to use the settings of the elements included in the macro slide.

## MacroProperties (class)

Properties of the macro definition

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroProperties/)

### Constructors
- `MacroProperties() | MacroProperties(element: MacroProperties)` — Initialize

### Methods
#### `__eq__(prop: MacroProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroProperties) — MacroProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroProperties/#NemAll_Python_BasisElements.MacroProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroProperties/#NemAll_Python_BasisElements.MacroProperties.__repr__)

### Properties
- `CatalogName` (str, get/set) — Get the Catalog name
- `DomainType` (int, get/set) — Get the Domain type
- `InsertionPoint` (Point3D, get/set) — Macro reference point
- `IsScaleDependent` (bool, get/set) — Whether the macro should be scale dependent
- `Name` (str, get/set) — Get the Name
- `PositionNr` (int, get/set) — Get the Position number
- `SubType` (int, get/set) — Get the Sub type
- `UnitFactor` (float, get/set) — Get the Unit factor

## MacroSlideElement (class)

Representation of a macro slide. In Allplan referred to as foil of a smart symbol

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/)

### Constructors
- `MacroSlideElement() | MacroSlideElement(macroSlideProp: MacroSlideProperties, objectList: list)` — Initialize

### Methods
#### `GetMacroSlideProperties() -> MacroSlideProperties`

Get the MacroSlide properties

**Remarks:** Get the MacroSlide properties

**Returns:** `MacroSlideProperties` — MacroSlide properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.GetMacroSlideProperties)

#### `GetObjectList() -> list`

Get the slide object list

**Remarks:** Get the slide object list

**Returns:** `list` — Slide object list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.GetObjectList)

#### `SetMacroSlideProperties(MacroSlideProp: MacroSlideProperties)`

Set the MacroSlide properties

**Remarks:** Set the MacroSlide properties

**Parameters:**
- `MacroSlideProp` (MacroSlideProperties) — MacroSlide properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.SetMacroSlideProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideElement/#NemAll_Python_BasisElements.MacroSlideElement.__repr__)

## MacroSlideProperties (class)

Properties of the macro slide

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideProperties/)

### Constructors
- `MacroSlideProperties() | MacroSlideProperties(element: MacroSlideProperties)` — Initialize

### Methods
#### `__eq__(prop: MacroSlideProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (MacroSlideProperties) — MacroSlideProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideProperties/#NemAll_Python_BasisElements.MacroSlideProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideProperties/#NemAll_Python_BasisElements.MacroSlideProperties.__repr__)

### Properties
- `EndScaleRange` (float, get/set) — The upper limit of the scale range in which the slide will be drawn
- `OffsetOfReferencePoint1` (Vector3D, get/set) — Get the First offset value to reference point
- `OffsetOfReferencePoint2` (Vector3D, get/set) — Get the Second offset value to reference point
- `ReferencePoint` (Point3D, get/set) — Get the Reference point of this slide
- `ResizeSettingVx` (CombinationType, get/set) — The resizing combination setting for x direction
- `ResizeSettingVy` (CombinationType, get/set) — The resizing combination setting for y direction
- `ResizeSettingVz` (CombinationType, get/set) — The resizing combination setting for z direction
- `StartScaleRange` (float, get/set) — The lower limit of the scale range in which the slide will be drawn
- `Type` (MacroSlideType, get/set) — Get the Type of macro slide
- `VisibilityGeo2D` (bool, get/set) — Whether the slide should be considered as 2D element (ex. to be visible in ground view)
- `VisibilityGeo3D` (bool, get/set) — Whether the slide should be considered as 3D element (ex. to be visible in isometric view)
- `VisibilityLayerA` (bool, get/set) — Whether the slide should be visible on the smart symbol foil A
- `VisibilityLayerB` (bool, get/set) — Whether the slide should be visible on the smart symbol foil B
- `VisibilityLayerC` (bool, get/set) — Whether the slide should be visible on the smart symbol foil C

## MacroSlideType (enum)

Definition of the macro slide type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/MacroSlideType/)

### Values
- `eCode` = `1`
- `eExtension` = `5`
- `eGeometry` = `0`
- `eReinforcement` = `2`
- `eReport` = `3`
- `eUndergroundCadaster` = `4`

## ModelElement2D (class)

Representation of a general two-dimensional model element, such as line, arc or polyline.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/)

### Constructors
- `ModelElement2D() | ModelElement2D(commonProp: CommonProperties, geometryObject: object) | ModelElement2D( commonProp: CommonProperties, patternCurveProp: PatternCurveProperties, endSymbolProp: EndSymbolsProperties, geometryObject: object, ) | ModelElement2D(element: ModelElement2D)` — Initialize

### Methods
#### `GetEndSymbolsProperties() -> EndSymbolsProperties`

Get the end symbols properties

**Remarks:** Get the end symbols properties

**Returns:** `EndSymbolsProperties` — End symbols properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.GetEndSymbolsProperties)

#### `GetPatternCurveProperties() -> PatternCurveProperties`

Get the pattern curve properties

**Remarks:** Get the pattern curve properties

**Returns:** `PatternCurveProperties` — Pattern curve properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.GetPatternCurveProperties)

#### `GetTransformationList() -> list`

Get transformation list

**Remarks:** Get transformation list

**Returns:** `list` — List with the transformations

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.GetTransformationList)

#### `SetEndSymbolsProperties(endSymbolsProp: EndSymbolsProperties)`

Set the end symbols properties

**Remarks:** Set the end symbols properties

**Parameters:**
- `endSymbolsProp` (EndSymbolsProperties) — End symbols properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.SetEndSymbolsProperties)

#### `SetPatternCurveProperties(patternCurveProp: PatternCurveProperties)`

Set the pattern curve properties

**Remarks:** Set the pattern curve properties

**Parameters:**
- `patternCurveProp` (PatternCurveProperties) — Pattern curve properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.SetPatternCurveProperties)

#### `SetTransformationList(transformationList: list)`

Set the transformation list

**Remarks:** Set the transformation list

**Parameters:**
- `transformationList` (list) — List with the transformations

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.SetTransformationList)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement2D/#NemAll_Python_BasisElements.ModelElement2D.__repr__)

### Properties
- `EndSymbolsProperties` (EndSymbolsProperties, get/set) — Get the end symbols properties
- `PatternCurveProperties` (PatternCurveProperties, get/set) — Get the pattern curve properties
- `TransformationList` (list, get/set) — Get transformation list

## ModelElement3D (class)

Representation of a general three-dimensional model element, such as 3D curve, surface or solid

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/)

### Constructors
- `ModelElement3D() | ModelElement3D(commonProp: CommonProperties, geometryObject: object) | ModelElement3D( commonProp: CommonProperties, BrepIsoLinesU: int, BrepIsoLinesV: int, geometryObject: object, ) | ModelElement3D( commonProp: CommonProperties, textureDefinition: TextureDefinition, geometryObject: object, ) | ModelElement3D( commonProp: CommonProperties, textureDefinition: TextureDefinition, BrepIsoLinesU: int, BrepIsoLinesV: int, geometryObject: object, ) | ModelElement3D( commonProp: CommonProperties, textureDefinition: TextureDefinition, textureMapping: TextureMapping, geometryObject: object, ) | ModelElement3D( commonProp: CommonProperties, textureDefinition: TextureDefinition, textureMapping: TextureMapping, BrepIsoLinesU: int, BrepIsoLinesV: int, geometryObject: object, ) | ModelElement3D(element: ModelElement3D)` — Initialize

### Methods
#### `GetTextureDefinition() -> TextureDefinition`

Get the texture definition

**Remarks:** Get the texture definition

**Returns:** `TextureDefinition` — Texture definition (surface filename)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.GetTextureDefinition)

#### `GetTextureMapping() -> TextureMapping`

Get the texture mapping

**Remarks:** Get the texture mapping

**Returns:** `TextureMapping` — Texture mapping properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.GetTextureMapping)

#### `GetTransformationList() -> list`

Get transformation list

**Remarks:** Get transformation list

**Returns:** `list` — List with the transformations

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.GetTransformationList)

#### `IsValidateGeometry() -> bool`

Get the validate geometry state

**Remarks:** Get the validate geometry state

**Returns:** `bool` — Validate the geometry state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.IsValidateGeometry)

#### `SetTextureDefinition(textureDefinition: TextureDefinition)`

Set the texture definition

**Remarks:** Set the texture definition

**Parameters:**
- `textureDefinition` (TextureDefinition) — Texture definition (surface filename)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetTextureDefinition)

#### `SetTextureMapping(textureMapping: TextureMapping)`

Set the texture mapping

**Remarks:** Set the texture mapping

**Parameters:**
- `textureMapping` (TextureMapping) — Texture mapping properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetTextureMapping)

#### `SetTransformationList(transformationList: list)`

Set the transformation list

**Remarks:** Set the transformation list

**Parameters:**
- `transformationList` (list) — List with the transformations

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetTransformationList)

#### `SetValidateGeometry(validateGeometry: bool)`

Set the validate geometry state

**Remarks:** Set the validate geometry state

**Parameters:**
- `validateGeometry` (bool) — Validate the geometry

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.SetValidateGeometry)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ModelElement3D/#NemAll_Python_BasisElements.ModelElement3D.__repr__)

### Properties
- `TextureDefinition` (TextureDefinition, get/set) — Get the texture definition
- `TextureMapping` (TextureMappingProperties, get/set) — Get the texture mapping
- `TransformationList` (list, get/set) — Get transformation list

## PatternCurveAlignment (enum)

Pattern curve alignment types of the pattern curve property

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveAlignment/)

### Values
- `eCenter` = `1`
- `eLeft` = `0`
- `eRight` = `2`

## PatternCurveIntersectionType (enum)

Pattern intersection types of the pattern curve property

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveIntersectionType/)

### Values
- `eDisabled` = `0`
- `eJoint` = `2`
- `eMiter` = `1`
- `eSeamless` = `3`

## PatternCurveProperties (class)

PatternCurveProperties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveProperties/)

### Constructors
- `PatternCurveProperties()` — Initialize

### Methods
#### `__eq__(prop: PatternCurveProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PatternCurveProperties) — PatternCurveProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveProperties/#NemAll_Python_BasisElements.PatternCurveProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternCurveProperties/#NemAll_Python_BasisElements.PatternCurveProperties.__repr__)

### Properties
- `AlignmentType` (None, get) — the position of the pattern relative to the reference curve
- `Height` (None, get) — Property for height of pattern :type: None
- `IntersectionType` (None, get) — Property for intersection type :type: None
- `IsDrawReferenceCurve` (None, get) — Whether to draw the reference curve
- `IsLockedToCorner` (None, get) — Whether to lock the corner
- `IsMirrorPattern` (None, get) — Whether to mirror the pattern
- `IsScaleDependent` (None, get) — Whether the pattern is to be scale dependent
- `PatternID` (None, get) — Property for pattern ID :type: None
- `Width` (None, get) — Property for width of pattern :type: None

## PatternElement (class)

Representation of pattern in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/)

### Constructors
- `PatternElement() | PatternElement( commonProp: CommonProperties, patternProp: PatternProperties, polygon: Polygon2D, ) | PatternElement(element: PatternElement)` — Initialize

### Methods
#### `GetPatternProperties() -> PatternProperties`

Get the Pattern properties

**Remarks:** Get the Pattern properties

**Returns:** `PatternProperties` — Pattern properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/#NemAll_Python_BasisElements.PatternElement.GetPatternProperties)

#### `SetPatternProperties(patternProp: PatternProperties)`

Set the Pattern properties

**Remarks:** Set the Pattern properties

**Parameters:**
- `patternProp` (PatternProperties) — Pattern properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/#NemAll_Python_BasisElements.PatternElement.SetPatternProperties)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternElement/#NemAll_Python_BasisElements.PatternElement.__repr__)

### Properties
- `PatternProperties` (PatternProperties, get/set) — Get the Pattern properties

## PatternProperties (class)

Properties of the pattern

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/)

### Constructors
- `PatternProperties() | PatternProperties(element: PatternProperties)` — Initialize

### Methods
#### `SetBackgroundColorBGR(value: int)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as BGR

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.SetBackgroundColorBGR)

#### `SetBackgroundColorIRGB(value: int)`

Set the background color

**Remarks:** Set the background color

**Parameters:**
- `value` (int) — Color value as IRGB

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.SetBackgroundColorIRGB)

#### `__eq__(prop: PatternProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (PatternProperties) — PatternProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PatternProperties/#NemAll_Python_BasisElements.PatternProperties.__repr__)

### Properties
- `BackgroundColor` (ARGB, get/set) — Background color. Relevant if UseBackgroundColor is set to True
- `IsScaleDependent` (bool, get/set) — Whether the pattern is to be scale dependent
- `PatternID` (int, get/set) — Get the pattern ID
- `PlacementType` (PlacementType, get/set) — Placing mode
- `ReferencePoint` (Point2D, get/set) — reference point. Relevant if UseReferencePoint is set to True.
- `RotationAngle` (Angle, get/set) — Rotation angle
- `UseBackgroundColor` (bool, get/set) — Whether to use a background color in the pattern
- `UseReferencePoint` (bool, get/set) — Whether to use the reference point from ReferencePoint property for the pattern origin
- `XScalingFactor` (float, get/set) — Get the scaling factor in x direction
- `YScalingFactor` (float, get/set) — Get the scaling factor in y direction

## PlacementType (enum)

Trimming modes of a pattern (PatternElement)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/PlacementType/)

### Values
- `eFitting` = `1` — Trim pattern along boundary
- `eInsideFitting` = `2` — Trim pattern to full segments
- `eOutsideFitting` = `0` — No trim

## SectionAlongPathClippingPathProperties (class)

Clipping path properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/)

### Constructors
- `SectionAlongPathClippingPathProperties(bInitFromSTW: bool = False) | SectionAlongPathClippingPathProperties(B: SectionAlongPathClippingPathProperties)` — constructor

### Methods
#### `GetClippingPathGeometryTolerance() -> float`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — ClippingPathGeometryTolerance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.GetClippingPathGeometryTolerance)

#### `GetPathLength() -> float`

Get length of path

**Remarks:** Get length of path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.GetPathLength)

#### `GetResultPathLength() -> float`

Get length of the part of path limited by EndCoord and StartCoord

**Remarks:** Get length of the part of path limited by EndCoord and StartCoord

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.GetResultPathLength)

#### `RestrictEmptySectionId()`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathProperties.RestrictEmptySectionId)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/)

### Constructors
- `SectionAlongPathClippingPathViewProperties(bInitFromSTW: bool = False) | SectionAlongPathClippingPathViewProperties(B: SectionAlongPathClippingPathViewProperties)` — constructor

### Methods
#### `ConvertDirectionSymbolNumberFromViewModel(arg2: int, iPaletteIconNum: bool)`

Convert icon number used in WPF palette to symbol number used in Allplan for the same graphical representation

**Remarks:** Convert icon number used in WPF palette to symbol number used in Allplan for the same graphical representation

**Parameters:**
- `iPaletteIconNum` (bool) — number of icon from palette (view model)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathViewProperties.ConvertDirectionSymbolNumberFromViewModel)

#### `ConvertDirectionSymbolNumberToViewModel(arg2: bool) -> int`

Convert symbol number used in Allplan to icon number used in WPF palette for the same graphical representation

**Remarks:** Convert symbol number used in Allplan to icon number used in WPF palette for the same graphical representation

**Returns:** `int` — icon number used in WPF

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathViewProperties.ConvertDirectionSymbolNumberToViewModel)

#### `GetDirectionSymbolHeightTolerance() -> float`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — symbol height tolerance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathClippingPathViewProperties/#NemAll_Python_BasisElements.SectionAlongPathClippingPathViewProperties.GetDirectionSymbolHeightTolerance)

### Properties
- `DirectionSymbolHeight` (float, get/set) — Get the DirectionSymbolHeight
- `DirectionSymbolNr` (int, get/set) — Get the DirectionSymbolNr
- `IsDirectionSymbolOn` (bool, get/set) — Get the IsDirectionSymbolOn
- `TextParameterProperties` (SectionAlongPathTextParameterProperties, get/set) — Access text params Get the SectionAlongPathTextParameterProperties

## SectionAlongPathElement (class)

Implementation of the section along path element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathElement/)

### Constructors
- `SectionAlongPathElement( sectionAlongPathProperties: SectionAlongPathProperties, sectionPathElement: BaseElementAdapter, ) | SectionAlongPathElement( sectionAlongPathProperties: SectionAlongPathProperties, sectionPath: Path2D, comProp: CommonProperties, ) | SectionAlongPathElement(element: SectionAlongPathElement)` — Default constructor

## SectionAlongPathElevationSpecifications (class)

Elevation specifications

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathElevationSpecifications/)

### Constructors
- `SectionAlongPathElevationSpecifications(bInitFromSTW: bool = False) | SectionAlongPathElevationSpecifications(B: SectionAlongPathElevationSpecifications)` — constructor

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFilterProperties/)

### Constructors
- `SectionAlongPathFilterProperties(bInitFromSTW: bool = False) | SectionAlongPathFilterProperties(B: SectionAlongPathFilterProperties)` — constructor

### Properties
- `DrawingFilesProperties` (SectionDrawingFilesProperties, get/set) — Access drawing file params Get the SectionDrawingFilesProperties
- `IsAssociativityOn` (bool, get/set) — Get the When the section is frozen / associativity is OFF -, no updates are available for it
- `IsSelectionFilterOn` (bool, get/set) — Get the when == true - only selected elements are included to UVS model / when == false (default value) - selected elements are excluded from UVS model
- `LayerProperties` (SectionLayerProperties, get/set) — Access layer params Get the SectionLayerProperties

## SectionAlongPathFormatProperties (class)

Format properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFormatProperties/)

### Constructors
- `SectionAlongPathFormatProperties(bInitFromSTW: bool = False) | SectionAlongPathFormatProperties(B: SectionAlongPathFormatProperties)` — constructor

### Methods
#### `GetEliminationAngleTolerance() -> float`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — elimination angle tolerance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFormatProperties/#NemAll_Python_BasisElements.SectionAlongPathFormatProperties.GetEliminationAngleTolerance)

#### `GetOverhangTolerance() -> float`

Access tolerances

**Remarks:** Access tolerances

**Returns:** `float` — overhang tolerance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathFormatProperties/#NemAll_Python_BasisElements.SectionAlongPathFormatProperties.GetOverhangTolerance)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/)

### Constructors
- `SectionAlongPathGeneralSectionProperties(bInitFromSTW: bool = False) | SectionAlongPathGeneralSectionProperties(B: SectionAlongPathGeneralSectionProperties)` — constructor

### Methods
#### `GetOffset() -> tuple[bool, float, float]`

Get offset

**Remarks:** Get offset

**Returns:** `tuple[bool, float, float]` — tuple(true if it was read from 55th element, X coordinate of section's left bottom corner, Y coordinate of section's left bottom corner)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/#NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties.GetOffset)

#### `IsOffsetValid() -> bool`

Get offset state

**Remarks:** Get offset state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/#NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties.IsOffsetValid)

#### `SetOffset(dOffset_X: float, dOffset_Y: float)`

Set offset

**Remarks:** Set offset

**Parameters:**
- `dOffset_X` (float) — X coordinate of section's left bottom corner
- `dOffset_Y` (float) — Y coordinate of section's left bottom corner

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathGeneralSectionProperties/#NemAll_Python_BasisElements.SectionAlongPathGeneralSectionProperties.SetOffset)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathLabelingStripSetting/)

### Constructors
- `SectionAlongPathLabelingStripSetting(bInitFromSTW: bool = False) | SectionAlongPathLabelingStripSetting(B: SectionAlongPathLabelingStripSetting)` — constructor

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathProperties/)

### Constructors
- `SectionAlongPathProperties(bInitFromSTW: bool = False) | SectionAlongPathProperties(B: SectionAlongPathProperties)` — constructor

### Properties
- `ClippingPathProperties` (SectionAlongPathClippingPathProperties, get/set) — Get the SectionAlongPathClippingPathProperties
- `FilterProperties` (SectionAlongPathFilterProperties, get/set) — Get the SectionAlongPathFilterProperties
- `GeneralSectionProperties` (SectionAlongPathGeneralSectionProperties, get/set) — Get the SectionAlongPathGeneralSectionProperties
- `ScaleProperties` (SectionAlongPathScaleProperties, get/set) — Get the SectionAlongPathScaleProperties

## SectionAlongPathScaleProperties (class)

Scale properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathScaleProperties/)

### Constructors
- `SectionAlongPathScaleProperties(bInitFromSTW: bool = False) | SectionAlongPathScaleProperties(B: SectionAlongPathScaleProperties)` — constructor

### Methods
#### `GetScaleFactorTolerance() -> float`

Access tolerance

**Remarks:** Access tolerance

**Returns:** `float` — scale factor tolerance

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathScaleProperties/#NemAll_Python_BasisElements.SectionAlongPathScaleProperties.GetScaleFactorTolerance)

### Properties
- `ScaleFactorX` (float, get/set) — Get the resizing factor X in Longitudinal direction
- `ScaleFactorY` (float, get/set) — Get the resizing factor Y in Transversal direction

## SectionAlongPathSectionLabelingProperties (class)

Section labeling properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionLabelingProperties/)

### Constructors
- `SectionAlongPathSectionLabelingProperties(bInitFromSTW: bool = False) | SectionAlongPathSectionLabelingProperties(B: SectionAlongPathSectionLabelingProperties)` — constructor

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionViewProperties/)

### Constructors
- `SectionAlongPathSectionViewProperties(bInitFromSTW: bool = False) | SectionAlongPathSectionViewProperties(B: SectionAlongPathSectionViewProperties)` — constructor

### Methods
#### `GetHorizontTolerance() -> float`

Access tolerances

**Remarks:** Access tolerances

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionViewProperties/#NemAll_Python_BasisElements.SectionAlongPathSectionViewProperties.GetHorizontTolerance)

#### `GetOffsetTolerance() -> float`

Access tolerances

**Remarks:** Access tolerances

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathSectionViewProperties/#NemAll_Python_BasisElements.SectionAlongPathSectionViewProperties.GetOffsetTolerance)

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

## SectionAlongPathTextParameterProperties (class)

Text parameter properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathTextParameterProperties/)

### Constructors
- `SectionAlongPathTextParameterProperties() | SectionAlongPathTextParameterProperties(iTextParameterID: int, bInitFromSTW: bool = False) | SectionAlongPathTextParameterProperties(bInitFromSTW: bool) | SectionAlongPathTextParameterProperties(A: SectionAlongPathTextParameterProperties)` — Initialize

### Methods
#### `DetectBackgroundColorType( bHasBgColor: bool, bgColor: int, backgroundIsTransparent: bool ) -> tuple[int, int]`

Args: bHasBgColor bgColor backgroundIsTransparent

**Remarks:** Args: bHasBgColor bgColor backgroundIsTransparent

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathTextParameterProperties/#NemAll_Python_BasisElements.SectionAlongPathTextParameterProperties.DetectBackgroundColorType)

#### `FillPropertiesFromAllplanTextParameters( richTextFlags: int, layerId: int, byLayerFlags: int ) -> object`

Args: richTextFlags layerId byLayerFlags

**Remarks:** Args: richTextFlags layerId byLayerFlags

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionAlongPathTextParameterProperties/#NemAll_Python_BasisElements.SectionAlongPathTextParameterProperties.FillPropertiesFromAllplanTextParameters)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDefinitionData/)

### Constructors
- `SectionDefinitionData()` — Initialize

### Properties
- `ClippingPath` (None, get) — Set/get the clipping path :type: None
- `DefinitionProperties` (None, get) — Set/Get the definition properties :type: None
- `DirectionVector` (None, get) — Set/get the direction vector :type: None
- `HeightDirection` (None, get) — Set/get the height direction :type: None
- `SectionBody` (None, get) — Set/get the section body :type: None

## SectionDefinitionProperties (class)

Section definition properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDefinitionProperties/)

### Constructors
- `SectionDefinitionProperties()` — Initialize

### Properties
- `ClippingPathProperties` (None, get) — Set/Get the clipping path properties :type: None
- `IsAdvancedOn` (None, get) — Property for type of section architecture= OFF /Engineering = ON :type: None
- `IsSectionBodyOn` (None, get) — Property for show the section body :type: None
- `RefMode` (None, get) — Property for the reference mode :type: None
- `ViewInputType` (None, get) — Property for the view input type :type: None

## SectionDrawingFilesProperties (class)

Class containing settings regarding which drawing files to be considered in a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDrawingFilesProperties/)

### Constructors
- `SectionDrawingFilesProperties() | SectionDrawingFilesProperties(A: SectionDrawingFilesProperties)` — Initialize

### Methods
#### `__eq__(A: SectionDrawingFilesProperties) -> bool`

operator ==

**Remarks:** operator ==

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionDrawingFilesProperties/#NemAll_Python_BasisElements.SectionDrawingFilesProperties.__eq__)

### Properties
- `DrawingNumbers` (list[int] | VecIntList, get/set) — Numbers of the drawing files to consider in a UVS

## SectionFilterProperties (class)

Representation of filter settings of a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionFilterProperties/)

### Constructors
- `SectionFilterProperties()` — Initialize

### Properties
- `DrawingFilesProperties` (None, get) — Settings regarding which drawing files are to be considered in the UVS
- `IsAssociativityOn` (None, get) — Whether to update the UVS automatically
- `LayerProperties` (None, get) — Settings regarding which layer are to be considered in the UVS

## SectionFormatProperties (class)

Section format properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionFormatProperties/)

### Constructors
- `SectionFormatProperties() | SectionFormatProperties(bInitFromSTW: bool)` — Initialize

### Properties
- `EliminationAngle` (None, get) — Boundary angle from which the adjacent edges are eliminated
- `IsEliminationOn` (None, get) — Whether to eliminate the adjacent edges

## SectionGeneralProperties (class)

General properties of the UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionGeneralProperties/)

### Constructors
- `SectionGeneralProperties() | SectionGeneralProperties(bInitFromSTW: bool)` — Initialize

### Properties
- `FilterProperties` (None, get) — Filtering settings of the UVS
- `FormatProperties` (None, get) — Format settings of a UVS
- `HiddenSectionLinesProperties` (None, get) — Settings regarding hidden section lines in the UVS
- `LabelingProperties` (None, get) — Settings regarding labeling the UVS
- `PlacementAngle` (None, get) — Property for the placement angle :type: None
- `PlacementPoint` (None, get) — Property for the placement point :type: None
- `PlacementPointType` (None, get) — Position of the placement point relative to the UVS (bottom left, top right, etc...)
- `ShowSectionBody` (None, get) — Whether to show the section body in the UVS
- `Status` (None, get) — Type of the UVS calculation
- `VisibleHiddenEdgesProperties` (None, get) — Settings regarding hidden and visible lines in the UVS

## SectionLayerProperties (class)

Class containing settings regarding which layers are to be considered in a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionLayerProperties/)

### Constructors
- `SectionLayerProperties()` — Initialize

### Methods
#### `SetLayerProperties( iVisibilityFilterMode: int, bAreInvisibleLyersStored: bool, layerIdVector: VecUShortList, )`

Set the layer properties

**Remarks:** Set the layer properties

**Parameters:**
- `iVisibilityFilterMode` (int) — Type of selected filter mode
- `bAreInvisibleLyersStored` (bool) — When set to True, elements in excludedLayerList are filtered out. Otherwise elements in excludedLayerList are taken into consideration
- `layerIdVector` (VecUShortList) — Elements belonging to these Layers should be filtered (excluded) from source up to flag bAreInvisibleLyersStored

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SectionLayerProperties/#NemAll_Python_BasisElements.SectionLayerProperties.SetLayerProperties)

## ShadingType (enum)

Shading style of filling

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ShadingType/)

### Values
- `eFromCenter` = `2`
- `eFromCorner` = `1`
- `eLinear` = `0`
- `eRound` = `3`

## SubType (enum)

Sub types of the element group property

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SubType/)

### Values
- `eMultiLine3D` = `1`
- `eMultiLine3D_Group` = `2`
- `eUseNoSpecialSubType` = `0`

## SurfaceDefinition (class)

Surface resource definition

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/)

### Methods
#### `CompareData(ldef: SurfaceDefinition) -> bool`

compare the surface properties

**Remarks:** compare the surface properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.CompareData)

#### `CopyData(pSrcSurface: SurfaceDefinition, bCopyName: bool = True)`

copy the data from an other surface

**Remarks:** copy the data from an other surface

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.CopyData)

#### `Create() -> SurfaceDefinition`

Create a surface definition

**Remarks:** Create a surface definition

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.Create)

#### `Dump(deep: bool)`

Dumps content of the resource

**Remarks:** Dumps content of the resource

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.Dump)

#### `GetDiffuseFactor() -> float`

get the diffuse factor calculated from diffuse color intensity, reflection intensity and the transparency intensity

**Remarks:** get the diffuse factor calculated from diffuse color intensity, reflection intensity and the transparency intensity

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetDiffuseFactor)

#### `GetHash() -> int`

get the hash of the surface definition data

**Remarks:** get the hash of the surface definition data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetHash)

#### `GetReflectionFactor() -> float`

calculate the reflection factor from transparency and reflection intensity

**Remarks:** calculate the reflection factor from transparency and reflection intensity

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetReflectionFactor)

#### `GetScaleU() -> float`

get the texture u scale factor

**Remarks:** get the texture u scale factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetScaleU)

#### `GetScaleV() -> float`

get the texture v scale factor

**Remarks:** get the texture v scale factor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetScaleV)

#### `GetSeasonStatus() -> bool`

get the surface season flag , true - not all color texture are the same

**Remarks:** get the surface season flag , true - not all color texture are the same

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetSeasonStatus)

#### `GetSurfaceName() -> str`

get the surface name

**Remarks:** get the surface name

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetSurfaceName)

#### `GetTextureID(tex: eSurfaceTextureID) -> int`

get the bitmap id of the texture

**Remarks:** get the bitmap id of the texture

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetTextureID)

#### `GetTranslateU() -> float`

get the texture u offset

**Remarks:** get the texture u offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetTranslateU)

#### `GetTranslateV() -> float`

get the texture v offset

**Remarks:** get the texture v offset

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.GetTranslateV)

#### `IsDefault() -> bool`

check whether the surface properties has default values

**Remarks:** check whether the surface properties has default values

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.IsDefault)

#### `LoadSurfaceData(surfacePath: str)`

load surface properties only

**Remarks:** load surface properties only

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.LoadSurfaceData)

#### `PreviewCalculationSkipped() -> bool`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.PreviewCalculationSkipped)

#### `Save( surfacePath: str, bitmapDict: Dict[eSurfaceTextureID, BitmapDefinition] ) -> bool`

Create a surface definition

**Remarks:** Create a surface definition

**Parameters:**
- `surfacePath` (str) — Path of the bitmap
- `bitmapDefinitions` (object) — Dict with the bitmap definitions

**Returns:** `bool` — Creation successful state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.Save)

#### `SetScale(u: float, v: float)`

set the texture scaling factors -10000 to 10000

**Remarks:** set the texture scaling factors -10000 to 10000

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.SetScale)

#### `SetTextureID(tex: eSurfaceTextureID, bitmapID: int)`

set the bitmapID for one texture

**Remarks:** set the bitmapID for one texture

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.SetTextureID)

#### `SetTranslate(u: float, v: float)`

set the texture offset -INT_MAX to INT_MAX

**Remarks:** set the texture offset -INT_MAX to INT_MAX

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/SurfaceDefinition/#NemAll_Python_BasisElements.SurfaceDefinition.SetTranslate)

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

Representation of the point symbol in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/)

### Constructors
- `Symbol2DElement() | Symbol2DElement( commonProp: CommonProperties, Symbol2DProp: Symbol2DProperties, geometryObject: object, )` — Initialize

### Methods
#### `GetSymbol2DProperties() -> Symbol2DProperties`

Get the Symbol2D properties

**Remarks:** Get the Symbol2D properties

**Returns:** `Symbol2DProperties` — Symbol2D properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/#NemAll_Python_BasisElements.Symbol2DElement.GetSymbol2DProperties)

#### `SetSymbol2DProperties(symbol2DProp: Symbol2DProperties)`

Set the Symbol2D properties

**Remarks:** Set the Symbol2D properties

**Parameters:**
- `symbol2DProp` (Symbol2DProperties) — Symbol2D properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/#NemAll_Python_BasisElements.Symbol2DElement.SetSymbol2DProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DElement/#NemAll_Python_BasisElements.Symbol2DElement.__repr__)

## Symbol2DProperties (class)

Properties of the point symbol

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DProperties/)

### Constructors
- `Symbol2DProperties()` — Initialize

### Methods
#### `__eq__(prop: Symbol2DProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (Symbol2DProperties) — Symbol2DProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DProperties/#NemAll_Python_BasisElements.Symbol2DProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol2DProperties/#NemAll_Python_BasisElements.Symbol2DProperties.__repr__)

### Properties
- `Height` (None, get) — Symbol height
- `IsScaleDependent` (None, get) — Whether the symbol should be scale dependent
- `PrimaryPointNumber` (None, get) — Primary point number
- `RotationAngle` (None, get) — Rotation angle
- `SecondaryPointNumber` (None, get) — Secondary point number
- `SymbolID` (None, get) — Symbol ID
- `Width` (None, get) — Symbol width

## Symbol3DElement (class)

Representation of the terrain point in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/)

### Constructors
- `Symbol3DElement() | Symbol3DElement( commonProp: CommonProperties, Symbol3DProp: Symbol3DProperties, geometryObject: object, )` — Initialize

### Methods
#### `GetSymbol3DProperties() -> Symbol3DProperties`

Get the Symbol3D properties

**Remarks:** Get the Symbol3D properties

**Returns:** `Symbol3DProperties` — Symbol3D properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/#NemAll_Python_BasisElements.Symbol3DElement.GetSymbol3DProperties)

#### `SetSymbol3DProperties(symbol3DProp: Symbol3DProperties)`

Set the Symbol3D properties

**Remarks:** Set the Symbol3D properties

**Parameters:**
- `symbol3DProp` (Symbol3DProperties) — Symbol3D properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/#NemAll_Python_BasisElements.Symbol3DElement.SetSymbol3DProperties)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DElement/#NemAll_Python_BasisElements.Symbol3DElement.__repr__)

## Symbol3DProperties (class)

Properties of the terrain point

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DProperties/)

### Constructors
- `Symbol3DProperties()` — Initialize

### Methods
#### `__eq__(prop: Symbol3DProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (Symbol3DProperties) — Symbol3DProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DProperties/#NemAll_Python_BasisElements.Symbol3DProperties.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/Symbol3DProperties/#NemAll_Python_BasisElements.Symbol3DProperties.__repr__)

### Properties
- `ControlPointOffset` (None, get) — Control point offset
- `DescriptionText` (None, get) — Description text (label)
- `Height` (None, get) — Symbol height
- `IsScaleDependent` (None, get) — Whether the point is to be scale dependent
- `IsStation` (None, get) — Whether the point is a station
- `PrimaryPointNumber` (None, get) — Primary point number
- `RotationAngle` (None, get) — Rotation angle
- `SecondaryPointNumber` (None, get) — Secondary point number
- `StationCode` (None, get) — Point code or station depending on the value of IsStation property.
- `SymbolID` (None, get) — Symbol ID
- `Width` (None, get) — Symbol width

## TextAlignment (enum)

Types of text alignments

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextAlignment/)

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

Representation of text in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/)

### Constructors
- `TextElement() | TextElement( commonProp: CommonProperties, textProp: TextProperties, text: str, textPnt: Point2D, ) | TextElement(element: TextElement)` — Initialize

### Methods
#### `GetDimensions(doc: DocumentAdapter) -> Vector2D`

Get the dimensions of the text

**Remarks:** Get the dimensions of the text

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `Vector2D` — Text dimensions

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetDimensions)

#### `GetText() -> str`

Get the text string

**Remarks:** Get the text string

**Returns:** `str` — Text string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetText)

#### `GetTextPoints(doc: DocumentAdapter, refPnt: Point2D) -> Point2DList`

Get the list of text points composed as follows:

**Remarks:** Get the list of text points composed as follows:

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `refPnt` (Point2D) — Reference point

**Returns:** `Point2DList` — Text points

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetTextPoints)

#### `GetTextProperties() -> TextProperties`

Get the Text properties

**Remarks:** Get the Text properties

**Returns:** `TextProperties` — Text properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.GetTextProperties)

#### `SetText(text: str)`

Set the text string

**Remarks:** Set the text string

**Parameters:**
- `text` (str) — Text string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.SetText)

#### `SetTextProperties(textProp: TextProperties)`

Set the Text properties

**Remarks:** Set the Text properties

**Parameters:**
- `textProp` (TextProperties) — Text properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.SetTextProperties)

#### `__eq__(textEle: TextElement) -> bool`

Comparison operator

**Remarks:** Comparison operator

**Parameters:**
- `textEle` (TextElement) — Text element to compare

**Returns:** `bool` — Comparison state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElement/#NemAll_Python_BasisElements.TextElement.__repr__)

### Properties
- `Text` (str, get/set) — Representation of text in Allplan
- `TextProperties` (TextProperties, get/set) — Get the Text properties

## TextElementList (class)

List for TextElement objects

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/)

### Constructors
- `TextElementList() | TextElementList(ele: TextElement) | TextElementList(eleList: list)` — Initialize

### Methods
#### `__contains__(value: TextElement) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (TextElement) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__contains__)

#### `__delitem__(value: TextElement)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (TextElement) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__delitem__)

#### `__eq__(compare_list: TextElementList) -> bool`

Compare two lists

**Remarks:** Compare two lists

**Parameters:**
- `compare_list` (TextElementList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__eq__)

#### `__getitem__(index: int) -> TextElement`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `TextElement` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__getitem__)

#### `__iadd__(eleList: list) -> TextElementList`

Add a list

**Remarks:** Add a list

**Parameters:**
- `eleList` (list) — TextElement list

**Returns:** `TextElementList` — Lists with the added elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__iadd__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__len__)

#### `__repr__() -> str`

Create a string from the elements of the list

**Remarks:** Create a string from the elements of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__repr__)

#### `__setitem__(index: int | slice, value: TextElement)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (TextElement) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.__setitem__)

#### `append(value: TextElement)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (TextElement) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.append)

#### `extend(iterable: TextElementList) | extend(eleList: list)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (TextElementList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextElementList/#NemAll_Python_BasisElements.TextElementList.extend)

## TextLocation (enum)

Location of the dimension text relative to the dimension line

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextLocation/)

### Values
- `eBASIS_DIM_BOTTOM_CENTER` = `-2` — Below the line, in the middle
- `eBASIS_DIM_BOTTOM_LEFT` = `-1` — Below the line, at the start
- `eBASIS_DIM_BOTTOM_RIGHT` = `-3` — Below the line, at the end
- `eBASIS_DIM_CENTER` = `0` — On the line, in the middle
- `eBASIS_DIM_CENTER_LEFT` = `-4` — On the line, at the start
- `eBASIS_DIM_CENTER_RIGHT` = `-5` — On the line, at the end
- `eBASIS_DIM_NONE` = `10` — Undefined
- `eBASIS_DIM_TOP_CENTER` = `2` — Above the line, in the middle
- `eBASIS_DIM_TOP_LEFT` = `1` — Above the line, at the start
- `eBASIS_DIM_TOP_RIGHT` = `3` — Above the line, at the end

## TextProperties (class)

Implementation of the text properties

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/)

### Constructors
- `TextProperties() | TextProperties(element: TextProperties)` — Initialize

### Methods
#### `GetRatio() -> float`

Get text height/width ratio

**Remarks:** Get text height/width ratio

**Returns:** `float` — height/width ratio

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.GetRatio)

#### `Init()`

Init section properties with default values

**Remarks:** Init section properties with default values

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.Init)

#### `IsDraftText() -> bool`

Check if the text is a draft text (unprintable)

**Remarks:** Check if the text is a draft text (unprintable)

**Returns:** `bool` — True if text is unprintable, otherwise False

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.IsDraftText)

#### `SetHasBackgroundColorAndTransparentBackgroundColor(value: bool)`

Setting for filling the text background with the color of the viewport

**Remarks:** Setting for filling the text background with the color of the viewport

**Parameters:**
- `value` (bool) — Set to to True to turn the background filling on.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.SetHasBackgroundColorAndTransparentBackgroundColor)

#### `__eq__(prop: TextProperties) -> bool`

equal operator

**Remarks:** equal operator

**Parameters:**
- `prop` (TextProperties) — TextProperties to compare

**Returns:** `bool` — true if they are equal, false otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextProperties/#NemAll_Python_BasisElements.TextProperties.__repr__)

### Properties
- `Alignment` (TextAlignment, get/set) — Text alignment
- `BackgroundColor` (int, get/set) — Background color of the text
- `ColumnSlopeAngle` (Angle, get/set) — Slope angle of columns in [rad]
- `Expansion` (int, get/set) — Text expansion
- `Font` (int, get/set) — Font ID
- `FontAngle` (Angle, get/set) — Angle of characters (italic) in [rad]
- `FontStyles` (int, get/set) — Font style (Bold, Italic, Underline, Crossed out) via bits, where:
- `HasBackgroundColor` (bool, get/set) — Whether the text has a background color
- `HasReferencePoint` (bool, get/set) — Whether the text has a reference point
- `HasTextFrame` (bool, get/set) — Whether to draw border around text
- `HasTransparentBackground` (bool, get/set) — Whether the text background is to be transparent
- `Height` (float, get/set) — Height of text in [mm]
- `IsScaleDependent` (bool, get/set) — Whether the text is to be scale dependent
- `IsUserModifiable` (bool, get/set) — Whether the text should be modifiable by the user
- `LineFeed` (float, get/set) — Line feed of text in [mm]
- `TextAngle` (Angle, get/set) — Angle of whole text in [rad]
- `TextFrameColor` (int, get/set) — Color ID of the text frame color
- `TextFramePen` (int, get/set) — Pen ID of the text frame
- `TextFrameStroke` (int, get/set) — Stroke ID of the text frame
- `Type` (TextType, get/set) — Text type (0 - normal text, >1 variable text)
- `Width` (float, get/set) — Text width in [mm]
- `WrappedText` (bool, get/set) — Whether to use text wrapping.
- `WrappingWidth` (float, get/set) — Wrapping width. Used when wrapping is active

## TextType (enum)

Types of text element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextType/)

### Values
- `eFormularText` = `1`
- `eNormalText` = `0`
- `eVariableText` = `2`

## TextureDefinition (class)

Representation of the texture definition

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureDefinition/)

### Constructors
- `TextureDefinition() | TextureDefinition(surfacePath: str)` — Initialize

### Methods
#### `__eq__(props: TextureDefinition) -> bool`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (TextureDefinition) — Properties to compare

**Returns:** `bool` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureDefinition/#NemAll_Python_BasisElements.TextureDefinition.__eq__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureDefinition/#NemAll_Python_BasisElements.TextureDefinition.__repr__)

### Properties
- `SurfacePath` (None, get) — Property for surface path :type: None

## TextureMapping (class)

Texture mapping property class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/)

### Constructors
- `TextureMapping() | TextureMapping( mappingType: TextureMappingType, mappingAngle: float, scaleX: float, scaleY: float, offsetX: float, offsetY: float, phongAngle: float, refFace: int, refEdge: int, ) | TextureMapping( mappingType: TextureMappingType, mappingAngle: float, scaleX: float, scaleY: float, offsetX: float, offsetY: float, phongAngle: float, refFace: int, refEdge: int, uvCoords: VecDoubleList, ) | TextureMapping(uvCoords: VecDoubleList) | TextureMapping(element: TextureMapping)` — Initialize

### Methods
#### `FromSurfaceMapping(mapping: object)`

Get the properties from the surface mapping object

**Remarks:** Get the properties from the surface mapping object

**Parameters:**
- `mapping` (object) — Surface mapping

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/#NemAll_Python_BasisElements.TextureMapping.FromSurfaceMapping)

#### `__eq__(props: TextureMapping) -> bool`

Compare operator

**Remarks:** Compare operator

**Parameters:**
- `props` (TextureMapping) — Properties to compare

**Returns:** `bool` — Properties a equal: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/#NemAll_Python_BasisElements.TextureMapping.__eq__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMapping/#NemAll_Python_BasisElements.TextureMapping.__repr__)

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

Types of texture mapping

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TextureMappingType/)

### Values
- `eCube` = `0` — From each side
- `eCylinder` = `4` — Cylindrical mapping
- `eGround` = `3` — Only from the top view
- `eRoof` = `2` — Mainly from the top view
- `eSphere` = `5` — Spherical mapping
- `eUV` = `6` — UV mapping
- `eWall` = `1` — Mainly from the front view

## TransitionType (enum)

Transition types of the color gradients of filling

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/TransitionType/)

### Values
- `eNoTransition` = `0` — Without gradients
- `eOneColorTransition` = `1` — Single-colored gradients
- `eTwoColorTransition` = `2` — Two-colored gradients

## VariantType (enum)

Variant types of the filling color gradient

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/VariantType/)

### Values
- `eVariant1` = `0` — Color1 to Color2
- `eVariant2` = `1` — Color2 to Color1
- `eVariant3` = `2` — Color2 to Color1 to Color2
- `eVariant4` = `3` — Color1 to Color2 to Color1

## ViewSectionElement (class)

Representation of a Unified View and Section (UVS) in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/)

### Constructors
- `ViewSectionElement() | ViewSectionElement(param: ViewSectionElement)` — Initialize

### Methods
#### `CreateSectionBody( doc: DocumentAdapter, insertionMat: Matrix3D, undoRedoService: object | None = None, )`

Create the section body

**Remarks:** Create the section body

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `insertionMat` (Matrix3D) — Placement matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/#NemAll_Python_BasisElements.ViewSectionElement.CreateSectionBody)

#### `DrawElement( modelElements: object, insertionMat: Matrix3D, docID: int, asStaticPreview: bool, ) -> MinMax3D`

Draw eLement preview in Allplan

**Remarks:** Draw eLement preview in Allplan

**Parameters:**
- `modelElements` (object) — Python object which should be drawn
- `insertionMat` (Matrix3D) — insertion matrix
- `docID` (int) — Document ID

**Returns:** `MinMax3D` — Min/max box of the preview

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/#NemAll_Python_BasisElements.ViewSectionElement.DrawElement)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/ViewSectionElement/#NemAll_Python_BasisElements.ViewSectionElement.__repr__)

### Properties
- `DimensionElements` (list, get/set) — Dimension elements
- `GeneralSectionProperties` (SectionGeneralProperties, get/set) — General section properties
- `ReinforcementLabels` (ReinforcementLabel, get/set) — Reinforcement labels
- `SectionDefinitionData` (SectionDefinitionData, get/set) — Section definition data
- `TextElements` (list, get/set) — Labels
- `ViewMatrix` (Matrix3D, get/set) — View transformation matrix

## VisibleHiddenEdgesProperties (class)

Properties of the visible and hidden edges in a UVS

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BasisElements/VisibleHiddenEdgesProperties/)

### Constructors
- `VisibleHiddenEdgesProperties()` — Initialize

### Properties
- `HiddenEdgesColor` (None, get) — Color ID of hidden edges
- `HiddenEdgesLayer` (None, get) — layer ID for hidden edges
- `HiddenEdgesLineType` (None, get) — stroke ID for hidden edges
- `HiddenEdgesPen` (None, get) — pen ID for hidden edges
- `IsHiddenEdgesColorFromLayer` (None, get) — Whether to get the color ID of the hidden edges from the layer definition
- `IsHiddenEdgesColorUsed` (None, get) — Whether to use the color specified in HiddenEdgesColor for drawing the hidden edges (True) or the color of the model elements (False)
- `IsHiddenEdgesLayerUsed` (None, get) — Whether to use the layer specified in HiddenEdgesLayer for drawing the hidden edges (True) or the layer of the model elements (False)
- `IsHiddenEdgesLineTypeFromLayer` (None, get) — Whether to get the stroke ID of the hidden edges from the layer definition
- `IsHiddenEdgesLineTypeUsed` (None, get) — Whether to use the stroke specified in HiddenEdgesLineType for drawing the hidden edges (True) or the stroke of the model elements (False)
- `IsHiddenEdgesOn` (None, get) — Whether to draw hidden edges
- `IsHiddenEdgesPenFromLayer` (None, get) — Whether to get the pen ID of the hidden edges from the layer definition
- `IsHiddenEdgesPenUsed` (None, get) — Whether to use the pen specified in HiddenEdgesPen for drawing the hidden edges (True) or the pen of the model elements (False)
- `IsVisibleEdgesColorFromLayer` (None, get) — Whether to get the color ID of the visible edges from the layer definition
- `IsVisibleEdgesColorUsed` (None, get) — Whether to use the color specified in VisibleEdgesColor for drawing the visible edges (True) or the color of the model elements (False)
- `IsVisibleEdgesLayerUsed` (None, get) — Whether to use the layer specified in VisibleEdgesLayer for drawing the visible edges (True) or the layer of the model elements (False)
- `IsVisibleEdgesLineTypeFromLayer` (None, get) — Whether to get the stroke ID of the visible edges from the layer definition
- `IsVisibleEdgesLineTypeUsed` (None, get) — Whether to use the stroke specified in VisibleEdgesLineType for drawing the visible edges (True) or the stroke of the model elements (False)
- `IsVisibleEdgesOn` (None, get) — Whether to draw visible edges
- `IsVisibleEdgesPenFromLayer` (None, get) — Whether to get the pen ID of the visible edges from the layer definition
- `IsVisibleEdgesPenUsed` (None, get) — Whether to use the pen specified in VisibleEdgesPen for drawing the visible edges (True) or the pen of the model elements (False)
- `VisibleEdgesColor` (None, get) — Color ID for visible edges
- `VisibleEdgesLayer` (None, get) — Layer ID for visible edges
- `VisibleEdgesLineType` (None, get) — Stroke ID for visible edges
- `VisibleEdgesPen` (None, get) — Pen ID for visible edges

