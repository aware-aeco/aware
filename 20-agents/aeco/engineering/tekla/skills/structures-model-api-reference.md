---
parent: tekla-structures-model
description: Full API reference for Tekla.Structures.Model classes, methods, properties, constructors, enums.
---

## Core Classes (Tier 1)

### Model
> The Model class represents a single model open in Tekla Structures. Before interaction with the model, the user will have to create one instance of this class.

**Constructors:** `Model()`

**Methods:**
- `bool GetConnectionStatus()` - check if connected to Tekla
- `bool CommitChanges()` - save changes to database
- `bool CommitChanges(string Message)` - save changes with message
- `ModelObjectSelector GetModelObjectSelector()` - get selector for querying objects
- `ModelInfo GetInfo()` - get model info (name, path, etc.)
- `ProjectInfo GetProjectInfo()` - get project info
- `PhaseCollection GetPhases()` - get all phases
- `WorkPlaneHandler GetWorkPlaneHandler()` - get work plane handler
- `ClashCheckHandler GetClashCheckHandler()` - get clash check handler
- `ModelObject SelectModelObject(Identifier ID)` - select object by identifier
- `string GetGUIDByIdentifier(Identifier identifier)` - get GUID from identifier
- `Identifier GetIdentifierByGUID(string guid)` - get identifier from GUID
- `List<ModelObject> FetchModelObjects(List<Identifier> Ids, bool SelectInstances)` - batch fetch
- `List<ModelObject> FetchModelObjects(List<string> Guids, bool SelectInstances)` - batch fetch by GUID
- `List<double> GetReportPropertyDouble(IList<Identifier> identifiers, string property)` - batch report property
- `List<int> GetReportPropertyInt(IList<Identifier> identifiers, string property)` - batch report property
- `List<string> GetReportPropertyStr(IList<Identifier> identifiers, string property)` - batch report property

---

### ModelObject : Object (abstract)
> Abstract base class for all model objects, such as parts, bolts, welds and reinforcements.

**Properties:**
- `Identifier Identifier { get; set; }` - the object identifier (inherited from Object)
- `bool IsUpToDate { get; }` - whether the object has no unshared modifications
- `DateTime? ModificationTime { get; }` - last modification time

**Methods:**
- `bool Select()` - read properties from database (MUST call before reading)
- `bool Insert()` - insert new object into model
- `bool Modify()` - save modified properties to database
- `bool Delete()` - delete object from model
- `int CompareTo(object obj)` - compare identifiers
- `ModelObjectEnumerator GetChildren()` - get child objects
- `CoordinateSystem GetCoordinateSystem()` - get object coordinate system
- `BaseComponent GetFatherComponent()` - get parent component
- `bool GetPhase(out Phase phase)` - get object phase
- `bool SetPhase(Phase phase)` - set object phase
- `bool SetLabel(string label)` - set object label
- `string GetCustomObjectType()` - get custom object type
- `bool SetCustomObjectType(string type)` - set custom object type
- `bool GetUserProperty(string name, ref string value)` - get string UDA
- `bool GetUserProperty(string name, ref double value)` - get double UDA
- `bool GetUserProperty(string name, ref int value)` - get int UDA
- `bool SetUserProperty(string name, string value)` - set string UDA
- `bool SetUserProperty(string name, double value)` - set double UDA
- `bool SetUserProperty(string name, int value)` - set int UDA
- `bool GetAllUserProperties(ref Hashtable values)` - get all UDAs
- `bool SetUserProperties(List<string> stringPropertyNames, List<string> stringValues, List<string> doublePropertyNames, List<double> doubleValues, List<string> intPropertyNames, List<int> intValues)` - batch set UDAs
- `bool GetReportProperty(string name, ref string value)` - get string report property
- `bool GetReportProperty(string name, ref double value)` - get double report property
- `bool GetReportProperty(string name, ref int value)` - get int report property
- `bool GetAllReportProperties(ArrayList stringNames, ArrayList doubleNames, ArrayList integerNames, ref Hashtable values)` - get multiple report properties
- `bool GetDynamicStringProperty(string name, ref string value)` - get dynamic property
- `bool SetDynamicStringProperty(string name, string value)` - set dynamic property
- `ModelObjectEnumerator GetHierarchicObjects()` - get hierarchic objects
- `bool GetDoubleUserProperties(ref Hashtable values)` - get all double UDAs
- `bool GetIntegerUserProperties(ref Hashtable values)` - get all int UDAs
- `bool GetStringUserProperties(ref Hashtable values)` - get all string UDAs
- `bool GetDoubleReportProperties(ArrayList names, ref Hashtable values)` - batch double report props
- `bool GetIntegerReportProperties(ArrayList names, ref Hashtable values)` - batch int report props
- `bool GetStringReportProperties(ArrayList names, ref Hashtable values)` - batch string report props

---

### Part : ModelObject (abstract)
> Represents a part in the model. A part can be either a beam, a polybeam or a contour plate.

**Properties:**
- `string Name { get; set; }` - part name
- `string Class { get; set; }` - part class
- `Material Material { get; set; }` - part material
- `Profile Profile { get; set; }` - part profile
- `string Finish { get; set; }` - part finish
- `Position Position { get; set; }` - part positioning
- `NumberingSeries PartNumber { get; set; }` - part numbering
- `NumberingSeries AssemblyNumber { get; set; }` - assembly numbering
- `DeformingData DeformingData { get; set; }` - deforming data
- `CastUnitTypeEnum CastUnitType { get; set; }` - PRECAST or CAST_IN_PLACE
- `int PourPhase { get; set; }` - pour phase

**Methods:**
- `bool CompareTo(Part partToCompare)` - compare parts
- `Assembly GetAssembly()` - get parent assembly
- `Solid GetSolid()` - get part solid geometry
- `Solid GetSolid(SolidCreationTypeEnum solidCreationType)` - get solid with creation type
- `Solid GetSolid(FormingStates formingStates)` - get solid with forming state
- `ArrayList GetCenterLine(bool withCutsFittings)` - get center line points
- `ArrayList GetReferenceLine(bool withCutsFittings)` - get reference line points
- `string GetPartMark()` - get part mark
- `ModelObjectEnumerator GetBolts()` - get bolts
- `ModelObjectEnumerator GetBooleans()` - get boolean operations
- `ModelObjectEnumerator GetComponents()` - get components
- `ModelObjectEnumerator GetReinforcements()` - get reinforcements
- `ModelObjectEnumerator GetWelds()` - get welds
- `ModelObjectEnumerator GetSurfaceTreatments()` - get surface treatments
- `ModelObjectEnumerator GetSurfaceObjects()` - get surface objects
- `ModelObjectEnumerator GetPours()` - get pour objects
- `CoordinateSystem GetDSTVCoordinateSystem()` - get DSTV coordinate system

---

### Beam : Part
> Represents a single beam in the model. A beam has a single start and end point.

**Constructors:** `Beam()`, `Beam(BeamTypeEnum beamType)`, `Beam(Point startPoint, Point endPoint)`

**Properties:**
- `Point StartPoint { get; set; }` - start point
- `Point EndPoint { get; set; }` - end point
- `Offset StartPointOffset { get; set; }` - start point offset
- `Offset EndPointOffset { get; set; }` - end point offset
- `BeamTypeEnum Type { get; }` - beam type (read-only)

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

```csharp
// Example: Create a beam
var beam = new Beam();
beam.StartPoint = new Point(0, 0, 0);
beam.EndPoint = new Point(6000, 0, 0);
beam.Profile.ProfileString = "HEA300";
beam.Material.MaterialString = "S235JR";
beam.Class = "1";
beam.Position.Depth = Position.DepthEnum.MIDDLE;
beam.Position.Plane = Position.PlaneEnum.MIDDLE;
beam.Position.Rotation = Position.RotationEnum.FRONT;
beam.Insert();
```

---

### ContourPlate : Part
> Represents a part made with a contour, such as a concrete slab.

**Constructors:** `ContourPlate()`

**Properties:**
- `Contour Contour { get; set; }` - the contour for the plate
- `ContourPlateTypeEnum Type { get; }` - PLATE or SLAB (read-only, based on material)

**Methods:**
- `bool AddContourPoint(ContourPoint contourPoint)` - add contour point
- `Polycurve GetContourPolycurve()` - get contour as polycurve
- `Insert()`, `Modify()`, `Delete()`, `Select()`

```csharp
// Example: Create a contour plate
var plate = new ContourPlate();
plate.AddContourPoint(new ContourPoint(new Point(0, 0, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(1000, 0, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(1000, 1000, 0), null));
plate.AddContourPoint(new ContourPoint(new Point(0, 1000, 0), null));
plate.Profile.ProfileString = "PL20";
plate.Material.MaterialString = "S235JR";
plate.Insert();
```

---

### PolyBeam : Part
> Represents a continuous beam with a contour as input.

**Constructors:** `PolyBeam()`, `PolyBeam(PolyBeamTypeEnum polyBeamType)`

**Properties:**
- `Contour Contour { get; set; }` - contour defining the polybeam path
- `PolyBeamTypeEnum Type { get; }` - BEAM, PANEL, STRIP_FOOTING, or COLUMN (read-only)

**Methods:**
- `bool AddContourPoint(ContourPoint contourPoint)` - add point to contour
- `Polycurve GetCenterLinePolycurve()` - get center line as polycurve
- `ArrayList GetPolybeamCoordinateSystems()` - get coordinate systems at each segment
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### BentPlate : Part
> A class for the bent plate.

**Constructors:** `BentPlate()`

**Properties:**
- `ConnectiveGeometry Geometry { get; set; }` - the geometry of the bent plate
- `double Thickness { get; }` - thickness (read-only)

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Brep : Part
> Represents a single brep in the model. A brep has a single start and end point.

**Constructors:** `Brep()`, `Brep(Point startPoint, Point endPoint)`

**Properties:**
- `Point StartPoint { get; set; }` - start point
- `Point EndPoint { get; set; }` - end point
- `Offset StartPointOffset { get; set; }` - start point offset
- `Offset EndPointOffset { get; set; }` - end point offset

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Assembly : ModelObject
> Defines a single manufacturing unit: objects that are bolted or welded together in the workshop.

**Constructors:** `Assembly()`

**Properties:**
- `NumberingSeries AssemblyNumber { get; set; }` - numbering series
- `string Name { get; set; }` - assembly name

**Methods:**
- `bool Add(IAssemblable Object)` - add an assemblable object
- `bool Add(Assembly Assembly)` - add a sub-assembly
- `bool Add(ArrayList Assemblables)` - add multiple assemblables
- `bool Remove(ModelObject Object)` - remove an object
- `ModelObject GetMainPart()` - get the main part
- `ModelObject GetMainObject()` - get the main object
- `bool SetMainPart(Part Part)` - set the main part
- `bool SetMainObject(ModelObject mainObject)` - set the main object
- `ArrayList GetSecondaries()` - get secondary parts
- `ArrayList GetSubAssemblies()` - get sub-assemblies
- `Assembly GetAssembly()` - get parent assembly
- `AssemblyTypeEnum GetAssemblyType()` - get type (STEEL, PRECAST, etc.)
- `bool CompareTo(Assembly AssemblyToCompare)` - compare assemblies
- `PourObject GetFatherPour()` - get father pour
- `PourUnit GetFatherPourUnit()` - get father pour unit
- `bool HasUserDefinedCoordSys()` - check for user-defined coord sys
- `bool SetUserDefinedCoordSys(CoordinateSystem cs)` - set user-defined coord sys
- `bool DeleteUserDefinedCoordSys()` - delete user-defined coord sys
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### BoltGroup : ModelObject (abstract)
> Abstract base class for all bolt shapes: array, circle and XY list.

**Properties:**
- `Point FirstPosition { get; set; }` - first position point
- `Point SecondPosition { get; set; }` - second position point
- `double BoltSize { get; set; }` - bolt size
- `string BoltStandard { get; set; }` - bolt standard
- `BoltTypeEnum BoltType { get; set; }` - BOLT_TYPE_SITE or BOLT_TYPE_WORKSHOP
- `Part PartToBoltTo { get; set; }` - part to bolt to
- `Part PartToBeBolted { get; set; }` - part to be bolted
- `bool Bolt { get; set; }` - true = bolt, false = just a hole
- `ArrayList BoltPositions { get; }` - bolt positions (read-only)
- `double Tolerance { get; set; }` - hole tolerance
- `BoltHoleTypeEnum HoleType { get; set; }` - hole type (OVERSIZED, SLOTTED, etc.)
- `double CutLength { get; set; }` - cut length
- `double ExtraLength { get; set; }` - extra length
- `double Length { get; set; }` - length (for studs)
- `Position Position { get; set; }` - position attributes
- `Offset StartPointOffset { get; set; }` - start point offset
- `Offset EndPointOffset { get; set; }` - end point offset
- `bool ConnectAssemblies { get; set; }` - connect as sub-assembly
- `bool Hole1..Hole5 { get; set; }` - hole usage flags
- `bool Nut1, Nut2 { get; set; }` - nut usage flags
- `bool Washer1..Washer3 { get; set; }` - washer usage flags
- `double SlottedHoleX/Y { get; set; }` - slotted hole allowances
- `BoltThreadInMaterialEnum ThreadInMaterial { get; set; }` - thread in material
- `BoltRotateSlotsEnum RotateSlots { get; set; }` - slot rotation

**Methods:**
- `bool AddOtherPartToBolt(Part M)` - add another part to bolt
- `bool RemoveOtherPartToBolt(Part M)` - remove a bolted part
- `ArrayList GetOtherPartsToBolt()` - get other bolted parts
- `Solid GetSolid()` - get bolt solid
- `Solid GetSolid(bool withHighAccuracy)` - get bolt solid with accuracy

---

### BoltArray : BoltGroup
> Defines a bolt group with an array shape.

**Constructors:** `BoltArray()`

**Methods:**
- `bool AddBoltDistX(double DistX)` - add X distance
- `bool AddBoltDistY(double DistY)` - add Y distance
- `double GetBoltDistX(int Index)` - get X distance at index
- `double GetBoltDistY(int Index)` - get Y distance at index
- `int GetBoltDistXCount()` - get X distance count
- `int GetBoltDistYCount()` - get Y distance count
- `bool SetBoltDistX(int Index, double DistX)` - set X distance
- `bool SetBoltDistY(int Index, double DistY)` - set Y distance
- `bool RemoveBoltDistX(int Index)` - remove X distance
- `bool RemoveBoltDistY(int Index)` - remove Y distance
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### BaseWeld : ModelObject (abstract)
> Abstract class defining a weld between two model objects.

**Properties:**
- `ModelObject MainObject { get; set; }` - main part of the weld
- `ModelObject SecondaryObject { get; set; }` - secondary part of the weld
- `bool ShopWeld { get; set; }` - true = shop weld, false = site weld
- `bool AroundWeld { get; set; }` - around weld or edge weld
- `WeldTypeEnum TypeAbove { get; set; }` - weld type above
- `WeldTypeEnum TypeBelow { get; set; }` - weld type below
- `double SizeAbove { get; set; }` - size above
- `double SizeBelow { get; set; }` - size below
- `double LengthAbove { get; set; }` - length above
- `double LengthBelow { get; set; }` - length below
- `double AngleAbove/Below { get; set; }` - angles
- `WeldContourEnum ContourAbove/Below { get; set; }` - contours
- `WeldFinishEnum FinishAbove/Below { get; set; }` - finishes
- `WeldIntermittentTypeEnum IntermittentType { get; set; }` - continuous/chain/staggered
- `double PitchAbove/Below { get; set; }` - pitch values
- `WeldProcessTypeEnum ProcessType { get; set; }` - SMAW, SAW, GMAW, etc.
- `double EffectiveThroatAbove/Below { get; set; }` - effective throat
- `double RootOpeningAbove/Below { get; set; }` - root opening
- `double RootFaceAbove/Below { get; set; }` - root face
- `bool ConnectAssemblies { get; set; }` - connect as sub-assembly
- `bool StitchWeld { get; set; }` - stitched weld
- `int WeldNumber { get; }` - weld number (read-only)
- `string WeldNumberPrefix { get; set; }` - weld number prefix
- `string Standard { get; set; }` - weld standard
- `string ReferenceText { get; set; }` - reference text
- `WeldPlacementTypeEnum Placement { get; set; }` - placement (AUTO, MAIN, SECONDARY)
- `WeldPreparationTypeEnum Preparation { get; set; }` - preparation
- `WeldNDTInspectionEnum NDTInspection { get; set; }` - NDT inspection level
- `WeldElectrodeClassificationEnum ElectrodeClassification { get; set; }` - electrode class
- `double ElectrodeCoefficient { get; set; }` - electrode coefficient
- `double ElectrodeStrength { get; set; }` - electrode strength

**Methods:**
- `Solid GetSolid()` - get weld solid
- `ArrayList GetWeldGeometries()` - get weld geometries

---

### Weld : BaseWeld
> Represents a normal weld in the model with a main and secondary part.

**Constructors:** `Weld()`

**Properties:**
- `Vector Direction { get; set; }` - weld edge search direction (normalized to 1000.0)
- `WeldPositionEnum Position { get; set; }` - weld position (+/-X, +/-Y, +/-Z)

**Methods:**
- `bool GetLogicalWeld(ref LogicalWeld LogicalWeld)` - get logical weld
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### PolygonWeld : BaseWeld
> Represents a polygon weld in the model with a polygon shape.

**Constructors:** `PolygonWeld()`

**Properties:**
- `Polygon Polygon { get; set; }` - polygon defining the weld path

**Methods:**
- `bool GetLogicalWeld(ref LogicalWeld LogicalWeld)` - get logical weld
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### BooleanPart : Boolean
> Represents a part cut or an add. A model object is cut with a part instance to create a void.

**Constructors:** `BooleanPart()`

**Properties:**
- `ModelObject Father { get; set; }` - father object (inherited from Boolean)
- `Part OperativePart { get; set; }` - the part that does the cut/add
- `BooleanTypeEnum Type { get; set; }` - BOOLEAN_ADD, BOOLEAN_CUT, or BOOLEAN_WELDPREP

**Methods:**
- `bool SetOperativePart(Part Part)` - set the operative part
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Fitting : Boolean
> Defines how the part end fits to a plane. Can make the part longer or shorter.

**Constructors:** `Fitting()`

**Properties:**
- `ModelObject Father { get; set; }` - father object (inherited from Boolean)
- `Plane Plane { get; set; }` - the fitting plane

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### CutPlane : Boolean
> Defines how the end of a part is cut with a plane. Cannot extend part boundaries.

**Constructors:** `CutPlane()`

**Properties:**
- `ModelObject Father { get; set; }` - father object (inherited from Boolean)
- `Plane Plane { get; set; }` - the cutting plane

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Reinforcement : ModelObject (abstract)
> Abstract base for all reinforcements: mesh, single rebar, rebar group, or strand.

**Properties:**
- `ModelObject Father { get; set; }` - the father part
- `string Name { get; set; }` - reinforcement name
- `int Class { get; set; }` - reinforcement class
- `string Grade { get; set; }` - steel grade
- `NumberingSeries NumberingSeries { get; set; }` - numbering series
- `double FromPlaneOffset { get; set; }` - offset from part surface
- `double StartPointOffsetValue { get; set; }` - cover thickness or leg length at start
- `double EndPointOffsetValue { get; set; }` - cover thickness or leg length at end
- `RebarOffsetTypeEnum StartPointOffsetType { get; set; }` - COVER_THICKNESS or LEG_LENGTH
- `RebarOffsetTypeEnum EndPointOffsetType { get; set; }` - COVER_THICKNESS or LEG_LENGTH
- `ArrayList OnPlaneOffsets { get; set; }` - per-leg plane offsets
- `ArrayList RadiusValues { get; set; }` - bend radius values

**Methods:**
- `Assembly GetAssembly()` - get assembly
- `PourObject GetFatherPour()` - get father pour
- `PourUnit GetFatherPourUnit()` - get father pour unit
- `int GetNumberOfRebars()` - get bar count
- `ArrayList GetRebarGeometries(bool withHooks)` - get bar geometries
- `ArrayList GetRebarGeometriesWithoutClashes(bool withHooks)` - get clash-free geometries
- `RebarGeometry GetSingleRebar(int index, bool withHooks)` - get single bar geometry
- `List<RebarComplexGeometry> GetRebarComplexGeometries(bool withHooks, bool withoutClashes, bool lengthAdjustments)` - get complex geometries
- `Solid GetSolid()` - get solid
- `bool IsGeometryValid()` - check geometry validity

---

### RebarGroup : BaseRebarGroup
> Represents a group of reinforcing bars.

**Constructors:** `RebarGroup()`

**Properties (inherited from BaseRebarGroup):**
- `Point StartPoint { get; set; }` - bar distribution start point
- `Point EndPoint { get; set; }` - bar distribution end point
- `string Size { get; set; }` - bar size
- `ArrayList Spacings { get; set; }` - spacing values
- `RebarGroupSpacingTypeEnum SpacingType { get; set; }` - spacing type
- `RebarHookData StartHook { get; set; }` - start hook
- `RebarHookData EndHook { get; set; }` - end hook
- `ExcludeTypeEnum ExcludeType { get; set; }` - exclude first/last bars
- `double FromPlaneOffset { get; set; }` - from plane offset
- `double StartFromPlaneOffset { get; set; }` - start from plane offset
- `double EndFromPlaneOffset { get; set; }` - end from plane offset

**Own Properties:**
- `ArrayList Polygons { get; set; }` - polygons defining bar shape (1 for non-tapered, 2-99 for tapered)
- `RebarGroupStirrupTypeEnum StirrupType { get; set; }` - POLYGONAL, SPIRAL, or TAPERED_CURVED

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### CircleRebarGroup : BaseRebarGroup
> Represents a group of reinforcing bars with a circular shape.

**Constructors:** `CircleRebarGroup()`

**Properties:**
- `Polygon Polygon { get; set; }` - polygon (must have 3 points)
- `CircleRebarGroupStirrupTypeEnum StirrupType { get; set; }` - CIRCLE or SPIRAL

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### BaseComponent : ModelObject (abstract)
> Abstract base class for generic components: component, connection, detail, and seam.

**Properties:**
- `string Name { get; set; }` - component name (identifies custom components or plug-ins)
- `int Number { get; set; }` - component number

**Methods:**
- `bool GetAttribute(string AttrName, ref int Value)` - get int attribute
- `bool GetAttribute(string AttrName, ref double DValue)` - get double attribute
- `bool GetAttribute(string AttrName, ref string StrValue)` - get string attribute
- `void SetAttribute(string AttrName, string StrValue)` - set string attribute
- `void SetAttribute(string AttrName, int Value)` - set int attribute
- `void SetAttribute(string AttrName, double DValue)` - set double attribute
- `bool LoadAttributesFromFile(string Filename)` - load from file

---

### Connection : BaseComponent
> Represents a connection that connects two or more parts together.

**Constructors:** `Connection()`

**Properties:**
- `int Class { get; set; }` - connection class
- `string Code { get; set; }` - connection code (max 20 chars)
- `ConnectionStatusEnum Status { get; }` - connection status (read-only)
- `Vector UpVector { get; set; }` - up direction vector
- `AutoDirectionTypeEnum AutoDirectionType { get; set; }` - auto direction type
- `PositionTypeEnum PositionType { get; set; }` - position type

**Methods:**
- `ModelObject GetPrimaryObject()` - get primary object
- `ArrayList GetSecondaryObjects()` - get secondary objects
- `bool SetPrimaryObject(ModelObject M)` - set primary object
- `bool SetSecondaryObject(ModelObject M)` - set single secondary
- `bool SetSecondaryObjects(ArrayList Secondaries)` - set multiple secondaries
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Detail : BaseComponent
> Represents a detail. A detail only connects to one part (unlike a connection).

**Constructors:** `Detail()`

**Properties:**
- `int Class { get; set; }` - detail class
- `string Code { get; set; }` - detail code (max 20 chars)
- `ConnectionStatusEnum Status { get; }` - detail status (read-only)
- `Vector UpVector { get; set; }` - up direction vector
- `AutoDirectionTypeEnum AutoDirectionType { get; set; }` - auto direction type
- `PositionTypeEnum PositionType { get; set; }` - position type
- `DetailTypeEnum DetailType { get; set; }` - detail type

**Methods:**
- `ModelObject GetPrimaryObject()` - get primary object
- `bool SetPrimaryObject(ModelObject M)` - set primary object
- `Point GetReferencePoint()` - get reference point
- `bool SetReferencePoint(Point ReferencePoint)` - set reference point
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Component : BaseComponent
> Represents a component (macro, custom part, or plug-in).

**Constructors:** `Component()`, `Component(ComponentInput I)`

**Methods:**
- `bool SetComponentInput(ComponentInput I)` - set component input
- `ComponentInput GetComponentInput()` - get component input
- `Assembly GetAssembly()` - get assembly
- `ModelObjectEnumerator GetBooleans()` - get boolean children
- `ModelObjectEnumerator GetComponents()` - get sub-components
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### ModelObjectSelector
> Provides methods to query model objects from the current model.

**Methods:**
- `ModelObjectEnumerator GetAllObjects()` - get all objects
- `ModelObjectEnumerator GetAllObjectsWithType(ModelObjectEnum Enum)` - get by type enum
- `ModelObjectEnumerator GetAllObjectsWithType(Type[] TypeFilter)` - get by .NET types
- `ModelObjectEnumerator GetFilteredObjectsWithType(ModelObjectEnum Enum, string FilterName)` - get filtered by type
- `ModelObjectEnumerator GetObjectsByBoundingBox(Point MinPoint, Point MaxPoint)` - get by bounding box
- `ModelObjectEnumerator GetObjectsByFilterName(string FilterName)` - get by filter name
- `ModelObjectEnumerator GetObjectsByFilter(FilterExpression FilterExpression)` - get by filter expression
- `ModelObjectEnumerator GetEnumerator()` - get all (same as GetAllObjects)

---

### ModelObjectEnumerator
> Provides means to iterate through model object instances.

**Properties:**
- `ModelObject Current { get; }` - current object (null if none left)
- `bool SelectInstances { get; set; }` - auto-call Select() on Current (default true)

**Methods:**
- `bool MoveNext()` - advance to next object
- `int GetSize()` - get count
- `void Reset()` - reset to beginning
- `IEnumerator GetEnumerator()` - get enumerator (for foreach in some scenarios)

```csharp
// Correct iteration pattern
var enumerator = selector.GetAllObjectsWithType(ModelObject.ModelObjectEnum.BEAM);
while (enumerator.MoveNext())
{
    var beam = enumerator.Current as Beam;
    if (beam != null)
    {
        // Properties are auto-selected if SelectInstances is true
    }
}
```

---

### Identifier
> Inherited from Object. The identifier uniquely identifies a model object in the database.

**Properties:**
- `int ID { get; set; }` - the numeric identifier

---

### NumberingSeries
> Describes how an object is to be numbered.

**Constructors:** `NumberingSeries()`, `NumberingSeries(string Prefix, int Number)`

**Properties:**
- `string Prefix { get; set; }` - prefix in numbering
- `int StartNumber { get; set; }` - start number

---

### Position
> Defines how a part is positioned relative to the input.

**Constructors:** `Position()`

**Properties:**
- `DepthEnum Depth { get; set; }` - MIDDLE, FRONT, or BEHIND
- `double DepthOffset { get; set; }` - offset from depth
- `PlaneEnum Plane { get; set; }` - MIDDLE, LEFT, or RIGHT
- `double PlaneOffset { get; set; }` - offset from plane
- `RotationEnum Rotation { get; set; }` - FRONT, TOP, BACK, or BELOW
- `double RotationOffset { get; set; }` - offset from rotation

---

### Phase
> Defines a model object phase.

**Constructors:** `Phase()`, `Phase(int PhaseNumber)`, `Phase(int PhaseNumber, string PhaseName, string PhaseComment, int IsCurrentPhase)`

**Properties:**
- `int PhaseNumber { get; set; }` - phase number
- `string PhaseName { get; set; }` - phase name
- `string PhaseComment { get; set; }` - phase comment
- `int IsCurrentPhase { get; set; }` - 1 if current, 0 otherwise

**Methods:**
- `Insert()`, `Modify()`, `Delete()`, `Select()`
- `bool GetUserProperty(string Name, ref int/string/double Value)`
- `bool SetUserProperty(string Name, int/string/double Value)`

---

### Task : ModelObject
> Defines a single building site task. May contain parts, assemblies, or subtasks.

**Constructors:** `Task()`, `Task(Identifier ID)`

**Properties:**
- `string Name { get; set; }` - task name
- `string Description { get; set; }` - task description
- `DateTime PlannedStartDate { get; set; }` - planned start
- `DateTime PlannedEndDate { get; set; }` - planned end
- `DateTime ActualStartDate { get; set; }` - actual start
- `DateTime ActualEndDate { get; set; }` - actual end
- `double PlannedWorkAmount { get; set; }` - planned work amount
- `double ActualWorkAmount { get; set; }` - actual work amount
- `int Completeness { get; set; }` - completion percentage (0-100)
- `bool Critical { get; set; }` - criticality flag
- `bool Local { get; set; }` - created locally or imported
- `string Url { get; set; }` - related link
- `HierarchicObject Scenario { get; set; }` - task scenario

**Methods:**
- `bool AddObjectsToTask(ArrayList ModelObjects)` - add objects
- `bool RemoveObjectsFromTask(ArrayList ModelObjects)` - remove objects
- `ModelObjectEnumerator GetDependencies()` - get dependencies
- `ModelObjectEnumerator GetFathers()` - get parent tasks
- `static ModelObjectEnumerator GetAllTasksOfSelectedObjects()` - get tasks for selected objects
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### Grid : GridBase
> Defines a grid with coordinate planes on X, Y, and Z axes.

**Constructors:** `Grid()`

**Properties (inherited from GridBase):**
- `string Name { get; set; }` - grid name
- `Point Origin { get; set; }` - grid origin
- `bool IsMagnetic { get; set; }` - whether magnetic
- `int FontSize { get; set; }` - label font size

**Own Properties:**
- `string CoordinateX { get; set; }` - X-axis coordinates (e.g., "0 3000 6000")
- `string CoordinateY { get; set; }` - Y-axis coordinates
- `string CoordinateZ { get; set; }` - Z-axis coordinates
- `string LabelX { get; set; }` - X-axis labels (e.g., "A B C")
- `string LabelY { get; set; }` - Y-axis labels
- `string LabelZ { get; set; }` - Z-axis labels
- `double ExtensionLeftX/Y/Z { get; set; }` - left extensions
- `double ExtensionRightX/Y/Z { get; set; }` - right extensions
- `double ExtensionForMagneticArea { get; set; }` - magnetic area extension
- `int Color { get; set; }` - grid color

**Methods:** `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### ReferenceModel : ModelObject
> Represents a reference to an external model (IFC, DWG, etc.).

**Constructors:** `ReferenceModel()`, `ReferenceModel(string filename, Point position, double scale)`

**Properties:**
- `string Filename { get; set; }` - path to original reference file
- `string ActiveFilePath { get; set; }` - path to local copy (read-only)
- `string Title { get; set; }` - reference model name
- `Point Position { get; set; }` - position in model
- `double Scale { get; set; }` - scale factor
- `double Rotation { get; set; }` - rotation around Z axis (degrees)
- `Rotation3D Rotation3D { get; set; }` - 3D rotation
- `VisibilityEnum Visibility { get; set; }` - HIDDEN or VISIBLE
- `bool UseWorkplane { get; set; }` - use workplane
- `Guid BasePointGuid { get; set; }` - base point GUID
- `Guid ModelGUID { get; set; }` - model GUID
- `Guid ProjectGUID { get; set; }` - project GUID

**Methods:**
- `ModelObjectEnumerator GetChildren()` - get child reference model objects
- `ModelObjectEnumerator GetConvertedObjects()` - get converted native objects
- `Revision GetCurrentRevision()` - get current revision
- `List<Revision> GetRevisions()` - get all revisions
- `bool RefreshFile()` - refresh from file
- `bool SetAsCurrentRevision(Revision revision)` - set current revision
- `ReferenceModelObject GetReferenceModelObjectByExternalGuid(string externalGuid)` - find by external GUID
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### TransformationPlane
> Describes a transformation from global model coordinates to local and back.

**Constructors:** `TransformationPlane()`, `TransformationPlane(CoordinateSystem CoordinateSystem)`, `TransformationPlane(Point Origo, Vector Xvector, Vector Yvector)`

**Properties:**
- `Matrix TransformationMatrixToGlobal { get; }` - matrix to convert local to global
- `Matrix TransformationMatrixToLocal { get; }` - matrix to convert global to local

```csharp
// Set to global coordinate system
var workPlaneHandler = model.GetWorkPlaneHandler();
var globalPlane = new TransformationPlane();
workPlaneHandler.SetCurrentTransformationPlane(globalPlane);
```

---

### WorkPlaneHandler
> Contains methods for getting and setting the current transformation plane.

**Methods:**
- `TransformationPlane GetCurrentTransformationPlane()` - get current plane
- `bool SetCurrentTransformationPlane(TransformationPlane TransformationPlane)` - set current plane

---

### ClashCheckHandler
> Contains clash check methods.

**Methods:**
- `bool RunClashCheck()` - run clash check
- `bool RunClashCheckWithOptions(bool betweenReferenceModels, bool objectsInsideReferenceModels, double minDistance, bool betweenParts)` - run with options
- `bool StopClashCheck()` - stop clash check
- `ArrayList GetIntersectionBoundingBoxes(Identifier ID1, Identifier ID2)` - get intersection boxes

---

### Solid
> Represents the physical geometry created by a part. Used to query geometry and intersect with lines/planes.

**Properties:**
- `Point MinimumPoint { get; }` - minimum axis-aligned point in current plane
- `Point MaximumPoint { get; }` - maximum axis-aligned point in current plane

**Methods:**
- `ArrayList Intersect(Point point1, Point point2)` - intersect with line (returns Points)
- `ArrayList Intersect(Point point1, Point point2, Point point3)` - intersect with plane
- `ArrayList Intersect(LineSegment line)` - intersect with line segment
- `FaceEnumerator GetFaceEnumerator()` - iterate faces
- `EdgeEnumerator GetEdgeEnumerator()` - iterate edges
- `ShellEnumerator GetCutPart(Solid CuttingPart)` - cut with another solid
- `bool IsValid()` - check validity

---

## Sub-namespace: Tekla.Structures.Model.UI

### UI.Picker
> Used to query the user to manually pick objects and points from the model. Methods throw an exception if user cancels.

**Constructors:** `Picker()`

**Methods:**
- `Point PickPoint()` - pick a single point
- `Point PickPoint(string Prompt)` - pick with prompt
- `Point PickPoint(string Prompt, Point ReferencePoint)` - pick with prompt and reference
- `ArrayList PickPoints(PickPointEnum Enum)` - pick multiple points
- `ArrayList PickPoints(PickPointEnum Enum, string Prompt)` - pick with prompt
- `ModelObject PickObject(PickObjectEnum Enum)` - pick a single object
- `ModelObject PickObject(PickObjectEnum Enum, string Prompt)` - pick with prompt
- `ModelObjectEnumerator PickObjects(PickObjectsEnum Enum)` - pick multiple objects
- `ModelObjectEnumerator PickObjects(PickObjectsEnum Enum, string Prompt)` - pick with prompt
- `PickInput PickFace()` - pick a face
- `PickInput PickFace(string Prompt)` - pick face with prompt
- `ArrayList PickLine()` - pick a line
- `ArrayList PickLine(string Prompt)` - pick line with prompt

---

### UI.ModelObjectSelector
> Used to select objects in the Tekla Structures user interface (highlights them visually).

**Constructors:** `ModelObjectSelector()`

**Methods:**
- `ModelObjectEnumerator GetSelectedObjects()` - get currently selected objects
- `bool Select(ArrayList ModelObjects)` - select objects in UI
- `bool Select(ArrayList ModelObjects, bool ShowDimensions)` - select with dimensions
- `ModelObjectEnumerator GetObjectsByBoundingBox(Point MinPoint, Point MaxPoint, View View)` - get by bounding box in view

---

### UI.ModelObjectVisualization (static)
> Set and clear temporary visualization (color/transparency) for model objects.

**Static Methods:**
- `bool SetTemporaryState(List<ModelObject> modelObjects, Color color)` - set temp color
- `bool SetTemporaryState(List<Identifier> identifiers, Color color)` - set temp color by IDs
- `bool SetTemporaryStateForAll(Color color)` - set temp color for all
- `bool ClearTemporaryStates(List<ModelObject> modelObjects)` - clear temp states
- `bool ClearAllTemporaryStates()` - clear all temp states
- `bool SetTransparency(List<ModelObject> modelObjects, TemporaryTransparency transparency)` - set transparency
- `bool SetTransparencyForAll(TemporaryTransparency transparency)` - set transparency for all
- `bool SetVisibility(List<ModelObject> modelObjects, bool? Visible)` - set visibility
- `void UnhideParts()` - unhide all parts
- `bool GetRepresentation(ModelObject modelObject, ref Color color)` - get permanent color
- `Color GetTempRepresentation(ModelObject modelObject)` - get temp color

---

### UI.GraphicsDrawer
> Draws temporary graphics in the currently active rendered view.

**Constructors:** `GraphicsDrawer()`

**Methods:**
- `bool DrawLineSegment(Point Point1, Point Point2, Color Color)` - draw line
- `bool DrawLineSegment(LineSegment LineSegment, Color Color)` - draw line segment
- `bool DrawText(Point Location, string Text, Color Color)` - draw text
- `int DrawTextToView(Point Location, string Text, Color Color, View View)` - draw text to specific view
- `int DrawPolyLine(GraphicPolyLine GraphicPolyLine)` - draw polyline (returns ID)
- `bool DrawMeshLines(Mesh Mesh, Color Color)` - draw mesh wireframe
- `bool DrawMeshSurface(Mesh Mesh, Color Color)` - draw mesh surface
- `void RemoveTemporaryGraphicsObject(int GraphicObjectID)` - remove by ID
- `void RemoveTemporaryGraphicsObjects(IEnumerable GraphicObjectIDs)` - remove multiple

---

### UI.View
> Contains methods related to views.

**Constructors:** `View()`

**Properties:**
- `string Name { get; set; }` - view name (max 84 chars)
- `Identifier Identifier { get; set; }` - view identifier
- `CoordinateSystem ViewCoordinateSystem { get; set; }` - view coordinate system
- `CoordinateSystem DisplayCoordinateSystem { get; set; }` - display coordinate system
- `double ViewDepthUp { get; set; }` - depth up
- `double ViewDepthDown { get; set; }` - depth down
- `AABB WorkArea { get; set; }` - working area
- `string ViewFilter { get; set; }` - view filter name (max 256 chars)
- `string CurrentRepresentation { get; set; }` - current representation
- `ViewProjectionType ViewProjection { get; set; }` - ORTHOGONAL or PERSPECTIVE
- `ViewRenderingType ViewRendering { get; }` - WIREFRAME or RENDERED (read-only)
- `DisplayOrientationType DisplayType { get; set; }` - display orientation
- `ViewVisibilitySettings VisibilitySettings { get; set; }` - object visibility settings
- `bool SharedView { get; set; }` - whether shared

**Methods:**
- `ClipPlaneCollection GetClipPlanes()` - get clip planes
- `bool IsVisible()` - check visibility
- `bool IsPerspectiveViewProjection()` - check projection type
- `bool CreateSnapshot(string filePath, SnapshotSettings settings, bool overwrite)` - create snapshot
- `Insert()`, `Modify()`, `Delete()`, `Select()`

---

### UI.ViewHandler (static methods)
> Handles view operations.

**Static Methods:**
- `View GetActiveView()` - get active view
- `ModelViewEnumerator GetAllViews()` - get all views
- `ModelViewEnumerator GetVisibleViews()` - get visible views
- `ModelViewEnumerator GetSelectedViews()` - get selected views
- `ModelViewEnumerator GetPermanentViews()` - get permanent views
- `ModelViewEnumerator GetTemporaryViews()` - get temporary views
- `bool ShowView(View view)` - show a view
- `bool HideView(View view)` - hide a view
- `bool RedrawView(View view)` - redraw a view
- `bool RedrawWorkplane()` - redraw work plane
- `bool SetRepresentation(string Representation)` - set representation
- `bool ZoomToBoundingBox(AABB box)` - zoom to bounding box
- `bool ZoomToBoundingBox(View view, AABB B)` - zoom in specific view

---

### UI.Color
> Represents an RGB color with transparency. Values between 0.0 and 1.0.

**Constructors:** `Color()`, `Color(double Red, double Green, double Blue)`, `Color(double Red, double Green, double Blue, double Transparency)`

**Properties:**
- `double Red { get; set; }` - 0.0 to 1.0
- `double Green { get; set; }` - 0.0 to 1.0
- `double Blue { get; set; }` - 0.0 to 1.0
- `double Transparency { get; set; }` - 0.0 (see-through) to 1.0 (solid)

---

## Sub-namespace: Tekla.Structures.Model.Operations

### Operation (static)
> Implements Tekla Structures level operations.

**Key Static Methods:**
- `bool MoveObject(ModelObject Object, Vector TranslationVector)` - move object
- `bool MoveObject(ModelObject Object, CoordinateSystem Start, CoordinateSystem End)` - move with coordinate systems
- `ModelObject CopyObject(ModelObject Object, Vector CopyVector)` - copy object
- `ModelObject CopyObject(ModelObject Object, CoordinateSystem Start, CoordinateSystem End)` - copy with coordinate systems
- `Beam Split(Beam Object, Point SplitPoint)` - split beam
- `PolyBeam Split(PolyBeam Object, Point SplitPoint)` - split polybeam
- `ContourPlate Split(ContourPlate Object, Polygon SplitPolygon)` - split contour plate
- `RebarGroup Split(RebarGroup Object, Line SplitLine)` - split rebar group
- `Beam Combine(Beam ObjectToCombineTo, Beam ObjectToBeCombined)` - combine beams
- `RebarGroup Combine(RebarGroup Obj1, RebarGroup Obj2)` - combine rebar groups
- `bool DisplayPrompt(string Message)` - display prompt in status bar
- `bool DisplayReport(string FileName)` - display report
- `bool CreateReportFromAll(string TemplateName, string FileName, string Title1, string Title2, string Title3)` - create report from all model objects. FileName should be a full path, typically under the model's Reports folder: `Path.Combine(model.GetInfo().ModelPath, "Reports", "output.xsr")`
- `bool CreateReportFromSelected(string TemplateName, string FileName, string Title1, string Title2, string Title3)` - create report from selected objects. FileName should be a full path, typically under the model's Reports folder: `Path.Combine(model.GetInfo().ModelPath, "Reports", "output.xsr")`
- `bool RunMacro(string FileName)` - run a macro
- `bool IsMacroRunning()` - check if macro is running
- `bool ObjectMatchesToFilter(ModelObject Object, string FilterName)` - check filter match
- `bool ObjectMatchesToFilter(ModelObject Object, FilterExpression FilterExpression)` - check filter expression match
- `bool IsNumberingUpToDate(ModelObject InputModelObject)` - check numbering status
- `bool IsNumberingUpToDateAll()` - check all numbering
- `bool IsNumberingAllowed()` - check if numbering is allowed
- `List<ModelObject> GetSimilarNumberedObjects(ModelObject ObjectToCompare)` - get similarly numbered objects
- `bool Highlight(List<ModelObject> ModelObjects)` - highlight objects
- `void ShowOnlySelected(UnselectedModeEnum UnselectedMode)` - show only selected (Hidden, Transparent, AsSticks)
- `bool ConvertPartToItem(ref Part originalPart, out Brep newItem)` - convert part to brep
- `string CreateShapeFromGeometry(Part originalPart)` - create shape from part geometry
- `BentPlate CreateBentPlateByParts(Part part1, Part part2)` - create bent plate from two parts
- `BentPlate CreateBentPlateByParts(Part part1, Part part2, double radius)` - with radius
- `BentPlate CreateBentPlateByFaces(Part part1, Face face1, Part part2, Face face2)` - create by faces
- `bool ExplodeBentPlate(BentPlate bentPlate)` - explode bent plate
- `bool CalculatePourUnits()` - calculate pour units
- `bool AddToPourUnit(PourUnit inputPourUnit, List<ModelObject> objectsToBeAdded)` - add to pour unit
- `bool RemoveFromPourUnit(List<ModelObject> objectsToBeRemoved)` - remove from pour unit
- `RebarGroup Group(IEnumerable RebarList)` - group rebars
- `ModelObjectEnumerator Ungrouping(RebarGroup Reinforcement)` - ungroup rebar group
- `bool CreateNCFilesFromAll(string Settings, string Folder)` - create NC files
- `bool CreateNCFilesFromSelected(string Settings, string Folder)` - create NC files from selected
- `bool CreateIFC4ExportFromAll(string fullPath, IFCExportViewTypeEnum viewType, IEnumerable<string> propertySets, ExportBasePoint location, string exportLayers, string coloring, IFCExportFlags flags, string basePointGuid)` - IFC4 export
- `bool CreateMISFileFromAll(MISExportTypeEnum MISType, string FileName)` - MIS export (DSTV, KISS, etc.)
- `List<Point> GetHandlePoints(string guid)` - get handle points
- `bool SetHandlePoints(string guid, List<Point> HandlePoints)` - set handle points

---

## Sub-namespace: Tekla.Structures.Model.Events

### Events : MarshalByRefObject
> Allows the user to register event listeners for model events.

**Constructors:** `Events()`

**Methods:**
- `void Register()` - register event listeners
- `void UnRegister()` - unregister event listeners
- `void Dispose()` - dispose

**Key Delegate Events (assign before Register):**
- `SelectionChangeDelegate` - `void Invoke()` - selection changed
- `ModelChangedDelegate` - `void Invoke()` - database committed
- `ModelObjectChangedDelegate` - `void Invoke(List<ChangeData> Changes)` - objects changed with details
- `ModelObjectNumberedDelegate` - `void Invoke(List<ModelObject> Objects)` - objects numbered
- `ModelLoadDelegate` - `void Invoke()` - model loaded
- `ModelLoadInfoDelegate` - `void Invoke(string ModelLoadInfo)` - model loaded with info
- `ModelSaveDelegate` - `void Invoke()` - model saved
- `ModelSaveAsDelegate` - `void Invoke()` - model saved as
- `ModelSaveInfoDelegate` - `void Invoke(string ModelSaveInfo)` - model saved with info
- `TeklaStructuresExitDelegate` - `void Invoke()` - Tekla exiting
- `NumberingDelegate` - `void Invoke()` - numbering event
- `ClashDetectedDelegate` - `void Invoke(ClashCheckData ClashData)` - clash detected
- `ClashCheckDoneDelegate` - `void Invoke(int NumbersOfClashes)` - clash check done
- `CommandStatusChangeDelegate` - `void Invoke(string TSCommand, string TSCommandParam, bool Status)` - command status change
- `InterruptedDelegate` - `void Invoke()` - interrupted
- `ProjectInfoChangedDelegate` - `void Invoke()` - project info changed
- `ViewCameraChangedDelegate` - `void Invoke(Identifier ViewId)` - view camera changed
- `ViewClosedDelegate` - `void Invoke(int viewId)` - view closed
- `HiddenObjectsChangedDelegate` - `void Invoke()` - hidden objects changed
- `TemporaryStatesChangedDelegate` - `void Invoke()` - temporary states changed
- `UndoClickedDelegate` - `void Invoke()` - undo clicked
- `ClipPlaneChangedDelegate` - `void Invoke(int viewId, int clipPlaneId, ChangeTypeEnum operation)` - clip plane changed

---

## Sub-namespace: Tekla.Structures.Model.History

### ModelHistory (static)
> Provides history information about model objects.

**Static Methods:**
- `ModificationStamp GetCurrentModificationStamp()` - get current stamp
- `ModelObjectEnumerator GetModifiedObjects(ModificationStamp ModStamp)` - get modified objects since stamp
- `ModelObjectEnumerator GetDeletedObjects(ModificationStamp ModStamp)` - get deleted objects since stamp
- `ModelObjectEnumerator GetModifiedObjectsWithType(ModificationStamp ModStamp, ModelObjectEnum Enum)` - get modified by type
- `ModelObjectEnumerator GetDeletedObjectsWithType(ModificationStamp ModStamp, ModelObjectEnum Enum)` - get deleted by type
- `ModelObjectEnumerator GetNotSharedObjects()` - get not-shared objects
- `ModificationInfo GetModifications(string Name, ModificationStamp PrevStamp)` - get modifications
- `ModificationInfo GetLocalChanges()` - get local changes

---

## Enumerations

### ModelObject.ModelObjectEnum
Used with `GetAllObjectsWithType()` to filter object types:
```
UNKNOWN = 0, BEAM = 1, POLYBEAM = 2, CONTOURPLATE = 3,
BOOLEANPART = 4, FITTING = 5, CUTPLANE = 6, SURFACE_TREATMENT = 7,
WELD = 8, ASSEMBLY = 9, SINGLEREBAR = 10, REBARGROUP = 11,
REBARMESH = 12, REBARSTRAND = 13, CONTROL_PLANE = 14,
BOLT_ARRAY = 15, BOLT_CIRCLE = 16, BOLT_XYLIST = 17,
LOAD_POINT = 18, LOAD_LINE = 19, LOAD_AREA = 20, LOAD_UNIFORM = 21,
GRID = 22, GRIDPLANE = 23, CONNECTION = 24, COMPONENT = 25,
SEAM = 26, DETAIL = 27, REFERENCE_MODEL = 28, REBAR_SPLICE = 29,
LOAD_GROUP = 30, TASK = 31, TASK_DEPENDENCY = 32, TASK_WORKTYPE = 34,
POLYGON_WELD = 35, LOGICAL_WELD = 36, CIRCLEREBAR = 37,
HIERARCHIC_DEFINITION = 38, HIERARCHIC_OBJECT = 39,
CUSTOM_PART = 43, CIRCLE_REBARGROUP = 44, CURVED_REBARGROUP = 45,
EDGE_CHAMFER = 46, POUR_OBJECT = 47, POUR_BREAK = 48,
CONTROL_LINE = 50, LOAD_TEMPERATURE = 51, BREP = 52,
CONTROL_CIRCLE = 53, CONTROL_POINT = 54, REBAR_SET = 55,
SURFACE_OBJECT = 60, BENT_PLATE = 61, SPIRAL_BEAM = 62,
POUR_UNIT = 63, CONTROL_ARC = 64, CONTROL_POLYCURVE = 66,
RADIAL_GRID = 67, LOFTED_PLATE = 69, STOREY = 71
```

### Beam.BeamTypeEnum
```
BEAM = 0, PANEL = 1, STRIP_FOOTING = 2, PAD_FOOTING = 3, COLUMN = 4
```

### ContourPlate.ContourPlateTypeEnum
```
UNKNOWN = 0, PLATE = 1, SLAB = 2
```

### PolyBeam.PolyBeamTypeEnum
```
BEAM = 0, PANEL = 1, STRIP_FOOTING = 2, COLUMN = 3
```

### Part.CastUnitTypeEnum
```
PRECAST = 0, CAST_IN_PLACE = 1
```

### Position.DepthEnum
```
MIDDLE = 0, FRONT = 1, BEHIND = 2
```

### Position.PlaneEnum
```
MIDDLE = 0, LEFT = 1, RIGHT = 2
```

### Position.RotationEnum
```
FRONT = 0, TOP = 1, BACK = 2, BELOW = 3
```

### BooleanPart.BooleanTypeEnum
```
BOOLEAN_ADD = 1, BOOLEAN_CUT = 2, BOOLEAN_WELDPREP = 3
```

### Assembly.AssemblyTypeEnum
```
STEEL_ASSEMBLY = 0, PRECAST_ASSEMBLY = 1, IN_SITU_ASSEMBLY = 2,
TIMBER_ASSEMBLY = 3, UNKNOWN_ASSEMBLY = 4, REBAR_ASSEMBLY = 5
```

### BoltGroup.BoltTypeEnum
```
BOLT_TYPE_SITE = 0, BOLT_TYPE_WORKSHOP = 1
```

### BoltGroup.BoltHoleTypeEnum
```
HOLE_TYPE_OVERSIZED = 0, HOLE_TYPE_SLOTTED = 1, HOLE_TYPE_NO_HOLE = 2, HOLE_TYPE_TAPPED = 3
```

### BoltGroup.BoltThreadInMaterialEnum
```
THREAD_IN_MATERIAL_NO = 0, THREAD_IN_MATERIAL_YES = 1
```

### BoltGroup.BoltRotateSlotsEnum
```
ROTATE_SLOTS_ODD = 0, ROTATE_SLOTS_EVEN = 1, ROTATE_SLOTS_PARALLEL = 2
```

### BaseWeld.WeldTypeEnum
```
WELD_TYPE_NONE = 0, WELD_TYPE_EDGE_FLANGE = 1,
WELD_TYPE_SQUARE_GROOVE_SQUARE_BUTT = 2,
WELD_TYPE_BEVEL_GROOVE_SINGLE_V_BUTT = 3,
WELD_TYPE_BEVEL_GROOVE_SINGLE_BEVEL_BUTT = 4,
WELD_TYPE_SINGLE_V_BUTT_WITH_BROAD_ROOT_FACE = 5,
WELD_TYPE_SINGLE_BEVEL_BUTT_WITH_BROAD_ROOT_FACE = 6,
WELD_TYPE_U_GROOVE_SINGLE_U_BUTT = 7,
WELD_TYPE_J_GROOVE_J_BUTT = 8, WELD_TYPE_BEVEL_BACKING = 9,
WELD_TYPE_FILLET = 10, WELD_TYPE_PLUG = 11, WELD_TYPE_SPOT = 12,
WELD_TYPE_SEAM = 13, WELD_TYPE_SLOT = 14,
WELD_TYPE_FLARE_BEVEL_GROOVE = 15, WELD_TYPE_FLARE_V_GROOVE = 16,
WELD_TYPE_CORNER_FLANGE = 17,
WELD_TYPE_PARTIAL_PENETRATION_SINGLE_BEVEL_BUTT_PLUS_FILLET = 18,
WELD_TYPE_PARTIAL_PENETRATION_SQUARE_GROOVE_PLUS_FILLET = 19,
WELD_TYPE_MELT_THROUGH = 20,
STEEP_FLANKED_BEVEL_GROOVE_SINGLE_V_BUTT = 21,
STEEP_FLANKED_BEVEL_GROOVE_SINGLE_BEVEL_BUTT = 22,
WELD_TYPE_EDGE = 23, WELD_TYPE_ISO_SURFACING = 24,
WELD_TYPE_FOLD = 25, WELD_TYPE_INCLINED = 26
```

### BaseWeld.WeldContourEnum
```
WELD_CONTOUR_NONE = 0, WELD_CONTOUR_FLUSH = 1,
WELD_CONTOUR_CONVEX = 2, WELD_CONTOUR_CONCAVE = 3
```

### BaseWeld.WeldFinishEnum
```
WELD_FINISH_NONE = 0, WELD_FINISH_GRIND = 1, WELD_FINISH_MACHINE = 2,
WELD_FINISH_CHIP = 3, WELD_FINISH_FINISHED_WELD = 4,
WELS_FINISH_SMOOTH_TRANSITION = 5
```

### BaseWeld.WeldIntermittentTypeEnum
```
CONTINUOUS = 0, CHAIN_INTERMITTENT = 1, STAGGERED_INTERMITTENT = 2
```

### BaseWeld.WeldProcessTypeEnum
```
WELD_PROCESS_NONE = 0, WELD_PROCESS_SMAW = 1, WELD_PROCESS_SAW = 2,
WELD_PROCESS_GMAW = 3, WELD_PROCESS_FCAW = 4, WELD_PROCESS_ESW = 5,
WELD_PROCESS_EGW = 6
```

### BaseWeld.WeldPlacementTypeEnum
```
PLACEMENT_AUTO = 0, PLACEMENT_MAIN = 1, PLACEMENT_SECONDARY = 2
```

### BaseWeld.WeldPreparationTypeEnum
```
PREPARATION_NONE = 0, PREPARATION_AUTO = 1, PREPARATION_MAIN = 2,
PREPARATION_SECONDARY = 3
```

### BaseWeld.WeldNDTInspectionEnum
```
WELD_NDT_INSPECTION_NONE = 0, WELD_NDT_INSPECTION_A = 1,
WELD_NDT_INSPECTION_B = 2, WELD_NDT_INSPECTION_C = 3,
WELD_NDT_INSPECTION_D = 4, WELD_NDT_INSPECTION_E = 5
```

### BaseWeld.WeldElectrodeClassificationEnum
```
WELD_ELECTRODE_CLASSIFICATION_NONE = 0,
WELD_ELECTRODE_CLASSIFICATION_35 = 1, _42 = 2, _50 = 3,
_E60XX = 4, _E70XX = 5, _E80XX = 6, _E90XX = 7
```

### Weld.WeldPositionEnum
```
WELD_POSITION_PLUS_X = 1, WELD_POSITION_MINUS_X = 2,
WELD_POSITION_PLUS_Y = 3, WELD_POSITION_MINUS_Y = 4,
WELD_POSITION_PLUS_Z = 5, WELD_POSITION_MINUS_Z = 6
```

### Chamfer.ChamferTypeEnum
```
CHAMFER_NONE = 0, CHAMFER_LINE = 1, CHAMFER_ROUNDING = 2,
CHAMFER_ARC = 3, CHAMFER_ARC_POINT = 4, CHAMFER_SQUARE = 5,
CHAMFER_SQUARE_PARALLEL = 6, CHAMFER_LINE_AND_ARC = 7
```

### Reinforcement.RebarOffsetTypeEnum
```
OFFSET_TYPE_COVER_THICKNESS = 0, OFFSET_TYPE_LEG_LENGTH = 1
```

### BaseRebarGroup.RebarGroupSpacingTypeEnum
```
SPACING_TYPE_UNDEFINED = 0, SPACING_TYPE_EXACT_SPACINGS = 1,
SPACING_TYPE_EXACT_NUMBER = 2, SPACING_TYPE_TARGET_SPACE = 3,
SPACING_TYPE_EXACT_SPACE_FLEX_AT_START = 4,
SPACING_TYPE_EXACT_SPACE_FLEX_AT_END = 5,
SPACING_TYPE_EXACT_SPACE_FLEX_AT_BOTH = 6,
SPACING_TYPE_EXACT_SPACE_FLEX_AT_MIDDLE = 7
```

### BaseRebarGroup.ExcludeTypeEnum
```
EXCLUDE_TYPE_NONE = 1, EXCLUDE_TYPE_FIRST = 2,
EXCLUDE_TYPE_LAST = 3, EXCLUDE_TYPE_BOTH = 4
```

### RebarGroup.RebarGroupStirrupTypeEnum
```
STIRRUP_TYPE_POLYGONAL = 0, STIRRUP_TYPE_SPIRAL = 1,
STIRRUP_TYPE_TAPERED_CURVED = 2
```

### RebarHookData.RebarHookShapeEnum
```
NO_HOOK = 0, HOOK_90_DEGREES = 1, HOOK_135_DEGREES = 2,
HOOK_180_DEGREES = 3, CUSTOM_HOOK = 4
```

### RebarSplice.RebarSpliceTypeEnum
```
SPLICE_TYPE_LAP_RIGHT = 0, SPLICE_TYPE_LAP_LEFT = 1,
SPLICE_TYPE_LAP_BOTH = 2, SPLICE_TYPE_MUFF = 3, SPLICE_TYPE_WELD = 4
```

### RebarMesh.RebarMeshTypeEnum
```
UNKNOWN_MESH = 0, RECTANGULAR_MESH = 1, POLYGON_MESH = 2, BENT_MESH = 3
```

### Solid.SolidCreationTypeEnum
```
RAW = 0, FITTED = 1, NORMAL = 2, HIGH_ACCURACY = 3,
PLANECUTTED = 4, NORMAL_WITHOUT_EDGECHAMFERS = 5,
NORMAL_WITHOUT_WELDPREPS = 6
```

### Picker.PickObjectEnum
```
PICK_ONE_OBJECT = 0, PICK_ONE_PART = 1, PICK_ONE_WELD = 2,
PICK_ONE_BOLTGROUP = 3, PICK_ONE_REINFORCEMENT = 4
```

### Picker.PickObjectsEnum
```
PICK_N_OBJECTS = 0, PICK_N_PARTS = 1, PICK_N_WELDS = 2,
PICK_N_BOLTGROUPS = 3, PICK_N_REINFORCEMENTS = 4
```

### Picker.PickPointEnum
```
PICK_ONE_POINT = 0, PICK_TWO_POINTS = 1, PICK_POLYGON = 2,
PICK_LINE = 3, PICK_FACE = 4
```

### ControlObjectLineType
```
SolidLine = 1, DashedLine = 2, SlashedLine = 3, DashDot = 4, DottedLine = 5
```

### ControlObjectColorEnum / ControlLineColorEnum / ControlCircleColorEnum
```
BLACK = 0, WHITE = 1, RED = 2, GREEN = 3, BLUE = 4,
CYAN = 5, YELLOW = 6, MAGENTA = 7, YELLOW_RED = -1
```

### TemporaryTransparency
```
HIDDEN = 0, TRANSPARENT = 1, SEMITRANSPARENT = 3,
SEMIVISIBLE = 5, VISIBLE = 10
```

### View.ViewProjectionType
```
ORTHOGONAL_PROJECTION = 0, PERSPECTIVE_PROJECTION = 1
```

### View.ViewRenderingType
```
WIREFRAME_VIEW = 0, RENDERED_VIEW = 1
```

### View.DisplayOrientationType
```
DISPLAY_VIEW_PLANE = 0, DISPLAY_3D = 1
```

### ChangeData.ChangeTypeEnum
```
OBJECT_INSERT = 0, OBJECT_MODIFY = 1, OBJECT_DELETE = 2,
USERPROPERTY_CHANGED = 3
```

### ChangeData.ChangeSourceTypeEnum
```
COMMIT = 0, UNDO_REDO = 1, ROLLBACK = 2
```

### ReferenceModel.VisibilityEnum
```
HIDDEN = 0, VISIBLE = 1
```

### TaskDependency.DependencyTypeEnum
```
FINISH_TO_FINISH = 0, FINISH_TO_START = 1,
START_TO_FINISH = 2, START_TO_START = 3
```

### Operation.UnselectedModeEnum
```
Hidden = 0, Transparent = 1, AsSticks = 2
```

### Operation.MISExportTypeEnum
```
DSTV = 0, KISS = 1, EJE = 2, EPC = 3, STEEL2000 = 4
```

### Operation.IFCExportViewTypeEnum
```
UNDEFINED = 0, REFERENCE_VIEW = 1, DESIGN_TRANSFER_VIEW = 2,
PRECAST_VIEW = 3, MEP_REFERENCE_VIEW = 4,
MEP_DESIGN_TRANSFER_VIEW = 5, BRIDGE_VIEW = 6
```

### GraphicPolyLine.LineType
```
Solid = 1, Dashed1 = 2, Dashed2 = 3, DashedAndDotted = 4, Dotted = 5
```

### BentPlate.BendShape
```
Cylindrical = 0, Conical = 1
```

### LoftedPlate.LoftedPlateFaceTypeEnum
```
Perpendicular = 0, BoundedByCurvePlanes = 1
```

### SurfaceTreatment.SurfaceTypeEnum
```
CONCRETE_FINISH = 1, SPECIAL_MIX = 2, TILE_SURFACE = 3, STEEL_FINISH = 4
```

### SurfaceTreatment.SurfaceColorEnum
```
WHITE = 1, RED = 2, GREEN = 3, BLUE = 4, CYAN = 5, YELLOW = 6, MAGENTA = 7
```

### HierarchicDefinitionTypeEnum
```
DOT_HIERARCHIC_CUSTOM_TYPE = 0, DOT_HIERARCHIC_LOGICAL_BUILDING_AREA = 1,
DOT_HIERARCHIC_OBJECT_TYPE = 2, DOT_HIERARCHIC_TASK_WORK_TYPE = 3,
DOT_HIERARCHIC_TASK_SCENARIO = 4
```

### ClashCheckData.ClashTypeEnum
```
CLASH_TYPE_INVALID = 0, CLASH_TYPE_ISINSIDE = 1, CLASH_TYPE_EXACTMATCH = 2,
CLASH_TYPE_CLASH = 3, CLASH_TYPE_MINDISTANCE = 4, CLASH_TYPE_FAILEDSOLID = 5,
CLASH_TYPE_CUTTHROUGH = 6, CLASH_TYPE_COMPLEXCLASH = 7, CLASH_TYPE_FAILEDTEST = 8
```

### EdgeChamfer.ChamferEndTypeEnum
```
FULL = 0, STRAIGHT = 1, BEVELLED = 2
```

### BasePoint.CoordinateSystemType
```
GLOBAL = 1, WORKPLANE = 2
```

---

## Type Reference (Tier 2)

| Type | Base | Description |
|------|------|-------------|
| `BoltCircle` | `BoltGroup` | Bolt group with circle shape. Props: `Diameter`, `NumberOfBolts` |
| `BoltXYList` | `BoltGroup` | Generic bolt group with XY list shape |
| `BoltHoleAttributes` | - | Bolt hole attributes: `HoleType`, `LongHoleX/Y`, `SlotOffsetX/Y` |
| `LogicalWeld` | `BaseWeld` | Group of welds. Methods: `AddWeld`, `RemoveWeld`, `GetMainWeld`, `Explode` |
| `SingleRebar` | `Reinforcement` | Single reinforcing bar. Props: `Polygon`, `Size`, `StartHook`, `EndHook` |
| `CurvedRebarGroup` | `BaseRebarGroup` | Curved rebar group. Polygon must have 3 points |
| `RebarMesh` | `Reinforcement` | Reinforcement mesh. Props: `MeshType`, `LongitudinalSize`, `CrossSize`, `Length`, `Width` |
| `RebarStrand` | `Reinforcement` | Prestressed strands. Props: `StartPoint`, `EndPoint`, `PullPerStrand`, `Patterns` |
| `RebarSet` | `ModelObject` | Set of reinforcing bars with guidelines and leg faces |
| `RebarSetAddition` | `ModelObject` | Addition to a rebar set |
| `RebarSplice` | `ModelObject` | Splice between two reinforcements |
| `RebarSplitter` | `BaseRebarModifier` | Splits rebars with lapping or cranking |
| `RebarPropertyModifier` | `BaseRebarModifier` | Modifies rebar properties and spacing |
| `RebarEndDetailModifier` | `BaseRebarModifier` | Modifies rebar end details (hooks/cranking) |
| `RebarHookData` | - | Hook definition: `Shape`, `Angle`, `Length`, `Radius` |
| `RebarGeometry` | - | Physical bar geometry: `Shape` (PolyLine), `Diameter`, `BendingRadiuses` |
| `RebarComplexGeometry` | - | Complex bar geometry with legs and bending radiuses |
| `RebarSpacing` | - | Spacing definition for rebar sets |
| `RebarProperties` | - | Rebar properties: `Name`, `Size`, `Grade`, `Class`, `BendingRadius` |
| `RebarGuideline` | - | Guideline for rebar sets with spacing |
| `RebarLegFace` | - | Leg face for rebar sets with contour |
| `RebarCranking` | - | Cranking data for rebar splitters |
| `RebarLapping` | - | Lapping data for rebar splitters |
| `Seam` | `BaseComponent` | Connects a main part to others along an edge |
| `CustomPart` | `BaseComponent` | Custom part object (tapered beam, sandwich panel, etc.) |
| `EdgeChamfer` | `Boolean` | Chamfers a part edge |
| `SpiralBeam` | `Part` | Spiral/helix beam with rotation parameters |
| `LoftedPlate` | `Part` | Lofted plate defined by base curves |
| `Contour` | - | Possibly chamfered contour. Props: `ContourPoints` (ArrayList of ContourPoint) |
| `ContourPoint` | `Point` | Point with chamfering info. Props: `Chamfer` |
| `Chamfer` | - | Corner chamfer: `Type`, `X`, `Y` |
| `Polygon` | - | Simple polygon. Props: `Points` (ArrayList of Point) |
| `Material` | - | Part material. Props: `MaterialString` |
| `Profile` | - | Part profile. Props: `ProfileString`. Static: `FormatProfileString`, `ParseProfileString` |
| `Offset` | - | Offset distances: `Dx`, `Dy`, `Dz` |
| `Plane` | - | Plane definition: `Origin`, `AxisX`, `AxisY` |
| `ModelInfo` | - | Model info: `ModelName`, `ModelPath`, `CurrentPhase`, `NorthDirection`, `SharedModel` |
| `ProjectInfo` | - | Project info: `Name`, `ProjectNumber`, `Builder`, `Designer`, `Address`, etc. |
| `ModelHandler` | - | Model file operations: `Open`, `Save`, `Close`, `CreateNewSingleUserModel` |
| `DeformingData` | - | Deforming data: `Cambering`, `Shortening`, `Angle`, `Angle2` |
| `ComponentInput` | - | Component input: `AddInputObject`, `AddOneInputPosition`, `AddTwoInputPositions`, `AddInputPolygon` |
| `ConnectiveGeometry` | - | Bent plate geometry with sections |
| `BentPlateGeometrySolver` | - | Solver for bent plate geometry modifications |
| `GridPlane` | `GridSurface` | Individual grid plane with extensions |
| `RadialGrid` | `GridBase` | Radial grid with angular and radial coordinates |
| `GridCylindricalSurface` | `GridSurface` | Cylindrical grid surface |
| `SurfaceTreatment` | `ModelObject` | Surface layer (tiles, finish, etc.) |
| `SurfaceObject` | `ModelObject` | Surface object with polymesh geometry |
| `PourObject` | `ModelObject` | Pour object in the model |
| `PourUnit` | `ModelObject` | Pour unit in the model |
| `PourBreak` | `ModelObject` | Pour break in the model |
| `ControlLine` | `ModelObject` | Construction line (possibly magnetic) |
| `ControlPlane` | `ModelObject` | Construction plane (possibly magnetic) |
| `ControlPoint` | `ModelObject` | Construction point |
| `ControlCircle` | `ModelObject` | Construction circle |
| `ControlArc` | `ModelObject` | Construction arc |
| `ControlPolycurve` | `ModelObject` | Construction polycurve |
| `TaskDependency` | `ModelObject` | Dependency between tasks |
| `TaskWorktype` | `ModelObject` | Worktype for tasks |
| `HierarchicDefinition` | `ModelObject` | Hierarchy structure definition |
| `HierarchicObject` | `ModelObject` | Object in a hierarchy |
| `LoadGroup` | `ModelObject` | Load group (color, type, direction) |
| `LoadPoint` | `Load` | Concentrated force/moment |
| `LoadLine` | `Load` | Line-distributed force/torsion |
| `LoadArea` | `Load` | Area load (triangle/quadrangle) |
| `LoadUniform` | `Load` | Uniform load bounded by polygon |
| `LoadTemperature` | `Load` | Temperature load |
| `ReferenceModelObject` | `ModelObject` | Sub-object of a reference model |
| `PointCloud` | - | Point cloud operations |
| `BasePoint` | - | Base point for coordinate systems |
| `PhaseCollection` | - | Collection of model phases |
| `ChangeData` | - | Change info: `Object`, `Type` (INSERT/MODIFY/DELETE), `Source` |
| `ClashCheckData` | - | Clash info: `Object1`, `Object2`, `Overlap`, `Type` |
| `Solid` | - | Part solid geometry for intersection queries |
| `Polymesh` | - | Polygon mesh with validation |
| `InputItem` | - | Stores component input data |
| `GuidConversion` | - | Converts old TS GUIDs to current GUIDs |
| `DisposableToken` | - | Disposable action token |
| `UI.ClipPlane` | - | Clip plane for views |
| `UI.ViewCamera` | - | View camera (location, direction, zoom) |
| `UI.ViewHandler` | - | Static view operations |
| `UI.ViewVisibilitySettings` | - | Object visibility per view |
| `UI.GraphicPolyLine` | - | Styled polyline for drawing |
| `UI.Mesh` | - | 3D mesh for drawing |
| `UI.ModelViewEnumerator` | - | Enumerator for views |
| `UI.PickInput` | - | Collection of picked items |
| `History.ModificationStamp` | - | Modification stamp (LocalStamp, ServerStamp) |
| `History.ModificationInfo` | - | Modification info from history |
| `Collaboration.ReferenceModelObjectAttribute` | - | Base for IFC profile attributes |

---

## Interfaces

| Interface | Description |
|-----------|-------------|
| `IAssemblable` | Objects that can belong to an assembly. Method: `GetAssembly()` |
| `IGeometryNode` | Geometry tree node. Methods: `AcceptVisitor()`, `Clone()`. Prop: `IsAutomatic` |
| `IGeometryNodeVisitor` | Visitor for geometry nodes. Methods: `Visit(SpiralNode)`, `Visit(CylindricalSurfaceNode)`, `Visit(PolygonNode)` |
