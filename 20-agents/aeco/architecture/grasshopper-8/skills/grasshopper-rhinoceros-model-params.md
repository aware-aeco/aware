---
name: grasshopper-grasshopper-rhinoceros-model-params
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Rhinoceros.Model.Params namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Param_EarthAnchorPoint, Param_ModelLayer, Param_ModelInstanceDefinition, Param_ModelObject.
---

# Grasshopper.Rhinoceros.Model.Params

Auto-generated from vendor docs for grasshopper 8.0. 4 types in this namespace.

## Param_EarthAnchorPoint (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.Params.Param_EarthAnchorPoint.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_Params_Param_EarthAnchorPoint.htm)

### Constructors
- `public Param_EarthAnchorPoint()` — Initializes a new instance of the Param_EarthAnchorPoint class

### Methods
#### `public virtual void AddRuntimeMessage(GH_RuntimeMessageLevel level, string text)`

Add a new message to this object. Valid message type flags are Warning and Error. If the Message string is empty or zero-length no message is added.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Type of message. Only Warnings and Errors are recorded.
- `text` (System.String) — Content of message.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AddRuntimeMessage.htm)

#### `public virtual void AddSource(IGH_Param source)`

Append a new Source parameter to the end of the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource.htm)

#### `public virtual void AddSource(IGH_Param source, int index)`

Insert a new Source parameter into the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.
- `index` (System.Int32) — Insertion index of source.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource_1.htm)

#### `public bool AddVolatileData(GH_Path path, int index, Object data)`

Inserts an item of volatile data into the data structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The branch path of the data. If the path doesn't exist yet, it will be created.
- `index` (System.Int32) — The item index of the data. If the path doesn't contain the index yet, it will be enlarged to encompass the index.
- `data` (System.Object) — The data to store.

**Returns:** `Boolean` — True on success, False on failure. If the data cannot be converted, the topology will remain unmolested.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData.htm)

#### `public bool AddVolatileData(GH_Path path, int index, T data)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `index` (System.Int32)
- `data` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, IEnumerable list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.IEnumerable)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, List<T> list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.Generic.List<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList.htm)

#### `public bool AddVolatileDataTree(GH_Structure<T> tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.GH_Structure<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree.htm)

#### `public bool AddVolatileDataTree(IGH_Structure tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.IGH_Structure)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree_1.htm)

#### `public virtual void AddedToDocument(GH_Document document)`

This method will be called when an object is added to a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_AddedToDocument.htm)

#### `public override void AppendAdditionalMenuItems(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AppendAdditionalMenuItems.htm)

#### `public override bool AppendMenuItems(ToolStripDropDown menu)`

This function is called when a context menu is about to be displayed. Override it to set custom items. GH_ActiveObject will already populate the menu with default items, if you merely wish to insert object-specific menu item, consider overriding AppendAdditionalMenuItems instead.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown) — Menu object to populate.

**Returns:** `Boolean` — If true, the menu will be displayed, if false the menu will be supressed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AppendMenuItems.htm)

#### `public void BakeGeometry(RhinoDoc doc, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry_1.htm)

#### `public void BakeGeometry(RhinoDoc doc, ObjectAttributes att, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry.htm)

#### `public override void ClearData()`

(Inherited from GH_PersistentReferenceParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentReferenceParam_1_ClearData.htm)

#### `public virtual void ClearProxySources()`

Remove all proxy sources without attempting to relink them.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ClearProxySources.htm)

#### `public virtual void ClearRuntimeMessages()`

Destroy all warning and error lists

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ClearRuntimeMessages.htm)

#### `public override sealed void CollectData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CollectData.htm)

#### `public override sealed void ComputeData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ComputeData.htm)

#### `public void CopyFrom(IGH_InstanceDescription other)`

Copy all fields (except the instance ID) from another instance description.

**Parameters:**
- `other` (Grasshopper.Kernel.IGH_InstanceDescription) — Object to mimic.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_CopyFrom.htm)

#### `public override void CreateAttributes()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateAttributes.htm)

#### `public virtual void CreateProxySources()`

Convert all proper source parameters into proxy sources.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateProxySources.htm)

#### `public override bool DependsOn(IGH_ActiveObject potential_source)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `potential_source` (Grasshopper.Kernel.IGH_ActiveObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_DependsOn.htm)

#### `public virtual void DocumentContextChanged(GH_Document document, GH_DocumentContext context)`

This method will be called when the document that owns this object moves into a different context.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that owns this object.
- `context` (Grasshopper.Kernel.GH_DocumentContext) — The reason for this event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_DocumentContextChanged.htm)

#### `public virtual void ExpirePreview(bool redraw)`

Call this function when you suspect that the preview has expired for this object. This will cause the display cache to be eradicated.

**Parameters:**
- `redraw` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_ExpirePreview.htm)

#### `public override void ExpireSolution(bool recompute)`

Informs the document that owns this object that the solution has expired. The current object will be set to BLANK as a result. This method is recursive, it will also expire any and all objects which depend on this object. If you want a less destructive expiration, consider using ClearData(). If this object is already Blank, you should consider not expiring it.

**Parameters:**
- `recompute` (System.Boolean) — If True, the document will be instructed to solve the new state.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ExpireSolution.htm)

#### `public void ExpireSolutionTopLevel(bool recompute)`

Invoke the Expiresolution(bool) method on the toplevel object.

**Parameters:**
- `recompute` (System.Boolean) — If true, a new computation will start right away.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ExpireSolutionTopLevel.htm)

#### `public override void IsolateObject()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_IsolateObject.htm)

#### `public virtual void MovedBetweenDocuments(GH_Document oldDocument, GH_Document newDocument)`

This method will be called when an object is moved from one document to another. Override this method if you want to handle such events.

**Parameters:**
- `oldDocument` (Grasshopper.Kernel.GH_Document) — Document that used to own this object.
- `newDocument` (Grasshopper.Kernel.GH_Document) — Document that now owns ths object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_MovedBetweenDocuments.htm)

#### `public void NewInstanceGuid()`

Generate a new random instance GUID

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid UUID)`

Set the instance ID to be a specific GUID. This is very dangerous, only use this function if you're 6"4' and your first name is David.

**Parameters:**
- `UUID` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid_1.htm)

#### `public void OnAttributesChanged()`

Raises the AttributesChanged event on the toplevel object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnAttributesChanged.htm)

#### `public void OnDisplayExpired(bool redraw)`

Raises the DisplayExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the canvas will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnDisplayExpired.htm)

#### `public void OnObjectChanged(GH_ObjectChangedEventArgs e)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `e` (Grasshopper.Kernel.GH_ObjectChangedEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_1.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_2.htm)

#### `public void OnObjectChanged(string customEvent)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_3.htm)

#### `public void OnObjectChanged(string customEvent, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_4.htm)

#### `public GH_Document OnPingDocument()`

Raise the PingDocument Event on the toplevel object and try to find the document which owns this object.

**Returns:** `GH_Document` — The document which owns this object if successful, or nothing if no document owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPingDocument.htm)

#### `public void OnPreviewExpired(bool redraw)`

Raises the PreviewExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the viewports will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPreviewExpired.htm)

#### `public void OnSolutionExpired(bool recompute)`

Raises the SolutionExpired event on the toplevel object. You probably want to call ExpireSolution() instead of this method directly.

**Parameters:**
- `recompute` (System.Boolean) — If True, the solution will be immediately recalculated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnSolutionExpired.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Read.htm)

#### `public virtual bool ReadFull(GH_IReader reader)`

GH_InstanceDescription does not by default serialize all fields. Use this function to read all fields from the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Writer for deserialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_ReadFull.htm)

#### `public void RecordUndoEvent(GH_UndoRecord record)`

Record an entire undo record.

**Parameters:**
- `record` (Grasshopper.Kernel.Undo.GH_UndoRecord) — Record to push.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent.htm)

#### `public Guid RecordUndoEvent(string undoName)`

Record a generic object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_1.htm)

#### `public Guid RecordUndoEvent(string undoName, IGH_UndoAction action)`

Record a specific object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.
- `action` (Grasshopper.Kernel.Undo.IGH_UndoAction) — Undo action to record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_2.htm)

#### `public override void RegisterRemoteIDs(GH_GuidTable table)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `table` (GH_GuidTable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_RegisterRemoteIDs.htm)

#### `public virtual bool RelinkProxySources(GH_Document document)`

Attempt to replace all proxy sources with real sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — The document from which to harvest the real source parameters.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RelinkProxySources.htm)

#### `public virtual void RemoveAllSources()`

Remove all sources from this parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveAllSources.htm)

#### `public virtual void RemoveEffects()`

Remove all post-process effects. Note to implementors, you must call the base method if you override this function.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveEffects.htm)

#### `public virtual void RemoveSource(Guid source_id)`

Remove the specified source from this parameter.

**Parameters:**
- `source_id` (System.Guid) — InstanceID of source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource_1.htm)

#### `public virtual void RemoveSource(IGH_Param source)`

Remove the specified source from this parameter.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource.htm)

#### `public virtual void RemovedFromDocument(GH_Document document)`

This method will be called when an object is removed from a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now no longer owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RemovedFromDocument.htm)

#### `public virtual void ReplaceSource(Guid old_source_id, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source_id` (System.Guid) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource_1.htm)

#### `public virtual void ReplaceSource(IGH_Param old_source, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source` (Grasshopper.Kernel.IGH_Param) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource.htm)

#### `public virtual IList<string> RuntimeMessages(GH_RuntimeMessageLevel level)`

Gets the list of cached runtime messages that were recorded during solver-time processes.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Runtime message type to retrieve.

**Returns:** `IList<String>` — A list of runtime message strings.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_RuntimeMessages.htm)

#### `public virtual bool SDKCompliancy(int exeVersion, int exeServiceRelease)`

Test whether this object is compliant with a given Rhino version.

**Parameters:**
- `exeVersion` (System.Int32) — Rhino major release (4, 5, ...).
- `exeServiceRelease` (System.Int32) — Rhino minor (service) release.

**Returns:** `Boolean` — True if this object is compliant with the given Rhino release.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_SDKCompliancy.htm)

#### `public void SetIconOverride(Bitmap customIcon)`

Set a new custom icon override for this object.

**Parameters:**
- `customIcon` (System.Drawing.Bitmap) — Icon override. Should be a 24x24 image.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetIconOverride.htm)

#### `public void SetPersistentData(GH_Structure<T> items)`

Assign a tree of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (Grasshopper.Kernel.Data.GH_Structure<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData.htm)

#### `public void SetPersistentData(IEnumerable<T> items)`

Assign a list of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_1.htm)

#### `public void SetPersistentData(params Object[] values)`

Add a collection of values to the persistent data.

**Parameters:**
- `values` (System.Object[]) — Values to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_2.htm)

#### `public void SetPersistentData(T item)`

Add a single item to the persistent data. This method will record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add more than one piece of data, you should use the appropriate overload for this method.

**Parameters:**
- `item` (T) — Item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_3.htm)

#### `public bool SetPrincipal(bool set, bool recordUndo, bool recompute)`

Set the principal parameter state.

**Parameters:**
- `set` (System.Boolean)
- `recordUndo` (System.Boolean)
- `recompute` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_SetPrincipal.htm)

#### `public void TriggerAutoSave()`

Triggers the AutoSave function on the owner document with the object_changed flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_1.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger, Guid id)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_2.htm)

#### `public void TriggerAutoSave(Guid id)`

Triggers the AutoSave function on the owner document with the object_changed flag.

**Parameters:**
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_3.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Write.htm)

#### `public virtual bool WriteFull(GH_IWriter writer)`

GH_InstanceDescription does not by default serialize all fields. Use this function to write all fields to the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer for serialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_WriteFull.htm)

### Properties
- `Access` (GH_ParamAccess, get/set) — Gets or sets the Access level for this parameter.
- `Attributes` (IGH_Attributes, get/set) — Gets or sets the attributes that are associated with this object. Only set custom attributes if you know what you are doing.
- `Category` (String, get/set) — Gets or sets the Category in which this object belongs. If HasCategory() returns false, this field has no meaning.
- `ComponentGuid` (Guid, get) — (Overrides GH_DocumentObject.ComponentGuid.)
- `DataMapping` (GH_DataMapping, get/set) — Gets or sets the data mapping of this Parameter.
- `DataType` (GH_ParamData, get) — Gets the type of data stored in this parameter.
- `Description` (String, get/set) — Gets or sets the description of the object. This field typically remains fixed during the lifetime of an object.
- `Exposure` (GH_Exposure, get) — (Overrides GH_DocumentObject.Exposure.)
- `HasCategory` (Boolean, get) — Gets whether or not the Category field has been set.
- `HasProxySources` (Boolean, get) — Gets a value indicating whether or not this parameter maintains proxy sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `HasSubCategory` (Boolean, get) — Gets whether or not the SubCategory field has been set.
- `Icon_24x24` (Bitmap, get) — The icon associated with this object.
- `Icon_24x24_Locked` (Bitmap, get) — The greyscale icon of this object.
- `IconCapableUI` (Boolean, get) — By default the NickName menu item supports the Icon Mode override toggle. If your UI is not capable of displaying icons, then override this property and return False.
- `IconDisplayMode` (GH_IconDisplayMode, get/set) — Gets the current display mode of the object.
- `InstanceDescription` (String, get) — Gets the description of this instance. The default description is about the amount and source of the local values.
- `InstanceGuid` (Guid, get) — Gets the ID of this runtime instance.
- `IsBakeCapable` (Boolean, get) — (Inherited from ModelContentParam<T>.)
- `IsDataProvider` (Boolean, get) — (Inherited from GH_Param<T>.)
- `IsPrincipal` (GH_PrincipalState, get) — Gets whether this parameter is a principal parameter. Principal parameters are maintained by components and are not part of the IGH_Param interface.
- `Keywords` (IEnumerable<String>, get) — Gets a list of additional keywords that describe the object. Typically this list is empty but you can override this property to aid in object searches.
- `Kind` (GH_ParamKind, get) — Gets the parameter kind. The kind is evaluated lazily and cached.
- `Locked` (Boolean, get/set) — Gets or sets the enabled flag of this object. Disabled objects are ignored during solutions.
- `MutableNickName` (Boolean, get/set) — Gets or sets a value that enables Nick name changes through the menu. The default is TRUE.
- `Name` (String, get/set) — Gets or sets the name of the object. This field typically remains fixed during the lifetime of an object.
- `NickName` (String, get/set) — Gets or sets the nickname of the object. This field can be changed by the user.
- `Obsolete` (Boolean, get) — Gets whether this object is obsolete. Default implementation returns true if the class name contains the string "OBSOLETE" or if this class has been decorated with the ObsoleteAttribute. You are free to override this if you want, but I suggest adding the ObsoleteAttribute instead.
- `Optional` (Boolean, get/set) — Gets or sets whether or not this parameter is considered optional by the owner component. Empty, non-optional parameters prevent the component from being solved.
- `PersistentData` (GH_Structure<T>, get) — Gets the persistent data stored in this parameter. If you modify the persistent data, be sure to call the: OnObjectChanged(GH_ObjectEventType.PersistentData) event.
- `PersistentDataCount` (Int32, get) — Gets the number of persistent data items stored in this parameter.
- `Phase` (GH_SolutionPhase, get/set) — Gets or sets the solution phase this object is currenly in.
- `ProcessorTime` (TimeSpan, get) — (Inherited from GH_Param<T>.)
- `ProxySourceCount` (Int32, get) — Gets the number of proxy sources for this parameter. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `Recipients` (IList<IGH_Param>, get) — Gets a list of all the recipients of this parameter. I.e. a recipient has this parameter as one of the sources. The Recipient list is maintained by the parameter, do not modify it yourself.
- `Reverse` (Boolean, get/set) — Gets or sets the data reverse modifier of this parameter.
- `RuntimeMessageLevel` (GH_RuntimeMessageLevel, get) — Returns the worst case flag for the current object If the object contains at least 1 error, the result is Error.If the object contains at least 1 warning, the result is Warning.If the object contains at least 1 message, the result is Remark.If the object contains no errors, no warnings and no messages, the result is Blank.
- `Simplify` (Boolean, get/set) — Gets or sets the simplify modifier for this parameter.
- `SourceCount` (Int32, get) — Gets the number of sources for this parameter.
- `Sources` (IList<IGH_Param>, get) — Gets a list of source parameters. Do not modify this list, if you wish to add or remove sources, use dedicated functions like AddSource() and RemoveSource() instead.
- `StateTags` (GH_StateTagList, get) — Gets all the StateTags that are associated with this parameter. A state tag is a visual feedback icon that represents specific internal settings.
- `SubCategory` (String, get/set) — Gets or sets the SubCategory in which this object belongs. If HasSubCategory() returns false, this field has no meaning.
- `Type` (Type, get) — Gets the Framework Type descriptor for the stored Data.
- `TypeName` (String, get) — (Inherited from GH_PersistentReferenceParam<T>.)
- `VolatileData` (IGH_Structure, get) — (Inherited from GH_Param<T>.)
- `VolatileDataCount` (Int32, get) — (Inherited from GH_Param<T>.)
- `WireDisplay` (GH_ParamWireDisplay, get/set) — Gets or sets the wire display style for this parameter. Wire display only affects the wires connected to the parameter input.

### Events
#### `AttributesChanged` (`Grasshopper.Kernel.IGH_DocumentObject.AttributesChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.AttributesChangedEventHandler AttributesChanged`

Raised whenever the number or kind of attributes changes. This event is handled by GH_Documents who subsequently wipe their attribute caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_AttributesChanged.htm)

#### `DisplayExpired` (`Grasshopper.Kernel.IGH_DocumentObject.DisplayExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.DisplayExpiredEventHandler DisplayExpired`

Raised whenever the display (on the Canvas) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_DisplayExpired.htm)

#### `ObjectChanged` (`Grasshopper.Kernel.IGH_DocumentObject.ObjectChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.ObjectChangedEventHandler ObjectChanged`

(Inherited from GH_DocumentObject.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_ObjectChanged.htm)

#### `PingDocument` (`Grasshopper.Kernel.IGH_DocumentObject.PingDocumentEventHandler`)

**Signature:** `public event IGH_DocumentObject.PingDocumentEventHandler PingDocument`

Raised whenever an object needs to know which GH_Document it belongs to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PingDocument.htm)

#### `PreviewExpired` (`Grasshopper.Kernel.IGH_DocumentObject.PreviewExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.PreviewExpiredEventHandler PreviewExpired`

Raised whenever the display (in the Rhino viewports) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PreviewExpired.htm)

#### `SolutionExpired` (`Grasshopper.Kernel.IGH_DocumentObject.SolutionExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.SolutionExpiredEventHandler SolutionExpired`

Raised whenever the solution of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_SolutionExpired.htm)

## Param_ModelInstanceDefinition (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.Params.Param_ModelInstanceDefinition.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_Params_Param_ModelInstanceDefinition.htm)

### Constructors
- `public Param_ModelInstanceDefinition()` — Initializes a new instance of the Param_ModelInstanceDefinition class

### Methods
#### `public virtual void AddRuntimeMessage(GH_RuntimeMessageLevel level, string text)`

Add a new message to this object. Valid message type flags are Warning and Error. If the Message string is empty or zero-length no message is added.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Type of message. Only Warnings and Errors are recorded.
- `text` (System.String) — Content of message.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AddRuntimeMessage.htm)

#### `public virtual void AddSource(IGH_Param source)`

Append a new Source parameter to the end of the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource.htm)

#### `public virtual void AddSource(IGH_Param source, int index)`

Insert a new Source parameter into the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.
- `index` (System.Int32) — Insertion index of source.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource_1.htm)

#### `public bool AddVolatileData(GH_Path path, int index, Object data)`

Inserts an item of volatile data into the data structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The branch path of the data. If the path doesn't exist yet, it will be created.
- `index` (System.Int32) — The item index of the data. If the path doesn't contain the index yet, it will be enlarged to encompass the index.
- `data` (System.Object) — The data to store.

**Returns:** `Boolean` — True on success, False on failure. If the data cannot be converted, the topology will remain unmolested.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData.htm)

#### `public bool AddVolatileData(GH_Path path, int index, T data)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `index` (System.Int32)
- `data` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, IEnumerable list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.IEnumerable)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, List<T> list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.Generic.List<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList.htm)

#### `public bool AddVolatileDataTree(GH_Structure<T> tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.GH_Structure<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree.htm)

#### `public bool AddVolatileDataTree(IGH_Structure tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.IGH_Structure)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree_1.htm)

#### `public virtual void AddedToDocument(GH_Document document)`

This method will be called when an object is added to a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_AddedToDocument.htm)

#### `public override void AppendAdditionalMenuItems(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AppendAdditionalMenuItems.htm)

#### `public override bool AppendMenuItems(ToolStripDropDown menu)`

This function is called when a context menu is about to be displayed. Override it to set custom items. GH_ActiveObject will already populate the menu with default items, if you merely wish to insert object-specific menu item, consider overriding AppendAdditionalMenuItems instead.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown) — Menu object to populate.

**Returns:** `Boolean` — If true, the menu will be displayed, if false the menu will be supressed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AppendMenuItems.htm)

#### `public void BakeGeometry(RhinoDoc doc, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry_1.htm)

#### `public void BakeGeometry(RhinoDoc doc, ObjectAttributes att, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry.htm)

#### `public override void ClearData()`

(Inherited from GH_PersistentReferenceParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentReferenceParam_1_ClearData.htm)

#### `public virtual void ClearProxySources()`

Remove all proxy sources without attempting to relink them.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ClearProxySources.htm)

#### `public virtual void ClearRuntimeMessages()`

Destroy all warning and error lists

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ClearRuntimeMessages.htm)

#### `public override sealed void CollectData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CollectData.htm)

#### `public override sealed void ComputeData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ComputeData.htm)

#### `public void CopyFrom(IGH_InstanceDescription other)`

Copy all fields (except the instance ID) from another instance description.

**Parameters:**
- `other` (Grasshopper.Kernel.IGH_InstanceDescription) — Object to mimic.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_CopyFrom.htm)

#### `public override void CreateAttributes()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateAttributes.htm)

#### `public virtual void CreateProxySources()`

Convert all proper source parameters into proxy sources.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateProxySources.htm)

#### `public override bool DependsOn(IGH_ActiveObject potential_source)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `potential_source` (Grasshopper.Kernel.IGH_ActiveObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_DependsOn.htm)

#### `public virtual void DocumentContextChanged(GH_Document document, GH_DocumentContext context)`

This method will be called when the document that owns this object moves into a different context.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that owns this object.
- `context` (Grasshopper.Kernel.GH_DocumentContext) — The reason for this event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_DocumentContextChanged.htm)

#### `public void DrawViewportMeshes(IGH_PreviewArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.IGH_PreviewArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_Params_Param_ModelInstanceDefinition_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(IGH_PreviewArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.IGH_PreviewArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_Params_Param_ModelInstanceDefinition_DrawViewportWires.htm)

#### `public virtual void ExpirePreview(bool redraw)`

Call this function when you suspect that the preview has expired for this object. This will cause the display cache to be eradicated.

**Parameters:**
- `redraw` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_ExpirePreview.htm)

#### `public override void ExpireSolution(bool recompute)`

Informs the document that owns this object that the solution has expired. The current object will be set to BLANK as a result. This method is recursive, it will also expire any and all objects which depend on this object. If you want a less destructive expiration, consider using ClearData(). If this object is already Blank, you should consider not expiring it.

**Parameters:**
- `recompute` (System.Boolean) — If True, the document will be instructed to solve the new state.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ExpireSolution.htm)

#### `public void ExpireSolutionTopLevel(bool recompute)`

Invoke the Expiresolution(bool) method on the toplevel object.

**Parameters:**
- `recompute` (System.Boolean) — If true, a new computation will start right away.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ExpireSolutionTopLevel.htm)

#### `public override void IsolateObject()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_IsolateObject.htm)

#### `public virtual void MovedBetweenDocuments(GH_Document oldDocument, GH_Document newDocument)`

This method will be called when an object is moved from one document to another. Override this method if you want to handle such events.

**Parameters:**
- `oldDocument` (Grasshopper.Kernel.GH_Document) — Document that used to own this object.
- `newDocument` (Grasshopper.Kernel.GH_Document) — Document that now owns ths object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_MovedBetweenDocuments.htm)

#### `public void NewInstanceGuid()`

Generate a new random instance GUID

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid UUID)`

Set the instance ID to be a specific GUID. This is very dangerous, only use this function if you're 6"4' and your first name is David.

**Parameters:**
- `UUID` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid_1.htm)

#### `public void OnAttributesChanged()`

Raises the AttributesChanged event on the toplevel object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnAttributesChanged.htm)

#### `public void OnDisplayExpired(bool redraw)`

Raises the DisplayExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the canvas will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnDisplayExpired.htm)

#### `public void OnObjectChanged(GH_ObjectChangedEventArgs e)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `e` (Grasshopper.Kernel.GH_ObjectChangedEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_1.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_2.htm)

#### `public void OnObjectChanged(string customEvent)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_3.htm)

#### `public void OnObjectChanged(string customEvent, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_4.htm)

#### `public GH_Document OnPingDocument()`

Raise the PingDocument Event on the toplevel object and try to find the document which owns this object.

**Returns:** `GH_Document` — The document which owns this object if successful, or nothing if no document owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPingDocument.htm)

#### `public void OnPreviewExpired(bool redraw)`

Raises the PreviewExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the viewports will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPreviewExpired.htm)

#### `public void OnSolutionExpired(bool recompute)`

Raises the SolutionExpired event on the toplevel object. You probably want to call ExpireSolution() instead of this method directly.

**Parameters:**
- `recompute` (System.Boolean) — If True, the solution will be immediately recalculated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnSolutionExpired.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Read.htm)

#### `public virtual bool ReadFull(GH_IReader reader)`

GH_InstanceDescription does not by default serialize all fields. Use this function to read all fields from the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Writer for deserialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_ReadFull.htm)

#### `public void RecordUndoEvent(GH_UndoRecord record)`

Record an entire undo record.

**Parameters:**
- `record` (Grasshopper.Kernel.Undo.GH_UndoRecord) — Record to push.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent.htm)

#### `public Guid RecordUndoEvent(string undoName)`

Record a generic object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_1.htm)

#### `public Guid RecordUndoEvent(string undoName, IGH_UndoAction action)`

Record a specific object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.
- `action` (Grasshopper.Kernel.Undo.IGH_UndoAction) — Undo action to record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_2.htm)

#### `public override void RegisterRemoteIDs(GH_GuidTable table)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `table` (GH_GuidTable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_RegisterRemoteIDs.htm)

#### `public virtual bool RelinkProxySources(GH_Document document)`

Attempt to replace all proxy sources with real sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — The document from which to harvest the real source parameters.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RelinkProxySources.htm)

#### `public virtual void RemoveAllSources()`

Remove all sources from this parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveAllSources.htm)

#### `public virtual void RemoveEffects()`

Remove all post-process effects. Note to implementors, you must call the base method if you override this function.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveEffects.htm)

#### `public virtual void RemoveSource(Guid source_id)`

Remove the specified source from this parameter.

**Parameters:**
- `source_id` (System.Guid) — InstanceID of source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource_1.htm)

#### `public virtual void RemoveSource(IGH_Param source)`

Remove the specified source from this parameter.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource.htm)

#### `public virtual void RemovedFromDocument(GH_Document document)`

This method will be called when an object is removed from a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now no longer owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RemovedFromDocument.htm)

#### `public virtual void ReplaceSource(Guid old_source_id, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source_id` (System.Guid) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource_1.htm)

#### `public virtual void ReplaceSource(IGH_Param old_source, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source` (Grasshopper.Kernel.IGH_Param) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource.htm)

#### `public virtual IList<string> RuntimeMessages(GH_RuntimeMessageLevel level)`

Gets the list of cached runtime messages that were recorded during solver-time processes.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Runtime message type to retrieve.

**Returns:** `IList<String>` — A list of runtime message strings.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_RuntimeMessages.htm)

#### `public virtual bool SDKCompliancy(int exeVersion, int exeServiceRelease)`

Test whether this object is compliant with a given Rhino version.

**Parameters:**
- `exeVersion` (System.Int32) — Rhino major release (4, 5, ...).
- `exeServiceRelease` (System.Int32) — Rhino minor (service) release.

**Returns:** `Boolean` — True if this object is compliant with the given Rhino release.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_SDKCompliancy.htm)

#### `public void SetIconOverride(Bitmap customIcon)`

Set a new custom icon override for this object.

**Parameters:**
- `customIcon` (System.Drawing.Bitmap) — Icon override. Should be a 24x24 image.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetIconOverride.htm)

#### `public void SetPersistentData(GH_Structure<T> items)`

Assign a tree of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (Grasshopper.Kernel.Data.GH_Structure<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData.htm)

#### `public void SetPersistentData(IEnumerable<T> items)`

Assign a list of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_1.htm)

#### `public void SetPersistentData(params Object[] values)`

Add a collection of values to the persistent data.

**Parameters:**
- `values` (System.Object[]) — Values to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_2.htm)

#### `public void SetPersistentData(T item)`

Add a single item to the persistent data. This method will record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add more than one piece of data, you should use the appropriate overload for this method.

**Parameters:**
- `item` (T) — Item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_3.htm)

#### `public bool SetPrincipal(bool set, bool recordUndo, bool recompute)`

Set the principal parameter state.

**Parameters:**
- `set` (System.Boolean)
- `recordUndo` (System.Boolean)
- `recompute` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_SetPrincipal.htm)

#### `public void TriggerAutoSave()`

Triggers the AutoSave function on the owner document with the object_changed flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_1.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger, Guid id)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_2.htm)

#### `public void TriggerAutoSave(Guid id)`

Triggers the AutoSave function on the owner document with the object_changed flag.

**Parameters:**
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_3.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Write.htm)

#### `public virtual bool WriteFull(GH_IWriter writer)`

GH_InstanceDescription does not by default serialize all fields. Use this function to write all fields to the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer for serialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_WriteFull.htm)

### Properties
- `Access` (GH_ParamAccess, get/set) — Gets or sets the Access level for this parameter.
- `Attributes` (IGH_Attributes, get/set) — Gets or sets the attributes that are associated with this object. Only set custom attributes if you know what you are doing.
- `Category` (String, get/set) — Gets or sets the Category in which this object belongs. If HasCategory() returns false, this field has no meaning.
- `ClippingBox` (BoundingBox, get) — 
- `ComponentGuid` (Guid, get) — (Overrides GH_DocumentObject.ComponentGuid.)
- `DataMapping` (GH_DataMapping, get/set) — Gets or sets the data mapping of this Parameter.
- `DataType` (GH_ParamData, get) — Gets the type of data stored in this parameter.
- `Description` (String, get/set) — Gets or sets the description of the object. This field typically remains fixed during the lifetime of an object.
- `Exposure` (GH_Exposure, get) — (Overrides GH_DocumentObject.Exposure.)
- `HasCategory` (Boolean, get) — Gets whether or not the Category field has been set.
- `HasProxySources` (Boolean, get) — Gets a value indicating whether or not this parameter maintains proxy sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `HasSubCategory` (Boolean, get) — Gets whether or not the SubCategory field has been set.
- `Hidden` (Boolean, get/set) — 
- `Icon_24x24` (Bitmap, get) — The icon associated with this object.
- `Icon_24x24_Locked` (Bitmap, get) — The greyscale icon of this object.
- `IconCapableUI` (Boolean, get) — By default the NickName menu item supports the Icon Mode override toggle. If your UI is not capable of displaying icons, then override this property and return False.
- `IconDisplayMode` (GH_IconDisplayMode, get/set) — Gets the current display mode of the object.
- `InstanceDescription` (String, get) — Gets the description of this instance. The default description is about the amount and source of the local values.
- `InstanceGuid` (Guid, get) — Gets the ID of this runtime instance.
- `IsBakeCapable` (Boolean, get) — (Inherited from ModelContentParam<T>.)
- `IsDataProvider` (Boolean, get) — (Inherited from GH_Param<T>.)
- `IsPreviewCapable` (Boolean, get) — 
- `IsPrincipal` (GH_PrincipalState, get) — Gets whether this parameter is a principal parameter. Principal parameters are maintained by components and are not part of the IGH_Param interface.
- `Keywords` (IEnumerable<String>, get) — Gets a list of additional keywords that describe the object. Typically this list is empty but you can override this property to aid in object searches.
- `Kind` (GH_ParamKind, get) — Gets the parameter kind. The kind is evaluated lazily and cached.
- `Locked` (Boolean, get/set) — Gets or sets the enabled flag of this object. Disabled objects are ignored during solutions.
- `MutableNickName` (Boolean, get/set) — Gets or sets a value that enables Nick name changes through the menu. The default is TRUE.
- `Name` (String, get/set) — Gets or sets the name of the object. This field typically remains fixed during the lifetime of an object.
- `NickName` (String, get/set) — Gets or sets the nickname of the object. This field can be changed by the user.
- `Obsolete` (Boolean, get) — Gets whether this object is obsolete. Default implementation returns true if the class name contains the string "OBSOLETE" or if this class has been decorated with the ObsoleteAttribute. You are free to override this if you want, but I suggest adding the ObsoleteAttribute instead.
- `Optional` (Boolean, get/set) — Gets or sets whether or not this parameter is considered optional by the owner component. Empty, non-optional parameters prevent the component from being solved.
- `PersistentData` (GH_Structure<T>, get) — Gets the persistent data stored in this parameter. If you modify the persistent data, be sure to call the: OnObjectChanged(GH_ObjectEventType.PersistentData) event.
- `PersistentDataCount` (Int32, get) — Gets the number of persistent data items stored in this parameter.
- `Phase` (GH_SolutionPhase, get/set) — Gets or sets the solution phase this object is currenly in.
- `ProcessorTime` (TimeSpan, get) — (Inherited from GH_Param<T>.)
- `ProxySourceCount` (Int32, get) — Gets the number of proxy sources for this parameter. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `Recipients` (IList<IGH_Param>, get) — Gets a list of all the recipients of this parameter. I.e. a recipient has this parameter as one of the sources. The Recipient list is maintained by the parameter, do not modify it yourself.
- `Reverse` (Boolean, get/set) — Gets or sets the data reverse modifier of this parameter.
- `RuntimeMessageLevel` (GH_RuntimeMessageLevel, get) — Returns the worst case flag for the current object If the object contains at least 1 error, the result is Error.If the object contains at least 1 warning, the result is Warning.If the object contains at least 1 message, the result is Remark.If the object contains no errors, no warnings and no messages, the result is Blank.
- `Simplify` (Boolean, get/set) — Gets or sets the simplify modifier for this parameter.
- `SourceCount` (Int32, get) — Gets the number of sources for this parameter.
- `Sources` (IList<IGH_Param>, get) — Gets a list of source parameters. Do not modify this list, if you wish to add or remove sources, use dedicated functions like AddSource() and RemoveSource() instead.
- `StateTags` (GH_StateTagList, get) — Gets all the StateTags that are associated with this parameter. A state tag is a visual feedback icon that represents specific internal settings.
- `SubCategory` (String, get/set) — Gets or sets the SubCategory in which this object belongs. If HasSubCategory() returns false, this field has no meaning.
- `Type` (Type, get) — Gets the Framework Type descriptor for the stored Data.
- `TypeName` (String, get) — (Inherited from GH_PersistentReferenceParam<T>.)
- `VolatileData` (IGH_Structure, get) — (Inherited from GH_Param<T>.)
- `VolatileDataCount` (Int32, get) — (Inherited from GH_Param<T>.)
- `WireDisplay` (GH_ParamWireDisplay, get/set) — Gets or sets the wire display style for this parameter. Wire display only affects the wires connected to the parameter input.

### Events
#### `AttributesChanged` (`Grasshopper.Kernel.IGH_DocumentObject.AttributesChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.AttributesChangedEventHandler AttributesChanged`

Raised whenever the number or kind of attributes changes. This event is handled by GH_Documents who subsequently wipe their attribute caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_AttributesChanged.htm)

#### `DisplayExpired` (`Grasshopper.Kernel.IGH_DocumentObject.DisplayExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.DisplayExpiredEventHandler DisplayExpired`

Raised whenever the display (on the Canvas) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_DisplayExpired.htm)

#### `ObjectChanged` (`Grasshopper.Kernel.IGH_DocumentObject.ObjectChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.ObjectChangedEventHandler ObjectChanged`

(Inherited from GH_DocumentObject.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_ObjectChanged.htm)

#### `PingDocument` (`Grasshopper.Kernel.IGH_DocumentObject.PingDocumentEventHandler`)

**Signature:** `public event IGH_DocumentObject.PingDocumentEventHandler PingDocument`

Raised whenever an object needs to know which GH_Document it belongs to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PingDocument.htm)

#### `PreviewExpired` (`Grasshopper.Kernel.IGH_DocumentObject.PreviewExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.PreviewExpiredEventHandler PreviewExpired`

Raised whenever the display (in the Rhino viewports) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PreviewExpired.htm)

#### `SolutionExpired` (`Grasshopper.Kernel.IGH_DocumentObject.SolutionExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.SolutionExpiredEventHandler SolutionExpired`

Raised whenever the solution of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_SolutionExpired.htm)

## Param_ModelLayer (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.Params.Param_ModelLayer.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_Params_Param_ModelLayer.htm)

### Constructors
- `public Param_ModelLayer()` — Initializes a new instance of the Param_ModelLayer class

### Methods
#### `public virtual void AddRuntimeMessage(GH_RuntimeMessageLevel level, string text)`

Add a new message to this object. Valid message type flags are Warning and Error. If the Message string is empty or zero-length no message is added.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Type of message. Only Warnings and Errors are recorded.
- `text` (System.String) — Content of message.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AddRuntimeMessage.htm)

#### `public virtual void AddSource(IGH_Param source)`

Append a new Source parameter to the end of the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource.htm)

#### `public virtual void AddSource(IGH_Param source, int index)`

Insert a new Source parameter into the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.
- `index` (System.Int32) — Insertion index of source.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource_1.htm)

#### `public bool AddVolatileData(GH_Path path, int index, Object data)`

Inserts an item of volatile data into the data structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The branch path of the data. If the path doesn't exist yet, it will be created.
- `index` (System.Int32) — The item index of the data. If the path doesn't contain the index yet, it will be enlarged to encompass the index.
- `data` (System.Object) — The data to store.

**Returns:** `Boolean` — True on success, False on failure. If the data cannot be converted, the topology will remain unmolested.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData.htm)

#### `public bool AddVolatileData(GH_Path path, int index, T data)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `index` (System.Int32)
- `data` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, IEnumerable list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.IEnumerable)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, List<T> list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.Generic.List<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList.htm)

#### `public bool AddVolatileDataTree(GH_Structure<T> tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.GH_Structure<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree.htm)

#### `public bool AddVolatileDataTree(IGH_Structure tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.IGH_Structure)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree_1.htm)

#### `public virtual void AddedToDocument(GH_Document document)`

This method will be called when an object is added to a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_AddedToDocument.htm)

#### `public override void AppendAdditionalMenuItems(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AppendAdditionalMenuItems.htm)

#### `public override bool AppendMenuItems(ToolStripDropDown menu)`

This function is called when a context menu is about to be displayed. Override it to set custom items. GH_ActiveObject will already populate the menu with default items, if you merely wish to insert object-specific menu item, consider overriding AppendAdditionalMenuItems instead.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown) — Menu object to populate.

**Returns:** `Boolean` — If true, the menu will be displayed, if false the menu will be supressed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AppendMenuItems.htm)

#### `public void BakeGeometry(RhinoDoc doc, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry_1.htm)

#### `public void BakeGeometry(RhinoDoc doc, ObjectAttributes att, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry.htm)

#### `public override void ClearData()`

(Inherited from GH_PersistentReferenceParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentReferenceParam_1_ClearData.htm)

#### `public virtual void ClearProxySources()`

Remove all proxy sources without attempting to relink them.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ClearProxySources.htm)

#### `public virtual void ClearRuntimeMessages()`

Destroy all warning and error lists

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ClearRuntimeMessages.htm)

#### `public override sealed void CollectData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CollectData.htm)

#### `public override sealed void ComputeData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ComputeData.htm)

#### `public void CopyFrom(IGH_InstanceDescription other)`

Copy all fields (except the instance ID) from another instance description.

**Parameters:**
- `other` (Grasshopper.Kernel.IGH_InstanceDescription) — Object to mimic.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_CopyFrom.htm)

#### `public override void CreateAttributes()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateAttributes.htm)

#### `public virtual void CreateProxySources()`

Convert all proper source parameters into proxy sources.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateProxySources.htm)

#### `public override bool DependsOn(IGH_ActiveObject potential_source)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `potential_source` (Grasshopper.Kernel.IGH_ActiveObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_DependsOn.htm)

#### `public virtual void DocumentContextChanged(GH_Document document, GH_DocumentContext context)`

This method will be called when the document that owns this object moves into a different context.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that owns this object.
- `context` (Grasshopper.Kernel.GH_DocumentContext) — The reason for this event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_DocumentContextChanged.htm)

#### `public virtual void ExpirePreview(bool redraw)`

Call this function when you suspect that the preview has expired for this object. This will cause the display cache to be eradicated.

**Parameters:**
- `redraw` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_ExpirePreview.htm)

#### `public override void ExpireSolution(bool recompute)`

Informs the document that owns this object that the solution has expired. The current object will be set to BLANK as a result. This method is recursive, it will also expire any and all objects which depend on this object. If you want a less destructive expiration, consider using ClearData(). If this object is already Blank, you should consider not expiring it.

**Parameters:**
- `recompute` (System.Boolean) — If True, the document will be instructed to solve the new state.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ExpireSolution.htm)

#### `public void ExpireSolutionTopLevel(bool recompute)`

Invoke the Expiresolution(bool) method on the toplevel object.

**Parameters:**
- `recompute` (System.Boolean) — If true, a new computation will start right away.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ExpireSolutionTopLevel.htm)

#### `public override void IsolateObject()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_IsolateObject.htm)

#### `public virtual void MovedBetweenDocuments(GH_Document oldDocument, GH_Document newDocument)`

This method will be called when an object is moved from one document to another. Override this method if you want to handle such events.

**Parameters:**
- `oldDocument` (Grasshopper.Kernel.GH_Document) — Document that used to own this object.
- `newDocument` (Grasshopper.Kernel.GH_Document) — Document that now owns ths object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_MovedBetweenDocuments.htm)

#### `public void NewInstanceGuid()`

Generate a new random instance GUID

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid UUID)`

Set the instance ID to be a specific GUID. This is very dangerous, only use this function if you're 6"4' and your first name is David.

**Parameters:**
- `UUID` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid_1.htm)

#### `public void OnAttributesChanged()`

Raises the AttributesChanged event on the toplevel object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnAttributesChanged.htm)

#### `public void OnDisplayExpired(bool redraw)`

Raises the DisplayExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the canvas will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnDisplayExpired.htm)

#### `public void OnObjectChanged(GH_ObjectChangedEventArgs e)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `e` (Grasshopper.Kernel.GH_ObjectChangedEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_1.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_2.htm)

#### `public void OnObjectChanged(string customEvent)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_3.htm)

#### `public void OnObjectChanged(string customEvent, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_4.htm)

#### `public GH_Document OnPingDocument()`

Raise the PingDocument Event on the toplevel object and try to find the document which owns this object.

**Returns:** `GH_Document` — The document which owns this object if successful, or nothing if no document owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPingDocument.htm)

#### `public void OnPreviewExpired(bool redraw)`

Raises the PreviewExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the viewports will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPreviewExpired.htm)

#### `public void OnSolutionExpired(bool recompute)`

Raises the SolutionExpired event on the toplevel object. You probably want to call ExpireSolution() instead of this method directly.

**Parameters:**
- `recompute` (System.Boolean) — If True, the solution will be immediately recalculated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnSolutionExpired.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Read.htm)

#### `public virtual bool ReadFull(GH_IReader reader)`

GH_InstanceDescription does not by default serialize all fields. Use this function to read all fields from the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Writer for deserialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_ReadFull.htm)

#### `public void RecordUndoEvent(GH_UndoRecord record)`

Record an entire undo record.

**Parameters:**
- `record` (Grasshopper.Kernel.Undo.GH_UndoRecord) — Record to push.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent.htm)

#### `public Guid RecordUndoEvent(string undoName)`

Record a generic object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_1.htm)

#### `public Guid RecordUndoEvent(string undoName, IGH_UndoAction action)`

Record a specific object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.
- `action` (Grasshopper.Kernel.Undo.IGH_UndoAction) — Undo action to record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_2.htm)

#### `public override void RegisterRemoteIDs(GH_GuidTable table)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `table` (GH_GuidTable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_RegisterRemoteIDs.htm)

#### `public virtual bool RelinkProxySources(GH_Document document)`

Attempt to replace all proxy sources with real sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — The document from which to harvest the real source parameters.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RelinkProxySources.htm)

#### `public virtual void RemoveAllSources()`

Remove all sources from this parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveAllSources.htm)

#### `public virtual void RemoveEffects()`

Remove all post-process effects. Note to implementors, you must call the base method if you override this function.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveEffects.htm)

#### `public virtual void RemoveSource(Guid source_id)`

Remove the specified source from this parameter.

**Parameters:**
- `source_id` (System.Guid) — InstanceID of source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource_1.htm)

#### `public virtual void RemoveSource(IGH_Param source)`

Remove the specified source from this parameter.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource.htm)

#### `public virtual void RemovedFromDocument(GH_Document document)`

This method will be called when an object is removed from a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now no longer owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RemovedFromDocument.htm)

#### `public virtual void ReplaceSource(Guid old_source_id, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source_id` (System.Guid) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource_1.htm)

#### `public virtual void ReplaceSource(IGH_Param old_source, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source` (Grasshopper.Kernel.IGH_Param) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource.htm)

#### `public virtual IList<string> RuntimeMessages(GH_RuntimeMessageLevel level)`

Gets the list of cached runtime messages that were recorded during solver-time processes.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Runtime message type to retrieve.

**Returns:** `IList<String>` — A list of runtime message strings.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_RuntimeMessages.htm)

#### `public virtual bool SDKCompliancy(int exeVersion, int exeServiceRelease)`

Test whether this object is compliant with a given Rhino version.

**Parameters:**
- `exeVersion` (System.Int32) — Rhino major release (4, 5, ...).
- `exeServiceRelease` (System.Int32) — Rhino minor (service) release.

**Returns:** `Boolean` — True if this object is compliant with the given Rhino release.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_SDKCompliancy.htm)

#### `public void SetIconOverride(Bitmap customIcon)`

Set a new custom icon override for this object.

**Parameters:**
- `customIcon` (System.Drawing.Bitmap) — Icon override. Should be a 24x24 image.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetIconOverride.htm)

#### `public void SetPersistentData(GH_Structure<T> items)`

Assign a tree of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (Grasshopper.Kernel.Data.GH_Structure<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData.htm)

#### `public void SetPersistentData(IEnumerable<T> items)`

Assign a list of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_1.htm)

#### `public void SetPersistentData(params Object[] values)`

Add a collection of values to the persistent data.

**Parameters:**
- `values` (System.Object[]) — Values to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_2.htm)

#### `public void SetPersistentData(T item)`

Add a single item to the persistent data. This method will record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add more than one piece of data, you should use the appropriate overload for this method.

**Parameters:**
- `item` (T) — Item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_3.htm)

#### `public bool SetPrincipal(bool set, bool recordUndo, bool recompute)`

Set the principal parameter state.

**Parameters:**
- `set` (System.Boolean)
- `recordUndo` (System.Boolean)
- `recompute` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_SetPrincipal.htm)

#### `public void TriggerAutoSave()`

Triggers the AutoSave function on the owner document with the object_changed flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_1.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger, Guid id)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_2.htm)

#### `public void TriggerAutoSave(Guid id)`

Triggers the AutoSave function on the owner document with the object_changed flag.

**Parameters:**
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_3.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Write.htm)

#### `public virtual bool WriteFull(GH_IWriter writer)`

GH_InstanceDescription does not by default serialize all fields. Use this function to write all fields to the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer for serialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_WriteFull.htm)

### Properties
- `Access` (GH_ParamAccess, get/set) — Gets or sets the Access level for this parameter.
- `Attributes` (IGH_Attributes, get/set) — Gets or sets the attributes that are associated with this object. Only set custom attributes if you know what you are doing.
- `Category` (String, get/set) — Gets or sets the Category in which this object belongs. If HasCategory() returns false, this field has no meaning.
- `ComponentGuid` (Guid, get) — (Overrides GH_DocumentObject.ComponentGuid.)
- `DataMapping` (GH_DataMapping, get/set) — Gets or sets the data mapping of this Parameter.
- `DataType` (GH_ParamData, get) — Gets the type of data stored in this parameter.
- `Description` (String, get/set) — Gets or sets the description of the object. This field typically remains fixed during the lifetime of an object.
- `Exposure` (GH_Exposure, get) — (Overrides GH_DocumentObject.Exposure.)
- `HasCategory` (Boolean, get) — Gets whether or not the Category field has been set.
- `HasProxySources` (Boolean, get) — Gets a value indicating whether or not this parameter maintains proxy sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `HasSubCategory` (Boolean, get) — Gets whether or not the SubCategory field has been set.
- `Icon_24x24` (Bitmap, get) — The icon associated with this object.
- `Icon_24x24_Locked` (Bitmap, get) — The greyscale icon of this object.
- `IconCapableUI` (Boolean, get) — By default the NickName menu item supports the Icon Mode override toggle. If your UI is not capable of displaying icons, then override this property and return False.
- `IconDisplayMode` (GH_IconDisplayMode, get/set) — Gets the current display mode of the object.
- `InstanceDescription` (String, get) — Gets the description of this instance. The default description is about the amount and source of the local values.
- `InstanceGuid` (Guid, get) — Gets the ID of this runtime instance.
- `IsBakeCapable` (Boolean, get) — (Inherited from ModelContentParam<T>.)
- `IsDataProvider` (Boolean, get) — (Inherited from GH_Param<T>.)
- `IsPrincipal` (GH_PrincipalState, get) — Gets whether this parameter is a principal parameter. Principal parameters are maintained by components and are not part of the IGH_Param interface.
- `Keywords` (IEnumerable<String>, get) — Gets a list of additional keywords that describe the object. Typically this list is empty but you can override this property to aid in object searches.
- `Kind` (GH_ParamKind, get) — Gets the parameter kind. The kind is evaluated lazily and cached.
- `Locked` (Boolean, get/set) — Gets or sets the enabled flag of this object. Disabled objects are ignored during solutions.
- `MutableNickName` (Boolean, get/set) — Gets or sets a value that enables Nick name changes through the menu. The default is TRUE.
- `Name` (String, get/set) — Gets or sets the name of the object. This field typically remains fixed during the lifetime of an object.
- `NickName` (String, get/set) — Gets or sets the nickname of the object. This field can be changed by the user.
- `Obsolete` (Boolean, get) — Gets whether this object is obsolete. Default implementation returns true if the class name contains the string "OBSOLETE" or if this class has been decorated with the ObsoleteAttribute. You are free to override this if you want, but I suggest adding the ObsoleteAttribute instead.
- `Optional` (Boolean, get/set) — Gets or sets whether or not this parameter is considered optional by the owner component. Empty, non-optional parameters prevent the component from being solved.
- `PersistentData` (GH_Structure<T>, get) — Gets the persistent data stored in this parameter. If you modify the persistent data, be sure to call the: OnObjectChanged(GH_ObjectEventType.PersistentData) event.
- `PersistentDataCount` (Int32, get) — Gets the number of persistent data items stored in this parameter.
- `Phase` (GH_SolutionPhase, get/set) — Gets or sets the solution phase this object is currenly in.
- `ProcessorTime` (TimeSpan, get) — (Inherited from GH_Param<T>.)
- `ProxySourceCount` (Int32, get) — Gets the number of proxy sources for this parameter. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `Recipients` (IList<IGH_Param>, get) — Gets a list of all the recipients of this parameter. I.e. a recipient has this parameter as one of the sources. The Recipient list is maintained by the parameter, do not modify it yourself.
- `Reverse` (Boolean, get/set) — Gets or sets the data reverse modifier of this parameter.
- `RuntimeMessageLevel` (GH_RuntimeMessageLevel, get) — Returns the worst case flag for the current object If the object contains at least 1 error, the result is Error.If the object contains at least 1 warning, the result is Warning.If the object contains at least 1 message, the result is Remark.If the object contains no errors, no warnings and no messages, the result is Blank.
- `Simplify` (Boolean, get/set) — Gets or sets the simplify modifier for this parameter.
- `SourceCount` (Int32, get) — Gets the number of sources for this parameter.
- `Sources` (IList<IGH_Param>, get) — Gets a list of source parameters. Do not modify this list, if you wish to add or remove sources, use dedicated functions like AddSource() and RemoveSource() instead.
- `StateTags` (GH_StateTagList, get) — Gets all the StateTags that are associated with this parameter. A state tag is a visual feedback icon that represents specific internal settings.
- `SubCategory` (String, get/set) — Gets or sets the SubCategory in which this object belongs. If HasSubCategory() returns false, this field has no meaning.
- `Type` (Type, get) — Gets the Framework Type descriptor for the stored Data.
- `TypeName` (String, get) — (Inherited from GH_PersistentReferenceParam<T>.)
- `VolatileData` (IGH_Structure, get) — (Inherited from GH_Param<T>.)
- `VolatileDataCount` (Int32, get) — (Inherited from GH_Param<T>.)
- `WireDisplay` (GH_ParamWireDisplay, get/set) — Gets or sets the wire display style for this parameter. Wire display only affects the wires connected to the parameter input.

### Events
#### `AttributesChanged` (`Grasshopper.Kernel.IGH_DocumentObject.AttributesChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.AttributesChangedEventHandler AttributesChanged`

Raised whenever the number or kind of attributes changes. This event is handled by GH_Documents who subsequently wipe their attribute caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_AttributesChanged.htm)

#### `DisplayExpired` (`Grasshopper.Kernel.IGH_DocumentObject.DisplayExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.DisplayExpiredEventHandler DisplayExpired`

Raised whenever the display (on the Canvas) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_DisplayExpired.htm)

#### `ObjectChanged` (`Grasshopper.Kernel.IGH_DocumentObject.ObjectChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.ObjectChangedEventHandler ObjectChanged`

(Inherited from GH_DocumentObject.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_ObjectChanged.htm)

#### `PingDocument` (`Grasshopper.Kernel.IGH_DocumentObject.PingDocumentEventHandler`)

**Signature:** `public event IGH_DocumentObject.PingDocumentEventHandler PingDocument`

Raised whenever an object needs to know which GH_Document it belongs to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PingDocument.htm)

#### `PreviewExpired` (`Grasshopper.Kernel.IGH_DocumentObject.PreviewExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.PreviewExpiredEventHandler PreviewExpired`

Raised whenever the display (in the Rhino viewports) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PreviewExpired.htm)

#### `SolutionExpired` (`Grasshopper.Kernel.IGH_DocumentObject.SolutionExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.SolutionExpiredEventHandler SolutionExpired`

Raised whenever the solution of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_SolutionExpired.htm)

## Param_ModelObject (class)

(No description provided in vendor docs for Grasshopper.Rhinoceros.Model.Params.Param_ModelObject.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Rhinoceros_Model_Params_Param_ModelObject.htm)

### Constructors
- `public Param_ModelObject()` — Initializes a new instance of the Param_ModelObject class

### Methods
#### `public virtual void AddRuntimeMessage(GH_RuntimeMessageLevel level, string text)`

Add a new message to this object. Valid message type flags are Warning and Error. If the Message string is empty or zero-length no message is added.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Type of message. Only Warnings and Errors are recorded.
- `text` (System.String) — Content of message.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AddRuntimeMessage.htm)

#### `public virtual void AddSource(IGH_Param source)`

Append a new Source parameter to the end of the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource.htm)

#### `public virtual void AddSource(IGH_Param source, int index)`

Insert a new Source parameter into the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.
- `index` (System.Int32) — Insertion index of source.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource_1.htm)

#### `public bool AddVolatileData(GH_Path path, int index, Object data)`

Inserts an item of volatile data into the data structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The branch path of the data. If the path doesn't exist yet, it will be created.
- `index` (System.Int32) — The item index of the data. If the path doesn't contain the index yet, it will be enlarged to encompass the index.
- `data` (System.Object) — The data to store.

**Returns:** `Boolean` — True on success, False on failure. If the data cannot be converted, the topology will remain unmolested.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData.htm)

#### `public bool AddVolatileData(GH_Path path, int index, T data)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `index` (System.Int32)
- `data` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, IEnumerable list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.IEnumerable)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, List<T> list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.Generic.List<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList.htm)

#### `public bool AddVolatileDataTree(GH_Structure<T> tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.GH_Structure<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree.htm)

#### `public bool AddVolatileDataTree(IGH_Structure tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.IGH_Structure)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree_1.htm)

#### `public virtual void AddedToDocument(GH_Document document)`

This method will be called when an object is added to a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_AddedToDocument.htm)

#### `public override void AppendAdditionalMenuItems(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AppendAdditionalMenuItems.htm)

#### `public override bool AppendMenuItems(ToolStripDropDown menu)`

This function is called when a context menu is about to be displayed. Override it to set custom items. GH_ActiveObject will already populate the menu with default items, if you merely wish to insert object-specific menu item, consider overriding AppendAdditionalMenuItems instead.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown) — Menu object to populate.

**Returns:** `Boolean` — If true, the menu will be displayed, if false the menu will be supressed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AppendMenuItems.htm)

#### `public void BakeGeometry(RhinoDoc doc, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry_1.htm)

#### `public void BakeGeometry(RhinoDoc doc, ObjectAttributes att, List<Guid> guids)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `doc` (RhinoDoc)
- `att` (ObjectAttributes)
- `guids` (System.Collections.Generic.List<Guid>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_BakeGeometry.htm)

#### `public override void ClearData()`

(Inherited from GH_PersistentReferenceParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentReferenceParam_1_ClearData.htm)

#### `public virtual void ClearProxySources()`

Remove all proxy sources without attempting to relink them.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ClearProxySources.htm)

#### `public virtual void ClearRuntimeMessages()`

Destroy all warning and error lists

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ClearRuntimeMessages.htm)

#### `public override sealed void CollectData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CollectData.htm)

#### `public override sealed void ComputeData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ComputeData.htm)

#### `public void CopyFrom(IGH_InstanceDescription other)`

Copy all fields (except the instance ID) from another instance description.

**Parameters:**
- `other` (Grasshopper.Kernel.IGH_InstanceDescription) — Object to mimic.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_CopyFrom.htm)

#### `public override void CreateAttributes()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateAttributes.htm)

#### `public virtual void CreateProxySources()`

Convert all proper source parameters into proxy sources.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateProxySources.htm)

#### `public override bool DependsOn(IGH_ActiveObject potential_source)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `potential_source` (Grasshopper.Kernel.IGH_ActiveObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_DependsOn.htm)

#### `public virtual void DocumentContextChanged(GH_Document document, GH_DocumentContext context)`

This method will be called when the document that owns this object moves into a different context.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that owns this object.
- `context` (Grasshopper.Kernel.GH_DocumentContext) — The reason for this event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_DocumentContextChanged.htm)

#### `public void DrawViewportMeshes(IGH_PreviewArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.IGH_PreviewArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_Params_Param_ModelObject_DrawViewportMeshes.htm)

#### `public void DrawViewportWires(IGH_PreviewArgs args)`



**Parameters:**
- `args` (Grasshopper.Kernel.IGH_PreviewArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Model_Params_Param_ModelObject_DrawViewportWires.htm)

#### `public virtual void ExpirePreview(bool redraw)`

Call this function when you suspect that the preview has expired for this object. This will cause the display cache to be eradicated.

**Parameters:**
- `redraw` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_ExpirePreview.htm)

#### `public override void ExpireSolution(bool recompute)`

Informs the document that owns this object that the solution has expired. The current object will be set to BLANK as a result. This method is recursive, it will also expire any and all objects which depend on this object. If you want a less destructive expiration, consider using ClearData(). If this object is already Blank, you should consider not expiring it.

**Parameters:**
- `recompute` (System.Boolean) — If True, the document will be instructed to solve the new state.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ExpireSolution.htm)

#### `public void ExpireSolutionTopLevel(bool recompute)`

Invoke the Expiresolution(bool) method on the toplevel object.

**Parameters:**
- `recompute` (System.Boolean) — If true, a new computation will start right away.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ExpireSolutionTopLevel.htm)

#### `public override void IsolateObject()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_IsolateObject.htm)

#### `public virtual void MovedBetweenDocuments(GH_Document oldDocument, GH_Document newDocument)`

This method will be called when an object is moved from one document to another. Override this method if you want to handle such events.

**Parameters:**
- `oldDocument` (Grasshopper.Kernel.GH_Document) — Document that used to own this object.
- `newDocument` (Grasshopper.Kernel.GH_Document) — Document that now owns ths object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_MovedBetweenDocuments.htm)

#### `public void NewInstanceGuid()`

Generate a new random instance GUID

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid UUID)`

Set the instance ID to be a specific GUID. This is very dangerous, only use this function if you're 6"4' and your first name is David.

**Parameters:**
- `UUID` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid_1.htm)

#### `public void OnAttributesChanged()`

Raises the AttributesChanged event on the toplevel object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnAttributesChanged.htm)

#### `public void OnDisplayExpired(bool redraw)`

Raises the DisplayExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the canvas will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnDisplayExpired.htm)

#### `public void OnObjectChanged(GH_ObjectChangedEventArgs e)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `e` (Grasshopper.Kernel.GH_ObjectChangedEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_1.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_2.htm)

#### `public void OnObjectChanged(string customEvent)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_3.htm)

#### `public void OnObjectChanged(string customEvent, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_4.htm)

#### `public GH_Document OnPingDocument()`

Raise the PingDocument Event on the toplevel object and try to find the document which owns this object.

**Returns:** `GH_Document` — The document which owns this object if successful, or nothing if no document owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPingDocument.htm)

#### `public void OnPreviewExpired(bool redraw)`

Raises the PreviewExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the viewports will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPreviewExpired.htm)

#### `public void OnSolutionExpired(bool recompute)`

Raises the SolutionExpired event on the toplevel object. You probably want to call ExpireSolution() instead of this method directly.

**Parameters:**
- `recompute` (System.Boolean) — If True, the solution will be immediately recalculated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnSolutionExpired.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Read.htm)

#### `public virtual bool ReadFull(GH_IReader reader)`

GH_InstanceDescription does not by default serialize all fields. Use this function to read all fields from the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Writer for deserialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_ReadFull.htm)

#### `public void RecordUndoEvent(GH_UndoRecord record)`

Record an entire undo record.

**Parameters:**
- `record` (Grasshopper.Kernel.Undo.GH_UndoRecord) — Record to push.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent.htm)

#### `public Guid RecordUndoEvent(string undoName)`

Record a generic object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_1.htm)

#### `public Guid RecordUndoEvent(string undoName, IGH_UndoAction action)`

Record a specific object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.
- `action` (Grasshopper.Kernel.Undo.IGH_UndoAction) — Undo action to record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_2.htm)

#### `public override void RegisterRemoteIDs(GH_GuidTable table)`

(Inherited from ModelContentParam<T>.)

**Parameters:**
- `table` (GH_GuidTable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Rhinoceros_Params_ModelContentParam_1_RegisterRemoteIDs.htm)

#### `public virtual bool RelinkProxySources(GH_Document document)`

Attempt to replace all proxy sources with real sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — The document from which to harvest the real source parameters.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RelinkProxySources.htm)

#### `public virtual void RemoveAllSources()`

Remove all sources from this parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveAllSources.htm)

#### `public virtual void RemoveEffects()`

Remove all post-process effects. Note to implementors, you must call the base method if you override this function.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveEffects.htm)

#### `public virtual void RemoveSource(Guid source_id)`

Remove the specified source from this parameter.

**Parameters:**
- `source_id` (System.Guid) — InstanceID of source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource_1.htm)

#### `public virtual void RemoveSource(IGH_Param source)`

Remove the specified source from this parameter.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource.htm)

#### `public virtual void RemovedFromDocument(GH_Document document)`

This method will be called when an object is removed from a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now no longer owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RemovedFromDocument.htm)

#### `public virtual void ReplaceSource(Guid old_source_id, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source_id` (System.Guid) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource_1.htm)

#### `public virtual void ReplaceSource(IGH_Param old_source, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source` (Grasshopper.Kernel.IGH_Param) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource.htm)

#### `public virtual IList<string> RuntimeMessages(GH_RuntimeMessageLevel level)`

Gets the list of cached runtime messages that were recorded during solver-time processes.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Runtime message type to retrieve.

**Returns:** `IList<String>` — A list of runtime message strings.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_RuntimeMessages.htm)

#### `public virtual bool SDKCompliancy(int exeVersion, int exeServiceRelease)`

Test whether this object is compliant with a given Rhino version.

**Parameters:**
- `exeVersion` (System.Int32) — Rhino major release (4, 5, ...).
- `exeServiceRelease` (System.Int32) — Rhino minor (service) release.

**Returns:** `Boolean` — True if this object is compliant with the given Rhino release.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_SDKCompliancy.htm)

#### `public void SetIconOverride(Bitmap customIcon)`

Set a new custom icon override for this object.

**Parameters:**
- `customIcon` (System.Drawing.Bitmap) — Icon override. Should be a 24x24 image.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetIconOverride.htm)

#### `public void SetPersistentData(GH_Structure<T> items)`

Assign a tree of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (Grasshopper.Kernel.Data.GH_Structure<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData.htm)

#### `public void SetPersistentData(IEnumerable<T> items)`

Assign a list of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_1.htm)

#### `public void SetPersistentData(params Object[] values)`

Add a collection of values to the persistent data.

**Parameters:**
- `values` (System.Object[]) — Values to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_2.htm)

#### `public void SetPersistentData(T item)`

Add a single item to the persistent data. This method will record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add more than one piece of data, you should use the appropriate overload for this method.

**Parameters:**
- `item` (T) — Item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_3.htm)

#### `public bool SetPrincipal(bool set, bool recordUndo, bool recompute)`

Set the principal parameter state.

**Parameters:**
- `set` (System.Boolean)
- `recordUndo` (System.Boolean)
- `recompute` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_SetPrincipal.htm)

#### `public void TriggerAutoSave()`

Triggers the AutoSave function on the owner document with the object_changed flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_1.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger, Guid id)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_2.htm)

#### `public void TriggerAutoSave(Guid id)`

Triggers the AutoSave function on the owner document with the object_changed flag.

**Parameters:**
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_3.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Write.htm)

#### `public virtual bool WriteFull(GH_IWriter writer)`

GH_InstanceDescription does not by default serialize all fields. Use this function to write all fields to the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer for serialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_WriteFull.htm)

### Properties
- `Access` (GH_ParamAccess, get/set) — Gets or sets the Access level for this parameter.
- `Attributes` (IGH_Attributes, get/set) — Gets or sets the attributes that are associated with this object. Only set custom attributes if you know what you are doing.
- `Category` (String, get/set) — Gets or sets the Category in which this object belongs. If HasCategory() returns false, this field has no meaning.
- `ClippingBox` (BoundingBox, get) — 
- `ComponentGuid` (Guid, get) — (Overrides GH_DocumentObject.ComponentGuid.)
- `DataMapping` (GH_DataMapping, get/set) — Gets or sets the data mapping of this Parameter.
- `DataType` (GH_ParamData, get) — Gets the type of data stored in this parameter.
- `Description` (String, get/set) — Gets or sets the description of the object. This field typically remains fixed during the lifetime of an object.
- `Exposure` (GH_Exposure, get) — (Overrides GH_DocumentObject.Exposure.)
- `HasCategory` (Boolean, get) — Gets whether or not the Category field has been set.
- `HasProxySources` (Boolean, get) — Gets a value indicating whether or not this parameter maintains proxy sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `HasSubCategory` (Boolean, get) — Gets whether or not the SubCategory field has been set.
- `Hidden` (Boolean, get/set) — 
- `Icon_24x24` (Bitmap, get) — The icon associated with this object.
- `Icon_24x24_Locked` (Bitmap, get) — The greyscale icon of this object.
- `IconCapableUI` (Boolean, get) — By default the NickName menu item supports the Icon Mode override toggle. If your UI is not capable of displaying icons, then override this property and return False.
- `IconDisplayMode` (GH_IconDisplayMode, get/set) — Gets the current display mode of the object.
- `InstanceDescription` (String, get) — Gets the description of this instance. The default description is about the amount and source of the local values.
- `InstanceGuid` (Guid, get) — Gets the ID of this runtime instance.
- `IsBakeCapable` (Boolean, get) — (Inherited from ModelContentParam<T>.)
- `IsDataProvider` (Boolean, get) — (Inherited from GH_Param<T>.)
- `IsPreviewCapable` (Boolean, get) — 
- `IsPrincipal` (GH_PrincipalState, get) — Gets whether this parameter is a principal parameter. Principal parameters are maintained by components and are not part of the IGH_Param interface.
- `Keywords` (IEnumerable<String>, get) — Gets a list of additional keywords that describe the object. Typically this list is empty but you can override this property to aid in object searches.
- `Kind` (GH_ParamKind, get) — Gets the parameter kind. The kind is evaluated lazily and cached.
- `Locked` (Boolean, get/set) — Gets or sets the enabled flag of this object. Disabled objects are ignored during solutions.
- `MutableNickName` (Boolean, get/set) — Gets or sets a value that enables Nick name changes through the menu. The default is TRUE.
- `Name` (String, get/set) — Gets or sets the name of the object. This field typically remains fixed during the lifetime of an object.
- `NickName` (String, get/set) — Gets or sets the nickname of the object. This field can be changed by the user.
- `Obsolete` (Boolean, get) — Gets whether this object is obsolete. Default implementation returns true if the class name contains the string "OBSOLETE" or if this class has been decorated with the ObsoleteAttribute. You are free to override this if you want, but I suggest adding the ObsoleteAttribute instead.
- `Optional` (Boolean, get/set) — Gets or sets whether or not this parameter is considered optional by the owner component. Empty, non-optional parameters prevent the component from being solved.
- `PersistentData` (GH_Structure<T>, get) — Gets the persistent data stored in this parameter. If you modify the persistent data, be sure to call the: OnObjectChanged(GH_ObjectEventType.PersistentData) event.
- `PersistentDataCount` (Int32, get) — Gets the number of persistent data items stored in this parameter.
- `Phase` (GH_SolutionPhase, get/set) — Gets or sets the solution phase this object is currenly in.
- `ProcessorTime` (TimeSpan, get) — (Inherited from GH_Param<T>.)
- `ProxySourceCount` (Int32, get) — Gets the number of proxy sources for this parameter. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.
- `Recipients` (IList<IGH_Param>, get) — Gets a list of all the recipients of this parameter. I.e. a recipient has this parameter as one of the sources. The Recipient list is maintained by the parameter, do not modify it yourself.
- `Reverse` (Boolean, get/set) — Gets or sets the data reverse modifier of this parameter.
- `RuntimeMessageLevel` (GH_RuntimeMessageLevel, get) — Returns the worst case flag for the current object If the object contains at least 1 error, the result is Error.If the object contains at least 1 warning, the result is Warning.If the object contains at least 1 message, the result is Remark.If the object contains no errors, no warnings and no messages, the result is Blank.
- `Simplify` (Boolean, get/set) — Gets or sets the simplify modifier for this parameter.
- `SourceCount` (Int32, get) — Gets the number of sources for this parameter.
- `Sources` (IList<IGH_Param>, get) — Gets a list of source parameters. Do not modify this list, if you wish to add or remove sources, use dedicated functions like AddSource() and RemoveSource() instead.
- `StateTags` (GH_StateTagList, get) — Gets all the StateTags that are associated with this parameter. A state tag is a visual feedback icon that represents specific internal settings.
- `SubCategory` (String, get/set) — Gets or sets the SubCategory in which this object belongs. If HasSubCategory() returns false, this field has no meaning.
- `Type` (Type, get) — Gets the Framework Type descriptor for the stored Data.
- `TypeName` (String, get) — (Inherited from GH_PersistentReferenceParam<T>.)
- `VolatileData` (IGH_Structure, get) — (Inherited from GH_Param<T>.)
- `VolatileDataCount` (Int32, get) — (Inherited from GH_Param<T>.)
- `WireDisplay` (GH_ParamWireDisplay, get/set) — Gets or sets the wire display style for this parameter. Wire display only affects the wires connected to the parameter input.

### Events
#### `AttributesChanged` (`Grasshopper.Kernel.IGH_DocumentObject.AttributesChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.AttributesChangedEventHandler AttributesChanged`

Raised whenever the number or kind of attributes changes. This event is handled by GH_Documents who subsequently wipe their attribute caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_AttributesChanged.htm)

#### `DisplayExpired` (`Grasshopper.Kernel.IGH_DocumentObject.DisplayExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.DisplayExpiredEventHandler DisplayExpired`

Raised whenever the display (on the Canvas) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_DisplayExpired.htm)

#### `ObjectChanged` (`Grasshopper.Kernel.IGH_DocumentObject.ObjectChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.ObjectChangedEventHandler ObjectChanged`

(Inherited from GH_DocumentObject.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_ObjectChanged.htm)

#### `PingDocument` (`Grasshopper.Kernel.IGH_DocumentObject.PingDocumentEventHandler`)

**Signature:** `public event IGH_DocumentObject.PingDocumentEventHandler PingDocument`

Raised whenever an object needs to know which GH_Document it belongs to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PingDocument.htm)

#### `PreviewExpired` (`Grasshopper.Kernel.IGH_DocumentObject.PreviewExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.PreviewExpiredEventHandler PreviewExpired`

Raised whenever the display (in the Rhino viewports) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PreviewExpired.htm)

#### `SolutionExpired` (`Grasshopper.Kernel.IGH_DocumentObject.SolutionExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.SolutionExpiredEventHandler SolutionExpired`

Raised whenever the solution of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_SolutionExpired.htm)

