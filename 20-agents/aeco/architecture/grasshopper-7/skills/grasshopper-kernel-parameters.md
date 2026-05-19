---
name: grasshopper-grasshopper-kernel-parameters
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Parameters namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IGH_TypeHint, Param_MeshParameters.
---

# Grasshopper.Kernel.Parameters

Auto-generated from vendor docs for grasshopper 7.0. 2 types in this namespace.

## IGH_TypeHint (interface)

Represents conversion logic for a certain data type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Parameters_IGH_TypeHint.htm)

### Methods
#### `bool Cast(Object data, out Object target)`

Apply conversion logic to a specific object. This function should not throw exceptions.

**Parameters:**
- `data` (System.Object) — Data to convert.
- `target` (System.Object) — Output instance.

**Returns:** `Boolean` — True on success, false on failure. If False is returned, target may be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Parameters_IGH_TypeHint_Cast.htm)

### Properties
- `HintID` (Guid, get) — Unique ID for every type of Hint.
- `TypeName` (String, get) — Name of target Type

## Param_MeshParameters (class)

(No description provided in vendor docs for Grasshopper.Kernel.Parameters.Param_MeshParameters.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Parameters_Param_MeshParameters.htm)

### Constructors
- `public Param_MeshParameters()` — Initializes a new instance of the Param_MeshParameters class

### Methods
#### `[ObsoleteAttribute("This method has been superceded by SetPersistenData overloads")] public bool AddPersistentData(Object data)`

Utility function to add a new persistent data item. If the data is null or cannot be converted into T, this method will fail. You should probably use the PersistentData property to add type-safe data if you can.

**Parameters:**
- `data` (System.Object) — Data to add.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AddPersistentData.htm)

#### `[ObsoleteAttribute("This method has been superceded by SetPersistenData overloads")] public bool AddPersistentData(T data)`

Utility function to add a new persistent data item. If the data is null or cannot be converted into T, this method will fail. You should probably use the PersistentData property to add type-safe data if you can.

**Parameters:**
- `data` (T) — Data to add.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AddPersistentData_1.htm)

#### `public virtual void AddRuntimeMessage(GH_RuntimeMessageLevel level, string text)`

Add a new message to this object. Valid message type flags are Warning and Error. If the Message string is empty or zero-length no message is added.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Type of message. Only Warnings and Errors are recorded.
- `text` (System.String) — Content of message.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AddRuntimeMessage.htm)

#### `public virtual void AddSource(IGH_Param source)`

Append a new Source parameter to the end of the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource.htm)

#### `public virtual void AddSource(IGH_Param source, int index)`

Insert a new Source parameter into the Sources list. Sources provide this parameter with data at runtime.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to append.
- `index` (System.Int32) — Insertion index of source.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddSource_1.htm)

#### `public bool AddVolatileData(GH_Path path, int index, Object data)`

Inserts an item of volatile data into the data structure.

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path) — The branch path of the data. If the path doesn't exist yet, it will be created.
- `index` (System.Int32) — The item index of the data. If the path doesn't contain the index yet, it will be enlarged to encompass the index.
- `data` (System.Object) — The data to store.

**Returns:** `Boolean` — True on success, False on failure. If the data cannot be converted, the topology will remain unmolested.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData.htm)

#### `public bool AddVolatileData(GH_Path path, int index, T data)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `index` (System.Int32)
- `data` (T)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileData_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, IEnumerable list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.IEnumerable)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList_1.htm)

#### `public bool AddVolatileDataList(GH_Path path, List<T> list)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `path` (Grasshopper.Kernel.Data.GH_Path)
- `list` (System.Collections.Generic.List<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataList.htm)

#### `public bool AddVolatileDataTree(GH_Structure<T> tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.GH_Structure<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree.htm)

#### `public bool AddVolatileDataTree(IGH_Structure tree)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `tree` (Grasshopper.Kernel.Data.IGH_Structure)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_AddVolatileDataTree_1.htm)

#### `public virtual void AddedToDocument(GH_Document document)`

This method will be called when an object is added to a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_AddedToDocument.htm)

#### `public override void AppendAdditionalMenuItems(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_AppendAdditionalMenuItems.htm)

#### `public override bool AppendMenuItems(ToolStripDropDown menu)`

This function is called when a context menu is about to be displayed. Override it to set custom items. GH_ActiveObject will already populate the menu with default items, if you merely wish to insert object-specific menu item, consider overriding AppendAdditionalMenuItems instead.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown) — Menu object to populate.

**Returns:** `Boolean` — If true, the menu will be displayed, if false the menu will be supressed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_AppendMenuItems.htm)

#### `protected T Cast_Object(Object data)`

Attempts to convert the Object reference into an instance of T. This method will perform a direct cast if possible or it will call Casting functions on T or Data if they exist. Data will not be duplicated unless a type conversion is required.

**Parameters:**
- `data` (System.Object)

**Returns:** `T` — The cast value or Nothing on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Cast_Object.htm)

#### `public override void ClearData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ClearData.htm)

#### `public virtual void ClearProxySources()`

Remove all proxy sources without attempting to relink them.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ClearProxySources.htm)

#### `public virtual void ClearRuntimeMessages()`

Destroy all warning and error lists

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ClearRuntimeMessages.htm)

#### `public override sealed void CollectData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CollectData.htm)

#### `protected override sealed void CollectVolatileData_Custom()`

(Inherited from GH_PersistentParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_CollectVolatileData_Custom.htm)

#### `protected virtual void CollectVolatileData_FromSources()`

This method collects volatile data from all the source parameters.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CollectVolatileData_FromSources.htm)

#### `public override sealed void ComputeData()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ComputeData.htm)

#### `protected virtual void ConversionCallback(Object source, T target)`

This method is called whenever a successful conversion takes place from some source data into local target data. Override it if you wish to keep tabs on conversions.

**Parameters:**
- `source` (System.Object) — Source data (never null).
- `target` (T) — Target data (never null).

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ConversionCallback.htm)

#### `public void CopyFrom(IGH_InstanceDescription other)`

Copy all fields (except the instance ID) from another instance description.

**Parameters:**
- `other` (Grasshopper.Kernel.IGH_InstanceDescription) — Object to mimic.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_CopyFrom.htm)

#### `public override void CreateAttributes()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateAttributes.htm)

#### `public virtual void CreateProxySources()`

Convert all proper source parameters into proxy sources.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_CreateProxySources.htm)

#### `public override bool DependsOn(IGH_ActiveObject potential_source)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `potential_source` (Grasshopper.Kernel.IGH_ActiveObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_DependsOn.htm)

#### `protected void DestroyIconCache()`

Call this method to erase the existing icon cache. You must call this if you want to change the display icon of an object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_DestroyIconCache.htm)

#### `public virtual void DocumentContextChanged(GH_Document document, GH_DocumentContext context)`

This method will be called when the document that owns this object moves into a different context.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that owns this object.
- `context` (Grasshopper.Kernel.GH_DocumentContext) — The reason for this event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_DocumentContextChanged.htm)

#### `protected override void ExpireDownStreamObjects()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ExpireDownStreamObjects.htm)

#### `public virtual void ExpirePreview(bool redraw)`

Call this function when you suspect that the preview has expired for this object. This will cause the display cache to be eradicated.

**Parameters:**
- `redraw` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_ExpirePreview.htm)

#### `public override void ExpireSolution(bool recompute)`

Informs the document that owns this object that the solution has expired. The current object will be set to BLANK as a result. This method is recursive, it will also expire any and all objects which depend on this object. If you want a less destructive expiration, consider using ClearData(). If this object is already Blank, you should consider not expiring it.

**Parameters:**
- `recompute` (System.Boolean) — If True, the document will be instructed to solve the new state.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_ExpireSolution.htm)

#### `public void ExpireSolutionTopLevel(bool recompute)`

Invoke the Expiresolution(bool) method on the toplevel object.

**Parameters:**
- `recompute` (System.Boolean) — If true, a new computation will start right away.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ExpireSolutionTopLevel.htm)

#### `protected virtual string Format(T Data)`

Returns "Null" if the data is a null reference, otherwise calls ToString() on the Data instance.

**Parameters:**
- `Data` (T)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Format.htm)

#### `protected bool GetValue(string valueName, bool default)`

Get a boolean value from the component value table.

**Parameters:**
- `valueName` (System.String) — Name of boolean to retrieve.
- `default` (System.Boolean) — Default value to return in case of missing named value.

**Returns:** `Boolean` — The boolean value with the given name of the default value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_GetValue.htm)

#### `protected Color GetValue(string valueName, Color default)`

Get a color value from the component value table.

**Parameters:**
- `valueName` (System.String) — Name of color to retrieve.
- `default` (System.Drawing.Color) — Default value to return in case of missing named value.

**Returns:** `Color` — The color value with the given name of the default value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_GetValue_2.htm)

#### `protected double GetValue(string valueName, double default)`

Get a double value from the component value table.

**Parameters:**
- `valueName` (System.String) — Name of double to retrieve.
- `default` (System.Double) — Default value to return in case of missing named value.

**Returns:** `Double` — The double value with the given name of the default value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_GetValue_1.htm)

#### `protected int GetValue(string valueName, int default)`

Get an integer value from the component value table.

**Parameters:**
- `valueName` (System.String) — Name of integer to retrieve.
- `default` (System.Int32) — Default value to return in case of missing named value.

**Returns:** `Int32` — The integer value with the given name of the default value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_GetValue_3.htm)

#### `protected string GetValue(string valueName, string default)`

Get a string value from the component value table.

**Parameters:**
- `valueName` (System.String) — Name of string to retrieve.
- `default` (System.String) — Default value to return in case of missing named value.

**Returns:** `String` — The string value with the given name of the default value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_GetValue_4.htm)

#### `protected internal virtual string HtmlHelp_Source()`

Return a String which contains HTML formatted source for the help topic. If you want to pass a URL that points to a remote page, then prefix the URL with a GOTO: tag, like so: GOTO:http://www.YourWebAddressHere.com

**Remarks:** You can use the GUI.GH_HtmlFormatter class to generate an HTML page which conforms to Grasshopper Help standards

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_HtmlHelp_Source.htm)

#### `protected virtual T InstantiateT()`

Attempts to instantiate a new instance of T.

**Remarks:** If T is an abstract class or if T has no default (empty) constructor, this function will ASSERT. In order to avoid a Try...Catch block performance impact, override this function and call the type constructor for T directly.

**Returns:** `T` — This function should return a newly constructed instance of T.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_InstantiateT.htm)

#### `public override void IsolateObject()`

(Inherited from GH_Param<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_IsolateObject.htm)

#### `protected void Menu_AppendBakeItem(ToolStripDropDown menu)`

Append the default Bake menu item.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_Menu_AppendBakeItem.htm)

#### `protected virtual void Menu_AppendDestroyPersistent(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_AppendDestroyPersistent.htm)

#### `protected void Menu_AppendDisconnectWires(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendDisconnectWires.htm)

#### `protected void Menu_AppendEnableItem(ToolStripDropDown menu)`

Append the default Enable/Disable menu item.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_Menu_AppendEnableItem.htm)

#### `protected virtual void Menu_AppendExtractParameter(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_AppendExtractParameter.htm)

#### `protected void Menu_AppendFlattenParameter(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendFlattenParameter.htm)

#### `protected void Menu_AppendGraftParameter(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendGraftParameter.htm)

#### `protected virtual void Menu_AppendInternaliseData(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_AppendInternaliseData.htm)

#### `protected virtual void Menu_AppendManageCollection(ToolStripDropDown menu)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_AppendManageCollection.htm)

#### `protected void Menu_AppendObjectHelp(ToolStripDropDown menu)`

Appends the default object Help menu item.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_Menu_AppendObjectHelp.htm)

#### `protected void Menu_AppendObjectName(ToolStripDropDown menu)`

Appends the old-fashioned object name menu item. If you also want the Display mode toggle then use Menu_AppendObjectNameEx()

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_Menu_AppendObjectName.htm)

#### `protected void Menu_AppendObjectNameEx(ToolStripDropDown menu)`

Appends the default object name + display mode menu item.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_Menu_AppendObjectNameEx.htm)

#### `protected void Menu_AppendPreviewItem(ToolStripDropDown menu)`

Append the default Show/Hide preview menu item.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_Menu_AppendPreviewItem.htm)

#### `protected void Menu_AppendPrincipalParameter(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendPrincipalParameter.htm)

#### `protected override void Menu_AppendPromptMore(ToolStripDropDown menu)`

(Overrides GH_PersistentParam<T>.Menu_AppendPromptMore(ToolStripDropDown).)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Parameters_Param_MeshParameters_Menu_AppendPromptMore.htm)

#### `protected override void Menu_AppendPromptOne(ToolStripDropDown menu)`

(Overrides GH_PersistentParam<T>.Menu_AppendPromptOne(ToolStripDropDown).)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Parameters_Param_MeshParameters_Menu_AppendPromptOne.htm)

#### `protected void Menu_AppendPublish(ToolStripDropDown menu)`

Appends the default item for publishing to RCP. This menu will only appear if the current class implement IRcpAwareObject

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_Menu_AppendPublish.htm)

#### `protected void Menu_AppendReverseParameter(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendReverseParameter.htm)

#### `protected void Menu_AppendRuntimeMessages(ToolStripDropDown menu)`

Append the default warnings and errors menu items.

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_Menu_AppendRuntimeMessages.htm)

#### `protected void Menu_AppendSimplifyParameter(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendSimplifyParameter.htm)

#### `[ObsoleteAttribute("This method has been replaced by Menu_AppendRuntimeMessages()")] protected void Menu_AppendWarningsAndErrors(ToolStripDropDown menu)`

This method is obsolete and has been replaced with Menu_AppendRuntimeMessages().

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_Menu_AppendWarningsAndErrors.htm)

#### `protected void Menu_AppendWireDisplay(ToolStripDropDown menu)`

(Inherited from GH_Param<T>.)

**Parameters:**
- `menu` (System.Windows.Forms.ToolStripDropDown)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Menu_AppendWireDisplay.htm)

#### `protected ToolStripMenuItem Menu_CreateMultilineTextEditItem()`

This function returns a ToolstripMenuItem that embeds a multi-line textbox for editing persistent data. Only call this method if you know that your parameter type supports proxies.

**Returns:** `ToolStripMenuItem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_CreateMultilineTextEditItem.htm)

#### `protected virtual ToolStripMenuItem Menu_CustomMultiValueItem()`

(Inherited from GH_PersistentParam<T>.)

**Returns:** `ToolStripMenuItem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_CustomMultiValueItem.htm)

#### `protected virtual ToolStripMenuItem Menu_CustomSingleValueItem()`

(Inherited from GH_PersistentParam<T>.)

**Returns:** `ToolStripMenuItem` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Menu_CustomSingleValueItem.htm)

#### `public virtual void MovedBetweenDocuments(GH_Document oldDocument, GH_Document newDocument)`

This method will be called when an object is moved from one document to another. Override this method if you want to handle such events.

**Parameters:**
- `oldDocument` (Grasshopper.Kernel.GH_Document) — Document that used to own this object.
- `newDocument` (Grasshopper.Kernel.GH_Document) — Document that now owns ths object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_MovedBetweenDocuments.htm)

#### `public void NewInstanceGuid()`

Generate a new random instance GUID

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid UUID)`

Set the instance ID to be a specific GUID. This is very dangerous, only use this function if you're 6"4' and your first name is David.

**Parameters:**
- `UUID` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_NewInstanceGuid_1.htm)

#### `public void OnAttributesChanged()`

Raises the AttributesChanged event on the toplevel object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnAttributesChanged.htm)

#### `public void OnDisplayExpired(bool redraw)`

Raises the DisplayExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the canvas will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnDisplayExpired.htm)

#### `public void OnObjectChanged(GH_ObjectChangedEventArgs e)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `e` (Grasshopper.Kernel.GH_ObjectChangedEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_1.htm)

#### `public void OnObjectChanged(GH_ObjectEventType eventType, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `eventType` (Grasshopper.Kernel.GH_ObjectEventType)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_2.htm)

#### `public void OnObjectChanged(string customEvent)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_3.htm)

#### `public void OnObjectChanged(string customEvent, Object tag)`

(Inherited from GH_DocumentObject.)

**Parameters:**
- `customEvent` (System.String)
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnObjectChanged_4.htm)

#### `public GH_Document OnPingDocument()`

Raise the PingDocument Event on the toplevel object and try to find the document which owns this object.

**Returns:** `GH_Document` — The document which owns this object if successful, or nothing if no document owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPingDocument.htm)

#### `public void OnPreviewExpired(bool redraw)`

Raises the PreviewExpired event on the toplevel object.

**Parameters:**
- `redraw` (System.Boolean) — If True, the viewports will be immediately redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnPreviewExpired.htm)

#### `public void OnSolutionExpired(bool recompute)`

Raises the SolutionExpired event on the toplevel object. You probably want to call ExpireSolution() instead of this method directly.

**Parameters:**
- `recompute` (System.Boolean) — If True, the solution will be immediately recalculated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_OnSolutionExpired.htm)

#### `protected virtual void OnVolatileDataCollected()`

Once volatile data has been collected this method will be calles. The basic implementation does nothing, but you can add code here to post-process or analyze the volatile data.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_OnVolatileDataCollected.htm)

#### `protected virtual T PreferredCast(Object data)`

Implement this function if you're certain that you'll be confronted with very common casts. For example, GH_Point has a preferred cast from Rhino.Geometry.Point3d and GH_Number has a preferred cast from System.Double.

**Parameters:**
- `data` (System.Object) — Data to convert. Data is never null.

**Returns:** `T` — An instance of T or null if you did not handle the cast.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_PreferredCast.htm)

#### `protected virtual void PrepareForPrompt()`

(Inherited from GH_PersistentParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_PrepareForPrompt.htm)

#### `protected BoundingBox Preview_ComputeClippingBox()`

This function can be used to iterate over all items in the Volatile data tree and collect the union clipping box of all non-null items that implement Preview.IGH_PreviewData. This function requires a lot of TypeOf and DirectCasting, so if you're worried about performance you should consider doing this yourself.

**Returns:** `BoundingBox` — The clipping box for all valid items.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Preview_ComputeClippingBox.htm)

#### `protected void Preview_DrawMeshes(IGH_PreviewArgs args)`

This function can be used to iterate over all items in the Volatile data tree and call DrawViewportWires on each item that implements Preview.IGH_PreviewData. This function requires a lot of TypeOf and DirectCasting, so if you're worried about performance you should consider doing this yourself.

**Parameters:**
- `args` (Grasshopper.Kernel.IGH_PreviewArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Preview_DrawMeshes.htm)

#### `protected void Preview_DrawWires(IGH_PreviewArgs args)`

This function can be used to iterate over all items in the Volatile data tree and call DrawViewportWires on each item that implements Preview.IGH_PreviewData. This function requires a lot of TypeOf and DirectCasting, so if you're worried about performance you should consider doing this yourself.

**Parameters:**
- `args` (Grasshopper.Kernel.IGH_PreviewArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Preview_DrawWires.htm)

#### `protected virtual bool Prompt_ManageCollection(GH_Structure<T> values)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `values` (Grasshopper.Kernel.Data.GH_Structure<T>)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Prompt_ManageCollection.htm)

#### `protected override GH_GetterResult Prompt_Plural(ref List<GH_MeshingParameters> values)`

(Overrides GH_PersistentParam<T>.Prompt_Plural(List<T>).)

**Parameters:**
- `values` (System.Collections.Generic.List<GH_MeshingParameters>)

**Returns:** `GH_GetterResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Parameters_Param_MeshParameters_Prompt_Plural.htm)

#### `protected override GH_GetterResult Prompt_Singular(ref GH_MeshingParameters value)`

(Overrides GH_PersistentParam<T>.Prompt_Singular(T).)

**Parameters:**
- `value` (Grasshopper.Kernel.Types.GH_MeshingParameters)

**Returns:** `GH_GetterResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Parameters_Param_MeshParameters_Prompt_Singular.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Read.htm)

#### `public virtual bool ReadFull(GH_IReader reader)`

GH_InstanceDescription does not by default serialize all fields. Use this function to read all fields from the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Writer for deserialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_ReadFull.htm)

#### `protected void RecordPersistentDataEvent(string name)`

Add an undo record that stores changes to persistent data.

**Parameters:**
- `name` (System.String) — Name of undo record.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_RecordPersistentDataEvent.htm)

#### `public void RecordUndoEvent(GH_UndoRecord record)`

Record an entire undo record.

**Parameters:**
- `record` (Grasshopper.Kernel.Undo.GH_UndoRecord) — Record to push.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent.htm)

#### `public Guid RecordUndoEvent(string undoName)`

Record a generic object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_1.htm)

#### `public Guid RecordUndoEvent(string undoName, IGH_UndoAction action)`

Record a specific object change undo event.

**Parameters:**
- `undoName` (System.String) — Name of undo record.
- `action` (Grasshopper.Kernel.Undo.IGH_UndoAction) — Undo action to record.

**Returns:** `Guid` — The ID of the newly added record or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RecordUndoEvent_2.htm)

#### `protected virtual void RecoverFromPrompt()`

(Inherited from GH_PersistentParam<T>.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_RecoverFromPrompt.htm)

#### `public virtual void RegisterRemoteIDs(GH_GuidTable id_list)`

Override this function if you want object changes in the Rhino document to trigger a new solution.

**Parameters:**
- `id_list` (GH_GuidTable) — The object ID database Grasshopper is responsive to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_RegisterRemoteIDs.htm)

#### `protected void RegisterRemoteIDsUtil(GH_GuidTable table)`

Utility function which treats all data in the Volatile cache as IGH_GeometricGoo and registers all referenced objects. Call this function from within RegisterRemoteIDs() if you are absolutely sure that all the items in volatiledata implement IGH_GeometricGoo.

**Parameters:**
- `table` (GH_GuidTable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RegisterRemoteIDsUtil.htm)

#### `public virtual bool RelinkProxySources(GH_Document document)`

Attempt to replace all proxy sources with real sources. Proxy sources are used during file IO, when actual sources might not be available yet. Once an IO operation has been completed there should be no more proxy sources.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — The document from which to harvest the real source parameters.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RelinkProxySources.htm)

#### `public virtual void RemoveAllSources()`

Remove all sources from this parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveAllSources.htm)

#### `public virtual void RemoveEffects()`

Remove all post-process effects. Note to implementors, you must call the base method if you override this function.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveEffects.htm)

#### `public virtual void RemoveSource(Guid source_id)`

Remove the specified source from this parameter.

**Parameters:**
- `source_id` (System.Guid) — InstanceID of source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource_1.htm)

#### `public virtual void RemoveSource(IGH_Param source)`

Remove the specified source from this parameter.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param) — Source to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_RemoveSource.htm)

#### `public virtual void RemovedFromDocument(GH_Document document)`

This method will be called when an object is removed from a document. Override this method if you want to handle such events.

**Parameters:**
- `document` (Grasshopper.Kernel.GH_Document) — Document that now no longer owns this object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_RemovedFromDocument.htm)

#### `[ObsoleteAttribute] protected void Render_AppendGeometry(GH_RenderArgs args)`

This function has been emptied because it is Obsolete.

**Parameters:**
- `args` (Grasshopper.Kernel.GH_RenderArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_Render_AppendGeometry.htm)

#### `public virtual void ReplaceSource(Guid old_source_id, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source_id` (System.Guid) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource_1.htm)

#### `public virtual void ReplaceSource(IGH_Param old_source, IGH_Param new_source)`

Replace an existing source with a new one. If the old_source does not exist in this parameter, nothing happens.

**Parameters:**
- `old_source` (Grasshopper.Kernel.IGH_Param) — Source to replace.
- `new_source` (Grasshopper.Kernel.IGH_Param) — Source to replace with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_ReplaceSource.htm)

#### `public virtual IList<string> RuntimeMessages(GH_RuntimeMessageLevel level)`

Gets the list of cached runtime messages that were recorded during solver-time processes.

**Parameters:**
- `level` (Grasshopper.Kernel.GH_RuntimeMessageLevel) — Runtime message type to retrieve.

**Returns:** `IList<String>` — A list of runtime message strings.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_RuntimeMessages.htm)

#### `public virtual bool SDKCompliancy(int exeVersion, int exeServiceRelease)`

Test whether this object is compliant with a given Rhino version.

**Parameters:**
- `exeVersion` (System.Int32) — Rhino major release (4, 5, ...).
- `exeServiceRelease` (System.Int32) — Rhino minor (service) release.

**Returns:** `Boolean` — True if this object is compliant with the given Rhino release.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_ActiveObject_SDKCompliancy.htm)

#### `public void Script_AddPersistentData(List<Object> data)`

Method used primarily for COM_Access code.

**Parameters:**
- `data` (System.Collections.Generic.List<Object>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Script_AddPersistentData.htm)

#### `public void Script_ClearPersistentData()`

Method used primarily for COM_Access code.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Script_ClearPersistentData.htm)

#### `public void SetIconOverride(Bitmap customIcon)`

Set a new custom icon override for this object.

**Parameters:**
- `customIcon` (System.Drawing.Bitmap) — Icon override. Should be a 24x24 image.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetIconOverride.htm)

#### `public void SetPersistentData(GH_Structure<T> items)`

Assign a tree of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (Grasshopper.Kernel.Data.GH_Structure<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData.htm)

#### `public void SetPersistentData(IEnumerable<T> items)`

Assign a list of items to the persistent data. This method will erase any existing data, record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add a tree of data, you should use the appropriate overload for this method.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_1.htm)

#### `public void SetPersistentData(params Object[] values)`

Add a collection of values to the persistent data.

**Parameters:**
- `values` (System.Object[]) — Values to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_2.htm)

#### `public void SetPersistentData(T item)`

Add a single item to the persistent data. This method will record an undo event, raise the OnObjectChanged event for PersistentData flags and place a call to ExpireSolution(False). If you want to add more than one piece of data, you should use the appropriate overload for this method.

**Parameters:**
- `item` (T) — Item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_SetPersistentData_3.htm)

#### `public bool SetPrincipal(bool set, bool recordUndo, bool recompute)`

Set the principal parameter state.

**Parameters:**
- `set` (System.Boolean)
- `recordUndo` (System.Boolean)
- `recompute` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_SetPrincipal.htm)

#### `protected void SetValue(string valueName, bool value)`

Set a named value. This value will be serialized with the component.

**Parameters:**
- `valueName` (System.String) — Name of value.
- `value` (System.Boolean) — Value itself.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetValue.htm)

#### `protected void SetValue(string valueName, Color value)`

Set a named value. This value will be serialized with the component.

**Parameters:**
- `valueName` (System.String) — Name of value.
- `value` (System.Drawing.Color) — Value itself.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetValue_2.htm)

#### `protected void SetValue(string valueName, double value)`

Set a named value. This value will be serialized with the component.

**Parameters:**
- `valueName` (System.String) — Name of value.
- `value` (System.Double) — Value itself.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetValue_1.htm)

#### `protected void SetValue(string valueName, int value)`

Set a named value. This value will be serialized with the component.

**Parameters:**
- `valueName` (System.String) — Name of value.
- `value` (System.Int32) — Value itself.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetValue_3.htm)

#### `protected void SetValue(string valueName, string value)`

Set a named value. This value will be serialized with the component.

**Parameters:**
- `valueName` (System.String) — Name of value.
- `value` (System.String) — Value itself.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_SetValue_4.htm)

#### `public void TriggerAutoSave()`

Triggers the AutoSave function on the owner document with the object_changed flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_1.htm)

#### `public void TriggerAutoSave(GH_AutoSaveTrigger trigger, Guid id)`

Triggers the AutoSave function on the owner document with a custom flag.

**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger) — Reason for the autosave operation. It is possible that a user has decided to avoid autosave events for specific types, so if you can, try and provide a correct trigger flag.
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_2.htm)

#### `public void TriggerAutoSave(Guid id)`

Triggers the AutoSave function on the owner document with the object_changed flag.

**Parameters:**
- `id` (System.Guid) — ID of autosave event. Consecutive autosave requests with the same ID will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_TriggerAutoSave_3.htm)

#### `protected virtual void ValuesChanged()`

Override this method if you want to respond to changes to the value table. The base implementation is empty, so you don't have to call it.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_DocumentObject_ValuesChanged.htm)

#### `protected virtual string VolatileDataDescription()`

This method is called to populate the Tooltip data description field.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_Param_1_VolatileDataDescription.htm)

#### `public override bool Write(GH_IWriter writer)`

(Inherited from GH_PersistentParam<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_PersistentParam_1_Write.htm)

#### `public virtual bool WriteFull(GH_IWriter writer)`

GH_InstanceDescription does not by default serialize all fields. Use this function to write all fields to the archive. This method is compatible with the default Write()/Read() operations.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer for serialization.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_GH_InstanceDescription_WriteFull.htm)

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
- `Icon` (Bitmap, get) — (Overrides GH_DocumentObject.Icon.)
- `Icon_24x24` (Bitmap, get) — The icon associated with this object.
- `Icon_24x24_Locked` (Bitmap, get) — The greyscale icon of this object.
- `IconCapableUI` (Boolean, get) — By default the NickName menu item supports the Icon Mode override toggle. If your UI is not capable of displaying icons, then override this property and return False.
- `IconDisplayMode` (GH_IconDisplayMode, get/set) — Gets the current display mode of the object.
- `InstanceDescription` (String, get) — Gets the description of this instance. The default description is about the amount and source of the local values.
- `InstanceGuid` (Guid, get) — Gets the ID of this runtime instance.
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
- `TypeName` (String, get) — Calls TypeName() on the first instance of T it can find. This is either an item in the volatile list, or a newly constructed instance.
- `VolatileData` (IGH_Structure, get) — (Inherited from GH_Param<T>.)
- `VolatileDataCount` (Int32, get) — (Inherited from GH_Param<T>.)
- `WireDisplay` (GH_ParamWireDisplay, get/set) — Gets or sets the wire display style for this parameter. Wire display only affects the wires connected to the parameter input.
- `m_attributes` (IGH_Attributes, get) — (Inherited from GH_DocumentObject.)
- `m_data` (GH_Structure<T>, get) — Contains the runtime data for this parameter, also known as "Volatile" data.

### Events
#### `AttributesChanged` (`Grasshopper.Kernel.IGH_DocumentObject.AttributesChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.AttributesChangedEventHandler AttributesChanged`

Raised whenever the number or kind of attributes changes. This event is handled by GH_Documents who subsequently wipe their attribute caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_AttributesChanged.htm)

#### `DisplayExpired` (`Grasshopper.Kernel.IGH_DocumentObject.DisplayExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.DisplayExpiredEventHandler DisplayExpired`

Raised whenever the display (on the Canvas) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_DisplayExpired.htm)

#### `ObjectChanged` (`Grasshopper.Kernel.IGH_DocumentObject.ObjectChangedEventHandler`)

**Signature:** `public event IGH_DocumentObject.ObjectChangedEventHandler ObjectChanged`

(Inherited from GH_DocumentObject.)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_ObjectChanged.htm)

#### `PingDocument` (`Grasshopper.Kernel.IGH_DocumentObject.PingDocumentEventHandler`)

**Signature:** `public event IGH_DocumentObject.PingDocumentEventHandler PingDocument`

Raised whenever an object needs to know which GH_Document it belongs to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PingDocument.htm)

#### `PreviewExpired` (`Grasshopper.Kernel.IGH_DocumentObject.PreviewExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.PreviewExpiredEventHandler PreviewExpired`

Raised whenever the display (in the Rhino viewports) of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_PreviewExpired.htm)

#### `SolutionExpired` (`Grasshopper.Kernel.IGH_DocumentObject.SolutionExpiredEventHandler`)

**Signature:** `public event IGH_DocumentObject.SolutionExpiredEventHandler SolutionExpired`

Raised whenever the solution of a certain object becomes invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/E_Grasshopper_Kernel_GH_DocumentObject_SolutionExpired.htm)

