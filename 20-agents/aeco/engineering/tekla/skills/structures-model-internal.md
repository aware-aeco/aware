---
name: tekla-structures-model-internal
description: "Tekla Open API - Tekla.Structures.ModelInternal.Operation. 271 static utility methods for undo/redo, export (IFC/DWG/DGN/Revit), advanced options, TQL queries, reference models, component attributes, view/selection, numbering, session log, temp colors + 777 dotStartCommand/dotStartAction command names for programmatic Tekla automation. This skill should be used when the public Operation API is insufficient."
---

# Tekla.Structures.ModelInternal.Operation API Reference (v2025)

## Important

This is an **internal/unsupported** API (`Tekla.Structures.ModelInternal`). Methods may change between Tekla versions without notice. Use the public `Tekla.Structures.Model.Operations.Operation` API when possible; fall back to `ModelInternal.Operation` only when needed functionality is not available publicly.

**Namespace:** `Tekla.Structures.ModelInternal`
**Type:** `static class Operation` (271 static methods, 15 nested types)

---

## Undo / Redo

- `void dotUndo()` - undo last operation
- `void dotRedo()` - redo last undone operation
- `void dotClearUndoLog()` - clear undo history
- `string GetLatestUndoLogInfo()` - get latest undo log entry
- `string GetCurrentUndoLogInfo()` - get current undo log info

---

## Model Save / Load / Quit

- `bool dotSaveModel(string Comment, string User, string Reason)`
- `bool dotSaveAsModel(string path, string Comment, string User, string Reason)`
- `bool dotAutoSaveModel(string Comment, string User, string Reason)`
- `bool dotQuitProgram(string Comment, string User, string Reason)`
- `bool dotLoadModel2(string folder, string modelName)` - load overlay model
- `void dotUnloadModel2()` - unload overlay model
- `bool dotIsModelSaved(string ModelFolder)`
- `bool dotCreateNewSingleUserModel(string ModelName, string ModelPath)`
- `bool dotCreateNewSingleUserModelFromTemplate(string ModelName, string ModelPath, string ModelTemplateNameOrPath)`
- `bool dotCreateNewMultiUserModel(string ModelName, string ModelPath, string ServerName)`
- `bool dotCreateNewSharedModel(string ModelName, string ModelPath)`
- `bool dotConnectToNewMultiUserServerAndOpenModel(string ModelFolder, string ServerName)`
- `bool dotConvertAndOpenAsMultiUserModel(string ModelFolder, string ServerName)`
- `bool dotConvertAndOpenAsSingleUserModel(string ModelFolder)`
- `bool dotGetDataBaseVersionInfoFromModel(string ModelName, string ModelPath, ref int ModelVersion, ref int CurrentVersion)`
- `int dotGetDatabaseVersion()`
- `void AllowAutoSave(bool allow)`

---

## Advanced Options

- `bool dotSetAdvancedOption(string VariableName, string Value)`
- `bool dotSetAdvancedOption(string VariableName, int Value)`
- `bool dotSetAdvancedOption(string VariableName, double Value)`
- `bool dotSetAdvancedOption(string VariableName, bool Value)`
- `bool dotResetUserOptionToDefaultValue(string VariableName)`

---

## TQL (Tekla Query Language)

- `int TqlExecute(string TqlQuery, out string TqlResult)` - execute TQL query, returns result count

---

## IFC Export

- `bool ExportIFCFromAll(string ModelName, string FullFileName, IFCExportViewTypeEnum ViewType, List<string> PropertySets, ExportBasePoint BasePoint, string ExportLayersAs, string ObjectColoring, IFCExportFlags Flags, bool UseTimer, bool CreateReport, string BasePointGuid)`
- `bool ExportIFCFromSelected(string ModelName, string FullFileName, IFCExportViewTypeEnum ViewType, List<string> PropertySets, ExportBasePoint BasePoint, string ExportLayersAs, string ObjectColoring, IFCExportFlags Flags, bool UseTimer, bool CreateReport, string BasePointGuid)`
- `bool ExportIFCFromObjects(string ModelName, string FullFileName, IFCExportViewTypeEnum ViewType, List<string> PropertySets, List<ModelObject> ModelObjects, ExportBasePoint BasePoint, string ExportLayersAs, string ObjectColoring, IFCExportFlags Flags, bool UseTimer, bool CreateReport, string BasePointGuid)`
- `bool ExportIFCFromFilteredObjects(string ModelName, string FullFileName, IFCExportViewTypeEnum ViewType, List<string> PropertySets, string FilterName, ExportBasePoint BasePoint, string ExportLayersAs, string ObjectColoring, IFCExportFlags Flags, bool UseTimer, bool CreateReport, string BasePointGuid)`
- `bool ExportMEPIFC(string ModelName, string FullFileName, MEPIFCExportViewTypeEnum ViewType, List<string> PropertySets, string FilterName, bool FromSelected, ExportBasePoint BasePoint, string ExportLayersAs, string ObjectColoring, IFCExportFlags Flags, bool CreateReport, string BasePointGuid)`
- `bool ConvertIFC4toSemantic(string ifc4File, string outputPath)`
- `string ConvertIfcGuidToTsGuid(string ifcGuid)`
- `string ConvertTsGuidToIfcGuid(string tsGuid)`

---

## DWG / DGN / Revit Export

- `int ExportDwgFromAll(string FileName, string Folder, ExportBasePoint BasePoint, string BasePointGuid, string ObjectColoring, string ExportLayersAs, out int exportedObjects, out int exportFailedObjects, out string userName)`
- `int ExportDwgFromSelected(string ModelName, string FullFileName, ExportBasePoint BasePoint, string BasePointGuid, string ObjectColoring, string ExportLayersAs, out int exportedObjects, out int exportFailedObjects, out string userName)`
- `int ExportDgnFromAll(string FileName, string Folder, ExportBasePoint BasePoint, string BasePointGuid, string ObjectColoring, string ExportLayersAs, out int exportedObjects, out int exportFailedObjects, out string userName)`
- `int ExportDgnFromSelected(string ModelName, string FullFileName, ExportBasePoint BasePoint, string BasePointGuid, string ObjectColoring, string ExportLayersAs, out int exportedObjects, out int exportFailedObjects, out string userName)`
- `int ExportRevitFromAll(string FileName, string Folder, ExportBasePoint BasePoint, string BasePointGuid, bool UpdateSelected, out int exportedObjects, out int exportFailedObjects, out string userName)`
- `int ExportRevitFromSelected(string FileName, string Folder, ExportBasePoint BasePoint, string BasePointGuid, bool UpdateSelected, out int exportedObjects, out int exportFailedObjects, out string userName)`
- `bool ExportModel(string outPath, string basePointGuid, string basePointExportType, string exportType, string propertiesPath, bool selectedOnly, dotCreateTrimBimFromModel_t info, Action<string, int> progressCb)` - generic export (TrimBim etc.)

---

## DSTV / NC

- `bool CreateDstvStructure(Identifier PartID, string NCFileSettings, out DstvStructure DstvOutput, bool CreatePopMarks, string PopMarkSettingsFileName, bool CreateContourMarking, string ContourMarkingSettingsFileName)`

---

## View / Selection / UI

- `View GetCurrentView()` - get the currently active view
- `bool Select(ArrayList ModelObjects, bool ShowDimensions, bool SuppressCallbacks)` - select objects
- `bool SelectObjectsFromModel(IEnumerable<string> guids, bool zoomTo)` - select by GUIDs with optional zoom
- `Point GetMousePositionInModelCoordinates()` - get current mouse position in model space
- `bool GetViewRotationPointFromCurrentView(ref Point viewPoint)` - get view rotation point
- `void SetViewPartRepresentationState(ViewPartRepresentationEnum RepresentationState)` - set rendering mode
- `bool InsertView(View view, bool openView, bool useAppliedValues)` - insert a new view
- `bool InsertViewByStandardFile(string standardFile, View view)` - insert view from standard file
- `bool Interrupt()` - interrupt current operation
- `List<int> GetHiddenObjects()` - get IDs of hidden objects

---

## Temporary Colors / States

- `bool ClearTemporaryColor(List<ModelObject> modelObjects)` - clear temp colors
- `bool GetTemporaryColor(int ID, out Color color)` - get temp color for object
- `bool dotExportGetColorRepresentationForObject(int ID, ref Color color)` - get color representation

---

## Object Fetch / Filter

- `List<ModelObject> FetchModelObjects(List<string> guids, bool SelectInstances)` - fetch by GUIDs
- `List<ModelObject> FetchModelObjects(List<int> ids, bool SelectInstances)` - fetch by IDs
- `List<ModelObject> ObjectsMatchToFilter(List<ModelObject> modelObjects, FilterExpression filterExpression)` - batch filter by expression
- `List<ModelObject> ObjectsMatchToFilter(List<ModelObject> modelObjects, string filterName)` - batch filter by name
- `List<int> GetRunTimeIdentifiersByGuid(List<string> guids)` - convert GUIDs to runtime IDs

---

## Component Attributes

- `Dictionary<string, object> ComponentGetAppliedAttributes(BaseComponent baseComponent)` - get all applied attributes
- `Dictionary<string, object> ComponentGetAppliedAttributes(BaseComponent baseComponent, bool ignoreDefaultValues)`
- `Dictionary<string, object> ComponentApplyAndGetAttributesFromFile(BaseComponent baseComponent, string fileName, bool ignoreDefaultValues)` - apply from file and get
- `bool ComponentSetAppliedAttributes(BaseComponent baseComponent, ref Dictionary<string, object> appliedAttributes)` - set attributes
- `bool ComponentSetAppliedAttributes(BaseComponent baseComponent, ref Dictionary<string, object> appliedAttributes, bool addNewAttributes)`
- `bool ComponentSaveAppliedAttributes(BaseComponent baseComponent, string fileName)` - save to file
- `bool ComponentDeleteAttributes(BaseComponent baseComponent, ref List<string> attributeNames)` - delete attributes
- `string ComponentGetAttributeFileExtension(BaseComponent baseComponent)` - get file extension for component type

---

## Numbering

- `bool AcceptNumberingResults()` - accept numbering preview
- `bool RejectNumberingResults()` - reject numbering preview
- `bool RunNumberingCommandWithPreview(bool NumberSelectedSeries, bool IsFullNumbering)` - run numbering with preview
- `bool ProcessABMNumbering(List<int> selectedModelObjects)` - process ABM numbering
- `bool CheckABMNumbering(ref List<CheckResult> checkResults)` - check ABM numbering
- `bool GetAbmSettings(ref dotAbmSettings_t pAbmSettings)` / `bool SetAbmSettings(ref dotAbmSettings_t pAbmSettings)`
- `bool GetNumberingSettings(ref dotNumberingSettingsType_t pNumberingSettings)`
- `byte[] GetNumberingTemporaryRecordList()`
- `bool UnnumberProcurementParts(List<int> selectedModelObjects)`
- `bool SendCustomPartsToABMNumbering(List<int> partIds, string propertiesPath, ref string changesFilePath, ref string allHistoryFilePath)`

---

## Reference Models (External)

- `bool ImportExternalReferenceModel(string fileAndPath, string basePointGuid, Point origin, Point axisX, Point axisY, double scale, string tcModelId, string tcModelVersionId, string tCModelName)`
- `bool RemoveExternalReferenceModels(IEnumerable<string> paths)`
- `bool ShowExternalReferenceModels(IEnumerable<string> paths, bool restrictToWorkArea)`
- `bool HideExternalReferenceModels(IEnumerable<string> paths)`
- `bool ZoomToExternalReferenceModel(string path)`
- `bool ZoomViewToExternalReferenceModel(string path, View view)`
- `bool SetSelectedExternalReferenceModel(string path)`
- `bool SelectObjectsInExternalReferenceModel(string path, IEnumerable<string> objectGuids)`
- `bool SetExternalReferenceModelLayerVisibility(string path, int index, VisibilityEnum visibility)`
- `bool SetExternalReferenceModelLayerVisibilities(string path, IEnumerable<dotExternalReferenceModelLayer_t> layers)`
- `bool SetExternalReferenceModelObjectColoring(string path, IEnumerable<string> objectGuids, double red, double green, double blue, double alpha)`
- `bool ResetExternalReferenceModelColoring(string path, IEnumerable<string> objectGuids)`
- `bool SetExternalReferenceModelObjectVisibilities(string path, IEnumerable<string> objectGuids, bool isVisible)`
- `bool StartExternalReferenceModelChangeDetection(string currentVersion, string newVersion, int changed, int inserted, int deleted, int unchanged, string comparisonSetTitle, string tolerancesTitle)`
- `bool StopExternalReferenceModelChangeDetection()`
- `bool CancelChangeDetectionInit()` / `bool ChangeDetectionInitInProgress()`
- `bool ConvertReferenceModels()`
- `bool ImportBuildingHierarchyDataFromReferenceModel(string referenceModelFile, int referenceModelId, bool includeSpaces)`
- `IEnumerable<string> GetImportedExternalReferenceModels()`
- `List<dotExternalReferenceModelLayer_t> GetExternalReferenceModelLayers(string path)`
- `int GetExternalReferenceModelEntitiesCount(string path)`
- `string GetExternalReferenceModelPath(string modelTsGuid)`
- `IEnumerable<dotExternalReferenceModelSelection_t> GetExternalReferenceModelSelections()`
- `IEnumerable<dotExternalReferenceModelSelection_t> GetHiddenExternalObjects()`
- `IEnumerable<string> GetExternalReferenceModelComparisonSets()`
- `IEnumerable<string> GetExternalReferenceModelComparisonTolerances()`
- `bool LaunchExternalReferenceModelComparisonSetsDialog()`
- `bool LaunchExternalReferenceModelComparisonTolerancesDialog()`
- `Dictionary<string, string> GetReferenceModelObjectAttributes(int refererenceModelObjectId)`
- `List<List<Point>> GetReferenceModelObjectFaces(Identifier referenceObjectIdentifier, int maxFaceCount)`
- `List<List<Point>> GetReferenceModelObjectFaces(Identifier referenceObjectIdentifier)`

---

## Overlay Models

- `Nullable<OverlayModel> GetOverlayModel(string path)`
- `bool ImportOverlayModelWithMetadata(string fileAndPath, string basePointGuid, Point origin, Point axisX, Point axisY, double scale, string tcModelId, string tcModelVersionId, string tCModelName)`
- `bool UpdateOverlayModel(string fileAndPath, string basePointGuid, Point origin, Point axisX, Point axisY, double scale, string tcModelVersionId)`
- `List<string> GetOverlayModelEntitiesIfcGuidsFromIds(string path, List<int> ids)`
- `List<int> GetOverlayModelEntityIdsFromIfcGuids(string path, List<string> guids)`
- `IEnumerable<string> GetOverlayModelHiddenEntityGuids(string path)`
- `IEnumerable<string> GetOverlayModelSelectedEntityGuids(string path)`
- `ValueTuple<string, string> GetOverlayModelTCIdAndVersionId(string path)`

---

## Shape Metadata

- `ShapeMetadataResult AddShapeMetadataKeyVal(string shapeGuid, string key, List<string> value)`
- `ShapeMetadataResult UpdateShapeMetadata(string shapeGuid, string key, List<string> newValue)`
- `bool DeleteShapeMetadata(string guid, string metaDataKey)`
- `List<string> GetShapeMetadataVal(string guid, string key)`
- `List<string> GetShapeAllMetadataKeys(string guid)`
- `List<Tuple<string, List<string>>> GetShapeStructuredMetadata(string guid)`
- `bool AppendShapeMetadata(string guid, List<string> keyValStrings)`

---

## Macros / Actions / Commands

- `bool dotStartCommand(string CommandName, string Parameters)` - start a named command (**synchronous** — blocks via `SyncHandler` callback until the command completes; use this for reliable execution)
- `bool dotStartAction(string ActionName, string Parameters)` - start a named action (**fire-and-forget** — returns immediately, no completion callback; avoid in workflows that depend on command completion)
- `bool dotStartCustomComponentCreation(string ComponentName)` - start custom component creation
- `bool dotStartPluginCreation(string ComponentName)` - start plugin creation
- `bool DeleteMacro(string fileName, MacroLocationEnum macroLocation)` - delete a macro file

---

## Session Log

- `bool dotWriteToSessionLog(string Message)` - write info to session log
- `bool dotWriteErrorToSessionLog(string Message)` - write error to session log
- `string dotTranslateLabel(string label)` - translate a UI label

---

## Modification Stamps / History

- `string dotGetCurrentModificationStampGuid()` - get current modification stamp
- `bool CheckObject1ModifiedAfterObject2(ModelObject object1, ModelObject object2)` - compare modification order
- `bool dotCheckObjectModifiedAfterStamp(Guid objectGuid, string ModStamp)` - check if modified after stamp
- `ModificationInfo dotGetModifications(string ModStamp, IEnumerable<ModelObjectEnum> ObjectTypes, bool returnAlsoIfObjectIsCreatedAndDeletedAfterEvent)`
- `ModelObjectEnumerator dotGetDeletedObjecs(string ModStamp, IEnumerable<ModelObjectEnum> ObjectTypes, bool returnAlsoIfObjectIsCreatedAndDeletedAfterEvent)`
- `ModelObjectEnumerator dotGetObjectsWithAnyModification(string ModStamp, IEnumerable<ModelObjectEnum> ObjectTypes)`
- `ModelObjectEnumerator dotGetModificationsByFilter(string ModStamp, string FilterName)`
- `bool dotMarkObjectModifiedForOrganizer(int objectId)`
- `string GetEventTimeGuid(string eventName)` - get event time GUID
- `bool EventsDbTimeOrdered(string OlderEventGuid, string NewerEventGuid)` - compare event order
- `bool WriteEventToDatabase(string eventName)` - write event
- `bool RenameEvent(string oldEventName, string newEventName)`

---

## Check Modified Definitions

- `bool dotCheckBoltDefinitionsModified(string ModStamp)`
- `bool dotCheckBoltAssemblyDefinitionsModified(string ModStamp)`
- `bool dotCheckMaterialDefinitionsModified(string ModStamp)`
- `bool dotCheckProfileDefinitionsModified(string ModStamp)`
- `bool dotCheckDrawingsModified(string ModStamp)`
- `bool dotCheckDrawingOptionsModified(string ModStamp)`
- `bool dotCheckModelOptionsModified(string ModStamp)`
- `bool dotCheckCustomPropertiesModified(string ModStamp)`

---

## Assembly Helpers

- `Identifier dotGetAssemblyId(Assembly ass)`
- `ModelObject dotGetMainPart(Assembly ass)` - get main part of assembly
- `ArrayList dotGetSecondaries(Assembly ass)` - get secondary parts
- `ArrayList dotGetSubAssemblies(Assembly ass)` - get sub-assemblies
- `int dotGetParentId(int identifier, ref int rootAssemblyIdentifier)` - get parent assembly ID
- `int GetObjectAssemblyPrimary(int objectId)` - get primary assembly for object
- `int dotGetFatherReferenceAssemblyId(int referenceObjectId)`

---

## Bolt Operations

- `void FlipBoltGroupDirection(BoltGroup boltGroup)` - flip bolt group direction
- `bool InsertSplitBolt(BoltGroup boltGroup)` - insert split bolt
- `Dictionary<int, Tuple<double, double>> GetBoltedPartsHoleElevations(BoltGroup boltGroup)` - get bolt hole elevations
- `bool HighlightBoltedParts(int boltID)` - highlight bolted parts

---

## Profile / Material Validation (Fabricator)

- `bool IsValidProfile(string profile)` - check if profile is valid
- `bool IsValidFabricatorProfile(string profileName)`
- `bool IsValidFabricatorMaterial(string profileName, string materialName)`
- `bool IsValidFabricatorFinish(string finishName)`
- `bool IsProfileAccepted(string profileName)` / `bool IsMaterialAccepted(string profileName, string materialName)` / `bool IsFinishAccepted(string finishName)`
- `bool AddAcceptedProfile(string profileName)` / `bool DeleteAcceptedProfile(string profileName)`
- `bool AddAcceptedMaterial(string profileName, string materialName)` / `bool DeleteAcceptedMaterial(string profileName, string materialName)`
- `bool AddAcceptedFinish(string finishName)` / `bool DeleteAcceptedFinish(string finishName)`
- `IEnumerable<string> GetAvailableMaterials(string profileName)`
- `IDictionary<string, IDictionary<string, object>> GetAvailableProfiles()`
- `bool GetNormalizedPlateProfileName(int objectId, int numOfDimensions, ref string profile)`

---

## Printing

- `IPrintingController GetPrintingController()` - get printing controller
- `int HandlePdfReport(string ReportPath)` - handle PDF report

---

## Filtering Cache

- `void EnableFilteringCache()` - enable filtering cache for performance
- `void DisableFilteringCache()` - disable filtering cache

---

## Export Attribute Files

- `string[] dotExportGetAttributeFiles(string suffix)` - get attribute files by suffix
- `string[] dotExportGetAttributeFilesWithPrefix(string prefix, string suffix)` - get by prefix and suffix
- `void dotExportSetAttributeFileSubdirectory(string suffix, string subdirectory)` - set subdirectory
- `bool dotExportClearAttributeFileCache()` - clear cache
- `bool dotTryGetSubdirectoryFromFileName(string filename, out string subdirectory)`
- `bool dotTryGetSubdirectoryFromFilePrefixAndSuffix(string prefix, string suffix, out string subdirectory)`

---

## Shadow Region Export

- `ArrayList dotExportShadowRegion(ArrayList PartIdentifiers)` - export shadow region
- `ArrayList dotExportShadowRegionComplement(ArrayList PartIdentifiers)` - export shadow region complement

---

## Connected Objects / Highlight

- `bool HighlightConnectedObjects(IEnumerable<ModelObject> ModelObjects)` - highlight connected objects
- `bool UnhighlightConnectedObjects()` - remove highlight
- `bool ObjectIncludedToCopyOrMoveOperation(Identifier modelObjectIdentifier)` - check if object in copy/move
- `List<Identifier> ObjectsIncludedToCopyOrMoveOperation()` - get all objects in copy/move

---

## Rebar

- `bool GetRebarShapeRecognitionData(Identifier rebarId, int barIndex, out RebarShapeRecognitionData data)` - get rebar shape recognition
- `bool GetSimilarNumberedObjectsCoordinateSystems(Identifier object1Id, Identifier object2Id, ref CoordinateSystem cs1, ref CoordinateSystem cs2)`
- `IList<Identifier> GetVirtualBarIdentifiersByGuid(string guid)` - get virtual bar identifiers
- `void ClearVirtualBarCache()` - clear virtual bar cache

---

## Drawings

- `bool dotCleanDrawingFiles(bool Silent, string BackupPath)` - clean drawing files
- `void dotShowPreviewDrawing(int DrawingId)` - show drawing preview
- `bool dotDisplayAutoDefaultSettings(ModelObjectEnum type, int componentNumber, string componentName)` - show auto default settings
- `bool dotDisplayComponentHelp(ModelObjectEnum type, int componentNumber, string componentName)` - show component help

---

## Live Sharing / Tekla Model Sharing

- `bool dotSharingIsEnabled()` - check if sharing is enabled
- `bool dotSharingMakeModelShareable(string xml)`
- `bool dotSharingWriteOut(DotSharingPrivilegeEnum permission, string packetFolder, DotSharingWriteOutModeEnum mode, string revisionInfo, out DotSharingErrorCodeEnum errorCode, out string errorDetail, out Dictionary<string, string> moduleBaselines, out bool isRevertPossible)`
- `bool dotSharingWriteOutCommit(bool success, bool isRevertPossible, string packetFolder, int packetNumber, out DotSharingErrorCodeEnum errorCode, out string errorDetail, out Dictionary<string, string> moduleBaselines)`
- `bool dotSharingReadIn(string packetFolder, int packetNumber, out DotSharingErrorCodeEnum errorCode, out string errorDetail, out Dictionary<string, string> moduleBaselines)`
- `bool dotSharingReadInStarting(bool joiningSharing, string currentUser)` / `bool dotSharingReadInCommit(bool success, bool joiningSharing)`
- `bool dotSharingShowReadInChanges()`
- `bool dotSharingSetMenu(DotSharingPrivilegeEnum privilege)`
- `bool dotSharingRegisterPlugin(string name, bool asynchronous)`
- `bool dotSharingGetVersionGuid(ref Guid versionGuid)` / `bool dotSharingSaveVersionGuid(Guid versionGuid, int packetNumber, Dictionary<string, string> baselines)`
- `bool dotSharingGetVersionPacket(ref int packetNumber)`
- `bool dotSharingCreateNewModel(string modelName, string modelPath)` / `bool dotSharingCreateEmptyModel(string modelName, string modelPath)`
- `bool dotSharingOpenModelForJoin(string modelFolder)` / `void dotSharingJoinFinished()`
- `bool dotSharingCreateStartSharingBackup(string backupFolder)` / `bool dotSharingRestoreStartSharingBackup(string backupFolder)`
- `bool dotSharingCheckFileToReadIn(string filePath, string packetFilePath)`
- `string dotSharingGetModelLocalHistoryFolder(int packetNumber, bool prepareForStoring)`
- `void dotSharingHistoryWriteLocal(string filePath)`
- `void dotSharingLogPrint(DotSharingLogTypeEnum type, string message)`
- `bool dotSharingIsPrivilegesInpOwner()` / `void dotSharingRestorePrivilegesInp()`
- `bool dotSharingCommandResult(int commandId, bool success, DotSharingErrorCodeEnum ErrorCode, string ErrorDetail)`
- `bool dotExcludeFromSharingAndOpen(string ModelFolder)`
- `bool dotStartSharingCommit()`
- `bool dotModelSharingLicenseInfo(string ProfileId)`
- `bool dotSetUserModelRole(Guid modelId, string modelFolder, Guid userId, DotSharingPrivilegeEnum role)`
- `bool StartLiveSharing(LiveSharingChangeSetDelegate callback, LiveSharingModelType modelType)` / `bool StopLiveSharing(LiveSharingModelType modelType)`
- `bool CreateBaseline(string trimbimFilePath, LiveSharingModelType modelType, CancellableProgressReportDelegate progressCb)`
- `byte[] GetPendingChangeSets(string eventName, LiveSharingModelType modelType)`
- `bool AddChangeSet(string changeSetName, string baseModelPath, byte[] trimbimBlob)` / `bool RemoveChangeSet(string baseModelPath)`
- `Task<Dictionary<string, object>> GetSharingLocalChangesAsync()`
- `bool dotLoadModel2FromLocalHistoryBySharingPacket(int packetNumber)`

---

## Miscellaneous

- `int dotRunDataRetrieverCommand(string parameter)` - run data retriever command
- `int dotDaoFindObject(int parentId, string type, string name, int position)` - DAO find object
- `object GetAttribute(int objectId, string attributeName)` - get attribute by object ID
- `string GetTSLanguageCultureString()` - get language culture string
- `bool SwapConstructionObjectOrder(ModelObject modelObject)` - swap construction order
- `bool CreateExportObjectList(string outPath, bool selectedOnly, dotExportSelectionInfo_t info)` - create export object list
- `IList<string> dotGetAllowedReferenceModelFileExtensions()` - get allowed reference model extensions
- `void ClearOldFabricatorSettings()` - clear old fabricator settings
- `int GetDeletedSubmittedPartsCount(string FilePath)` - get deleted submitted parts
- `LoftedPlateValidityStatus ValidateLoftedPlateBaseCurves(List<ICurve> baseCurves)` - validate lofted plate curves
- `IPropertySetDialogFeature GetPropertySetDialogFeature()` / `void SetPropertySetDialogFeature(IPropertySetDialogFeature instance)`
- `Component GetMEPIFCExport()` - get MEP IFC export component
- `bool RegisterAssertCallback(AssertEventDelegate callback)` - register assert callback
- `bool AutomatedTesting_BroadcastMessage(string testMessage)` - testing broadcast

---

## Testing Save Points

- `void SetTestSavePoint()` - set a test save point
- `void RollbackToTestSavePoint()` - rollback to test save point
- `void RollbackToTestSavePoint(bool resetSelection)` - rollback with optional selection reset

---

## Nested Enums

### SaveOperationEnum
`SAVE(0)`, `SAVEAS_WITH_PATH(1)`, `AUTOSAVE(2)`, `QUIT_PROGRAM_WITHOUT_SAVE(3)`

### UndoDirectionEnum
`UNDO(0)`, `REDO(1)`, `UNDEFINED(2)`

### UndoOperationEnum
`UNDO(1)`, `REDO(2)`, `CLEAR_UNDO_LOG(3)`, `GET_LATEST_UNDO_LOG_INFO(4)`, `GET_CURRENT_UNDO_LOG_INFO(5)`

### Model2OperationEnum
`LOAD_UNLOAD_MODEL2(0)`, `SET_UNSET_MODEL2(1)`

### MacroLocationEnum
`GLOBAL(0)`, `LOCAL(1)`, `ABSOLUTE(2)`, `INTERNAL(3)`

### ViewPartRepresentationEnum
`WIRE_FRAME(0)`, `SHADED_WIRE_FRAME(1)`, `HIDDEN_LINES(2)`, `RENDERED(3)`, `SHOW_ONLY_SELECTED(4)`

### MEPIFCExportViewTypeEnum
`UNDEFINED(0)`, `IFC4_MEP_REFERENCE_VIEW(1)`, `IFC4_MEP_DESIGN_TRANSFER_VIEW(2)`, `IFC2X3_MEP_COORDINATION_VIEW(3)`, `IFC2X3_MEP_SURFACE_GEOMETRY_VIEW(4)`

### DotSharingPrivilegeEnum
`NONE(0)`, `VIEWER(1)`, `EDITOR(2)`, `OWNER(3)`, `NOT_AUTHORIZED(4)`, `PROJECT_VIEWER(5)`

### DotSharingWriteOutModeEnum
`FIRST_FULL(0)`, `FULL(1)`, `INCREMENTAL(2)`

### DotSharingLogTypeEnum
`BASIC(0)`, `INFORMATION(1)`, `WARNING(2)`, `ERROR(3)`

### DotSharingErrorCodeEnum
`NO_ERROR(0)`, `DISK_FULL(1)`, `FILE_BUSY(2)`, `FILE_PATH_TOO_LONG(3)`, `FILE_COPY_ERROR(4)`, `FOLDER_CREATION_ERROR(5)`, `DATABASE_ERROR(6)`, `UNKNOWN_ERROR(7)`, `UNPERMITTED_WRITE_OUT_CHANGES(8)`

### SharingOperationEnum
32 values covering all sharing operations (MAKE_MODEL_SHAREABLE through GET_MODEL_LOCAL_HISTORY_FOLDER).

---

## Command Reference (777 Commands)

### Execution method depends on the command prefix

| Prefix | Method | Why |
|--------|--------|-----|
| `ail_` / `ail[A-Z]` | `dotStartCommand` | Interactive picking commands. `dotStartCommand` is synchronous (blocks via `SyncHandler` callback until the command completes). Call directly on any thread. |
| `acmd` / `acmd_` | `MacroBuilder.Callback` + `Run()` | UI/dialog commands. Must go through the AKIT macro pipeline. `dotStartCommand("acmd_...")` bypasses the macro pipeline and can crash Tekla. |
| `dia` / `gdr_` / `gr` | `MacroBuilder.Callback` + `Run()` | Same as `acmd` — dialog/graphics commands routed through AKIT. |

### ail_ commands — use dotStartCommand directly

```csharp
// ail_ commands are interactive (picking, creation). dotStartCommand blocks until done.
Tekla.Structures.ModelInternal.Operation.dotStartCommand("ailDMSetWorkplane", "");
Tekla.Structures.ModelInternal.Operation.dotStartCommand("ail_create_bolt", "");
```

### acmd_ commands — use MacroBuilder.Callback on a BackgroundWorker

```csharp
// acmd_ commands must go through the AKIT macro pipeline.
// MacroBuilder.Callback() generates akit.Callback() script code.
// Run() writes a .cs macro file and calls Operation.RunMacro().
// BackgroundWorker keeps the UI thread free while the macro executes.
// WARNING: Task.Run crashes Tekla — use BackgroundWorker instead.
// IMPORTANT: TeklaStructures.Connect() MUST be called before MacroBuilder.Run() or macros silently fail.
using System.ComponentModel;
using Tekla.Structures;

if (TeklaStructures.Connect() is false)
{
    // Handle connection failure — MacroBuilder.Run() will not work without a connection
}

var macroBuilder = new MacroBuilder();
macroBuilder.Callback("acmd_fit_workarea_by_parts", "", "main_frame");
macroBuilder.Callback("acmd_change_workfloor", "", "main_frame");

var worker = new BackgroundWorker();
worker.DoWork += (sender, e) => { macroBuilder.Run(); };
worker.RunWorkerAsync();
```

### Mixed sequences — ail_ then acmd_

```csharp
// 1. Run interactive command synchronously (blocks until user finishes picking)
Tekla.Structures.ModelInternal.Operation.dotStartCommand("ailDMSetWorkplane", "");

// 2. Connect before using MacroBuilder (required or macros silently fail)
if (TeklaStructures.Connect() is false)
{
    // Handle connection failure
}

// 3. Run acmd_ commands via macro pipeline on background thread
var macroBuilder = new MacroBuilder();
macroBuilder.Callback("acmd_fit_workarea_by_parts", "", "main_frame");
macroBuilder.Callback("acmd_change_workfloor", "", "main_frame");

var worker = new BackgroundWorker();
worker.DoWork += (sender, e) => { macroBuilder.Run(); };
worker.RunWorkerAsync();
```

**Context:** `[M]` = Modeling mode only, `[D]` = Drawing mode only, `[?]` = Any/Unknown

### Analysis

- `acmdDisplayAnalysisModelsDialog` [M] — Analysis And Models
- `acmdAnalysisAppliedPropertiesconcrete_beam` [M] — Concrete Beam Properties
- `acmdAnalysisAppliedPropertiesconcrete_column` [M] — Concrete Column Properties
- `acmdAnalysisAppliedPropertiespad_footing` [M] — Concrete Pad Footing Properties
- `acmdAnalysisAppliedPropertiesconcrete_panel` [M] — Concrete Panel Properties
- `acmdAnalysisAppliedPropertiesconcrete_slab` [M] — Concrete Slab Properties
- `acmdAnalysisAppliedPropertiesstrip_footing` [M] — Concrete Strip Footing Properties
- `ail_create_analysis_node` [M] — Create Node
- `ail_create_analysis_nodelink` [M] — Create Rigid Link
- `ail_merge_nodes_to_node` [M] — Merge Selected Nodes
- `acmdAnalysisPartProperties` [M] — Part Properties
- `acmdAnalysisRecreatePartsreset` [M] — Reset Editing Of Selected Parts
- `acmdAnalysisAppliedPropertiesbeam` [M] — Steel Beam Properties
- `acmdAnalysisAppliedPropertiescolumn` [M] — Steel Column Properties
- `acmdAnalysisAppliedPropertiescontourplate` [M] — Steel Contour Plate Properties
- `acmdAnalysisAppliedPropertiesbeamortho` [M] — Steel Orthogonal Beam Properties
- `acmdAnalysisAppliedPropertiestwinprofile` [M] — Steel Twin Profile Properties
- `acmdAnalysisToggleShowMemberNumbers` [M] — Toggle Show Member Numbers (toggle)
- `acmdAnalysisToggleShowNodeNumbers` [M] — Toggle Show Node Numbers (toggle)

### Annotations

- `ail_create_noteGR_NOTE_ALONG_LINE` [D] — Add Associative Note > Along Line
- `ail_create_noteGR_NOTE_APPLIED` [D] — Add Associative Note > With Applied
- `ail_create_noteGR_NOTE_WITH_LEADER` [D] — Add Associative Note > With Leader Line
- `ail_create_noteGR_NOTE_WITHOUT_LEADER` [D] — Add Associative Note > Without Leader Line
- `ail_create_drawing_detail_symbolSymbol` [D] — Add Detail Mark
- `acmdDisplayAnyLinkPropertiesAndStartCommandlink` [D] — Add Hyperlink
- `ail_create_levelGR_LEVEL` [D] — Add Level Mark
- `acmdDisplayAnyLinkPropertiesAndStartCommandlink_drawing` [D] — Add Link To Other Drawing
- `acmd_create_marks_all` [D] — Add Part Marks For All
- `acmd_create_marks_selected` [D] — Add Part Marks For Selected
- `acmdDisplayAnyRevisionMarkAndStartCommandGR_REVISION0N` [D] — Add Revision Mark
- `acmdDisplayAnyRevisionMarkAndStartCommandGR_REVISION2` [D] — Add Revision Mark > Along Line With Left Arrow
- `acmdDisplayAnyRevisionMarkAndStartCommandGR_REVISION2R` [D] — Add Revision Mark > Along Line With Right Arrow
- `acmdDisplayAnyRevisionMarkAndStartCommandGR_REVISION0` [D] — Add Revision Mark > With Left Arrow
- `acmdDisplayAnyRevisionMarkAndStartCommandGR_REVISION0R` [D] — Add Revision Mark > With Right Arrow
- `ail_create_cut_arrows` [D] — Add Section Mark
- `ail_create_symbolGR_SYMBOL0` [D] — Add Symbol
- `ail_create_symbolGR_SYMBOL2` [D] — Add Symbol > Along Line
- `ail_create_symbolGR_SYMBOL1` [D] — Add Symbol > With Leader Line
- `ail_create_textGR_TEXT0` [D] — Add Text
- `ail_create_textGR_TEXT2` [D] — Add Text > Along Line
- `ail_create_textGR_TEXT3` [D] — Add Text > Along Line And Arrow At End
- `ail_create_textGR_TEXT32` [D] — Add Text > Along Line And Arrow At Start
- `ail_create_textGR_TEXT1` [D] — Add Text > With Leader Line
- `acmdDisplayAnyLinkPropertiesAndStartCommandasc` [D] — Add Text From File
- `ail_create_wld` [D] — Add Weld Mark
- `acmd_display_attr_dialognote_dial` [D] — Associative Note Properties
- `acmd_display_attr_dialogsmark_dial` [D] — Bolt Mark Properties
- `acmd_display_attr_dialogjmark_dial` [D] — Connection Mark Properties
- `acmd_display_attr_dialogdetail_dial` [D] — Detail Mark Properties
- `acmd_display_attr_dialoggo_arc_dial` [D] — Drawing Arc Properties
- `acmd_display_attr_dialogscrew_dial` [D] — Drawing Bolt Properties
- `acmd_display_attr_dialoggo_circle_dial` [D] — Drawing Circle Properties
- `acmd_display_attr_dialogfo_dwg_dial` [D] — Drawing DXFProperties
- `acmd_display_attr_dialoggrid_line_dial` [D] — Drawing Grid Line Properties
- `acmd_display_attr_dialoggrid_dial` [D] — Drawing Grid Properties
- `acmd_display_attr_dialogfo_link_dial` [D] — Drawing Hyperlink Properties
- `acmd_display_attr_dialogfo_image_dial` [D] — Drawing Image Properties
- `acmd_display_attr_dialoggo_line_dial` [D] — Drawing Line Properties
- `acmd_display_attr_dialoglink_drawing_dial` [D] — Drawing Link Properties
- `acmd_display_part_attrpart_dial` [D] — Drawing Part Properties
- `acmd_display_attr_dialoggo_polygon_dial` [D] — Drawing Polygon Properties
- `acmd_display_attr_dialoggo_polyline_dial` [D] — Drawing Polyline Properties
- `acmd_display_attr_dialogpour_break_dial` [D] — Drawing Pour Break Properties
- `acmd_display_attr_dialogpour_dial` [D] — Drawing Pour Object Properties
- `acmd_display_attr_dialoggo_rectangle_dial` [D] — Drawing Rectangle Properties
- `acmd_display_attr_dialogrefobj_dial` [D] — Drawing Reference Object Properties
- `acmd_display_rebar_attrrebar_mesh_dial` [D] — Drawing Reinforcement Mesh Properties
- `acmd_display_rebar_attrrebar_dial` [D] — Drawing Reinforcement Properties
- `acmd_display_attr_dialogsurfacing_dial` [D] — Drawing Surface Treatment Properties
- `acmd_display_attr_dialogfo_asc_dial` [D] — Drawing Text File Properties
- `acmd_display_attr_dialogweldobject_dial` [D] — Drawing Weld Properties
- `acmd_display_attr_dialoglevel_dial` [D] — Level Mark Properties
- `acmd_display_attr_dialogmrm_dial` [D] — Merged Reinforcement Mark Properties
- `acmd_display_attr_dialogpmark_dial` [D] — Part Mark Properties
- `acmd_display_attr_dialogpour_mark_dial` [D] — Pour Mark Properties
- `acmd_display_attr_dialogrebar_mark_dial` [D] — Reinforcement Mark Properties
- `ailRemoveAssociativeNoteChangeSymbol` [D] — Remove Associative Note Change Symbol
- `acmdRemoveAllAssociativeNoteChangeSymbols` [D] — Remove Associative Note Change Symbols
- `acmdRemoveAllChangeSymbols` [D] — Remove Change Symbols
- `ailRemoveMarkChangeSymbol` [D] — Remove Mark Change Symbol
- `acmdRemoveAllMarkChangeSymbols` [D] — Remove Mark Change Symbols
- `acmd_display_attr_dialogrevision_dial` [D] — Revision Mark Properties
- `acmd_display_attr_dialogcsym_dial` [D] — Section Mark Properties
- `acmd_display_attr_dialogsurf_mark_dial` [D] — Surface Treatment Mark Properties
- `acmd_display_attr_dialogsymbol_dial` [D] — Symbol Properties
- `acmd_display_attr_dialogtext_dial` [D] — Text Properties
- `acmd_update_marks_all` [D] — Update All Part Marks
- `acmd_update_drawing_weldings` [D] — Update All Weld Marks
- `acmd_update_marks_selected` [D] — Update Selected Part Marks
- `acmd_display_attr_dialogwld_dial` [D] — Weld Mark Properties
- `acmd_display_attr_dialogweld_dial` [D] — Welding Mark Properties

### Assembly

- `ailAddToAssembly` [M] — Add As Sub Assembly
- `acmd_display_selected_assembly` [M] — Display Selected Assembly
- `ailExplodeAssembly` [M] — Explode
- `ailExplodeSubassembly` [M] — Explode Sub Assembly
- `ailMakeIntoAssembly` [M] — Make Into Assembly
- `ailRemoveFromAssembly` [M] — Remove From Assembly
- `ailChangeMainObject` [M] — Set As New Main Object Of Assembly

### AutoConnection

- `acmdDisplayAutoDefaultsSetupDialog` [M] — Auto Defaults Settings
- `acmdDisplayAutoConnectionDialog` [M] — Create
- `acmdDisplayAutoConnectionSetupDialog` [M] — Settings

### Bolts

- `ail_create_bolt` [M] — Create Bolts
- `ail_create_DM_bolts` [M] — Create Bolts With DM
- `ail_modify_screw_parts` [M] — Edit Bolted Parts
- `acmd_display_screw_attr_dialog` [M] — Properties

### CastUnit

- `ailAddToCastUnit` [M] — Add To Cast Unit
- `ailCreateCastUnit` [M] — Create
- `ailExplodeCastUnit` [M] — Explode
- `ailRemoveFromCastUnit` [M] — Remove From Cast Unit
- `ail_set_top_in_form_face` [M] — Set Top In Form Face
- `ail_view_top_in_form_face` [M] — Show Top In Form Face

### Catalogs

- `acmd_display_ass_dialogedit` [M] — Bolt Assembly Catalog
- `acmd_display_screwdb_dialogedit` [M] — Bolt Catalog
- `acmd_export_screw_database` [M] — Export Bolt Catalog
- `acmd_import_screw_database` [M] — Import Bolt Catalog
- `acmd_display_mat_dialogdiaModifyMaterialDialog` [M] — Material Catalog
- `acmd_display_prof_dialogdiaModifyProfileDialog` [M] — Profile Catalog
- `ail_create_pluginRebarSetShapeCatalog` [M] — Rebar Set Shape Catalog

### Chamfer

- `ail_chamfer_polygon_vertex` [M] — Create For Part Corner
- `ail_create_common_objectco_chamfer` [M] — Create For Part Edge
- `acmd_display_chamfer_attr_dialog` [M] — Part Corner Chamfer Properties
- `acmdDisplayChamferEdgeAttrDialog` [M] — Part Edge Chamfer Properties

### Common

- `acmd_autosave_model` [M] — Autosave
- `acmdDisplayLanguageSelectionDialog` [M] — Change Language
- `grDisplayChangeLicenseServerDialog` [M] — Change License Server
- `acmd_display_plotdb_dialogedit` [M] — Choose Printer
- `acmd_delete_selected_objects` [M] — Delete
- `acmd_delete_selected_dr` [D] — Delete_Drawing
- `acmd_exit_program` [M] — Exit
- `acmdFinalizeActiveCommand` [M] — Finalize Active Command
- `acmdFinishPolygonInput` [M] — Finish Polygon Input
- `acmd_interrupt` [M] — Interrupt
- `acmdIncreaseSnapSkipCount` [M] — Next Position
- `acmdKeyInCB$` [M] — Numeric Location > Absolute
- `acmdKeyInCB!` [M] — Numeric Location > Global
- `acmdKeyInCB@` [M] — Numeric Location > Relative
- `acmdOpenCurrentWorkingFolder` [M] — Open Working Directory
- `acmdDecreaseSnapSkipCount` [M] — Previous Position
- `acmd_display_plot_dialog` [M] — Print Drawings
- `acmd_display_print_report_dialog` [M] — Print Reports
- `acmd_display_project_dialog` [M] — Project Properties
- `acmd_display_selected_object_dialog` [M] — Properties
- `acmd_display_selected_drawing_object_dialog` [D] — Properties_Drawing
- `acmdRepeatLastCommand` [M] — Repeat Last Command
- `acmdSaveAndBackup` [M] — Save And Backup
- `acmdDisplaySaveModelAsDialog` [M] — Save As
- `acmdDisplaySaveAsModelTemplateDialog` [M] — Save As Model Template
- `acmd_save_database_edit_drawing` [D] — Save_Drawing
- `acmd_undo` [M] — Undo
- `acmdUndoLastPolygonPick` [M] — Undo Last Polygon Pick

### Components

- `diaDisplayToolsAndComponentsCB` [M] — Applications And Components
- `ailChangeJointTypeToDETAIL` [M] — Convert To Detailing Component
- `ail_create_joint` [M] — Create Current Connection
- `acmdDisplayCustomObjectCreateDialog` [M] — Define Custom Component
- `ail_edit_library_connection` [M] — Edit Custom Component
- `ail_edit_custom_component_dialog_box` [M] — Edit Custom Component Dialog Box
- `ail_explode_joint` [M] — Explode

### Detailing

- `acmd_display_joint_dialog` [M] — Connection Properties
- `ail_create_cut_with_part` [M] — Cut Part > With Another Part
- `ail_create_plane_cut` [M] — Cut Part > With Line
- `ail_create_cut_with_polygon` [M] — Cut Part > With Polygon
- `ail_create_fitting` [M] — Fit Part End
- `ail_modify_polygon_shape` [M] — Modify Polygon Shape

### DiagnoseAndRepair

- `acmd_check_databaseXS_LIB_CHECK` [M] — Diagnose Library Database
- `acmd_check_database0` [M] — Diagnose Model
- `acmdFindDistantObjects` [M] — Find Distant Objects
- `acmdDiagnoseAttributeDefinitions` [M] — Fix Attribute Definitions
- `acmdFixIdGaps` [M] — Fix Id Gaps
- `acmdRemoveUnnecessaryDrawingFiles` [M] — Remove Unnecessary Drawing Files
- `acmd_check_databaseXS_LIB_CORRECT` [M] — Repair Library Database
- `acmd_check_database1` [M] — Repair Model

### Dimensions

- `ail_create_rebar_dimension_mark` [D] — Add Rebar Dimension Mark
- `acmd_combine_dims` [D] — Combine Dimension Lines
- `ail_create_angle_dimension` [D] — Create Angular
- `ail_create_drawing_pluginCOGDimensioning` [D] — Create Center Of Gravity
- `ail_create_curved_dimensioncurved` [D] — Create Curved > With Orthogonal Reference Lines
- `ail_create_curved_dimensioncurved_radial` [D] — Create Curved > With Radial Reference Lines
- `ail_create_dimensionfree` [D] — Create Free
- `ail_create_grid_dimensions` [D] — Create General Arrangement Drawing Dimensions > Along Grid
- `acmdGAPartToGridDimensioningX` [D] — Create General Arrangement Drawing Dimensions > Along X
- `acmdGAPartToGridDimensioningXY` [D] — Create General Arrangement Drawing Dimensions > Along XY
- `acmdGAPartToGridDimensioningY` [D] — Create General Arrangement Drawing Dimensions > Along Y
- `ail_create_dimensionx` [D] — Create Horizontal
- `ail_create_dimensionxy` [D] — Create Orthogonal
- `ail_create_ortho_dimensionortho` [D] — Create Parallel
- `ail_create_ortho_dimensionparallel` [D] — Create Perpendicular
- `ail_add_dim_point` [D] — Create Point
- `ail_create_radius_dimension` [D] — Create Radial
- `ail_create_dimensiony` [D] — Create Vertical
- `ailFlipOutsideDimension` [D] — Flip Outside Dimension
- `acmdLinkDimensionLines` [D] — Link Dimension Lines
- `acmd_display_attr_dialogdim_dial` [D] — Properties
- `acmd_display_attr_dialogrebar_dim_dial` [D] — Rebar Dimension Mark Properties
- `acmdRedimensionDrawing` [D] — Recreate Dimensions
- `ailRemoveDimPointCloud` [D] — Remove Dimension Change Symbol
- `acmdRemoveAllDimPointClouds` [D] — Remove Dimension Change Symbols
- `ail_remove_dim_point` [D] — Remove Point
- `acmd_grDimensionSelectNextAssociation` [D] — Select Next Associativity Option
- `ail_set_dim_zero_point` [D] — Set Start Point
- `acmdUnlinkDimensionLines` [D] — Unlink Dimension Lines

### Drawing

- `acmd_display_attr_dialogadraw_dial` [M] — Assembly Drawing Properties
- `acmd_create_drawings_auto` [M] — Auto Drawing
- `acmd_display_attr_dialogcudraw_dial` [M] — Cast Unit Drawing Properties
- `ail_close_drawing_editor` [D] — Close Drawing Mode
- `acmdCloseLayoutEditor` [D] — Close Layout Editor
- `acmd_create_dim_assembly_drawings` [M] — Create Assembly Drawing
- `acmdCreateCastUnitDrawing` [M] — Create Cast Unit Drawing
- `acmdMasterDrawingCatalog` [M] — Create Drawing
- `acmd_create_multi_assembly_drawing` [M] — Create Empty Multi Drawing
- `acmdCreateFabricationDrawingWithDialog` [M] — Create Fabrication Drawing
- `acmdCreateFabricationDrawingInServiceWithDialog` [M] — Create Fabrication Drawing In Service
- `acmd_create_dim_general_assembly_drawing` [M] — Create General Arrangement Drawing
- `ail_create_drawing_pluginAdvancedGridLabels` [D] — Create Grid Labels
- `acmd_create_multi_assembly_drawingby_assemblies` [M] — Create Multi Drawing > From Assembly Drawings > From Selected Parts
- `acmd_create_multi_assembly_drawingby_assemblies_use_layout` [M] — Create Multi Drawing > From Assembly Drawings > From Selected Parts > With Original Layout
- `acmd_create_multi_assembly_drawingby_drawings` [M] — Create Multi Drawing > From Selected Drawings
- `acmd_create_multi_assembly_drawingby_drawings_use_layout` [M] — Create Multi Drawing > From Selected Drawings > With Original Layout
- `acmd_create_multi_assembly_drawingby_parts` [M] — Create Multi Drawing > From Single Part Drawings > From Selected Part
- `acmd_create_multi_assembly_drawingby_parts_use_layout` [M] — Create Multi Drawing > From Single Part Drawings > From Selected Part > With Original Layout
- `acmd_create_dim_part_drawings` [M] — Create Single Part Drawing
- `acmd_display_layout_dialog` [M] — Drawing Layout
- `gdr_menu_select_active_draw` [M] — Drawing List
- `acmd_display_attr_dialoggdraw_dial` [M] — General Arrangement Drawing Properties
- `acmdOpenLayoutEditor` [D] — Layout Editor
- `acmdLayoutEditorAddTables` [D] — Layout Editor Add Tables
- `acmdLayoutEditorAddLayout` [D] — Layout Editor Create Layout
- `acmdLayoutEditorSave` [D] — Layout Editor Save
- `acmdLayoutEditorSaveAs` [D] — Layout Editor Save As
- `acmd_display_attr_dialogmdraw_dial` [M] — Multi Drawing Properties
- `grOpenNextDrawingCB` [D] — Next
- `acmdFabricationDrawingCollectionsDialog` [M] — Open Fabrication Drawing Collections
- `acmd_shellexecute_open%XSBIN%\Applications\Tekla\Model\PatternLineEditor\PatternLineEditor.exe` [M] — Pattern Line Editor
- `grOpenPreviousDrawingCB` [D] — Previous
- `acmd_display_dr_attr` [D] — Properties
- `acmd_display_attr_dialogdrefobj_dial` [D] — Reference Model List
- `acmdRestoreLayout` [D] — Restore Layout
- `acmd_display_attr_dialogwdraw_dial` [M] — Single Part Drawing Properties
- `acmd_edit_symbols` [M] — Symbol Editor
- `acmd_edit_templates` [M] — Template Editor
- `acmdCreateFabricationDrawingDialogToggle` [M] — Toggle Fabrication Drawing Dialog (toggle)

### Edit

- `ail_CustomizeRebarLocationWithPick` [D] — Adjust Reinforcing Bars
- `acmdRunAlignmentMacroAlignBottom` [D] — Align Bottom
- `acmdRunAlignmentMacroAlignCenter` [D] — Align Center
- `acmdRunAlignmentMacroAlignLeft` [D] — Align Left
- `acmdRunAlignmentMacroAlignMiddle` [D] — Align Middle
- `acmdRunAlignmentMacroAlignRight` [D] — Align Right
- `acmdRunAlignmentMacroAlignTop` [D] — Align Top
- `acmdRunAlignmentMacroEqualHorizontalDistance` [D] — Arrange Horizontally At Equal Distances
- `acmdRunAlignmentMacroEqualVerticalDistance` [D] — Arrange Vertically At Equal Distances
- `ail_grPartialCloning` [D] — Clone Area
- `acmd_grPartialCloningSettings` [D] — Clone Area > Settings
- `ail_grPartialCloningWithPreviousSelection` [D] — Clone Area Previous
- `ail_combine_member` [M] — Combine
- `ail_copy_without_dialog` [M] — Copy
- `ail_grCopyObjects` [D] — Copy Linear
- `ail_gr_copy_objects_many_times` [D] — Copy Many Times
- `ail_grCopyMirror` [D] — Copy Mirror
- `ail_grCopyRotate` [D] — Copy Rotate
- `ail_copy_all_content_to_another_object_editmenu` [M] — Copy Special > All Content To Another Object
- `acmd_copy_from_model` [M] — Copy Special > From Another Model
- `ail_copy_translate` [M] — Copy Special > Linear
- `ail_copy_mirror` [M] — Copy Special > Mirror
- `ail_copy_rotate` [M] — Copy Special > Rotate
- `ail_copy_to_object` [M] — Copy Special > To Another Object
- `ail_copy_p2p` [M] — Copy Special > To Another Plane
- `acmdRunTSDrawingToolsMacroOffset` [D] — Copy With Offset
- `acmdRunTSDrawingToolsMacroCutLine` [D] — Create Cut Lines
- `acmdRunTSDrawingToolsMacroFillet` [D] — Create Fillet
- `ailConvertPartToItem` [M] — Create Item From Native
- `acmdRunTSDrawingToolsMacroChamferArc` [D] — Create Round Chamfer
- `ailCreateShapeFromNative` [M] — Create Shape From Native
- `acmdRunTSDrawingToolsMacroChamferLine` [D] — Create Straight Chamfer
- `ailDisplayDetailing` [M] — Display Detailing
- `ail_grCommandDivide` [D] — Divide
- `ail_grMarkAlign` [D] — Mark Align To Point
- `acmdRunMarkPlacingToolSolveCrossingLeaderlines` [D] — Mark Solve Crossing Leaderlines
- `acmdRunMarkPlacingToolSolveMarksWithStacking` [D] — Mark Solve Crossing With Stacking
- `ail_move_without_dialog` [M] — Move
- `ail_gr_move_objects` [D] — Move Linear
- `ail_grMoveMirror` [D] — Move Mirror
- `ail_grMoveRotate` [D] — Move Rotate
- `ail_move_translate` [M] — Move Special > Linear
- `ail_move_mirror` [M] — Move Special > Mirror
- `ail_move_rotate` [M] — Move Special > Rotate
- `ail_move_to_object` [M] — Move Special > To Another Object
- `ail_move_p2p` [M] — Move Special > To Another Plane
- `acmd_redo` [M] — Redo
- `acmdSelectAll` [M] — Select All
- `acmdSelectPrevious` [M] — Select Previous
- `diaDisplayObjectGroupDialogCBdiaSelectObjectGroup` [M] — Selection Filter
- `ailDMSplit` [M] — Split
- `ail_grCommandSplit` [D] — Split_Drawing
- `ail_grCommandTrim` [D] — Trim

### Export

- `ail_create_macro440000003` [M] — 3D_DGN
- `ail_create_macro440000004` [M] — 3D_DWG_DXF
- `acmdDisplay3DPDFDialog` [M] — 3D_PDF
- `acmd_create_ascii_output` [M] — ASCII
- `ail_create_pluginBVBSExport` [M] — BVBS
- `ail_create_macroS7` [M] — CAD
- `acmd_display_nc_dialog` [M] — Create NCFiles
- `acmdOpenCreateRoundTubeNC` [M] — Create Tube NCFiles
- `acmdDisplayExportDrawingsDialog` [M] — Drawings
- `ail_create_pluginTekla PowerFab Plugin` [M] — EPM
- `ail_create_macro170000068` [M] — Eli Plan
- `ail_create_macroS4` [M] — FEM
- `acmdDisplayHmsExportDialog%XSBIN%\Applications\Tekla\Model\HMSExport\HMSExport` [M] — HMS
- `ail_create_pluginExportIFC` [M] — IFC
- `acmd_display_mis_dialog` [M] — MIS
- `acmdExportSketchUpCB` [M] — Sketch Up
- `acmdExportToTeklaStructuralDesignerCB` [M] — Tekla Structural Designer
- `acmdExportToTeklaStructuralDesignerWithADModelCB` [M] — Tekla Structural Designer With AD
- `ail_create_macro160000079` [M] — Unitechnik

### Help

- `acmd_helpmailto_support` [?] — Contact Support
- `acmd_helpcontents` [?] — Contents
- `acmd_helpvideos` [?] — Videos

### Import

- `acmd_read_ascii_output` [M] — ASCII
- `ail_create_macro10000015` [M] — Attributes
- `acmd_open_planS8` [M] — CAD
- `acmd_display_import_dwg_dialog` [M] — DWG_DXF
- `ail_create_macro170000069` [M] — Eli Plan
- `acmd_open_planS5` [M] — FEM
- `diaImportModelCB` [M] — Model
- `acmd_open_plan` [M] — Other
- `acmdInsertPDFReferenceModelCB` [M] — PDFReference Model
- `acmdImportFromTeklaStructuralDesignerCB` [M] — Tekla Structural Designer

### Inquiry

- `ailInquireAssembly` [M] — Assembly Objects
- `acmd_inquire_center_of_gravity` [M] — Center Of Gravity
- `ail_show_joint_content` [M] — Component Objects
- `ailInquireRebarSetLayerData` [M] — Layer Data
- `acmd_display_model_size_dialog` [M] — Model Size
- `ail_check_edit_mode_and_inquire_object` [M] — Object
- `acmd_display_inquire_systems_dialog` [M] — Phases
- `ail_inquire_welding_by_picking_partTO` [M] — Primary Welded Part
- `ail_inquire_welding_by_picking_partALL` [M] — Welded Parts

### Loads

- `acmdDisplayAreaLoadDialog` [M] — Area Load Properties
- `ail_create_common_objectloadm_areaload` [M] — Create Area Load
- `ail_create_common_objectloadm_lineload` [M] — Create Line Load
- `ail_create_common_objectloadm_pointload` [M] — Create Point Load
- `ail_create_common_objectloadm_temperatureload` [M] — Create Temperature Load
- `ail_create_common_objectloadm_uniformload` [M] — Create Uniform Load
- `ail_create_macro10000028` [M] — Create Wind Load
- `acmdDisplayLineLoadDialog` [M] — Line Load Properties
- `acmdDisplayLoadGroupDialog` [M] — Load Groups
- `acmdDisplayPointLoadDialog` [M] — Point Load Properties
- `acmdDisplayTemperatureLoadDialog` [M] — Temperature Load Properties
- `acmdDisplayDistributedLoadDialog` [M] — Uniform Load Properties
- `acmdDisplayWindLoadDialog` [M] — Wind Load Properties

### Logs

- `acmd_display_analysis_log_fileAnalysis.log` [M] — Analysis History
- `acmdDisplayClashCheckHistory` [M] — Clash Check History
- `acmd_display_file_scrolled_to_enddrawing_history.log` [M] — Drawing History
- `acmd_display_file_scrolled_to_endnumberinghistory.txt` [M] — Numbering History
- `acmd_display_file_scrolled_to_endsave_history.log` [M] — Saving History
- `acmd_display_file_scrolled_to_endXsteel.log` [M] — Session History
- `acmdDisplayWithExternalViewerDisplayWithExternalViewer` [M] — Toggle Associated Viewer (toggle)
- `acmd_checkoption_callbackDisplayWithExternalViewer` [D] — Toggle Associated Viewer_Drawing (toggle)

### Material

- `ail_attach_to_part` [M] — Attach To Part
- `ail_detach_from_part` [M] — Detach From Part
- `ail_explode_part` [M] — Explode Part

### Measure

- `ail_create_angle_measure` [M] — Angle
- `ail_create_arc_measure` [M] — Arc
- `ail_create_screw_measure` [M] — Bolt Spacing
- `ail_create_measureb` [M] — Distance
- `ail_create_measurex` [M] — Horizontal Distance
- `ail_create_measurey` [M] — Vertical Distance

### Misc

- `ailAddSelectedObjectToPour` [M] — Add Selected To Pour
- `ailAddToPour` [M] — Add To Pour
- `ailDemoMode` [?] — Demo Mode
- `acmd_dummy_callback` [D] — Dummy_Drawing (toggle)
- `acmdKeyInCB0` [M] — Input > Key In CB0
- `acmdKeyInCB1` [M] — Input > Key In CB1
- `acmdKeyInCB2` [M] — Input > Key In CB2
- `acmdKeyInCB3` [M] — Input > Key In CB3
- `acmdKeyInCB4` [M] — Input > Key In CB4
- `acmdKeyInCB5` [M] — Input > Key In CB5
- `acmdKeyInCB6` [M] — Input > Key In CB6
- `acmdKeyInCB7` [M] — Input > Key In CB7
- `acmdKeyInCB8` [M] — Input > Key In CB8
- `acmdKeyInCB9` [M] — Input > Key In CB9
- `acmdKeyInCB*` [D] — Input > Key In CBAsterisk
- `acmdKeyInCBN0` [M] — Input > Key In CBN0
- `acmdKeyInCBN1` [M] — Input > Key In CBN1
- `acmdKeyInCBN2` [M] — Input > Key In CBN2
- `acmdKeyInCBN3` [M] — Input > Key In CBN3
- `acmdKeyInCBN4` [M] — Input > Key In CBN4
- `acmdKeyInCBN5` [M] — Input > Key In CBN5
- `acmdKeyInCBN6` [M] — Input > Key In CBN6
- `acmdKeyInCBN7` [M] — Input > Key In CBN7
- `acmdKeyInCBN8` [M] — Input > Key In CBN8
- `acmdKeyInCBN9` [M] — Input > Key In CBN9
- `acmdKeyInCB-` [M] — Input > acmd Key In CBDash
- `acmdKeyInCB.` [M] — Input > acmd Key In CBDot
- `ailRemoveFromPour` [M] — Remove From Pour
- `ailRemoveSelectedObjectFromPour` [M] — Remove Selected From Pour
- `diaToggleSnapValueCBdepth_position_3D` [M] — Snapping Switches > Snap To Auto3D
- `diaToggleSnapValueCBdepth_position_plane` [M] — Snapping Switches > Snap To Auto Plane
- `acmdResetPamuIncrementalTable` [M] — reset PAMU
- `acmdResetPamuIncrementalTableExceptManual` [M] — reset PAMUExcept Manual
- `acmdResetPamuIncrementalTableOnlyManual` [M] — reset PAMUOnly Manual

### Modeling

- `ail_create_DM_construction_arc` [M] — Add Construction Arc
- `ail_create_DM_construction_circle` [M] — Add Construction Circle
- `ail_create_construction_line` [M] — Add Construction Line
- `ail_create_DM_construction_object_with_offset` [M] — Add Construction Object With Offset
- `ail_create_user_planeco_user_plane` [M] — Add Construction Plane
- `ail_create_DM_construction_polycurve` [M] — Add Construction Polycurve
- `ail_create_common_objectco_distance` [M] — Add Fixed Distance
- `ail_create_grid_line` [M] — Add Grid Line
- `ail_create_common_objectco_inquire_distance` [M] — Add Reference Distance
- `acmdDisplayConcreteBeamDialog` [M] — Concrete Beam Properties
- `acmd_display_column_dialogconcrete_column` [M] — Concrete Column Properties
- `acmdDisplayConcreteItemDialog` [M] — Concrete Item Properties
- `acmd_display_column_dialogpad_footing` [M] — Concrete Pad Footing Properties
- `acmdDisplayConcretePanelDialog` [M] — Concrete Panel Properties
- `acmdDisplayConcreteSlabDialog` [M] — Concrete Slab Properties
- `acmdDisplayStripFootingDialog` [M] — Concrete Strip Footing Properties
- `ail_create_beamconcrete_beam` [M] — Create Concrete Beam
- `ail_create_columnconcrete_column` [M] — Create Concrete Column
- `ail_create_part_by_two_pointsconcrete_item` [M] — Create Concrete Item
- `ail_create_pad_footingpad_footing` [M] — Create Concrete Pad Footing
- `ail_create_concrete_panelconcrete_panel` [M] — Create Concrete Panel
- `ail_create_polygon_profileconcrete_beam` [M] — Create Concrete Polybeam
- `ail_create_concrete_slabconcrete_slab` [M] — Create Concrete Slab
- `ail_create_DM_helixconcrete` [M] — Create Concrete Spiral Beam
- `ail_create_concrete_strip_footingstrip_footing` [M] — Create Concrete Strip Footing
- `ail_create_DM_bent_plateconical` [M] — Create Conical Bent Plate
- `ail_create_DM_bent_platecylindrical` [M] — Create Cylindrical Bent Plate
- `ail_create_pluginFloorLayout` [M] — Create Floor Layout
- `ail_create_DM_lofted_platesteel` [M] — Create Lofted Plate
- `ail_create_DM_lofted_plateconcrete` [M] — Create Lofted Slab
- `ail_create_DM_pour_breakby_n_points` [M] — Create Pour Break > Using Multiple Points
- `ail_create_DM_pour_breakby_one_point` [M] — Create Pour Break > Using One Point
- `ail_create_DM_pour_breakby_two_points` [M] — Create Pour Break > Using Two Points
- `ail_create_DM_radial_grid` [M] — Create Radial Grid
- `ail_create_DM_rectangular_grid` [M] — Create Rectangular Grid
- `ail_create_DM_standalone_bend` [M] — Create Standalone Bend
- `ail_create_beambeam` [M] — Create Steel Beam
- `ail_create_columncolumn` [M] — Create Steel Column
- `ail_create_contour_plate` [M] — Create Steel Contour Plate
- `ail_create_part_by_three_points` [M] — Create Steel Curved Beam
- `ail_create_part_by_two_pointsitem` [M] — Create Steel Item
- `ail_create_crosssect` [M] — Create Steel Orthogonal Beam
- `ail_create_polygon_profilebeam` [M] — Create Steel Polybeam
- `ail_create_DM_helixsteel` [M] — Create Steel Spiral Beam
- `ail_create_diagon` [M] — Create Steel Twin Profile
- `ail_create_pluginWallLayout` [M] — Create Wall Layout
- `acmdDisplayGridLinePropertiesDialog` [M] — Grid Line Properties
- `acmdDisplayGridPropertiesDialog` [M] — Grid Properties
- `acmd_display_part_attr_dialog` [M] — Steel Beam Properties
- `acmd_display_column_dialog` [M] — Steel Column Properties
- `acmd_display_contour_plate_dialog` [M] — Steel Contour Plate Properties
- `acmdDisplayItemDialog` [M] — Steel Item Properties
- `acmd_display_crosssect_dialog` [M] — Steel Orthogonal Beam Properties
- `acmd_display_diagon_attr_dialog` [M] — Steel Twin Profile Properties
- `acmdDisplayParameterTableDialog` [M] — Variables

### Numbering

- `ail_create_macroS9` [M] — Assign Control Numbers
- `acmdAssignPositionNumbermulti_assembly` [M] — Change Assembly Multi Number
- `acmdAssignPositionNumberassembly` [M] — Change Assembly Number
- `acmdAssignPositionNumberfamily` [M] — Change Family Number
- `acmdAssignPositionNumbermulti_part` [M] — Change Part Multi Number
- `acmdAssignPositionNumberpart` [M] — Change Part Number
- `acmd_unnumber_selected_partsassembly_only` [M] — Clear Assembly Numbers
- `acmd_unnumber_selected_partspart_and_assembly_and_rebar` [M] — Clear Part And Assembly Numbers
- `acmd_unnumber_selected_partspart_only` [M] — Clear Part Numbers
- `acmd_unnumber_selected_partsrebar_only` [M] — Clear Rebar Numbers
- `acmd_shellexecute_open%XSBIN%\Applications\Tekla\Model\DesignGroupNumbering\DesignGroupNumbering` [M] — Design Group Numbering
- `acmdDisplayWeldNumberingDialog` [M] — Number Welds
- `acmd_prelim_mark_selected` [M] — Save Preliminary Numbers
- `acmd_display_partnumbers_set_options` [M] — Settings
- `ail_create_macroS10` [M] — Toggle Lock Control Numbers

### Options

- `acmdDisplayEnvironmentVariablesDialogFromMenu` [M] — Advanced Options
- `acmd_checkoption_callbackchkDrawingColorModeBlackAndWhite` [D] — Drawing Color Mode > Black And White (toggle)
- `acmd_checkoption_callbackchkDrawingColorModeColor` [D] — Drawing Color Mode > Color (toggle)
- `acmd_checkoption_callbackchkDrawingColorModeGrayScale` [D] — Drawing Color Mode > Gray Scale (toggle)
- `acmd_checkoption_callbackchkDrawingColorModeNext` [D] — Drawing Color Mode > Switch To Next (toggle)
- `acmdDisplayOptionSettingsDialog` [M] — Options
- `acmd_display_pick_grid_dialog` [M] — Snap Settings
- `acmdOptionsCheckoptionCallbackSnapTooltips` [M] — Snap Tooltips (toggle)
- `acmd_checkoption_callbackassociative_symbol` [D] — Toggle Associative Symbol (toggle)
- `acmdOptionsCheckoptionCallbackauto_rotation_center` [M] — Toggle Automatic Rotation Center (toggle)
- `acmd_basic_view_auto_rotate_callback` [M] — Toggle Basic View Auto Rotation (toggle)
- `acmd_zoom_centered_callback` [M] — Toggle Centered Zooms (toggle)
- `acmdOptionsCheckoptionCallbackcrossing_selection` [M] — Toggle Crossing Selection (toggle)
- `acmdCheckOptionCallbackcrossing_selection` [D] — Toggle Crossing Selection_Drawing (toggle)
- `acmdOptionsCheckoptionCallbackToggleDashedLineForHiddenLine` [M] — Toggle Dashed Line For Hidden Line (toggle)
- `acmd_checkoption_callbackdimension_creation_associativity` [D] — Toggle Display Associativity During Dimension Creation (toggle)
- `acmdOptionsCheckoptionCallbackenable_drag` [M] — Toggle Drag Drop (toggle)
- `acmd_checkoption_callbackdrawing_drag` [D] — Toggle Drawing Drag And Drop (toggle)
- `acmd_checkoption_callbackghost_drawing_enabled` [D] — Toggle Ghost Outline (toggle)
- `acmdOptionsCheckoptionCallbackToggleHatchingOfOverlappingSurfaces` [M] — Toggle Hatching Of Overlapping Surfaces (toggle)
- `acmdOptionsCheckoptionCallbackmiddle_button_pan` [M] — Toggle Middle Button Pan (toggle)
- `acmd_checkoption_callbackmiddle_button_pan` [D] — Toggle Middle Button Pan_Drawing (toggle)
- `acmdToggleOrtho` [M] — Toggle Ortho (toggle)
- `acmdOptionsCheckoptionCallbackpours_and_pour_breaks` [M] — Toggle Pours And Pour Breaks (toggle)
- `acmdOptionsCheckoptionCallbackrollover_highlight` [M] — Toggle Rollover Highlight (toggle)
- `acmdOptionsCheckoptionCallbackpopup_select` [M] — Toggle Select On Right Click (toggle)
- `acmd_set_draw_freeplace` [D] — Toggle Show Freeplace (toggle)
- `acmd_draw_protect` [D] — Toggle Show Protect (toggle)
- `acmdOptionsCheckoptionCallbacksmart_select` [M] — Toggle Smart Select (toggle)
- `acmd_set_DX_Rendering` [M] — Use Direct XRendering (toggle)
- `acmd_checkoption_callbackplotter_linecolors` [D] — Use Printer Line Colors (toggle)
- `acmd_checkoption_callbackplotter_linewidths` [D] — Use Printer Linewidths (toggle)

### Planning

- `acmd_do_selected_differences_to_model` [?] — Apply Selected Differences
- `acmd_close_plan` [?] — Close Planning Mode
- `acmd_ignore_selected_objects` [?] — Ignore Selected Objects
- `acmd_display_plan_info_dialog` [?] — Import Model Info
- `acmd_display_plan_setup_dialog` [?] — Import Model Setup
- `acmd_reject_selected_objects` [?] — Reject Selected Objects
- `acmd_plan_display_model` [?] — Toggle Display Model (toggle)
- `acmd_unignore_selected_objects` [?] — Unignore Selected Objects

### Points

- `acmdDisplayPropertiesAndAddPointAlongExtension` [M] — Add Along Extension Of Two Points
- `ail_create_vertex_point` [M] — Add At Any Position
- `ail_create_circle_line_intersection_points` [M] — Add At Intersection Of Circle And Line
- `ail_create_part_line_intersection_points` [M] — Add At Intersection Of Part And Line
- `ail_create_plane_line_intersection_point` [M] — Add At Intersection Of Plane And Line
- `ail_create_line_line_intersection_point` [M] — Add At Intersection Of Two Lines
- `ail_create_axis_intersection_point` [M] — Add At Intersection Of Two Part Axes
- `ail_create_screw_points` [M] — Add Bolt Points
- `acmdDisplayPropertiesAndAddPointOnLine` [M] — Add On Line
- `ail_create_point_raster` [M] — Add On Plane
- `acmdDisplayPropertiesAndAddPointParallelToPoints` [M] — Add Parallel To Two Points
- `ail_create_projection_point` [M] — Add Projected Points On Line
- `ail_create_tangent_point` [M] — Add Tangent To Circle
- `acmdDisplayPropertiesAndAddPointAlongArcUsingCandAPoints` [M] — Add Using Center And Arc Points
- `acmdDisplayPropertiesAndAddPointAlongArcUsingThreeArcPoints` [M] — Add Using Three Arc Points

### Pours

- `acmdCalculatePourUnitsCB` [M] — Calculate Pour Units
- `acmdDisplayPourBreakPropertiesDialog` [M] — Pour Break
- `acmdDisplayPourObjectPropertiesDialog` [M] — Pour Object

### Profiles

- `diaCreateNewCrossSectionSketch` [M] — Define Cross Section In Sketch Editor
- `acmdDisplayDWGProfileMacroDialog` [M] — Define Cross Section Using DWGFile
- `acmdDisplayPlateProfileMacroDialog` [M] — Define Cross Section Using Plate
- `ail_define_user_cross_sec_with_holes` [M] — Define Cross Section Using Polygon
- `acmdDisplayProfileEditorStartFromMenuDialog` [M] — Define Profile With Variable Cross Section
- `acmd_display_profcs_dialogNewModifyCrossSection` [M] — Edit Polygon Cross Section

### RebarAssembly

- `ailRebarAssAddToAssembly` [M] — Add As Sub Assembly
- `ailRebarAssExplodeAssembly` [M] — Explode
- `ailRebarAssExplodeSubassembly` [M] — Explode Sub Assembly
- `ailRebarAssRemoveFromAssembly` [M] — Remove From Assembly
- `ailRebarAssChangeMainObject` [M] — Set As New Main Object Of Assembly

### Reinforcement

- `ailAttachRebarAskRebarAndPart` [M] — Attach To Part
- `acmdDisplayCircularRebarGroupDialog` [M] — Circular Rebar Group Properties
- `ail_create_circular_rebarCoRebar.Circle` [M] — Create Circular Rebar Group
- `ail_create_DM_rebar_setcrossing beam` [M] — Create Crossing Beam Reinforcement Set
- `ail_create_curved_rebarCoRebar.Curve` [M] — Create Curved Reinforcing Bar Group
- `ail_create_DM_rebar_setlongitudinal beam` [M] — Create Longitudinal Beam Reinforcement Set
- `ail_create_single_rebarCoRebar.Single` [M] — Create Rebar
- `ailCreateRebarAssembly` [M] — Create Rebar Assembly
- `ail_create_DM_rebar_seton surface` [M] — Create Rebars On Surface
- `ail_create_pluginRebarEndAnchor` [M] — Create Reinforcement Anchor
- `ail_create_pluginRebarCoupler` [M] — Create Reinforcement Coupler
- `ail_create_rebar_meshCoRebar.Mesh` [M] — Create Reinforcement Mesh
- `ail_create_DM_rebar_setpoint input` [M] — Create Reinforcement Set Via Point Input
- `ail_create_rebar_spliceCoRebarSplice` [M] — Create Reinforcement Splice
- `ail_create_rebar_groupCoRebar.Group` [M] — Create Reinforcing Bar Group
- `ail_create_DM_rebar_setin slab` [M] — Create Slab Or Wall Reinforcement Set
- `ail_create_strand_rebarCoRebar.Strand` [M] — Create Strand Pattern
- `acmdDisplayCurvedRebarGroupDialog` [M] — Curved Rebar Group Properties
- `ailDetachRebar` [M] — Detach From Part
- `ailGroupRebars` [M] — Group Rebars
- `acmdRebarSetVisibilityrebar_dimensions` [M] — Rebar Dimension Visibility (toggle)
- `acmdDisplayRebarGroupDialog` [M] — Rebar Group Properties
- `acmdDisplaySingleRebarDialog` [M] — Rebar Properties
- `acmdRebarSetVisibilityend_detail_strips` [M] — Rebar Set End Detail Strip Visibility (toggle)
- `acmdRebarSetVisibilitygroup_coloring` [M] — Rebar Set Group Coloring (toggle)
- `acmdRebarSetVisibilityguidelines` [M] — Rebar Set Guideline Visibility (toggle)
- `acmdRebarSetVisibilitylegfaces` [M] — Rebar Set Leg Face Visibility (toggle)
- `acmdRebarSetVisibilityproperty_strips` [M] — Rebar Set Property Strip Visibility (toggle)
- `acmdRebarSetVisibilitysplitters` [M] — Rebar Set Splitter Visibility (toggle)
- `acmdRegenerateRebarSets` [M] — Regenerate Selected Reinforcement Sets
- `acmdDisplayRebarMeshDialog` [M] — Reinforcement Mesh Properties
- `acmdDisplayRebarSpliceDialog` [M] — Reinforcement Splice Properties
- `acmdDisplayStrandPatternDialog` [M] — Strand Pattern Properties
- `ailUngroupRebarGroup` [M] — Ungroup Rebars

### Reinforcements

- `ail_create_pluginRebarShapeCatalog` [M] — Rebar Shape Catalog

### Reports

- `acmd_display_report_dialog` [M] — Create Report

### Representation

- `acmd_set_view_representationhidden_lines_joint` [M] — Components > Hidden Lines
- `acmd_set_view_representationrendered_joint` [M] — Components > Rendered
- `acmd_set_view_representationshaded_wire_frame_joint` [M] — Components > Shaded Wireframe
- `acmd_set_view_representationshow_only_selected_joint` [M] — Components > Show Only Selected
- `acmd_set_view_representationwire_frame_joint` [M] — Components > Wireframe
- `acmd_set_view_representationshaded_wire_frame_hierarchy` [M] — Hierarchies > Shaded Wireframe
- `acmd_set_view_representationwire_frame_hierarchy` [M] — Hierarchies > Wireframe
- `acmdDisplayObjectRepresentationDialogViewMenu` [M] — Object Representation
- `acmd_set_view_representationhidden_lines` [M] — Parts > Hidden Lines
- `acmd_set_view_representationrendered` [M] — Parts > Rendered
- `acmd_set_view_representationshaded_wire_frame` [M] — Parts > Shaded Wireframe
- `acmd_set_view_representationshow_only_selected` [M] — Parts > Show Only Selected
- `acmd_set_view_representationwire_frame` [M] — Parts > Wireframe
- `acmd_set_view_representationhidden_lines_ref` [M] — References > Hidden Lines
- `acmd_set_view_representationrendered_ref` [M] — References > Rendered
- `acmd_set_view_representationshaded_wire_frame_ref` [M] — References > Shaded Wireframe
- `acmd_set_view_representationshow_only_selected_ref` [M] — References > Show Only Selected
- `acmd_set_view_representationwire_frame_ref` [M] — References > Wireframe
- `ail_display_joint` [M] — Show Component Content
- `ail_draw_hidden_linesown` [M] — Show Part With Exact Lines
- `diaPourRepresentation` [M] — Show Pours

### Shapes

- `acmdRunTSDrawingToolsMacroCombineRightClick` [D] — Combine
- `ail_draw_arc_2pc` [D] — Draw Arch By End Points And Center
- `ail_draw_arc_3p` [D] — Draw Arch By Three Points
- `ail_draw_circle` [D] — Draw Circle By Center And Radius
- `ail_draw_circle_3p` [D] — Draw Circle By Three Points
- `ail_draw_cloud` [D] — Draw Cloud
- `ail_draw_patch` [D] — Draw Cover Up Area
- `ail_draw_line_patch` [D] — Draw Cover Up Line
- `ail_draw_polygon_patch` [D] — Draw Cover Up Polygon Area
- `ail_draw_polyline_patch` [D] — Draw Cover Up Polyline
- `ail_draw_line` [D] — Draw Line
- `ail_draw_continuous_line` [D] — Draw Line > Continuous
- `ail_create_drawing_pluginPattern line` [D] — Draw Pattern Line
- `ail_draw_polygon` [D] — Draw Polygon
- `ail_draw_polyline` [D] — Draw Polyline
- `ail_draw_rectangle` [D] — Draw Rectangle
- `acmdRunTSDrawingToolsMacroExplodeRibbon` [D] — Explode

### Sharing

- `acmdExternalActionCallbackSharedModels` [M] — Browse Shared Models
- `acmdCreateBaselineCB` [M] — Create Baseline
- `acmdExcludeFromSharing` [M] — Exclude
- `acmdExternalActionCallbackManageUsers` [M] — List Users
- `acmdSynchronizationReadInCB` [M] — Read In
- `acmdExternalActionCallbackSharingUI.SharingSettings` [M] — Settings
- `acmdLaunchModelSharingChangeHistory` [M] — Sharing History
- `acmdLaunchModelSharingChangeManager` [M] — Show Read In Changes
- `acmdExternalActionCallbackStartSharing` [M] — Start Sharing
- `acmdSynchronizationWriteOutCB` [M] — Write Out

### SurfaceTreatment

- `ail_create_DM_surface_objectby_face` [M] — Create Surface Object
- `ail_create_surfacing_by_partco_part_surface` [M] — Create To All Faces Of Part
- `ail_create_surfacing_by_faceco_face_surface` [M] — Create To Part Face
- `ail_create_surfacingco_polygon_surface` [M] — Create To Selected Area On Part Face
- `acmdDisplaySurfacingDialog` [M] — Properties

### Tools

- `acmdDisplayActiveMultiusersDialog` [M] — Active Multi Users
- `acmdOpenClashCheckManager` [M] — Clash Check Manager
- `acmd_compare_two_objectsassembly` [M] — Compare Assemblies
- `acmd_compare_two_objectspart` [M] — Compare Parts
- `acmdLaunchObjectConverter` [M] — Convert IFCObjects
- `ail_create_thumbnail` [M] — Create Preview Image
- `amcdCreatePreviewImageCBclient_area_only` [D] — Create Preview Image_Drawing
- `acmdObjectConverterSettings` [M] — IFCObject Converter Settings
- `acmdDisplayLayoutManagerDialog` [M] — Layout Manager
- `acmd_load_model_attributes` [M] — Load Defaults
- `acmd_lotting` [M] — Lotting
- `acmdOpenConstructionManagement%XSBIN%\Applications\Tekla\Model\ObjectBrowser\ObjectBrowser` [M] — Organizer
- `acmd_display_active_system_dialog` [M] — Phase Manager
- `acmd4DTool` [M] — Project Status Visualization
- `ail_gr_bring_forward` [D] — Re Order > Bring Forward
- `ail_gr_bring_in_front_model_objects` [D] — Re Order > Bring In Front Of Model Objects
- `ail_gr_bring_to_front` [D] — Re Order > Bring To Front
- `ail_gr_send_backward` [D] — Re Order > Send Backward
- `ail_gr_send_behind_model_objects` [D] — Re Order > Send Behind Model Objects
- `ail_gr_send_to_back` [D] — Re Order > Send To Back
- `ail_create_pluginRebarShapeManager` [M] — Rebar Shape Manager
- `acmdRefreshIntelligence` [D] — Refresh Associativity
- `acmdRenumberGADrawing` [M] — Renumber GADrawings
- `acmd_save_model_attributes` [M] — Save Defaults
- `diaDisplaySnapshotDialog` [M] — Screenshot
- `dia_snapshot_cbdialog` [D] — Screenshot > Dialog
- `dia_snapshot_cb` [D] — Screenshot > From View
- `dia_snapshot_cbclient_area_only` [D] — Screenshot > From View Without Borders
- `dia_snapshot_cbmdi_frame` [D] — Screenshot > Main Frame
- `ail_create_sequence` [M] — Sequencer
- `acmdDisplaySiteManagerDialogTASK_PANE` [M] — Task Manager
- `acmd_snap_toggle_lock_cbx` [M] — Toggle Coordinate Lock X
- `acmd_snap_toggle_lock_cby` [M] — Toggle Coordinate Lock Y
- `acmd_snap_toggle_lock_cbz` [M] — Toggle Coordinate Lock Z
- `acmdToggleDimensions` [D] — Toggle Dimensions

### UserCoordinateSystem

- `acmd_reset_ucs_cball` [D] — Reset All
- `acmd_reset_ucs_cb` [D] — Reset Current
- `ail_gr_set_ucs_by_two_points` [D] — Set By Two Points
- `ail_gr_set_ucs_origin` [D] — Set Origin
- `acmd_toggle_ucs_orientation_cb` [D] — Toggle Orientation

### View

- `acmdDisplayAnyLinkPropertiesAndStartCommanddwg` [D] — Add From DWG_DFX
- `acmdDisplayAnyLinkPropertiesAndStartCommandimage` [D] — Add Image
- `acmdReFreeplaceSelected` [D] — Arrange Objects Ignoring Current Location
- `acmd_freeplace_selected` [D] — Arrange Objects Near Current Location
- `acmdClipOnlyReferenceObjects` [M] — Clip Only Reference Objects (toggle)
- `acmd_delete_view` [M] — Close
- `acmd_include_drawing_viewscopy_no_placing` [D] — Copy Views From Other Drawings
- `acmd_include_drawing_viewscopy_no_placing_use_layout` [D] — Copy Views From Other Drawings > With Layout
- `ailCreate3dView` [D] — Create3DView From Drawing
- `ail_create_DM_clip_box` [M] — Create Clip Box
- `ailCreateClipPlane` [M] — Create Clip Plane
- `ail_create_joint_basic_view` [M] — Create Component3DView
- `ail_create_joint_4_views` [M] — Create Component Default Views
- `ailCreateCurvedCutView` [D] — Create Drawing Curved Section View
- `ail_create_drawing_detail_viewView` [D] — Create Drawing Detail View
- `ail_create_cut_view` [D] — Create Drawing Section View
- `ail_create_DM_model_or_drawing_viewdrawing view` [M] — Create Drawing View
- `ail_create_view_from_modelFITTED` [D] — Create Drawing View From Entire Model
- `ail_create_view_from_model_by_area` [D] — Create Drawing View From Model By Area
- `ail_create_view_from_viewEXTREMA` [D] — Create Drawing View From Selected Area
- `ailCreateBackView` [D] — Create From Part In Drawing > From Back Plane
- `ailCreateDownView` [D] — Create From Part In Drawing > From Bottom Plane
- `ailCreateFrontView` [D] — Create From Part In Drawing > From Front Plane
- `ailCreateUpView` [D] — Create From Part In Drawing > From Top Plane
- `ail_create_basic_view` [M] — Create Model Basic View
- `ail_create_DM_model_or_drawing_viewmodel view` [M] — Create Model View
- `acmdDisplayGridViewsDialog` [M] — Create Model View Along Grid Lines
- `ail_create_view_on_part_planeback` [M] — Create Model View On Part Back Plane
- `ail_create_view_on_part_planebottom` [M] — Create Model View On Part Bottom Plane
- `ail_create_view_on_part_planefront` [M] — Create Model View On Part Front Plane
- `ail_create_view_on_part_planetop` [M] — Create Model View On Part Top Plane
- `ail_create_DM_viewby_face` [M] — Create Model View On Plane
- `acmd_create_view_to_workplane` [M] — Create Model View On Work Plane
- `ail_create_view_by_points` [M] — Create Model View Using Three Points
- `ail_create_view_by_two_points` [M] — Create Model View Using Two Points
- `ail_create_part_basic_view` [M] — Create Part3DView
- `ail_create_part_basic_views` [M] — Create Part Default Views
- `ail_create_part_undeformed_view` [M] — Create Part Undeformed View
- `ailZoomCreateWindow` [D] — Create Zoom Window
- `ailDeleteAllClipPlanes` [M] — Delete All Clip Planes
- `acmd_fit_workarea` [M] — Fit To Entire Model
- `acmd_fit_workarea_all` [M] — Fit To Entire Model > All Views
- `acmd_fit_workarea_by_parts` [M] — Fit To Selected Parts
- `acmd_fit_workarea_by_parts_all` [M] — Fit To Selected Parts > All Views
- `ail_pick_work_area` [M] — Fit Using Two Points
- `ailFly` [?] — Fly
- `ailHideObject` [M] — Hide Object
- `acmdDrawingHideObjectdrawing` [D] — Hide Object From Drawing
- `acmdDrawingHideObjectview` [D] — Hide Object From View
- `acmd_include_drawing_viewslink_no_placing` [D] — Link Views From Other Drawings
- `acmd_include_drawing_viewslink_no_placing_use_layout` [D] — Link Views From Other Drawings > With Layout
- `acmd_center` [?] — Move > Center By Cursor
- `acmd_move_viewX-5.0` [?] — Move X_Minus5_0
- `acmd_move_viewX5.0` [?] — Move X_Plus5_0
- `acmd_move_viewZ-5.0` [?] — Move Z_Minus5_0
- `acmd_move_viewZ5.0` [?] — Move Z_Plus5_0
- `ailPanCONTINUOUS` [?] — Pan
- `acmd_display_dialogdia_view_dialog` [M] — Properties
- `acmd_display_attr_dialogview_dial` [D] — Properties_Drawing
- `acmd_redraw_view` [M] — Redraw
- `ailRotate` [?] — Rotate
- `ailAutoRotate` [?] — Rotate > One Round
- `ail_set_view_point` [?] — Rotate > Set View Point
- `acmd_disable_view_rotation` [?] — Rotate > Toggle Disable View Rotation (toggle)
- `acmd_display_rotate_view_dialog` [D] — Rotate View
- `acmd_rotate_viewX-15.0` [?] — Rotate X_Minus15_0
- `acmd_rotate_viewX-5.0` [?] — Rotate X_Minus5_0
- `acmd_rotate_viewX15.0` [?] — Rotate X_Plus15_0
- `acmd_rotate_viewX5.0` [?] — Rotate X_Plus5_0
- `acmd_rotate_viewZ-15.0` [?] — Rotate Z_Minus15_0
- `acmd_rotate_viewZ-5.0` [?] — Rotate Z_Minus5_0
- `acmd_rotate_viewZ15.0` [?] — Rotate Z_Plus15_0
- `acmd_rotate_viewZ5.0` [?] — Rotate Z_Plus5_0
- `ail_workplane_to_view` [M] — Set Work Plane > Parallel To View Plane
- `acmd_change_workfloor` [M] — Set Work Plane > Parallel To XYZ
- `ail_change_work_plane_on_part_planeback` [M] — Set Work Plane > To Part Back Plane
- `ail_change_work_plane_on_part_planebottom` [M] — Set Work Plane > To Part Bottom Plane
- `ail_change_work_plane_on_part_planefront` [M] — Set Work Plane > To Part Front Plane
- `ail_change_work_plane_on_part_planetop` [M] — Set Work Plane > To Part Top Plane
- `ail_set_workfloor_by_point` [M] — Set Work Plane > Using One Point
- `ail_set_workfloor_by_points` [M] — Set Work Plane > Using Three Points
- `ail_set_workfloor_by_two_points` [M] — Set Work Plane > Using Two Points
- `ailDMSetWorkplane` [M] — Set Work Plane > Using Workplane Tool
- `acmdDrawingUnhideObjectdrawing` [D] — Show Object In Drawing
- `acmdDrawingUnhideObjectview` [D] — Show Object In View
- `acmd_toggle_view_perspective` [?] — Toggle Perspective
- `acmd_update_view` [?] — Update
- `acmd_zoom_in` [?] — Zoom In
- `ailZoomInCONTINUOUS` [?] — Zoom In Special
- `acmd_zoom_original` [?] — Zoom Original
- `ailZoomOriginal` [?] — Zoom Original Special
- `acmd_zoom_out` [?] — Zoom Out
- `ailZoomOutCONTINUOUS` [?] — Zoom Out Special
- `acmd_zoom_previous` [?] — Zoom Previous
- `ailZoomPrevious` [?] — Zoom Previous Special
- `acmdZoomToSelected` [?] — Zoom To Selected

### Views

- `acmd_place_drawing_views` [D] — Arrange
- `acmd_cascade_views` [?] — Cascade
- `acmd_close_views` [M] — Close All
- `acmd_redraw_views` [M] — Redraw All
- `acmd_tile_views` [?] — Tile Horizontally
- `acmd_tile_viewsVERTICALLY` [?] — Tile Vertically
- `acmd_update_views` [?] — Update All
- `ail_named_views` [?] — View List

### Weld

- `ail_convert_to_polygon_weld` [M] — Convert To Polygon Weld
- `ail_create_welding` [M] — Create Between Parts
- `ail_create_welding_with_polygon` [M] — Create Polygon
- `ail_create_welding_single_part` [M] — Create To Part
- `ail_create_weld_cut` [M] — Prepare Part > With Another Part
- `ail_create_weld_cut_with_polygon` [M] — Prepare Part > With Polygon
- `acmd_display_welding_attr_dialog` [M] — Properties
