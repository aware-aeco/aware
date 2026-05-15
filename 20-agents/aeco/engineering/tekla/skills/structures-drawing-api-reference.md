---
parent: tekla-structures-drawing
description: Full API reference for Tekla.Structures.Drawing classes, views, marks, dimensions, symbols, text, automation.
---

## Core Classes (Tier 1)

### DrawingHandler
> The DrawingHandler class initializes the interface from a .NET application to Tekla Structures. This object must be created before any actions can be performed on Tekla Structures drawings.

**Constructors:** `DrawingHandler()`

**Methods:**
- `Drawing GetActiveDrawing()` - returns the currently open drawing (null if none)
- `bool SetActiveDrawing(Drawing drawing)` - opens a drawing
- `bool SetActiveDrawing(Drawing drawing, bool showDrawing)` - opens a drawing, optionally hidden
- `bool SetActiveDrawing(Drawing drawing, bool showDrawing, bool forceOpen)` - opens with force option
- `bool CloseActiveDrawing()` - closes the active drawing without saving
- `bool CloseActiveDrawing(bool save)` - closes with save option
- `bool SaveActiveDrawing()` - saves the active drawing
- `bool IsAnyDrawingOpen()` - checks if any drawing is currently open
- `bool GetConnectionStatus()` - checks connection to Tekla Structures
- `DrawingEnumerator GetDrawings()` - gets all drawings in the model
- `DrawingObjectSelector GetDrawingObjectSelector()` - gets the drawing object selector (UI)
- `DrawingSelector GetDrawingSelector()` - gets the drawing list selector
- `Picker GetPicker()` - gets the picker for interactive point/object selection
- `List<Identifier> GetModelObjectIdentifiers(Drawing drawing)` - gets model object IDs in a drawing
- `bool PrintDrawing(Drawing drawing, DPMPrinterAttributes printAttributes)` - prints a drawing
- `bool PrintDrawing(Drawing drawing, DPMPrinterAttributes printAttributes, string outputFile)` - prints to file
- `bool PrintDrawings(List<Drawing> drawings, DPMPrinterAttributes printAttributes)` - batch print
- `bool PrintDrawings(IEnumerable<Drawing> drawings, DPMPrinterAttributes printAttributes, string OutputFolder)` - batch print to folder
- `bool UpdateDrawing(Drawing drawing)` - updates a drawing
- `bool IssueDrawing(Drawing drawing)` - issues a drawing
- `bool UnissueDrawing(Drawing drawing)` - unissues a drawing
- `static MessageExecutionModeEnum SetMessageExecutionStatus(MessageExecutionModeEnum mode)` - sets message execution mode
- `static MessageExecutionModeEnum GetMessageExecutionStatus()` - gets current message execution mode

```csharp
// Enum: DrawingHandler.MessageExecutionModeEnum
INSTANT = 0    // Executes messages after each database operation (default)
BY_COMMIT = 1  // Executes messages only when Drawing.CommitChanges() is called
```

---

### Drawing : DatabaseObject
> Base class for all drawing types. Cannot be instantiated directly.
>
> **Identity:** `Drawing` has NO `Identifier`, `Id`, or `GUID` property (verified via reflection on Tekla 2025). The inheritance chain is `Drawing → Tekla.Structures.Drawing.DatabaseObject → System.Object` and neither base class exposes a database identifier. To locate a drawing at runtime, iterate `drawingHandler.GetDrawings()` and match by `Name` or `Mark`. For the underlying MODEL object, use subclass properties (`AssemblyIdentifier`, `PartIdentifier`, `CastUnitIdentifier`) — those are `Tekla.Structures.Identifier` values pointing to the model, not the drawing.

**Properties:**
- `string Name { get; set; }` - the name of the drawing
- `string Mark { get; set; }` - the drawing mark
- `string Title1 { get; set; }` - first drawing title
- `string Title2 { get; set; }` - second drawing title
- `string Title3 { get; set; }` - third drawing title
- `DateTime CreationDate { get; set; }` - creation date
- `DateTime ModificationDate { get; set; }` - modification date
- `DateTime IssuingDate { get; set; }` - issuing date
- `DateTime OutputDate { get; set; }` - output date
- `DrawingUpToDateStatus UpToDateStatus { get; set; }` - up-to-date status
- `bool IsFrozen { get; set; }` - whether the drawing is frozen
- `bool IsIssued { get; set; }` - whether the drawing is issued
- `bool IsLocked { get; set; }` - whether the drawing is locked
- `string IsLockedBy { get; set; }` - username who locked
- `bool IsReadyForIssue { get; set; }` - ready for issue flag
- `bool IsMasterDrawing { get; set; }` - master drawing flag
- `LayoutAttributes Layout { get; set; }` - drawing layout attributes
- `string CommitMessage { get; set; }` - commit message
- `string DrawingTypeStr { get; }` - type of drawing as string

**Methods:**
- `bool CommitChanges()` - commits pending changes
- `bool CommitChanges(string Message)` - commits with message
- `ContainerView GetSheet()` - gets the drawing sheet (root container view)
- `bool PlaceViews()` - auto-places views on the sheet
- `string GetPlotFileName(bool includeRevisionInfo)` - gets plot file name
- `static IList<Identifier> GetIdsOfUnnumberedDrawings(IList<Drawing> drawingsToBeChecked)` - finds unnumbered drawings

---

### AssemblyDrawing : Drawing
> Handles assembly drawings.

**Constructors:** `AssemblyDrawing(Identifier assemblyIdentifier)`, `AssemblyDrawing(Identifier assemblyIdentifier, string AttributeFile)`, `AssemblyDrawing(Identifier assemblyIdentifier, int sheetNumber)`, `AssemblyDrawing(Identifier assemblyIdentifier, int sheetNumber, string AttributeFile)`

**Properties:**
- `Identifier AssemblyIdentifier { get; }` - the model assembly identifier
- `int SheetNumber { get; }` - the sheet number

---

### SinglePartDrawing : Drawing
> Handles single part drawings.

**Constructors:** `SinglePartDrawing(Identifier partIdentifier)`, `SinglePartDrawing(Identifier partIdentifier, string AttributeFile)`, `SinglePartDrawing(Identifier partIdentifier, int sheetNumber)`, `SinglePartDrawing(Identifier partIdentifier, int sheetNumber, string AttributeFile)`

**Properties:**
- `Identifier PartIdentifier { get; }` - the model part identifier
- `int SheetNumber { get; }` - the sheet number

---

### GADrawing : Drawing
> Handles general arrangement drawings.

**Constructors:** `GADrawing()`, `GADrawing(string AttributeFile)`, `GADrawing(string Name, string AttributeFile)`, `GADrawing(string AttributeFile, Size SheetSize)`

---

### CastUnitDrawing : Drawing
> Handles cast unit drawings.

**Constructors:** `CastUnitDrawing(Identifier castUnitIdentifier)`, `CastUnitDrawing(Identifier castUnitIdentifier, string AttributeFile)`, `CastUnitDrawing(Identifier castUnitIdentifier, CastUnitDrawingCreationType castUnitDrawingCreationType)`, plus overloads with sheetNumber

**Properties:**
- `Identifier CastUnitIdentifier { get; }` - cast unit identifier
- `bool CastUnitById { get; }` - true = by ID, false = by position
- `int SheetNumber { get; }` - sheet number

```csharp
// Enum: CastUnitDrawing.CastUnitDrawingCreationType
CastUnitDrawingByPosition = 0
CastUnitDrawingById = 1
```

---

### MultiDrawing : Drawing
> Handles multidrawings. No additional constructors or properties.

---

### ContainerView : ViewBase
> Contains other drawing objects and views. The drawing sheet is a container view.

**Methods:**
- `DrawingObjectEnumerator GetAllViews()` - gets all views (recursive)
- `DrawingObjectEnumerator GetViews()` - gets direct child views

---

### View : ViewBase
> Contains drawing objects and model object representations.

**Constructors:**
- `View(ContainerView View, CoordinateSystem ViewCoordinateSystem, CoordinateSystem DisplayCoordinateSystem, AABB RestrictionBox)`
- `View(ContainerView View, CoordinateSystem ViewCoordinateSystem, CoordinateSystem DisplayCoordinateSystem, ArrayList PartList)`
- Overloads with `string Attributes` parameter

**Properties:**
- `ViewAttributes Attributes { get; set; }` - view attributes
- `CoordinateSystem ViewCoordinateSystem { get; set; }` - view plane coordinate system
- `CoordinateSystem DisplayCoordinateSystem { get; set; }` - display plane coordinate system
- `string Name { get; set; }` - view name
- `AABB RestrictionBox { get; set; }` - view volume bounding box
- `ViewTypes ViewType { get; set; }` - view type

**Methods:**
- `DrawingObjectEnumerator GetModelObjects()` - gets model objects in the view
- `bool RotateViewOnAxisX(double rotationAngle)` - rotates view on X axis
- `bool RotateViewOnAxisY(double rotationAngle)` - rotates view on Y axis
- `bool RotateViewOnAxisZ(double rotationAngle)` - rotates view on Z axis
- `bool RotateViewOnDrawingPlane(double rotationAngle)` - rotates on drawing plane

**Static Factory Methods:**
- `static bool CreateFrontView(Drawing, Point, ViewAttributes, out View)` - creates front view
- `static bool CreateTopView(Drawing, Point, ViewAttributes, out View)` - creates top view
- `static bool CreateBackView(Drawing, Point, ViewAttributes, out View)` - creates back view
- `static bool CreateBottomView(Drawing, Point, ViewAttributes, out View)` - creates bottom view
- `static bool Create3dView(Drawing, Point, ViewAttributes, out View)` - creates 3D view
- `static bool Create3dView(Drawing, CoordinateSystem, CoordinateSystem, AABB, Point, ViewAttributes, out View)` - creates 3D view with planes
- `static bool CreateSectionView(View, Point startPoint, Point endPoint, Point viewInsertionPoint, double depthUp, double depthDown, ViewAttributes, SectionMarkAttributes, out View sectionView, out SectionMark)` - creates section view
- `static bool CreateCurvedSectionView(View, Point start, Point middle, Point end, Point viewInsertionPoint, double depthUp, double depthDown, ViewAttributes, SectionMarkAttributes, out View, out CurvedSectionMark)` - creates curved section view
- `static bool CreateDetailView(View, Point centerPoint, Point boundaryPoint, Point labelPoint, Point viewInsertionPoint, ViewAttributes, DetailMarkAttributes, out View, out DetailMark)` - creates detail view

#### View.ViewAttributes : AttributesBase

**Properties:**
- `double Scale { get; set; }` - view scale
- `bool FixedViewPlacing { get; set; }` - fixed vs free placing
- `bool ReflectedView { get; set; }` - reflected view
- `bool UnfoldedView { get; set; }` - unfolded view
- `bool UndeformedView { get; set; }` - undeformed view
- `bool PourView { get; set; }` - pour view
- `double DatumLevel { get; set; }` - datum level
- `bool ViewPlaneDatumPointForElevations { get; set; }` - datum point mode
- `HorizontalLabelPosition LabelPositionHorizontal { get; set; }` - horizontal label position
- `VerticalLabelPosition LabelPositionVertical { get; set; }` - vertical label position
- `ViewMarkSymbolAttributes MarkSymbolAttributes { get; set; }` - mark symbol attributes
- `ViewMarkTagsAttributes TagsAttributes { get; set; }` - tag attributes
- `ViewShorteningAttributes Shortening { get; set; }` - shortening attributes
- `double ViewExtensionForNeighbourParts { get; set; }` - neighbour part extension
- `bool ShowPartOpeningsOrRecessSymbol { get; set; }` - show openings symbol
- `string LocationBy { get; set; }` - location reference
- `double PartialProfileLength { get; set; }` - partial profile length
- `double PartialProfileOffset { get; set; }` - partial profile offset

---

### ViewBase : DrawingObject
> Base class for ContainerView and View.

**Properties:**
- `Point Origin { get; set; }` - view origin in sheet coordinates
- `Vector FrameOrigin { get; set; }` - vector from view origin to frame origin
- `double Width { get; set; }` - view frame width (paper coords)
- `double Height { get; set; }` - view frame height (paper coords)
- `Point ExtremaCenter { get; set; }` - view extrema center
- `bool IsSheet { get; set; }` - whether this is the drawing sheet

**Methods:**
- `DrawingObjectEnumerator GetObjects()` - gets direct child objects
- `DrawingObjectEnumerator GetObjects(Type[] TypeFilter)` - gets filtered child objects
- `DrawingObjectEnumerator GetAllObjects()` - gets all objects (recursive)
- `DrawingObjectEnumerator GetAllObjects(Type Type)` - gets all objects of one type
- `DrawingObjectEnumerator GetAllObjects(Type[] TypeFilter)` - gets all objects matching filter
- `DrawingObjectEnumerator GetModelObjects(Identifier ModelIdentifier)` - gets drawing objects for a model object
- `RectangleBoundingBox GetAxisAlignedBoundingBox()` - gets bounding box
- `Drawing GetOriginalDrawing()` - gets the drawing this view belongs to

---

### DrawingObject : DatabaseObject
> Abstract base class for all objects in a drawing.

**Properties:**
- `AttributesBase Attributes { get; set; }` - object attributes

**Methods:**
- `Drawing GetDrawing()` - gets the parent drawing
- `ViewBase GetView()` - gets the parent view
- `DrawingObjectEnumerator GetRelatedObjects()` - gets related objects (e.g., leader lines)
- `DrawingObjectEnumerator GetRelatedObjects(Type[] typeFilter)` - gets filtered related objects

---

### DatabaseObject
> Base class for all drawing objects (`Tekla.Structures.Drawing.DatabaseObject`). Provides user property access and CRUD operations.
>
> **Important:** This type has ONLY `QueryReturnValue` as a public property. It does NOT expose `Identifier`, `Id`, or `GUID`. Do NOT confuse this with `Tekla.Structures.Model.ModelObject`, which DOES have `.Identifier`. Drawing-side objects (drawings, views, marks, dimensions, drawing objects) are identified by their business properties — there is no drawing-DB identifier on this base class.

**Properties:**
- `ReturnValuesEnum QueryReturnValue { get; set; }` - status of last database operation

**Methods:**
- `bool Insert()` - inserts the object into the database
- `bool Modify()` - modifies the object in the database
- `bool Delete()` - deletes the object from the database
- `bool Select()` - selects (fetches) the object from the database
- `bool GetUserProperty(string Name, ref string Value)` - gets string UDA
- `bool GetUserProperty(string Name, ref int Value)` - gets int UDA
- `bool GetUserProperty(string Name, ref double Value)` - gets double UDA
- `bool SetUserProperty(string Name, string Value)` - sets string UDA
- `bool SetUserProperty(string Name, int Value)` - sets int UDA
- `bool SetUserProperty(string Name, double Value)` - sets double UDA
- `bool GetStringUserProperties(List<string> names, out Dictionary<string, string> values)` - batch get string UDAs
- `bool GetIntegerUserProperties(List<string> names, out Dictionary<string, int> values)` - batch get int UDAs
- `bool GetDoubleUserProperties(List<string> names, out Dictionary<string, double> values)` - batch get double UDAs
- `bool IsSameDatabaseObject(DatabaseObject Object)` - compares database identity

---

### Part : ModelObject
> Drawing representation of a model part.

**Properties:**
- `PartAttributes Attributes { get; set; }` - part drawing attributes

#### Part.PartAttributes : AttributesBase

**Properties:**
- `Part.Representation Representation { get; set; }` - part representation type
- `bool DrawHiddenLines { get; set; }` - show hidden lines
- `bool DrawOwnHiddenLines { get; set; }` - show own hidden lines
- `bool DrawCenterLine { get; set; }` - show center line
- `bool DrawReferenceLine { get; set; }` - show reference line
- `bool DrawChamfers { get; set; }` - show chamfers
- `bool DrawConnectingSideMarks { get; set; }` - show connecting side marks
- `bool DrawOrientationMark { get; set; }` - show orientation mark
- `bool DrawPopMarks { get; set; }` - show pop marks
- `LineTypeAttributes VisibleLines { get; set; }` - visible line style
- `LineTypeAttributes HiddenLines { get; set; }` - hidden line style
- `LineTypeAttributes OwnHiddenLines { get; set; }` - own hidden line style
- `LineTypeAttributes CenterLine { get; set; }` - center line style
- `LineTypeAttributes ReferenceLine { get; set; }` - reference line style
- `LineTypeAttributes SectionLines { get; set; }` - section line style
- `ModelObjectHatchAttributes FaceHatch { get; set; }` - face hatch
- `ModelObjectHatchAttributes SectionFaceHatch { get; set; }` - section face hatch
- `string CustomPresentation { get; set; }` - custom presentation GUID
- `double PartialProfileLength { get; set; }` - partial profile length
- `double PartialProfileOffset { get; set; }` - partial profile offset
- `double SymbolOffset { get; set; }` - symbol offset

---

### Bolt : ModelObject
> Drawing representation of a model bolt.

**Properties:**
- `BoltAttributes Attributes { get; set; }` - bolt drawing attributes

#### Bolt.BoltAttributes : AttributesBase

**Properties:**
- `DrawingColors Color { get; set; }` - bolt color
- `TeklaDrawingColor TrueColor { get; set; }` - true color (RGB)
- `Bolt.Representation Representation { get; set; }` - bolt representation
- `bool SymbolContainsAxis { get; set; }` - show axis in symbol
- `bool SymbolContainsHole { get; set; }` - show hole in symbol
- `string CustomPresentation { get; set; }` - custom presentation GUID

---

### ModelObject : DrawingObject
> Abstract base class for all model objects on the drawing side.

**Properties:**
- `Hideable Hideable { get; set; }` - hidden state control
- `Identifier ModelIdentifier { get; set; }` - model database identifier (links to Tekla.Structures.Model)

---

### Mark : MarkBase
> A mark object containing a single mark. Multiple marks can merge into a MarkSet.

**Constructors:** `Mark(ModelObject modelObject)`

**Properties:**
- `MarkAttributes Attributes { get; set; }` - mark attributes

#### Mark.MarkAttributes : MarkBaseAttributes

**Properties:**
- `ContainerElement Content { get; set; }` - mark content as list of elements

---

### MarkBase : DrawingObject
> Base class for marks. Contains general mark information.

**Properties:**
- `MarkBaseAttributes Attributes { get; set; }` - mark base attributes
- `Hideable Hideable { get; set; }` - hidden state
- `Point InsertionPoint { get; set; }` - insertion point
- `PlacingBase Placing { get; set; }` - placing type
- `bool IsAssociativeNote { get; set; }` - whether it is an associative note

**Methods:**
- `RectangleBoundingBox GetAxisAlignedBoundingBox()` - axis-aligned bounding box
- `RectangleBoundingBox GetObjectAlignedBoundingBox()` - object-aligned bounding box
- `DrawingObjectEnumerator GetObjects()` - gets child objects (e.g., leader lines)
- `bool MoveObjectRelative(Vector MoveVector)` - moves the mark

#### MarkBase.MarkBaseAttributes : AttributesBase

**Properties:**
- `double Angle { get; set; }` - actual angle in degrees
- `double RotationAngle { get; set; }` - user-set rotation angle in degrees
- `ArrowheadAttributes ArrowHead { get; set; }` - leader line arrowhead
- `Frame Frame { get; set; }` - frame around mark labels
- `PlacingAttributes PlacingAttributes { get; set; }` - placing configuration
- `PreferredPlacingTypeBase PreferredPlacing { get; set; }` - preferred placing type
- `TextAlignOptions TextAlignment { get; set; }` - text alignment (Left=0, Center=1, Right=2)
- `bool TransparentBackground { get; set; }` - background transparency
- `string CustomPresentation { get; set; }` - custom presentation GUID

---

### Text : DrawingObject
> A text object displayed in a drawing.

**Constructors:** `Text(ViewBase view, Point insertionPoint, string text)`, `Text(ViewBase view, Point insertionPoint, string text, PlacingBase Placing)`, `Text(ViewBase view, Point insertionPoint, string Text, TextAttributes inAttributes)`, `Text(ViewBase view, Point insertionPoint, string Text, PlacingBase Placing, TextAttributes inAttributes)`

**Properties:**
- `TextAttributes Attributes { get; set; }` - text attributes
- `Hideable Hideable { get; set; }` - hidden state
- `Point InsertionPoint { get; set; }` - insertion point
- `PlacingBase Placing { get; set; }` - placing type
- `string TextString { get; set; }` - the text content

**Methods:**
- `RectangleBoundingBox GetAxisAlignedBoundingBox()` - bounding box
- `RectangleBoundingBox GetObjectAlignedBoundingBox()` - object-aligned bounding box
- `DrawingObjectEnumerator GetObjects()` - child objects (leader lines)
- `bool MoveObjectRelative(Vector MoveVector)` - moves the text

#### Text.TextAttributes : AttributesBase

**Properties:**
- `FontAttributes Font { get; set; }` - font settings
- `Frame Frame { get; set; }` - frame around text
- `double Angle { get; set; }` - text frame angle
- `TextAlignment Alignment { get; set; }` - alignment (Left, Center, Right, LeaderLine)
- `ArrowheadAttributes ArrowHead { get; set; }` - leader line arrowhead
- `PlacingAttributes PlacingAttributes { get; set; }` - placing config
- `PreferredPlacingTypeBase PreferredPlacing { get; set; }` - preferred placing type
- `bool TransparentBackground { get; set; }` - background transparency
- `bool UseWordWrapping { get; set; }` - word wrapping
- `double RulerWidth { get; set; }` - ruler width for word wrapping
- `string CustomPresentation { get; set; }` - custom presentation GUID

---

### StraightDimensionSet : DimensionSetBase
> A straight dimension set containing one or more dimensions.

**Properties:**
- `StraightDimensionSetAttributes Attributes { get; set; }` - dimension set attributes
- `double Distance { get; set; }` - distance from first point to dimension line (paper mm)
- `double LeftTagLineOffset { get; set; }` - left tag line offset (paper mm)
- `double RightTagLineOffset { get; set; }` - right tag line offset (paper mm)

**Static Methods:**
- `static List<string> GetAllExcludePartsAccordingToFilter()` - gets available filter names

#### StraightDimensionSetHandler

**Constructors:** `StraightDimensionSetHandler()`

**Methods:**
- `StraightDimensionSet CreateDimensionSet(ViewBase View, PointList DimensionPoints, Vector UpDirection, double Distance)` - creates a dimension set
- `StraightDimensionSet CreateDimensionSet(ViewBase View, PointList DimensionPoints, Vector UpDirection, double Distance, StraightDimensionSetAttributes Attributes)` - creates with attributes
- Overloads with `LeftTagLineOffset` and `RightTagLineOffset`

#### StraightDimensionSet.StraightDimensionSetAttributes : DimensionSetBaseAttributes

**Properties:**
- `DimensionTypes DimensionType { get; set; }` - dimension type (Relative, Absolute, etc.)
- `ExtensionLineTypes ExtensionLine { get; set; }` - extension line visibility
- `ShortDimensionTypes ShortDimension { get; set; }` - short dimension handling
- `CombinedDimensionAttributes CombinedDimension { get; set; }` - combining settings
- `DimensionExaggerationAttributes Exaggeration { get; set; }` - exaggeration settings
- `string ExcludePartsAccordingToFilter { get; set; }` - filter for excluding parts from tags
- `ContainerElement LeftUpperTag { get; set; }` - left upper tag content
- `ContainerElement LeftMiddleTag { get; set; }` - left middle tag content
- `ContainerElement LeftLowerTag { get; set; }` - left lower tag content
- `ContainerElement RightUpperTag { get; set; }` - right upper tag content
- `ContainerElement RightMiddleTag { get; set; }` - right middle tag content
- `ContainerElement RightLowerTag { get; set; }` - right lower tag content
- `bool IncludePartCountInTag { get; set; }` - part count in any tag
- `bool IncludePartCountInTagA..G { get; set; }` - part count per tag position

---

### StraightDimension : DimensionBase
> An individual straight dimension within a dimension set.

**Constructors:** `StraightDimension(ViewBase targetView, Point startPoint, Point endPoint, Vector upDirection, double distance)`, `StraightDimension(ViewBase targetView, Point startPoint, Point endPoint, Vector upDirection, double distance, StraightDimensionSetAttributes attributes)`

**Properties:**
- `StraightDimensionAttributes Attributes { get; set; }` - dimension attributes
- `Point StartPoint { get; set; }` - start point
- `Point EndPoint { get; set; }` - end point
- `Vector UpDirection { get; set; }` - direction to dimension line
- `double Distance { get; set; }` - distance from start point to dimension line
- `ContainerElement Value { get; set; }` - dimension mark content

**Methods:**
- `DimensionSetBase GetDimensionSet()` - gets parent dimension set
- `DimensionSetBase GetDimensionSet(bool Select)` - gets parent set, optionally selecting it

---

### Symbol : DrawingObject
> A symbol object displayed in a drawing. Symbols come from .sym files.

**Constructors:** `Symbol(ViewBase View, Point InsertionPoint)`, `Symbol(ViewBase View, Point InsertionPoint, SymbolInfo SymbolInfo)`, `Symbol(ViewBase View, Point InsertionPoint, SymbolInfo SymbolInfo, SymbolAttributes Attributes)`, plus overloads with PlacingBase

**Properties:**
- `SymbolAttributes Attributes { get; set; }` - symbol attributes
- `Hideable Hideable { get; set; }` - hidden state
- `Point InsertionPoint { get; set; }` - insertion point
- `PlacingBase Placing { get; set; }` - placing type
- `SymbolInfo SymbolInfo { get; set; }` - symbol file and index info

**Methods:**
- `RectangleBoundingBox GetAxisAlignedBoundingBox()` / `GetObjectAlignedBoundingBox()`
- `bool MoveObjectRelative(Vector MoveVector)` - moves the symbol

#### Symbol.SymbolAttributes : AttributesBase

**Properties:**
- `double Angle { get; set; }` - symbol angle
- `double Height { get; set; }` - symbol height
- `DrawingColors Color { get; set; }` - symbol color
- `TeklaDrawingColor TrueColor { get; set; }` - true color (RGB)
- `Frame Frame { get; set; }` - frame around symbol
- `PreferredPlacingTypeBase PreferredPlacing { get; set; }` - preferred placing type

#### SymbolInfo

**Constructors:** `SymbolInfo()`, `SymbolInfo(string SymbolFile, int SymbolIndex)`

**Properties:**
- `string SymbolFile { get; set; }` - symbol file name (e.g. "xsteel")
- `int SymbolIndex { get; set; }` - symbol index (0-255)

---

### Line : OpenGraphicObject
> A two-point line with optional bulge (curve).

**Constructors:** `Line(ViewBase view, Point startPoint, Point endPoint)`, `Line(ViewBase view, Point startPoint, Point endPoint, double bulge)`, `Line(ViewBase view, Point startPoint, Point endPoint, LineAttributes attributes)`, `Line(ViewBase view, Point startPoint, Point endPoint, double bulge, LineAttributes attributes)`

**Properties:**
- `LineAttributes Attributes { get; set; }` - line attributes
- `Point StartPoint { get; set; }` - start point
- `Point EndPoint { get; set; }` - end point
- `double Bulge { get; set; }` - curve ratio (height/width)

---

### Polyline : OpenGraphicObject
> A multipoint line with optional bulge.

**Constructors:** `Polyline(ViewBase view, PointList pointList)`, `Polyline(ViewBase view, PointList pointList, PolylineAttributes attributes)`

**Properties:**
- `PolylineAttributes Attributes { get; set; }` - polyline attributes
- `PointList Points { get; set; }` - point list
- `double Bulge { get; set; }` - uniform bulge
- `List<double> Bulges { get; set; }` - individual segment bulges

---

### Polygon : ClosedGraphicObject
> A closed multipoint line, optionally hatched.

**Constructors:** `Polygon(ViewBase view, PointList pointList)`, `Polygon(ViewBase view, PointList pointList, PolygonAttributes attributes)`

**Properties:**
- `PolygonAttributes Attributes { get; set; }` - polygon attributes
- `PointList Points { get; set; }` - point list
- `double Bulge { get; set; }` - uniform bulge
- `List<double> Bulges { get; set; }` - individual segment bulges

---

### Circle : ClosedGraphicObject
> A circle, optionally hatched.

**Constructors:** `Circle(ViewBase view, Point centerPoint, double radius)`, `Circle(ViewBase view, Point centerPoint, double radius, CircleAttributes attributes)`

**Properties:**
- `CircleAttributes Attributes { get; set; }` - circle attributes
- `Point CenterPoint { get; set; }` - center point
- `double Radius { get; set; }` - radius

---

### Arc : OpenGraphicObject
> A two-point arc with radius or three-point arc.

**Constructors:** `Arc(ViewBase, Point, Point, double radius)`, `Arc(ViewBase, Point, Point, Point centerPoint, ArcAttributes)`, plus overloads with attributes

**Properties:**
- `ArcAttributes Attributes { get; set; }` - arc attributes
- `Point StartPoint { get; set; }` - start point
- `Point EndPoint { get; set; }` - end point
- `double Radius { get; set; }` - radius

---

### Cloud : ClosedGraphicObject
> A cloud shape (polygon with bulged edges).

**Constructors:** `Cloud(ViewBase view, PointList pointList)`, `Cloud(ViewBase view, PointList pointList, CloudAttributes attributes)`

**Properties:**
- `CloudAttributes Attributes { get; set; }` - cloud attributes
- `PointList Points { get; set; }` - corner points
- `double Bulge { get; set; }` - bulge ratio

---

### Rectangle : ClosedGraphicObject
> A rectangle with four orthogonal sides.

**Constructors:** `Rectangle(ViewBase, Point startPoint, Point endPoint)`, `Rectangle(ViewBase, Point startPoint, double width, double height)`, plus overloads with attributes

**Properties:**
- `RectangleAttributes Attributes { get; set; }` - rectangle attributes
- `Point StartPoint { get; set; }` - start point (rotation pivot)
- `Point EndPoint { get; set; }` - end point
- `double Width { get; set; }` - width
- `double Height { get; set; }` - height
- `double Angle { get; set; }` - rotation angle

---

### SectionMark : SectionMarkBase
> Illustrates a straight section in a view.

**Constructors:** `SectionMark(View view, Point leftPoint, Point rightPoint)`, `SectionMark(View view, Point leftPoint, Point rightPoint, SectionMarkAttributes attributes)`

---

### SectionMarkBase : DrawingObject
> Base class for SectionMark and CurvedSectionMark.

**Properties:**
- `SectionMarkAttributes Attributes { get; set; }` - section mark attributes
- `Point LeftPoint { get; set; }` - left symbol point
- `Point RightPoint { get; set; }` - right symbol point

#### SectionMarkBase.SectionMarkAttributes : AttributesBase

**Properties:**
- `string MarkName { get; set; }` - section mark name
- `SectionMarkSymbol LeftSymbol { get; set; }` - left side symbol
- `SectionMarkSymbol RightSymbol { get; set; }` - right side symbol
- `double LineLength { get; set; }` - section mark line length
- `double LineWidth { get; set; }` - section mark line width
- `DrawingColors LineColor { get; set; }` - line color
- `DrawingColors SymbolColor { get; set; }` - symbol color
- `TeklaDrawingColor TrueLineColor { get; set; }` - true line color
- `TeklaDrawingColor TrueSymbolColor { get; set; }` - true symbol color
- `SectionMarkTagsAttributes TagsAttributes { get; set; }` - tag attributes
- `double LineLengthOffset { get; set; }` - line end offset
- `double LineWidthOffsetLeft { get; set; }` - left line start offset
- `double LineWidthOffsetRight { get; set; }` - right line start offset

---

### DetailMark : DrawingObject
> Illustrates a detail in a view.

**Constructors:** `DetailMark(View view, Point centerPoint, Point boundaryPoint, Point labelPoint)`, `DetailMark(View view, Point centerPoint, Point boundaryPoint, Point labelPoint, DetailMarkAttributes attributes)`

**Properties:**
- `DetailMarkAttributes Attributes { get; set; }` - detail mark attributes
- `Point CenterPoint { get; set; }` - center point
- `Point BoundaryPoint { get; set; }` - boundary point
- `Point LabelPoint { get; set; }` - label point

---

### WeldMark : DrawingObject
> A welding mark in a drawing view.

**Properties:**
- `WeldMarkAttributes Attributes { get; set; }` - weld mark attributes
- `Hideable Hideable { get; set; }` - hidden state
- `Point InsertionPoint { get; set; }` - insertion point
- `Identifier ModelIdentifier { get; set; }` - model weld identifier (zero for editor-created)

**Methods:**
- `RectangleBoundingBox GetAxisAlignedBoundingBox()` / `GetObjectAlignedBoundingBox()`
- `DrawingObjectEnumerator GetObjects()` - child objects
- `bool MoveObjectRelative(Vector MoveVector)` - moves the weld mark

---

### LevelMark : DrawingObject
> A level mark in a drawing.

**Constructors:** `LevelMark(ViewBase View, Point InsertionPoint, Point BasePoint)`, plus overloads with PlacingBase, LevelMarkAttributes, and ModelObject

**Properties:**
- `LevelMarkAttributes Attributes { get; set; }` - level mark attributes
- `Point InsertionPoint { get; set; }` - insertion point
- `Point BasePoint { get; set; }` - base point
- `Hideable Hideable { get; set; }` - hidden state
- `PlacingBase Placing { get; set; }` - placing type
- `LevelMarkType SubType { get; set; }` - level mark display type
- `Identifier ModelObjectIdentifier { get; set; }` - model object reference

#### LevelMark.LevelMarkAttributes : AttributesBase

**Properties:**
- `double Angle { get; set; }` - angle in degrees
- `ArrowheadAttributes ArrowHead { get; set; }` - arrow attributes
- `FontAttributes Font { get; set; }` - font
- `Frame Frame { get; set; }` - frame
- `string Prefix { get; set; }` - text before level value
- `string Postfix { get; set; }` - text after level value
- `bool TextHidden { get; set; }` - hide numeric values
- `bool TransparentBackground { get; set; }` - transparency
- `UnitAttributes Unit { get; set; }` - unit attributes
- `bool UseGrouping { get; set; }` - digit grouping
- `bool UsePositiveSignForPositiveLevels { get; set; }` - show + sign
- `PlacingAttributes PlacingAttributes { get; set; }` - placing config

---

### Grid : ModelObject
> Drawing representation of a model grid.

**Properties:**
- `GridAttributes Attributes { get; set; }` - grid attributes

**Methods:**
- `DrawingObjectEnumerator GetObjects()` - gets child objects (GridLine instances)

---

### GridLine : ModelObject
> A single grid line within a Grid.

**Properties:**
- `GridLineAttributes Attributes { get; set; }` - grid line attributes
- `GridLabel StartLabel { get; set; }` - label at start
- `GridLabel EndLabel { get; set; }` - label at end

#### GridLine.GridLabel

**Properties:**
- `string GridLabelText { get; set; }` - label text
- `Point CenterPoint { get; set; }` - center point of label
- `Point GridPoint { get; }` - grid line point (model coords)
- `Point OffsetGridPoint { get; }` - grid line point with offsets
- `Point GridLabelPoint { get; set; }` - label position (set different from GridPoint to create leader)
- `double FrameHeight { get; set; }` / `FrameWidth { get; set; }` - label frame dimensions

---

### ReinforcementBase : ModelObject
> Base class for reinforcement drawing objects (groups, singles, meshes, strands).

**Properties:**
- `ReinforcementBaseAttributes Attributes { get; set; }` - reinforcement attributes
- (Inherited) `Hideable Hideable { get; set; }`
- (Inherited) `Identifier ModelIdentifier { get; set; }`

Subclasses: `ReinforcementGroup`, `ReinforcementSingle`, `ReinforcementMesh`, `ReinforcementStrand`, `ReinforcementSetGroup`

---

### DrawingObjectSelector (UI)
> Selects/deselects drawing objects in the open drawing editor.

**Methods:**
- `DrawingObjectEnumerator GetSelected()` - gets currently selected objects
- `bool SelectObject(DrawingObject obj)` - selects one object
- `bool SelectObjects(ArrayList objects, bool ExtendSelection)` - selects multiple
- `bool UnselectAllObjects()` - deselects all
- `bool UnselectObject(DrawingObject obj)` - deselects one
- `bool UnselectObjects(ArrayList objects)` - deselects multiple

---

### DrawingSelector (UI)
> Accesses the drawing list dialog selections.

**Methods:**
- `DrawingEnumerator GetSelected()` - gets drawings selected in the drawing list

---

### UI.Picker
> Interactive picking of points and objects in the drawing editor.

**Methods:**
- `void PickPoint(string prompt, out Point pickedPoint, out ViewBase pickedView)` - pick one point
- `Tuple<Point, ViewBase> PickPoint(string prompt)` - pick one point (tuple)
- `void PickTwoPoints(string firstPrompt, string secondPrompt, out Point first, out Point second, out ViewBase view)` - pick two points
- `void PickThreePoints(...)` - pick three points
- `void PickPoints(int numberOfPicks, StringList prompts, out PointList pickedPoints, out ViewBase pickedView)` - pick N points
- `void PickPoints(StringList prompts, out PointList pickedPoints, out ViewBase pickedView)` - pick until ESC
- `void PickObject(string prompt, out DrawingObject pickedObject, out ViewBase pickedView)` - pick an object
- `void PickObject(string prompt, out DrawingObject pickedObject, out ViewBase pickedView, out Point pickedPoint)` - pick object with point
- `bool IsInteractive()` - whether picker is in interactive mode

---

### UI.Events : MarshalByRefObject
> Register event listeners for drawing UI events.

**Events (via delegates):**
- `DrawingEditorOpenedDelegate` - drawing editor opened
- `DrawingEditorClosedDelegate` - drawing editor closed
- `SelectionChangeDelegate` - selection changed in editor
- `DrawingListSelectionChangedDelegate` - drawing list selection changed
- `DrawingLoadedDelegate` - drawing loaded
- `DocumentManagerClosedDelegate` - document manager closed
- `InterruptedDelegate` - interrupted
- Layout-related delegates

**Methods:**
- `void Register()` - start listening
- `void UnRegister()` - stop listening
- `void Dispose()` - cleanup

---

### Events (Drawing-level)
> Register event listeners for drawing database events.

**Events (via delegates):**
- `DrawingInsertedDelegate` - drawing created
- `DrawingDeletedDelegate` - drawing deleted
- `DrawingStatusChangedDelegate` - drawing status changed
- `DrawingUpdatedDelegate` - drawing inserted/modified/deleted (with type enum)
- `DrawingReadyForIssuingChangeDelegate` - ready for issuing changed

**Methods:**
- `void Register()` / `void UnRegister()` / `void Dispose()`

---

### Automation.DrawingCreator (static)
> Creates drawings using Tekla Structures built-in logic.

**Static Methods:**
- `static bool CreateDrawings(AutoDrawingRule Rule, Identifier ModelObjectIdentifier)` - creates drawings for one object
- `static bool CreateDrawings(AutoDrawingRule Rule, Identifier ModelObjectIdentifier, out AutoDrawingsStatusEnum status)` - with status
- `static bool CreateDrawings(AutoDrawingRule Rule, List<Identifier> ids, out AutoDrawingsStatusEnum status)` - batch create

#### AutoDrawingRule

**Constructors:** `AutoDrawingRule(string RuleFromFile)`

**Properties:**
- `string Filename { get; set; }` - AutoDrawing script filename

---

### Operations.Operation
> Static operations for drawing objects.

**Static Methods:**
- `static bool MergeMarks(IList<MarkBase> marks, out List<MarkBase> mergedMarks)` - merges marks
- `static bool SplitMarks(IList<MarkBase> marks)` - splits merged marks
- `static bool SortObjectsByPresentationOrder(IList<DrawingObject> objects, out List<DrawingObject> sorted)` - sorts by presentation order

---

### Tools.DrawingCoordinateConverter
> Converts coordinates between views.

**Static Methods:**
- `static Point Convert(ViewBase fromView, ViewBase toView, Point point)` - converts a point
- `static PointList Convert(ViewBase fromView, ViewBase toView, PointList pointList)` - converts point list

---

## Mark Content Elements

### ContainerElement : ElementBase
> A mark element that contains other elements with a frame.

**Properties:**
- `Frame Frame { get; set; }` - frame around the container
- `int Count { get; }` - number of child elements

**Methods:**
- `void Add(ElementBase value)` - adds an element
- `void Clear()` - removes all elements
- `IEnumerator GetEnumerator()` - iterates elements

### PropertyElement : ElementBase
> Represents a property value (profile, material, position, etc.) in a mark.

**Constructors:** `PropertyElement(PropertyElementType PropertyType)`

**Properties:**
- `PropertyElementType PropertyType { get; set; }` - property type
- `FontAttributes Font { get; set; }` - font
- `string Name { get; set; }` - property name
- `string Value { get; set; }` - current value

#### Available PropertyElementTypes (static factory classes):
- `PartMarkPropertyElementTypes`: Profile(), Material(), Name(), Size(), Length(), AssemblyPosition(), PartPosition(), Class(), Finish(), etc.
- `BoltMarkPropertyElementTypes`: BoltDiameter(), BoltLength(), HoleDiameter(), NumberOfBolts(), Standard(), Material(), etc.
- `ReinforcementMarkPropertyElementTypes`: Diameter(), Grade(), Length(), Number(), Cc(), Position(), Shape(), etc.
- `ConnectionMarkPropertyElementTypes`: Code(), Name(), ConnectionNumber(), Group(), etc.
- `SectionMarkPropertyElementTypes`: SectionName(), SourceDrawingName()
- `DetailMarkPropertyElementTypes`: DetailName(), SourceDrawingName()
- `PourMarkPropertyElementTypes`: PourNumber(), PourType(), Material(), etc.
- `MergedMarkPropertyElementTypes`: SingleMarkContent(), BlockPrefix(), etc.

### TextElement : ElementBase
> User-defined text in a mark content.

**Constructors:** `TextElement(string Text)`, `TextElement(string Text, FontAttributes Font)`

**Properties:**
- `string Value { get; set; }` - text value
- `FontAttributes Font { get; set; }` - font

### SymbolElement : ElementBase
> A symbol in a mark content.

**Constructors:** `SymbolElement()`, `SymbolElement(SymbolInfo Symbol)`, `SymbolElement(SymbolInfo Symbol, DrawingColors Color, double Height)`

**Properties:**
- `SymbolInfo Symbol { get; set; }` - symbol info
- `DrawingColors Color { get; set; }` - color
- `double Height { get; set; }` - height (mm)

### NewLineElement : ElementBase
> Line feed between elements to create multi-row marks.

### SpaceElement : ElementBase
> Space between elements.

### UserDefinedElement : ElementBase
> A UDA value element in a mark.

**Constructors:** `UserDefinedElement(string Name)`, `UserDefinedElement(string Name, FontAttributes Font)`

**Properties:**
- `string Name { get; set; }` - UDA name
- `string Value { get; set; }` - current value
- `FontAttributes Font { get; set; }` - font
- `UnitAttributes Unit { get; set; }` - unit/format

### ReinforcementPulloutElement : ElementBase
> Illustrates shape and dimensions of a reinforcing bar in a mark.

**Properties:**
- `bool AutomaticScaling { get; set; }` - auto or manual scaling
- `bool Dimensioning { get; set; }` - show dimensions
- `bool BendingAngle { get; set; }` - show bending angles
- `bool BendingRadius { get; set; }` - show bending radii
- `bool Exaggeration { get; set; }` - exaggeration
- `EndSymbols EndSymbolType { get; set; }` - end symbol type (None=1, Single=2, Both=3)
- `Rotation RotationAxis { get; set; }` - rotation (Automatic=0, Plane=1, Global=2)
- `double ScaleX { get; set; }` / `double ScaleY { get; set; }` - scaling
- `FontAttributes Font { get; set; }` - dimension font

---

## Common Attribute Classes

### FontAttributes : GenericAttributesBase
> Font settings for text objects.

**Constructors:** `FontAttributes()`, `FontAttributes(DrawingColors color, double height, string fontName, bool italic, bool bold)`

**Properties:**
- `string Name { get; set; }` - font name
- `double Height { get; set; }` - height in mm
- `DrawingColors Color { get; set; }` - font color
- `TeklaDrawingColor TrueColor { get; set; }` - true color (RGB)
- `bool Bold { get; set; }` - bold
- `bool Italic { get; set; }` - italic

### Frame
> A frame (box) around an object.

**Constructors:** `Frame(FrameTypes FrameType, DrawingColors FrameColor)`, `Frame(FrameTypes FrameType, TeklaDrawingColor FrameColor)`

**Properties:**
- `FrameTypes Type { get; set; }` - frame type
- `DrawingColors Color { get; set; }` - frame color
- `TeklaDrawingColor TrueColor { get; set; }` - true color

### LineTypeAttributes : GenericAttributesBase
> Basic line attributes.

**Constructors:** `LineTypeAttributes()`, `LineTypeAttributes(LineTypes lineType, DrawingColors color)`

**Properties:**
- `LineTypes Type { get; set; }` - line type
- `DrawingColors Color { get; set; }` - line color
- `TeklaDrawingColor TrueColor { get; set; }` - true color

### ArrowheadAttributes : GenericAttributesBase
> Arrowhead settings.

**Constructors:** `ArrowheadAttributes()`, `ArrowheadAttributes(ArrowheadPositions, ArrowheadTypes, double height, double width)`

**Properties:**
- `ArrowheadPositions ArrowPosition { get; set; }` - position (None, Start, End, Both)
- `ArrowheadTypes Head { get; set; }` - type (NoArrow, FilledArrow, LineArrow, CircleArrow, FilledCircleArrow)
- `double Height { get; set; }` - arrowhead height
- `double Width { get; set; }` - arrowhead width

### Hideable
> Controls object visibility in drawings.

**Properties:**
- `bool IsHidden { get; }` - whether the object is currently hidden

**Methods:**
- `void HideFromDrawing()` - hides from entire drawing
- `void HideFromDrawingView()` - hides from current view
- `void ShowInDrawing()` - shows in drawing
- `void ShowInDrawingView()` - shows in view

### TeklaDrawingColor
> RGB-enabled drawing color.

**Constructors:** `TeklaDrawingColor()`, `TeklaDrawingColor(Color RGB)`, `TeklaDrawingColor(DrawingColors ColorEnum)`, `TeklaDrawingColor(DrawingHatchColors ColorEnum)`

**Properties:**
- `Color RGBColor { get; }` - RGB color value
- `DrawingColors DrawingColorEnum { get; }` - standard color enum

### Size
> Width and height container.

**Constructors:** `Size()`, `Size(double Width, double Height)`

**Properties:**
- `double Width { get; set; }`
- `double Height { get; set; }`

---

## Placing Types

### PlacingBase (abstract)
> Base class for all placing types.

### Concrete placing types:
- `PointPlacing()` - placed at insertion point only
- `LeaderLinePlacing(Point LeaderLineStartPoint)` - attached to leader line
- `AlongLinePlacing(Point StartPoint, Point EndPoint)` - along a defined line
- `BaseLinePlacing(Point StartPoint, Point EndPoint)` - using a reference line for X-axis

### PlacingTypes (static factory)
- `PlacingBase AlongLinePlacing(Point Start, Point End)`
- `PlacingBase BaseLinePlacing(Point Start, Point End)`
- `PlacingBase LeaderLinePlacing(Point LeaderLineStartPoint)`
- `PlacingBase PointPlacing()`

### PreferredPlacingTypeBase (abstract)
> Base for preferred placing types set in attributes.

### PreferredPlacingTypes (static factory - all types):
- `PointPlacingType()`
- `LeaderLinePlacingType()`
- `AlongLinePlacingType()`
- `BaseLinePlacingType()`
- `BaseLineWithArrowAtStartPointPlacingType()`
- `BaseLineWithArrowAtEndPointPlacingType()`
- `AlongPartCenteredPlacingType()`
- `InsidePartAlongPartPlacingType()`
- `InsidePartAlongPartOrWithLeaderLinePlacingType()`
- `InsidePartHorizontalPlacingType()`
- `InsidePartHorizontalOrWithLeaderLinePlacingType()`
- `LeaderLineAndParentObjectAlongPartPlacingType()`
- `AlongLineOrWithLeaderLinePlacingType()`
- `AlongLineOrWithLeaderLineAndParentObjectAlongPartPlacingType()`

### Specialized placing type factories:
- `PreferredMarkPlacingTypes` - subset valid for marks
- `PreferredTextPlacingTypes` - subset valid for texts
- `PreferredSymbolPlacingTypes` - subset valid for symbols

---

## Enumerations

```csharp
// DrawingColors - standard drawing colors
Gray30 = 130, Gray50 = 131, Gray70 = 132, Gray90 = 133
Invisible = 152, Black = 153
NewLine1 = 154 (brown), NewLine2 = 155 (green), NewLine3 = 156 (dark blue)
NewLine4 = 157 (forest green), NewLine5 = 158 (orange), NewLine6 = 159 (gray)
Red = 160, Green = 161, Blue = 162, Cyan = 163, Yellow = 164, Magenta = 165

// DrawingHatchColors - same values as DrawingColors, plus:
Special = 120

// FrameTypes
None = 0, Rectangular = 1, Line = 2, Round = 3
Circle = 4, Diamond = 5, Hexagon = 6, Triangle = 7, Sharpened = 8

// LineTypesEnum
UndefinedLine = 0, SolidLine = 1, DashedLine = 2, SlashedLine = 3
DashDot = 4, DottedLine = 5, DashDoubleDot = 6, SlashDash = 7, Custom = 9999

// TextAlignment
Left = 0, Center = 1, Right = 2, LeaderLine = 3

// ArrowheadPositions
None = 0, Start = 1, End = 2, Both = 3

// ArrowheadTypes
NoArrow = -1005, FilledArrow = -1004, LineArrow = -1003
CircleArrow = -1002, FilledCircleArrow = -1001

// AngleTypes
AngleOnSide = 1, Triangle = 2, AngleAtVertex = 3, TriangleWithDegrees = 4
AngleOnSideGradian = 5, AngleAtVertexGradian = 6, TriangleWithGradians = 7

// DimensionSetBaseAttributes.DimensionTypes
Relative = 1, Absolute = 2, RelativeAndAbsolute = 3, USAbsolute = 4
Elevation = 8, USAbsolute2 = 16
AbsoluteWithShortRelatives = 35, AbsoluteWithAllRelativesAbove = 99

// DimensionSetBaseAttributes.DimensionValueFormats
NoDecimals = 0, OneOptionalDecimal = 1, OneDecimal = 2
TwoOptionalDecimals = 3, TwoDecimals = 4
ThreeOptionalDecimals = 5, ThreeDecimals = 6
RationalPart = 7, SuperscriptEnding = 8

// DimensionSetBaseAttributes.DimensionValuePrecisions
Whole = 1, OnePerTwo = 2, OnePerThree = 3, OnePerFour = 4
OnePerEight = 8, OnePerTen = 10, OnePerSixteen = 16
OnePerThirtytwo = 32, OnePerHundred = 100, OnePerThousand = 1000
TenPerOne = -100, FivePerOne = -50, TwoAndHalfPerOne = -25

// DimensionSetBaseAttributes.DimensionValueUnits
Automatic = 0, Millimeter = 1, Centimeter = 2, Meter = 3, Inch = 4

// DimensionSetBaseAttributes.DimensionTextPlacings
AboveDimensionLine = 0, OnDimensionLine = 1

// DimensionSetBaseAttributes.ExtensionLineTypes
No = 0 (visible), Yes = 1 (hidden), NotOnGridlines = 2

// DimensionSetBaseAttributes.ShortDimensionTypes
Inside = 0, Outside = 1

// DimensionSetBaseAttributes.Placings
Free = 0, Fixed = 1

// DimensionSetBaseAttributes.FrameTypes
None = 0, Rectangle = 1, Underline = 2, RoundedRectangle = 3, SharpenedRectangle = 8

// DimensionSetBaseAttributes.CombineFormats
Off = 0, NumberOfItemsTimesLength = 1, NumberOfItemsTimesLengthEqualsResult = 2

// DimensionSetBaseAttributes.CurvedDimensionTypes
Distance = 0, Angle = 1

// Part.Representation
Outline = 1, Symbol = 2, WorkshopForm = 4, Exact = 8
BoundingBox = 16, BaseBox = 32, SolidWithPartialProfile = 64

// Bolt.Representation
Solid = 1, Symbol = 2, Symbol2 = 4, Symbol3 = 5
DINSymbol = 7, UserDefined = 8, ExactSolid = 9

// Weld.Representation
Outline = 1, Path = 2

// View.ViewTypes
UnknownViewType = 0, FrontView = 1, TopView = 2, BackView = 3, BottomView = 4
EndView = 5, SectionView = 6, ModelView = 7, DetailView = 8, _3DView = 9

// View.HorizontalLabelPosition
CenteredByViewFrame = 1, CenteredByViewRestrictionBox = 2
LeftAlignedByViewFrame = 3, RightAlignedByViewFrame = 4
LeftAlignByViewRestrictionBox = 5, RightAlignByViewRestrictionBox = 6

// View.VerticalLabelPosition
Top = 1, Bottom = 2

// View.ShorteningCutPartType
BothDirections = 1, X_Direction = 2, Y_Direction = 3

// View.LabelLineLengthType
Minimal = 1, Custom = 2

// DrawingUpToDateStatus
DrawingIsUpToDate = 1, PartsWereModified = 2, DrawingIsUpToDateButMayNeedChecking = 4
IncludedDrawingModified = 5, OriginalPartDeleted = 6, AllPartsDeleted = 7
NumberOfPartsInNumberingSeriesIncreased = 8, NumberOfPartsInNumberingSeriesDecreased = 9
DrawingWasCloned = 10, DrawingWasUpdated = 11, CopiedViewChanged = 12
DrawingWasSplitted = 13, MovedViewDeleted = 14, MovedViewLabelChanged = 15
DrawingWasClonedFromCloud = 16

// DetailMark.DetailMarkAttributes.DetailBoundaryShape
None = 1, Circular = 2, Rectangular = 3

// SectionMarkBase.SectionMarkSymbol.SymbolShapes
None = 1, Custom = 4
SymbolShape1 = 101 (triangle halves), SymbolShape2 = 102 (arrow to line)
SymbolShape3 = 103 (half arrow), SymbolShape4 = 104 (flat rect)
SymbolShape5 = 105 (triangle+circle mixed), SymbolShape6 = 106 (triangle on top)
SymbolShape7 = 107 (empty triangle), SymbolShape8 = 108 (filled triangle)

// MarkSymbolShape
None = 1, Circular = 2, SlashedCircular = 3, Custom = 4

// LevelMark.LevelMarkType
NoArrowNoLeaderLine = 0, ArrowWithoutLeaderLine = 1
InclinedLeaderLine = 2, OrthogonalLeaderLine = 3

// LeaderLine.LeaderLineTypes
Undefined = 0, NormalLeaderLine = 1, SupportLeaderLine = 2, ExtensionLeaderLine = 3

// TagLocation
AboveLine = 1, BelowLine = 2, MiddleLevelOfSymbol = 3
AboveSymbolCenterLine = 4, BelowSymbolCenterLine = 5, CustomRelativeToSymbol = 6

// Hideable.HiddenFlags
NotHidden = 0, HiddenBySelf = 1, HiddenByParent = 2

// Hideable.ShouldBeHiddenFlags
ShouldNotBeHidden = 0, HideFromDrawingView = 1, HideFromDrawing = 2
ShowInDrawingView = 3, ShowInDrawing = 4

// Units
Automatic = 0, Millimeters = 1, Centimeters = 2, Meters = 3
FeetAndInches = 4, CentimetersOrMeters = 5, Inches = 6, Feet = 7

// FormatTypes
Automatic = 0, WholeNumber = 1, OneDecimalIfValidDecimal = 2, OneDecimal = 3
TwoDecimalsIfValidDecimals = 4, TwoDecimals = 5, ThreeDecimalsIfValidDecimals = 6
ThreeDecimals = 7, Fractional = 8

// AutoSizeOptions
CalculatedSizes = 0, FixedSizes = 1, CalculatedAndFixedSizes = 2

// SizeDefinitionMode
AutoSize = 0, SpecifiedSize = 1

// EmbeddedObjectScalingOptions
ScaleX = 1, ScaleXY = 3, ScaleToFit = 4, ScaleBestFit = 8

// ScalingOptions (for links/text files)
NoScaling = 0, ScaleToFit = 4

// Automation.AutoDrawingsStatusEnum
OPERATION_OK = 0, OPERATION_FAILED = 1
ERROR_NUMBERING_NOT_UPTODATE = 2, ERROR_DRAWING_EDITOR_MUST_BE_CLOSED = 3

// Events.DrawingUpdateTypeEnum
INSERTED = 0, MODIFIED = 1, DELETED = 2

// ReinforcementBase.ReinforcementRepresentationTypes
SingleLine = 1, DoubleLine = 2, Filled = 3, Stick = 4
OutLine = 5, DoubleLineFilledEnds = 6, SingleLineFilledEnds = 7, OutlineIgnoreHoles = 8

// ReinforcementBase.ReinforcementVisibilityTypes
All = 1, First = 2, Last = 3, FirstAndLast = 4
OneInTheMiddle = 5, TwoInTheMiddle = 6, Customized = 7

// ReinforcementBase.HookedEndSymbolTypes
NoHook = 1, HalfHook = 2, FullHook = 3

// ReinforcementBase.StraightEndSymbolTypes
NoSymbol = 1, HooksOnInnerSide = 2, HooksOnOuterSide = 3
HookOnReinforcementStartOuterSide = 4, HookOnReinforcementStartInnerSide = 5, FullHooks = 6

// DotPrintOutputType
Printer = 0, PDF = 2, Plot = 3, Image = 4

// DotPrintPaperSize
Auto = 0, A0 = 1, A1 = 2, A2 = 3, A3 = 4, A4 = 5, A5 = 6

// DotPrintOrientationType
Auto = 0, Landscape = 1, Portrait = 2

// DotPrintColor
Color = 0, BlackAndWhite = 1, GreyScale = 2

// DotPrintAreaType
EntireDrawing = 0, VisibleArea = 1

// DotPrintScalingType
Auto = 0, Scale = 1

// DotPrintToMultipleSheet
Off = 0, LeftToRightAndTopToBottom = 1, BottomToTopAndRightToLeft = 2

// IncludeRevisionMarkEnum
Never = 0, Always = 1, ByFormatString = 2

// SectionMarkBase.SectionMarkTagAttributes.TagShowOnSide
ShowOnBothSides = 1, ShowOnLeftSideOnly = 2, ShowOnRightSideOnly = 3

// SectionMarkBase.SectionMarkTagAttributes.TagTextRotation
AlwaysRotate = 1, AlwaysHorizontal = 2, DoNotRotateVertically = 3, AlwaysOrthogonal = 4
```

---

## Type Reference (Tier 2)

| Type | Base | Description |
|------|------|-------------|
| `DrawingEnumerator` | `DrawingEnumeratorBase` | Iterates `Drawing` instances. `.Current` returns `Drawing` |
| `DrawingObjectEnumerator` | `DrawingEnumeratorBase` | Iterates `DrawingObject` instances. `.Current` returns `DrawingObject` |
| `DrawingEnumeratorBase` | - | Base enumerator: `MoveNext()`, `Reset()`, `GetSize()`, `AutoFetch` |
| `PointList` | `CollectionBase` | Type-safe list of `Point` with `Add`, `Contains`, `IndexOf`, `Remove` |
| `StringList` | `CollectionBase` | Type-safe list of `string` |
| `IntList` | `CollectionBase` | Type-safe list of `int` |
| `LeaderLine` | `DrawingObject` | Leader line attached to mark/text. Props: `StartPoint`, `EndPoint`, `ElbowPoints`, `LeaderLineType` |
| `DimensionLink` | `DrawingObject` | Link between two `StraightDimensionSet` instances |
| `AngleDimension` | `DimensionBase` | Angle dimension between two lines |
| `RadiusDimension` | `DimensionBase` | Radius dimension for arcs |
| `CurvedDimensionOrthogonal` | `CurvedDimensionBase` | Curved dimension with orthogonal reference |
| `CurvedDimensionRadial` | `CurvedDimensionBase` | Curved dimension with radial reference |
| `CurvedDimensionSetOrthogonal` | `CurvedDimensionSetBase` | Curved dimension set (orthogonal) |
| `CurvedDimensionSetRadial` | `CurvedDimensionSetBase` | Curved dimension set (radial) |
| `CurvedDimensionSetHandler` | - | Creates curved dimension sets |
| `StraightDimensionSetHandler` | - | Creates straight dimension sets |
| `CurvedSectionMark` | `SectionMarkBase` | Curved section mark. Has `MiddlePoint` property |
| `MarkSet` | `MarkBase` | Multiple merged marks |
| `Connection` | `ModelObject` | Drawing connection object |
| `Weld` | `ModelObject` | Drawing weld (line representation) |
| `Surfacing` | `ModelObject` | Drawing surfacing |
| `PourObject` | `ModelObject` | Drawing pour |
| `PourBreak` | `ModelObject` | Drawing pour break |
| `EdgeChamfer` | `ModelObject` | Drawing edge chamfer |
| `ReferenceModel` | `ModelObject` | Drawing reference model |
| `ReinforcementGroup` | `ReinforcementBase` | Rebar group in drawing |
| `ReinforcementSingle` | `ReinforcementBase` | Single rebar in drawing |
| `ReinforcementMesh` | `ReinforcementBase` | Rebar mesh in drawing |
| `ReinforcementStrand` | `ReinforcementBase` | Prestressing strand in drawing |
| `ReinforcementSetGroup` | `ReinforcementBase` | Rebar set group in drawing |
| `Image` | `EmbeddedObjectBase` | Image file in a drawing |
| `DwgObject` | `EmbeddedObjectBase` | DWG/DXF file in a drawing |
| `TextFile` | `EmbeddedObjectBase` | Text file in a drawing |
| `HyperLink` | `LinkBase` | URL hyperlink in a drawing |
| `DrawingLink` | `LinkBase` | Link to another drawing |
| `Plugin` | `DrawingObject` | Drawing plug-in instance |
| `PluginPickerInput` | - | Predefined picker inputs for plug-ins |
| `RectangleBoundingBox` | `AABB` | Bounding box with corner points, width, height, angle |
| `LayoutAttributes` | `AttributesBase` | Drawing layout (sheet size, auto-size) |
| `PrintAttributes` | `GenericAttributesBase` | Print settings (old API) |
| `DPMPrinterAttributes` | `GenericAttributesBase` | Print settings (DPM API - PDF, printer, image) |
| `PlacingAttributes` | `GenericAttributesBase` | Fixed/free placing, distance, quarter |
| `PlacingDistanceAttributes` | `GenericAttributesBase` | Min/max distance, search margin |
| `PlacingQuarterAttributes` | `GenericAttributesBase` | TopLeft, TopRight, BottomLeft, BottomRight |
| `PlacingDirectionAttributes` | `GenericAttributesBase` | Positive/Negative direction |
| `UnitAttributes` | `GenericAttributesBase` | Unit and format settings |
| `HatchAttributes` | `GenericAttributesBase` | Base hatch: Name, Color, Angle, Scale |
| `ModelObjectHatchAttributes` | `HatchAttributes` | Hatch for model objects |
| `GraphicObjectHatchAttributes` | `HatchAttributes` | Hatch for graphic objects |
| `CustomLineType` | `LineTypes` | Custom line type with description |
| `CustomLineTypeCatalog` | static | `Get()`, `Get(string)`, `Get(int)` - fetch custom line types |
| `NormalLineType` | `LineTypes` | Standard line types |
| `SymbolLibrary` | static | `GetSymbolLibraries()` - list symbol files |
| `ReportTemplateElement` | `ElementBase` | Template reference in mark content |
| `TemplateInfo` | `GenericAttributesBase` | Template file info (Name property) |
| `Tools.InputDefinitionFactory` | static | Creates `InputDefinition` for plug-in inputs |

---

## Sub-namespaces

- **Tekla.Structures.Drawing** - main namespace: drawings, views, objects, marks, dimensions, graphic objects
- **Tekla.Structures.Drawing.UI** - `DrawingObjectSelector`, `DrawingSelector`, `Picker`, UI `Events`
- **Tekla.Structures.Drawing.UI.Events** - UI event delegates (editor open/close, selection, layout)
- **Tekla.Structures.Drawing.Events** - drawing database event delegates (insert, delete, modify, status)
- **Tekla.Structures.Drawing.Tools** - `DrawingCoordinateConverter`, `InputDefinitionFactory`
- **Tekla.Structures.Drawing.Automation** - `DrawingCreator`, `AutoDrawingRule`
- **Tekla.Structures.Drawing.Operations** - `Operation` (merge/split marks, sort by presentation order)

---

## Picker Input Classes (for Plugin automation)

| Class | Usage |
|-------|-------|
| `PickerInput` | Abstract base |
| `PickerInputInterrupt` | Simulates user pressing Escape |
| `PickerInputPoint` | One picked point in a view |
| `PickerInputTwoPoints` | Two picked points in a view |
| `PickerInputThreePoints` | Three picked points in a view |
| `PickerInputNPoints` | N picked points in a view |
| `PickerInputObject` | One picked object in a view |
| `PickerInputObjectAndAPoint` | Object and point in a view |

---

## Exception Classes

| Exception | When Thrown |
|-----------|------------|
| `CannotDeleteActiveDrawingException` | Deleting an active drawing |
| `CannotInsertDrawingException` | Inserting when another drawing is active |
| `CannotModifyNonActiveDrawingException` | Modifying a non-active drawing |
| `CannotPerformOperationDrawingMustBeActiveException` | Operation requires active drawing |
| `CannotPerformOperationDrawingEditorMustBeClosedException` | Operation requires editor closed |
| `CannotPerformOperationDrawingIsActiveException` | Operation requires drawing not active |
| `CannotPerformOperationDrawingNotUpToDateException` | Drawing not up to date |
| `CannotPerformOperationNumberingNotUpToDate` | Numbering not up to date |
| `CannotLoadAttributesException` | Attribute file loading failed |
| `CannotCreateSectionViewFrom3dView` | Section view from 3D view |
| `PickerInterruptedException` | User interrupted the picker |
| `InvalidAttributesForOperationException` | Invalid attributes set |
| `InvalidTypeException` | Wrong type used |
