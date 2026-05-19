---
name: rhino-rhino-render-datasources
description: This skill encodes the rhino 7.0 surface of the Rhino.Render.DataSources namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ContentFactory, ContentFactories, MetaData, RdkEdit, RdkModalEditContentBucket, RdkSelectionNavigator, RhinoSettings, AssignBys, and 3 more types.
---

# Rhino.Render.DataSources

Auto-generated from vendor docs for rhino 7.0. 11 types in this namespace.

## AssignBys (enum)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.AssignBys"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_AssignBys.htm)

### Values
- `Unset` = `0`
- `Layer` = `1`
- `Parent` = `2`
- `Object` = `3`
- `Varies` = `4`
- `PlugIn` = `5`

## ContentFactories (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.ContentFactories"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_ContentFactories.htm)

### Constructors
- `public ContentFactories(IntPtr pRdkContentFactories)` — Initializes a new instance of the ContentFactories class

### Methods
#### `public void Dispose()`

Releases all resources used by the ContentFactories

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactories_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactories_Finalize.htm)

#### `public ContentFactory FindFactory(Guid uuid)`



**Parameters:**
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Render.DataSources.ContentFactories.FindFactory(System.Guid)"]

**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.ContentFactories.FindFactory(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactories_FindFactory.htm)

#### `public ContentFactory FirstFactory()`



**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.ContentFactories.FirstFactory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactories_FirstFactory.htm)

#### `public ContentFactory NextFactory()`



**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.ContentFactories.NextFactory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactories_NextFactory.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## ContentFactory (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.ContentFactory"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_ContentFactory.htm)

### Constructors
- `public ContentFactory(IntPtr pRdkContentFactory)` — Initializes a new instance of the ContentFactory class

### Methods
#### `public Guid ContentTypeId()`



**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.ContentFactory.ContentTypeId"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactory_ContentTypeId.htm)

#### `public void Dispose()`

Releases all resources used by the ContentFactory

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactory_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactory_Finalize.htm)

#### `public RenderContentKind Kind()`



**Returns:** `RenderContentKind` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.ContentFactory.Kind"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactory_Kind.htm)

#### `public RenderContent NewContent()`

New Content returns a new content, which is Initialized with the Initialize() function. The content should be unitilized after use with the Unitialize function.

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.ContentFactory.NewContent"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_ContentFactory_NewContent.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## MetaData (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.MetaData"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_MetaData.htm)

### Constructors
- `public MetaData(IntPtr pMetaData)` — Initializes a new instance of the MetaData class

### Methods
#### `public Guid ContentInstanceId()`



**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.MetaData.ContentInstanceId"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_MetaData_ContentInstanceId.htm)

#### `public void Dispose()`

Releases all resources used by the MetaData

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_MetaData_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_MetaData_Finalize.htm)

#### `public string Geometry()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.MetaData.Geometry"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_MetaData_Geometry.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## Modes (enum)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.Modes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_Modes.htm)

### Values
- `Unset` = `0`
- `Grid` = `1`
- `List` = `2`
- `Tree` = `3`

## RdkEdit (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.RdkEdit"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_RdkEdit.htm)

### Constructors
- `public RdkEdit(IntPtr pRdkEdit)` — Initializes a new instance of the RdkEdit class

### Methods
#### `public void Dispose()`

Releases all resources used by the RdkEdit

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkEdit_Dispose.htm)

#### `public bool Execute(RenderContentCollection collection)`



**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.DataSources.RdkEdit.Execute(Rhino.Render.RenderContentCollection)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RdkEdit.Execute(Rhino.Render.RenderContentCollection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkEdit_Execute.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkEdit_Finalize.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## RdkModalEditContentBucket (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.RdkModalEditContentBucket"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_RdkModalEditContentBucket.htm)

### Constructors
- `public RdkModalEditContentBucket(IntPtr pRdkModalEditContentBucket)` — Initializes a new instance of the RdkModalEditContentBucket class

### Methods
#### `public RenderContentCollection ContentsIn()`



**Returns:** `RenderContentCollection` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RdkModalEditContentBucket.ContentsIn"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkModalEditContentBucket_ContentsIn.htm)

#### `public void Dispose()`

Releases all resources used by the RdkModalEditContentBucket

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkModalEditContentBucket_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkModalEditContentBucket_Finalize.htm)

#### `public void SetContentsOut(RenderContentCollection collection)`



**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.DataSources.RdkModalEditContentBucket.SetContentsOut(Rhino.Render.RenderContentCollection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkModalEditContentBucket_SetContentsOut.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## RdkSelectionNavigator (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.RdkSelectionNavigator"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_RdkSelectionNavigator.htm)

### Constructors
- `public RdkSelectionNavigator(IntPtr pRhinoSettings)` — Initializes a new instance of the RdkSelectionNavigator class

### Methods
#### `public void Add(RenderContentCollection selectedContentArray)`

Add a selection of contents at the current position. Clears the navigator ahead of the current position.

**Parameters:**
- `selectedContentArray` (Rhino.Render.RenderContentCollection) — The selected content

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_Add.htm)

#### `public bool CanGoBackwards()`

Check the backwards status of the navigator

**Returns:** `Boolean` — True if it is possible to navigate backwards, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_CanGoBackwards.htm)

#### `public bool CanGoForwards()`

Check the forwards status of the navigator

**Returns:** `Boolean` — True if it is possible to navigate forwards, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_CanGoForwards.htm)

#### `public void Clear()`

Clear the navigator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_Clear.htm)

#### `public void Dispose()`

Releases all resources used by the RdkSelectionNavigator

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_Finalize.htm)

#### `public bool GoBackwards(ref RenderContentCollection selectedContentArray)`

Navigate backwards if possible

**Parameters:**
- `selectedContentArray` (Rhino.Render.RenderContentCollection) — selectedContentArray is the new selection after navigating backwards

**Returns:** `Boolean` — true on success, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_GoBackwards.htm)

#### `public bool GoForwards(ref RenderContentCollection selectedContentArray)`

Navigate forwards if possible

**Parameters:**
- `selectedContentArray` (Rhino.Render.RenderContentCollection) — selectedContentArray is the new selection after navigating forwards

**Returns:** `Boolean` — true on success, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RdkSelectionNavigator_GoForwards.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## RhinoSettings (class)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.RhinoSettings"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_RhinoSettings.htm)

### Constructors
- `public RhinoSettings(IntPtr pRhinoSettings)` — Initializes a new instance of the RhinoSettings class

### Methods
#### `public RhinoView ActiveView()`



**Returns:** `RhinoView` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RhinoSettings.ActiveView"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_ActiveView.htm)

#### `public void Dispose()`

Releases all resources used by the RhinoSettings

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_Finalize.htm)

#### `public List<Size> GetCustomRenderSizes()`



**Returns:** `List<Size>` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RhinoSettings.GetCustomRenderSizes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_GetCustomRenderSizes.htm)

#### `public RenderSettings GetRenderSettings()`



**Returns:** `RenderSettings` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RhinoSettings.GetRenderSettings"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_GetRenderSettings.htm)

#### `public bool GroundPlaneOnInViewDisplayMode(RhinoView view)`



**Parameters:**
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.Render.DataSources.RhinoSettings.GroundPlaneOnInViewDisplayMode(Rhino.Display.RhinoView)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RhinoSettings.GroundPlaneOnInViewDisplayMode(Rhino.Display.RhinoView)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_GroundPlaneOnInViewDisplayMode.htm)

#### `public ViewInfo RenderingView()`



**Returns:** `ViewInfo` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RhinoSettings.RenderingView"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_RenderingView.htm)

#### `public void SetGroundPlaneOnInViewDisplayMode(RhinoView view, bool bOn)`



**Parameters:**
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.Render.DataSources.RhinoSettings.SetGroundPlaneOnInViewDisplayMode(Rhino.Display.RhinoView,System.Boolean)"]
- `bOn` (System.Boolean) — [Missing <param name="bOn"/> documentation for "M:Rhino.Render.DataSources.RhinoSettings.SetGroundPlaneOnInViewDisplayMode(Rhino.Display.RhinoView,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_SetGroundPlaneOnInViewDisplayMode.htm)

#### `public void SetRenderSettings(RenderSettings renderSettings)`



**Parameters:**
- `renderSettings` (Rhino.Render.RenderSettings) — [Missing <param name="renderSettings"/> documentation for "M:Rhino.Render.DataSources.RhinoSettings.SetRenderSettings(Rhino.Render.RenderSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_SetRenderSettings.htm)

#### `public bool ViewSupportsShading(RhinoView view)`



**Parameters:**
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.Render.DataSources.RhinoSettings.ViewSupportsShading(Rhino.Display.RhinoView)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.DataSources.RhinoSettings.ViewSupportsShading(Rhino.Display.RhinoView)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DataSources_RhinoSettings_ViewSupportsShading.htm)

### Properties
- `CppPointer` (IntPtr, get) — 
- `CustomImageSizeIsPreset` (Boolean, get/set) — 

## Shapes (enum)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.Shapes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_Shapes.htm)

### Values
- `Square` = `0`
- `Wide` = `1`

## Sizes (enum)

[Missing <summary> documentation for "T:Rhino.Render.DataSources.Sizes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DataSources_Sizes.htm)

### Values
- `Unset` = `0`
- `Tiny` = `1`
- `Small` = `2`
- `Medium` = `3`
- `Large` = `4`

