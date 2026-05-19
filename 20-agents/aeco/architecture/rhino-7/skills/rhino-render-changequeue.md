---
name: rhino-rhino-render-changequeue
description: This skill encodes the rhino 7.0 surface of the Rhino.Render.ChangeQueue namespace — 16 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ChangeQueue, ClippingPlane, DisplayRenderSettings, Environment, DynamicObjectTransform, GroundPlane, Light, MappingChannelCollection, and 8 more types.
---

# Rhino.Render.ChangeQueue

Auto-generated from vendor docs for rhino 7.0. 16 types in this namespace.

## ChangeQueue (class)

Base class for ChangeQueue. Generally used by render plugins to build interactive updating of scenes that are being rendered.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_ChangeQueue.htm)

### Constructors
- `protected ChangeQueue(Guid pluginId, CreatePreviewEventArgs createPreviewEventArgs)` — Construct a new ChangeQueue using the given CreatePreviewEventArgs. The preview scene for these args will be used to populate the world.
- `[ObsoleteAttribute] protected ChangeQueue(Guid pluginId, uint docRuntimeSerialNumber, ViewInfo viewinfo, bool bRespectDisplayPipelineAttributes)` — Construct a new ChangeQueue from the given document
- `protected ChangeQueue(Guid pluginId, uint docRuntimeSerialNumber, ViewInfo viewinfo, DisplayPipelineAttributes attributes, bool bRespectDisplayPipelineAttributes, bool bNotifyChanges)` — Construct a new ChangeQueue from the given document, using given display pipeline attributes.

### Methods
#### `protected virtual void ApplyClippingPlaneChanges(Guid[] deleted, List<ClippingPlane> addedOrModified)`

Override this when you want to handle clippingplane changes

**Parameters:**
- `deleted` (System.Guid[]) — [Missing <param name="deleted"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyClippingPlaneChanges(System.Guid[],System.Collections.Generic.List{Rhino.Render.ChangeQueue.ClippingPlane})"]
- `addedOrModified` (System.Collections.Generic.List<ClippingPlane>) — [Missing <param name="addedOrModified"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyClippingPlaneChanges(System.Guid[],System.Collections.Generic.List{Rhino.Render.ChangeQueue.ClippingPlane})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyClippingPlaneChanges.htm)

#### `protected virtual void ApplyDisplayPipelineAttributesChanges(DisplayPipelineAttributes displayPipelineAttributes)`

Override if you need to react to display attribute changes. This function is needed to be able to react to different sample settings for i.e. capture preview rendering.

**Parameters:**
- `displayPipelineAttributes` (Rhino.Display.DisplayPipelineAttributes) — The changed DisplayPipelineAttributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyDisplayPipelineAttributesChanges.htm)

#### `protected virtual void ApplyDynamicClippingPlaneChanges(List<ClippingPlane> changed)`

Override this when you want to handle dynamicclippingplane changes

**Parameters:**
- `changed` (System.Collections.Generic.List<ClippingPlane>) — [Missing <param name="changed"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyDynamicClippingPlaneChanges(System.Collections.Generic.List{Rhino.Render.ChangeQueue.ClippingPlane})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyDynamicClippingPlaneChanges.htm)

#### `protected virtual void ApplyDynamicLightChanges(List<Light> dynamicLightChanges)`

Handle dynamic light changes

**Parameters:**
- `dynamicLightChanges` (System.Collections.Generic.List<Light>) — [Missing <param name="dynamicLightChanges"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyDynamicLightChanges(System.Collections.Generic.List{Rhino.Geometry.Light})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyDynamicLightChanges.htm)

#### `protected virtual void ApplyDynamicObjectTransforms(List<DynamicObjectTransform> dynamicObjectTransforms)`

Handle dynamic object transforms

**Parameters:**
- `dynamicObjectTransforms` (System.Collections.Generic.List<DynamicObjectTransform>) — [Missing <param name="dynamicObjectTransforms"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyDynamicObjectTransforms(System.Collections.Generic.List{Rhino.Render.ChangeQueue.DynamicObjectTransform})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyDynamicObjectTransforms.htm)

#### `protected virtual void ApplyEnvironmentChanges(RenderEnvironment.Usage usage)`

Override this when you want to handle environment changes

**Parameters:**
- `usage` (Rhino.Render.RenderEnvironment.Usage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyEnvironmentChanges(Rhino.Render.RenderEnvironment.Usage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyEnvironmentChanges.htm)

#### `protected virtual void ApplyGroundPlaneChanges(GroundPlane gp)`

Override this when you want to handle groundplane changes

**Parameters:**
- `gp` (Rhino.Render.ChangeQueue.GroundPlane) — [Missing <param name="gp"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyGroundPlaneChanges(Rhino.Render.ChangeQueue.GroundPlane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyGroundPlaneChanges.htm)

#### `protected virtual void ApplyLightChanges(List<Light> lightChanges)`

Override this when you want to handle light changes

**Parameters:**
- `lightChanges` (System.Collections.Generic.List<Light>) — [Missing <param name="lightChanges"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyLightChanges(System.Collections.Generic.List{Rhino.Render.ChangeQueue.Light})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyLightChanges.htm)

#### `protected virtual void ApplyLinearWorkflowChanges(LinearWorkflow lw)`

Override this when you want to handle linear workflow changes

**Parameters:**
- `lw` (Rhino.Render.LinearWorkflow) — [Missing <param name="lw"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyLinearWorkflowChanges(Rhino.Render.LinearWorkflow)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyLinearWorkflowChanges.htm)

#### `protected virtual void ApplyMaterialChanges(List<Material> mats)`

Override when you want to handle material changes

**Parameters:**
- `mats` (System.Collections.Generic.List<Material>) — List of ChangeQueue::Material that have changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyMaterialChanges.htm)

#### `protected virtual void ApplyMeshChanges(Guid[] deleted, List<Mesh> added)`

Override this when you want to handle mesh changes.

**Parameters:**
- `deleted` (System.Guid[]) — List of Guids to meshes that have been deleted
- `added` (System.Collections.Generic.List<Mesh>) — List of ChangeQueueMeshes that have been added or changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyMeshChanges.htm)

#### `protected virtual void ApplyMeshInstanceChanges(List<uint> deleted, List<MeshInstance> addedOrChanged)`

Override this when you want to handle mesh instance changes (block instances and first-time added new meshes)

**Parameters:**
- `deleted` (System.Collections.Generic.List<UInt32>) — [Missing <param name="deleted"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyMeshInstanceChanges(System.Collections.Generic.List{System.UInt32},System.Collections.Generic.List{Rhino.Render.ChangeQueue.MeshInstance})"]
- `addedOrChanged` (System.Collections.Generic.List<MeshInstance>) — [Missing <param name="addedOrChanged"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyMeshInstanceChanges(System.Collections.Generic.List{System.UInt32},System.Collections.Generic.List{Rhino.Render.ChangeQueue.MeshInstance})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyMeshInstanceChanges.htm)

#### `protected virtual void ApplyRenderSettingsChanges(DisplayRenderSettings settings)`

Override this when you want to handle render setting changes. These are for the viewport settings.

**Parameters:**
- `settings` (Rhino.Render.ChangeQueue.DisplayRenderSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyRenderSettingsChanges(Rhino.Render.ChangeQueue.DisplayRenderSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyRenderSettingsChanges.htm)

#### `protected virtual void ApplyRenderSettingsChanges(RenderSettings rs)`

Override this when you need to handle background changes (through RenderSettings)

**Parameters:**
- `rs` (Rhino.Render.RenderSettings) — [Missing <param name="rs"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplyRenderSettingsChanges(Rhino.Render.RenderSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyRenderSettingsChanges_1.htm)

#### `protected virtual void ApplySkylightChanges(Skylight skylight)`

Override this when you want to handle skylight changes

**Parameters:**
- `skylight` (Rhino.Render.ChangeQueue.Skylight) — [Missing <param name="skylight"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplySkylightChanges(Rhino.Render.ChangeQueue.Skylight)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplySkylightChanges.htm)

#### `protected virtual void ApplySunChanges(Light sun)`

Override this when you want to handle sun changes

**Parameters:**
- `sun` (Rhino.Geometry.Light) — [Missing <param name="sun"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ApplySunChanges(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplySunChanges.htm)

#### `protected virtual void ApplyViewChange(ViewInfo viewInfo)`

Override ApplyViewChange when you want to receive changes to the view/camera

**Parameters:**
- `viewInfo` (Rhino.DocObjects.ViewInfo) — The ViewInfo with the changes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ApplyViewChange.htm)

#### `public virtual bool AreViewsEqual(ViewInfo aView, ViewInfo bView)`

Compare to ViewInfo instances and decide whether they are equal or not. If you need to change the way the comparison is done you can override this function and implement your custom comparison.

**Parameters:**
- `aView` (Rhino.DocObjects.ViewInfo) — First ViewInfo to compare
- `bView` (Rhino.DocObjects.ViewInfo) — Second ViewInfo to compare

**Returns:** `Boolean` — true if the views are considered equal

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_AreViewsEqual.htm)

#### `protected virtual ChangeQueue.BakingFunctions BakeFor()`

Override this if you need to control baking behavior for textures. By default bake everything.

**Returns:** `ChangeQueue.BakingFunctions` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.BakeFor"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_BakeFor.htm)

#### `protected virtual int BakingSize(RhinoObject ro, RenderMaterial material, TextureType type)`

Override this if you need to control the size of the baked bitmaps for textures. By default the value returned is 2048.

**Parameters:**
- `ro` (Rhino.DocObjects.RhinoObject) — [Missing <param name="ro"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.BakingSize(Rhino.DocObjects.RhinoObject,Rhino.Render.RenderMaterial,Rhino.DocObjects.TextureType)"]
- `material` (Rhino.Render.RenderMaterial) — [Missing <param name="material"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.BakingSize(Rhino.DocObjects.RhinoObject,Rhino.Render.RenderMaterial,Rhino.DocObjects.TextureType)"]
- `type` (Rhino.DocObjects.TextureType) — [Missing <param name="type"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.BakingSize(Rhino.DocObjects.RhinoObject,Rhino.Render.RenderMaterial,Rhino.DocObjects.TextureType)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.BakingSize(Rhino.DocObjects.RhinoObject,Rhino.Render.RenderMaterial,Rhino.DocObjects.TextureType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_BakingSize.htm)

#### `public static void ConvertCameraBasedLightToWorld(ChangeQueue changequeue, Light light, ViewInfo vp)`

Convert given (camera-based) light to a world-based light (in-place)

**Parameters:**
- `changequeue` (Rhino.Render.ChangeQueue.ChangeQueue) — [Missing <param name="changequeue"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ConvertCameraBasedLightToWorld(Rhino.Render.ChangeQueue.ChangeQueue,Rhino.Render.ChangeQueue.Light,Rhino.DocObjects.ViewInfo)"]
- `light` (Rhino.Render.ChangeQueue.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ConvertCameraBasedLightToWorld(Rhino.Render.ChangeQueue.ChangeQueue,Rhino.Render.ChangeQueue.Light,Rhino.DocObjects.ViewInfo)"]
- `vp` (Rhino.DocObjects.ViewInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ConvertCameraBasedLightToWorld(Rhino.Render.ChangeQueue.ChangeQueue,Rhino.Render.ChangeQueue.Light,Rhino.DocObjects.ViewInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ConvertCameraBasedLightToWorld.htm)

#### `public static uint CrcFromGuid(Guid guid)`

Helper function to get a CRC from a Guid.

**Parameters:**
- `guid` (System.Guid) — [Missing <param name="guid"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.CrcFromGuid(System.Guid)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.CrcFromGuid(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_CrcFromGuid.htm)

#### `public void CreateWorld()`

Calls CreateWorld with true passed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_CreateWorld.htm)

#### `public void CreateWorld(bool bFlushWhenReady)`

Signal the queue to do the initialisation of the queue, seeding it with the content currently available.

**Parameters:**
- `bFlushWhenReady` (System.Boolean) — Set to true CreateWorld should automatically flush at the end. Note that the Flush called when true is passed doesn't apply changes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_CreateWorld_1.htm)

#### `public void Dispose()`

Dispose our ChangeQueue

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Dispose our ChangeQueue. Disposes unmanaged memory.

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_Dispose_1.htm)

#### `public RenderEnvironment EnvironmentForid(uint crc)`

Get RenderEnvironment for given RenderHash. Can return null.

**Parameters:**
- `crc` (System.UInt32) — [Missing <param name="crc"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.EnvironmentForid(System.UInt32)"]

**Returns:** `RenderEnvironment` — RenderEnvironment if render hash gives a match, null otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_EnvironmentForid.htm)

#### `public uint EnvironmentIdForUsage(RenderEnvironment.Usage usage)`

Get RenderEnvironment RenderHash for given usage.

**Parameters:**
- `usage` (Rhino.Render.RenderEnvironment.Usage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.EnvironmentIdForUsage(Rhino.Render.RenderEnvironment.Usage)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.EnvironmentIdForUsage(Rhino.Render.RenderEnvironment.Usage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_EnvironmentIdForUsage.htm)

#### `public void Flush()`

Tell the ChangeQueue to flush all collected events. This can trigger a host of Apply* calls. The following is the order in which those calls get made if there are changes for that specific data type: ApplyViewChange ApplyLinearWorkflowChanges ApplyDynamicObjectTransforms ApplyDynamicLightChanges ApplyRenderSettingsChanges ApplyEnvironmentChanges (background) ApplyEnvironmentChanges (refl) ApplyEnvironmentChanges (sky) ApplySkylightChanges ApplySunChanges ApplyLightChanges ApplyMaterialChanges ApplyMeshChanges ApplyMeshInstanceChanges ApplyGroundPlaneChanges ApplyClippingPlaneChanges

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_Flush.htm)

#### `public GroundPlane GetQueueGroundPlane()`

Get groundplane known to the queue at the time of the Flush

**Returns:** `GroundPlane` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.GetQueueGroundPlane"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_GetQueueGroundPlane.htm)

#### `public RenderSettings GetQueueRenderSettings()`

Get render settings known to the queue at the time of the Flush

**Returns:** `RenderSettings` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.GetQueueRenderSettings"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_GetQueueRenderSettings.htm)

#### `public BoundingBox GetQueueSceneBoundingBox()`

Get the scene bounding box

**Returns:** `BoundingBox` — Scene bounding box

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_GetQueueSceneBoundingBox.htm)

#### `public Skylight GetQueueSkylight()`

Get skylight known to the queue at the time of the Flush

**Returns:** `Skylight` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.GetQueueSkylight"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_GetQueueSkylight.htm)

#### `public Light GetQueueSun()`

Get sun known to the queue at the time of the Flush

**Returns:** `Light` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.GetQueueSun"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_GetQueueSun.htm)

#### `public ViewInfo GetQueueView()`

Get view known to the queue at the time of the Flush

**Returns:** `ViewInfo` — ViewInfo

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_GetQueueView.htm)

#### `public RenderMaterial MaterialFromId(uint crc)`

Get the RenderMaterial from the ChangeQueue material cache based on RenderHash

**Parameters:**
- `crc` (System.UInt32) — The RenderHash

**Returns:** `RenderMaterial` — RenderMaterial

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_MaterialFromId.htm)

#### `protected virtual void NotifyBeginUpdates()`

Override this when you want to be notified of begin of changes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_NotifyBeginUpdates.htm)

#### `protected virtual void NotifyDynamicUpdatesAreAvailable()`

Override this when you want to be notified dynamic updates are available

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_NotifyDynamicUpdatesAreAvailable.htm)

#### `protected virtual void NotifyEndUpdates()`

Override this when you want to be notified when change notifications have ended.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_NotifyEndUpdates.htm)

#### `public void OneShot()`

Call Flush() once, after that automatically dispose the queue.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_OneShot.htm)

#### `protected virtual bool ProvideOriginalObject()`

Override ProvideOriginalObject and return true if you want original objects supplied with ChangeQueue.Mesh. This will then also allow access to the Attributes.UserData of the original object from which the Mesh was generated.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.ChangeQueue.ProvideOriginalObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_ProvideOriginalObject.htm)

#### `public RenderTexture TextureForId(uint crc)`

Get RenderTexture for given RenderHash. Can return null.

**Parameters:**
- `crc` (System.UInt32) — render hash of the content to search for

**Returns:** `RenderTexture` — RenderTexture if found for render hash, null otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_ChangeQueue_TextureForId.htm)

### Properties
- `DisplayPipelineAttributes` (DisplayPipelineAttributes, get) — Get the DisplayPipelineAttributes if available, null otherwise
- `IsPreview` (Boolean, get) — Return true if this ChangeQueue is created for preview rendering. No view was set for preview rendering.
- `ViewId` (Guid, get) — Return view ID for a RhinoDoc based ChangeQueue. Returns Guid.Empty if no view was associated with the changequeue, i.e. preview rendering.

## ChangeQueue.BakingFunctions (enum)

Enumeration of functions for baking to conduct.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_ChangeQueue_BakingFunctions.htm)

### Values
- `None` = `0` — No baking
- `Decals` = `1` — Bake decals
- `ProceduralTextures` = `2` — Bake procedural textures
- `CustomObjectMappings` = `4` — Bake custom object mappings
- `WcsBasedMappings` = `8` — Bake WCS-based mappings
- `MultipleMappingChannels` = `16` — Bake multiple mapping channels
- `NoRepeatTextures` = `32` — Bake no-repeat textures
- `All` = `4294967295` — Bake everything

## ClippingPlane (class)

ChangeQueue clipping plane

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_ClippingPlane.htm)

### Properties
- `Attributes` (ObjectAttributes, get) — Get the ClippingPlaneObject for this clipping plane
- `Id` (Guid, get) — Get Guid for this clipping plane
- `IsEnabled` (Boolean, get) — True if clipping plane is enabled
- `Plane` (Plane, get) — Get the plane that represents this clipping plane
- `ViewIds` (List<Guid>, get) — Get list of View IDs this clipping plane is supposed to clip.

## DisplayRenderSettings (class)

ChangeQueue DisplayRenderSettings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_DisplayRenderSettings.htm)

### Properties
- `CullBackFaces` (Boolean, get) — True if backfaces should be culled
- `ForceFlatShading` (Boolean, get) — True if flat shading is forced
- `SceneLightingOn` (Boolean, get) — True if scene lighting is enabled

## DynamicObjectTransform (class)

ChangeQueue DynamicObject

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_DynamicObjectTransform.htm)

### Methods
#### `public override string ToString()`

String representation of DynamicObject

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.DynamicObjectTransform.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_DynamicObjectTransform_ToString.htm)

### Properties
- `MeshInstanceId` (UInt32, get) — Get the mesh instance id for this dynamic object.
- `Transform` (Transform, get) — Transform for the DynamicObject

## Environment (class)

ChangeQueue environment

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Environment.htm)

## Environment.FrameBufferFillMode (enum)

Fillmode for background

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Environment_FrameBufferFillMode.htm)

### Values
- `None` = `0` — None set
- `DefaultColor` = `1` — Use default color
- `SolidColor` = `2` — Use specified solid color
- `Gradient2Color` = `3` — Use 2-color gradient
- `Gradient4Color` = `4` — Use 4-color gradient (colors are specified by corners)
- `Bitmap` = `5` — Use bitmap
- `Renderer` = `6` — Use whatever renderer chooses
- `Transparent` = `7` — Transparent background
- `Force32Bit` = `4294967295` — Use 32bit color @todo verify what this means

## GroundPlane (class)

ChangeQueue ground plane

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_GroundPlane.htm)

### Properties
- `Altitude` (Double, get) — Get the altitude for ground plane
- `Crc` (UInt32, get) — Get the checksum of this groundplane object
- `Enabled` (Boolean, get) — Return true if ground plane is enabled
- `IsShadowOnly` (Boolean, get) — Get true if ground plane should be shadow-only
- `MaterialId` (UInt32, get) — The CRC / RenderHash of the material on this ground plane
- `ShowUnderside` (Boolean, get) — True if ground plane underside should be shown.
- `TextureOffset` (Vector2d, get) — Get texture offset on the ground plane
- `TextureRotation` (Double, get) — Get texture rotation on the ground plane
- `TextureScale` (Vector2d, get) — Get texture scale on the ground plane

## Light (class)

ChangeQueue Light change representation

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Light.htm)

### Properties
- `ChangeType` (Light.Event, get) — Get what type of light change this represents
- `Data` (Light, get) — Get the actual light data
- `Id` (Guid, get) — Get the light object id
- `IdCrc` (UInt32, get) — Get CRC computed from Id
- `MaterialId` (UInt32, get) — Get material ID of the material assigned to the light. Material ID is 0 if no material was assigned.

## Light.Event (enum)

Light change type

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Light_Event.htm)

### Values
- `Added` = `0` — Light was added
- `Deleted` = `1` — Light was deleted
- `Undeleted` = `2` — Light was undeleted
- `Modified` = `3` — Light was modified
- `Sorted` = `4` — Light was sorted in LightTable

## MappingChannel (class)

Mapping Channel for a ChangeQueue Mesh

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_MappingChannel.htm)

### Properties
- `Local` (Transform, get) — Local transform for the mapping
- `Mapping` (TextureMapping, get) — Return TexturMapping for this MappingChannel

## MappingChannelCollection (class)

MappingChannels for a Mesh

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_MappingChannelCollection.htm)

### Properties
- `Channels` (IEnumerable<MappingChannel>, get) — Enumerate all available channels in this mapping
- `Count` (Int32, get) — Get count of MappingChannels in this collection
- `Item` (MappingChannel, get) — Get MappingChannel on index

## Material (class)

Representation of a Material through the change queue

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Material.htm)

### Properties
- `Id` (UInt32, get) — Get the material ID (crc)
- `MeshIndex` (Int32, get) — Get mesh index
- `MeshInstanceId` (UInt32, get) — Get the material InstanceAncestry

## Mesh (class)

Representation of ChangeQueue Mesh

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Mesh.htm)

### Methods
#### `public Mesh[] GetMeshes()`

Get a SimpleArrayMeshPointer containing all meshes for the related Mesh

**Returns:** `Mesh[]` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.Mesh.GetMeshes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_Mesh_GetMeshes.htm)

#### `public Guid Id()`

Get the Object Guid this mesh is for.

**Returns:** `Guid` — Guid of parent object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_Mesh_Id.htm)

### Properties
- `Attributes` (ObjectAttributes, get) — Get object attributes of object associated to this mesh. This will be possible only after returning true from ChangeQueue.ProvideOriginalObject()
- `Mapping` (MappingChannelCollection, get) — Get the mapping for this mesh.
- `Object` (RhinoObject, get) — Get a copy of the original RhinoObject this Mesh is created from. Possible only after return true from ChangeQueue.ProvideOriginalObject(). Access this only with a using(var o = mesh.Object) {} construct. Note that the object is free floating, i.e. not part of a document.
- `SingleMapping` (MappingChannel, get) — Get texture mapping info as single mapping

## MeshInstance (class)

Representation of ChangeQueue MeshInstance

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_MeshInstance.htm)

### Properties
- `CastShadows` (Boolean, get) — Return true if mesh instance should cast shadows
- `Decals` (Decals, get) — Access to the decals for this MeshInstance if any. Null if no decal iterator exists for this mesh instance
- `GroupId` (UInt32, get) — Get identifier that specifies the grouping of these mesh instances - usually based on the object that this originally comprised.
- `InstanceId` (UInt32, get) — Get the instance identifier for this mesh instance.
- `MaterialId` (UInt32, get) — The Material CRC / RenderHash
- `MeshId` (Guid, get) — Get the mesh identifier for this mesh instance.
- `MeshIndex` (Int32, get) — Get the mesh index for this mesh instance.
- `OcsTransform` (Transform, get) — OCS Transform for the MeshInstance - this is the Object Coordinate System texture mapping transformation set in the texture mapping properties dialog. (identity means no OCS, potentially just simple WCS/WCS Box)
- `ParentId` (Guid, get) — Get the Guid of the object that is at the parent of the MeshInstance ancestry, or Guid.Empty if there is no parent.
- `ReceiveShadows` (Boolean, get) — Return true if mesh instance should receive shadows
- `RenderMaterial` (RenderMaterial, get) — RenderMaterial associated with mesh instance
- `RootId` (Guid, get) — Get the Guid of the object that is at the root of the MeshInstance ancestry, or Guid.Empty if this has no root.
- `Transform` (Transform, get) — Transform for the MeshInstance

## Skylight (class)

ChangeQueue skylight

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ChangeQueue_Skylight.htm)

### Methods
#### `public override string ToString()`

Textual representation of Skylight

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.ChangeQueue.Skylight.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ChangeQueue_Skylight_ToString.htm)

### Properties
- `Enabled` (Boolean, get) — Return true if skylight is enabled
- `ShadowIntensity` (Double, get) — Get shadow intensity for skylight. This is unused at present.
- `UsesCustomEnvironment` (Boolean, get) — Return true if skylight uses custom environment

