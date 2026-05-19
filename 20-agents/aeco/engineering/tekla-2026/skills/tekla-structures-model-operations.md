---
name: tekla-tekla-structures-model-operations
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Model.Operations namespace — 13 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GuidConversion, Operation, Operation.ProgressBar, Operation.ReportParameters, Operation.IFC2x3ExportFlags, Operation.IFCExportFlags, Operation.IFC2x3ExportViewTypeEnum, Operation.ExportBasePoint, and 5 more types.
---

# Tekla.Structures.Model.Operations

Auto-generated from vendor docs for tekla 2026.0. 13 types in this namespace.

## GuidConversion (class)

Conversion of old TS GUIDs to current GUIDs. GUIDs are changed in TS save as operation, this class can be used to convert old GUIDs to current GUIDs. To recognize the need for GUID conversion, application needs to save project GUID and compare to the current project GUID. Note: With big models the instance uses a lot of memory.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/920e2903-8177-35de-c9d3-ba5e095fec10)

### Constructors
- `public GuidConversion()` — Initializes a new instance of the GuidConversion class.

### Methods
#### `public Dictionary<Guid, Guid> GetGuidMapping()`

Gets the GUID mapping.

**Returns:** `Dictionary<Guid,Guid>` — The GUID mapping

[Docs](https://developer.tekla.com/topic/en/18/47/53a1428d-6004-3a22-ae05-3736cf718674)

#### `public Guid GetNewGuid(Guid oldGuid)`

Gets the new GUID.

**Parameters:**
- `oldGuid` (System.Guid) — The old GUID.

**Returns:** `Guid` — The current GUID

[Docs](https://developer.tekla.com/topic/en/18/47/772a3a7a-8ebb-fbd1-7e80-885d4127556d)

## Operation (class)

The Operation class implements Tekla Structures level operations.

**Remarks:** Using this method is much faster than checking each object individually.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d5bbc096-a164-49ea-16a8-3820e40b00be)

### Methods
#### `public static bool AddToPourUnit(PourUnit inputPourUnit, List<ModelObject> objectsToBeAdded)`

Adds model objects as part of a pour unit Model object types accepted are assembly types except cast in situ, reinforcements of different kind, components and bolts

**Parameters:**
- `inputPourUnit` (Tekla.Structures.Model.PourUnit) — the instance of pour unit to add objects to.
- `objectsToBeAdded` (System.Collections.Generic.List<ModelObject>) — the list of model objects to be added.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/e0d977f1-f6d9-4b5d-09ff-39534452abbd)

#### `public static bool CalculatePourUnits()`

Calculate and assign objects to pour unit Model object types that are associated with pour unit are assembly types except cast in situ, reinforcements of different kind, components and screws

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/90216a45-57e6-4bd6-6338-f18371c6b2c2)

#### `public static Beam Combine(Beam ObjectToCombineTo, Beam ObjectToBeCombined)`

Combines two beams into one beam.

**Parameters:**
- `ObjectToCombineTo` (Tekla.Structures.Model.Beam) — The beam to be combined to.
- `ObjectToBeCombined` (Tekla.Structures.Model.Beam) — The beam which will be deleted after a successful operation.

**Returns:** `Beam` — The combined beam on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/ee549780-0d63-26ba-456a-49cfdb50da7b)

#### `public static RebarGroup Combine(RebarGroup ObjectToCombineTo, RebarGroup ObjectToBeCombined)`

Combines two rebar groups into one rebar group.

**Parameters:**
- `ObjectToCombineTo` (Tekla.Structures.Model.RebarGroup) — The rebar group to be combined to.
- `ObjectToBeCombined` (Tekla.Structures.Model.RebarGroup) — The rebar group which will be deleted after a successful operation.

**Returns:** `RebarGroup` — The combined rebar group on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/3147d034-cbbc-cd89-46ee-142d4658e591)

#### `public static SingleRebar Combine(SingleRebar ObjectToCombineTo, SingleRebar ObjectToBeCombined)`

Combines two single rebars into one rebar.

**Parameters:**
- `ObjectToCombineTo` (Tekla.Structures.Model.SingleRebar) — The rebar to be combined to.
- `ObjectToBeCombined` (Tekla.Structures.Model.SingleRebar) — The rebar which will be deleted after a successful operation.

**Returns:** `SingleRebar` — The combined single rebar on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/102fb25d-b5f4-6506-136d-36f88a02eb85)

#### `public static bool ConvertPartToItem(in Part originalPart, out Brep newItem)`

Convert a part to an item. This will also create a new shape in the shape catalog.

**Parameters:**
- `originalPart` (Tekla.Structures.Model.Part.) — Original part
- `newItem` (Tekla.Structures.Model.Brep.) — Newly created item

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/92da650f-fa05-8dc5-d313-ca063058634d)

#### `public static ModelObject CopyObject(ModelObject Object, CoordinateSystem StartCoordinateSystem, CoordinateSystem EndCoordinateSystem)`

Copies the model object between the given translation coordinate systems.

**Parameters:**
- `Object` (Tekla.Structures.Model.ModelObject) — The model object to copy.
- `StartCoordinateSystem` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to copy the object from.
- `EndCoordinateSystem` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to copy the object to.

**Returns:** `ModelObject` — The copied model object on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/d67c7649-00ef-f48a-bbf8-5bedf53ce30d)

#### `public static ModelObject CopyObject(ModelObject Object, Vector CopyVector)`

Copies the model object using the given translation vector.

**Parameters:**
- `Object` (Tekla.Structures.Model.ModelObject) — The model object to copy.
- `CopyVector` (Tekla.Structures.Geometry3d.Vector) — The translation vector for copying.

**Returns:** `ModelObject` — The copied model object on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/9ff01c4b-9f9a-aedd-731b-c8e5b04f50e0)

#### `public static ModelObject CopyObjectMirror(ModelObject modelObject, double x, double y, double angle)`

Creates a mirrored copy of the given model object.

**Parameters:**
- `modelObject` (Tekla.Structures.Model.ModelObject) — The model object to copy.
- `x` (System.Double) — The x coordinate of a point on the mirror line.
- `y` (System.Double) — The y coordinate of a point on the mirror line.
- `angle` (System.Double) — The angle of the mirror line in radians to the x axis.

**Returns:** `ModelObject` — The copied model object if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/2d75b51c-f658-03f0-dadb-7e2588e98461)

#### `public static BentPlate CreateBentPlateByFaces(Part part1, Face face1, Part part2, Face face2)`

Modifies the first plate by adding a cylindrical bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. See CreateBentPlateByFaces(Part, Face, Part, Face, BentPlate.BendShape).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (Tekla.Structures.Solid.Face) — The selected face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (Tekla.Structures.Solid.Face) — The selected face on the second part.

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/f401c9ed-80c1-f692-6736-73d779c8577e)

#### `public static BentPlate CreateBentPlateByFaces(Part part1, Face face1, Part part2, Face face2, BentPlate.BendShape bendShape)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, BentPlate.BendShape).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (Tekla.Structures.Solid.Face) — The selected solid face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (Tekla.Structures.Solid.Face) — The selected solid face on the second part.
- `bendShape` (Tekla.Structures.Model.BentPlate.BendShape) — Shape of the bend (cylindrical or conical)

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/621e8e34-f192-2b5e-4be6-03ef655300ca)

#### `public static BentPlate CreateBentPlateByFaces(Part part1, Face face1, Part part2, Face face2, double radius)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part and radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (Tekla.Structures.Solid.Face) — The selected solid face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (Tekla.Structures.Solid.Face) — The selected solid face on the second part.
- `radius` (System.Double) — The target radius for the created cylindrical section.

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/3001ef45-94f0-0b94-2843-907272f839e5)

#### `public static BentPlate CreateBentPlateByFaces(Part part1, IList<Point> face1, Part part2, IList<Point> face2)`

Modifies the first plate by adding a cylindrical bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. See CreateBentPlateByFaces(Part, IList<Point>, Part, IList<Point>, BentPlate.BendShape).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (System.Collections.Generic.IList<Point>) — The selected face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (System.Collections.Generic.IList<Point>) — The selected face on the second part.

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/1d1f228a-de86-1150-f8eb-12ab1cfc98f9)

#### `public static BentPlate CreateBentPlateByFaces(Part part1, IList<Point> face1, Part part2, IList<Point> face2, BentPlate.BendShape bendShape)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, BentPlate.BendShape).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (System.Collections.Generic.IList<Point>) — The selected face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (System.Collections.Generic.IList<Point>) — The selected face on the second part.
- `bendShape` (Tekla.Structures.Model.BentPlate.BendShape) — Shape of the bend (cylindrical or conical)

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/0dca7df4-8430-6f6c-e648-2d40134a06a7)

#### `public static BentPlate CreateBentPlateByFaces(Part part1, IList<Point> face1, Part part2, IList<Point> face2, double radius)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts, selected faces in each part and radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (System.Collections.Generic.IList<Point>) — The selected face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (System.Collections.Generic.IList<Point>) — The selected face on the second part.
- `radius` (System.Double) — The target radius for the created cylindrical section.

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/c7e50f00-b8cd-3418-4bfb-561fdf9bae0c)

#### `public static BentPlate CreateBentPlateByParts(Part part1, Part part2)`

Modifies the first plate by adding a cylindrical bend that connects it to the second plate creating a new BentPlate instance based on two parts. See CreateBentPlateByParts(Part, Part, BentPlate.BendShape).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — One part used for creating the bent plate.
- `part2` (Tekla.Structures.Model.Part) — The other part used for creating the bent plate.

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/693a8447-9e68-ed71-dc28-fb0a1753e45a)

#### `public static BentPlate CreateBentPlateByParts(Part part1, Part part2, BentPlate.BendShape bendShape)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, ConnectiveGeometry, BentPlate.BendShape).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — One part used for creating the bent plate.
- `part2` (Tekla.Structures.Model.Part) — The other part used for creating the bent plate.
- `bendShape` (Tekla.Structures.Model.BentPlate.BendShape) — Shape of the bend (cylindrical or conical)

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/a9f4e2ea-0881-3278-a468-ff1d483544d3)

#### `public static BentPlate CreateBentPlateByParts(Part part1, Part part2, double radius)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and a radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, ConnectiveGeometry, Double).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — One part used for creating the bent plate.
- `part2` (Tekla.Structures.Model.Part) — The other part used for creating the bent plate.
- `radius` (System.Double) — The target radius for the created cylindrical section.

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/49a6d6e3-83ec-e179-c4c0-7545a44723d6)

#### `public static BentPlate CreateConicalBentPlateByFaces(Part part1, Face face1, Part part2, Face face2, double largestRadius, double halfAperture)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part, and the largest radius of the conical section and the cone aperture. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (Tekla.Structures.Solid.Face) — The selected solid face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (Tekla.Structures.Solid.Face) — The selected solid face on the second part.
- `largestRadius` (System.Double) — Largest radius of the conical section
- `halfAperture` (System.Double) — Angle between a generatrix of the cone and its center line (i.e. axis)

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/3e17cb16-5db2-96ff-a9bc-7b6ac320e2b9)

#### `public static BentPlate CreateConicalBentPlateByFaces(Part part1, IList<Point> face1, Part part2, IList<Point> face2, double largestRadius, double halfAperture)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts, selected faces in each part and radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — The first part.
- `face1` (System.Collections.Generic.IList<Point>) — The selected face on the first part.
- `part2` (Tekla.Structures.Model.Part) — The second part.
- `face2` (System.Collections.Generic.IList<Point>) — The selected face on the second part.
- `largestRadius` (System.Double) — The largest target radius for the created conical section.
- `halfAperture` (System.Double) — Angle between a generatrix of the cone and its center line (i.e. axis)

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/b70630f0-c290-2237-1249-f37b1e6d4469)

#### `public static BentPlate CreateConicalBentPlateByPartsAndAperture(Part part1, Part part2, double largestRadius, double halfAperture)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts. The resulting bend will have the given aperture and the provided larger radius.

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — One part used for creating the bent plate.
- `part2` (Tekla.Structures.Model.Part) — The other part used for creating the bent plate.
- `largestRadius` (System.Double) — Radius of the largest section of the cone
- `halfAperture` (System.Double) — Angle between a generatrix of the cone and its center line (i.e. axis)

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/ec65a439-38a2-82af-b6bf-a97b395a9b70)

#### `public static BentPlate CreateConicalBentPlateByPartsAndTwoRadiuses(Part part1, Part part2, double firstRadius, double secondRadius)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts. The resulting bend will have the two given radiuses.

**Parameters:**
- `part1` (Tekla.Structures.Model.Part) — One part used for creating the bent plate.
- `part2` (Tekla.Structures.Model.Part) — The other part used for creating the bent plate.
- `firstRadius` (System.Double) — Radius of one section of the cone
- `secondRadius` (System.Double) — Radius of the other section of the cone

**Returns:** `BentPlate` — The bent plate object if successful, null otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/986edfe5-9465-7814-159b-cef1dee20d5a)

#### `public static bool CreateDGNv8Export(string FileName, string Folder, Operation.ExportBasePoint BasePoint, string BasePointGuid, string ObjectColoring, string ExportLayersAs, bool OnlyFromSelectedObjects)`

Dgn export from all or selected objects.

**Parameters:**
- `FileName` (System.String) — The file name.
- `Folder` (System.String) — The folder.
- `BasePoint` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — Defines which base point is used in export.
- `BasePointGuid` (System.String) — Base point guid if BasePoint parameter is set to BASE_POINT.
- `ObjectColoring` (System.String) — Defines which object coloring filter is to be used in export.
- `ExportLayersAs` (System.String) — Defines which report field to use for layers export.
- `OnlyFromSelectedObjects` (System.Boolean) — Defines if only selected (value true) or all (value false) objects are exported.

**Returns:** `Boolean` — True, if Dgn export was successful, otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/47/470aa2f9-a4d1-5ba5-9675-7f88e88a08d7)

#### `public static bool CreateIFC2x3Export(string fullPathAndFileName, Operation.IFC2x3ExportViewTypeEnum exportViewType, IEnumerable<string> attributesOverridesFileNames, string additionalPropertySetFileName, Operation.ExportBasePoint locationByValue, bool layersNamesAsPartNames, bool useViewColors, Operation.IFC2x3ExportFlags flags, string basePointGuid)`

Creates IFC2x3 export using the given file name. See Tekla Structures Help for more information about IFC2x3 export files.

**Parameters:**
- `fullPathAndFileName` (System.String) — The full path and file name including extension (.ifc, .ifczip or .ifcxml).
- `exportViewType` (Tekla.Structures.Model.Operations.Operation.IFC2x3ExportViewTypeEnum) — The export view type.
- `attributesOverridesFileNames` (System.Collections.Generic.IEnumerable<String>) — The collection of full paths and file names of the attributes overrides files (*_overrideSetData.attributeoverride and *_overrideData.attributeoverride) to use in export. Empty for none.
- `additionalPropertySetFileName` (System.String) — The full path and file name of the property set file (.xml) to use in export. Empty for none.
- `locationByValue` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — The location by value.
- `layersNamesAsPartNames` (System.Boolean) — True for layers names as part names. False for default.
- `useViewColors` (System.Boolean) — True for using view colors in export. False for default.
- `flags` (Tekla.Structures.Model.Operations.Operation.IFC2x3ExportFlags) — The additional flags.
- `basePointGuid` (System.String) — Base point guid, if a base point used.

**Returns:** `Boolean` — True, if IFC2x3 export was successful, otherwise false. See log for additional info in case of failure.

[Docs](https://developer.tekla.com/topic/en/18/47/d455e837-b27d-de76-3f8f-f4c8c98bc344)

#### `public static bool CreateIFC4ExportFromAll(string fullPathAndFileName, Operation.IFCExportViewTypeEnum exportViewType, IEnumerable<string> propertySets, Operation.ExportBasePoint locationByValue, string exportLayersAsValue, string objectColoring, Operation.IFCExportFlags flags, string basePointGuid)`

Creates IFC4 export from all objects using the given file name. See Tekla Structures Help for more information about IFC4 export files.

**Parameters:**
- `fullPathAndFileName` (System.String) — The full path and file name including extension (.ifc or .ifcZip).
- `exportViewType` (Tekla.Structures.Model.Operations.Operation.IFCExportViewTypeEnum) — The export view type.
- `propertySets` (System.Collections.Generic.IEnumerable<String>) — The full path and file name of the property set files (.xml) to use in export.
- `locationByValue` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — The location by value.
- `exportLayersAsValue` (System.String) — The export layers as value: __Name__, __Phase__ or UDA value.
- `objectColoring` (System.String) — The object coloring value: Use "ByObjectClass" for color by object class. For additional coloring, use the name of the object representation.
- `flags` (Tekla.Structures.Model.Operations.Operation.IFCExportFlags) — The additional flags.
- `basePointGuid` (System.String) — Base point guid, if a base point used.

**Returns:** `Boolean` — True, if Ifc4 export was successful, otherwise false. See session log for additional info in case failure.

[Docs](https://developer.tekla.com/topic/en/18/47/35b592da-834c-6c6f-b8ee-5cb14b2d8df0)

#### `public static bool CreateIFC4ExportFromAll(string fullPathAndFileName, Operation.IFCExportViewTypeEnum exportViewType, Operation.IFCExportVersionEnum exportVersion, IEnumerable<string> attributesOverridesFileNames, IEnumerable<string> propertySets, Operation.ExportBasePoint locationByValue, string exportLayersAsValue, string objectColoring, Operation.IFCExportFlags flags, string basePointGuid)`

Creates IFC4 export from all objects using the given file name. See Tekla Structures Help for more information about IFC4 export files.

**Parameters:**
- `fullPathAndFileName` (System.String) — The full path and file name including extension (.ifc or .ifcZip).
- `exportViewType` (Tekla.Structures.Model.Operations.Operation.IFCExportViewTypeEnum) — The export view type.
- `exportVersion` (Tekla.Structures.Model.Operations.Operation.IFCExportVersionEnum) — The IFC export version.
- `attributesOverridesFileNames` (System.Collections.Generic.IEnumerable<String>) — The collection of full paths and file names of the attributes overrides files (*_overrideSetData.attributeoverride and *_overrideData.attributeoverride) to use in export. Empty for none.
- `propertySets` (System.Collections.Generic.IEnumerable<String>) — The full path and file name of the property set files (.xml) to use in export.
- `locationByValue` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — The location by value.
- `exportLayersAsValue` (System.String) — The export layers as value: __Name__, __Phase__ or UDA value.
- `objectColoring` (System.String) — The object coloring value: Use "ByObjectClass" for color by object class. For additional coloring, use the name of the object representation.
- `flags` (Tekla.Structures.Model.Operations.Operation.IFCExportFlags) — The additional flags.
- `basePointGuid` (System.String) — Base point guid, if a base point used.

**Returns:** `Boolean` — True, if Ifc4 export was successful, otherwise false. See session log for additional info in case failure.

[Docs](https://developer.tekla.com/topic/en/18/47/84a2b76e-89c5-4976-c391-de09f4f12712)

#### `public static bool CreateIFC4ExportFromSelected(string fullPathAndFileName, Operation.IFCExportViewTypeEnum exportViewType, IEnumerable<string> propertySets, Operation.ExportBasePoint locationByValue, string exportLayersAsValue, string objectColoring, Operation.IFCExportFlags flags, string basePointGuid)`

Creates IFC4 export from the selected parts using the given file name. See Tekla Structures Help for more information about IFC4 export files.

**Parameters:**
- `fullPathAndFileName` (System.String) — The full path and file name including extension (.ifc or .ifcZip).
- `exportViewType` (Tekla.Structures.Model.Operations.Operation.IFCExportViewTypeEnum) — The export view type.
- `propertySets` (System.Collections.Generic.IEnumerable<String>) — The full path and file name of the property set files (.xml) to use in export.
- `locationByValue` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — The location by value.
- `exportLayersAsValue` (System.String) — The export layers as value: __Name__, __Phase__ or UDA value.
- `objectColoring` (System.String) — The object coloring value: Use "ByObjectClass" for color by object class. For additional coloring, use the name of the object representation.
- `flags` (Tekla.Structures.Model.Operations.Operation.IFCExportFlags) — The additional flags.
- `basePointGuid` (System.String) — Base point guid, if a base point used.

**Returns:** `Boolean` — True, if Ifc4 export was successful, otherwise false. See session log for additional info in case failure.

[Docs](https://developer.tekla.com/topic/en/18/47/2e42dad3-18a9-cf38-79a7-f9ca7ddbee07)

#### `public static bool CreateIFC4ExportFromSelected(string fullPathAndFileName, Operation.IFCExportViewTypeEnum exportViewType, Operation.IFCExportVersionEnum exportVersion, IEnumerable<string> attributesOverridesFileNames, IEnumerable<string> propertySets, Operation.ExportBasePoint locationByValue, string exportLayersAsValue, string objectColoring, Operation.IFCExportFlags flags, string basePointGuid)`

Creates IFC4 export from the selected parts only using the given file name. See Tekla Structures Help for more information about IFC4 export files.

**Parameters:**
- `fullPathAndFileName` (System.String) — The full path and file name including extension (.ifc or .ifcZip).
- `exportViewType` (Tekla.Structures.Model.Operations.Operation.IFCExportViewTypeEnum) — The export view type.
- `exportVersion` (Tekla.Structures.Model.Operations.Operation.IFCExportVersionEnum) — The IFC export version.
- `attributesOverridesFileNames` (System.Collections.Generic.IEnumerable<String>) — The collection of full paths and file names of the attributes overrides files (*_overrideSetData.attributeoverride and *_overrideData.attributeoverride) to use in export. Empty for none.
- `propertySets` (System.Collections.Generic.IEnumerable<String>) — The full path and file name of the property set files (.xml) to use in export.
- `locationByValue` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — The location by value.
- `exportLayersAsValue` (System.String) — The export layers as value: __Name__, __Phase__ or UDA value.
- `objectColoring` (System.String) — The object coloring value: Use "ByObjectClass" for color by object class. For additional coloring, use the name of the object representation.
- `flags` (Tekla.Structures.Model.Operations.Operation.IFCExportFlags) — The additional flags.
- `basePointGuid` (System.String) — Base point guid, if a base point used.

**Returns:** `Boolean` — True, if Ifc4 export was successful, otherwise false. See session log for additional info in case failure.

[Docs](https://developer.tekla.com/topic/en/18/47/0fbe065a-e49b-bd3e-850c-655f7934a85c)

#### `public static bool CreateIFC4ExportFromSelectedAndTheirAssemblies(string fullPathAndFileName, Operation.IFCExportViewTypeEnum exportViewType, Operation.IFCExportVersionEnum exportVersion, IEnumerable<string> attributesOverridesFileNames, IEnumerable<string> propertySets, Operation.ExportBasePoint locationByValue, string exportLayersAsValue, string objectColoring, Operation.IFCExportFlags flags, string basePointGuid)`

Creates IFC4 export from the selected parts and their assemblies using the given file name. See Tekla Structures Help for more information about IFC4 export files.

**Parameters:**
- `fullPathAndFileName` (System.String) — The full path and file name including extension (.ifc or .ifcZip).
- `exportViewType` (Tekla.Structures.Model.Operations.Operation.IFCExportViewTypeEnum) — The export view type.
- `exportVersion` (Tekla.Structures.Model.Operations.Operation.IFCExportVersionEnum) — The IFC export version.
- `attributesOverridesFileNames` (System.Collections.Generic.IEnumerable<String>) — The collection of full paths and file names of the attributes overrides files (*_overrideSetData.attributeoverride and *_overrideData.attributeoverride) to use in export. Empty for none.
- `propertySets` (System.Collections.Generic.IEnumerable<String>) — The full path and file name of the property set files (.xml) to use in export.
- `locationByValue` (Tekla.Structures.Model.Operations.Operation.ExportBasePoint) — The location by value.
- `exportLayersAsValue` (System.String) — The export layers as value: __Name__, __Phase__ or UDA value.
- `objectColoring` (System.String) — The object coloring value: Use "ByObjectClass" for color by object class. For additional coloring, use the name of the object representation.
- `flags` (Tekla.Structures.Model.Operations.Operation.IFCExportFlags) — The additional flags.
- `basePointGuid` (System.String) — Base point guid, if a base point used.

**Returns:** `Boolean` — True, if Ifc4 export was successful, otherwise false. See session log for additional info in case failure.

[Docs](https://developer.tekla.com/topic/en/18/47/194b2297-3487-1711-3a17-5c6d7a3ec795)

#### `public static bool CreateMISFileFromAll(Operation.MISExportTypeEnum MISType, string FileName)`

Creates MIS files from all parts using the given file name. See Tekla Structures Help for more information about MIS files.

**Parameters:**
- `MISType` (Tekla.Structures.Model.Operations.Operation.MISExportTypeEnum) — The type of the MIS export.
- `FileName` (System.String) — The name of the MIS file to be used in creation.

**Returns:** `Boolean` — True if the MIS files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/3090fd50-4614-041c-e0cf-85edc182e5c5)

#### `public static bool CreateMISFileFromSelected(Operation.MISExportTypeEnum MISType, string FileName)`

Creates MIS files from the selected parts using the given file name. See Tekla Structures Help for more information about MIS files.

**Parameters:**
- `MISType` (Tekla.Structures.Model.Operations.Operation.MISExportTypeEnum) — The type of the MIS export.
- `FileName` (System.String) — The name of the MIS file to be used in creation.

**Returns:** `Boolean` — True if the MIS files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/4d66ce5b-826f-0efd-a9cd-e6ca304e2d93)

#### `public static bool CreateNCFilesByPartId(string NCFileSettings, string DestinationFolder, Identifier PartID, out string DstvOutput, bool CreatePopMarks = false, string PopMarkSettingsFileName = "", bool CreateContourMarking = false, string ContourMarkingSettingsFileName = "")`

Creates NC files from the selected parts using the given NC template name. See Tekla Structures Help for more information about NC files.

**Parameters:**
- `NCFileSettings` (System.String) — The name of the NC setting template to be used in creation.
- `DestinationFolder` (System.String) — The name of the folder where NC files are created. If defined, overrides the default folder in the setting template.
- `PartID` (Tekla.Structures.Identifier) — The identifier of the part.
- `DstvOutput` (System.String.) — The DSTV output as string.
- `CreatePopMarks` (System.Boolean) — Create pop-marks during export.
- `PopMarkSettingsFileName` (System.String) — The name of the pop-mark setting file to be used in creation.
- `CreateContourMarking` (System.Boolean) — Create contour marking during export.
- `ContourMarkingSettingsFileName` (System.String) — The name of the contour marking setting file to be used in creation.

**Returns:** `Boolean` — True if the NC files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/39156aa2-9243-836c-f5cd-66cef7b6728f)

#### `public static bool CreateNCFilesFromAll(string NCFileSettings, string DestinationFolder)`

Creates NC files from all parts using the given NC template name. See Tekla Structures Help for more information about NC files.

**Parameters:**
- `NCFileSettings` (System.String) — The name of the NC setting template to be used in creation.
- `DestinationFolder` (System.String) — The name of the folder where NC files are created. If defined, overrides the default folder in the setting template.

**Returns:** `Boolean` — True if the NC files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/736ca893-968e-c433-7b5d-4830dd055711)

#### `public static bool CreateNCFilesFromAll(string NCFileSettings, string DestinationFolder, bool CreatePopMarks = false, string PopMarkSettingsFileName = "", bool CreateContourMarking = false, string ContourMarkingSettingsFileName = "")`

Creates NC files from all parts using the given NC template name. See Tekla Structures Help for more information about NC files.

**Parameters:**
- `NCFileSettings` (System.String) — The name of the NC setting template to be used in creation.
- `DestinationFolder` (System.String) — The name of the folder where NC files are created. If defined, overrides the default folder in the setting template.
- `CreatePopMarks` (System.Boolean) — Create pop-marks during export.
- `PopMarkSettingsFileName` (System.String) — The name of the pop-mark setting file to be used in creation.
- `CreateContourMarking` (System.Boolean) — Create contour marking during export.
- `ContourMarkingSettingsFileName` (System.String) — The name of the contour marking setting file to be used in creation.

**Returns:** `Boolean` — True if the NC files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/1b185764-bea1-d294-a375-c32ab26d4d64)

#### `public static bool CreateNCFilesFromSelected(string NCFileSettings, string DestinationFolder)`

Creates NC files from the selected parts using the given NC template name. See Tekla Structures Help for more information about NC files.

**Parameters:**
- `NCFileSettings` (System.String) — The name of the NC setting template to be used in creation.
- `DestinationFolder` (System.String) — The name of the folder where NC files are created. If defined, overrides the default folder in the setting template.

**Returns:** `Boolean` — True if the NC files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/a832ed01-97e7-61c8-ee9e-bfdb0d0239ad)

#### `public static bool CreateNCFilesFromSelected(string NCFileSettings, string DestinationFolder, bool CreatePopMarks = false, string PopMarkSettingsFileName = "", bool CreateContourMarking = false, string ContourMarkingSettingsFileName = "")`

Creates NC files from the selected parts using the given NC template name. See Tekla Structures Help for more information about NC files.

**Parameters:**
- `NCFileSettings` (System.String) — The name of the NC setting template to be used in creation.
- `DestinationFolder` (System.String) — The name of the folder where NC files are created. If defined, overrides the default folder in the setting template.
- `CreatePopMarks` (System.Boolean) — Create pop-marks during export.
- `PopMarkSettingsFileName` (System.String) — The name of the pop-mark setting file to be used in creation.
- `CreateContourMarking` (System.Boolean) — Create contour marking during export.
- `ContourMarkingSettingsFileName` (System.String) — The name of the contour marking setting file to be used in creation.

**Returns:** `Boolean` — True if the NC files are created, false if the numbering is not up-to-date or the used configuration is wrong.

[Docs](https://developer.tekla.com/topic/en/18/47/8f4e08b2-41eb-d6a1-859d-ddb50111d9e5)

#### `public static bool CreateReport(Operation.ReportParameters parameters)`

Creates a report using the given report parameters. A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.See Tekla Structures Help for more information about reports.

**Parameters:**
- `parameters` (Tekla.Structures.Model.Operations.Operation.ReportParameters) — The report parameters.

**Returns:** `Boolean` — True if the report is created.

[Docs](https://developer.tekla.com/topic/en/18/47/3235aaad-349c-8cc1-1222-5f4e3c994561)

#### `public static bool CreateReportFromAll(string TemplateName, string FileName, string Title1, string Title2, string Title3)`

Creates a report from all the objects using the given template name and filename. A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.Internally, this method is asynchronous, and because of that the output file cannot be immediately available.See Tekla Structures Help for more information about reports.

**Parameters:**
- `TemplateName` (System.String) — The name of the report template to be used in report creation. The name must contain more than three characters.
- `FileName` (System.String) — The name of the created report. The name must contain more than three characters.
- `Title1` (System.String) — The first title for the created report.
- `Title2` (System.String) — The second title for the created report.
- `Title3` (System.String) — The third title for the created report.

**Returns:** `Boolean` — True if the report is created.

[Docs](https://developer.tekla.com/topic/en/18/47/b4ff470c-a784-75b1-3204-0bfe7cfcdf04)

#### `public static bool CreateReportFromSelected(string TemplateName, string FileName, string Title1, string Title2, string Title3)`

Creates a report from the selected objects using the given template name and filename. A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.See Tekla Structures Help for more information about reports.

**Parameters:**
- `TemplateName` (System.String) — The name of the report template to be used in report creation. The name must contain more than three characters.
- `FileName` (System.String) — The name of the created report. The name must contain more than three characters.
- `Title1` (System.String) — The first title for the created report.
- `Title2` (System.String) — The second title for the created report.
- `Title3` (System.String) — The third title for the created report.

**Returns:** `Boolean` — True if the report is created.

[Docs](https://developer.tekla.com/topic/en/18/47/20138139-84ec-063b-734f-d73210a43196)

#### `public static string CreateShapeFromGeometry(Part originalPart)`

Create a new shape in the shape catalog, based on the part geometry.

**Parameters:**
- `originalPart` (Tekla.Structures.Model.Part) — Original part

**Returns:** `String` — The newly created shape name, or empty string on failure

[Docs](https://developer.tekla.com/topic/en/18/47/f742bc27-2758-364e-bab3-51554907d1d3)

#### `public static bool DisplayPrintReportDialog(string FileName)`

Opens the print dialog for a report with the given filename.

**Parameters:**
- `FileName` (System.String)

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/8e7486cf-b2bd-3400-3d72-5f7003a421dd)

#### `public static bool DisplayPrompt(string Message)`

Displays a message in the status bar.

**Remarks:** Prompts Tekla Structures prefixes the given prompt with "prompt_" and looks for a translation in the prompts.ail file. If the translation (e.g. "prompt_Pick_first_position") is not found in the prompts.ail file, the prompt string is displayed as such. This feature can be used to give already translated strings to the picker.

**Parameters:**
- `Message` (System.String) — The message to display.

**Returns:** `Boolean` — True if the message could be displayed.

[Docs](https://developer.tekla.com/topic/en/18/47/0be24953-f509-559d-1682-51d772836434)

#### `public static bool DisplayReport(string FileName)`

Opens and displays a report with the given name. If a path is not given in the filename, the file is searched from the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.See Tekla Structures Help for more information about reports.

**Parameters:**
- `FileName` (System.String) — The name of the report to display. The name must contain more than three characters.

**Returns:** `Boolean` — True if the report existed.

[Docs](https://developer.tekla.com/topic/en/18/47/f0341426-3054-511a-e795-d26e240bcd95)

#### `public static bool ExplodeBentPlate(BentPlate bentPlate)`

Deletes bentPlate and inserts ContourPlates instances equivalent to the ones used to create bentPlate. The ContourPlate created from the main polygon has the same identifier as bentPlate.

**Parameters:**
- `bentPlate` (Tekla.Structures.Model.BentPlate) — the BentPlate instance to explode.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/90695733-55df-251e-05cd-b9115f915769)

#### `public static bool Freeze(RebarSet rebarSet, bool value)`

Freezes or unfreezes a rebar set.

**Parameters:**
- `rebarSet` (Tekla.Structures.Model.RebarSet) — The rebar set to be frozen or unfrozen.
- `value` (System.Boolean) — True if it is to be frozen, false if it is to be unfrozen.

**Returns:** `Boolean` — True if the frozen state has been successfully changed.

[Docs](https://developer.tekla.com/topic/en/18/47/0ed78310-48b6-2e28-a003-5ab7932edfcb)

#### `public static List<Point> GetHandlePoints(string guid)`

Get the Shape Item Handle Points

**Parameters:**
- `guid` (System.String) — The guid of the shape

**Returns:** `List<Point>` — List of Handle Points

[Docs](https://developer.tekla.com/topic/en/18/47/75751ea1-681e-d44e-de08-475b46d257df)

#### `public static List<ModelObject> GetSimilarNumberedObjects(ModelObject ObjectToCompare)`

Gets similar objects based on numbering of given object.

**Remarks:** This method works currently only with parts and assemblies.

**Parameters:**
- `ObjectToCompare` (Tekla.Structures.Model.ModelObject) — The object for comparison.

**Returns:** `List<ModelObject>` — List of similar objects.

[Docs](https://developer.tekla.com/topic/en/18/47/5f906cbe-5507-55e3-d31f-00043acf148a)

#### `public static RebarGroup Group(IEnumerable RebarList)`

Groups a list of single rebars or rebar groups and creates a new rebar group.

**Parameters:**
- `RebarList` (System.Collections.IEnumerable) — The list of single rebars and rebar groups to be grouped.

**Returns:** `RebarGroup` — The created rebar group on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/179f6a0a-95ea-6517-ddcd-b9a82c374d25)

#### `public static bool Highlight(List<ModelObject> ModelObjects)`

Highlight the list of objects from the user interface. Only the list of objects will be highlighted, the rest will be unhighlighted. But it will keep the status of the selected objects. To unhighlight give an empty list as a parameter.

**Parameters:**
- `ModelObjects` (System.Collections.Generic.List<ModelObject>) — The list of model objects to highlight.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/b67e14f2-d5c5-c685-d527-b9d451460899)

#### `public static bool IsMacroRunning()`

Returns true if a macro is running, false otherwise. Macros are saved as *.cs files in the folder defined with the XS_MACRO_DIRECTORY variable.See Tekla Structures Help for more information about macros.

**Returns:** `Boolean` — True if a macro is running.

[Docs](https://developer.tekla.com/topic/en/18/47/2f242e58-9e53-908e-5f49-6e67f27b3ae4)

#### `[ObsoleteAttribute("Use the method in ModelHandler class instead. Will be removed in near future.")] public static bool IsModelAutoSaved(string ModelFolder)`

Tells whether a model has auto saved information.

**Parameters:**
- `ModelFolder` (System.String) — The model folder to be used.

**Returns:** `Boolean` — True if there is auto saved information.

[Docs](https://developer.tekla.com/topic/en/18/47/afcedc6c-a2a5-12a2-f8f3-03f0a83283fa)

#### `public static bool IsNumberingAllowed()`

Checks whether the numbering is allowed or not in the current configuration.

**Returns:** `Boolean` — True if the numbering is allowed.

[Docs](https://developer.tekla.com/topic/en/18/47/3bc7887e-5852-1d2f-1de8-b0eebb84b6f7)

#### `public static bool IsNumberingUpToDate(ModelObject InputModelObject)`

Checks whether the numbering is up-to-date for an assembly, part, rebar, surface treatment, pour object or break.

**Parameters:**
- `InputModelObject` (Tekla.Structures.Model.ModelObject) — The model object to check. The object must be an assembly, a part, a rebar or an inherited object.

**Returns:** `Boolean` — True if the numbering information is up-to-date.

[Docs](https://developer.tekla.com/topic/en/18/47/27f48c6d-a775-ddf5-c54c-f168fc904322)

#### `public static bool IsNumberingUpToDateAll()`

Checks whether the numbering is up-to-date for every assembly, part and rebar on the model. Remarks Using this method is much faster than checking each object individually.

**Remarks:** Using this method is much faster than checking each object individually.

**Returns:** `Boolean` — True if the numbering information is up-to-date.

[Docs](https://developer.tekla.com/topic/en/18/47/92637418-2b30-ce77-7dc1-038322a9544a)

#### `public static bool MoveObject(ModelObject Object, CoordinateSystem StartCoordinateSystem, CoordinateSystem EndCoordinateSystem)`

Moves the model object between the given translation coordinate systems.

**Remarks:** Note that the object is moved and updated in the view so ModelObject.Modify() is not needed. Call Modify() only after the object's data has been updated with the ModelObject.Select() method.

**Parameters:**
- `Object` (Tekla.Structures.Model.ModelObject) — The model object to move.
- `StartCoordinateSystem` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to move the object from.
- `EndCoordinateSystem` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to move the object to.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/c8c000f1-f5d4-0429-45c4-021bdf02036c)

#### `public static bool MoveObject(ModelObject Object, Vector TranslationVector)`

Moves the model object using the given translation vector.

**Remarks:** Note that the object is moved and updated in the view so ModelObject.Modify() is not needed. Call Modify() only after the object's data has been updated with the ModelObject.Select() method.

**Parameters:**
- `Object` (Tekla.Structures.Model.ModelObject) — The model object to move.
- `TranslationVector` (Tekla.Structures.Geometry3d.Vector) — The vector for moving the object.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/e0d1a2cd-6145-ff68-a8f0-f0e74c1dc78f)

#### `public static bool MoveObjectMirror(ModelObject modelObject, double x, double y, double angle)`

Mirrors the object in the plane defined by the given data is the current workplane.

**Parameters:**
- `modelObject` (Tekla.Structures.Model.ModelObject) — The model object to mirror.
- `x` (System.Double) — The x coordinate of a point on the mirror line.
- `y` (System.Double) — The y coordinate of a point on the mirror line.
- `angle` (System.Double) — The angle of the mirror line in radians to the x axis.

**Returns:** `Boolean` — True if successful.

[Docs](https://developer.tekla.com/topic/en/18/47/515aa261-bbf2-764d-45c2-a2ebddace452)

#### `public static bool ObjectMatchesToFilter(ModelObject ModelObject, FilterExpression FilterExpression)`

Checks whether the object matches to the criteria in the given filter.

**Parameters:**
- `ModelObject` (Tekla.Structures.Model.ModelObject) — The model object to check.
- `FilterExpression` (Tekla.Structures.Filtering.FilterExpression) — The definition of a selection filter to check against.

**Returns:** `Boolean` — True if the object matches to the given filter criteria.

[Docs](https://developer.tekla.com/topic/en/18/47/05260766-8da9-47ba-7421-8dc18edce140)

#### `public static bool ObjectMatchesToFilter(ModelObject ModelObject, string FilterName)`

Checks whether the object matches to the criteria in the given filter.

**Parameters:**
- `ModelObject` (Tekla.Structures.Model.ModelObject) — The model object to check.
- `FilterName` (System.String) — The filter file to check against.

**Returns:** `Boolean` — True if the object matches to the given filter criteria.

[Docs](https://developer.tekla.com/topic/en/18/47/25fb8b93-31f3-5c3e-3538-263151971be2)

#### `[ObsoleteAttribute("Use the method in ModelHandler class instead. Will be removed in near future.")] public static bool Open(string ModelFolder)`

Opens a new model to Tekla Structures ignoring auto saved information.

**Parameters:**
- `ModelFolder` (System.String) — The model folder to be used.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/2c908a15-5315-833e-9794-bf16a0687e3d)

#### `[ObsoleteAttribute("Use the method in ModelHandler class instead. Will be removed in near future.")] public static bool Open(string ModelFolder, bool OpenAutoSaved)`

Opens a new model to Tekla Structures.

**Parameters:**
- `ModelFolder` (System.String) — The model folder to be used.
- `OpenAutoSaved` (System.Boolean) — Tells whether to open auto saved information or not.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/90bb1f01-bf9a-9bab-6a7e-12ce9ca36a0b)

#### `public static bool RemoveFromPourUnit(List<ModelObject> objectsToBeRemoved)`

Removes model object from pour unit Model object types accepted are assembly types except cast in situ, reinforcements of different kind, components and bolts

**Parameters:**
- `objectsToBeRemoved` (System.Collections.Generic.List<ModelObject>) — the list of model objects to be added.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/ed267ff7-02e0-bd4c-36a8-bc7e258d3c83)

#### `public static bool RunMacro(string FileName)`

Starts a macro with the given name. Throws an exception if the file is not found. Macros are saved as *.cs files in the folder defined with the XS_MACRO_DIRECTORY variable.It is possible to run drawing macros using relative paths.See Tekla Structures Help for more information about macros.

**Parameters:**
- `FileName` (System.String) — The name of the macro to start.

**Returns:** `Boolean` — True if the macro existed.

[Docs](https://developer.tekla.com/topic/en/18/47/42bee48d-632b-6b20-6c82-3e88af869716)

#### `public static bool SaveAsWebModel(string Filename)`

Saves the current model as a web model. You can save the model as a web model that can be viewed via the Internet using a web browser (e.g. Internet Explorer).

**Parameters:**
- `Filename` (System.String) — The filename to be used.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/00aec8f7-3875-a8db-f76a-967ac47d9496)

#### `public static bool SaveSelectedAsWebModel(string Filename)`

Saves the selected objects as a web model. You can save the selected objects as a web model that can be viewed via the Internet using a web browser (e.g. Internet Explorer).

**Parameters:**
- `Filename` (System.String) — The filename to be used.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/7c54947d-03b0-32c4-5562-57889c799cf2)

#### `public static bool SetHandlePoints(string guid, List<Point> HandlePoints)`

Set the Shape Item Handle Points

**Parameters:**
- `guid` (System.String) — The guid of the shape
- `HandlePoints` (System.Collections.Generic.List<Point>) — An array of handle points

**Returns:** `Boolean` — True if the appending operation was successful

[Docs](https://developer.tekla.com/topic/en/18/47/7baa03fe-b8f6-c104-8642-747666367f6e)

#### `public static void ShowOnlySelected(Operation.UnselectedModeEnum UnselectedMode)`

Show Only Selected objects in current view.

**Parameters:**
- `UnselectedMode` (Tekla.Structures.Model.Operations.Operation.UnselectedModeEnum) — Specify what to do with unselected parts.

[Docs](https://developer.tekla.com/topic/en/18/47/ef58f1d7-1bc2-ad42-f1f5-21f092444369)

#### `public static Beam Split(Beam Object, Point SplitPoint)`

Splits the beam and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.Beam) — The beam object to be split.
- `SplitPoint` (Tekla.Structures.Geometry3d.Point) — The position where splitting is executed.

**Returns:** `Beam` — The created beam on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/ade59197-8112-9d79-3d9d-0de5673413f1)

#### `public static CircleRebarGroup Split(CircleRebarGroup Object, Line SplitLine)`

Splits the circle rebar group and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.CircleRebarGroup) — The circle rebar group object to be splitted.
- `SplitLine` (Tekla.Structures.Geometry3d.Line) — The line where splitting is executed.

**Returns:** `CircleRebarGroup` — The created circle rebar group on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/38823af2-3c64-c8c2-5399-b43e45fb039f)

#### `public static ContourPlate Split(ContourPlate Object, Polygon SplitPolygon)`

Splits the contour plate and creates a new one along the given polygon.

**Parameters:**
- `Object` (Tekla.Structures.Model.ContourPlate) — The contour plate object to be splitted.
- `SplitPolygon` (Tekla.Structures.Model.Polygon) — The position where splitting is executed.

**Returns:** `ContourPlate` — The created contour plate on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/cc26cef4-45ff-a6aa-dc13-ba549930cf47)

#### `public static CurvedRebarGroup Split(CurvedRebarGroup Object, Line SplitLine)`

Splits the curved rebar group and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.CurvedRebarGroup) — The curved rebar group object to be splitted.
- `SplitLine` (Tekla.Structures.Geometry3d.Line) — The line where splitting is executed.

**Returns:** `CurvedRebarGroup` — The created curved rebar group on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/2aadb317-0235-c6e1-778a-dd605cf16913)

#### `public static LoftedPlate Split(LoftedPlate Object, Point SplitPoint)`

Splits the lofted plate and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.LoftedPlate) — The lofted plate to be split.
- `SplitPoint` (Tekla.Structures.Geometry3d.Point) — The position where splitting is executed.

**Returns:** `LoftedPlate` — The created lofted plate on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/9f4fe99c-fdef-f833-52be-627717373b13)

#### `public static PolyBeam Split(PolyBeam Object, Point SplitPoint)`

Splits the polybeam and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.PolyBeam) — The beam object to be split.
- `SplitPoint` (Tekla.Structures.Geometry3d.Point) — The position where splitting is executed.

**Returns:** `PolyBeam` — The created polybeam on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/060e6ed9-cce7-0799-e26b-19403dde7f61)

#### `public static RebarGroup Split(RebarGroup Object, Line SplitLine)`

Splits the rebar group and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.RebarGroup) — The rebar group object to be splitted.
- `SplitLine` (Tekla.Structures.Geometry3d.Line) — The line where splitting is executed.

**Returns:** `RebarGroup` — The created rebar group on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/e0678430-540d-a572-190e-7d35dfdae18d)

#### `public static RebarSet Split(RebarSet Object, Line SplitLine)`

Splits the rebar set and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.RebarSet) — The rebar set object to be split.
- `SplitLine` (Tekla.Structures.Geometry3d.Line) — The line where splitting is executed.

**Returns:** `RebarSet` — The created rebar set on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/ff23c743-8528-91b0-41d0-7b84c96407ed)

#### `public static SingleRebar Split(SingleRebar Object, Line SplitLine)`

Splits the single rebar and creates a new one in the given position.

**Parameters:**
- `Object` (Tekla.Structures.Model.SingleRebar) — The single rebar object to be splitted.
- `SplitLine` (Tekla.Structures.Geometry3d.Line) — The line where splitting is executed.

**Returns:** `SingleRebar` — The created single rebar on success, null on failure.

[Docs](https://developer.tekla.com/topic/en/18/47/e4368110-15e9-c24a-02d2-1bcee426db55)

#### `public static bool SplitSlab(int PartId, FacetedBrep Polymesh)`

This command is meant for specifically splitting a concrete slab with advanced solid operations to create more robust and user friendly results than the command: public static ContourPlate Split(ContourPlate Object, Polygon SplitPolygon). No validation is done for the type, it is the caller's responsibility to call this only for valid types (slabs). Behavior for non-slabs is undetermined.

**Parameters:**
- `PartId` (System.Int32) — The part ID that identifies the slab to be split
- `Polymesh` (Tekla.Structures.Geometry3d.FacetedBrep) — The polymesh that defines the splitting surface

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/47/82d2dc9c-b4c0-d413-4570-9c2e6ecb7f91)

#### `public static ModelObjectEnumerator Ungrouping(RebarGroup Reinforcement)`

Ungroups the rebar group and creates new single rebars.

**Parameters:**
- `Reinforcement` (Tekla.Structures.Model.RebarGroup) — The rebar group to be ungrouped.

**Returns:** `ModelObjectEnumerator` — An enumerator of single rebars.

[Docs](https://developer.tekla.com/topic/en/18/47/9c5db27f-c493-c9b1-2493-91cfc61bfb83)

#### `public static ModelObjectEnumerator Ungrouping(RebarMesh Reinforcement)`

Ungroups the rebar mesh and creates new single rebars.

**Parameters:**
- `Reinforcement` (Tekla.Structures.Model.RebarMesh) — The rebar mesh to be ungrouped.

**Returns:** `ModelObjectEnumerator` — An enumerator of single rebars.

[Docs](https://developer.tekla.com/topic/en/18/47/02acb6c5-8915-ea56-1399-1a112a57961e)

## Operation.ExportBasePoint (enum)

Export base point

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a23be964-0f05-80db-4c82-afd85fd91a77)

## Operation.IFC2x3ExportFlags (struct)

Boolean flags for IFC 2x3 export.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7ded9144-e182-c22c-f38e-230f52071919)

### Properties
- `ExcludeSinglePartAssemblies` (Boolean, get/set) — Boolean indicating whether single part assemblies are excluded.
- `ExportAllObjects` (Boolean, get/set) — Boolean indicating whether to export all objects.
- `ExportBrepAsExactSolid` (Boolean, get/set) — Boolean indicating whether BREP is exported as exact solid.
- `ExportBuildingHierarchy` (Boolean, get/set) — Boolean indicating whether the building hierarchy export is enabled.
- `ExportFlatBeamsAsPlates` (Boolean, get/set) — Boolean indicating whether flat beams are exported as plates.
- `ExportLocationsFromOrganizer` (Boolean, get/set) — Boolean indicating whether locations are obtained from the organizer.
- `ExportPolyBeamAsSectionedSpine` (Boolean, get/set) — Boolean indicating whether polybeams are exported as sectioned spines.
- `ExportSelectedObjectsAndTheirAssemblies` (Boolean, get/set) — Boolean indicating whether to export selected objects and their assemblies.
- `IncludeAssemblyHiearachy` (Boolean, get/set) — Boolean indicating whether the assembly hierarhcy (IFCELEMENTASSEMBLY) is exported.
- `IsBoltsEnabled` (Boolean, get/set) — Boolean indicating whether the bolts export is enabled.
- `IsGridsEnabled` (Boolean, get/set) — Boolean indicating whether the grids export is enabled.
- `IsPoursEnabled` (Boolean, get/set) — Pours export is enabled. True for pours, false for CIP cast-units and/or parts
- `IsRebarsEnabled` (Boolean, get/set) — Boolean indicating whether the rebars export is enabled.
- `IsSpacesEnabled` (Boolean, get/set) — Boolean indicating whether the export of spaces in building hierarchy is enabled.
- `IsSurfaceTreatmentsAndSurfacesEnabled` (Boolean, get/set) — Boolean indicating whether the surfaces and surface treatments export is enabled.
- `IsWeldsEnabled` (Boolean, get/set) — Boolean indicating whether the welds export is enabled.

## Operation.IFC2x3ExportViewTypeEnum (enum)

View configuration type for IFC 2x3 export

[Vendor docs](https://developer.tekla.com/topic/en/18/47/76a2a7d0-56a6-a8ee-1667-c264f10424df)

## Operation.IFCExportFlags (struct)

Boolean flags for IFC 4 export.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a79a1b5b-f9a9-9753-0d4e-f97962336baa)

### Properties
- `UseIfcMapConversion` (Nullable<Boolean>, get/set) — Boolean indicating whether to use IfcMapConversion with base points is enabled. By default IFC4 export should use IfcMapConversion. Uninitialized null will be converted to true during export.

## Operation.IFCExportVersionEnum (enum)

IFC export version enum.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/db39e1d0-5b7b-a4f5-9fd5-f4f9ebdae72a)

## Operation.IFCExportViewTypeEnum (enum)

View configuration type for IFC 4 export

[Vendor docs](https://developer.tekla.com/topic/en/18/47/63baf60e-d0bf-f23f-d879-c6b104295c44)

## Operation.MISExportTypeEnum (enum)

The MIS export types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/011d6e9c-6827-b36b-a2bd-2cfb3a90c70d)

### Values
- `DSTV` = `0` — The DSTV type.
- `KISS` = `1` — The KISS type.
- `EJE` = `2` — The EJE type.
- `EPC` = `3` — The EPC type.
- `STEEL2000` = `4` — The STEEL2000 type.

## Operation.ProgressBar (class)

The ProgressBar class implements progress bar with cancel button.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0f2ddcb2-4dc5-597a-4ed6-237543023362)

### Constructors
- `public ProgressBar()` — Initializes a new instance of the Operation.ProgressBar class

### Methods
#### `public bool Canceled()`

Check if cancel has been pressed.

**Returns:** `Boolean` — True if cancel has been pressed.

[Docs](https://developer.tekla.com/topic/en/18/47/65b92871-fd3b-0b78-8047-52a5973e9271)

#### `public void Close()`

Close progress bar. Can be called even if Display was not successful.

[Docs](https://developer.tekla.com/topic/en/18/47/b0de203c-dfc1-a501-a91b-29428d25567a)

#### `public bool Display(int SleepTime, string Title, string Message, string CancelButtonLabel, string ProgressLabel)`

Display progress bar dialog with cancel button. Display will fail if progress bar is already displayed.

**Parameters:**
- `SleepTime` (System.Int32) — Time (ms) to wait until bar is displayed.
- `Title` (System.String) — Title of the dialog.
- `Message` (System.String) — Message to be displayed on the dialog above progress bar.
- `CancelButtonLabel` (System.String) — Label of cancel button.
- `ProgressLabel` (System.String) — Initial progress label (updated with SetProgress). If empty of null no bar exists.

**Returns:** `Boolean` — True if bar was displayed successfully (meaning bar must be closed later).

[Docs](https://developer.tekla.com/topic/en/18/47/6758e312-e9fb-ddeb-9631-20faa288a575)

#### `public void SetProgress(string ProgressLabel, int Progress)`

Update status information on the progress bar.

**Parameters:**
- `ProgressLabel` (System.String) — Bar label text.
- `Progress` (System.Int32) — Progess, number between 0..100

[Docs](https://developer.tekla.com/topic/en/18/47/4868a504-d0dd-c456-1f41-48eb4ebf8443)

## Operation.ReportParameters (class)

Represents the parameters required for generating a report.

**Remarks:** This class encapsulates various settings and options that define how a report is generated, including template names, file names, titles, and processing flags. It provides properties to specify whether the report is generated from selected objects or all objects, and allows customization of the report's appearance and format.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f3230537-529d-b6e5-72dc-e718a4190599)

### Constructors
- `public ReportParameters()` — Initializes a new instance of the Operation.ReportParameters class

### Properties
- `FileName` (String, get/set) — Gets or sets the name of the created report. The name must contain more than three characters.
- `IsFromSelected` (Boolean, get/set) — Gets or sets a value indicating whether the report is from selected or from all objects.
- `TemplateName` (String, get/set) — Gets or sets the name of the report template to be used in report creation. The name must contain more than three characters.
- `Title1` (String, get/set) — Gets or sets the first title for the created report.
- `Title2` (String, get/set) — Gets or sets the second title for the created report.
- `Title3` (String, get/set) — Gets or sets the third title for the created report.
- `XLSXFlags` (Int32, get/set) — Gets or sets the flags used for processing XLSX files.
- `XLTXFileName` (String, get/set) — Gets or sets the name of the Excel template to be used.

## Operation.ShapeMetadataResult (enum)

The result type of the shape metadata operations. If you alter this, check if you need to change ShapeMetadataResult_e on the Tekla Structures core side

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7fc3aca4-5f38-d63a-48c2-a05120f9faf5)

### Values
- `NoResult` = `0` — Operation failed.
- `OK` = `1` — Operation succeeded.
- `DuplicateKeyExist` = `2` — At least one identical pre-existing key was found in the shape when trying to insert key and value
- `NoMatchingShape` = `3` — No matching shape found for the GUID in the catalog
- `NoMatchingKey` = `4` — No matching key found for the GUID in the shape

## Operation.UnselectedModeEnum (enum)

Specifies what ShowOnlySelected(Operation.UnselectedModeEnum) should do to unselected parts.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/84bd970f-43fd-db25-3195-d3a5901fe7d4)

### Values
- `Hidden` = `0` — Completely hide.
- `Transparent` = `1` — Make almost transparent.
- `AsSticks` = `2` — Show as sticks.

