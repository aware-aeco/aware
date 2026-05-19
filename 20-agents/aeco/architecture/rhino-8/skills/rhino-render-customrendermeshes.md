---
name: rhino-rhino-render-customrendermeshes
description: This skill encodes the rhino 8.0 surface of the Rhino.Render.CustomRenderMeshes namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomRenderMeshProviderAttribute, Instance, MeshProviderIds, RenderMeshes, RenderMeshProvider, RenderMeshProviderProgress, RenderMeshProvider.Flags.
---

# Rhino.Render.CustomRenderMeshes

Auto-generated from vendor docs for rhino 8.0. 7 types in this namespace.

## CustomRenderMeshProviderAttribute (class)

Attributes for RenderMeshProvider

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_CustomRenderMeshProviderAttribute.htm)

### Constructors
- `public CustomRenderMeshProviderAttribute(bool nonObjectIdsOnly = false)` — Constructor

### Properties
- `NonObjectIdsOnly` (Boolean, get/set) — If set to true, this will quickly return false for HasCustomRenderMeshes if the objectId is in the document.

## Instance (class)

The core of the custom render primitive delivery system - and instance defines a single mesh. Each instance has a shared mesh and a transform, along with a material, mapping channels (at the mesh and instance level).

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_Instance.htm)

### Constructors
- `public Instance()` — Construct a new Instance object

### Methods
#### `public void Dispose()`

Dispose method

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_Instance_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Dispose method.

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_Instance_Dispose_1.htm)

#### `protected override void Finalize()`

Finalizer to ensure correctly implemented dispose pattern.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_Instance_Finalize.htm)

### Properties
- `IsForcedMaterial` (Boolean, get/set) — The material for this instance should override any display mode materials.
- `IsRequestingPlugInDependent` (Boolean, get/set) — If this instance will change depending on the requesting plug-in.
- `IsViewDependent` (Boolean, get/set) — If this instance will change depending on the view direction.
- `Material` (RenderMaterial, get/set) — The material for this instance.
- `Mesh` (Mesh, get/set) — Returns the mesh associated with this geometry. Note that this function will always return a mesh even if the type is not a mesh...so you can always call this function without checking the type to give you the mesh. It's more efficient to use the primitive directly of course...but if you don't support it...
- `Transform` (Transform, get/set) — The transform for this instance. Transform the geometry by this to set its final location in world space.

## MeshProviderIds (class)

Built in primitive provider ids.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_MeshProviderIds.htm)

### Properties
- `CurvePiping` (Guid, get) — 
- `Displacement` (Guid, get) — 
- `EdgeSoftening` (Guid, get) — 
- `Shutlining` (Guid, get) — 
- `Thickening` (Guid, get) — 

## RenderMeshProvider (class)

A RenderMeshProvider delivers custom render primitives (in the form of RenderMeshes). Derive a public class from this with a public constructor, and this primitive provider will be added to the RDK forming a collection of providers. Each RenderMeshProvider::HasCustomRenderMeshes will be called and if it returns true, a call to RenderMeshes will be made. It is up to the provider to cache its own primitives - the IRenderMeshes::ProviderTracking class is provided for that. A provider may optionally return a collection of non-object Ids that it will provide custom render primitives for. An example of this is Grasshopper, which which will typically return a collection of the Ids of each CustomPreview component. Override this class if you are a plug-in developer intending to supply a custom set of primitives for a given object, or objectId. Examples of IRenderMeshProviders are CurvePiping, EdgeSoftening, Displacement, Shutlining, Grasshopper's CustomPreview component.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_RenderMeshProvider.htm)

### Constructors
- `protected RenderMeshProvider()` — Initializes a new instance of the RenderMeshProvider class

### Methods
#### `public void Dispose()`

Dispose method

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Dispose method. Note that you should only ever call Dispose if you explicitly registered this RenderMeshProvider using RegisterProvider If this provider was created using automatic registration, the Dispose function will thrown an exception.

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_Dispose_1.htm)

#### `protected override void Finalize()`

Finalizer to ensure correctly implemented dispose pattern.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_Finalize.htm)

#### `public virtual Object GetParameter(RhinoDoc doc, Guid objectId, string parameterName)`

Runtime access to specific parameters on for a given objectId relating to this primitive provider.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document of interest.
- `objectId` (System.Guid) — The objectId for which the parameter should be supplied.
- `parameterName` (System.String) — The name of the parameter.

**Returns:** `Object` — The value of the parameter for the specified ObjectId. IConvertable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_GetParameter.htm)

#### `public abstract bool HasCustomRenderMeshes(MeshType mt, ViewportInfo vp, RhinoDoc doc, Guid objectId, ref RenderMeshProvider.Flags flags, PlugIn plugin, DisplayPipelineAttributes attrs)`

Determine if custom render primitives will be supplied for a particular object.

**Parameters:**
- `mt` (Rhino.Geometry.MeshType) — The mesh type to provide meshes for
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered
- `doc` (Rhino.RhinoDoc) — The Rhino document associated with the objectId
- `objectId` (System.Guid) — The objectId for which the primitives should be supplied.
- `flags` (Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Flags) — See Flags.
- `plugin` (Rhino.PlugIns.PlugIn) — The requesting plug-in - typically the render plug-in requesting the primitives.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — The display attributes currently being used - nullptr if this is production rendering.

**Returns:** `Boolean` — True if RenderMeshes will return a set of primitives, otherwise false. RenderMeshes may still return an empty collection - in case of cancelation something similar.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_HasCustomRenderMeshes.htm)

#### `public virtual RenderMeshProviderProgress Progress(RhinoDoc doc, Guid[] optional_objectIds)`

Provides details on the progress of custom render meshes for a document or collection of objects

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Progress(Rhino.RhinoDoc,System.Guid[])"]
- `optional_objectIds` (System.Guid[]) — [Missing <param name="optional_objectIds"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Progress(Rhino.RhinoDoc,System.Guid[])"]

**Returns:** `RenderMeshProviderProgress` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Progress(Rhino.RhinoDoc,System.Guid[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_Progress.htm)

#### `public static RenderMeshProviderProgress[] ProgressForAll(RhinoDoc doc, Guid[] optional_objectIds)`

Returns a complete list of progress reports for all providers for either the document or a collection of objects

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.ProgressForAll(Rhino.RhinoDoc,System.Guid[])"]
- `optional_objectIds` (System.Guid[]) — [Missing <param name="optional_objectIds"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.ProgressForAll(Rhino.RhinoDoc,System.Guid[])"]

**Returns:** `RenderMeshProviderProgress[]` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.ProgressForAll(Rhino.RhinoDoc,System.Guid[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_ProgressForAll.htm)

#### `public static bool RegisterProvider(RenderMeshProvider provider, PlugIn plugin)`

Register RenderMeshProvider without relying on the auto-plug-in registration.

**Parameters:**
- `provider` (Rhino.Render.CustomRenderMeshes.RenderMeshProvider) — [Missing <param name="provider"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RegisterProvider(Rhino.Render.CustomRenderMeshes.RenderMeshProvider,Rhino.PlugIns.PlugIn)"]
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RegisterProvider(Rhino.Render.CustomRenderMeshes.RenderMeshProvider,Rhino.PlugIns.PlugIn)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RegisterProvider(Rhino.Render.CustomRenderMeshes.RenderMeshProvider,Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_RegisterProvider.htm)

#### `public static void RegisterProviders(Assembly assembly, PlugIn plugin)`

Register RenderMeshProviders without relying on the auto-plug-in registration

**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RegisterProviders(System.Reflection.Assembly,Rhino.PlugIns.PlugIn)"]
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RegisterProviders(System.Reflection.Assembly,Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_RegisterProviders_1.htm)

#### `public static void RegisterProviders(PlugIn plugin)`

Register RenderMeshProviders without relying on the auto-plug-in registration

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RegisterProviders(Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_RegisterProviders.htm)

#### `public abstract RenderMeshes RenderMeshes(MeshType mt, ViewportInfo vp, RhinoDoc doc, Guid objectId, List<InstanceObject> ancestry, ref RenderMeshProvider.Flags flags, RenderMeshes previousPrimitives, PlugIn plugin, DisplayPipelineAttributes attrs)`

Returns a complete set of custom render primitives for a given ObjectId.

**Parameters:**
- `mt` (Rhino.Geometry.MeshType) — The mesh type to provide meshes for
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered
- `doc` (Rhino.RhinoDoc) — The Rhino document associated with the objectId
- `objectId` (System.Guid) — The objectId for which the primitives should be supplied.
- `ancestry` (System.Collections.Generic.List<InstanceObject>) — The ancestry of the object - ie, block references referring to this definition. This is only used to handle ByParent attributes, and should not be used to transform geometry.
- `flags` (Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Flags) — See Flags.
- `previousPrimitives` (Rhino.Render.CustomRenderMeshes.RenderMeshes) — Optionally a custom set of initial primitives. If this parameter is empty, the standard render primitives for the object will be used, or an empty set of primitives will be used where they are not available. This is typically used in meshing exporters, which produce specific meshes for the output rather than using the render primitives.
- `plugin` (Rhino.PlugIns.PlugIn) — The requesting plug-in - typically the render plug-in requesting the meshes.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — The display attributes currently being used - nullptr if this is production rendering.

**Returns:** `RenderMeshes` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshProvider.RenderMeshes(Rhino.Geometry.MeshType,Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,System.Guid,System.Collections.Generic.List{Rhino.DocObjects.InstanceObject},Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Flags@,Rhino.Render.CustomRenderMeshes.RenderMeshes,Rhino.PlugIns.PlugIn,Rhino.Display.DisplayPipelineAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_RenderMeshes.htm)

#### `public virtual void SetParameter(RhinoDoc doc, Guid objectId, string parameterName, Object value)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document of interest.
- `objectId` (System.Guid) — The objectId for which the parameter should be supplied.
- `parameterName` (System.String) — The name of the parameter.
- `value` (System.Object) — The value to set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_SetParameter.htm)

### Properties
- `Name` (String, get) — The localized name of the provider for UI display.
- `NonObjectIds` (List<Guid>, get) — A provider may optionally return a collection of non-object Ids that it will provide custom render primitives for. An example of this is Grasshopper, which which will typically return a collection of the Ids of each CustomPreview component.
- `ProviderId` (Guid, get) — The localized name of the provider for UI display.
- `m_disposed` (Boolean, get) — m_disposed available for sub-classes to check.

## RenderMeshProvider.Flags (enum)

Flags to be passed into and returned from RenderMeshes and HasCustomRenderMeshes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_RenderMeshProvider_Flags.htm)

### Values
- `None` = `0` — No flags set.
- `Canceled` = `1` — If this flag is set when returned, then RenderMeshes was canceled.
- `DisableCaching` = `2` — Pass in this flag to bypass any primitive caching that the RenderMeshProviders might do.
- `Recursive` = `4` — Pass in this flag to get the render meshes for entire blocks. Only has an effect if the ObjectId is a block reference, and if it will not produce CRMs of its own.
- `IsDocumentObject` = `8` — Pass in this flag to indicate that the object id is a document resident object
- `AlwaysCopyDocumentContent` = `16` — Pass in this flag to ensure that materials, environments and textures that refer to document objects are always copied. This makes the RenderMeshes object safe to be stored outside of the lifetime of those objects - in caches, or for use in threaded situtations.
- `ReturnNullForStandardMaterial` = `32` — Pass in this flag to request that null is returned when the material on a mesh should be the standard material used - in otherwords, not customized by this RenderMeshes object. Note that this will require document access to resolve - do not use in threaded situations.
- `Incomplete` = `64` — This flag is returned when an asynchronous operation (like Displacement) has not yet completed.

## RenderMeshProviderProgress (class)

Expresses the progress of a custom render mesh provider.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_RenderMeshProviderProgress.htm)

### Constructors
- `public RenderMeshProviderProgress()` — Initializes a new instance of the RenderMeshProviderProgress class

### Properties
- `Amount` (Double, get/set) — The amount of progress towards the target
- `IsComplete` (Boolean, get/set) — true if complete, otherwise false.
- `ProviderId` (Guid, get/set) — The Id of the RenderMeshProvider that this report came from.
- `Target` (Double, get/set) — Target progress
- `Text` (String, get/set) — Return a textural progress report

## RenderMeshes (class)

RenderMeshes is a collection of geometry instances for a given ObjectId typically returned by the Custom Render primitive system. As each set of primitives is returned by the RenderMeshProviders in turn, the running hash is updated with new information about the primitive modifications that have been made along the way.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_CustomRenderMeshes_RenderMeshes.htm)

### Constructors
- `[ObsoleteAttribute] public RenderMeshes(RhinoDoc doc, Guid objectId, Guid renderMeshProviderId, uint runningHash)` — Create a new render primitives collection.
- `public RenderMeshes(RhinoDoc doc, Guid objectId, Guid renderMeshProviderId, uint runningHash, uint flags)` — Create a new render primitives collection.

### Methods
#### `public void AddInstance(Instance instance)`

Adds a new instance to the render primitives collection.

**Parameters:**
- `instance` (Rhino.Render.CustomRenderMeshes.Instance) — The instance to be added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshes_AddInstance.htm)

#### `public void Dispose()`

Dispose this collection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshes_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Dispose

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshes_Dispose_1.htm)

#### `protected override void Finalize()`

Finalizer for this collection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshes_Finalize.htm)

#### `public IEnumerator<Instance> GetEnumerator()`

Return an object to iterator over the instances in the collection

**Returns:** `IEnumerator<Instance>` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshes.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshes_GetEnumerator.htm)

#### `public override int GetHashCode()`

Overrides default hash code function - just returns Hash

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshes.RenderMeshes.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_CustomRenderMeshes_RenderMeshes_GetHashCode.htm)

### Properties
- `Document` (RhinoDoc, get) — The document with which this IRenderMeshes object is associated.
- `Hash` (UInt32, get/set) — The running hash for these render meshes.
- `InstanceCount` (Int32, get) — The number of instance objects in this collection.
- `ObjectId` (Guid, get) — The ObjectId with which this IRenderMeshes object is associated.

