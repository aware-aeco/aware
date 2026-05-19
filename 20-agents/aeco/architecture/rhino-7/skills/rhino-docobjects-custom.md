---
name: rhino-rhino-docobjects-custom
description: This skill encodes the rhino 7.0 surface of the Rhino.DocObjects.Custom namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ClassIdAttribute, CustomBrepObject, CustomCurveObject, CustomGripObject, CustomMeshObject, CustomObjectGrips, CustomPointObject, GripsDrawEventArgs, and 7 more types.
---

# Rhino.DocObjects.Custom

Auto-generated from vendor docs for rhino 7.0. 15 types in this namespace.

## ClassIdAttribute (class)

Useful for legacy UserData

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_ClassIdAttribute.htm)

### Constructors
- `public ClassIdAttribute(string id)` — Initializes a class id attribute.

### Properties
- `Id` (Guid, get) — Gets the associated style.

## CustomBrepObject (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.CustomBrepObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_CustomBrepObject.htm)

### Constructors
- `protected CustomBrepObject()` — Initializes a new instance of the CustomBrepObject class
- `protected CustomBrepObject(Brep brep)` — Initializes a new instance of the CustomBrepObject class

### Methods
#### `public void ClearId()`

Resets the HasId property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearId.htm)

#### `public void ClearIndex()`

Resets the HasIndex property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearIndex.htm)

#### `public void ClearName()`

Resets the HasName property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearName.htm)

#### `public bool CommitChanges()`

Moves changes made to this RhinoObject into the RhinoDoc.

**Returns:** `Boolean` — true if changes were made.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CommitChanges.htm)

#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public bool CopyHistoryOnReplace()`

Gets the setting of the CopyOnReplace field in this object's history

**Returns:** `Boolean` — true if this object has history and the field is set false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CopyHistoryOnReplace.htm)

#### `public virtual int CreateMeshes(MeshType meshType, MeshingParameters parameters, bool ignoreCustomParameters)`

Create meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of meshes to create
- `parameters` (Rhino.Geometry.MeshingParameters) — in parameters that control the quality of the meshes that are created
- `ignoreCustomParameters` (System.Boolean) — Default should be false. Should the object ignore any custom meshing parameters on the object's attributes

**Returns:** `Int32` — number of meshes created

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CreateMeshes.htm)

#### `public uint DataCRC(uint currentRemainder)`

Increments the Cyclic Redundancy Check value by this instance.

**Parameters:**
- `currentRemainder` (System.UInt32) — The current remainder value.

**Returns:** `UInt32` — The updated remainder value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_DataCRC.htm)

#### `public void Description(TextLog textLog)`

Get a brief description of a object, including it's attributes and geometry.

**Parameters:**
- `textLog` (Rhino.FileIO.TextLog) — A text log for collecting the description.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Description.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Dispose.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the CustomBrepObject and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomBrepObject_Dispose.htm)

#### `public Brep DuplicateBrepGeometry()`

Constructs a new deep copy of the Brep geometry.

**Returns:** `Brep` — The copy of the geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_BrepObject_DuplicateBrepGeometry.htm)

#### `public GeometryBase DuplicateGeometry()`

Constructs a deep (full) copy of the geometry.

**Returns:** `GeometryBase` — A copy of the internal geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_DuplicateGeometry.htm)

#### `public bool EnableCustomGrips(CustomObjectGrips customGrips)`

Turns on/off the object's editing grips.

**Parameters:**
- `customGrips` (Rhino.DocObjects.Custom.CustomObjectGrips) — The custom object grips.

**Returns:** `Boolean` — true if the call succeeded. If you attempt to add custom grips to an object that does not support custom grips, then false is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableCustomGrips.htm)

#### `public bool EnableVisualAnalysisMode(VisualAnalysisMode mode, bool enable)`

Used to turn analysis modes on and off.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — A visual analysis mode.
- `enable` (System.Boolean) — true if the mode should be activated; false otherwise.

**Returns:** `Boolean` — true if this object supports the analysis mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableVisualAnalysisMode.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

(Overrides CommonObject.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomBrepObject_Finalize.htm)

#### `public VisualAnalysisMode[] GetActiveVisualAnalysisModes()`

Gets a list of currently enabled analysis modes for this object.

**Returns:** `VisualAnalysisMode[]` — An array of visual analysis modes. The array can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetActiveVisualAnalysisModes.htm)

#### `public virtual IConvertible GetCustomRenderMeshParameter(Guid providerId, string parameterName)`

Query the object for the value of a given named custom render mesh parameter.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — Name of the parameter

**Returns:** `IConvertible` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetCustomRenderMeshParameter.htm)

#### `public bool GetDynamicTransform(out Transform transform)`

While an object is being dynamically transformed (dragged, rotated, ...), the current transformation can be retrieved and used for creating dynamic display.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetDynamicTransform(Rhino.Geometry.Transform@)"]

**Returns:** `Boolean` — True if the object is being edited and its transformation is available. False if the object is not being edited, in which case the identity transform is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetDynamicTransform.htm)

#### `public GripObject[] GetGrips()`

Returns grips for this object If grips are enabled. If grips are not enabled, returns null.

**Returns:** `GripObject[]` — An array of grip objects; or null if there are no grips.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGrips.htm)

#### `public int[] GetGroupList()`

Allocates an array of group indices of length GroupCount. If GroupCount is 0, then this method returns null.

**Returns:** `Int32[]` — An array of group indices, or null if GroupCount is 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGroupList.htm)

#### `public ComponentIndex[] GetHighlightedSubObjects()`

Gets a list of all highlighted sub-objects.

**Returns:** `ComponentIndex[]` — An array of all highlighted sub-objects; or null is there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetHighlightedSubObjects.htm)

#### `public Material GetMaterial(bool frontMaterial)`

Gets material that this object uses based on it's attributes and the document that the object is associated with. In the rare case that a document is not associated with this object, null will be returned.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side

**Returns:** `Material` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetMaterial(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_3.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_1.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_2.htm)

#### `public virtual Mesh[] GetMeshes(MeshType meshType)`

Get existing meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetMeshes(Rhino.Geometry.MeshType)"]

**Returns:** `Mesh[]` — An array of meshes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMeshes.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public RenderMaterial GetRenderMaterial(bool frontMaterial)`

Gets the RenderMaterial that this object uses based on it's attributes and the document that the object is associated with. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side otherwise; gets the material used to render the back side of the object.

**Returns:** `RenderMaterial` — If there is a RenderMaterial associated with this objects' associated Material then it is returned otherwise; null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_3.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_1.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_2.htm)

#### `public MeshingParameters GetRenderMeshParameters()`

Returns the meshing parameters that this object uses for generating render meshes. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned.

**Returns:** `MeshingParameters` — The render meshing parameters.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters.htm)

#### `public MeshingParameters GetRenderMeshParameters(bool returnDocumentParametersIfUnset)`

Returns the meshing parameters that this object uses for generating render meshes.

**Parameters:**
- `returnDocumentParametersIfUnset` (System.Boolean) — If true, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned. If false, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then null is returned.

**Returns:** `MeshingParameters` — The render meshing parameters if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters_1.htm)

#### `[ObsoleteAttribute] public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList_1.htm)

#### `public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList.htm)

#### `public ComponentIndex[] GetSelectedSubObjects()`

Get a list of all selected sub-objects.

**Returns:** `ComponentIndex[]` — An array of sub-object indices, or null if there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSelectedSubObjects.htm)

#### `public RhinoObject[] GetSubObjects()`

Explodes the object into sub-objects. It is up to the caller to add the returned objects to the document.

**Returns:** `RhinoObject[]` — An array of Rhino objects, or null if this object cannot be exploded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSubObjects.htm)

#### `public int[] GetTextureChannels()`

Get a list of the texture mapping channel Id's associated with object.

**Returns:** `Int32[]` — Returns an array of channel Id's or an empty list if there are not mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureChannels.htm)

#### `public TextureMapping GetTextureMapping(int channel)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping.htm)

#### `public TextureMapping GetTextureMapping(int channel, out Transform objectTransform)`

Get objects texture mapping

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]
- `objectTransform` (Rhino.Geometry.Transform) — [Missing <param name="objectTransform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping_1.htm)

#### `public bool HasHistoryRecord()`

Returns whether this object has a history record

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasHistoryRecord"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasHistoryRecord.htm)

#### `public bool HasTextureMapping()`

Returns true if this object has a texture mapping form any source (pluginId)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasTextureMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasTextureMapping.htm)

#### `public bool Highlight(bool enable)`

Modifies the highlighting of the object.

**Parameters:**
- `enable` (System.Boolean) — true if highlighting should be enabled.

**Returns:** `Boolean` — true if the object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Highlight.htm)

#### `public bool HighlightSubObject(ComponentIndex componentIndex, bool highlight)`

Highlights a sub-object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.
- `highlight` (System.Boolean) — true if the sub-object should be highlighted.

**Returns:** `Boolean` — true if the sub-object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HighlightSubObject.htm)

#### `public bool InVisualAnalysisMode()`

Reports if any visual analysis mode is currently active for an object.

**Returns:** `Boolean` — true if an analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode.htm)

#### `public bool InVisualAnalysisMode(VisualAnalysisMode mode)`

Reports if a visual analysis mode is currently active for an object.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — The mode to check for. Use null if you want to see if any mode is active.

**Returns:** `Boolean` — true if the specified analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode_1.htm)

#### `public virtual bool IsActiveInViewport(RhinoViewport viewport)`

Determine if this object is active in a particular viewport.

**Remarks:** The default implementation tests for space and viewport id. This handles things like testing if a page space object is visible in a modeling view.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsActiveInViewport(Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — True if the object is active in viewport

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsActiveInViewport.htm)

#### `public int IsHighlighted(bool checkSubObjects)`

Check highlight state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — If true and the entire object is not highlighted, and some subset of the object is highlighted, like some edges of a surface, then 3 is returned. If false and the entire object is not highlighted, then zero is returned.

**Returns:** `Int32` — 0: object is not highlighted.1: entire object is highlighted.3: one or more proper sub-objects are highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsHighlighted.htm)

#### `public virtual bool IsMeshable(MeshType meshType)`

Returns true if the object is capable of having a mesh of the specified type

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsMeshable.htm)

#### `public bool IsSelectable()`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable.htm)

#### `public bool IsSelectable(bool ignoreSelectionState, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers are selectable. If false, then objects on locked layers are not selectable.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers are selectable. If false, then objects on hidden layers are not selectable.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable_1.htm)

#### `public int IsSelected(bool checkSubObjects)`

Check selection state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — (false is good default) If true and the entire object is not selected, and some subset of the object is selected, like some edges of a surface, then 3 is returned. If false and the entire object is not selected, then zero is returned.

**Returns:** `Int32` — 0 = object is not selected. 1 = object is selected. 2 = entire object is selected persistently. 3 = one or more proper sub-objects are selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelected.htm)

#### `public bool IsSubObjectHighlighted(ComponentIndex componentIndex)`

Determines if a sub-object is highlighted.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.

**Returns:** `Boolean` — true if the sub-object is highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectHighlighted.htm)

#### `public bool IsSubObjectSelectable(ComponentIndex componentIndex, bool ignoreSelectionState)`

Reports if a sub-object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — index of sub-object to check.
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.

**Returns:** `Boolean` — true if the specified sub-object can be selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelectable.htm)

#### `public bool IsSubObjectSelected(ComponentIndex componentIndex)`

Check sub-object selection state.

**Remarks:** A sub-object cannot be persistently selected.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.

**Returns:** `Boolean` — true if the sub-object is selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelected.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `public void LockId()`

Locks the component Id property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockId.htm)

#### `public void LockIndex()`

Locks the component Index property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockIndex.htm)

#### `public void LockName()`

Locks the component Name property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockName.htm)

#### `public uint MemoryEstimate()`

Computes an estimate of the number of bytes that this object is using in memory. Note that this is a runtime memory estimate and does not directly compare to the amount of space take up by the object when saved to a file.

**Returns:** `UInt32` — The estimated number of bytes this object occupies in memory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MemoryEstimate.htm)

#### `public virtual int MeshCount(MeshType meshType, MeshingParameters parameters)`

RhinoObjects can have several different types of meshes and different numbers of meshes. A b-rep can have a render and an analysis mesh on each face. A mesh object has a single render mesh and no analysis mesh. Curve, point, and annotation objects have no meshes.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of mesh to count
- `parameters` (Rhino.Geometry.MeshingParameters) — if not null and if the object can change its mesh (like a brep), then only meshes that were created with these mesh parameters are counted.

**Returns:** `Int32` — number of meshes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MeshCount.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnAddToDocument(RhinoDoc doc)`

This call informs an object it is about to be added to the list of active objects in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnAddToDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnAddToDocument.htm)

#### `protected virtual void OnDeleteFromDocument(RhinoDoc doc)`

This call informs an object it is about to be deleted. Some objects, like clipping planes, need to do a little extra cleanup before they are deleted.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDeleteFromDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDeleteFromDocument.htm)

#### `protected virtual void OnDraw(DrawEventArgs e)`

Called when Rhino wants to draw this object

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDraw(Rhino.Display.DrawEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDraw.htm)

#### `protected virtual void OnDuplicate(RhinoObject source)`

Called when this a new instance of this object is created and copied from an existing object

**Parameters:**
- `source` (Rhino.DocObjects.RhinoObject) — [Missing <param name="source"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDuplicate(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDuplicate.htm)

#### `protected virtual IEnumerable<ObjRef> OnPick(PickContext context)`

Called to determine if this object or some sub-portion of this object should be picked given a pick context.

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

**Returns:** `IEnumerable<ObjRef>` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPick.htm)

#### `protected virtual void OnPicked(PickContext context, IEnumerable<ObjRef> pickedItems)`

Called when this object has been picked

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPicked(Rhino.Input.Custom.PickContext,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]
- `pickedItems` (System.Collections.Generic.IEnumerable<ObjRef>) — Items that were picked. This parameter is enumerable because there may have been multiple sub-objects picked

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPicked.htm)

#### `protected virtual void OnSelectionChanged()`

Called when the selection state of this object has changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSelectionChanged.htm)

#### `protected virtual void OnSpaceMorph(SpaceMorph morph)`

Called when a space morph has been applied to the geometry. Currently this only works for CustomMeshObject instances

**Parameters:**
- `morph` (Rhino.Geometry.SpaceMorph) — [Missing <param name="morph"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnSpaceMorph(Rhino.Geometry.SpaceMorph)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSpaceMorph.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `protected virtual void OnTransform(Transform transform)`

Called when a transformation has been applied to the geometry

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnTransform.htm)

#### `public int Select(bool on)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select.htm)

#### `public int Select(bool on, bool syncHighlight)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and not highlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_1.htm)

#### `public int Select(bool on, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected. If false, then objects on locked layers cannot be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable. If false, then objects on hidden layers cannot be selected.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_2.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Int32` — 0: object is not selected 1: object is selected 2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight, bool persistentSelect)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — When true, selection persists even after the current command terminates.

**Returns:** `Int32` — 0: object is not selected1: object is selected2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject_1.htm)

#### `public void SetCopyHistoryOnReplace(bool bCopy)`

If this object has a history record, the CopyOnReplace field is set When an object is replaced in a document and the old object has a history record with this field set, the history record is copied and attached to the new object. That allows a descendant object to continue the history linkage after it is edited.

**Parameters:**
- `bCopy` (System.Boolean) — [Missing <param name="bCopy"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCopyHistoryOnReplace(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCopyHistoryOnReplace.htm)

#### `public virtual void SetCustomRenderMeshParameter(Guid providerId, string parameterName, Object value)`

Set the named custom render mesh parameter value for this object.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCustomRenderMeshParameter.htm)

#### `public bool SetRenderMeshParameters(MeshingParameters mp)`

Sets the per-object meshing parameters for this object. When set, this object will use these meshing parameters when generating a render mesh, instead of those provided by the document.

**Parameters:**
- `mp` (Rhino.Geometry.MeshingParameters) — The per-object meshing parameters. Note: if null, then the per-object meshing parameters will be removed, and this object will revert to using the meshing parameters provided by the document.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetRenderMeshParameters.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm, Transform objectTransform)`

Sets texture mapping and mapping object transform for a channel

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `objectTransform` (Rhino.Geometry.Transform) — Mapping channel object transform

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping_1.htm)

#### `public virtual string ShortDescription(bool plural)`

Gets a localized short descriptive name of the object.

**Parameters:**
- `plural` (System.Boolean) — true if the descriptive name should in plural.

**Returns:** `String` — A string with the short localized descriptive name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_ShortDescription.htm)

#### `[ObsoleteAttribute] public bool SupportsRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList_1.htm)

#### `public bool SupportsRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

#### `public override string ToString()`

Returns the name of the model component type, and then its name and index.

**Remarks:** This is provided as a visual aid. Do not rely on this for serialization.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.ModelComponent.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ToString.htm)

#### `public bool TryGetGumballFrame(out GumballFrame frame)`

If a Rhino object has been manipulated by Rhino's gumball, and the gumball is not in its default position, then the object's repositioned gumball frame is returned.

**Parameters:**
- `frame` (Rhino.UI.Gumball.GumballFrame) — The gumball frame.

**Returns:** `Boolean` — true if the object has a gumball frame, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetGumballFrame.htm)

#### `[ObsoleteAttribute] public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, bool preview, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox_1.htm)

#### `public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox.htm)

#### `public int UnhighlightAllSubObjects()`

Removes highlighting from all sub-objects.

**Returns:** `Int32` — The number of changed sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnhighlightAllSubObjects.htm)

#### `public int UnselectAllSubObjects()`

Removes selection from all sub-objects.

**Returns:** `Int32` — The number of unselected sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnselectAllSubObjects.htm)

### Properties
- `Attributes` (ObjectAttributes, get/set) — Gets or sets the object attributes.
- `BrepGeometry` (Brep, get) — Gets the Brep geometry linked with this object.
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Document` (RhinoDoc, get) — Gets the document that owns this object.
- `Geometry` (GeometryBase, get) — Gets the underlying geometry for this object. All rhino objects are composed of geometry and attributes.
- `GripsOn` (Boolean, get/set) — Gets or sets the activation state of object default editing grips.
- `GripsSelected` (Boolean, get) — true if grips are turned on and at least one is selected.
- `GroupCount` (Int32, get) — Number of groups object belongs to.
- `HasDynamicTransform` (Boolean, get) — True if the object has a dynamic transformation
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasSubobjectMaterials` (Boolean, get) — Will be true if the object contains sub object meshes with materials that are different than the top level object.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get/set) — Every object has a Guid (globally unique identifier, also known as UUID, or universally unique identifier). The default value is Guid.Empty. When an object is added to a model, the value is checked. If the value is Guid.Empty, a new Guid is created. If the value is not null but it is already used by another object in the model, a new Guid is created. If the value is not Guid.Empty and it is not used by another object in the model, then that value persists. When an object is updated, by a move for example, the value of ObjectId persists. This value is the same as the one returned by this.Attributes.ObjectId.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — Gets or sets the model component index attribute.
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsDeletable` (Boolean, get) — Some objects cannot be deleted, like grips on lights and annotation objects.
- `IsDeleted` (Boolean, get) — true if the object is deleted. Deleted objects are kept by the document for undo purposes. Call RhinoDoc.UndeleteObject to undelete an object.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsHidden` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsInstanceDefinitionGeometry` (Boolean, get) — true if the object is used as part of an instance definition.
- `IsLocked` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsNormal` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsPictureFrame` (Boolean, get) — Returns true if the object is a picture frame. A picture frame object is an object that displays a texture map in all views.
- `IsReference` (Boolean, get) — Gets a value indicating if an object is a reference object. An object from a work session reference model is a reference object and cannot be modified. An object is a reference object if, and only if, it is on a reference layer.
- `IsSolid` (Boolean, get) — Returns true if object is a closed solid, otherwise false.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Name` (String, get/set) — Rhino objects have optional text names. More than one object in a model can have the same name and some objects may have no name.
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `ObjectType` (ObjectType, get) — Gets the Rhino-based object type.
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `RenderMaterial` (RenderMaterial, get/set) — Gets the render material associated with this object or null if there is none. This does not pay attention to the material source and will not check parent objects or layers for a RenderMaterial.
- `RuntimeSerialNumber` (UInt32, get) — Gets the objects runtime serial number.
- `SubobjectMaterialComponents` (ComponentIndex[], get) — (Inherited from RhinoObject.)
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `Visible` (Boolean, get) — Gets the object visibility.
- `WorksessionReferenceSerialNumber` (UInt32, get) — Obsolete - use ReferenceModelSerialNumber

## CustomCurveObject (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.CustomCurveObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_CustomCurveObject.htm)

### Constructors
- `protected CustomCurveObject()` — Initializes a new instance of the CustomCurveObject class
- `protected CustomCurveObject(Curve curve)` — Initializes a new instance of the CustomCurveObject class

### Methods
#### `public void ClearId()`

Resets the HasId property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearId.htm)

#### `public void ClearIndex()`

Resets the HasIndex property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearIndex.htm)

#### `public void ClearName()`

Resets the HasName property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearName.htm)

#### `public bool CommitChanges()`

Moves changes made to this RhinoObject into the RhinoDoc.

**Returns:** `Boolean` — true if changes were made.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CommitChanges.htm)

#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public bool CopyHistoryOnReplace()`

Gets the setting of the CopyOnReplace field in this object's history

**Returns:** `Boolean` — true if this object has history and the field is set false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CopyHistoryOnReplace.htm)

#### `public virtual int CreateMeshes(MeshType meshType, MeshingParameters parameters, bool ignoreCustomParameters)`

Create meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of meshes to create
- `parameters` (Rhino.Geometry.MeshingParameters) — in parameters that control the quality of the meshes that are created
- `ignoreCustomParameters` (System.Boolean) — Default should be false. Should the object ignore any custom meshing parameters on the object's attributes

**Returns:** `Int32` — number of meshes created

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CreateMeshes.htm)

#### `public uint DataCRC(uint currentRemainder)`

Increments the Cyclic Redundancy Check value by this instance.

**Parameters:**
- `currentRemainder` (System.UInt32) — The current remainder value.

**Returns:** `UInt32` — The updated remainder value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_DataCRC.htm)

#### `public void Description(TextLog textLog)`

Get a brief description of a object, including it's attributes and geometry.

**Parameters:**
- `textLog` (Rhino.FileIO.TextLog) — A text log for collecting the description.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Description.htm)

#### `public void Dispose()`

Releases all resources used by the CustomCurveObject

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomCurveObject_Dispose.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the CustomCurveObject and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomCurveObject_Dispose_1.htm)

#### `public Curve DuplicateCurveGeometry()`

Returns a copy of the underlying curve geometry.

**Returns:** `Curve` — [Missing <returns> documentation for "M:Rhino.DocObjects.CurveObject.DuplicateCurveGeometry"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_CurveObject_DuplicateCurveGeometry.htm)

#### `public GeometryBase DuplicateGeometry()`

Constructs a deep (full) copy of the geometry.

**Returns:** `GeometryBase` — A copy of the internal geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_DuplicateGeometry.htm)

#### `public bool EnableCustomGrips(CustomObjectGrips customGrips)`

Turns on/off the object's editing grips.

**Parameters:**
- `customGrips` (Rhino.DocObjects.Custom.CustomObjectGrips) — The custom object grips.

**Returns:** `Boolean` — true if the call succeeded. If you attempt to add custom grips to an object that does not support custom grips, then false is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableCustomGrips.htm)

#### `public bool EnableVisualAnalysisMode(VisualAnalysisMode mode, bool enable)`

Used to turn analysis modes on and off.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — A visual analysis mode.
- `enable` (System.Boolean) — true if the mode should be activated; false otherwise.

**Returns:** `Boolean` — true if this object supports the analysis mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableVisualAnalysisMode.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

(Overrides CommonObject.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomCurveObject_Finalize.htm)

#### `public VisualAnalysisMode[] GetActiveVisualAnalysisModes()`

Gets a list of currently enabled analysis modes for this object.

**Returns:** `VisualAnalysisMode[]` — An array of visual analysis modes. The array can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetActiveVisualAnalysisModes.htm)

#### `public virtual IConvertible GetCustomRenderMeshParameter(Guid providerId, string parameterName)`

Query the object for the value of a given named custom render mesh parameter.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — Name of the parameter

**Returns:** `IConvertible` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetCustomRenderMeshParameter.htm)

#### `public bool GetDynamicTransform(out Transform transform)`

While an object is being dynamically transformed (dragged, rotated, ...), the current transformation can be retrieved and used for creating dynamic display.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetDynamicTransform(Rhino.Geometry.Transform@)"]

**Returns:** `Boolean` — True if the object is being edited and its transformation is available. False if the object is not being edited, in which case the identity transform is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetDynamicTransform.htm)

#### `public GripObject[] GetGrips()`

Returns grips for this object If grips are enabled. If grips are not enabled, returns null.

**Returns:** `GripObject[]` — An array of grip objects; or null if there are no grips.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGrips.htm)

#### `public int[] GetGroupList()`

Allocates an array of group indices of length GroupCount. If GroupCount is 0, then this method returns null.

**Returns:** `Int32[]` — An array of group indices, or null if GroupCount is 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGroupList.htm)

#### `public ComponentIndex[] GetHighlightedSubObjects()`

Gets a list of all highlighted sub-objects.

**Returns:** `ComponentIndex[]` — An array of all highlighted sub-objects; or null is there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetHighlightedSubObjects.htm)

#### `public Material GetMaterial(bool frontMaterial)`

Gets material that this object uses based on it's attributes and the document that the object is associated with. In the rare case that a document is not associated with this object, null will be returned.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side

**Returns:** `Material` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetMaterial(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_3.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_1.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_2.htm)

#### `public virtual Mesh[] GetMeshes(MeshType meshType)`

Get existing meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetMeshes(Rhino.Geometry.MeshType)"]

**Returns:** `Mesh[]` — An array of meshes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMeshes.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public RenderMaterial GetRenderMaterial(bool frontMaterial)`

Gets the RenderMaterial that this object uses based on it's attributes and the document that the object is associated with. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side otherwise; gets the material used to render the back side of the object.

**Returns:** `RenderMaterial` — If there is a RenderMaterial associated with this objects' associated Material then it is returned otherwise; null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_3.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_1.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_2.htm)

#### `public MeshingParameters GetRenderMeshParameters()`

Returns the meshing parameters that this object uses for generating render meshes. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned.

**Returns:** `MeshingParameters` — The render meshing parameters.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters.htm)

#### `public MeshingParameters GetRenderMeshParameters(bool returnDocumentParametersIfUnset)`

Returns the meshing parameters that this object uses for generating render meshes.

**Parameters:**
- `returnDocumentParametersIfUnset` (System.Boolean) — If true, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned. If false, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then null is returned.

**Returns:** `MeshingParameters` — The render meshing parameters if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters_1.htm)

#### `[ObsoleteAttribute] public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList_1.htm)

#### `public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList.htm)

#### `public ComponentIndex[] GetSelectedSubObjects()`

Get a list of all selected sub-objects.

**Returns:** `ComponentIndex[]` — An array of sub-object indices, or null if there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSelectedSubObjects.htm)

#### `public RhinoObject[] GetSubObjects()`

Explodes the object into sub-objects. It is up to the caller to add the returned objects to the document.

**Returns:** `RhinoObject[]` — An array of Rhino objects, or null if this object cannot be exploded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSubObjects.htm)

#### `public int[] GetTextureChannels()`

Get a list of the texture mapping channel Id's associated with object.

**Returns:** `Int32[]` — Returns an array of channel Id's or an empty list if there are not mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureChannels.htm)

#### `public TextureMapping GetTextureMapping(int channel)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping.htm)

#### `public TextureMapping GetTextureMapping(int channel, out Transform objectTransform)`

Get objects texture mapping

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]
- `objectTransform` (Rhino.Geometry.Transform) — [Missing <param name="objectTransform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping_1.htm)

#### `public bool HasHistoryRecord()`

Returns whether this object has a history record

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasHistoryRecord"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasHistoryRecord.htm)

#### `public bool HasTextureMapping()`

Returns true if this object has a texture mapping form any source (pluginId)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasTextureMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasTextureMapping.htm)

#### `public bool Highlight(bool enable)`

Modifies the highlighting of the object.

**Parameters:**
- `enable` (System.Boolean) — true if highlighting should be enabled.

**Returns:** `Boolean` — true if the object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Highlight.htm)

#### `public bool HighlightSubObject(ComponentIndex componentIndex, bool highlight)`

Highlights a sub-object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.
- `highlight` (System.Boolean) — true if the sub-object should be highlighted.

**Returns:** `Boolean` — true if the sub-object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HighlightSubObject.htm)

#### `public bool InVisualAnalysisMode()`

Reports if any visual analysis mode is currently active for an object.

**Returns:** `Boolean` — true if an analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode.htm)

#### `public bool InVisualAnalysisMode(VisualAnalysisMode mode)`

Reports if a visual analysis mode is currently active for an object.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — The mode to check for. Use null if you want to see if any mode is active.

**Returns:** `Boolean` — true if the specified analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode_1.htm)

#### `public virtual bool IsActiveInViewport(RhinoViewport viewport)`

Determine if this object is active in a particular viewport.

**Remarks:** The default implementation tests for space and viewport id. This handles things like testing if a page space object is visible in a modeling view.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsActiveInViewport(Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — True if the object is active in viewport

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsActiveInViewport.htm)

#### `public int IsHighlighted(bool checkSubObjects)`

Check highlight state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — If true and the entire object is not highlighted, and some subset of the object is highlighted, like some edges of a surface, then 3 is returned. If false and the entire object is not highlighted, then zero is returned.

**Returns:** `Int32` — 0: object is not highlighted.1: entire object is highlighted.3: one or more proper sub-objects are highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsHighlighted.htm)

#### `public virtual bool IsMeshable(MeshType meshType)`

Returns true if the object is capable of having a mesh of the specified type

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsMeshable.htm)

#### `public bool IsSelectable()`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable.htm)

#### `public bool IsSelectable(bool ignoreSelectionState, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers are selectable. If false, then objects on locked layers are not selectable.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers are selectable. If false, then objects on hidden layers are not selectable.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable_1.htm)

#### `public int IsSelected(bool checkSubObjects)`

Check selection state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — (false is good default) If true and the entire object is not selected, and some subset of the object is selected, like some edges of a surface, then 3 is returned. If false and the entire object is not selected, then zero is returned.

**Returns:** `Int32` — 0 = object is not selected. 1 = object is selected. 2 = entire object is selected persistently. 3 = one or more proper sub-objects are selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelected.htm)

#### `public bool IsSubObjectHighlighted(ComponentIndex componentIndex)`

Determines if a sub-object is highlighted.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.

**Returns:** `Boolean` — true if the sub-object is highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectHighlighted.htm)

#### `public bool IsSubObjectSelectable(ComponentIndex componentIndex, bool ignoreSelectionState)`

Reports if a sub-object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — index of sub-object to check.
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.

**Returns:** `Boolean` — true if the specified sub-object can be selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelectable.htm)

#### `public bool IsSubObjectSelected(ComponentIndex componentIndex)`

Check sub-object selection state.

**Remarks:** A sub-object cannot be persistently selected.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.

**Returns:** `Boolean` — true if the sub-object is selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelected.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `public void LockId()`

Locks the component Id property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockId.htm)

#### `public void LockIndex()`

Locks the component Index property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockIndex.htm)

#### `public void LockName()`

Locks the component Name property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockName.htm)

#### `public uint MemoryEstimate()`

Computes an estimate of the number of bytes that this object is using in memory. Note that this is a runtime memory estimate and does not directly compare to the amount of space take up by the object when saved to a file.

**Returns:** `UInt32` — The estimated number of bytes this object occupies in memory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MemoryEstimate.htm)

#### `public virtual int MeshCount(MeshType meshType, MeshingParameters parameters)`

RhinoObjects can have several different types of meshes and different numbers of meshes. A b-rep can have a render and an analysis mesh on each face. A mesh object has a single render mesh and no analysis mesh. Curve, point, and annotation objects have no meshes.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of mesh to count
- `parameters` (Rhino.Geometry.MeshingParameters) — if not null and if the object can change its mesh (like a brep), then only meshes that were created with these mesh parameters are counted.

**Returns:** `Int32` — number of meshes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MeshCount.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnAddToDocument(RhinoDoc doc)`

This call informs an object it is about to be added to the list of active objects in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnAddToDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnAddToDocument.htm)

#### `protected virtual void OnDeleteFromDocument(RhinoDoc doc)`

This call informs an object it is about to be deleted. Some objects, like clipping planes, need to do a little extra cleanup before they are deleted.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDeleteFromDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDeleteFromDocument.htm)

#### `protected virtual void OnDraw(DrawEventArgs e)`

Called when Rhino wants to draw this object

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDraw(Rhino.Display.DrawEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDraw.htm)

#### `protected virtual void OnDuplicate(RhinoObject source)`

Called when this a new instance of this object is created and copied from an existing object

**Parameters:**
- `source` (Rhino.DocObjects.RhinoObject) — [Missing <param name="source"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDuplicate(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDuplicate.htm)

#### `protected virtual IEnumerable<ObjRef> OnPick(PickContext context)`

Called to determine if this object or some sub-portion of this object should be picked given a pick context.

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

**Returns:** `IEnumerable<ObjRef>` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPick.htm)

#### `protected virtual void OnPicked(PickContext context, IEnumerable<ObjRef> pickedItems)`

Called when this object has been picked

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPicked(Rhino.Input.Custom.PickContext,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]
- `pickedItems` (System.Collections.Generic.IEnumerable<ObjRef>) — Items that were picked. This parameter is enumerable because there may have been multiple sub-objects picked

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPicked.htm)

#### `protected virtual void OnSelectionChanged()`

Called when the selection state of this object has changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSelectionChanged.htm)

#### `protected virtual void OnSpaceMorph(SpaceMorph morph)`

Called when a space morph has been applied to the geometry. Currently this only works for CustomMeshObject instances

**Parameters:**
- `morph` (Rhino.Geometry.SpaceMorph) — [Missing <param name="morph"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnSpaceMorph(Rhino.Geometry.SpaceMorph)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSpaceMorph.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `protected virtual void OnTransform(Transform transform)`

Called when a transformation has been applied to the geometry

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnTransform.htm)

#### `public int Select(bool on)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select.htm)

#### `public int Select(bool on, bool syncHighlight)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and not highlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_1.htm)

#### `public int Select(bool on, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected. If false, then objects on locked layers cannot be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable. If false, then objects on hidden layers cannot be selected.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_2.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Int32` — 0: object is not selected 1: object is selected 2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight, bool persistentSelect)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — When true, selection persists even after the current command terminates.

**Returns:** `Int32` — 0: object is not selected1: object is selected2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject_1.htm)

#### `public void SetCopyHistoryOnReplace(bool bCopy)`

If this object has a history record, the CopyOnReplace field is set When an object is replaced in a document and the old object has a history record with this field set, the history record is copied and attached to the new object. That allows a descendant object to continue the history linkage after it is edited.

**Parameters:**
- `bCopy` (System.Boolean) — [Missing <param name="bCopy"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCopyHistoryOnReplace(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCopyHistoryOnReplace.htm)

#### `protected Curve SetCurve(Curve curve)`

Only for developers who are defining custom subclasses of CurveObject. Directly sets the internal curve geometry for this object. Note that this function does not work with Rhino's "undo".

**Remarks:** Note that this function does not work with Rhino's "undo". The typical approach for adjusting the curve geometry is to modify the object that you get when you call the CurveGeometry property and then call CommitChanges.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — [Missing <param name="curve"/> documentation for "M:Rhino.DocObjects.Custom.CustomCurveObject.SetCurve(Rhino.Geometry.Curve)"]

**Returns:** `Curve` — The old curve geometry that was set for this object

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomCurveObject_SetCurve.htm)

#### `public virtual void SetCustomRenderMeshParameter(Guid providerId, string parameterName, Object value)`

Set the named custom render mesh parameter value for this object.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCustomRenderMeshParameter.htm)

#### `public bool SetRenderMeshParameters(MeshingParameters mp)`

Sets the per-object meshing parameters for this object. When set, this object will use these meshing parameters when generating a render mesh, instead of those provided by the document.

**Parameters:**
- `mp` (Rhino.Geometry.MeshingParameters) — The per-object meshing parameters. Note: if null, then the per-object meshing parameters will be removed, and this object will revert to using the meshing parameters provided by the document.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetRenderMeshParameters.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm, Transform objectTransform)`

Sets texture mapping and mapping object transform for a channel

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `objectTransform` (Rhino.Geometry.Transform) — Mapping channel object transform

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping_1.htm)

#### `public virtual string ShortDescription(bool plural)`

Gets a localized short descriptive name of the object.

**Parameters:**
- `plural` (System.Boolean) — true if the descriptive name should in plural.

**Returns:** `String` — A string with the short localized descriptive name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_ShortDescription.htm)

#### `[ObsoleteAttribute] public bool SupportsRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList_1.htm)

#### `public bool SupportsRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

#### `public override string ToString()`

Returns the name of the model component type, and then its name and index.

**Remarks:** This is provided as a visual aid. Do not rely on this for serialization.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.ModelComponent.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ToString.htm)

#### `public bool TryGetGumballFrame(out GumballFrame frame)`

If a Rhino object has been manipulated by Rhino's gumball, and the gumball is not in its default position, then the object's repositioned gumball frame is returned.

**Parameters:**
- `frame` (Rhino.UI.Gumball.GumballFrame) — The gumball frame.

**Returns:** `Boolean` — true if the object has a gumball frame, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetGumballFrame.htm)

#### `[ObsoleteAttribute] public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, bool preview, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox_1.htm)

#### `public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox.htm)

#### `public int UnhighlightAllSubObjects()`

Removes highlighting from all sub-objects.

**Returns:** `Int32` — The number of changed sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnhighlightAllSubObjects.htm)

#### `public int UnselectAllSubObjects()`

Removes selection from all sub-objects.

**Returns:** `Int32` — The number of unselected sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnselectAllSubObjects.htm)

### Properties
- `Attributes` (ObjectAttributes, get/set) — Gets or sets the object attributes.
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `CurveGeometry` (Curve, get) — Returns the underlying curve geometry.
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Document` (RhinoDoc, get) — Gets the document that owns this object.
- `Geometry` (GeometryBase, get) — Gets the underlying geometry for this object. All rhino objects are composed of geometry and attributes.
- `GripsOn` (Boolean, get/set) — Gets or sets the activation state of object default editing grips.
- `GripsSelected` (Boolean, get) — true if grips are turned on and at least one is selected.
- `GroupCount` (Int32, get) — Number of groups object belongs to.
- `HasDynamicTransform` (Boolean, get) — True if the object has a dynamic transformation
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasSubobjectMaterials` (Boolean, get) — Will be true if the object contains sub object meshes with materials that are different than the top level object.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get/set) — Every object has a Guid (globally unique identifier, also known as UUID, or universally unique identifier). The default value is Guid.Empty. When an object is added to a model, the value is checked. If the value is Guid.Empty, a new Guid is created. If the value is not null but it is already used by another object in the model, a new Guid is created. If the value is not Guid.Empty and it is not used by another object in the model, then that value persists. When an object is updated, by a move for example, the value of ObjectId persists. This value is the same as the one returned by this.Attributes.ObjectId.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — Gets or sets the model component index attribute.
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsDeletable` (Boolean, get) — Some objects cannot be deleted, like grips on lights and annotation objects.
- `IsDeleted` (Boolean, get) — true if the object is deleted. Deleted objects are kept by the document for undo purposes. Call RhinoDoc.UndeleteObject to undelete an object.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsHidden` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsInstanceDefinitionGeometry` (Boolean, get) — true if the object is used as part of an instance definition.
- `IsLocked` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsNormal` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsPictureFrame` (Boolean, get) — Returns true if the object is a picture frame. A picture frame object is an object that displays a texture map in all views.
- `IsReference` (Boolean, get) — Gets a value indicating if an object is a reference object. An object from a work session reference model is a reference object and cannot be modified. An object is a reference object if, and only if, it is on a reference layer.
- `IsSolid` (Boolean, get) — Returns true if object is a closed solid, otherwise false.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Name` (String, get/set) — Rhino objects have optional text names. More than one object in a model can have the same name and some objects may have no name.
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `ObjectType` (ObjectType, get) — Gets the Rhino-based object type.
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `RenderMaterial` (RenderMaterial, get/set) — Gets the render material associated with this object or null if there is none. This does not pay attention to the material source and will not check parent objects or layers for a RenderMaterial.
- `RuntimeSerialNumber` (UInt32, get) — Gets the objects runtime serial number.
- `SubobjectMaterialComponents` (ComponentIndex[], get) — (Inherited from RhinoObject.)
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `Visible` (Boolean, get) — Gets the object visibility.
- `WorksessionReferenceSerialNumber` (UInt32, get) — Obsolete - use ReferenceModelSerialNumber

## CustomGripObject (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.CustomGripObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_CustomGripObject.htm)

### Constructors
- `public CustomGripObject()` — Initializes a new instance of the CustomGripObject class

### Methods
#### `public void ClearId()`

Resets the HasId property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearId.htm)

#### `public void ClearIndex()`

Resets the HasIndex property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearIndex.htm)

#### `public void ClearName()`

Resets the HasName property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearName.htm)

#### `public bool CommitChanges()`

Moves changes made to this RhinoObject into the RhinoDoc.

**Returns:** `Boolean` — true if changes were made.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CommitChanges.htm)

#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public bool CopyHistoryOnReplace()`

Gets the setting of the CopyOnReplace field in this object's history

**Returns:** `Boolean` — true if this object has history and the field is set false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CopyHistoryOnReplace.htm)

#### `public virtual int CreateMeshes(MeshType meshType, MeshingParameters parameters, bool ignoreCustomParameters)`

Create meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of meshes to create
- `parameters` (Rhino.Geometry.MeshingParameters) — in parameters that control the quality of the meshes that are created
- `ignoreCustomParameters` (System.Boolean) — Default should be false. Should the object ignore any custom meshing parameters on the object's attributes

**Returns:** `Int32` — number of meshes created

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CreateMeshes.htm)

#### `public uint DataCRC(uint currentRemainder)`

Increments the Cyclic Redundancy Check value by this instance.

**Parameters:**
- `currentRemainder` (System.UInt32) — The current remainder value.

**Returns:** `UInt32` — The updated remainder value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_DataCRC.htm)

#### `public void Description(TextLog textLog)`

Get a brief description of a object, including it's attributes and geometry.

**Parameters:**
- `textLog` (Rhino.FileIO.TextLog) — A text log for collecting the description.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Description.htm)

#### `public void Dispose()`

Releases all resources used by the CustomGripObject

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomGripObject_Dispose.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the CustomGripObject and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomGripObject_Dispose_1.htm)

#### `public GeometryBase DuplicateGeometry()`

Constructs a deep (full) copy of the geometry.

**Returns:** `GeometryBase` — A copy of the internal geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_DuplicateGeometry.htm)

#### `public bool EnableCustomGrips(CustomObjectGrips customGrips)`

Turns on/off the object's editing grips.

**Parameters:**
- `customGrips` (Rhino.DocObjects.Custom.CustomObjectGrips) — The custom object grips.

**Returns:** `Boolean` — true if the call succeeded. If you attempt to add custom grips to an object that does not support custom grips, then false is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableCustomGrips.htm)

#### `public bool EnableVisualAnalysisMode(VisualAnalysisMode mode, bool enable)`

Used to turn analysis modes on and off.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — A visual analysis mode.
- `enable` (System.Boolean) — true if the mode should be activated; false otherwise.

**Returns:** `Boolean` — true if this object supports the analysis mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableVisualAnalysisMode.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

(Overrides CommonObject.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomGripObject_Finalize.htm)

#### `public VisualAnalysisMode[] GetActiveVisualAnalysisModes()`

Gets a list of currently enabled analysis modes for this object.

**Returns:** `VisualAnalysisMode[]` — An array of visual analysis modes. The array can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetActiveVisualAnalysisModes.htm)

#### `public bool GetCageParameters(out double u, out double v, out double w)`

Retrieves the 2d parameter space values of this GripObject from the cage it's associated with.

**Parameters:**
- `u` (System.Double) — [Missing <param name="u"/> documentation for "M:Rhino.DocObjects.GripObject.GetCageParameters(System.Double@,System.Double@,System.Double@)"]
- `v` (System.Double) — [Missing <param name="v"/> documentation for "M:Rhino.DocObjects.GripObject.GetCageParameters(System.Double@,System.Double@,System.Double@)"]
- `w` (System.Double) — [Missing <param name="w"/> documentation for "M:Rhino.DocObjects.GripObject.GetCageParameters(System.Double@,System.Double@,System.Double@)"]

**Returns:** `Boolean` — True on success. Output is unreliable if return is false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_GetCageParameters.htm)

#### `public bool GetCurveParameters(out double t)`

Retrieves the 2d parameter space values of this GripObject from the curve it's associated with.

**Parameters:**
- `t` (System.Double) — [Missing <param name="t"/> documentation for "M:Rhino.DocObjects.GripObject.GetCurveParameters(System.Double@)"]

**Returns:** `Boolean` — True on success. Output is unreliable if return is false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_GetCurveParameters.htm)

#### `public virtual IConvertible GetCustomRenderMeshParameter(Guid providerId, string parameterName)`

Query the object for the value of a given named custom render mesh parameter.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — Name of the parameter

**Returns:** `IConvertible` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetCustomRenderMeshParameter.htm)

#### `public bool GetDynamicTransform(out Transform transform)`

While an object is being dynamically transformed (dragged, rotated, ...), the current transformation can be retrieved and used for creating dynamic display.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetDynamicTransform(Rhino.Geometry.Transform@)"]

**Returns:** `Boolean` — True if the object is being edited and its transformation is available. False if the object is not being edited, in which case the identity transform is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetDynamicTransform.htm)

#### `public bool GetGripDirections(out Vector3d u, out Vector3d v, out Vector3d normal)`

Sometimes grips have directions. These directions can have any length and do not have to be orthogonal.

**Parameters:**
- `u` (Rhino.Geometry.Vector3d) — u direction
- `v` (Rhino.Geometry.Vector3d) — v direction
- `normal` (Rhino.Geometry.Vector3d) — normal direction

**Returns:** `Boolean` — True if the grip has directions.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_GetGripDirections.htm)

#### `public GripObject[] GetGrips()`

Returns grips for this object If grips are enabled. If grips are not enabled, returns null.

**Returns:** `GripObject[]` — An array of grip objects; or null if there are no grips.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGrips.htm)

#### `public int[] GetGroupList()`

Allocates an array of group indices of length GroupCount. If GroupCount is 0, then this method returns null.

**Returns:** `Int32[]` — An array of group indices, or null if GroupCount is 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGroupList.htm)

#### `public ComponentIndex[] GetHighlightedSubObjects()`

Gets a list of all highlighted sub-objects.

**Returns:** `ComponentIndex[]` — An array of all highlighted sub-objects; or null is there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetHighlightedSubObjects.htm)

#### `public Material GetMaterial(bool frontMaterial)`

Gets material that this object uses based on it's attributes and the document that the object is associated with. In the rare case that a document is not associated with this object, null will be returned.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side

**Returns:** `Material` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetMaterial(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_3.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_1.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_2.htm)

#### `public virtual Mesh[] GetMeshes(MeshType meshType)`

Get existing meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetMeshes(Rhino.Geometry.MeshType)"]

**Returns:** `Mesh[]` — An array of meshes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMeshes.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public RenderMaterial GetRenderMaterial(bool frontMaterial)`

Gets the RenderMaterial that this object uses based on it's attributes and the document that the object is associated with. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side otherwise; gets the material used to render the back side of the object.

**Returns:** `RenderMaterial` — If there is a RenderMaterial associated with this objects' associated Material then it is returned otherwise; null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_3.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_1.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_2.htm)

#### `public MeshingParameters GetRenderMeshParameters()`

Returns the meshing parameters that this object uses for generating render meshes. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned.

**Returns:** `MeshingParameters` — The render meshing parameters.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters.htm)

#### `public MeshingParameters GetRenderMeshParameters(bool returnDocumentParametersIfUnset)`

Returns the meshing parameters that this object uses for generating render meshes.

**Parameters:**
- `returnDocumentParametersIfUnset` (System.Boolean) — If true, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned. If false, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then null is returned.

**Returns:** `MeshingParameters` — The render meshing parameters if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters_1.htm)

#### `[ObsoleteAttribute] public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList_1.htm)

#### `public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList.htm)

#### `public ComponentIndex[] GetSelectedSubObjects()`

Get a list of all selected sub-objects.

**Returns:** `ComponentIndex[]` — An array of sub-object indices, or null if there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSelectedSubObjects.htm)

#### `public RhinoObject[] GetSubObjects()`

Explodes the object into sub-objects. It is up to the caller to add the returned objects to the document.

**Returns:** `RhinoObject[]` — An array of Rhino objects, or null if this object cannot be exploded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSubObjects.htm)

#### `public bool GetSurfaceParameters(out double u, out double v)`

Retrieves the 2d parameter space values of this GripObject from the surface it's associated with.

**Parameters:**
- `u` (System.Double) — [Missing <param name="u"/> documentation for "M:Rhino.DocObjects.GripObject.GetSurfaceParameters(System.Double@,System.Double@)"]
- `v` (System.Double) — [Missing <param name="v"/> documentation for "M:Rhino.DocObjects.GripObject.GetSurfaceParameters(System.Double@,System.Double@)"]

**Returns:** `Boolean` — True on success. Output is unreliable if return is false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_GetSurfaceParameters.htm)

#### `public int[] GetTextureChannels()`

Get a list of the texture mapping channel Id's associated with object.

**Returns:** `Int32[]` — Returns an array of channel Id's or an empty list if there are not mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureChannels.htm)

#### `public TextureMapping GetTextureMapping(int channel)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping.htm)

#### `public TextureMapping GetTextureMapping(int channel, out Transform objectTransform)`

Get objects texture mapping

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]
- `objectTransform` (Rhino.Geometry.Transform) — [Missing <param name="objectTransform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping_1.htm)

#### `public bool HasHistoryRecord()`

Returns whether this object has a history record

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasHistoryRecord"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasHistoryRecord.htm)

#### `public bool HasTextureMapping()`

Returns true if this object has a texture mapping form any source (pluginId)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasTextureMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasTextureMapping.htm)

#### `public bool Highlight(bool enable)`

Modifies the highlighting of the object.

**Parameters:**
- `enable` (System.Boolean) — true if highlighting should be enabled.

**Returns:** `Boolean` — true if the object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Highlight.htm)

#### `public bool HighlightSubObject(ComponentIndex componentIndex, bool highlight)`

Highlights a sub-object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.
- `highlight` (System.Boolean) — true if the sub-object should be highlighted.

**Returns:** `Boolean` — true if the sub-object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HighlightSubObject.htm)

#### `public bool InVisualAnalysisMode()`

Reports if any visual analysis mode is currently active for an object.

**Returns:** `Boolean` — true if an analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode.htm)

#### `public bool InVisualAnalysisMode(VisualAnalysisMode mode)`

Reports if a visual analysis mode is currently active for an object.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — The mode to check for. Use null if you want to see if any mode is active.

**Returns:** `Boolean` — true if the specified analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode_1.htm)

#### `public virtual bool IsActiveInViewport(RhinoViewport viewport)`

Determine if this object is active in a particular viewport.

**Remarks:** The default implementation tests for space and viewport id. This handles things like testing if a page space object is visible in a modeling view.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsActiveInViewport(Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — True if the object is active in viewport

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsActiveInViewport.htm)

#### `public int IsHighlighted(bool checkSubObjects)`

Check highlight state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — If true and the entire object is not highlighted, and some subset of the object is highlighted, like some edges of a surface, then 3 is returned. If false and the entire object is not highlighted, then zero is returned.

**Returns:** `Int32` — 0: object is not highlighted.1: entire object is highlighted.3: one or more proper sub-objects are highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsHighlighted.htm)

#### `public virtual bool IsMeshable(MeshType meshType)`

Returns true if the object is capable of having a mesh of the specified type

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsMeshable.htm)

#### `public bool IsSelectable()`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable.htm)

#### `public bool IsSelectable(bool ignoreSelectionState, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers are selectable. If false, then objects on locked layers are not selectable.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers are selectable. If false, then objects on hidden layers are not selectable.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable_1.htm)

#### `public int IsSelected(bool checkSubObjects)`

Check selection state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — (false is good default) If true and the entire object is not selected, and some subset of the object is selected, like some edges of a surface, then 3 is returned. If false and the entire object is not selected, then zero is returned.

**Returns:** `Int32` — 0 = object is not selected. 1 = object is selected. 2 = entire object is selected persistently. 3 = one or more proper sub-objects are selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelected.htm)

#### `public bool IsSubObjectHighlighted(ComponentIndex componentIndex)`

Determines if a sub-object is highlighted.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.

**Returns:** `Boolean` — true if the sub-object is highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectHighlighted.htm)

#### `public bool IsSubObjectSelectable(ComponentIndex componentIndex, bool ignoreSelectionState)`

Reports if a sub-object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — index of sub-object to check.
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.

**Returns:** `Boolean` — true if the specified sub-object can be selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelectable.htm)

#### `public bool IsSubObjectSelected(ComponentIndex componentIndex)`

Check sub-object selection state.

**Remarks:** A sub-object cannot be persistently selected.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.

**Returns:** `Boolean` — true if the sub-object is selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelected.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `public void LockId()`

Locks the component Id property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockId.htm)

#### `public void LockIndex()`

Locks the component Index property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockIndex.htm)

#### `public void LockName()`

Locks the component Name property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockName.htm)

#### `public uint MemoryEstimate()`

Computes an estimate of the number of bytes that this object is using in memory. Note that this is a runtime memory estimate and does not directly compare to the amount of space take up by the object when saved to a file.

**Returns:** `UInt32` — The estimated number of bytes this object occupies in memory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MemoryEstimate.htm)

#### `public virtual int MeshCount(MeshType meshType, MeshingParameters parameters)`

RhinoObjects can have several different types of meshes and different numbers of meshes. A b-rep can have a render and an analysis mesh on each face. A mesh object has a single render mesh and no analysis mesh. Curve, point, and annotation objects have no meshes.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of mesh to count
- `parameters` (Rhino.Geometry.MeshingParameters) — if not null and if the object can change its mesh (like a brep), then only meshes that were created with these mesh parameters are counted.

**Returns:** `Int32` — number of meshes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MeshCount.htm)

#### `public void Move(Point3d newLocation)`

Moves the grip to a new location.

**Parameters:**
- `newLocation` (Rhino.Geometry.Point3d) — New location for grip.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_Move.htm)

#### `public void Move(Transform xform)`

Moves the grip to a new location.

**Parameters:**
- `xform` (Rhino.Geometry.Transform) — Transformation applied to the OriginalLocation point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_Move_1.htm)

#### `public void Move(Vector3d delta)`

Moves the grip to a new location.

**Parameters:**
- `delta` (Rhino.Geometry.Vector3d) — Translation applied to the OriginalLocation point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_Move_2.htm)

#### `public GripObject NeighborGrip(int directionR, int directionS, int directionT, bool wrap)`

Used to get a grip's logical neighbors, like NURBS curve, surface, and cage control point grips.

**Parameters:**
- `directionR` (System.Int32) — -1 to go back one grip, +1 to move forward one grip. For curves, surfaces and cages, this is the first parameter direction.
- `directionS` (System.Int32) — -1 to go back one grip, +1 to move forward one grip. For surfaces and cages this is the second parameter direction.
- `directionT` (System.Int32) — For cages this is the third parameter direction
- `wrap` (System.Boolean) — [Missing <param name="wrap"/> documentation for "M:Rhino.DocObjects.GripObject.NeighborGrip(System.Int32,System.Int32,System.Int32,System.Boolean)"]

**Returns:** `GripObject` — logical neighbor or null if the is no logical neighbor

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_NeighborGrip.htm)

#### `public virtual void NewLocation()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomGripObject_NewLocation.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnAddToDocument(RhinoDoc doc)`

This call informs an object it is about to be added to the list of active objects in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnAddToDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnAddToDocument.htm)

#### `protected virtual void OnDeleteFromDocument(RhinoDoc doc)`

This call informs an object it is about to be deleted. Some objects, like clipping planes, need to do a little extra cleanup before they are deleted.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDeleteFromDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDeleteFromDocument.htm)

#### `protected virtual void OnDraw(DrawEventArgs e)`

Called when Rhino wants to draw this object

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDraw(Rhino.Display.DrawEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDraw.htm)

#### `protected virtual void OnDuplicate(RhinoObject source)`

Called when this a new instance of this object is created and copied from an existing object

**Parameters:**
- `source` (Rhino.DocObjects.RhinoObject) — [Missing <param name="source"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDuplicate(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDuplicate.htm)

#### `protected virtual IEnumerable<ObjRef> OnPick(PickContext context)`

Called to determine if this object or some sub-portion of this object should be picked given a pick context.

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

**Returns:** `IEnumerable<ObjRef>` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPick.htm)

#### `protected virtual void OnPicked(PickContext context, IEnumerable<ObjRef> pickedItems)`

Called when this object has been picked

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPicked(Rhino.Input.Custom.PickContext,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]
- `pickedItems` (System.Collections.Generic.IEnumerable<ObjRef>) — Items that were picked. This parameter is enumerable because there may have been multiple sub-objects picked

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPicked.htm)

#### `protected virtual void OnSelectionChanged()`

Called when the selection state of this object has changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSelectionChanged.htm)

#### `protected virtual void OnSpaceMorph(SpaceMorph morph)`

Called when a space morph has been applied to the geometry. Currently this only works for CustomMeshObject instances

**Parameters:**
- `morph` (Rhino.Geometry.SpaceMorph) — [Missing <param name="morph"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnSpaceMorph(Rhino.Geometry.SpaceMorph)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSpaceMorph.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `protected virtual void OnTransform(Transform transform)`

Called when a transformation has been applied to the geometry

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnTransform.htm)

#### `public int Select(bool on)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select.htm)

#### `public int Select(bool on, bool syncHighlight)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and not highlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_1.htm)

#### `public int Select(bool on, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected. If false, then objects on locked layers cannot be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable. If false, then objects on hidden layers cannot be selected.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_2.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Int32` — 0: object is not selected 1: object is selected 2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight, bool persistentSelect)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — When true, selection persists even after the current command terminates.

**Returns:** `Int32` — 0: object is not selected1: object is selected2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject_1.htm)

#### `public void SetCopyHistoryOnReplace(bool bCopy)`

If this object has a history record, the CopyOnReplace field is set When an object is replaced in a document and the old object has a history record with this field set, the history record is copied and attached to the new object. That allows a descendant object to continue the history linkage after it is edited.

**Parameters:**
- `bCopy` (System.Boolean) — [Missing <param name="bCopy"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCopyHistoryOnReplace(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCopyHistoryOnReplace.htm)

#### `public virtual void SetCustomRenderMeshParameter(Guid providerId, string parameterName, Object value)`

Set the named custom render mesh parameter value for this object.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCustomRenderMeshParameter.htm)

#### `public bool SetRenderMeshParameters(MeshingParameters mp)`

Sets the per-object meshing parameters for this object. When set, this object will use these meshing parameters when generating a render mesh, instead of those provided by the document.

**Parameters:**
- `mp` (Rhino.Geometry.MeshingParameters) — The per-object meshing parameters. Note: if null, then the per-object meshing parameters will be removed, and this object will revert to using the meshing parameters provided by the document.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetRenderMeshParameters.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm, Transform objectTransform)`

Sets texture mapping and mapping object transform for a channel

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `objectTransform` (Rhino.Geometry.Transform) — Mapping channel object transform

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping_1.htm)

#### `public virtual string ShortDescription(bool plural)`

Gets a localized short descriptive name of the object.

**Parameters:**
- `plural` (System.Boolean) — true if the descriptive name should in plural.

**Returns:** `String` — A string with the short localized descriptive name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_ShortDescription.htm)

#### `[ObsoleteAttribute] public bool SupportsRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList_1.htm)

#### `public bool SupportsRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

#### `public override string ToString()`

Returns the name of the model component type, and then its name and index.

**Remarks:** This is provided as a visual aid. Do not rely on this for serialization.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.ModelComponent.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ToString.htm)

#### `public bool TryGetGumballFrame(out GumballFrame frame)`

If a Rhino object has been manipulated by Rhino's gumball, and the gumball is not in its default position, then the object's repositioned gumball frame is returned.

**Parameters:**
- `frame` (Rhino.UI.Gumball.GumballFrame) — The gumball frame.

**Returns:** `Boolean` — true if the object has a gumball frame, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetGumballFrame.htm)

#### `[ObsoleteAttribute] public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, bool preview, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox_1.htm)

#### `public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox.htm)

#### `public void UndoMove()`

Undoes any grip moves made by calling Move.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_GripObject_UndoMove.htm)

#### `public int UnhighlightAllSubObjects()`

Removes highlighting from all sub-objects.

**Returns:** `Int32` — The number of changed sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnhighlightAllSubObjects.htm)

#### `public int UnselectAllSubObjects()`

Removes selection from all sub-objects.

**Returns:** `Int32` — The number of unselected sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnselectAllSubObjects.htm)

### Properties
- `Attributes` (ObjectAttributes, get/set) — Gets or sets the object attributes.
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `CurrentLocation` (Point3d, get/set) — (Inherited from GripObject.)
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Document` (RhinoDoc, get) — Gets the document that owns this object.
- `Geometry` (GeometryBase, get) — Gets the underlying geometry for this object. All rhino objects are composed of geometry and attributes.
- `GripsOn` (Boolean, get/set) — Gets or sets the activation state of object default editing grips.
- `GripsSelected` (Boolean, get) — true if grips are turned on and at least one is selected.
- `GroupCount` (Int32, get) — Number of groups object belongs to.
- `HasDynamicTransform` (Boolean, get) — True if the object has a dynamic transformation
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasSubobjectMaterials` (Boolean, get) — Will be true if the object contains sub object meshes with materials that are different than the top level object.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get/set) — Every object has a Guid (globally unique identifier, also known as UUID, or universally unique identifier). The default value is Guid.Empty. When an object is added to a model, the value is checked. If the value is Guid.Empty, a new Guid is created. If the value is not null but it is already used by another object in the model, a new Guid is created. If the value is not Guid.Empty and it is not used by another object in the model, then that value persists. When an object is updated, by a move for example, the value of ObjectId persists. This value is the same as the one returned by this.Attributes.ObjectId.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — 
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsDeletable` (Boolean, get) — Some objects cannot be deleted, like grips on lights and annotation objects.
- `IsDeleted` (Boolean, get) — true if the object is deleted. Deleted objects are kept by the document for undo purposes. Call RhinoDoc.UndeleteObject to undelete an object.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsHidden` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsInstanceDefinitionGeometry` (Boolean, get) — true if the object is used as part of an instance definition.
- `IsLocked` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsNormal` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsPictureFrame` (Boolean, get) — Returns true if the object is a picture frame. A picture frame object is an object that displays a texture map in all views.
- `IsReference` (Boolean, get) — Gets a value indicating if an object is a reference object. An object from a work session reference model is a reference object and cannot be modified. An object is a reference object if, and only if, it is on a reference layer.
- `IsSolid` (Boolean, get) — Returns true if object is a closed solid, otherwise false.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Moved` (Boolean, get) — true if the grip has moved from OriginalLocation.
- `Name` (String, get/set) — Rhino objects have optional text names. More than one object in a model can have the same name and some objects may have no name.
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `ObjectType` (ObjectType, get) — Gets the Rhino-based object type.
- `OriginalLocation` (Point3d, get/set) — 
- `OwnerId` (Guid, get) — (Inherited from GripObject.)
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `RenderMaterial` (RenderMaterial, get/set) — Gets the render material associated with this object or null if there is none. This does not pay attention to the material source and will not check parent objects or layers for a RenderMaterial.
- `RuntimeSerialNumber` (UInt32, get) — Gets the objects runtime serial number.
- `SubobjectMaterialComponents` (ComponentIndex[], get) — (Inherited from RhinoObject.)
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `Visible` (Boolean, get) — Gets the object visibility.
- `Weight` (Double, get/set) — (Overrides GripObject.Weight.)
- `WorksessionReferenceSerialNumber` (UInt32, get) — Obsolete - use ReferenceModelSerialNumber

## CustomMeshObject (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.CustomMeshObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_CustomMeshObject.htm)

### Constructors
- `protected CustomMeshObject()` — Initializes a new instance of the CustomMeshObject class
- `protected CustomMeshObject(Mesh mesh)` — Initializes a new instance of the CustomMeshObject class

### Methods
#### `public void ClearId()`

Resets the HasId property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearId.htm)

#### `public void ClearIndex()`

Resets the HasIndex property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearIndex.htm)

#### `public void ClearName()`

Resets the HasName property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearName.htm)

#### `public bool CommitChanges()`

Moves changes made to this RhinoObject into the RhinoDoc.

**Returns:** `Boolean` — true if changes were made.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CommitChanges.htm)

#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public bool CopyHistoryOnReplace()`

Gets the setting of the CopyOnReplace field in this object's history

**Returns:** `Boolean` — true if this object has history and the field is set false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CopyHistoryOnReplace.htm)

#### `public virtual int CreateMeshes(MeshType meshType, MeshingParameters parameters, bool ignoreCustomParameters)`

Create meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of meshes to create
- `parameters` (Rhino.Geometry.MeshingParameters) — in parameters that control the quality of the meshes that are created
- `ignoreCustomParameters` (System.Boolean) — Default should be false. Should the object ignore any custom meshing parameters on the object's attributes

**Returns:** `Int32` — number of meshes created

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CreateMeshes.htm)

#### `public uint DataCRC(uint currentRemainder)`

Increments the Cyclic Redundancy Check value by this instance.

**Parameters:**
- `currentRemainder` (System.UInt32) — The current remainder value.

**Returns:** `UInt32` — The updated remainder value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_DataCRC.htm)

#### `public void Description(TextLog textLog)`

Get a brief description of a object, including it's attributes and geometry.

**Parameters:**
- `textLog` (Rhino.FileIO.TextLog) — A text log for collecting the description.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Description.htm)

#### `public void Dispose()`

Releases all resources used by the CustomMeshObject

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomMeshObject_Dispose.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the CustomMeshObject and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomMeshObject_Dispose_1.htm)

#### `public GeometryBase DuplicateGeometry()`

Constructs a deep (full) copy of the geometry.

**Returns:** `GeometryBase` — A copy of the internal geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_DuplicateGeometry.htm)

#### `public Mesh DuplicateMeshGeometry()`

(Inherited from MeshObject.)

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.DocObjects.MeshObject.DuplicateMeshGeometry"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_MeshObject_DuplicateMeshGeometry.htm)

#### `public bool EnableCustomGrips(CustomObjectGrips customGrips)`

Turns on/off the object's editing grips.

**Parameters:**
- `customGrips` (Rhino.DocObjects.Custom.CustomObjectGrips) — The custom object grips.

**Returns:** `Boolean` — true if the call succeeded. If you attempt to add custom grips to an object that does not support custom grips, then false is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableCustomGrips.htm)

#### `public bool EnableVisualAnalysisMode(VisualAnalysisMode mode, bool enable)`

Used to turn analysis modes on and off.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — A visual analysis mode.
- `enable` (System.Boolean) — true if the mode should be activated; false otherwise.

**Returns:** `Boolean` — true if this object supports the analysis mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableVisualAnalysisMode.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

(Overrides CommonObject.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomMeshObject_Finalize.htm)

#### `public VisualAnalysisMode[] GetActiveVisualAnalysisModes()`

Gets a list of currently enabled analysis modes for this object.

**Returns:** `VisualAnalysisMode[]` — An array of visual analysis modes. The array can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetActiveVisualAnalysisModes.htm)

#### `public virtual IConvertible GetCustomRenderMeshParameter(Guid providerId, string parameterName)`

Query the object for the value of a given named custom render mesh parameter.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — Name of the parameter

**Returns:** `IConvertible` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetCustomRenderMeshParameter.htm)

#### `public bool GetDynamicTransform(out Transform transform)`

While an object is being dynamically transformed (dragged, rotated, ...), the current transformation can be retrieved and used for creating dynamic display.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetDynamicTransform(Rhino.Geometry.Transform@)"]

**Returns:** `Boolean` — True if the object is being edited and its transformation is available. False if the object is not being edited, in which case the identity transform is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetDynamicTransform.htm)

#### `public GripObject[] GetGrips()`

Returns grips for this object If grips are enabled. If grips are not enabled, returns null.

**Returns:** `GripObject[]` — An array of grip objects; or null if there are no grips.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGrips.htm)

#### `public int[] GetGroupList()`

Allocates an array of group indices of length GroupCount. If GroupCount is 0, then this method returns null.

**Returns:** `Int32[]` — An array of group indices, or null if GroupCount is 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGroupList.htm)

#### `public ComponentIndex[] GetHighlightedSubObjects()`

Gets a list of all highlighted sub-objects.

**Returns:** `ComponentIndex[]` — An array of all highlighted sub-objects; or null is there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetHighlightedSubObjects.htm)

#### `public Material GetMaterial(bool frontMaterial)`

Gets material that this object uses based on it's attributes and the document that the object is associated with. In the rare case that a document is not associated with this object, null will be returned.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side

**Returns:** `Material` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetMaterial(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_3.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_1.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_2.htm)

#### `public virtual Mesh[] GetMeshes(MeshType meshType)`

Get existing meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetMeshes(Rhino.Geometry.MeshType)"]

**Returns:** `Mesh[]` — An array of meshes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMeshes.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public RenderMaterial GetRenderMaterial(bool frontMaterial)`

Gets the RenderMaterial that this object uses based on it's attributes and the document that the object is associated with. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side otherwise; gets the material used to render the back side of the object.

**Returns:** `RenderMaterial` — If there is a RenderMaterial associated with this objects' associated Material then it is returned otherwise; null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_3.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_1.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_2.htm)

#### `public MeshingParameters GetRenderMeshParameters()`

Returns the meshing parameters that this object uses for generating render meshes. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned.

**Returns:** `MeshingParameters` — The render meshing parameters.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters.htm)

#### `public MeshingParameters GetRenderMeshParameters(bool returnDocumentParametersIfUnset)`

Returns the meshing parameters that this object uses for generating render meshes.

**Parameters:**
- `returnDocumentParametersIfUnset` (System.Boolean) — If true, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned. If false, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then null is returned.

**Returns:** `MeshingParameters` — The render meshing parameters if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters_1.htm)

#### `[ObsoleteAttribute] public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList_1.htm)

#### `public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList.htm)

#### `public ComponentIndex[] GetSelectedSubObjects()`

Get a list of all selected sub-objects.

**Returns:** `ComponentIndex[]` — An array of sub-object indices, or null if there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSelectedSubObjects.htm)

#### `public RhinoObject[] GetSubObjects()`

Explodes the object into sub-objects. It is up to the caller to add the returned objects to the document.

**Returns:** `RhinoObject[]` — An array of Rhino objects, or null if this object cannot be exploded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSubObjects.htm)

#### `public int[] GetTextureChannels()`

Get a list of the texture mapping channel Id's associated with object.

**Returns:** `Int32[]` — Returns an array of channel Id's or an empty list if there are not mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureChannels.htm)

#### `public TextureMapping GetTextureMapping(int channel)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping.htm)

#### `public TextureMapping GetTextureMapping(int channel, out Transform objectTransform)`

Get objects texture mapping

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]
- `objectTransform` (Rhino.Geometry.Transform) — [Missing <param name="objectTransform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping_1.htm)

#### `public bool HasHistoryRecord()`

Returns whether this object has a history record

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasHistoryRecord"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasHistoryRecord.htm)

#### `public bool HasTextureMapping()`

Returns true if this object has a texture mapping form any source (pluginId)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasTextureMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasTextureMapping.htm)

#### `public bool Highlight(bool enable)`

Modifies the highlighting of the object.

**Parameters:**
- `enable` (System.Boolean) — true if highlighting should be enabled.

**Returns:** `Boolean` — true if the object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Highlight.htm)

#### `public bool HighlightSubObject(ComponentIndex componentIndex, bool highlight)`

Highlights a sub-object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.
- `highlight` (System.Boolean) — true if the sub-object should be highlighted.

**Returns:** `Boolean` — true if the sub-object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HighlightSubObject.htm)

#### `public bool InVisualAnalysisMode()`

Reports if any visual analysis mode is currently active for an object.

**Returns:** `Boolean` — true if an analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode.htm)

#### `public bool InVisualAnalysisMode(VisualAnalysisMode mode)`

Reports if a visual analysis mode is currently active for an object.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — The mode to check for. Use null if you want to see if any mode is active.

**Returns:** `Boolean` — true if the specified analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode_1.htm)

#### `public virtual bool IsActiveInViewport(RhinoViewport viewport)`

Determine if this object is active in a particular viewport.

**Remarks:** The default implementation tests for space and viewport id. This handles things like testing if a page space object is visible in a modeling view.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsActiveInViewport(Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — True if the object is active in viewport

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsActiveInViewport.htm)

#### `public int IsHighlighted(bool checkSubObjects)`

Check highlight state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — If true and the entire object is not highlighted, and some subset of the object is highlighted, like some edges of a surface, then 3 is returned. If false and the entire object is not highlighted, then zero is returned.

**Returns:** `Int32` — 0: object is not highlighted.1: entire object is highlighted.3: one or more proper sub-objects are highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsHighlighted.htm)

#### `public virtual bool IsMeshable(MeshType meshType)`

Returns true if the object is capable of having a mesh of the specified type

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsMeshable.htm)

#### `public bool IsSelectable()`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable.htm)

#### `public bool IsSelectable(bool ignoreSelectionState, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers are selectable. If false, then objects on locked layers are not selectable.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers are selectable. If false, then objects on hidden layers are not selectable.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable_1.htm)

#### `public int IsSelected(bool checkSubObjects)`

Check selection state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — (false is good default) If true and the entire object is not selected, and some subset of the object is selected, like some edges of a surface, then 3 is returned. If false and the entire object is not selected, then zero is returned.

**Returns:** `Int32` — 0 = object is not selected. 1 = object is selected. 2 = entire object is selected persistently. 3 = one or more proper sub-objects are selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelected.htm)

#### `public bool IsSubObjectHighlighted(ComponentIndex componentIndex)`

Determines if a sub-object is highlighted.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.

**Returns:** `Boolean` — true if the sub-object is highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectHighlighted.htm)

#### `public bool IsSubObjectSelectable(ComponentIndex componentIndex, bool ignoreSelectionState)`

Reports if a sub-object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — index of sub-object to check.
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.

**Returns:** `Boolean` — true if the specified sub-object can be selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelectable.htm)

#### `public bool IsSubObjectSelected(ComponentIndex componentIndex)`

Check sub-object selection state.

**Remarks:** A sub-object cannot be persistently selected.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.

**Returns:** `Boolean` — true if the sub-object is selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelected.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `public void LockId()`

Locks the component Id property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockId.htm)

#### `public void LockIndex()`

Locks the component Index property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockIndex.htm)

#### `public void LockName()`

Locks the component Name property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockName.htm)

#### `public uint MemoryEstimate()`

Computes an estimate of the number of bytes that this object is using in memory. Note that this is a runtime memory estimate and does not directly compare to the amount of space take up by the object when saved to a file.

**Returns:** `UInt32` — The estimated number of bytes this object occupies in memory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MemoryEstimate.htm)

#### `public virtual int MeshCount(MeshType meshType, MeshingParameters parameters)`

RhinoObjects can have several different types of meshes and different numbers of meshes. A b-rep can have a render and an analysis mesh on each face. A mesh object has a single render mesh and no analysis mesh. Curve, point, and annotation objects have no meshes.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of mesh to count
- `parameters` (Rhino.Geometry.MeshingParameters) — if not null and if the object can change its mesh (like a brep), then only meshes that were created with these mesh parameters are counted.

**Returns:** `Int32` — number of meshes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MeshCount.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnAddToDocument(RhinoDoc doc)`

This call informs an object it is about to be added to the list of active objects in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnAddToDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnAddToDocument.htm)

#### `protected virtual void OnDeleteFromDocument(RhinoDoc doc)`

This call informs an object it is about to be deleted. Some objects, like clipping planes, need to do a little extra cleanup before they are deleted.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDeleteFromDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDeleteFromDocument.htm)

#### `protected virtual void OnDraw(DrawEventArgs e)`

Called when Rhino wants to draw this object

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDraw(Rhino.Display.DrawEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDraw.htm)

#### `protected virtual void OnDuplicate(RhinoObject source)`

Called when this a new instance of this object is created and copied from an existing object

**Parameters:**
- `source` (Rhino.DocObjects.RhinoObject) — [Missing <param name="source"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDuplicate(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDuplicate.htm)

#### `protected virtual IEnumerable<ObjRef> OnPick(PickContext context)`

Called to determine if this object or some sub-portion of this object should be picked given a pick context.

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

**Returns:** `IEnumerable<ObjRef>` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPick.htm)

#### `protected virtual void OnPicked(PickContext context, IEnumerable<ObjRef> pickedItems)`

Called when this object has been picked

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPicked(Rhino.Input.Custom.PickContext,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]
- `pickedItems` (System.Collections.Generic.IEnumerable<ObjRef>) — Items that were picked. This parameter is enumerable because there may have been multiple sub-objects picked

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPicked.htm)

#### `protected virtual void OnSelectionChanged()`

Called when the selection state of this object has changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSelectionChanged.htm)

#### `protected virtual void OnSpaceMorph(SpaceMorph morph)`

Called when a space morph has been applied to the geometry. Currently this only works for CustomMeshObject instances

**Parameters:**
- `morph` (Rhino.Geometry.SpaceMorph) — [Missing <param name="morph"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnSpaceMorph(Rhino.Geometry.SpaceMorph)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSpaceMorph.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `protected virtual void OnTransform(Transform transform)`

Called when a transformation has been applied to the geometry

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnTransform.htm)

#### `public int Select(bool on)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select.htm)

#### `public int Select(bool on, bool syncHighlight)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and not highlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_1.htm)

#### `public int Select(bool on, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected. If false, then objects on locked layers cannot be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable. If false, then objects on hidden layers cannot be selected.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_2.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Int32` — 0: object is not selected 1: object is selected 2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight, bool persistentSelect)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — When true, selection persists even after the current command terminates.

**Returns:** `Int32` — 0: object is not selected1: object is selected2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject_1.htm)

#### `public void SetCopyHistoryOnReplace(bool bCopy)`

If this object has a history record, the CopyOnReplace field is set When an object is replaced in a document and the old object has a history record with this field set, the history record is copied and attached to the new object. That allows a descendant object to continue the history linkage after it is edited.

**Parameters:**
- `bCopy` (System.Boolean) — [Missing <param name="bCopy"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCopyHistoryOnReplace(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCopyHistoryOnReplace.htm)

#### `public virtual void SetCustomRenderMeshParameter(Guid providerId, string parameterName, Object value)`

Set the named custom render mesh parameter value for this object.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCustomRenderMeshParameter.htm)

#### `protected Mesh SetMesh(Mesh mesh)`

Only for developers who are defining custom subclasses of MeshObject. Directly sets the internal mesh geometry for this object. Note that this function does not work with Rhino's "undo".

**Remarks:** Note that this function does not work with Rhino's "undo". The typical approach for adjusting the mesh geometry is to modify the object that you get when you call the MeshGeometry property and then call CommitChanges.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.DocObjects.MeshObject.SetMesh(Rhino.Geometry.Mesh)"]

**Returns:** `Mesh` — The old mesh geometry that was set for this object

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_MeshObject_SetMesh.htm)

#### `public bool SetRenderMeshParameters(MeshingParameters mp)`

Sets the per-object meshing parameters for this object. When set, this object will use these meshing parameters when generating a render mesh, instead of those provided by the document.

**Parameters:**
- `mp` (Rhino.Geometry.MeshingParameters) — The per-object meshing parameters. Note: if null, then the per-object meshing parameters will be removed, and this object will revert to using the meshing parameters provided by the document.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetRenderMeshParameters.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm, Transform objectTransform)`

Sets texture mapping and mapping object transform for a channel

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `objectTransform` (Rhino.Geometry.Transform) — Mapping channel object transform

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping_1.htm)

#### `public virtual string ShortDescription(bool plural)`

Gets a localized short descriptive name of the object.

**Parameters:**
- `plural` (System.Boolean) — true if the descriptive name should in plural.

**Returns:** `String` — A string with the short localized descriptive name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_ShortDescription.htm)

#### `[ObsoleteAttribute] public bool SupportsRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList_1.htm)

#### `public bool SupportsRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

#### `public override string ToString()`

Returns the name of the model component type, and then its name and index.

**Remarks:** This is provided as a visual aid. Do not rely on this for serialization.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.ModelComponent.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ToString.htm)

#### `public bool TryGetGumballFrame(out GumballFrame frame)`

If a Rhino object has been manipulated by Rhino's gumball, and the gumball is not in its default position, then the object's repositioned gumball frame is returned.

**Parameters:**
- `frame` (Rhino.UI.Gumball.GumballFrame) — The gumball frame.

**Returns:** `Boolean` — true if the object has a gumball frame, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetGumballFrame.htm)

#### `[ObsoleteAttribute] public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, bool preview, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox_1.htm)

#### `public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox.htm)

#### `public int UnhighlightAllSubObjects()`

Removes highlighting from all sub-objects.

**Returns:** `Int32` — The number of changed sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnhighlightAllSubObjects.htm)

#### `public int UnselectAllSubObjects()`

Removes selection from all sub-objects.

**Returns:** `Int32` — The number of unselected sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnselectAllSubObjects.htm)

### Properties
- `Attributes` (ObjectAttributes, get/set) — Gets or sets the object attributes.
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Document` (RhinoDoc, get) — Gets the document that owns this object.
- `Geometry` (GeometryBase, get) — Gets the underlying geometry for this object. All rhino objects are composed of geometry and attributes.
- `GripsOn` (Boolean, get/set) — Gets or sets the activation state of object default editing grips.
- `GripsSelected` (Boolean, get) — true if grips are turned on and at least one is selected.
- `GroupCount` (Int32, get) — Number of groups object belongs to.
- `HasDynamicTransform` (Boolean, get) — True if the object has a dynamic transformation
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasSubobjectMaterials` (Boolean, get) — Will be true if the object contains sub object meshes with materials that are different than the top level object.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get/set) — Every object has a Guid (globally unique identifier, also known as UUID, or universally unique identifier). The default value is Guid.Empty. When an object is added to a model, the value is checked. If the value is Guid.Empty, a new Guid is created. If the value is not null but it is already used by another object in the model, a new Guid is created. If the value is not Guid.Empty and it is not used by another object in the model, then that value persists. When an object is updated, by a move for example, the value of ObjectId persists. This value is the same as the one returned by this.Attributes.ObjectId.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — Gets or sets the model component index attribute.
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsCustomObject` (Boolean, get) — (Inherited from MeshObject.)
- `IsDeletable` (Boolean, get) — Some objects cannot be deleted, like grips on lights and annotation objects.
- `IsDeleted` (Boolean, get) — true if the object is deleted. Deleted objects are kept by the document for undo purposes. Call RhinoDoc.UndeleteObject to undelete an object.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsHidden` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsInstanceDefinitionGeometry` (Boolean, get) — true if the object is used as part of an instance definition.
- `IsLocked` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsNormal` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsPictureFrame` (Boolean, get) — Returns true if the object is a picture frame. A picture frame object is an object that displays a texture map in all views.
- `IsReference` (Boolean, get) — Gets a value indicating if an object is a reference object. An object from a work session reference model is a reference object and cannot be modified. An object is a reference object if, and only if, it is on a reference layer.
- `IsSolid` (Boolean, get) — Returns true if object is a closed solid, otherwise false.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `MeshGeometry` (Mesh, get) — (Inherited from MeshObject.)
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Name` (String, get/set) — Rhino objects have optional text names. More than one object in a model can have the same name and some objects may have no name.
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `ObjectType` (ObjectType, get) — Gets the Rhino-based object type.
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `RenderMaterial` (RenderMaterial, get/set) — Gets the render material associated with this object or null if there is none. This does not pay attention to the material source and will not check parent objects or layers for a RenderMaterial.
- `RuntimeSerialNumber` (UInt32, get) — Gets the objects runtime serial number.
- `SubobjectMaterialComponents` (ComponentIndex[], get) — (Inherited from RhinoObject.)
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `Visible` (Boolean, get) — Gets the object visibility.
- `WorksessionReferenceSerialNumber` (UInt32, get) — Obsolete - use ReferenceModelSerialNumber

## CustomObjectGrips (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.CustomObjectGrips"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_CustomObjectGrips.htm)

### Constructors
- `protected CustomObjectGrips()` — Initializes a new instance of the CustomObjectGrips class

### Methods
#### `protected void AddGrip(CustomGripObject grip)`



**Parameters:**
- `grip` (Rhino.DocObjects.Custom.CustomGripObject) — [Missing <param name="grip"/> documentation for "M:Rhino.DocObjects.Custom.CustomObjectGrips.AddGrip(Rhino.DocObjects.Custom.CustomGripObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_AddGrip.htm)

#### `public void Dispose()`

Releases all resources used by the CustomObjectGrips

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the CustomObjectGrips and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_Dispose_1.htm)

#### `public static bool Dragging()`

Determines if grips are currently being dragged.

**Returns:** `Boolean` — true if grips are dragged.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_Dragging.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_Finalize.htm)

#### `public CustomGripObject Grip(int index)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Custom.CustomObjectGrips.Grip(System.Int32)"]

**Returns:** `CustomGripObject` — [Missing <returns> documentation for "M:Rhino.DocObjects.Custom.CustomObjectGrips.Grip(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_Grip.htm)

#### `protected virtual GripObject NeighborGrip(int gripIndex, int dr, int ds, int dt, bool wrap)`

Get neighbors.

**Parameters:**
- `gripIndex` (System.Int32) — index of grip where the search begins.
- `dr` (System.Int32) — 1 = next grip in the first parameter direction.-1 = previous grip in the first parameter direction.
- `ds` (System.Int32) — 1 = next grip in the second parameter direction.-1 = previous grip in the second parameter direction.
- `dt` (System.Int32) — 1 = next grip in the third parameter direction.-1 = rev grip in the third parameter direction.
- `wrap` (System.Boolean) — If true and object is "closed", the search will wrap.

**Returns:** `GripObject` — Pointer to the desired neighbor or NULL if there is no neighbor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_NeighborGrip.htm)

#### `protected virtual GeometryBase NewGeometry()`

If the grips control just one object, then override NewGeometry(). When NewGeometry() is called, return new geometry calculated from the current grip locations. This happens once at the end of a grip drag.

**Returns:** `GeometryBase` — The new geometry. The default implementation returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_NewGeometry.htm)

#### `protected virtual NurbsSurface NurbsSurface()`

If the grips control a NURBS surface, this returns a pointer to that surface. You can look at but you must NEVER change this surface.

**Returns:** `NurbsSurface` — A pointer to a NURBS surface or null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_NurbsSurface.htm)

#### `protected virtual GripObject NurbsSurfaceGrip(int i, int j)`

If the grips are control points of a NURBS surface, then this gets the index of the grip that controls the (i,j)-th CV.

**Parameters:**
- `i` (System.Int32) — The index in the first dimension.
- `j` (System.Int32) — The index in the second dimension.

**Returns:** `GripObject` — A grip controlling a NURBS surface CV or null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_NurbsSurfaceGrip.htm)

#### `protected virtual void OnDraw(GripsDrawEventArgs args)`

Draws the grips. In your implementation, override this if you need to draw dynamic elements and then call this base implementation to draw the grips themselves.

**Parameters:**
- `args` (Rhino.DocObjects.Custom.GripsDrawEventArgs) — The grips draw event arguments.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_OnDraw.htm)

#### `protected virtual void OnReset()`

Resets location of all grips to original spots and cleans up stuff that was created by dynamic dragging. This is required when dragging is canceled or in the Copy command when grips are "copied". The override should clean up dynamic workspace stuff.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_OnReset.htm)

#### `protected virtual void OnResetMeshes()`

Just before Rhino turns off object grips, it calls this function. If grips have modified any display meshes, they must override this function and restore the meshes to their original states.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_OnResetMeshes.htm)

#### `protected virtual void OnUpdateMesh(MeshType meshType)`

Just before Rhino shades an object with grips on, it calls this method to update the display meshes. Grips that modify surface or mesh objects must override this function and modify the display meshes here.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — The mesh type being updated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_OnUpdateMesh.htm)

#### `public static void RegisterGripsEnabler(TurnOnGripsEventHandler enabler, Type customGripsType)`



**Parameters:**
- `enabler` (Rhino.DocObjects.Custom.TurnOnGripsEventHandler) — [Missing <param name="enabler"/> documentation for "M:Rhino.DocObjects.Custom.CustomObjectGrips.RegisterGripsEnabler(Rhino.DocObjects.Custom.TurnOnGripsEventHandler,System.Type)"]
- `customGripsType` (System.Type) — [Missing <param name="customGripsType"/> documentation for "M:Rhino.DocObjects.Custom.CustomObjectGrips.RegisterGripsEnabler(Rhino.DocObjects.Custom.TurnOnGripsEventHandler,System.Type)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomObjectGrips_RegisterGripsEnabler.htm)

### Properties
- `GripCount` (Int32, get) — 
- `GripsMoved` (Boolean, get) — If GripsMoved is true if some of the grips have ever been moved GripObject.NewLocation() sets GripsMoved=true.
- `NewLocation` (Boolean, get/set) — true if some of the grips have been moved. GripObject.NewLocation() sets NewLocation=true. Derived classes can set NewLocation to false after updating temporary display information.
- `OwnerObject` (RhinoObject, get) — Owner of the grips.

## CustomPointObject (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.CustomPointObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_CustomPointObject.htm)

### Constructors
- `protected CustomPointObject()` — Initializes a new instance of the CustomPointObject class
- `protected CustomPointObject(Point point)` — Initializes a new instance of the CustomPointObject class

### Methods
#### `public void ClearId()`

Resets the HasId property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearId.htm)

#### `public void ClearIndex()`

Resets the HasIndex property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearIndex.htm)

#### `public void ClearName()`

Resets the HasName property of the model component to false, if possible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ClearName.htm)

#### `public bool CommitChanges()`

Moves changes made to this RhinoObject into the RhinoDoc.

**Returns:** `Boolean` — true if changes were made.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CommitChanges.htm)

#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public bool CopyHistoryOnReplace()`

Gets the setting of the CopyOnReplace field in this object's history

**Returns:** `Boolean` — true if this object has history and the field is set false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CopyHistoryOnReplace.htm)

#### `public virtual int CreateMeshes(MeshType meshType, MeshingParameters parameters, bool ignoreCustomParameters)`

Create meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of meshes to create
- `parameters` (Rhino.Geometry.MeshingParameters) — in parameters that control the quality of the meshes that are created
- `ignoreCustomParameters` (System.Boolean) — Default should be false. Should the object ignore any custom meshing parameters on the object's attributes

**Returns:** `Int32` — number of meshes created

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_CreateMeshes.htm)

#### `public uint DataCRC(uint currentRemainder)`

Increments the Cyclic Redundancy Check value by this instance.

**Parameters:**
- `currentRemainder` (System.UInt32) — The current remainder value.

**Returns:** `UInt32` — The updated remainder value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_DataCRC.htm)

#### `public void Description(TextLog textLog)`

Get a brief description of a object, including it's attributes and geometry.

**Parameters:**
- `textLog` (Rhino.FileIO.TextLog) — A text log for collecting the description.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Description.htm)

#### `public void Dispose()`

Releases all resources used by the CustomPointObject

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomPointObject_Dispose.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the CustomPointObject and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomPointObject_Dispose_1.htm)

#### `public GeometryBase DuplicateGeometry()`

Constructs a deep (full) copy of the geometry.

**Returns:** `GeometryBase` — A copy of the internal geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_DuplicateGeometry.htm)

#### `public Point DuplicatePointGeometry()`

(Inherited from PointObject.)

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.DocObjects.PointObject.DuplicatePointGeometry"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_PointObject_DuplicatePointGeometry.htm)

#### `public bool EnableCustomGrips(CustomObjectGrips customGrips)`

Turns on/off the object's editing grips.

**Parameters:**
- `customGrips` (Rhino.DocObjects.Custom.CustomObjectGrips) — The custom object grips.

**Returns:** `Boolean` — true if the call succeeded. If you attempt to add custom grips to an object that does not support custom grips, then false is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableCustomGrips.htm)

#### `public bool EnableVisualAnalysisMode(VisualAnalysisMode mode, bool enable)`

Used to turn analysis modes on and off.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — A visual analysis mode.
- `enable` (System.Boolean) — true if the mode should be activated; false otherwise.

**Returns:** `Boolean` — true if this object supports the analysis mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_EnableVisualAnalysisMode.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

(Overrides CommonObject.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_CustomPointObject_Finalize.htm)

#### `public VisualAnalysisMode[] GetActiveVisualAnalysisModes()`

Gets a list of currently enabled analysis modes for this object.

**Returns:** `VisualAnalysisMode[]` — An array of visual analysis modes. The array can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetActiveVisualAnalysisModes.htm)

#### `public virtual IConvertible GetCustomRenderMeshParameter(Guid providerId, string parameterName)`

Query the object for the value of a given named custom render mesh parameter.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — Name of the parameter

**Returns:** `IConvertible` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetCustomRenderMeshParameter.htm)

#### `public bool GetDynamicTransform(out Transform transform)`

While an object is being dynamically transformed (dragged, rotated, ...), the current transformation can be retrieved and used for creating dynamic display.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetDynamicTransform(Rhino.Geometry.Transform@)"]

**Returns:** `Boolean` — True if the object is being edited and its transformation is available. False if the object is not being edited, in which case the identity transform is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetDynamicTransform.htm)

#### `public GripObject[] GetGrips()`

Returns grips for this object If grips are enabled. If grips are not enabled, returns null.

**Returns:** `GripObject[]` — An array of grip objects; or null if there are no grips.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGrips.htm)

#### `public int[] GetGroupList()`

Allocates an array of group indices of length GroupCount. If GroupCount is 0, then this method returns null.

**Returns:** `Int32[]` — An array of group indices, or null if GroupCount is 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetGroupList.htm)

#### `public ComponentIndex[] GetHighlightedSubObjects()`

Gets a list of all highlighted sub-objects.

**Returns:** `ComponentIndex[]` — An array of all highlighted sub-objects; or null is there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetHighlightedSubObjects.htm)

#### `public Material GetMaterial(bool frontMaterial)`

Gets material that this object uses based on it's attributes and the document that the object is associated with. In the rare case that a document is not associated with this object, null will be returned.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side

**Returns:** `Material` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetMaterial(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_3.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_1.htm)

#### `public Material GetMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Get the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the material associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `Material` — Returns the Material associated with the sub object identified by componentIndex if the component index is set to ComponentIndex.Unset then the top level material is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMaterial_2.htm)

#### `public virtual Mesh[] GetMeshes(MeshType meshType)`

Get existing meshes used to render and analyze surface and polysurface objects.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetMeshes(Rhino.Geometry.MeshType)"]

**Returns:** `Mesh[]` — An array of meshes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetMeshes.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public RenderMaterial GetRenderMaterial(bool frontMaterial)`

Gets the RenderMaterial that this object uses based on it's attributes and the document that the object is associated with. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `frontMaterial` (System.Boolean) — If true, gets the material used to render the object's front side otherwise; gets the material used to render the back side of the object.

**Returns:** `RenderMaterial` — If there is a RenderMaterial associated with this objects' associated Material then it is returned otherwise; null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_3.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to ComponentIndex.Unset

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_1.htm)

#### `public RenderMaterial GetRenderMaterial(ComponentIndex componentIndex, Guid plugInId, ObjectAttributes attributes)`

Gets the RenderMaterial associated with this object if there is one. If there is no RenderMaterial associated with this object then null is returned. If null is returned you should call GetMaterial to get the material used to render this object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Returns the RenderMaterial associated with the specified sub object or the objects top level material if it is set to
- `plugInId` (System.Guid) — The plug-in specific material to look for.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Optional object attributes used to determine the material source, if null the objects attributes are used.

**Returns:** `RenderMaterial` — Returns the RenderMaterial associated with the sub object identified by componentIndex if the component index is set to then the top level RenderMaterail is returned. If this method returns null it means there is no RenderMaterial associated with the object or sub object so you should may GetMaterial get the objects generic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMaterial_2.htm)

#### `public MeshingParameters GetRenderMeshParameters()`

Returns the meshing parameters that this object uses for generating render meshes. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned.

**Returns:** `MeshingParameters` — The render meshing parameters.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters.htm)

#### `public MeshingParameters GetRenderMeshParameters(bool returnDocumentParametersIfUnset)`

Returns the meshing parameters that this object uses for generating render meshes.

**Parameters:**
- `returnDocumentParametersIfUnset` (System.Boolean) — If true, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then the document's meshing parameters are returned. If false, then return the per-object meshing parameters for this object. If this object does not have per-object meshing parameters, then null is returned.

**Returns:** `MeshingParameters` — The render meshing parameters if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderMeshParameters_1.htm)

#### `[ObsoleteAttribute] public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList_1.htm)

#### `public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Build custom render mesh(es) for this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetRenderPrimitiveList.htm)

#### `public ComponentIndex[] GetSelectedSubObjects()`

Get a list of all selected sub-objects.

**Returns:** `ComponentIndex[]` — An array of sub-object indices, or null if there are none.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSelectedSubObjects.htm)

#### `public RhinoObject[] GetSubObjects()`

Explodes the object into sub-objects. It is up to the caller to add the returned objects to the document.

**Returns:** `RhinoObject[]` — An array of Rhino objects, or null if this object cannot be exploded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetSubObjects.htm)

#### `public int[] GetTextureChannels()`

Get a list of the texture mapping channel Id's associated with object.

**Returns:** `Int32[]` — Returns an array of channel Id's or an empty list if there are not mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureChannels.htm)

#### `public TextureMapping GetTextureMapping(int channel)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping.htm)

#### `public TextureMapping GetTextureMapping(int channel, out Transform objectTransform)`

Get objects texture mapping

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]
- `objectTransform` (Rhino.Geometry.Transform) — [Missing <param name="objectTransform"/> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.GetTextureMapping(System.Int32,Rhino.Geometry.Transform@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_GetTextureMapping_1.htm)

#### `public bool HasHistoryRecord()`

Returns whether this object has a history record

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasHistoryRecord"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasHistoryRecord.htm)

#### `public bool HasTextureMapping()`

Returns true if this object has a texture mapping form any source (pluginId)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.HasTextureMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HasTextureMapping.htm)

#### `public bool Highlight(bool enable)`

Modifies the highlighting of the object.

**Parameters:**
- `enable` (System.Boolean) — true if highlighting should be enabled.

**Returns:** `Boolean` — true if the object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Highlight.htm)

#### `public bool HighlightSubObject(ComponentIndex componentIndex, bool highlight)`

Highlights a sub-object.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.
- `highlight` (System.Boolean) — true if the sub-object should be highlighted.

**Returns:** `Boolean` — true if the sub-object is now highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_HighlightSubObject.htm)

#### `public bool InVisualAnalysisMode()`

Reports if any visual analysis mode is currently active for an object.

**Returns:** `Boolean` — true if an analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode.htm)

#### `public bool InVisualAnalysisMode(VisualAnalysisMode mode)`

Reports if a visual analysis mode is currently active for an object.

**Parameters:**
- `mode` (Rhino.Display.VisualAnalysisMode) — The mode to check for. Use null if you want to see if any mode is active.

**Returns:** `Boolean` — true if the specified analysis mode is active; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_InVisualAnalysisMode_1.htm)

#### `public virtual bool IsActiveInViewport(RhinoViewport viewport)`

Determine if this object is active in a particular viewport.

**Remarks:** The default implementation tests for space and viewport id. This handles things like testing if a page space object is visible in a modeling view.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsActiveInViewport(Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — True if the object is active in viewport

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsActiveInViewport.htm)

#### `public int IsHighlighted(bool checkSubObjects)`

Check highlight state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — If true and the entire object is not highlighted, and some subset of the object is highlighted, like some edges of a surface, then 3 is returned. If false and the entire object is not highlighted, then zero is returned.

**Returns:** `Int32` — 0: object is not highlighted.1: entire object is highlighted.3: one or more proper sub-objects are highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsHighlighted.htm)

#### `public virtual bool IsMeshable(MeshType meshType)`

Returns true if the object is capable of having a mesh of the specified type

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — [Missing <param name="meshType"/> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.IsMeshable(Rhino.Geometry.MeshType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsMeshable.htm)

#### `public bool IsSelectable()`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable.htm)

#### `public bool IsSelectable(bool ignoreSelectionState, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers are selectable. If false, then objects on locked layers are not selectable.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers are selectable. If false, then objects on hidden layers are not selectable.

**Returns:** `Boolean` — true if object is capable of being selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelectable_1.htm)

#### `public int IsSelected(bool checkSubObjects)`

Check selection state.

**Parameters:**
- `checkSubObjects` (System.Boolean) — (false is good default) If true and the entire object is not selected, and some subset of the object is selected, like some edges of a surface, then 3 is returned. If false and the entire object is not selected, then zero is returned.

**Returns:** `Int32` — 0 = object is not selected. 1 = object is selected. 2 = entire object is selected persistently. 3 = one or more proper sub-objects are selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSelected.htm)

#### `public bool IsSubObjectHighlighted(ComponentIndex componentIndex)`

Determines if a sub-object is highlighted.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — A sub-object component index.

**Returns:** `Boolean` — true if the sub-object is highlighted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectHighlighted.htm)

#### `public bool IsSubObjectSelectable(ComponentIndex componentIndex, bool ignoreSelectionState)`

Reports if a sub-object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — index of sub-object to check.
- `ignoreSelectionState` (System.Boolean) — If true, then selected objects are selectable. If false, then selected objects are not selectable.

**Returns:** `Boolean` — true if the specified sub-object can be selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelectable.htm)

#### `public bool IsSubObjectSelected(ComponentIndex componentIndex)`

Check sub-object selection state.

**Remarks:** A sub-object cannot be persistently selected.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.

**Returns:** `Boolean` — true if the sub-object is selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_IsSubObjectSelected.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `public void LockId()`

Locks the component Id property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockId.htm)

#### `public void LockIndex()`

Locks the component Index property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockIndex.htm)

#### `public void LockName()`

Locks the component Name property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_LockName.htm)

#### `public uint MemoryEstimate()`

Computes an estimate of the number of bytes that this object is using in memory. Note that this is a runtime memory estimate and does not directly compare to the amount of space take up by the object when saved to a file.

**Returns:** `UInt32` — The estimated number of bytes this object occupies in memory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MemoryEstimate.htm)

#### `public virtual int MeshCount(MeshType meshType, MeshingParameters parameters)`

RhinoObjects can have several different types of meshes and different numbers of meshes. A b-rep can have a render and an analysis mesh on each face. A mesh object has a single render mesh and no analysis mesh. Curve, point, and annotation objects have no meshes.

**Parameters:**
- `meshType` (Rhino.Geometry.MeshType) — type of mesh to count
- `parameters` (Rhino.Geometry.MeshingParameters) — if not null and if the object can change its mesh (like a brep), then only meshes that were created with these mesh parameters are counted.

**Returns:** `Int32` — number of meshes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_MeshCount.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnAddToDocument(RhinoDoc doc)`

This call informs an object it is about to be added to the list of active objects in the document.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnAddToDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnAddToDocument.htm)

#### `protected virtual void OnDeleteFromDocument(RhinoDoc doc)`

This call informs an object it is about to be deleted. Some objects, like clipping planes, need to do a little extra cleanup before they are deleted.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDeleteFromDocument(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDeleteFromDocument.htm)

#### `protected virtual void OnDraw(DrawEventArgs e)`

Called when Rhino wants to draw this object

**Parameters:**
- `e` (Rhino.Display.DrawEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDraw(Rhino.Display.DrawEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDraw.htm)

#### `protected virtual void OnDuplicate(RhinoObject source)`

Called when this a new instance of this object is created and copied from an existing object

**Parameters:**
- `source` (Rhino.DocObjects.RhinoObject) — [Missing <param name="source"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnDuplicate(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnDuplicate.htm)

#### `protected virtual IEnumerable<ObjRef> OnPick(PickContext context)`

Called to determine if this object or some sub-portion of this object should be picked given a pick context.

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

**Returns:** `IEnumerable<ObjRef>` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.OnPick(Rhino.Input.Custom.PickContext)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPick.htm)

#### `protected virtual void OnPicked(PickContext context, IEnumerable<ObjRef> pickedItems)`

Called when this object has been picked

**Parameters:**
- `context` (Rhino.Input.Custom.PickContext) — [Missing <param name="context"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnPicked(Rhino.Input.Custom.PickContext,System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef})"]
- `pickedItems` (System.Collections.Generic.IEnumerable<ObjRef>) — Items that were picked. This parameter is enumerable because there may have been multiple sub-objects picked

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnPicked.htm)

#### `protected virtual void OnSelectionChanged()`

Called when the selection state of this object has changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSelectionChanged.htm)

#### `protected virtual void OnSpaceMorph(SpaceMorph morph)`

Called when a space morph has been applied to the geometry. Currently this only works for CustomMeshObject instances

**Parameters:**
- `morph` (Rhino.Geometry.SpaceMorph) — [Missing <param name="morph"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnSpaceMorph(Rhino.Geometry.SpaceMorph)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnSpaceMorph.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `protected virtual void OnTransform(Transform transform)`

Called when a transformation has been applied to the geometry

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — [Missing <param name="transform"/> documentation for "M:Rhino.DocObjects.RhinoObject.OnTransform(Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_OnTransform.htm)

#### `public int Select(bool on)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select.htm)

#### `public int Select(bool on, bool syncHighlight)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and not highlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_1.htm)

#### `public int Select(bool on, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Selects an object.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `on` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected. Highlighting can be and stay out of sync, as its specification is independent.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected. If false, then objects on locked layers cannot be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable. If false, then objects on hidden layers cannot be selected.

**Returns:** `Int32` — 0: object is not selected.1: object is selected.2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_Select_2.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Int32` — 0: object is not selected 1: object is selected 2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject.htm)

#### `public int SelectSubObject(ComponentIndex componentIndex, bool select, bool syncHighlight, bool persistentSelect)`

Reports if an object can be selected.

**Remarks:** Objects that are locked, hidden, or on locked or hidden layers cannot be selected. If IsSelectableWithGripsOn() returns false, then an that object is not selectable if it has grips turned on.

**Parameters:**
- `componentIndex` (Rhino.Geometry.ComponentIndex) — Index of sub-object to check.
- `select` (System.Boolean) — The new selection state; true activates selection.
- `syncHighlight` (System.Boolean) — (default=true) If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — When true, selection persists even after the current command terminates.

**Returns:** `Int32` — 0: object is not selected1: object is selected2: object is selected persistently.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SelectSubObject_1.htm)

#### `public void SetCopyHistoryOnReplace(bool bCopy)`

If this object has a history record, the CopyOnReplace field is set When an object is replaced in a document and the old object has a history record with this field set, the history record is copied and attached to the new object. That allows a descendant object to continue the history linkage after it is edited.

**Parameters:**
- `bCopy` (System.Boolean) — [Missing <param name="bCopy"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCopyHistoryOnReplace(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCopyHistoryOnReplace.htm)

#### `public virtual void SetCustomRenderMeshParameter(Guid providerId, string parameterName, Object value)`

Set the named custom render mesh parameter value for this object.

**Parameters:**
- `providerId` (System.Guid) — Id of the custom render mesh provider
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetCustomRenderMeshParameter(System.Guid,System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetCustomRenderMeshParameter.htm)

#### `public bool SetRenderMeshParameters(MeshingParameters mp)`

Sets the per-object meshing parameters for this object. When set, this object will use these meshing parameters when generating a render mesh, instead of those provided by the document.

**Parameters:**
- `mp` (Rhino.Geometry.MeshingParameters) — The per-object meshing parameters. Note: if null, then the per-object meshing parameters will be removed, and this object will revert to using the meshing parameters provided by the document.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetRenderMeshParameters.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm)`

(Inherited from RhinoObject.)

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping.htm)

#### `public int SetTextureMapping(int channel, TextureMapping tm, Transform objectTransform)`

Sets texture mapping and mapping object transform for a channel

**Parameters:**
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `tm` (Rhino.Render.TextureMapping) — [Missing <param name="tm"/> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]
- `objectTransform` (Rhino.Geometry.Transform) — Mapping channel object transform

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.RhinoObject.SetTextureMapping(System.Int32,Rhino.Render.TextureMapping,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SetTextureMapping_1.htm)

#### `public virtual string ShortDescription(bool plural)`

Gets a localized short descriptive name of the object.

**Parameters:**
- `plural` (System.Boolean) — true if the descriptive name should in plural.

**Returns:** `String` — A string with the short localized descriptive name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_ShortDescription.htm)

#### `[ObsoleteAttribute] public bool SupportsRenderPrimitiveList(ViewportInfo viewport, bool preview)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList_1.htm)

#### `public bool SupportsRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Type of mesh to build. If attributes is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_SupportsRenderPrimitiveList.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

#### `public override string ToString()`

Returns the name of the model component type, and then its name and index.

**Remarks:** This is provided as a visual aid. Do not rely on this for serialization.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.ModelComponent.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_ToString.htm)

#### `public bool TryGetGumballFrame(out GumballFrame frame)`

If a Rhino object has been manipulated by Rhino's gumball, and the gumball is not in its default position, then the object's repositioned gumball frame is returned.

**Parameters:**
- `frame` (Rhino.UI.Gumball.GumballFrame) — The gumball frame.

**Returns:** `Boolean` — true if the object has a gumball frame, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetGumballFrame.htm)

#### `[ObsoleteAttribute] public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, bool preview, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `preview` (System.Boolean) — Type of mesh to build, if preview is true then a smaller mesh may be generated in less time, false is meant when actually rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox_1.htm)

#### `public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this object.

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_TryGetRenderPrimitiveBoundingBox.htm)

#### `public int UnhighlightAllSubObjects()`

Removes highlighting from all sub-objects.

**Returns:** `Int32` — The number of changed sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnhighlightAllSubObjects.htm)

#### `public int UnselectAllSubObjects()`

Removes selection from all sub-objects.

**Returns:** `Int32` — The number of unselected sub-objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_RhinoObject_UnselectAllSubObjects.htm)

### Properties
- `Attributes` (ObjectAttributes, get/set) — Gets or sets the object attributes.
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Document` (RhinoDoc, get) — Gets the document that owns this object.
- `Geometry` (GeometryBase, get) — Gets the underlying geometry for this object. All rhino objects are composed of geometry and attributes.
- `GripsOn` (Boolean, get/set) — Gets or sets the activation state of object default editing grips.
- `GripsSelected` (Boolean, get) — true if grips are turned on and at least one is selected.
- `GroupCount` (Int32, get) — Number of groups object belongs to.
- `HasDynamicTransform` (Boolean, get) — True if the object has a dynamic transformation
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasSubobjectMaterials` (Boolean, get) — Will be true if the object contains sub object meshes with materials that are different than the top level object.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get/set) — Every object has a Guid (globally unique identifier, also known as UUID, or universally unique identifier). The default value is Guid.Empty. When an object is added to a model, the value is checked. If the value is Guid.Empty, a new Guid is created. If the value is not null but it is already used by another object in the model, a new Guid is created. If the value is not Guid.Empty and it is not used by another object in the model, then that value persists. When an object is updated, by a move for example, the value of ObjectId persists. This value is the same as the one returned by this.Attributes.ObjectId.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — Gets or sets the model component index attribute.
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsDeletable` (Boolean, get) — Some objects cannot be deleted, like grips on lights and annotation objects.
- `IsDeleted` (Boolean, get) — true if the object is deleted. Deleted objects are kept by the document for undo purposes. Call RhinoDoc.UndeleteObject to undelete an object.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsHidden` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsInstanceDefinitionGeometry` (Boolean, get) — true if the object is used as part of an instance definition.
- `IsLocked` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsNormal` (Boolean, get) — An object must be in one of three modes: normal, locked or hidden. If an object is in normal mode, then the object's layer controls visibility and selectability. If an object is locked, then the object's layer controls visibility by the object cannot be selected. If the object is hidden, it is not visible and it cannot be selected.
- `IsPictureFrame` (Boolean, get) — Returns true if the object is a picture frame. A picture frame object is an object that displays a texture map in all views.
- `IsReference` (Boolean, get) — Gets a value indicating if an object is a reference object. An object from a work session reference model is a reference object and cannot be modified. An object is a reference object if, and only if, it is on a reference layer.
- `IsSolid` (Boolean, get) — Returns true if object is a closed solid, otherwise false.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Name` (String, get/set) — Rhino objects have optional text names. More than one object in a model can have the same name and some objects may have no name.
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `ObjectType` (ObjectType, get) — Gets the Rhino-based object type.
- `PointGeometry` (Point, get) — (Inherited from PointObject.)
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `RenderMaterial` (RenderMaterial, get/set) — Gets the render material associated with this object or null if there is none. This does not pay attention to the material source and will not check parent objects or layers for a RenderMaterial.
- `RuntimeSerialNumber` (UInt32, get) — Gets the objects runtime serial number.
- `SubobjectMaterialComponents` (ComponentIndex[], get) — (Inherited from RhinoObject.)
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `Visible` (Boolean, get) — Gets the object visibility.
- `WorksessionReferenceSerialNumber` (UInt32, get) — Obsolete - use ReferenceModelSerialNumber

## GripStatus (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.GripStatus"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_GripStatus.htm)

### Properties
- `Culled` (Boolean, get/set) — 
- `Visible` (Boolean, get) — 

## GripsDrawEventArgs (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.GripsDrawEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_GripsDrawEventArgs.htm)

### Methods
#### `public void DrawControlPolygonLine(Line line, GripStatus startStatus, GripStatus endStatus)`

Draws the lines in a control polygons. This is an helper function.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line between two grips.
- `startStatus` (Rhino.DocObjects.Custom.GripStatus) — Grip status at start of line.
- `endStatus` (Rhino.DocObjects.Custom.GripStatus) — Grip status at end of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_GripsDrawEventArgs_DrawControlPolygonLine.htm)

#### `public void DrawControlPolygonLine(Line line, int startStatus, int endStatus)`

Draws the lines in a control polygons. This is an helper function.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line between two grips.
- `startStatus` (System.Int32) — Index of Grip status at start of line.
- `endStatus` (System.Int32) — Index if Grip status at end of line.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_GripsDrawEventArgs_DrawControlPolygonLine_1.htm)

#### `public void DrawControlPolygonLine(Point3d start, Point3d end, int startStatus, int endStatus)`

Draws the lines in a control polygons. This is an helper function.

**Parameters:**
- `start` (Rhino.Geometry.Point3d) — The point start.
- `end` (Rhino.Geometry.Point3d) — The point end.
- `startStatus` (System.Int32) — Index of Grip status at start of line defined by start and end.
- `endStatus` (System.Int32) — Index if Grip status at end of line defined by start and end.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_GripsDrawEventArgs_DrawControlPolygonLine_2.htm)

#### `public GripStatus GripStatus(int index)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Custom.GripsDrawEventArgs.GripStatus(System.Int32)"]

**Returns:** `GripStatus` — [Missing <returns> documentation for "M:Rhino.DocObjects.Custom.GripsDrawEventArgs.GripStatus(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_GripsDrawEventArgs_GripStatus.htm)

#### `public void RestoreViewportSettings()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_GripsDrawEventArgs_RestoreViewportSettings.htm)

### Properties
- `ControlPolygonStyle` (Int32, get/set) — What kind of line is used to display things like control polygons. 0 = no control polygon, 1 = solid control polygon, 2 = dotted control polygon.
- `Display` (DisplayPipeline, get) — (Inherited from DrawEventArgs.)
- `DrawDynamicStuff` (Boolean, get) — If true, then draw stuff that does not move when grips are dragged, like the control polygon of the "original" curve.
- `DrawStaticStuff` (Boolean, get) — If true, then draw stuff that moves when grips are dragged, like the curve being bent by a dragged control point.
- `GripColor` (Color, get/set) — 
- `GripStatusCount` (Int32, get) — 
- `LockedGripColor` (Color, get/set) — 
- `RhinoDoc` (RhinoDoc, get) — (Inherited from DrawEventArgs.)
- `SelectedGripColor` (Color, get/set) — 
- `Viewport` (RhinoViewport, get) — (Inherited from DrawEventArgs.)

## TurnOnGripsEventHandler (delegate)

[Missing <summary> documentation for "T:Rhino.DocObjects.Custom.TurnOnGripsEventHandler"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_TurnOnGripsEventHandler.htm)

## UnknownUserData (class)

Represents user data with unknown origin.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_UnknownUserData.htm)

### Constructors
- `public UnknownUserData(IntPtr pointerNativeUserData)` — Constructs a new unknown data entity.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Finalize.htm)

#### `protected virtual void OnDuplicate(UserData source)`

Is called when the object is being duplicated.

**Parameters:**
- `source` (Rhino.DocObjects.Custom.UserData) — The source data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_OnDuplicate.htm)

#### `protected virtual void OnTransform(Transform transform)`

Is called when the object associated with this data is transformed. If you override this function, make sure to call the base class if you want the stored Transform to be updated.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — The transform being applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_OnTransform.htm)

#### `protected virtual bool Read(BinaryArchiveReader archive)`

Reads the content of this data from a stream archive.

**Parameters:**
- `archive` (Rhino.FileIO.BinaryArchiveReader) — An archive.

**Returns:** `Boolean` — true if the data was successfully written. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Read.htm)

#### `protected virtual bool Write(BinaryArchiveWriter archive)`

Writes the content of this data to a stream archive.

**Parameters:**
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — An archive.

**Returns:** `Boolean` — true if the data was successfully written. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Write.htm)

### Properties
- `Description` (String, get) — Descriptive name of the user data.
- `ShouldWrite` (Boolean, get) — If you want to save this user data in a 3dm file, override ShouldWrite and return true. If you do support serialization, you must also override the Read and Write functions.
- `Transform` (Transform, get) — Updated if user data is attached to a piece of geometry that is transformed and the virtual OnTransform() is not overridden. If you override OnTransform() and want Transform to be updated, then call the base class OnTransform() in your override. The default constructor sets Transform to the identity.

## UserData (class)

Provides a base class for custom classes of information which may be attached to geometry or attribute classes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_UserData.htm)

### Constructors
- `protected UserData()` — Initializes a new instance of the UserData class

### Methods
#### `public static void Copy(CommonObject source, CommonObject destination)`

Expert user tool that copies user data that has a positive CopyCount from the source object to a destination object. Generally speaking you don't need to use Copy(). Simply rely on things like the copy constructors to do the right thing.

**Parameters:**
- `source` (Rhino.Runtime.CommonObject) — A source object for the data.
- `destination` (Rhino.Runtime.CommonObject) — A destination object for the data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Copy.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Finalize.htm)

#### `public static Guid MoveUserDataFrom(CommonObject objectWithUserData)`

Moves the user data from objectWithUserData to a temporary data storage identified by the return Guid. When MoveUserDataFrom returns, the objectWithUserData will not have any user data.

**Parameters:**
- `objectWithUserData` (Rhino.Runtime.CommonObject) — Object with user data attached.

**Returns:** `Guid` — Guid identifier for storage of UserData that is held in a temporary list by this class. This function should be used in conjunction with MoveUserDataTo to transfer the user data to a different object. Returns Guid.Empty if there was no user data to transfer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_MoveUserDataFrom.htm)

#### `public static void MoveUserDataTo(CommonObject objectToGetUserData, Guid id, bool append)`

Moves the user data. See MoveUserDataFrom(CommonObject) for more information.

**Parameters:**
- `objectToGetUserData` (Rhino.Runtime.CommonObject) — Object data source.
- `id` (System.Guid) — Target.
- `append` (System.Boolean) — If the data should be appended or replaced.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_MoveUserDataTo.htm)

#### `protected virtual void OnDuplicate(UserData source)`

Is called when the object is being duplicated.

**Parameters:**
- `source` (Rhino.DocObjects.Custom.UserData) — The source data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_OnDuplicate.htm)

#### `protected virtual void OnTransform(Transform transform)`

Is called when the object associated with this data is transformed. If you override this function, make sure to call the base class if you want the stored Transform to be updated.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — The transform being applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_OnTransform.htm)

#### `protected virtual bool Read(BinaryArchiveReader archive)`

Reads the content of this data from a stream archive.

**Parameters:**
- `archive` (Rhino.FileIO.BinaryArchiveReader) — An archive.

**Returns:** `Boolean` — true if the data was successfully written. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Read.htm)

#### `protected virtual bool Write(BinaryArchiveWriter archive)`

Writes the content of this data to a stream archive.

**Parameters:**
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — An archive.

**Returns:** `Boolean` — true if the data was successfully written. The default implementation always returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Write.htm)

### Properties
- `Description` (String, get) — Descriptive name of the user data.
- `ShouldWrite` (Boolean, get) — If you want to save this user data in a 3dm file, override ShouldWrite and return true. If you do support serialization, you must also override the Read and Write functions.
- `Transform` (Transform, get) — Updated if user data is attached to a piece of geometry that is transformed and the virtual OnTransform() is not overridden. If you override OnTransform() and want Transform to be updated, then call the base class OnTransform() in your override. The default constructor sets Transform to the identity.

## UserDataList (class)

Represents a collection of user data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_UserDataList.htm)

### Methods
#### `public bool Add(UserData userdata)`

If the user-data is already in a different UserDataList, it will be removed from that list and added to this list.

**Parameters:**
- `userdata` (Rhino.DocObjects.Custom.UserData) — Data element.

**Returns:** `Boolean` — Whether this operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataList_Add.htm)

#### `public bool Contains(Guid userdataId)`

Checks for the existence of a specific type of user-data in this list Both .NET and native user-data is checked

**Parameters:**
- `userdataId` (System.Guid) — [Missing <param name="userdataId"/> documentation for "M:Rhino.DocObjects.Custom.UserDataList.Contains(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Custom.UserDataList.Contains(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataList_Contains.htm)

#### `public UserData Find(Type userdataType)`

Finds a specific data type in this regulated collection.

**Parameters:**
- `userdataType` (System.Type) — A data type.

**Returns:** `UserData` — The found data, or null of nothing was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataList_Find.htm)

#### `public IEnumerator<UserData> GetEnumerator()`

Get enumerator for UserDataList

**Returns:** `IEnumerator<UserData>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Custom.UserDataList.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataList_GetEnumerator.htm)

#### `public void Purge()`

Removes all user data from this geometry.

**Remarks:** User Remove(UserData) to delete a single, known, item.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataList_Purge.htm)

#### `public bool Remove(UserData userdata)`

Remove the user-data from this list

**Parameters:**
- `userdata` (Rhino.DocObjects.Custom.UserData) — [Missing <param name="userdata"/> documentation for "M:Rhino.DocObjects.Custom.UserDataList.Remove(Rhino.DocObjects.Custom.UserData)"]

**Returns:** `Boolean` — true if the user data was successfully removed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataList_Remove.htm)

### Properties
- `Count` (Int32, get) — Number of UserData objects in this list.
- `Item` (UserData, get) — Retrieve through indexer. Read-only access.

## UserDataListEnumerator (class)

Enumerator for UserDataList

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_UserDataListEnumerator.htm)

### Constructors
- `public UserDataListEnumerator(UserDataList udl)` — Create new UserDataListEnumerator

### Methods
#### `public void Dispose()`

Implement Dispose(). NOP.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataListEnumerator_Dispose.htm)

#### `public bool MoveNext()`

Advance enumerator to next UserData item.

**Returns:** `Boolean` — True if there is a next item.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataListEnumerator_MoveNext.htm)

#### `public void Reset()`

Reset the enumerator

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDataListEnumerator_Reset.htm)

### Properties
- `Current` (UserData, get) — Get current UserData on the enumerator.

## UserDictionary (class)

Defines the storage data class for a user dictionary.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Custom_UserDictionary.htm)

### Constructors
- `public UserDictionary()` — Initializes a new instance of the UserDictionary class

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_Finalize.htm)

#### `protected override void OnDuplicate(UserData source)`

Clones the user data.

**Parameters:**
- `source` (Rhino.DocObjects.Custom.UserData) — The source data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDictionary_OnDuplicate.htm)

#### `protected virtual void OnTransform(Transform transform)`

Is called when the object associated with this data is transformed. If you override this function, make sure to call the base class if you want the stored Transform to be updated.

**Parameters:**
- `transform` (Rhino.Geometry.Transform) — The transform being applied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserData_OnTransform.htm)

#### `protected override bool Read(BinaryArchiveReader archive)`

Is called to read this entity.

**Parameters:**
- `archive` (Rhino.FileIO.BinaryArchiveReader) — An archive.

**Returns:** `Boolean` — Always returns true.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDictionary_Read.htm)

#### `protected override bool Write(BinaryArchiveWriter archive)`

Is called to write this entity.

**Parameters:**
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — An archive.

**Returns:** `Boolean` — Always returns true.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Custom_UserDictionary_Write.htm)

### Properties
- `Description` (String, get) — Gets the text "RhinoCommon UserDictionary".
- `Dictionary` (ArchivableDictionary, get) — Gets the dictionary that is associated with this class. This dictionary is unique.
- `ShouldWrite` (Boolean, get) — Writes this entity if the count is larger than 0.
- `Transform` (Transform, get) — Updated if user data is attached to a piece of geometry that is transformed and the virtual OnTransform() is not overridden. If you override OnTransform() and want Transform to be updated, then call the base class OnTransform() in your override. The default constructor sets Transform to the identity.

