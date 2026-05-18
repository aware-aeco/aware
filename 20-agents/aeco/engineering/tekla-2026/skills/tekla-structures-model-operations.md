---
name: tekla-tekla-structures-model-operations
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Model.Operations namespace — 13 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GuidConversion, Operation.ReportParameters, Operation.ProgressBar, Operation, Operation.IFC2x3ExportFlags, Operation.IFCExportFlags, Operation.ExportBasePoint, Operation.IFC2x3ExportViewTypeEnum, and 5 more types.
---

# Tekla.Structures.Model.Operations

Auto-generated from vendor docs for tekla 2026.0. 13 types in this namespace.

## GuidConversion (class)

Conversion of old TS GUIDs to current GUIDs. GUIDs are changed in TS save as operation, this class can be used to convert old GUIDs to current GUIDs. To recognize the need for GUID conversion, application needs to save project GUID and compare to the current project GUID. Note: With big models the instance uses a lot of memory.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/920e2903-8177-35de-c9d3-ba5e095fec10)

### Constructors
- `GuidConversion(...)` — Initializes a new instance of the GuidConversion class.

### Methods
#### `GetGuidMapping(...)`

Gets the GUID mapping.

[Docs](https://developer.tekla.com/topic/en/18/47/53a1428d-6004-3a22-ae05-3736cf718674)

#### `GetNewGuid(...)`

Gets the new GUID.

[Docs](https://developer.tekla.com/topic/en/18/47/772a3a7a-8ebb-fbd1-7e80-885d4127556d)

## Operation (class)

The Operation class implements Tekla Structures level operations.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d5bbc096-a164-49ea-16a8-3820e40b00be)

### Methods
#### `AddToPourUnit(...)`

Adds model objects as part of a pour unit Model object types accepted are assembly types except cast in situ, reinforcements of different kind, components and bolts

[Docs](https://developer.tekla.com/topic/en/18/47/e0d977f1-f6d9-4b5d-09ff-39534452abbd)

#### `CalculatePourUnits(...)`

Calculate and assign objects to pour unit Model object types that are associated with pour unit are assembly types except cast in situ, reinforcements of different kind, components and screws

[Docs](https://developer.tekla.com/topic/en/18/47/90216a45-57e6-4bd6-6338-f18371c6b2c2)

#### `Combine(Beam, Beam)(...)`

Combines two beams into one beam.

[Docs](https://developer.tekla.com/topic/en/18/47/ee549780-0d63-26ba-456a-49cfdb50da7b)

#### `Combine(RebarGroup, RebarGroup)(...)`

Combines two rebar groups into one rebar group.

[Docs](https://developer.tekla.com/topic/en/18/47/3147d034-cbbc-cd89-46ee-142d4658e591)

#### `Combine(SingleRebar, SingleRebar)(...)`

Combines two single rebars into one rebar.

[Docs](https://developer.tekla.com/topic/en/18/47/102fb25d-b5f4-6506-136d-36f88a02eb85)

#### `ConvertPartToItem(...)`

Convert a part to an item. This will also create a new shape in the shape catalog.

[Docs](https://developer.tekla.com/topic/en/18/47/92da650f-fa05-8dc5-d313-ca063058634d)

#### `CopyObject(ModelObject, CoordinateSystem, CoordinateSystem)(...)`

Copies the model object between the given translation coordinate systems.

[Docs](https://developer.tekla.com/topic/en/18/47/d67c7649-00ef-f48a-bbf8-5bedf53ce30d)

#### `CopyObject(ModelObject, Vector)(...)`

Copies the model object using the given translation vector.

[Docs](https://developer.tekla.com/topic/en/18/47/9ff01c4b-9f9a-aedd-731b-c8e5b04f50e0)

#### `CopyObjectMirror(...)`

Creates a mirrored copy of the given model object.

[Docs](https://developer.tekla.com/topic/en/18/47/2d75b51c-f658-03f0-dadb-7e2588e98461)

#### `CreateBentPlateByFaces(Part, Face, Part, Face)(...)`

Modifies the first plate by adding a cylindrical bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. See CreateBentPlateByFaces(Part, Face, Part, Face, BentPlate.BendShape).

[Docs](https://developer.tekla.com/topic/en/18/47/f401c9ed-80c1-f692-6736-73d779c8577e)

#### `CreateBentPlateByFaces(Part, Face, Part, Face, BentPlate.BendShape)(...)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, BentPlate.BendShape).

[Docs](https://developer.tekla.com/topic/en/18/47/621e8e34-f192-2b5e-4be6-03ef655300ca)

#### `CreateBentPlateByFaces(Part, Face, Part, Face, Double)(...)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part and radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

[Docs](https://developer.tekla.com/topic/en/18/47/3001ef45-94f0-0b94-2843-907272f839e5)

#### `CreateBentPlateByFaces(Part, IList.Point., Part, IList.Point.)(...)`

Modifies the first plate by adding a cylindrical bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. See CreateBentPlateByFaces(Part, IList.Point., Part, IList.Point., BentPlate.BendShape).

[Docs](https://developer.tekla.com/topic/en/18/47/1d1f228a-de86-1150-f8eb-12ab1cfc98f9)

#### `CreateBentPlateByFaces(Part, IList.Point., Part, IList.Point., BentPlate.BendShape)(...)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, BentPlate.BendShape).

[Docs](https://developer.tekla.com/topic/en/18/47/0dca7df4-8430-6f6c-e648-2d40134a06a7)

#### `CreateBentPlateByFaces(Part, IList.Point., Part, IList.Point., Double)(...)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts, selected faces in each part and radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

[Docs](https://developer.tekla.com/topic/en/18/47/c7e50f00-b8cd-3418-4bfb-561fdf9bae0c)

#### `CreateBentPlateByParts(Part, Part)(...)`

Modifies the first plate by adding a cylindrical bend that connects it to the second plate creating a new BentPlate instance based on two parts. See CreateBentPlateByParts(Part, Part, BentPlate.BendShape).

[Docs](https://developer.tekla.com/topic/en/18/47/693a8447-9e68-ed71-dc28-fb0a1753e45a)

#### `CreateBentPlateByParts(Part, Part, BentPlate.BendShape)(...)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, ConnectiveGeometry, BentPlate.BendShape).

[Docs](https://developer.tekla.com/topic/en/18/47/a9f4e2ea-0881-3278-a468-ff1d483544d3)

#### `CreateBentPlateByParts(Part, Part, Double)(...)`

Modifies the first plate by adding a bend that connects it to the second plate creating a new BentPlate instance based on two parts and a radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, ConnectiveGeometry, Double).

[Docs](https://developer.tekla.com/topic/en/18/47/49a6d6e3-83ec-e179-c4c0-7545a44723d6)

#### `CreateConicalBentPlateByFaces(Part, Face, Part, Face, Double, Double)(...)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts and selected faces in each part, and the largest radius of the conical section and the cone aperture. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

[Docs](https://developer.tekla.com/topic/en/18/47/3e17cb16-5db2-96ff-a9bc-7b6ac320e2b9)

#### `CreateConicalBentPlateByFaces(Part, IList.Point., Part, IList.Point., Double, Double)(...)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts, selected faces in each part and radius. This method can change GUID when using from plug-ins. To keep GUID, use AddLeg(ConnectiveGeometry, LineSegment, ConnectiveGeometry, LineSegment, Double).

[Docs](https://developer.tekla.com/topic/en/18/47/b70630f0-c290-2237-1249-f37b1e6d4469)

#### `CreateConicalBentPlateByPartsAndAperture(...)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts. The resulting bend will have the given aperture and the provided larger radius.

[Docs](https://developer.tekla.com/topic/en/18/47/ec65a439-38a2-82af-b6bf-a97b395a9b70)

#### `CreateConicalBentPlateByPartsAndTwoRadiuses(...)`

Modifies the first plate by adding a conical bend that connects it to the second plate creating a new BentPlate instance based on two parts. The resulting bend will have the two given radiuses.

[Docs](https://developer.tekla.com/topic/en/18/47/986edfe5-9465-7814-159b-cef1dee20d5a)

#### `CreateDGNv8Export(...)`

Dgn export from all or selected objects.

[Docs](https://developer.tekla.com/topic/en/18/47/470aa2f9-a4d1-5ba5-9675-7f88e88a08d7)

#### `CreateIFC2x3Export(...)`

Creates IFC2x3 export using the given file name. See Tekla Structures Help for more information about IFC2x3 export files.

[Docs](https://developer.tekla.com/topic/en/18/47/d455e837-b27d-de76-3f8f-f4c8c98bc344)

#### `CreateIFC4ExportFromAll(String, Operation.IFCExportViewTypeEnum, IEnumerable.String., Operation.ExportBasePoint, String, String, Operation.IFCExportFlags, String)(...)`

Creates IFC4 export from all objects using the given file name. See Tekla Structures Help for more information about IFC4 export files.

[Docs](https://developer.tekla.com/topic/en/18/47/35b592da-834c-6c6f-b8ee-5cb14b2d8df0)

#### `CreateIFC4ExportFromAll(String, Operation.IFCExportViewTypeEnum, Operation.IFCExportVersionEnum, IEnumerable.String., IEnumerable.String., Operation.ExportBasePoint, String, String, Operation.IFCExportFlags, String)(...)`

Creates IFC4 export from all objects using the given file name. See Tekla Structures Help for more information about IFC4 export files.

[Docs](https://developer.tekla.com/topic/en/18/47/84a2b76e-89c5-4976-c391-de09f4f12712)

#### `CreateIFC4ExportFromSelected(String, Operation.IFCExportViewTypeEnum, IEnumerable.String., Operation.ExportBasePoint, String, String, Operation.IFCExportFlags, String)(...)`

Creates IFC4 export from the selected parts using the given file name. See Tekla Structures Help for more information about IFC4 export files.

[Docs](https://developer.tekla.com/topic/en/18/47/2e42dad3-18a9-cf38-79a7-f9ca7ddbee07)

#### `CreateIFC4ExportFromSelected(String, Operation.IFCExportViewTypeEnum, Operation.IFCExportVersionEnum, IEnumerable.String., IEnumerable.String., Operation.ExportBasePoint, String, String, Operation.IFCExportFlags, String)(...)`

Creates IFC4 export from the selected parts only using the given file name. See Tekla Structures Help for more information about IFC4 export files.

[Docs](https://developer.tekla.com/topic/en/18/47/0fbe065a-e49b-bd3e-850c-655f7934a85c)

#### `CreateIFC4ExportFromSelectedAndTheirAssemblies(...)`

Creates IFC4 export from the selected parts and their assemblies using the given file name. See Tekla Structures Help for more information about IFC4 export files.

[Docs](https://developer.tekla.com/topic/en/18/47/194b2297-3487-1711-3a17-5c6d7a3ec795)

#### `CreateMISFileFromAll(...)`

Creates MIS files from all parts using the given file name. See Tekla Structures Help for more information about MIS files.

[Docs](https://developer.tekla.com/topic/en/18/47/3090fd50-4614-041c-e0cf-85edc182e5c5)

#### `CreateMISFileFromSelected(...)`

Creates MIS files from the selected parts using the given file name. See Tekla Structures Help for more information about MIS files.

[Docs](https://developer.tekla.com/topic/en/18/47/4d66ce5b-826f-0efd-a9cd-e6ca304e2d93)

#### `CreateNCFilesByPartId(...)`

Creates NC files from the selected parts using the given NC template name. See Tekla Structures Help for more information about NC files.

[Docs](https://developer.tekla.com/topic/en/18/47/39156aa2-9243-836c-f5cd-66cef7b6728f)

#### `CreateNCFilesFromAll(String, String)(...)`

Creates NC files from all parts using the given NC template name. See Tekla Structures Help for more information about NC files.

[Docs](https://developer.tekla.com/topic/en/18/47/736ca893-968e-c433-7b5d-4830dd055711)

#### `CreateNCFilesFromAll(String, String, Boolean, String, Boolean, String)(...)`

Creates NC files from all parts using the given NC template name. See Tekla Structures Help for more information about NC files.

[Docs](https://developer.tekla.com/topic/en/18/47/1b185764-bea1-d294-a375-c32ab26d4d64)

#### `CreateNCFilesFromSelected(String, String)(...)`

Creates NC files from the selected parts using the given NC template name. See Tekla Structures Help for more information about NC files.

[Docs](https://developer.tekla.com/topic/en/18/47/a832ed01-97e7-61c8-ee9e-bfdb0d0239ad)

#### `CreateNCFilesFromSelected(String, String, Boolean, String, Boolean, String)(...)`

Creates NC files from the selected parts using the given NC template name. See Tekla Structures Help for more information about NC files.

[Docs](https://developer.tekla.com/topic/en/18/47/8f4e08b2-41eb-d6a1-859d-ddb50111d9e5)

#### `CreateReport(...)`

Creates a report using the given report parameters. A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.See Tekla Structures Help for more information about reports.

[Docs](https://developer.tekla.com/topic/en/18/47/3235aaad-349c-8cc1-1222-5f4e3c994561)

#### `CreateReportFromAll(...)`

Creates a report from all the objects using the given template name and filename. A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.Internally, this method is asynchronous, and because of that the output file cannot be immediately available.See Tekla Structures Help for more information about reports.

[Docs](https://developer.tekla.com/topic/en/18/47/b4ff470c-a784-75b1-3204-0bfe7cfcdf04)

#### `CreateReportFromSelected(...)`

Creates a report from the selected objects using the given template name and filename. A template with the name given in the TemplateName parameter must exist in model folder or in the folder defined with the advanced option XS_TEMPLATE_DIRECTORY.If a path is not given in the filename, the file is created to the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.If the given folder does not exist, the report creation fails.See Tekla Structures Help for more information about reports.

[Docs](https://developer.tekla.com/topic/en/18/47/20138139-84ec-063b-734f-d73210a43196)

#### `CreateShapeFromGeometry(...)`

Create a new shape in the shape catalog, based on the part geometry.

[Docs](https://developer.tekla.com/topic/en/18/47/f742bc27-2758-364e-bab3-51554907d1d3)

#### `DisplayPrintReportDialog(...)`

Opens the print dialog for a report with the given filename.

[Docs](https://developer.tekla.com/topic/en/18/47/8e7486cf-b2bd-3400-3d72-5f7003a421dd)

#### `DisplayPrompt(...)`

Displays a message in the status bar.

[Docs](https://developer.tekla.com/topic/en/18/47/0be24953-f509-559d-1682-51d772836434)

#### `DisplayReport(...)`

Opens and displays a report with the given name. If a path is not given in the filename, the file is searched from the folder defined with the advanced option XS_REPORT_OUTPUT_DIRECTORY.See Tekla Structures Help for more information about reports.

[Docs](https://developer.tekla.com/topic/en/18/47/f0341426-3054-511a-e795-d26e240bcd95)

#### `ExplodeBentPlate(...)`

Deletes bentPlate and inserts ContourPlates instances equivalent to the ones used to create bentPlate. The ContourPlate created from the main polygon has the same identifier as bentPlate.

[Docs](https://developer.tekla.com/topic/en/18/47/90695733-55df-251e-05cd-b9115f915769)

#### `Freeze(...)`

Freezes or unfreezes a rebar set.

[Docs](https://developer.tekla.com/topic/en/18/47/0ed78310-48b6-2e28-a003-5ab7932edfcb)

#### `GetHandlePoints(...)`

Get the Shape Item Handle Points

[Docs](https://developer.tekla.com/topic/en/18/47/75751ea1-681e-d44e-de08-475b46d257df)

#### `GetSimilarNumberedObjects(...)`

Gets similar objects based on numbering of given object.

[Docs](https://developer.tekla.com/topic/en/18/47/5f906cbe-5507-55e3-d31f-00043acf148a)

#### `Group(...)`

Groups a list of single rebars or rebar groups and creates a new rebar group.

[Docs](https://developer.tekla.com/topic/en/18/47/179f6a0a-95ea-6517-ddcd-b9a82c374d25)

#### `Highlight(...)`

Highlight the list of objects from the user interface. Only the list of objects will be highlighted, the rest will be unhighlighted. But it will keep the status of the selected objects. To unhighlight give an empty list as a parameter.

[Docs](https://developer.tekla.com/topic/en/18/47/b67e14f2-d5c5-c685-d527-b9d451460899)

#### `IsMacroRunning(...)`

Returns true if a macro is running, false otherwise. Macros are saved as *.cs files in the folder defined with the XS_MACRO_DIRECTORY variable.See Tekla Structures Help for more information about macros.

[Docs](https://developer.tekla.com/topic/en/18/47/2f242e58-9e53-908e-5f49-6e67f27b3ae4)

#### `IsModelAutoSaved(...)`

Tells whether a model has auto saved information.

[Docs](https://developer.tekla.com/topic/en/18/47/afcedc6c-a2a5-12a2-f8f3-03f0a83283fa)

#### `IsNumberingAllowed(...)`

Checks whether the numbering is allowed or not in the current configuration.

[Docs](https://developer.tekla.com/topic/en/18/47/3bc7887e-5852-1d2f-1de8-b0eebb84b6f7)

#### `IsNumberingUpToDate(...)`

Checks whether the numbering is up-to-date for an assembly, part, rebar, surface treatment, pour object or break.

[Docs](https://developer.tekla.com/topic/en/18/47/27f48c6d-a775-ddf5-c54c-f168fc904322)

#### `IsNumberingUpToDateAll(...)`

Checks whether the numbering is up-to-date for every assembly, part and rebar on the model. Remarks Using this method is much faster than checking each object individually.

[Docs](https://developer.tekla.com/topic/en/18/47/92637418-2b30-ce77-7dc1-038322a9544a)

#### `MoveObject(ModelObject, CoordinateSystem, CoordinateSystem)(...)`

Moves the model object between the given translation coordinate systems.

[Docs](https://developer.tekla.com/topic/en/18/47/c8c000f1-f5d4-0429-45c4-021bdf02036c)

#### `MoveObject(ModelObject, Vector)(...)`

Moves the model object using the given translation vector.

[Docs](https://developer.tekla.com/topic/en/18/47/e0d1a2cd-6145-ff68-a8f0-f0e74c1dc78f)

#### `MoveObjectMirror(...)`

Mirrors the object in the plane defined by the given data is the current workplane.

[Docs](https://developer.tekla.com/topic/en/18/47/515aa261-bbf2-764d-45c2-a2ebddace452)

#### `ObjectMatchesToFilter(ModelObject, FilterExpression)(...)`

Checks whether the object matches to the criteria in the given filter.

[Docs](https://developer.tekla.com/topic/en/18/47/05260766-8da9-47ba-7421-8dc18edce140)

#### `ObjectMatchesToFilter(ModelObject, String)(...)`

Checks whether the object matches to the criteria in the given filter.

[Docs](https://developer.tekla.com/topic/en/18/47/25fb8b93-31f3-5c3e-3538-263151971be2)

#### `Open(String)(...)`

Opens a new model to Tekla Structures ignoring auto saved information.

[Docs](https://developer.tekla.com/topic/en/18/47/2c908a15-5315-833e-9794-bf16a0687e3d)

#### `Open(String, Boolean)(...)`

Opens a new model to Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/90bb1f01-bf9a-9bab-6a7e-12ce9ca36a0b)

#### `RemoveFromPourUnit(...)`

Removes model object from pour unit Model object types accepted are assembly types except cast in situ, reinforcements of different kind, components and bolts

[Docs](https://developer.tekla.com/topic/en/18/47/ed267ff7-02e0-bd4c-36a8-bc7e258d3c83)

#### `RunMacro(...)`

Starts a macro with the given name. Throws an exception if the file is not found. Macros are saved as *.cs files in the folder defined with the XS_MACRO_DIRECTORY variable.It is possible to run drawing macros using relative paths.See Tekla Structures Help for more information about macros.

[Docs](https://developer.tekla.com/topic/en/18/47/42bee48d-632b-6b20-6c82-3e88af869716)

#### `SaveAsWebModel(...)`

Saves the current model as a web model. You can save the model as a web model that can be viewed via the Internet using a web browser (e.g. Internet Explorer).

[Docs](https://developer.tekla.com/topic/en/18/47/00aec8f7-3875-a8db-f76a-967ac47d9496)

#### `SaveSelectedAsWebModel(...)`

Saves the selected objects as a web model. You can save the selected objects as a web model that can be viewed via the Internet using a web browser (e.g. Internet Explorer).

[Docs](https://developer.tekla.com/topic/en/18/47/7c54947d-03b0-32c4-5562-57889c799cf2)

#### `SetHandlePoints(...)`

Set the Shape Item Handle Points

[Docs](https://developer.tekla.com/topic/en/18/47/7baa03fe-b8f6-c104-8642-747666367f6e)

#### `ShowOnlySelected(...)`

Show Only Selected objects in current view.

[Docs](https://developer.tekla.com/topic/en/18/47/ef58f1d7-1bc2-ad42-f1f5-21f092444369)

#### `Split(Beam, Point)(...)`

Splits the beam and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/ade59197-8112-9d79-3d9d-0de5673413f1)

#### `Split(CircleRebarGroup, Line)(...)`

Splits the circle rebar group and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/38823af2-3c64-c8c2-5399-b43e45fb039f)

#### `Split(ContourPlate, Polygon)(...)`

Splits the contour plate and creates a new one along the given polygon.

[Docs](https://developer.tekla.com/topic/en/18/47/cc26cef4-45ff-a6aa-dc13-ba549930cf47)

#### `Split(CurvedRebarGroup, Line)(...)`

Splits the curved rebar group and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/2aadb317-0235-c6e1-778a-dd605cf16913)

#### `Split(LoftedPlate, Point)(...)`

Splits the lofted plate and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/9f4fe99c-fdef-f833-52be-627717373b13)

#### `Split(PolyBeam, Point)(...)`

Splits the polybeam and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/060e6ed9-cce7-0799-e26b-19403dde7f61)

#### `Split(RebarGroup, Line)(...)`

Splits the rebar group and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/e0678430-540d-a572-190e-7d35dfdae18d)

#### `Split(RebarSet, Line)(...)`

Splits the rebar set and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/ff23c743-8528-91b0-41d0-7b84c96407ed)

#### `Split(SingleRebar, Line)(...)`

Splits the single rebar and creates a new one in the given position.

[Docs](https://developer.tekla.com/topic/en/18/47/e4368110-15e9-c24a-02d2-1bcee426db55)

#### `SplitSlab(...)`

This command is meant for specifically splitting a concrete slab with advanced solid operations to create more robust and user friendly results than the command: public static ContourPlate Split(ContourPlate Object, Polygon SplitPolygon). No validation is done for the type, it is the caller's responsibility to call this only for valid types (slabs). Behavior for non-slabs is undetermined.

[Docs](https://developer.tekla.com/topic/en/18/47/82d2dc9c-b4c0-d413-4570-9c2e6ecb7f91)

#### `Ungrouping(RebarGroup)(...)`

Ungroups the rebar group and creates new single rebars.

[Docs](https://developer.tekla.com/topic/en/18/47/9c5db27f-c493-c9b1-2493-91cfc61bfb83)

#### `Ungrouping(RebarMesh)(...)`

Ungroups the rebar mesh and creates new single rebars.

[Docs](https://developer.tekla.com/topic/en/18/47/02acb6c5-8915-ea56-1399-1a112a57961e)

## Operation.ExportBasePoint (enum)

Export base point

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a23be964-0f05-80db-4c82-afd85fd91a77)

## Operation.IFC2x3ExportFlags (struct)

Boolean flags for IFC 2x3 export.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7ded9144-e182-c22c-f38e-230f52071919)

### Properties
- `ExcludeSinglePartAssemblies` (object, get/set) — Boolean indicating whether single part assemblies are excluded.
- `ExportAllObjects` (object, get/set) — Boolean indicating whether to export all objects.
- `ExportBrepAsExactSolid` (object, get/set) — Boolean indicating whether BREP is exported as exact solid.
- `ExportBuildingHierarchy` (object, get/set) — Boolean indicating whether the building hierarchy export is enabled.
- `ExportFlatBeamsAsPlates` (object, get/set) — Boolean indicating whether flat beams are exported as plates.
- `ExportLocationsFromOrganizer` (object, get/set) — Boolean indicating whether locations are obtained from the organizer.
- `ExportPolyBeamAsSectionedSpine` (object, get/set) — Boolean indicating whether polybeams are exported as sectioned spines.
- `ExportSelectedObjectsAndTheirAssemblies` (object, get/set) — Boolean indicating whether to export selected objects and their assemblies.
- `IncludeAssemblyHiearachy` (object, get/set) — Boolean indicating whether the assembly hierarhcy (IFCELEMENTASSEMBLY) is exported.
- `IsBoltsEnabled` (object, get/set) — Boolean indicating whether the bolts export is enabled.
- `IsGridsEnabled` (object, get/set) — Boolean indicating whether the grids export is enabled.
- `IsPoursEnabled` (object, get/set) — Pours export is enabled. True for pours, false for CIP cast-units and/or parts
- `IsRebarsEnabled` (object, get/set) — Boolean indicating whether the rebars export is enabled.
- `IsSpacesEnabled` (object, get/set) — Boolean indicating whether the export of spaces in building hierarchy is enabled.
- `IsSurfaceTreatmentsAndSurfacesEnabled` (object, get/set) — Boolean indicating whether the surfaces and surface treatments export is enabled.
- `IsWeldsEnabled` (object, get/set) — Boolean indicating whether the welds export is enabled.

## Operation.IFC2x3ExportViewTypeEnum (enum)

View configuration type for IFC 2x3 export

[Vendor docs](https://developer.tekla.com/topic/en/18/47/76a2a7d0-56a6-a8ee-1667-c264f10424df)

## Operation.IFCExportFlags (struct)

Boolean flags for IFC 4 export.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a79a1b5b-f9a9-9753-0d4e-f97962336baa)

### Properties
- `UseIfcMapConversion` (object, get/set) — Boolean indicating whether to use IfcMapConversion with base points is enabled. By default IFC4 export should use IfcMapConversion. Uninitialized null will be converted to true during export.

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
- `Operation.ProgressBar(...)` — Initializes a new instance of the Operation.ProgressBar class

### Methods
#### `Canceled(...)`

Check if cancel has been pressed.

[Docs](https://developer.tekla.com/topic/en/18/47/65b92871-fd3b-0b78-8047-52a5973e9271)

#### `Close(...)`

Close progress bar. Can be called even if Display was not successful.

[Docs](https://developer.tekla.com/topic/en/18/47/b0de203c-dfc1-a501-a91b-29428d25567a)

#### `Display(...)`

Display progress bar dialog with cancel button. Display will fail if progress bar is already displayed.

[Docs](https://developer.tekla.com/topic/en/18/47/6758e312-e9fb-ddeb-9631-20faa288a575)

#### `SetProgress(...)`

Update status information on the progress bar.

[Docs](https://developer.tekla.com/topic/en/18/47/4868a504-d0dd-c456-1f41-48eb4ebf8443)

## Operation.ReportParameters (class)

Represents the parameters required for generating a report.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f3230537-529d-b6e5-72dc-e718a4190599)

### Constructors
- `Operation.ReportParameters(...)` — Initializes a new instance of the Operation.ReportParameters class

### Properties
- `FileName` (object, get/set) — Gets or sets the name of the created report. The name must contain more than three characters.
- `IsFromSelected` (object, get/set) — Gets or sets a value indicating whether the report is from selected or from all objects.
- `TemplateName` (object, get/set) — Gets or sets the name of the report template to be used in report creation. The name must contain more than three characters.
- `Title1` (object, get/set) — Gets or sets the first title for the created report.
- `Title2` (object, get/set) — Gets or sets the second title for the created report.
- `Title3` (object, get/set) — Gets or sets the third title for the created report.
- `XLSXFlags` (object, get/set) — Gets or sets the flags used for processing XLSX files.
- `XLTXFileName` (object, get/set) — Gets or sets the name of the Excel template to be used.

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

