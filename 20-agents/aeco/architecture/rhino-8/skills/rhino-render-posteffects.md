---
name: rhino-rhino-render-posteffects
description: This skill encodes the rhino 8.0 surface of the Rhino.Render.PostEffects namespace — 19 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomPostEffectAttribute, PostEffect, PostEffectCollection, PostEffectChannel, PostEffectData, PostEffectExecutionControl, PostEffectJob, PostEffectJobChannels, and 11 more types.
---

# Rhino.Render.PostEffects

Auto-generated from vendor docs for rhino 8.0. 19 types in this namespace.

## CustomPostEffectAttribute (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.CustomPostEffectAttribute"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_CustomPostEffectAttribute.htm)

### Constructors
- `public CustomPostEffectAttribute(PostEffectType postEffectType, string name, PostEffectStyles styles = PostEffectStyles.ExecuteForProductionRendering|PostEffectStyles.ExecuteForRealtimeRendering|PostEffectStyles.DefaultOn, PostEffectExecuteWhileRenderingOptions executeWhileRenderingOption = PostEffectExecuteWhileRenderingOptions.Always, bool canDisplayHelp = false, int executeWhileRenderingDelay = 0)` — Initializes a new instance of the CustomPostEffectAttribute class

### Properties
- `CanDisplayHelp` (Boolean, get/set) — 
- `ExecuteWhileRenderingDelay` (Int32, get/set) — 
- `ExecuteWhileRenderingOption` (PostEffectExecuteWhileRenderingOptions, get/set) — 
- `Name` (String, get/set) — 
- `PostEffectType` (PostEffectType, get/set) — 
- `Styles` (PostEffectStyles, get/set) — 

## IPostEffects (interface)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.IPostEffects"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_IPostEffects.htm)

### Methods
#### `PostEffect[] GetPostEffects(PostEffectType type)`



**Parameters:**
- `type` (Rhino.Render.PostEffects.PostEffectType) — [Missing <param name="type"/> documentation for "M:Rhino.Render.PostEffects.IPostEffects.GetPostEffects(Rhino.Render.PostEffects.PostEffectType)"]

**Returns:** `PostEffect[]` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.IPostEffects.GetPostEffects(Rhino.Render.PostEffects.PostEffectType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_IPostEffects_GetPostEffects.htm)

#### `PostEffect PostEffectFromId(Guid uuid)`



**Parameters:**
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Render.PostEffects.IPostEffects.PostEffectFromId(System.Guid)"]

**Returns:** `PostEffect` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.IPostEffects.PostEffectFromId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_IPostEffects_PostEffectFromId.htm)

## PostEffect (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffect"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffect.htm)

### Constructors
- `public PostEffect()` — Initializes a new instance of the PostEffect class

### Methods
#### `public abstract void AddUISections(PostEffectUI ui)`

Create each of your UI sections using 'new' and then call ui.AddSection() on them. RDK takes ownership of the sections.If your post effect does not need a UI, then your implementation of this method can be a no-op.

**Parameters:**
- `ui` (Rhino.Render.PostEffects.PostEffectUI) — PostEffectUI

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_AddUISections.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.PostEffects.PostEffect.BeginChange(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_BeginChange.htm)

#### `public virtual bool CanExecute(PostEffectPipeline pipeline)`

Return true if the post effect can execute, else false. The base implementation checks if the post effect is 'On' and 'Shown'. Post effect authors can override this to include other criteria but cannot disable the base criteria.

**Parameters:**
- `pipeline` (Rhino.Render.PostEffects.PostEffectPipeline) — PostEffectPipeline

**Returns:** `Boolean` — Return true if the post effect can execute, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_CanExecute.htm)

#### `public void Changed()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_Changed.htm)

#### `public abstract bool DisplayHelp()`

Displays the post effect's help page, if any.

**Returns:** `Boolean` — Return true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_DisplayHelp.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffect

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_Dispose.htm)

#### `public virtual void Dispose(bool bDisposing)`

Releases the unmanaged resources used by the PostEffect and optionally releases the managed resources

**Parameters:**
- `bDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_Dispose_1.htm)

#### `public bool EndChange()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffect.EndChange"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_EndChange.htm)

#### `public abstract bool Execute(PostEffectPipeline pipeline, Rectangle rect)`

Execute the post effect.

**Parameters:**
- `pipeline` (Rhino.Render.PostEffects.PostEffectPipeline) — pipeline provides access to the post-effect pipeline.
- `rect` (System.Drawing.Rectangle) — rect is the pixel area to process.

**Returns:** `Boolean` — Return true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_Execute.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_Finalize.htm)

#### `public override int GetHashCode()`

A CRC of the state of this post effect.

**Returns:** `Int32` — returns a crc of post effect state

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_GetHashCode.htm)

#### `public abstract bool GetParam(string param, ref Object v)`

Get a parameter.

**Parameters:**
- `param` (System.String) — is the name of the parameter to get.
- `v` (System.Object) — accepts the parameter value.

**Returns:** `Boolean` — Returns true if successful or false if the parameter was not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_GetParam.htm)

#### `[ObsoleteAttribute("This function is no longer called")] public virtual bool ReadFromDocumentDefaults(RhinoDoc doc)`

Because post effects are now in the render settings, this function can no longer be called.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.PostEffects.PostEffect.ReadFromDocumentDefaults(Rhino.RhinoDoc)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffect.ReadFromDocumentDefaults(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_ReadFromDocumentDefaults.htm)

#### `public abstract bool ReadState(PostEffectState state)`

Read the state. If your post effect has no state, you must still return true for success.

**Parameters:**
- `state` (Rhino.Render.PostEffects.PostEffectState) — PostEffectState

**Returns:** `Boolean` — Return true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_ReadState.htm)

#### `public static Type[] RegisterPostEffect(Assembly assembly, Guid pluginId)`



**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.Render.PostEffects.PostEffect.RegisterPostEffect(System.Reflection.Assembly,System.Guid)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.Render.PostEffects.PostEffect.RegisterPostEffect(System.Reflection.Assembly,System.Guid)"]

**Returns:** `Type[]` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffect.RegisterPostEffect(System.Reflection.Assembly,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_RegisterPostEffect_1.htm)

#### `public static Type[] RegisterPostEffect(PlugIn plugin)`



**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.PostEffects.PostEffect.RegisterPostEffect(Rhino.PlugIns.PlugIn)"]

**Returns:** `Type[]` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffect.RegisterPostEffect(Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_RegisterPostEffect.htm)

#### `public abstract void ResetToFactoryDefaults()`

Reset the state to factory defaults.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_ResetToFactoryDefaults.htm)

#### `public abstract bool SetParam(string param, Object v)`

Set a parameter.

**Parameters:**
- `param` (System.String) — is the name of the parameter to set.
- `v` (System.Object) — specifies the type and value to set.

**Returns:** `Boolean` — Return true if successful or false if the parameter could not be set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_SetParam.htm)

#### `public abstract bool WriteState(ref PostEffectState state)`

Write the state. If your post effect has no state, you must still return true for success.

**Parameters:**
- `state` (Rhino.Render.PostEffects.PostEffectState) — PostEffectState

**Returns:** `Boolean` — Return true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_WriteState.htm)

#### `[ObsoleteAttribute("This function is no longer called")] public virtual bool WriteToDocumentDefaults(RhinoDoc doc)`

Because post effects are now in the render settings, this function can no longer be called.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.PostEffects.PostEffect.WriteToDocumentDefaults(Rhino.RhinoDoc)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffect.WriteToDocumentDefaults(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffect_WriteToDocumentDefaults.htm)

### Properties
- `CanDisplayHelp` (Boolean, get) — 
- `ExecuteWhileRenderingOption` (PostEffectExecuteWhileRenderingOptions, get) — 
- `Id` (Guid, get) — 
- `IsSelected` (Boolean, get) — Return true if the post effect is selected.
- `LocalName` (String, get) — 
- `On` (Boolean, get/set) — 
- `PostEffectType` (PostEffectType, get) — 
- `RequiredChannels` (Guid[], get) — The RDK calls this method to determine which channels a post effect requires. If a required channel is not available, the RDK will hide the post effect's UI and display explanatory text instead. Note: As a convenience, the default implementation adds IRhRdkRenderWindow::chanRGBA to the output array. Most post effects should be able to use this default with no need to override the method.
- `SerialNumber` (Int32, get/set) — 
- `Shown` (Boolean, get/set) — 
- `Styles` (PostEffectStyles, get) — 

## PostEffectChannel (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectChannel"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectChannel.htm)

### Methods
#### `public RenderWindow.Channel CPU()`

Return an interface to this channel for doing channel operations on the CPU.

**Returns:** `RenderWindow.Channel` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectChannel.CPU"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectChannel_CPU.htm)

#### `public PostEffectChannel Clone()`

Return a clone of this channel.

**Returns:** `PostEffectChannel` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectChannel.Clone"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectChannel_Clone.htm)

#### `public void Commit()`

Commit changes to the channel so that those changes can be used by subsequent post effects in the chain. Only valid for channels that were obtained by calling GetChannelForWrite(). If the channel has the same id as an existing channel, the existing channel will be replaced by the new one. If the existing channel was created by a previous post effect in the chain, it will be deleted. Changes to channels that are not commited simply get ignored. Note: This call merely sets a flag. The process is deferred until after the post effect has finished executing.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectChannel_Commit.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectChannel

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectChannel_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectChannel_Finalize.htm)

#### `public RenderWindow.ChannelGPU GPU()`

Return an interface to this channel for doing channel operations on the GPU.

**Returns:** `RenderWindow.ChannelGPU` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectChannel.GPU"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectChannel_GPU.htm)

### Properties
- `Id` (Guid, get) — Return the channel id which indicates what type of data is used in this channel.
- `PixelSize` (Int32, get) — Return the pixel size in bytes for this channel.
- `m_cpp` (IntPtr, get) — 

## PostEffectCollection (class)

Represents the collection of post effects in render settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectCollection.htm)

### Constructors
- `public PostEffectCollection()` — Create a utility object not associated with any document
- `public PostEffectCollection(PostEffectCollection c)` — Create a utility object not associated with any document from another object

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

(Inherited from DocumentOrFreeFloatingBase.)

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="cc"/> documentation for "M:Rhino.Render.DocumentOrFreeFloatingBase.BeginChange(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_CopyFrom.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectCollection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_Dispose.htm)

#### `public bool EndChange()`

(Inherited from DocumentOrFreeFloatingBase.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.DocumentOrFreeFloatingBase.EndChange"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `protected override void Finalize()`

(Overrides FreeFloatingBase.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_Finalize.htm)

#### `public IEnumerator<PostEffectData> GetEnumerator()`



**Returns:** `IEnumerator<PostEffectData>` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_GetEnumerator.htm)

#### `public bool GetSelectedPostEffect(PostEffectType type, out Guid id)`

Gets the selected post effect for a certain type into 'id'. Returns true if successful or false if the selection information could not be found.

**Parameters:**
- `type` (Rhino.Render.PostEffects.PostEffectType) — [Missing <param name="type"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.GetSelectedPostEffect(Rhino.Render.PostEffects.PostEffectType,System.Guid@)"]
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.GetSelectedPostEffect(Rhino.Render.PostEffects.PostEffectType,System.Guid@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.GetSelectedPostEffect(Rhino.Render.PostEffects.PostEffectType,System.Guid@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_GetSelectedPostEffect.htm)

#### `public bool MovePostEffectBefore(Guid id_move, Guid id_before)`

Move a post effect before another post effect in the list. Param 'id_move' is the id of the post effect to move. Param 'id_before' is the id of a post effect before which the post effect should be moved. If this is Guid.Empty, the post effect is moved to the end of the list. If the post effect identified by 'id_before' is not found, the method will fail. Returns true if successful, else false.

**Parameters:**
- `id_move` (System.Guid) — [Missing <param name="id_move"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.MovePostEffectBefore(System.Guid,System.Guid)"]
- `id_before` (System.Guid) — [Missing <param name="id_before"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.MovePostEffectBefore(System.Guid,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.MovePostEffectBefore(System.Guid,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_MovePostEffectBefore.htm)

#### `public PostEffectData PostEffectDataFromId(Guid id)`

Get a post effect data for an id.

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.PostEffectDataFromId(System.Guid)"]

**Returns:** `PostEffectData` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.PostEffectDataFromId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_PostEffectDataFromId.htm)

#### `public void SetSelectedPostEffect(PostEffectType type, Guid id)`

Sets the selected post effect for a certain type.

**Parameters:**
- `type` (Rhino.Render.PostEffects.PostEffectType) — [Missing <param name="type"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.SetSelectedPostEffect(Rhino.Render.PostEffects.PostEffectType,System.Guid)"]
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Render.PostEffects.PostEffectCollection.SetSelectedPostEffect(Rhino.Render.PostEffects.PostEffectType,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectCollection_SetSelectedPostEffect.htm)

## PostEffectData (class)

This is a wrapper around the data ('on', 'shown', 'state' parameters, etc.) of a post effect.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectData.htm)

### Methods
#### `public uint DataCRC(uint current_remainder)`

Get a CRC representing the state of the entire post effect.

**Parameters:**
- `current_remainder` (System.UInt32) — [Missing <param name="current_remainder"/> documentation for "M:Rhino.Render.PostEffects.PostEffectData.DataCRC(System.UInt32)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectData.DataCRC(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectData_DataCRC.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectData

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectData_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the PostEffectData and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectData_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectData_Finalize.htm)

#### `public IConvertible GetParameter(string param_name)`

Get an arbitrary parameter from this post effect by its name. If the parameter is not known to the post effect, the method will fail. Returns a variant object if successful or null on failure.

**Parameters:**
- `param_name` (System.String) — [Missing <param name="param_name"/> documentation for "M:Rhino.Render.PostEffects.PostEffectData.GetParameter(System.String)"]

**Returns:** `IConvertible` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectData.GetParameter(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectData_GetParameter.htm)

#### `public bool SetParameter(string param_name, Object param_value)`

Set an arbitrary parameter to the post effect by its name. If the parameter is not known to the post effect, the method will fail. Returns true if successful or false on failure.

**Parameters:**
- `param_name` (System.String) — [Missing <param name="param_name"/> documentation for "M:Rhino.Render.PostEffects.PostEffectData.SetParameter(System.String,System.Object)"]
- `param_value` (System.Object) — [Missing <param name="param_value"/> documentation for "M:Rhino.Render.PostEffects.PostEffectData.SetParameter(System.String,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectData.SetParameter(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectData_SetParameter.htm)

### Properties
- `Collection` (PostEffectCollection, get) — 
- `Id` (Guid, get) — Returns the unique id of this post effect.
- `LocalName` (String, get) — Returns the localized name of this post effect.
- `On` (Boolean, get/set) — The 'on' state of this post effect.
- `Shown` (Boolean, get/set) — The 'shown' state of this post effect.
- `Type` (PostEffectType, get) — Returns the type of this post effect.

## PostEffectExecuteContexts (enum)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectExecuteContexts"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectExecuteContexts.htm)

### Values
- `ProductionRendering` = `0`
- `RealtimeRendering` = `1`
- `ViewportDisplay` = `2`
- `ThumbnailCreation` = `3`
- `ConvertingToHDR` = `4`

## PostEffectExecuteWhileRenderingOptions (enum)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectExecuteWhileRenderingOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectExecuteWhileRenderingOptions.htm)

### Values
- `None` = `0`
- `Never` = `0`
- `Always` = `1`
- `UseDelay` = `2`
- `UseExecutionControl` = `3`

## PostEffectExecutionControl (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectExecutionControl"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectExecutionControl.htm)

### Constructors
- `public PostEffectExecutionControl()` — Initializes a new instance of the PostEffectExecutionControl class

### Methods
#### `public IntPtr Detach()`



**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectExecutionControl.Detach"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectExecutionControl_Detach.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectExecutionControl

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectExecutionControl_Dispose.htm)

#### `public virtual void Dispose(bool bDisposing)`

Releases the unmanaged resources used by the PostEffectExecutionControl and optionally releases the managed resources

**Parameters:**
- `bDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectExecutionControl_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectExecutionControl_Finalize.htm)

#### `public abstract bool ReadyToExecutePostEffect(Guid pep_id)`



**Parameters:**
- `pep_id` (System.Guid) — [Missing <param name="pep_id"/> documentation for "M:Rhino.Render.PostEffects.PostEffectExecutionControl.ReadyToExecutePostEffect(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectExecutionControl.ReadyToExecutePostEffect(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectExecutionControl_ReadyToExecutePostEffect.htm)

## PostEffectHistograms (enum)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectHistograms"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectHistograms.htm)

### Values
- `None` = `0`
- `BeforeEarlyEffects` = `1`
- `BeforeToneMapping` = `2`
- `AfterToneMapping` = `4`
- `AfterLateEffects` = `8`
- `All` = `15`
- `ToneMappingDisplay` = `6`
- `AfterEarlyEffects` = `2`
- `BeforeLateEffects` = `4`

## PostEffectJob (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectJob"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectJob.htm)

### Constructors
- `public PostEffectJob()` — Initializes a new instance of the PostEffectJob class

### Methods
#### `public abstract PostEffectJob Clone()`



**Returns:** `PostEffectJob` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectJob.Clone"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJob_Clone.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectJob

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJob_Dispose.htm)

#### `public virtual void Dispose(bool bDisposing)`

Releases the unmanaged resources used by the PostEffectJob and optionally releases the managed resources

**Parameters:**
- `bDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJob_Dispose_1.htm)

#### `public abstract bool Execute(Rectangle rect, PostEffectJobChannels access)`



**Parameters:**
- `rect` (System.Drawing.Rectangle) — [Missing <param name="rect"/> documentation for "M:Rhino.Render.PostEffects.PostEffectJob.Execute(System.Drawing.Rectangle,Rhino.Render.PostEffects.PostEffectJobChannels)"]
- `access` (Rhino.Render.PostEffects.PostEffectJobChannels) — [Missing <param name="access"/> documentation for "M:Rhino.Render.PostEffects.PostEffectJob.Execute(System.Drawing.Rectangle,Rhino.Render.PostEffects.PostEffectJobChannels)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectJob.Execute(System.Drawing.Rectangle,Rhino.Render.PostEffects.PostEffectJobChannels)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJob_Execute.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJob_Finalize.htm)

## PostEffectJobChannels (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectJobChannels"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectJobChannels.htm)

### Methods
#### `public void Dispose()`

Releases all resources used by the PostEffectJobChannels

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJobChannels_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJobChannels_Finalize.htm)

#### `public PostEffectChannel GetChannel(Guid channelId)`



**Parameters:**
- `channelId` (System.Guid) — [Missing <param name="channelId"/> documentation for "M:Rhino.Render.PostEffects.PostEffectJobChannels.GetChannel(System.Guid)"]

**Returns:** `PostEffectChannel` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectJobChannels.GetChannel(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectJobChannels_GetChannel.htm)

## PostEffectPipeline (class)

This object provides a way for post effects to access the frame buffer channels from a rendering and create new channels containing post-processed information which can be passed to the next post effect in the chain. Consider a simple post effect that just modifies the red component of a rendering.It will call GetChannel() to get the red channel as its input, and it will call NewChannel() to get a new red channel for its output. It will then read the input channel, do calculations and write to the output channel.When finished, it will call Commit() passing the new channel.Because both channels have the same identifier, this will replace the old channel with the new one so that subsequent post effects in the chain will use the new channel instead of the original.Note that this will only replace the channel used by the pipeline.The original channel will still exist in the frame buffer.This system allows any post effect to access any number of channels for reading and create any number of new channels which may or may not replace existing channels depending on the channel id.The final stage (convert to 8-bit) operates on the channels left in the pipeline by the post effect chain to produce the final 32-bit RGBA image in a dib. It is also possible for a post effect to create and use any number of 'scratch' channels.If a post effect needs a temporary pixel buffer for some intermediate results, it can call NewChannel() with a custom (random) id. Once it is finished with this scratch channel, it can call Discard() on it.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectPipeline.htm)

### Methods
#### `public Size Dimensions()`

Get the dimensions of the frame buffer. All channels in the frame buffer have the same dimensions.

**Returns:** `Size` — Dimension as Size

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_Dimensions.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectPipeline

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_Dispose.htm)

#### `public bool Execute(Rectangle p, bool renderingInProgress, PostEffectExecuteContexts usageContexts, PostEffectHistograms histogramsToUpdate)`

Execute the pipeline. This executes all the post effects in order. Only this rectangle need be modified by the post effects.

**Parameters:**
- `p` (System.Drawing.Rectangle) — p is a rectangle within the frame buffer.
- `renderingInProgress` (System.Boolean) — rendering is true if rendering is in progress.
- `usageContexts` (Rhino.Render.PostEffects.PostEffectExecuteContexts) — Context this pipeline is being executed in.
- `histogramsToUpdate` (Rhino.Render.PostEffects.PostEffectHistograms) — Bitwise list of histograms to update during the execution of the pipeline

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectPipeline.Execute(System.Drawing.Rectangle,System.Boolean,Rhino.Render.PostEffects.PostEffectExecuteContexts,Rhino.Render.PostEffects.PostEffectHistograms)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_Execute.htm)

#### `public Guid[] ExecutionOrder()`

Returns a list of the post effects to be executed by this pipeline in order.

**Returns:** `Guid[]` — A list of the post effects to be executed by this pipeline in order

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_ExecutionOrder.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_Finalize.htm)

#### `public PostEffectChannel GetChannelForRead(Guid id)`

Get a channel for reading. A post effect will use this to get channel data as input to its process. Output will be written to new channel(s). \see GetChannelForWrite() This method returns the current state of the channel at this stage in the pipeline. If the first post effect calls this, it will get the actual frame buffer channel. Subsequent post effects will get the data left behind by the previous post effect. A post effect calls GetChannelForRead() to get its input and GetChannelForWrite() to get the object to which it will write its output. Even when the same channel id is specified, these are separate, unconnected objects.

**Parameters:**
- `id` (System.Guid) — The channel identifier.

**Returns:** `PostEffectChannel` — A pointer to the channel or null if the channel is not available.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_GetChannelForRead.htm)

#### `public PostEffectChannel GetChannelForWrite(Guid id)`

Get a channel for writing. A post effect will use this to get channel(s) to write the output of its processing to. Input will usually come from existing channels, although a post effect is free to read its own output channels if needed. See GetChannelForRead() You are allowed to create one new channel with the same identifier as an existing channel, in which case IChannel::Commit() will replace the existing channel with the new one in the pipeline.

**Parameters:**
- `id` (System.Guid) — The channel identifier.

**Returns:** `PostEffectChannel` — A pointer to the new channel or null if the channel could not be created.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_GetChannelForWrite.htm)

#### `public ulong GetEndTimeInMilliseconds()`

Get the end time of the rendering expressed in milliseconds since some unspecified epoch. Do not make assumptions about what the epoch is; it might be different on different platforms.

**Returns:** `UInt64` — milliseconds

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_GetEndTimeInMilliseconds.htm)

#### `public float GetMaxLuminance()`

Get the max luminance in the rendering.

**Returns:** `Single` — max luminance

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_GetMaxLuminance.htm)

#### `public ulong GetStartTimeInMilliseconds()`

Get the start time of the rendering expressed in milliseconds since some unspecified epoch. Do not make assumptions about what the epoch is; it might be different on different platforms.

**Returns:** `UInt64` — milliseconds

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_GetStartTimeInMilliseconds.htm)

#### `public void SetStartTimeInMilliseconds(ulong ms)`

Set the start time of the rendering in milliseconds since some unspecified epoch.

**Parameters:**
- `ms` (System.UInt64) — milliseconds

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_SetStartTimeInMilliseconds.htm)

#### `public PostEffectThreadEngine ThreadEngine()`

Get the post effect thread engine.

**Returns:** `PostEffectThreadEngine` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectPipeline.ThreadEngine"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectPipeline_ThreadEngine.htm)

### Properties
- `GPUAllowed` (Boolean, get) — Post effect authors should check that GPU use is allowed before using the GPU in a post effect.
- `IsRendering` (Boolean, get) — IsRendering
- `RenderingId` (Guid, get) — Return a UUID that uniquely identifies the rendering being processed.

## PostEffectState (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectState"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectState.htm)

### Methods
#### `public void Dispose()`

Releases all resources used by the PostEffectState

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectState_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectState_Finalize.htm)

#### `public bool SetValue<T>(string name, T vValue)`



**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.PostEffects.PostEffectState.SetValue``1(System.String,``0)"]
- `vValue` (T) — [Missing <param name="vValue"/> documentation for "M:Rhino.Render.PostEffects.PostEffectState.SetValue``1(System.String,``0)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectState.SetValue``1(System.String,``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectState_SetValue__1.htm)

#### `public bool TryGetValue<T>(string name, out T vValue)`



**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.PostEffects.PostEffectState.TryGetValue``1(System.String,``0@)"]
- `vValue` (T) — [Missing <param name="vValue"/> documentation for "M:Rhino.Render.PostEffects.PostEffectState.TryGetValue``1(System.String,``0@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectState.TryGetValue``1(System.String,``0@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectState_TryGetValue__1.htm)

## PostEffectStyles (enum)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectStyles"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectStyles.htm)

### Values
- `ExecuteForProductionRendering` = `1`
- `ExecuteForRealtimeRendering` = `2`
- `ExecuteForViewportDisplay` = `4`
- `Fixed` = `256`
- `DefaultShown` = `512`
- `DefaultOn` = `1024`

## PostEffectThreadEngine (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectThreadEngine"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectThreadEngine.htm)

### Methods
#### `public void Dispose()`

Releases all resources used by the PostEffectThreadEngine

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectThreadEngine_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectThreadEngine_Finalize.htm)

#### `public bool RunPostEffect(PostEffectJob job, PostEffectPipeline pipeline, PostEffect plugin, Rectangle rect, Guid[] channels)`



**Parameters:**
- `job` (Rhino.Render.PostEffects.PostEffectJob) — [Missing <param name="job"/> documentation for "M:Rhino.Render.PostEffects.PostEffectThreadEngine.RunPostEffect(Rhino.Render.PostEffects.PostEffectJob,Rhino.Render.PostEffects.PostEffectPipeline,Rhino.Render.PostEffects.PostEffect,System.Drawing.Rectangle,System.Guid[])"]
- `pipeline` (Rhino.Render.PostEffects.PostEffectPipeline) — [Missing <param name="pipeline"/> documentation for "M:Rhino.Render.PostEffects.PostEffectThreadEngine.RunPostEffect(Rhino.Render.PostEffects.PostEffectJob,Rhino.Render.PostEffects.PostEffectPipeline,Rhino.Render.PostEffects.PostEffect,System.Drawing.Rectangle,System.Guid[])"]
- `plugin` (Rhino.Render.PostEffects.PostEffect) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.PostEffects.PostEffectThreadEngine.RunPostEffect(Rhino.Render.PostEffects.PostEffectJob,Rhino.Render.PostEffects.PostEffectPipeline,Rhino.Render.PostEffects.PostEffect,System.Drawing.Rectangle,System.Guid[])"]
- `rect` (System.Drawing.Rectangle) — [Missing <param name="rect"/> documentation for "M:Rhino.Render.PostEffects.PostEffectThreadEngine.RunPostEffect(Rhino.Render.PostEffects.PostEffectJob,Rhino.Render.PostEffects.PostEffectPipeline,Rhino.Render.PostEffects.PostEffect,System.Drawing.Rectangle,System.Guid[])"]
- `channels` (System.Guid[]) — [Missing <param name="channels"/> documentation for "M:Rhino.Render.PostEffects.PostEffectThreadEngine.RunPostEffect(Rhino.Render.PostEffects.PostEffectJob,Rhino.Render.PostEffects.PostEffectPipeline,Rhino.Render.PostEffects.PostEffect,System.Drawing.Rectangle,System.Guid[])"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.PostEffects.PostEffectThreadEngine.RunPostEffect(Rhino.Render.PostEffects.PostEffectJob,Rhino.Render.PostEffects.PostEffectPipeline,Rhino.Render.PostEffects.PostEffect,System.Drawing.Rectangle,System.Guid[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectThreadEngine_RunPostEffect.htm)

## PostEffectType (enum)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectType.htm)

### Values
- `Early` = `0`
- `ToneMapping` = `1`
- `Late` = `2`

## PostEffectUI (class)

PostEffectUI class provides a way for post effect plug-ins to add ui sections.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectUI.htm)

### Methods
#### `public void AddSection(ICollapsibleSection section)`

Add a section to the UI.

**Parameters:**
- `section` (Rhino.UI.Controls.ICollapsibleSection) — [Missing <param name="section"/> documentation for "M:Rhino.Render.PostEffects.PostEffectUI.AddSection(Rhino.UI.Controls.ICollapsibleSection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectUI_AddSection.htm)

#### `public void Dispose()`

Releases all resources used by the PostEffectUI

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectUI_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_PostEffects_PostEffectUI_Finalize.htm)

## PostEffectUuids (class)

[Missing <summary> documentation for "T:Rhino.Render.PostEffects.PostEffectUuids"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_PostEffects_PostEffectUuids.htm)

### Properties
- `Bloom` (Guid, get) — 
- `BriCon` (Guid, get) — 
- `DepthOfField` (Guid, get) — 
- `Dithering` (Guid, get) — 
- `Fog` (Guid, get) — 
- `Gamma` (Guid, get) — 
- `GaussianBlur` (Guid, get) — 
- `Glare` (Guid, get) — 
- `Glow` (Guid, get) — 
- `HueSatLum` (Guid, get) — 
- `Multiplier` (Guid, get) — 
- `Noise` (Guid, get) — 
- `ToneMapper_BlackWhitePoint` (Guid, get) — 
- `ToneMapper_Clamp` (Guid, get) — 
- `ToneMapper_FalseColor` (Guid, get) — 
- `ToneMapper_Filmic` (Guid, get) — 
- `ToneMapper_Logarithmic` (Guid, get) — 
- `Watermark` (Guid, get) — 
- `WireframeAnnotationsRGBA` (Guid, get) — 
- `WireframeCurvesRGBA` (Guid, get) — 
- `WireframeIsocurvesRGBA` (Guid, get) — 
- `WireframePointsRGBA` (Guid, get) — 

