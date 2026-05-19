---
name: rhino-rhino-render
description: This skill encodes the rhino 7.0 surface of the Rhino.Render namespace — 155 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AddCustomUISections, AddCustomUISectionsEventArgs, AsyncRenderContext, CachedTextureCoordinates, ContentCollectionIterator, City, ContentUndoBlocker, ContentUndoHelper, and 147 more types.
---

# Rhino.Render

Auto-generated from vendor docs for rhino 7.0. 155 types in this namespace.

## AddCustomUISections (class)

This class contains the event to add custom ui sections when the content ui is created.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_AddCustomUISections.htm)

### Events
#### `OnAddCustomUISections` (`System.EventHandler<AddCustomUISectionsEventArgs>`)

**Signature:** `public static event EventHandler<AddCustomUISectionsEventArgs> OnAddCustomUISections`

This event is raised when a OnAddCustomUISections Event is triggered in rdk.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_AddCustomUISections_OnAddCustomUISections.htm)

## AddCustomUISectionsEventArgs (class)

Used as Rhino.Render Custom Events args.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_AddCustomUISectionsEventArgs.htm)

### Properties
- `EventType` (Guid, get) — The type of the event.
- `ExpandableContentUI` (ExpandableContentUI, get) — 

## AsyncRenderContext (class)

\ingroup rhino_render Inherit from AsyncRenderContext to be able to create asynchronous render engine implementations through RhinoCommon.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_AsyncRenderContext.htm)

### Constructors
- `protected AsyncRenderContext()` — Initializes a new instance of the AsyncRenderContext class

### Methods
#### `protected virtual void DeleteThis()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_DeleteThis.htm)

#### `public void Dispose()`

Releases all resources used by the AsyncRenderContext

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the AsyncRenderContext and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_Finalize.htm)

#### `public void JoinRenderThread()`

Join the render thread, then set to null;

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_JoinRenderThread.htm)

#### `public bool StartRenderThread(ThreadStart threadStart, string threadName)`

Start a new render thread with given function.

**Parameters:**
- `threadStart` (System.Threading.ThreadStart) — Function to start in render thread
- `threadName` (System.String) — Name for the thread

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.AsyncRenderContext.StartRenderThread(System.Threading.ThreadStart,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_StartRenderThread.htm)

#### `public virtual void StopRendering()`

Override StopRendering if you need to do additional tasks besides having Cancel set to true. Note: you should always base.StopRendering() in your overriding implementation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_AsyncRenderContext_StopRendering.htm)

### Properties
- `Cancel` (Boolean, get) — If set to true rendering should be stopped. Is set to true only by StopRendering().
- `RenderThread` (Thread, get) — Holder for render thread, that gets set through StartRenderThread()
- `RenderWindow` (RenderWindow, get/set) — Handle to the RenderWindow for the instance of this class. This is a convenience property for implementors to use.

## CachedTextureCoordinates (class)

Used for cached texture coordinates

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CachedTextureCoordinates.htm)

### Methods
#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public bool Contains(Point3d item)`

Determines whether this collection contains a specific value.

**Parameters:**
- `item` (Rhino.Geometry.Point3d) — [Missing <param name="item"/> documentation for "M:Rhino.Render.CachedTextureCoordinates.Contains(Rhino.Geometry.Point3d)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.CachedTextureCoordinates.Contains(Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CachedTextureCoordinates_Contains.htm)

#### `public void CopyTo(Point3d[] array, int arrayIndex)`

Copies the elements of the this collection to an System.Array, starting at a particular System.Array index.

**Parameters:**
- `array` (Rhino.Geometry.Point3d[]) — The one-dimensional System.Array that is the destination of the elements copied from this collection. The System.Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CachedTextureCoordinates_CopyTo.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Dispose_1.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Finalize.htm)

#### `public IEnumerator<Point3d> GetEnumerator()`

Returns an enumerator that iterates through this collection.

**Returns:** `IEnumerator<Point3d>` — A enumerator that can be used to iterate through this collection.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CachedTextureCoordinates_GetEnumerator.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public int IndexOf(Point3d item)`

Determines the index of a specific point in this collection.

**Parameters:**
- `item` (Rhino.Geometry.Point3d) — The point (UV or UVW) to locate in this collection.

**Returns:** `Int32` — The index of item if found in the list; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CachedTextureCoordinates_IndexOf.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

#### `public bool TryGetAt(int index, out double u, out double v, out double w)`

Use this method to iterate the cached texture coordinate array.

**Parameters:**
- `index` (System.Int32) — Index for the vertex to fetch.
- `u` (System.Double) — Output parameter which will receive the U value.
- `v` (System.Double) — Output parameter which will receive the V value.
- `w` (System.Double) — Output parameter which will receive the W value, this is only meaningful if Dim is 3.

**Returns:** `Boolean` — Returns true if index is valid; otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CachedTextureCoordinates_TryGetAt.htm)

### Properties
- `Count` (Int32, get) — Number of cached coordinates.
- `Dim` (Int32, get) — Coordinate dimension: 2 = UV, 3 = UVW
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsReadOnly` (Boolean, get) — This collection is always read-only
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `Item` (Point3d, get) — Gets the element at the specified index. Never call the set method, it will always throw a NotSupportedException because this list is read-only.
- `MappingId` (Guid, get) — The texture mapping Id.
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.

## City (class)

City

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_City.htm)

### Methods
#### `public static int Cities()`

Returns number of available cities.

**Returns:** `Int32` — City count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_City_Cities.htm)

#### `public static City CityAt(int index)`

Returns city at given index.

**Parameters:**
- `index` (System.Int32) — index.

**Returns:** `City` — City at index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_City_CityAt.htm)

#### `public static City FindNearest(double latitude, double longitude)`

Finds nearest city of specified input parameters.

**Parameters:**
- `latitude` (System.Double) — latitude.
- `longitude` (System.Double) — longitude.

**Returns:** `City` — Nearest city.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_City_FindNearest.htm)

### Properties
- `Latitude` (Double, get) — Gets latitude of city.
- `Longitude` (Double, get) — Gets longitude of city.
- `Name` (String, get) — Gets name of city.
- `TimeZone` (Double, get) — Gets time zone of city.

## ComponentOrders (enum)

Pixel component order for channels in the RenderWindow and PostEffects.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ComponentOrders.htm)

### Values
- `Irrelevant` = `0` — Used in single-value channels.
- `RGBA` = `1` — This is the default (to match Rhino 5)
- `ARGB` = `2` — ARGB component order
- `RGB` = `3` — This will only access 3 components, even in the case of the RGBA channel
- `BGR` = `4` — This will only access 3 components, even in the case of the RGBA channel.
- `ABGR` = `5` — ABGR component order
- `BGRA` = `6` — BGRA component order
- `XYZ` = `3` — For readability when using the Normal XYZ channel. Same as RGB
- `ZYX` = `4` — For readability when using the Normal XYZ channel. Same as BGR

## ContentCollectionIterator (class)

An iterator for the RenderContentCollection

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ContentCollectionIterator.htm)

### Constructors
- `public ContentCollectionIterator(IntPtr pCollection)` — Initializes a new instance of the ContentCollectionIterator class

### Methods
#### `public void DeleteThis()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentCollectionIterator_DeleteThis.htm)

#### `public void Dispose()`

Releases all resources used by the ContentCollectionIterator

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentCollectionIterator_Dispose.htm)

#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentCollectionIterator_Finalize.htm)

#### `public RenderContent First()`



**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.ContentCollectionIterator.First"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentCollectionIterator_First.htm)

#### `public RenderContent Next()`



**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.ContentCollectionIterator.Next"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentCollectionIterator_Next.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## ContentUndoBlocker (class)

[Missing <summary> documentation for "T:Rhino.Render.ContentUndoBlocker"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ContentUndoBlocker.htm)

### Constructors
- `public ContentUndoBlocker()` — Constructs a ContentUndoBlocker object inside a using block to block undo when modifying a RenderContent while a ContentUndoHelper is active. Alternatively - create the ContentUndoBlocker and explicitly call Dispose when you are done.

### Methods
#### `public void Dispose()`

Releases all resources used by the ContentUndoBlocker

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoBlocker_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the ContentUndoBlocker and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoBlocker_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoBlocker_Finalize.htm)

## ContentUndoHelper (class)

Content undo helper to be used with "using {}" to enclose a block of changes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ContentUndoHelper.htm)

### Constructors
- `public ContentUndoHelper(RhinoDoc doc, string description)` — Constructs a ContentUndoHelper object inside a using block to handle undo when modifying a RenderContent or - alternatively - create the ContentUndoHelper and explicitly call Dispose when you are done.

### Methods
#### `public bool AddContent(RenderContent content, RenderContent parent)`

Call this *after* adding a content. Undo will cause the content to be deleted.

**Parameters:**
- `content` (Rhino.Render.RenderContent) — Content you just added to the ContentList.
- `parent` (Rhino.Render.RenderContent) — is the content that will become the parent of the new content, or null if the new content is being added at the top level (i.e., not a child).

**Returns:** `Boolean` — true if the content was added.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoHelper_AddContent.htm)

#### `public void Dispose()`

Releases all resources used by the ContentUndoHelper

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoHelper_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the ContentUndoHelper and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoHelper_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoHelper_Finalize.htm)

#### `public bool ModifyContent(RenderContent content)`

Call this before modifying or deleting a content. Undo will cause the content to be restored.

**Parameters:**
- `content` (Rhino.Render.RenderContent) — Content you are about to modify.

**Returns:** `Boolean` — true if the content was modified.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoHelper_ModifyContent.htm)

#### `public bool TweakContent(RenderContent content, string parameterName)`

Call this before tweaking a single content parameter. Undo will cause the parameter to be restored.

**Parameters:**
- `content` (Rhino.Render.RenderContent) — The render content
- `parameterName` (System.String) — The parameter name you are about to change.

**Returns:** `Boolean` — true if the content was tweaked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ContentUndoHelper_TweakContent.htm)

## ContentUuids (class)

Content Guids of RenderContent provided by the RDK SDK. These Guids can be used to check against RenderContent.TypeId.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ContentUuids.htm)

### Properties
- `AdvancedDotTextureType` (Guid, get) — 
- `BasicEnvironmentCCI` (Guid, get) — 
- `BasicEnvironmentType` (Guid, get) — 
- `BasicMaterialCCI` (Guid, get) — 
- `BasicMaterialType` (Guid, get) — 
- `BitmapTextureType` (Guid, get) — 
- `BlendMaterialCCI` (Guid, get) — 
- `BlendMaterialType` (Guid, get) — 
- `BlendTextureType` (Guid, get) — 
- `CompositeMaterialCCI` (Guid, get) — 
- `CompositeMaterialType` (Guid, get) — 
- `CrossHatchBumpTexture` (Guid, get) — 
- `CubeMapTextureType` (Guid, get) — 
- `DefaultEnvironmentInstance` (Guid, get) — 
- `DefaultMaterialInstance` (Guid, get) — 
- `DisplayAttributeMaterialType` (Guid, get) — 
- `DotBumpTexture` (Guid, get) — 
- `DoubleSidedMaterialType` (Guid, get) — 
- `EmissionMaterialType` (Guid, get) — 
- `ExposureTextureType` (Guid, get) — 
- `EXRTextureType` (Guid, get) — 
- `FBmTextureType` (Guid, get) — 
- `GemMaterialType` (Guid, get) — 
- `GlassMaterialType` (Guid, get) — 
- `GradientTextureType` (Guid, get) — 
- `GraniteTextureType` (Guid, get) — 
- `GridTextureType` (Guid, get) — 
- `GritBumpTexture` (Guid, get) — 
- `HatchBumpTexture` (Guid, get) — 
- `HDRTextureType` (Guid, get) — 
- `LeatherBumpTexture` (Guid, get) — 
- `MarbleTextureType` (Guid, get) — 
- `MaskTextureType` (Guid, get) — 
- `MetalMaterialType` (Guid, get) — 
- `NoiseTextureType` (Guid, get) — 
- `PaintMaterialType` (Guid, get) — 
- `PerlinMarbleTextureType` (Guid, get) — 
- `PerturbingTextureType` (Guid, get) — 
- `PhysicallyBasedMaterialType` (Guid, get) — 
- `PictureMaterialType` (Guid, get) — 
- `PlasterMaterialType` (Guid, get) — 
- `PlasticMaterialType` (Guid, get) — 
- `ProjectionChangerTextureType` (Guid, get) — 
- `RealtimeDisplayMaterialType` (Guid, get) — 
- `ResampleTextureType` (Guid, get) — 
- `SimpleBitmapTextureType` (Guid, get) — 
- `SingleColorTextureType` (Guid, get) — 
- `SpeckleBumpTexture` (Guid, get) — 
- `StuccoTextureType` (Guid, get) — 
- `Texture2DCheckerTextureType` (Guid, get) — 
- `Texture3DCheckerTextureType` (Guid, get) — 
- `TextureAdjustmentTextureType` (Guid, get) — 
- `TileTextureType` (Guid, get) — 
- `TurbulenceTextureType` (Guid, get) — 
- `WavesTextureType` (Guid, get) — 
- `WoodBumpTexture` (Guid, get) — 
- `WoodTextureType` (Guid, get) — 

## ConvertibleExtensions (class)

Extension methods for IConvertible that work when an object is a Variant.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ConvertibleExtensions.htm)

### Methods
#### `public static Color4f ToColor4f(this IConvertible variant)`



**Parameters:**
- `variant` (System.IConvertible) — [Missing <param name="variant"/> documentation for "M:Rhino.Render.ConvertibleExtensions.ToColor4f(System.IConvertible)"]

**Returns:** `Color4f` — [Missing <returns> documentation for "M:Rhino.Render.ConvertibleExtensions.ToColor4f(System.IConvertible)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ConvertibleExtensions_ToColor4f.htm)

#### `public static Vector2d ToVector2d(this IConvertible variant)`



**Parameters:**
- `variant` (System.IConvertible) — [Missing <param name="variant"/> documentation for "M:Rhino.Render.ConvertibleExtensions.ToVector2d(System.IConvertible)"]

**Returns:** `Vector2d` — [Missing <returns> documentation for "M:Rhino.Render.ConvertibleExtensions.ToVector2d(System.IConvertible)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ConvertibleExtensions_ToVector2d.htm)

#### `public static Vector3d ToVector3d(this IConvertible variant)`



**Parameters:**
- `variant` (System.IConvertible) — [Missing <param name="variant"/> documentation for "M:Rhino.Render.ConvertibleExtensions.ToVector3d(System.IConvertible)"]

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.ConvertibleExtensions.ToVector3d(System.IConvertible)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_ConvertibleExtensions_ToVector3d.htm)

## CrcRenderHashFlags (enum)

[Missing <summary> documentation for "T:Rhino.Render.CrcRenderHashFlags"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CrcRenderHashFlags.htm)

### Values
- `Normal` = `0` — Normal render CRC; nothing is excluded.
- `ExcludeLinearWorkflow` = `1` — Linear Workflow color and texture changes are not included.
- `ExcludeLocalMapping` = `2` — Local mapping is excluded (only used by Textures).
- `ExcludeUnits` = `4` — Units are excluded (only used by Textures).
- `Reserved2` = `8` — Reserved for future use
- `ForSimulation` = `1` — Use this flag when simulating
- `ExcludeDocumentEffects` = `13` — Use this flag when you want to calculate the CRC independantly of any effect the document has on the content.

## CreatePreviewEventArgs (class)

Used in RenderPlugIn virtual CreatePreview function

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CreatePreviewEventArgs.htm)

### Methods
#### `public void SkipInitialisation()`

Call this if you don't want the argument to handle data initialisation. This is for use with the ChangeQueue

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CreatePreviewEventArgs_SkipInitialisation.htm)

### Properties
- `Cancel` (Boolean, get) — Get set by Rhino if the preview generation should be canceled for this
- `ContentKind` (RenderContentKind, get) — Description of content that preview is being generated for.
- `ContentTypeId` (Guid, get) — The class Id of content that preview is being generated for.
- `Environment` (RenderEnvironment, get) — The environment that the previewed object is rendered in.
- `Id` (Int32, get) — Unique Id for this scene.
- `Lights` (List<Light>, get) — 
- `Objects` (List<CreatePreviewEventArgs.SceneObject>, get) — 
- `PreviewContent` (RenderContent, get) — Obsolete, will return always null
- `PreviewImage` (Bitmap, get/set) — Initially null. If this image is set, then this image will be used for the preview. If never set, the default internal simulation preview will be used.
- `PreviewImageSize` (Size, get) — Pixel size of the image that is being requested for the preview scene
- `PreviewNotifier` (PreviewNotification, get) — 
- `Quality` (PreviewSceneQuality, get) — Quality of the preview image that is being requested for the preview scene
- `Reason` (CreatePreviewReason, get) — Reason the preview is getting generated
- `Viewport` (ViewportInfo, get) — 

## CreatePreviewEventArgs.SceneObject (class)

[Missing <summary> documentation for "T:Rhino.Render.CreatePreviewEventArgs.SceneObject"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CreatePreviewEventArgs_SceneObject.htm)

### Properties
- `Material` (RenderMaterial, get) — 
- `Mesh` (Mesh, get) — 

## CreatePreviewReason (enum)

Reason the content preview is being generated

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CreatePreviewReason.htm)

### Values
- `ContentChanged` = `0`
- `ViewChanged` = `1`
- `RefreshDisplay` = `2`
- `UpdateBitmap` = `3`
- `Other` = `99`

## CreateTexture2dPreviewEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.CreateTexture2dPreviewEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CreateTexture2dPreviewEventArgs.htm)

### Properties
- `PreviewImage` (Bitmap, get/set) — Initially null. If this image is set, then this image will be used for the preview. If never set, the default internal simulation preview will be used.
- `PreviewImageSize` (Size, get) — Pixel size of the image that is being requested for the preview scene

## CustomRenderContentAttribute (class)

Attributes for RenderContent

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CustomRenderContentAttribute.htm)

### Constructors
- `public CustomRenderContentAttribute(string renderEngineGuid = "", bool imageBased = false, string category = "General", bool is_elevated = false, bool is_built_in = false, bool is_private = false)` — Initializes a new instance of the CustomRenderContentAttribute class
- `public CustomRenderContentAttribute(string renderEngineGuid, bool imageBased, string category, bool is_elevated, bool is_built_in, bool is_private, bool is_linear, bool is_hdrcapable, bool is_normalmap)` — Initializes a new instance of the CustomRenderContentAttribute class

### Properties
- `Category` (String, get/set) — 
- `ImageBased` (Boolean, get/set) — 
- `IsBuiltIn` (Boolean, get/set) — 
- `IsElevated` (Boolean, get/set) — 
- `IsHdrCapable` (Boolean, get/set) — 
- `IsLinear` (Boolean, get/set) — 
- `IsNormalMap` (Boolean, get/set) — 
- `IsPrivate` (Boolean, get/set) — 
- `RenderEngineId` (Guid, get) — 

## CustomRenderMeshProvider (class)

You must call CustomRenderMeshProvider.RegisterProviders() from your plug-ins OnLoad override for each assembly containing a custom mesh provider. Only publicly exported classes derived from CustomRenderMeshProvider with a public constructor that has no parameters will get registered.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CustomRenderMeshProvider.htm)

### Constructors
- `protected CustomRenderMeshProvider()` — Default constructor

### Methods
#### `[ObsoleteAttribute("Use version that requires a document")] public static void AllObjectsChanged()`

Call this method if your render meshes change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_AllObjectsChanged.htm)

#### `public static void AllObjectsChanged(RhinoDoc doc)`

Call this method if your render meshes change.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.AllObjectsChanged(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_AllObjectsChanged_1.htm)

#### `public virtual BoundingBox BoundingBox(ViewportInfo vp, RhinoObject obj, Guid requestingPlugIn, bool preview)`

Returns a bounding box for the custom render meshes for the given object.

**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `obj` (Rhino.DocObjects.RhinoObject) — The Rhino object of interest. This can be null in the case where document meshes (not associated with any object) are being requested.
- `requestingPlugIn` (System.Guid) — UUID of the RDK plug-in requesting the meshes.
- `preview` (System.Boolean) — Type of mesh to build.

**Returns:** `BoundingBox` — A bounding box value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_BoundingBox_1.htm)

#### `public BoundingBox BoundingBox(ViewportInfo vp, RhinoObject obj, RhinoDoc doc, Guid requestingPlugIn, DisplayPipelineAttributes attrs)`



**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — [Missing <param name="attrs"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

**Returns:** `BoundingBox` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_BoundingBox.htm)

#### `public abstract bool BuildCustomMeshes(ViewportInfo vp, RenderPrimitiveList objMeshes, Guid requestingPlugIn, bool meshType)`

Build custom render mesh(es).

**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `objMeshes` (Rhino.Render.RenderPrimitiveList) — The meshes class to populate with custom meshes.
- `requestingPlugIn` (System.Guid) — UUID of the RDK plug-in requesting the meshes.
- `meshType` (System.Boolean) — Type of mesh to build.

**Returns:** `Boolean` — true if operation was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_BuildCustomMeshes.htm)

#### `public static void DocumentBasedMeshesChanged(RhinoDoc doc)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.DocumentBasedMeshesChanged(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_DocumentBasedMeshesChanged.htm)

#### `public static void ObjectChanged(RhinoDoc doc, RhinoObject obj)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.ObjectChanged(Rhino.RhinoDoc,Rhino.DocObjects.RhinoObject)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider.ObjectChanged(Rhino.RhinoDoc,Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_ObjectChanged.htm)

#### `public static void RegisterProviders(Assembly assembly, Guid pluginId)`

Call this method once from your plug-ins OnLoad override for each assembly containing a custom mesh provider. Only publicly exported classes derived from CustomRenderMeshProvider with a public constructor that has no parameters will get registered.

**Parameters:**
- `assembly` (System.Reflection.Assembly) — Assembly to search for valid CustomRenderMeshProvider derived classes.
- `pluginId` (System.Guid) — The plug-in that owns the custom mesh providers.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_RegisterProviders.htm)

#### `public abstract bool WillBuildCustomMeshes(ViewportInfo vp, RhinoObject obj, Guid requestingPlugIn, bool preview)`

Determines if custom render meshes will be built for a particular object.

**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `obj` (Rhino.DocObjects.RhinoObject) — The Rhino object of interest. This can be null in the case where document meshes (not associated with any object) are being requested.
- `requestingPlugIn` (System.Guid) — UUID of the RDK plug-in requesting the meshes.
- `preview` (System.Boolean) — Type of mesh to build.

**Returns:** `Boolean` — true if custom meshes will be built.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider_WillBuildCustomMeshes.htm)

### Properties
- `CurvePipingId` (Guid, get) — 
- `DisplacementId` (Guid, get) — 
- `EdgeSofteningId` (Guid, get) — 
- `Name` (String, get) — The name of the provider for UI display.
- `ShutLiningId` (Guid, get) — 
- `ThickeningId` (Guid, get) — 

## CustomRenderMeshProvider2 (class)

[Missing <summary> documentation for "T:Rhino.Render.CustomRenderMeshProvider2"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_CustomRenderMeshProvider2.htm)

### Constructors
- `protected CustomRenderMeshProvider2()` — Initializes a new instance of the CustomRenderMeshProvider2 class

### Methods
#### `public override BoundingBox BoundingBox(ViewportInfo vp, RhinoObject obj, Guid requestingPlugIn, bool preview)`

(Overrides CustomRenderMeshProvider.BoundingBox(ViewportInfo, RhinoObject, Guid, Boolean).)

**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]
- `preview` (System.Boolean) — [Missing <param name="preview"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]

**Returns:** `BoundingBox` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider2_BoundingBox_1.htm)

#### `public virtual BoundingBox BoundingBox(ViewportInfo vp, RhinoObject obj, RhinoDoc doc, Guid requestingPlugIn, DisplayPipelineAttributes attrs)`



**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — [Missing <param name="attrs"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

**Returns:** `BoundingBox` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BoundingBox(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider2_BoundingBox.htm)

#### `public override bool BuildCustomMeshes(ViewportInfo vp, RenderPrimitiveList objMeshes, Guid requestingPlugIn, bool preview)`

(Overrides CustomRenderMeshProvider.BuildCustomMeshes(ViewportInfo, RenderPrimitiveList, Guid, Boolean).)

**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.Render.RenderPrimitiveList,System.Guid,System.Boolean)"]
- `objMeshes` (Rhino.Render.RenderPrimitiveList) — [Missing <param name="objMeshes"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.Render.RenderPrimitiveList,System.Guid,System.Boolean)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.Render.RenderPrimitiveList,System.Guid,System.Boolean)"]
- `preview` (System.Boolean) — [Missing <param name="preview"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.Render.RenderPrimitiveList,System.Guid,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.Render.RenderPrimitiveList,System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider2_BuildCustomMeshes.htm)

#### `public abstract bool BuildCustomMeshes(ViewportInfo vp, RhinoDoc doc, RenderPrimitiveList objMeshes, Guid requestingPlugIn, DisplayPipelineAttributes attrs)`



**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,Rhino.Render.RenderPrimitiveList,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,Rhino.Render.RenderPrimitiveList,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `objMeshes` (Rhino.Render.RenderPrimitiveList) — [Missing <param name="objMeshes"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,Rhino.Render.RenderPrimitiveList,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,Rhino.Render.RenderPrimitiveList,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — [Missing <param name="attrs"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,Rhino.Render.RenderPrimitiveList,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.BuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.RhinoDoc,Rhino.Render.RenderPrimitiveList,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider2_BuildCustomMeshes_1.htm)

#### `public override bool WillBuildCustomMeshes(ViewportInfo vp, RhinoObject obj, Guid requestingPlugIn, bool preview)`

(Overrides CustomRenderMeshProvider.WillBuildCustomMeshes(ViewportInfo, RhinoObject, Guid, Boolean).)

**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]
- `preview` (System.Boolean) — [Missing <param name="preview"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider2_WillBuildCustomMeshes_1.htm)

#### `public abstract bool WillBuildCustomMeshes(ViewportInfo vp, RhinoObject obj, RhinoDoc doc, Guid requestingPlugIn, DisplayPipelineAttributes attrs)`



**Parameters:**
- `vp` (Rhino.DocObjects.ViewportInfo) — [Missing <param name="vp"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `requestingPlugIn` (System.Guid) — [Missing <param name="requestingPlugIn"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — [Missing <param name="attrs"/> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.CustomRenderMeshProvider2.WillBuildCustomMeshes(Rhino.DocObjects.ViewportInfo,Rhino.DocObjects.RhinoObject,Rhino.RhinoDoc,System.Guid,Rhino.Display.DisplayPipelineAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_CustomRenderMeshProvider2_WillBuildCustomMeshes.htm)

### Properties
- `Name` (String, get) — The name of the provider for UI display.

## Decal (class)

Represents a decal, or a picture that can be moved on an object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Decal.htm)

### Methods
#### `public IntPtr ConstPointer()`



**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Render.Decal.ConstPointer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_ConstPointer.htm)

#### `public static Decal Create(DecalCreateParams createParams)`



**Parameters:**
- `createParams` (Rhino.Render.DecalCreateParams) — [Missing <param name="createParams"/> documentation for "M:Rhino.Render.Decal.Create(Rhino.Render.DecalCreateParams)"]

**Returns:** `Decal` — [Missing <returns> documentation for "M:Rhino.Render.Decal.Create(Rhino.Render.DecalCreateParams)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_Create.htm)

#### `public List<NamedValue> CustomData()`

Gets custom data associated with this decal - see Rhino.Plugins.RenderPlugIn.ShowDecalProperties.

**Returns:** `List<NamedValue>` — The return value can be null if there is no data associated with this decal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_CustomData.htm)

#### `public void Dispose()`

Releases all resources used by the Decal

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_Dispose.htm)

#### `public void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the Decal and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_Finalize.htm)

#### `public TextureMapping GetTextureMapping()`

The TextureMapping of the decal.

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.Render.Decal.GetTextureMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_GetTextureMapping.htm)

#### `public IntPtr NonConstPointer()`



**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Render.Decal.NonConstPointer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_NonConstPointer.htm)

#### `public uint TextureRenderCRC(TextureRenderHashFlags rh)`

Get the texture render CRC for the referenced texture using the TextureRenderHashFlags given

**Parameters:**
- `rh` (Rhino.Render.TextureRenderHashFlags) — [Missing <param name="rh"/> documentation for "M:Rhino.Render.Decal.TextureRenderCRC(Rhino.Render.TextureRenderHashFlags)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.Decal.TextureRenderCRC(Rhino.Render.TextureRenderHashFlags)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_TextureRenderCRC.htm)

#### `public uint TextureRenderCRC(TextureRenderHashFlags rh, LinearWorkflow lw)`

Get the texture render CRC for the referenced texture using the TextureRenderHashFlags given along with the LinearWorkflow

**Parameters:**
- `rh` (Rhino.Render.TextureRenderHashFlags) — [Missing <param name="rh"/> documentation for "M:Rhino.Render.Decal.TextureRenderCRC(Rhino.Render.TextureRenderHashFlags,Rhino.Render.LinearWorkflow)"]
- `lw` (Rhino.Render.LinearWorkflow) — [Missing <param name="lw"/> documentation for "M:Rhino.Render.Decal.TextureRenderCRC(Rhino.Render.TextureRenderHashFlags,Rhino.Render.LinearWorkflow)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.Decal.TextureRenderCRC(Rhino.Render.TextureRenderHashFlags,Rhino.Render.LinearWorkflow)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_TextureRenderCRC_1.htm)

#### `public bool TryGetColor(Point3d point, Vector3d normal, ref Color4f colInOut, ref Point2d uvOut)`

Blend color with the decal color at a given point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — The point in space or, if the decal is uv-mapped, the uv-coordinate of that point.
- `normal` (Rhino.Geometry.Vector3d) — The face normal of the given point.
- `colInOut` (Rhino.Display.Color4f) — The color to blend the decal color to.
- `uvOut` (Rhino.Geometry.Point2d) — the UV on the texture that the color point was read from.

**Returns:** `Boolean` — true if the given point hits the decal, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_TryGetColor.htm)

#### `public void UVBounds(ref double minUOut, ref double minVOut, ref double maxUOut, ref double maxVOut)`

The UV bounds of the decal. Only used when mapping is UV.

**Parameters:**
- `minUOut` (System.Double) — [Missing <param name="minUOut"/> documentation for "M:Rhino.Render.Decal.UVBounds(System.Double@,System.Double@,System.Double@,System.Double@)"]
- `minVOut` (System.Double) — [Missing <param name="minVOut"/> documentation for "M:Rhino.Render.Decal.UVBounds(System.Double@,System.Double@,System.Double@,System.Double@)"]
- `maxUOut` (System.Double) — [Missing <param name="maxUOut"/> documentation for "M:Rhino.Render.Decal.UVBounds(System.Double@,System.Double@,System.Double@,System.Double@)"]
- `maxVOut` (System.Double) — [Missing <param name="maxVOut"/> documentation for "M:Rhino.Render.Decal.UVBounds(System.Double@,System.Double@,System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decal_UVBounds.htm)

### Properties
- `CRC` (Int32, get) — The decal CRC identifies a decal by its state. Multiple decals which would be exactly the same would have the same CRC and are culled from the system. If you store this value with the intention of using it to find the decal again later, you must update your stored value whenever the decal state changes. You can detect when a decal changes by watching for the OnUserDataTransformed event.
- `DecalMapping` (DecalMapping, get) — Gets the mapping of the decal.
- `DecalProjection` (DecalProjection, get) — Gets the decal's projection. Used only when mapping is planar.
- `EndLatitude` (Double, get) — Gets the end angle of the decal's arc of latitude or 'horizontal sweep'. This is actually a LONGITUDINAL angle. Only used when mapping is cylindrical or spherical.
- `EndLongitude` (Double, get) — Gets the end angle of the decal's arc of longitude or 'vertical sweep'. This is actually a LATITUDINAL angle. Only used when mapping is spherical.
- `Height` (Double, get) — Gets the height of the decal. Only used when mapping is cylindrical.
- `MapToInside` (Boolean, get) — Used only when mapping is cylindrical or spherical.
- `Origin` (Point3d, get) — Gets the origin of the decal in world space.
- `Radius` (Double, get) — Gets the radius of the decal. Only used when mapping is cylindrical or spherical.
- `StartLatitude` (Double, get) — Gets the start angle of the decal's arc of latitude or 'horizontal sweep'. This is actually a LONGITUDINAL angle. Only used when mapping is cylindrical or spherical.
- `StartLongitude` (Double, get) — Gets the start angle of the decal's arc of longitude or 'vertical sweep'. This is actually a LATITUDINAL angle. Only used when mapping is spherical.
- `TextureInstanceId` (Guid, get) — Gets the texture ID for this decal.
- `Transparency` (Double, get) — Gets the decal's transparency in the range 0 to 1.
- `VectorAcross` (Vector3d, get) — Gets the vector across. For cylindrical and spherical mapping, the vector is unitized.
- `VectorUp` (Vector3d, get) — For cylindrical and spherical mapping, the vector is unitized.

## DecalCreateParams (class)

Used by RhinoObject.AddDecal() to create and add a decal

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DecalCreateParams.htm)

### Constructors
- `public DecalCreateParams()` — Initializes a new instance of the DecalCreateParams class

### Properties
- `DecalMapping` (DecalMapping, get/set) — 
- `DecalProjection` (DecalProjection, get/set) — 
- `EndLatitude` (Double, get/set) — 
- `EndLongitude` (Double, get/set) — 
- `Height` (Double, get/set) — 
- `MapToInside` (Boolean, get/set) — 
- `MaxU` (Double, get/set) — 
- `MaxV` (Double, get/set) — 
- `MinU` (Double, get/set) — 
- `MinV` (Double, get/set) — 
- `Origin` (Point3d, get/set) — 
- `Radius` (Double, get/set) — 
- `StartLatitude` (Double, get/set) — 
- `StartLongitude` (Double, get/set) — 
- `TextureInstanceId` (Guid, get/set) — 
- `Transparency` (Double, get/set) — 
- `VectorAcross` (Vector3d, get/set) — 
- `VectorUp` (Vector3d, get/set) — 

## DecalMapping (enum)

[Missing <summary> documentation for "T:Rhino.Render.DecalMapping"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DecalMapping.htm)

### Values
- `Planar` = `0` — Planar mapping. Uses projection, origin, up and across vectors (not unitized).
- `Cylindrical` = `1` — Cylindrical mapping. Uses origin, up, across, height, radius, latitude start and stop.
- `Spherical` = `2` — Spherical mapping. Uses origin, up, across, radius, latitude/longitude start and stop.
- `UV` = `3` — UV mapping.

## DecalProjection (enum)

[Missing <summary> documentation for "T:Rhino.Render.DecalProjection"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DecalProjection.htm)

### Values
- `Forward` = `0` — Project forward
- `Backward` = `1` — Project backward
- `Both` = `2` — Project forward and backward

## Decals (class)

Represents all the decals of an object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Decals.htm)

### Methods
#### `public uint Add(Decal decal)`

Add a new Decal to the decals list, use Decal.Create to create a new decal instance to add.

**Parameters:**
- `decal` (Rhino.Render.Decal) — [Missing <param name="decal"/> documentation for "M:Rhino.Render.Decals.Add(Rhino.Render.Decal)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.Decals.Add(Rhino.Render.Decal)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decals_Add.htm)

#### `public void Clear()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decals_Clear.htm)

#### `public IEnumerator<Decal> GetEnumerator()`



**Returns:** `IEnumerator<Decal>` — [Missing <returns> documentation for "M:Rhino.Render.Decals.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decals_GetEnumerator.htm)

#### `public bool Remove(Decal decal)`



**Parameters:**
- `decal` (Rhino.Render.Decal) — [Missing <param name="decal"/> documentation for "M:Rhino.Render.Decals.Remove(Rhino.Render.Decal)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.Decals.Remove(Rhino.Render.Decal)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Decals_Remove.htm)

## Dithering (class)

This is the interface to linear workflow settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Dithering.htm)

### Constructors
- `public Dithering()` — Create a utility object not associated with any document
- `public Dithering(Dithering d)` — Create a utility object not associated with any document from another object

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.Dithering.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Dithering_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

### Properties
- `Method` (Dithering.Methods, get/set) — 
- `On` (Boolean, get/set) — 

## Dithering.Methods (enum)

Dithering algorithm.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Dithering_Methods.htm)

### Values
- `None` = `0` — No dithering
- `FloydSteinberg` = `1` — Floyd Steinberg algorithm
- `SimpleNoise` = `2` — Simple random noise

## DocumentOrFreeFloatingBase (class)

Base class for Rhino.Render objects that are owned by the document, or can be delivered separately from other functions. In general, you cannot create these objects yourself.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DocumentOrFreeFloatingBase.htm)

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public abstract void CopyFrom(FreeFloatingBase src)`

Copy from another object

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.FreeFloatingBase.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `protected override void Finalize()`

Handle destruction of the un-managed CPP object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_Finalize.htm)

## DynamicIconUsage (enum)

[Missing <summary> documentation for "T:Rhino.Render.DynamicIconUsage"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_DynamicIconUsage.htm)

### Values
- `TreeControl` = `0` — Used in a tree control (e.g., TreeGridView).
- `SubnodeControl` = `1` — Used in a sub-node control (\see CRhRdkSubNodeCtrl)
- `ContentControl` = `2` — Used in a content control (\see CRhRdkContentCtrl)
- `General` = `3` — Used in a custom user interface. The background will be plain white.

## FilterContentByUsage (enum)

Content collection filter value

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_FilterContentByUsage.htm)

### Values
- `None` = `0` — No filter in use
- `Used` = `1` — Display only used contents
- `Unused` = `2` — Display only unused contents

## FreeFloatingBase (class)

Base class for Rhino.Render objects that are owned by the document, or can be delivered separately from other functions. In general, you cannot create these objects yourself.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_FreeFloatingBase.htm)

### Methods
#### `public abstract void CopyFrom(FreeFloatingBase src)`

Copy from another object

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.FreeFloatingBase.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_CopyFrom.htm)

#### `protected override void Finalize()`

Handle destruction of the un-managed CPP object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_Finalize.htm)

## GroundPlane (class)

Represents an infinite plane for implementation by renderers. See SupportsFeature.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_GroundPlane.htm)

### Constructors
- `public GroundPlane()` — Create an utility object not associated with any document
- `public GroundPlane(GroundPlane g)` — Create an utility object not associated with any document from another object

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.GroundPlane.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_GroundPlane_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `protected override void Finalize()`

Handle destruction of the un-managed CPP object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_Finalize.htm)

### Properties
- `Altitude` (Double, get/set) — Height above world XY plane in model units.
- `AutoAltitude` (Boolean, get/set) — Determines whether the ground plane is fixed by the Altitude property, or whether it is automatically placed at the lowest point in the model.
- `Enabled` (Boolean, get/set) — Determines whether the document ground plane is enabled.
- `MaterialInstanceId` (Guid, get/set) — Id of material in material table for this ground plane.
- `ShadowOnly` (Boolean, get/set) — Determines whether the ground plane shows the material assigned, or whether it is transparent, but captures shadows.
- `ShowUnderside` (Boolean, get/set) — If this is off, the ground plane will not be visible when seen from below.
- `TextureOffset` (Vector2d, get/set) — Texture mapping offset in world units.
- `TextureOffsetLocked` (Boolean, get/set) — Texture offset locked.
- `TextureRotation` (Double, get/set) — Texture mapping rotation around world origin + offset in degrees.
- `TextureSize` (Vector2d, get/set) — Texture mapping single UV span size in world units.
- `TextureSizeLocked` (Boolean, get/set) — Texture size locked.

### Events
#### `Changed` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> Changed`

This event is raised when a GroundPlane property value is changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_GroundPlane_Changed.htm)

## ICurrentEnvironment (interface)

[Missing <summary> documentation for "T:Rhino.Render.ICurrentEnvironment"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ICurrentEnvironment.htm)

### Properties
- `ForAnyUsage` (RenderEnvironment, set) — 
- `ForBackground` (RenderEnvironment, get/set) — 
- `ForBackground_CheckMode` (RenderEnvironment, get/set) — 
- `ForLighting` (RenderEnvironment, get/set) — 
- `ForReflectionAndRefraction` (RenderEnvironment, get/set) — 

## ICurrentEnvironment_Get (interface)

[Missing <summary> documentation for "T:Rhino.Render.ICurrentEnvironment_Get"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ICurrentEnvironment_Get.htm)

### Properties
- `ForBackground` (RenderEnvironment, get) — 
- `ForBackground_CheckMode` (RenderEnvironment, get) — 
- `ForLighting` (RenderEnvironment, get) — 
- `ForReflectionAndRefraction` (RenderEnvironment, get) — 

## IRhRdkPreviewSceneServer_eRotationType (enum)

c# version of IRhRdkPreviewSceneServer eRotationType enum

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_IRhRdkPreviewSceneServer_eRotationType.htm)

### Values
- `Camera` = `0` — Camera
- `Object` = `1` — Object

## ImageFile (class)

Controls interaction with RDK render image files

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ImageFile.htm)

### Events
#### `Deleted` (`System.EventHandler<ImageFileEventArgs>`)

**Signature:** `public static event EventHandler<ImageFileEventArgs> Deleted`

Called when the RDK is cleaning up old render image files, a plug-in should delete any plug-in specific image files at this time.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_ImageFile_Deleted.htm)

#### `Loaded` (`System.EventHandler<ImageFileEventArgs>`)

**Signature:** `public static event EventHandler<ImageFileEventArgs> Loaded`

Generally called when the "RenderOpenLastRender" command is run, this event is raised after the render window has been created and the saved scene has been loaded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_ImageFile_Loaded.htm)

#### `Saved` (`System.EventHandler<ImageFileEventArgs>`)

**Signature:** `public static event EventHandler<ImageFileEventArgs> Saved`

Render image file saved, happens when a rendering completes. If a plug-in needs to save additional file information it should write it to the same folder as the Rhino render image file. Rhino will take care of deleting old data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_ImageFile_Saved.htm)

## ImageFileEvent (enum)

[Missing <summary> documentation for "T:Rhino.Render.ImageFileEvent"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ImageFileEvent.htm)

### Values
- `Saved` = `0` — Render image file has been successfully written
- `Loaded` = `1` — Render image file has been successfully loaded
- `Deleted` = `2` — Render image file was just deleted

## ImageFileEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.ImageFileEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ImageFileEventArgs.htm)

### Properties
- `EllapsedTime` (Int32, get) — 
- `Event` (ImageFileEvent, get) — 
- `FileName` (String, get) — 
- `RenderEngine` (String, get) — 
- `RenderEngineId` (Guid, get) — 
- `SessionId` (Guid, get) — 

## LightArray (class)

[Missing <summary> documentation for "T:Rhino.Render.LightArray"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_LightArray.htm)

### Constructors
- `public LightArray()` — Initializes a new instance of the LightArray class
- `public LightArray(IntPtr pLightArray)` — Initializes a new instance of the LightArray class

### Methods
#### `public void Append(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightArray.Append(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightArray_Append.htm)

#### `public int Count()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.LightArray.Count"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightArray_Count.htm)

#### `public void Dispose()`

Releases all resources used by the LightArray

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightArray_Dispose.htm)

#### `public Light ElementAt(int index)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Render.LightArray.ElementAt(System.Int32)"]

**Returns:** `Light` — [Missing <returns> documentation for "M:Rhino.Render.LightArray.ElementAt(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightArray_ElementAt.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightArray_Finalize.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## LightManagerSupport (class)

Base class for implementing custom light managers in .NET

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_LightManagerSupport.htm)

### Constructors
- `protected LightManagerSupport()` — Default constructor

### Methods
#### `public abstract bool DeleteLight(RhinoDoc doc, Light light, bool bUndelete)`

Delete light

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.DeleteLight(Rhino.RhinoDoc,Rhino.Geometry.Light,System.Boolean)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupport.DeleteLight(Rhino.RhinoDoc,Rhino.Geometry.Light,System.Boolean)"]
- `bUndelete` (System.Boolean) — [Missing <param name="bUndelete"/> documentation for "M:Rhino.Render.LightManagerSupport.DeleteLight(Rhino.RhinoDoc,Rhino.Geometry.Light,System.Boolean)"]

**Returns:** `Boolean` — If delete is successful, then return true, else return false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_DeleteLight.htm)

#### `public virtual bool GetLightSolo(RhinoDoc doc, Guid uuid_light)`

Returns the value of "ON_LIght::m_bOn" if the light is in solo storage, or false if not in solo storage (ie - this is the checkbox state on the light manager dialog)

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.GetLightSolo(Rhino.RhinoDoc,System.Guid)"]
- `uuid_light` (System.Guid) — [Missing <param name="uuid_light"/> documentation for "M:Rhino.Render.LightManagerSupport.GetLightSolo(Rhino.RhinoDoc,System.Guid)"]

**Returns:** `Boolean` — Returns true if the light is in solo storage, or false if not in solo storage

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_GetLightSolo.htm)

#### `public abstract void GetLights(RhinoDoc doc, ref LightArray light_array)`

Get all the lights that are associated to the light manager. The lights are added to the LightArray parameter passed to the GetLights method

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.GetLights(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]
- `light_array` (Rhino.Render.LightArray) — [Missing <param name="light_array"/> documentation for "M:Rhino.Render.LightManagerSupport.GetLights(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_GetLights.htm)

#### `public abstract void GroupLights(RhinoDoc doc, ref LightArray light_array)`

Creates a new group with the lights

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.GroupLights(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]
- `light_array` (Rhino.Render.LightArray) — [Missing <param name="light_array"/> documentation for "M:Rhino.Render.LightManagerSupport.GroupLights(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_GroupLights.htm)

#### `public abstract string LightDescription(RhinoDoc doc, ref Light light)`

Gets the string representation of the light description

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.LightDescription(Rhino.RhinoDoc,Rhino.Geometry.Light@)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupport.LightDescription(Rhino.RhinoDoc,Rhino.Geometry.Light@)"]

**Returns:** `String` — Returns the string representation of the light description

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_LightDescription.htm)

#### `public abstract bool LightFromId(RhinoDoc doc, Guid uuid, ref Light light)`

Get Rhino.Geometry.Light object associated to Guig uuid

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.LightFromId(Rhino.RhinoDoc,System.Guid,Rhino.Geometry.Light@)"]
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Render.LightManagerSupport.LightFromId(Rhino.RhinoDoc,System.Guid,Rhino.Geometry.Light@)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupport.LightFromId(Rhino.RhinoDoc,System.Guid,Rhino.Geometry.Light@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupport.LightFromId(Rhino.RhinoDoc,System.Guid,Rhino.Geometry.Light@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_LightFromId.htm)

#### `public virtual int LightsInSoloStorage(RhinoDoc doc)`

Returns the number of lights in solo storage - any number other than 0 means "in solo mode"

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.LightsInSoloStorage(Rhino.RhinoDoc)"]

**Returns:** `Int32` — Returns the number of lights in solo storage - any number other than 0 means "in solo mode"

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_LightsInSoloStorage.htm)

#### `public abstract void ModifyLight(RhinoDoc doc, Light light)`

Modify properties of the light

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.ModifyLight(Rhino.RhinoDoc,Rhino.Geometry.Light)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupport.ModifyLight(Rhino.RhinoDoc,Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_ModifyLight.htm)

#### `public abstract int ObjectSerialNumberFromLight(RhinoDoc doc, ref Light light)`

Get the object serial number of the light

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.ObjectSerialNumberFromLight(Rhino.RhinoDoc,Rhino.Geometry.Light@)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupport.ObjectSerialNumberFromLight(Rhino.RhinoDoc,Rhino.Geometry.Light@)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupport.ObjectSerialNumberFromLight(Rhino.RhinoDoc,Rhino.Geometry.Light@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_ObjectSerialNumberFromLight.htm)

#### `public void OnCustomLightEvent(RhinoDoc doc, LightMangerSupportCustomEvent le, ref Light light)`

Generates LightMangerSupportCustomEvent: light_added, light_deleted, light_undeleted, light_modified, light_sorted, The event triggers a Light table event that the rdk lightmanager listens too

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.OnCustomLightEvent(Rhino.RhinoDoc,Rhino.Render.LightMangerSupportCustomEvent,Rhino.Geometry.Light@)"]
- `le` (Rhino.Render.LightMangerSupportCustomEvent) — [Missing <param name="le"/> documentation for "M:Rhino.Render.LightManagerSupport.OnCustomLightEvent(Rhino.RhinoDoc,Rhino.Render.LightMangerSupportCustomEvent,Rhino.Geometry.Light@)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupport.OnCustomLightEvent(Rhino.RhinoDoc,Rhino.Render.LightMangerSupportCustomEvent,Rhino.Geometry.Light@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_OnCustomLightEvent.htm)

#### `public abstract bool OnEditLight(RhinoDoc doc, ref LightArray light_array)`

The default implementation of OnEditLight selects the lights and opens the Lights Properties page

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.OnEditLight(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]
- `light_array` (Rhino.Render.LightArray) — [Missing <param name="light_array"/> documentation for "M:Rhino.Render.LightManagerSupport.OnEditLight(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]

**Returns:** `Boolean` — Returns true if successful, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_OnEditLight.htm)

#### `public abstract Guid PluginId()`

The Guid of the plugin

**Returns:** `Guid` — Returns the Guid of the plugin

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_PluginId.htm)

#### `public static void RegisterLightManager(PlugIn plugin)`

Find and register classes that derive from LightManagerSupport from the given plug-in.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.LightManagerSupport.RegisterLightManager(Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_RegisterLightManager.htm)

#### `public static void RegisterProviders(Assembly assembly, Guid pluginId)`

Find and register classes that derive from RealtimeDisplayMode from the given plug-in. The plug-in is found in the given assembly

**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.Render.LightManagerSupport.RegisterProviders(System.Reflection.Assembly,System.Guid)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.Render.LightManagerSupport.RegisterProviders(System.Reflection.Assembly,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_RegisterProviders.htm)

#### `public abstract Guid RenderEngineId()`

The Guid of the render engine

**Returns:** `Guid` — Returns the Guid of the render engine that is associated with this light manager

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_RenderEngineId.htm)

#### `public virtual bool SetLightSolo(RhinoDoc doc, Guid uuid_light, bool bSolo)`

First checks to see if we are in "solo mode" - which means that there are any lights that respond "true" to IsInSoloStorage. If in solo mode: If bSolo = true Sets this light on. If bSolo = false If this is the last light "on", forces all lights out of solo mode. If there are other lights on, turns this light off. If not in solo mode: If bSolo = true Forces all lights into solo storage and sets this light on. If bSolo = false This shouldn't happen. Will cause an ASSERT

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.SetLightSolo(Rhino.RhinoDoc,System.Guid,System.Boolean)"]
- `uuid_light` (System.Guid) — [Missing <param name="uuid_light"/> documentation for "M:Rhino.Render.LightManagerSupport.SetLightSolo(Rhino.RhinoDoc,System.Guid,System.Boolean)"]
- `bSolo` (System.Boolean) — [Missing <param name="bSolo"/> documentation for "M:Rhino.Render.LightManagerSupport.SetLightSolo(Rhino.RhinoDoc,System.Guid,System.Boolean)"]

**Returns:** `Boolean` — Returns true if action is successful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_SetLightSolo.htm)

#### `public abstract void UnGroup(RhinoDoc doc, ref LightArray light_array)`

UnGroups the lights

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.LightManagerSupport.UnGroup(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]
- `light_array` (Rhino.Render.LightArray) — [Missing <param name="light_array"/> documentation for "M:Rhino.Render.LightManagerSupport.UnGroup(Rhino.RhinoDoc,Rhino.Render.LightArray@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupport_UnGroup.htm)

## LightManagerSupportClient (class)

[Missing <summary> documentation for "T:Rhino.Render.LightManagerSupportClient"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_LightManagerSupportClient.htm)

### Constructors
- `public LightManagerSupportClient(uint doc_uuid)` — Initializes a new instance of the LightManagerSupportClient class

### Methods
#### `public void DeleteLight(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupportClient.DeleteLight(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_DeleteLight.htm)

#### `public void Dispose()`

Releases all resources used by the LightManagerSupportClient

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_Finalize.htm)

#### `public Light GetLightFromId(Guid uuid)`



**Parameters:**
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Render.LightManagerSupportClient.GetLightFromId(System.Guid)"]

**Returns:** `Light` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.GetLightFromId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_GetLightFromId.htm)

#### `public bool GetLightSolo(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupportClient.GetLightSolo(Rhino.Geometry.Light)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.GetLightSolo(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_GetLightSolo.htm)

#### `public LightArray GetLights()`



**Returns:** `LightArray` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.GetLights"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_GetLights.htm)

#### `public void GroupLights(LightArray lights)`



**Parameters:**
- `lights` (Rhino.Render.LightArray) — [Missing <param name="lights"/> documentation for "M:Rhino.Render.LightManagerSupportClient.GroupLights(Rhino.Render.LightArray)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_GroupLights.htm)

#### `public string LightDescription(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupportClient.LightDescription(Rhino.Geometry.Light)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.LightDescription(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_LightDescription.htm)

#### `public int LightsInSoloStorage()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.LightsInSoloStorage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_LightsInSoloStorage.htm)

#### `public void ModifyLight(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupportClient.ModifyLight(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_ModifyLight.htm)

#### `public RhinoObject ObjectFromLight(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupportClient.ObjectFromLight(Rhino.Geometry.Light)"]

**Returns:** `RhinoObject` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.ObjectFromLight(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_ObjectFromLight.htm)

#### `public void OnEditLight(LightArray lights)`



**Parameters:**
- `lights` (Rhino.Render.LightArray) — [Missing <param name="lights"/> documentation for "M:Rhino.Render.LightManagerSupportClient.OnEditLight(Rhino.Render.LightArray)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_OnEditLight.htm)

#### `public bool SetLightSolo(Light light, bool bSolo)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.Render.LightManagerSupportClient.SetLightSolo(Rhino.Geometry.Light,System.Boolean)"]
- `bSolo` (System.Boolean) — [Missing <param name="bSolo"/> documentation for "M:Rhino.Render.LightManagerSupportClient.SetLightSolo(Rhino.Geometry.Light,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.LightManagerSupportClient.SetLightSolo(Rhino.Geometry.Light,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_SetLightSolo.htm)

#### `public void UnGroup(LightArray lights)`



**Parameters:**
- `lights` (Rhino.Render.LightArray) — [Missing <param name="lights"/> documentation for "M:Rhino.Render.LightManagerSupportClient.UnGroup(Rhino.Render.LightArray)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LightManagerSupportClient_UnGroup.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## LightMangerSupportCustomEvent (enum)

LightMangerSupportCustomEvent

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_LightMangerSupportCustomEvent.htm)

### Values
- `light_added` = `0`
- `light_deleted` = `1`
- `light_undeleted` = `2`
- `light_modified` = `3`
- `light_sorted` = `4`

## LinearWorkflow (class)

This is the interface to linear workflow settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_LinearWorkflow.htm)

### Constructors
- `public LinearWorkflow()` — Create a utility object not associated with any document
- `public LinearWorkflow(LinearWorkflow src)` — Create a utility object not associated with any document from another object

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

Create a copy of linearworkflow

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.LinearWorkflow.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LinearWorkflow_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `public override bool Equals(Object obj)`

Compare two LinearWorkflow objects. They are considered equal when their Hashes match.

**Parameters:**
- `obj` (System.Object) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.LinearWorkflow.Equals(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.LinearWorkflow.Equals(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LinearWorkflow_Equals.htm)

#### `public override int GetHashCode()`

Get hash code for this object. It is the Hash property cast to int.

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.LinearWorkflow.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_LinearWorkflow_GetHashCode.htm)

### Properties
- `Hash` (UInt32, get) — Linear workflow CRC
- `PostProcessFrameBuffer` (Boolean, get/set) — Linear workflow active state
- `PostProcessGamma` (Single, get/set) — Linear workflow gamma
- `PostProcessGammaOn` (Boolean, get/set) — Linear workflow gamma
- `PostProcessGammaReciprocal` (Single, get) — Linear workflow gamma
- `PreProcessColors` (Boolean, get/set) — Linear workflow active state
- `PreProcessGamma` (Single, get/set) — Linear workflow gamma
- `PreProcessTextures` (Boolean, get/set) — Linear workflow active state

## MappingTag (class)

Holds texture mapping information.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_MappingTag.htm)

### Constructors
- `public MappingTag()` — Initializes a new instance of the MappingTag class

### Properties
- `Id` (Guid, get/set) — Gets or sets a map globally unique identifier.
- `MappingCRC` (UInt32, get/set) — Gets or sets the cyclic redundancy check on the mapping. See also CRC32(UInt32, Byte[]).
- `MappingType` (TextureMappingType, get/set) — Gets or sets a texture mapping type: linear, cylinder, etc...
- `MeshTransform` (Transform, get/set) — Gets or sets a 4x4 matrix transform.

## MetaDataProxy (class)

ProxyClass for MetaData

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_MetaDataProxy.htm)

### Constructors
- `public MetaDataProxy()` — Constructor for MetaDataProxy

### Methods
#### `public void Dispose()`

Dispose for MetaDataProxy

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_MetaDataProxy_Dispose.htm)

#### `protected override void Finalize()`

Destructor for MetaDataProxy

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_MetaDataProxy_Finalize.htm)

#### `public void SetContentInstanceId(Guid uuid)`

Set Content instance id for meta data

**Parameters:**
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Render.MetaDataProxy.SetContentInstanceId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_MetaDataProxy_SetContentInstanceId.htm)

### Properties
- `CppPointer` (IntPtr, get) — MetaDataProxy c++ pointer

## NamedValue (class)

[Missing <summary> documentation for "T:Rhino.Render.NamedValue"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_NamedValue.htm)

### Constructors
- `public NamedValue(string name, Object value)` — Initializes a new instance of the NamedValue class

### Properties
- `Name` (String, get/set) — 
- `Value` (Object, get/set) — 

## PhysicallyBasedMaterial (class)

[Missing <summary> documentation for "T:Rhino.Render.PhysicallyBasedMaterial"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PhysicallyBasedMaterial.htm)

### Constructors
- `public PhysicallyBasedMaterial()` — Initializes a new instance of the PhysicallyBasedMaterial class

## PhysicallyBasedMaterial.ParametersNames (class)

Helper class with fields containing the names of fields available in our PBR implementation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PhysicallyBasedMaterial_ParametersNames.htm)

### Constructors
- `public ParametersNames()` — Initializes a new instance of the PhysicallyBasedMaterial.ParametersNames class

### Properties
- `AmbientOcclusion` (String, get) — Obsolete.
- `Anisotropic` (String, get) — Obsolete.
- `AnisotropicRotation` (String, get) — Obsolete.
- `BaseColor` (String, get) — Obsolete.
- `BRDF` (String, get) — Obsolete.
- `Bump` (String, get) — Obsolete.
- `Clearcoat` (String, get) — Obsolete.
- `ClearcoatBump` (String, get) — Obsolete.
- `ClearcoatRoughness` (String, get) — Obsolete.
- `Displacement` (String, get) — Obsolete.
- `Emission` (String, get) — Obsolete.
- `Metallic` (String, get) — Obsolete.
- `Normal` (String, get) — Obsolete.
- `Opacity` (String, get) — Obsolete.
- `OpacityIor` (String, get) — Obsolete.
- `OpacityRoughness` (String, get) — Obsolete.
- `Roughness` (String, get) — Obsolete.
- `Sheen` (String, get) — Obsolete.
- `SheenTint` (String, get) — Obsolete.
- `Smudge` (String, get) — Obsolete.
- `Specular` (String, get) — Obsolete.
- `SpecularTint` (String, get) — Obsolete.
- `Subsurface` (String, get) — Obsolete.
- `SubsurfaceScatteringColor` (String, get) — Obsolete.
- `SubsurfaceScatteringRadius` (String, get) — Obsolete.

## PixelBuffer (class)

[Missing <summary> documentation for "T:Rhino.Render.PixelBuffer"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PixelBuffer.htm)

### Constructors
- `public PixelBuffer(IntPtr bufferPointer)` — Create a new PixelBuffer pointing to the (unmanaged) pixel data buffer behind IntPtr

### Properties
- `Buffer` (IntPtr, get) — 

## PreviewAppearance (class)

PreviewAppearance class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PreviewAppearance.htm)

### Constructors
- `public PreviewAppearance(IntPtr pRenderContent)` — Constructor for previewappearance

### Methods
#### `public PreviewBackground Background()`

Background

**Returns:** `PreviewBackground` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.Background"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_Background.htm)

#### `public void Dispose()`

Dispose for previewappearance

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_Dispose.htm)

#### `protected override void Finalize()`

Destructor for previewappearance

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_Finalize.htm)

#### `public void FromMetaData(MetaData md)`

Copy data from MetaData to PreviewAppearance

**Parameters:**
- `md` (Rhino.Render.DataSources.MetaData) — [Missing <param name="md"/> documentation for "M:Rhino.Render.PreviewAppearance.FromMetaData(Rhino.Render.DataSources.MetaData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_FromMetaData.htm)

#### `public PreviewGeometry Geometry()`

Geometry

**Returns:** `PreviewGeometry` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.Geometry"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_Geometry.htm)

#### `public PreviewLighting Lighting()`

Lighting

**Returns:** `PreviewLighting` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.Lighting"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_Lighting.htm)

#### `public IRhRdkPreviewSceneServer_eRotationType RotationType()`

RotationType

**Returns:** `IRhRdkPreviewSceneServer_eRotationType` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.RotationType"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_RotationType.htm)

#### `public double RotationX()`

RotationX

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.RotationX"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_RotationX.htm)

#### `public double RotationY()`

RotationY

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.RotationY"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_RotationY.htm)

#### `public void SetRotationType(IRhRdkPreviewSceneServer_eRotationType type)`

SetRotationType

**Parameters:**
- `type` (Rhino.Render.IRhRdkPreviewSceneServer_eRotationType) — [Missing <param name="type"/> documentation for "M:Rhino.Render.PreviewAppearance.SetRotationType(Rhino.Render.IRhRdkPreviewSceneServer_eRotationType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_SetRotationType.htm)

#### `public void SetRotationX(double d)`

SetRotationX

**Parameters:**
- `d` (System.Double) — [Missing <param name="d"/> documentation for "M:Rhino.Render.PreviewAppearance.SetRotationX(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_SetRotationX.htm)

#### `public void SetRotationY(double d)`

SetRotationY

**Parameters:**
- `d` (System.Double) — [Missing <param name="d"/> documentation for "M:Rhino.Render.PreviewAppearance.SetRotationY(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_SetRotationY.htm)

#### `[ObsoleteAttribute("Use Scale instead")] public void SetSize(double d)`

SetSize

**Parameters:**
- `d` (System.Double) — [Missing <param name="d"/> documentation for "M:Rhino.Render.PreviewAppearance.SetSize(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_SetSize.htm)

#### `[ObsoleteAttribute("Use Scale instead")] public double Size()`

Size - used in the UI. Always meters.

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.PreviewAppearance.Size"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_Size.htm)

#### `public void ToMetaData()`

Copy PreviewAppearance to MetaData

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_ToMetaData.htm)

#### `public void ToMetaData(MetaDataProxy mdp)`

Copy PreviewAppearance to MetaData

**Parameters:**
- `mdp` (Rhino.Render.MetaDataProxy) — [Missing <param name="mdp"/> documentation for "M:Rhino.Render.PreviewAppearance.ToMetaData(Rhino.Render.MetaDataProxy)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewAppearance_ToMetaData_1.htm)

### Properties
- `CppPointer` (IntPtr, get) — Previewappearances c++ pointer
- `MetaData` (MetaData, get) — Previewappearance MetaData
- `Scale` (Double, get/set) — Size - used in the UI. Always meters.
- `SceneScale` (Double, get) — Scene size - the actual size that is rendered

## PreviewBackground (class)

PreviewBackGround takes care of constucting and desctrutction of PreviewLight

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PreviewBackground.htm)

### Constructors
- `public PreviewBackground(IntPtr pPreviewBackground)` — Constructor for PreivewLighting

### Methods
#### `public string ElementKind()`

ElementKind

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.PreviewBackground.ElementKind"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewBackground_ElementKind.htm)

#### `public Guid EnvironmentInstanceId()`

EnvironmentInstanceId

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Render.PreviewBackground.EnvironmentInstanceId"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewBackground_EnvironmentInstanceId.htm)

#### `public void SetEnvironmentInstanceId(Guid guid)`

SetEnvironmentInstanceId

**Parameters:**
- `guid` (System.Guid) — [Missing <param name="guid"/> documentation for "M:Rhino.Render.PreviewBackground.SetEnvironmentInstanceId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewBackground_SetEnvironmentInstanceId.htm)

#### `public void SetUpPreview(IntPtr sceneServerPointer, Guid guid)`

SetUpPreview

**Parameters:**
- `sceneServerPointer` (System.IntPtr) — [Missing <param name="sceneServerPointer"/> documentation for "M:Rhino.Render.PreviewBackground.SetUpPreview(System.IntPtr,System.Guid)"]
- `guid` (System.Guid) — [Missing <param name="guid"/> documentation for "M:Rhino.Render.PreviewBackground.SetUpPreview(System.IntPtr,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewBackground_SetUpPreview.htm)

### Properties
- `CppPointer` (IntPtr, get) — CppPointer for PreivewLighting

## PreviewGeometry (class)

PreviewAppearance takes care of constucting and desctrutction of PreivewGeometry

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PreviewGeometry.htm)

### Constructors
- `public PreviewGeometry(IntPtr pPreviewGeometry)` — Constructor for PreviewGeometry

### Methods
#### `public string ElementKind()`

ElementKind

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.PreviewGeometry.ElementKind"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewGeometry_ElementKind.htm)

#### `public void SetUpPreview(IntPtr sceneServerPointer, IntPtr pRenderContent, bool bCopy)`

SetUpPreview

**Parameters:**
- `sceneServerPointer` (System.IntPtr) — [Missing <param name="sceneServerPointer"/> documentation for "M:Rhino.Render.PreviewGeometry.SetUpPreview(System.IntPtr,System.IntPtr,System.Boolean)"]
- `pRenderContent` (System.IntPtr) — [Missing <param name="pRenderContent"/> documentation for "M:Rhino.Render.PreviewGeometry.SetUpPreview(System.IntPtr,System.IntPtr,System.Boolean)"]
- `bCopy` (System.Boolean) — [Missing <param name="bCopy"/> documentation for "M:Rhino.Render.PreviewGeometry.SetUpPreview(System.IntPtr,System.IntPtr,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewGeometry_SetUpPreview.htm)

### Properties
- `CppPointer` (IntPtr, get) — CppPointer for PreviewGeometry

## PreviewLighting (class)

PreviewAppearance takes care of constucting and desctrutction of PreviewLight

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PreviewLighting.htm)

### Constructors
- `public PreviewLighting(IntPtr pPreviewLighting)` — Constructor for PreivewLighting

### Methods
#### `public string ElementKind()`

ElementKind

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.PreviewLighting.ElementKind"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewLighting_ElementKind.htm)

#### `public void SetUpPreview(IntPtr sceneServerPointer)`

SetUpPreview

**Parameters:**
- `sceneServerPointer` (System.IntPtr) — [Missing <param name="sceneServerPointer"/> documentation for "M:Rhino.Render.PreviewLighting.SetUpPreview(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewLighting_SetUpPreview.htm)

### Properties
- `CppPointer` (IntPtr, get) — CppPointer for PreivewLighting

## PreviewSceneQuality (enum)

Quality levels when creating preview images

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PreviewSceneQuality.htm)

### Values
- `RealtimeQuick` = `1` — Obsolete. Very fast preview. Typically using the internal OpenGL preview generator.
- `RefineFirstPass` = `1` — Obsolete. Low quality rendering for quick preview.
- `RefineSecondPass` = `2` — Obsolete. Medium quality rendering for intermediate preview.
- `RefineThirdPass` = `4` — Obsolete. Full quality rendering (quality comes from user settings)
- `None` = `0` — No quality set.
- `Low` = `1` — Low quality rendering for quick preview.
- `Medium` = `2` — Medium quality rendering for intermediate preview.
- `IntermediateProgressive` = `3` — Intermediate update, always considered better quality than the previous IntermediateProgressive, but not as high as Full.
- `Full` = `4` — Full quality rendering (quality comes from user settings).

## PreviewSceneServer (class)

PreviewSceneServer

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_PreviewSceneServer.htm)

### Constructors
- `public PreviewSceneServer(IntPtr pPreviewSceneServer)` — Constructor for PreviewSceneServer

### Methods
#### `public void ApplyRotation(double X, double Y, IRhRdkPreviewSceneServer_eRotationType type)`

Set Scene Server Rotation

**Parameters:**
- `X` (System.Double) — [Missing <param name="X"/> documentation for "M:Rhino.Render.PreviewSceneServer.ApplyRotation(System.Double,System.Double,Rhino.Render.IRhRdkPreviewSceneServer_eRotationType)"]
- `Y` (System.Double) — [Missing <param name="Y"/> documentation for "M:Rhino.Render.PreviewSceneServer.ApplyRotation(System.Double,System.Double,Rhino.Render.IRhRdkPreviewSceneServer_eRotationType)"]
- `type` (Rhino.Render.IRhRdkPreviewSceneServer_eRotationType) — [Missing <param name="type"/> documentation for "M:Rhino.Render.PreviewSceneServer.ApplyRotation(System.Double,System.Double,Rhino.Render.IRhRdkPreviewSceneServer_eRotationType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewSceneServer_ApplyRotation.htm)

#### `public void Dispose()`

Dispose for PreviewSceneServer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewSceneServer_Dispose.htm)

#### `protected override void Finalize()`

Destructor for PreviewSceneServer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewSceneServer_Finalize.htm)

#### `public void SetSceneScale(double scale)`

Set Scene Server Scale

**Parameters:**
- `scale` (System.Double) — [Missing <param name="scale"/> documentation for "M:Rhino.Render.PreviewSceneServer.SetSceneScale(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_PreviewSceneServer_SetSceneScale.htm)

### Properties
- `CppPointer` (IntPtr, get) — The CppPointer of PreviewSceneServer

## ProxyTypes (enum)

Defines the proxy type of the render content

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_ProxyTypes.htm)

### Values
- `None` = `0` — No proxy type specified.
- `Single` = `1` — A proxy that represents a single material in the CRhinoDoc::m_material_table
- `Multi` = `2` — A proxy that represents multiple materials - all similar.
- `Texture` = `3` — A proxy that represents textures, either in the texture table, or attached to materials or environments.

## RdkUndo (class)

RdkUndo class, which is used to get the RdkUndoRecord

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RdkUndo.htm)

### Constructors
- `public RdkUndo(IntPtr pUndoRecord)` — Constructor for RdkUndo

### Methods
#### `public void Dispose()`

Dispose for RdkUndo

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RdkUndo_Dispose.htm)

#### `protected override void Finalize()`

Destructor for RdkUndo

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RdkUndo_Finalize.htm)

#### `public RdkUndoRecord NewUndoRecord()`

Get a new UndoRecord

**Returns:** `RdkUndoRecord` — [Missing <returns> documentation for "M:Rhino.Render.RdkUndo.NewUndoRecord"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RdkUndo_NewUndoRecord.htm)

## RdkUndoRecord (class)

RdkUndoRecord class

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RdkUndoRecord.htm)

### Constructors
- `public RdkUndoRecord(IntPtr pUndoRecord)` — Constructor for RdkUndoRecord

### Methods
#### `public void Dispose()`

Dispose for RdkUndoRecord

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RdkUndoRecord_Dispose.htm)

#### `protected override void Finalize()`

Destructor for RdkUndoRecord

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RdkUndoRecord_Finalize.htm)

#### `public void SetDescription(string description)`

Set description for RdkUndoRecord

**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.Render.RdkUndoRecord.SetDescription(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RdkUndoRecord_SetDescription.htm)

## RealtimeDisplayMode (class)

Base class for implementing real-time display modes in .NET. Pay special attention that in StartRenderer the RenderWindow.SetSize() function is called if the implementation relies on the RenderWindow to do the drawing to the viewport. If i.e. OpenGL is used to draw render results to the viewport then SetSize() doesn't have to be called, nor should the implementation then access channels on the RenderWindow, as those then don't exist. For OpenGL-based drawing the RenderWindow is used as a container for ViewInfo management, nothing else.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RealtimeDisplayMode.htm)

### Constructors
- `protected RealtimeDisplayMode()` — Initializes a new instance of the RealtimeDisplayMode class

### Methods
#### `public virtual double CaptureProgress()`

Override to communicate the progress of a capture.

**Returns:** `Double` — A number between 0.0 and 1.0 inclusive. 1.0 means 100%.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_CaptureProgress.htm)

#### `public uint ComputeViewportCrc(ViewInfo view)`

Compute viewport CRC for the given ViewInfo

**Parameters:**
- `view` (Rhino.DocObjects.ViewInfo) — [Missing <param name="view"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.ComputeViewportCrc(Rhino.DocObjects.ViewInfo)"]

**Returns:** `UInt32` — the CRC for the given view

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_ComputeViewportCrc.htm)

#### `public virtual void CreateWorld(RhinoDoc doc, ViewInfo viewInfo, DisplayPipelineAttributes displayPipelineAttributes)`

Implement if you need to handle the initial CreateWorld call initiated by the display pipeline system. Note that this is not the same as the CreateWorld call in Rhino.Render.ChangeQueue.ChangeQueue, although related.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Rhino document
- `viewInfo` (Rhino.DocObjects.ViewInfo) — active viewport info
- `displayPipelineAttributes` (Rhino.Display.DisplayPipelineAttributes) — display pipeline attributes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_CreateWorld.htm)

#### `public virtual bool DrawOpenGl()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.DrawOpenGl"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_DrawOpenGl.htm)

#### `public static RealtimeDisplayMode GetRealtimeViewport(IntPtr realtimeViewport)`

Retrieve RealtimeDisplayMode instance that the IntPtr refers to.

**Parameters:**
- `realtimeViewport` (System.IntPtr) — IntPtr of the instance searched for. If the instance doesn't exist, a new one is created.

**Returns:** `RealtimeDisplayMode` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.GetRealtimeViewport(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_GetRealtimeViewport.htm)

#### `public static RealtimeDisplayMode GetRealtimeViewport(IntPtr realtimeViewport, bool create)`

Retrieve RealtimeDisplayMode instance. If create is set to true then a new instance is created if not found, null is returned for false.

**Parameters:**
- `realtimeViewport` (System.IntPtr) — IntPtr
- `create` (System.Boolean) — true to create if not found, false to return null if not found.

**Returns:** `RealtimeDisplayMode` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.GetRealtimeViewport(System.IntPtr,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_GetRealtimeViewport_1.htm)

#### `public abstract void GetRenderSize(out int width, out int height)`

Get the current render resolution for the running render session.

**Parameters:**
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.GetRenderSize(System.Int32@,System.Int32@)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.GetRenderSize(System.Int32@,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_GetRenderSize.htm)

#### `public ViewInfo GetView()`

Get ViewInfo that has been registered with this RealtimeDisplayMode instance.

**Returns:** `ViewInfo` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.GetView"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_GetView.htm)

#### `public virtual bool HudAllowEditMaxPasses()`

Override to allow maximum pass editing. By default disabled.

**Returns:** `Boolean` — Return true to allow users to edit the maximum pass count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudAllowEditMaxPasses.htm)

#### `public virtual string HudCustomStatusText()`

Override to display status of the render engine.

**Returns:** `String` — Status text to display

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudCustomStatusText.htm)

#### `public virtual int HudLastRenderedPass()`

Override to communicate the last completed pass. Can be shown in the HUD

**Returns:** `Int32` — Last completed pass

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudLastRenderedPass.htm)

#### `public virtual int HudMaximumPasses()`

Override to communicate the maximum passes count currently in use for the render session. Can be shown in the HUD

**Returns:** `Int32` — Maximum passes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudMaximumPasses.htm)

#### `public virtual string HudProductName()`

Override to return the name of your product. This will be printed in the HUD.

**Returns:** `String` — Name of the product.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudProductName.htm)

#### `public virtual bool HudRendererLocked()`

Implement to support locking in the viewport

**Returns:** `Boolean` — Return true if the render engine is locked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudRendererLocked.htm)

#### `public virtual bool HudRendererPaused()`

Implement to support pausing and resuming in the viewport

**Returns:** `Boolean` — Return true if the render engine is paused.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudRendererPaused.htm)

#### `public virtual bool HudShow()`

Override if you want to hide the HUD. Shown by default

**Returns:** `Boolean` — Return false to hide the HUD.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudShow.htm)

#### `public virtual bool HudShowControls()`

Show control buttons on the realtime display HUD. By default these are shown, override this function and return false if HUD controls aren't needed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.HudShowControls"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudShowControls.htm)

#### `public virtual bool HudShowCustomStatusText()`

Override to show status text in HUD. By default disabled.

**Returns:** `Boolean` — Return true to show status text in HUD

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudShowCustomStatusText.htm)

#### `public virtual bool HudShowMaxPasses()`

Override to show maximum passes in HUD. By default disabled.

**Returns:** `Boolean` — Return true to show maximum passes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudShowMaxPasses.htm)

#### `public virtual bool HudShowPasses()`

Override to show current pass in HUD. By default disabled.

**Returns:** `Boolean` — Return true to show current pass in HUD.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudShowPasses.htm)

#### `public virtual DateTime HudStartTime()`



**Returns:** `DateTime` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.HudStartTime"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_HudStartTime.htm)

#### `public abstract bool IsCompleted()`

Implement to tell if your render engine has completed a frame for drawing into the viewport

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.IsCompleted"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_IsCompleted.htm)

#### `public abstract bool IsFrameBufferAvailable(ViewInfo view)`

Implement to tell the render pipeline that a framebuffer is ready

**Parameters:**
- `view` (Rhino.DocObjects.ViewInfo) — [Missing <param name="view"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.IsFrameBufferAvailable(Rhino.DocObjects.ViewInfo)"]

**Returns:** `Boolean` — Return true when a framebuffer is ready. This is generally the case when StartRenderer as returned successfully.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_IsFrameBufferAvailable.htm)

#### `public abstract bool IsRendererStarted()`

Override to tell the started state of your render engine.

**Returns:** `Boolean` — true if render engine is ready and rendering

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_IsRendererStarted.htm)

#### `public virtual int LastRenderedPass()`

Implement to communicate last completed pass to the underlying system.

**Returns:** `Int32` — the last completed pass

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_LastRenderedPass.htm)

#### `public virtual bool OnRenderSizeChanged(int width, int height)`

Override to restart your render engine

**Parameters:**
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.OnRenderSizeChanged(System.Int32,System.Int32)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.OnRenderSizeChanged(System.Int32,System.Int32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.OnRenderSizeChanged(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_OnRenderSizeChanged.htm)

#### `public int OpenGlVersion()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.OpenGlVersion"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_OpenGlVersion.htm)

#### `public virtual void PostConstruct()`

Override PostConstruct if you need to initialize where the underlying RealtimeDisplayMode is available. The connection is made right after RealtimeDisplayMode has been instantiated, but just before PostConstruct is called. For instance finding out OpenGL information can be done in PostConstruct.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_PostConstruct.htm)

#### `public static RealtimeDisplayModeClassInfo[] RegisterDisplayModes(Assembly assembly, Guid pluginId)`

Find and register classes that derive from RealtimeDisplayMode from the given plug-in. The plug-in is found in the given assembly

**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.RegisterDisplayModes(System.Reflection.Assembly,System.Guid)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.RegisterDisplayModes(System.Reflection.Assembly,System.Guid)"]

**Returns:** `RealtimeDisplayModeClassInfo[]` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.RegisterDisplayModes(System.Reflection.Assembly,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_RegisterDisplayModes_1.htm)

#### `public static RealtimeDisplayModeClassInfo[] RegisterDisplayModes(PlugIn plugin)`

Find and register classes that derive from RealtimeDisplayMode from the given plug-in.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.RegisterDisplayModes(Rhino.PlugIns.PlugIn)"]

**Returns:** `RealtimeDisplayModeClassInfo[]` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.RegisterDisplayModes(Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_RegisterDisplayModes.htm)

#### `public static void RemoveRealtimeViewport(IntPtr realtimeViewport)`

Remove RealtimeDisplayMode instance from internal dictionary.

**Parameters:**
- `realtimeViewport` (System.IntPtr) — IntPtr to the RealtimeDisplayMode instance to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_RemoveRealtimeViewport.htm)

#### `public virtual void SetUseDrawOpenGl(bool use)`

During run-time change whether to use OpenGL drawing of results or not. For instance offline rendering (viewcapture* with different resolution than viewport) could use RenderWindow instead of direct OpenGL drawing.

**Parameters:**
- `use` (System.Boolean) — Set to true if OpenGL drawing is wanted, set to false if RenderWindow method is needed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_SetUseDrawOpenGl.htm)

#### `public void SetView(ViewInfo view)`

Set ViewInfo for this RealtimeDisplayMode instance.

**Parameters:**
- `view` (Rhino.DocObjects.ViewInfo) — The ViewInfo to set for subsequent tests.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_SetView.htm)

#### `public virtual bool ShowCaptureProgress()`

Override if you want to i.e. hide the progress dialog for capture progress.

**Returns:** `Boolean` — Return true to show, false to hide

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_ShowCaptureProgress.htm)

#### `public abstract void ShutdownRenderer()`

Override to shutdown your render engine

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_ShutdownRenderer.htm)

#### `public void SignalRedraw()`

Use to signal the underlying pipeline a redraw is wanted. This can be used for instance when a renderer has completed a pass which should be updated in the associated viewport.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_SignalRedraw.htm)

#### `public abstract bool StartRenderer(int w, int h, RhinoDoc doc, ViewInfo view, ViewportInfo viewportInfo, bool forCapture, RenderWindow renderWindow)`

Override to start your render engine. Note that before using the RenderWindow you *must* call SetSize to properly initialize the underlying DIB.

**Parameters:**
- `w` (System.Int32) — Width of resolution
- `h` (System.Int32) — Height of resolution
- `doc` (Rhino.RhinoDoc) — Rhino document
- `view` (Rhino.DocObjects.ViewInfo) — active view
- `viewportInfo` (Rhino.DocObjects.ViewportInfo) — active viewport info
- `forCapture` (System.Boolean) — true if renderer is started for capture purposes (ViewCaptureTo*), false for regular interactive rendering
- `renderWindow` (Rhino.Render.RenderWindow) — RenderWindow to hold render results in.

**Returns:** `Boolean` — Return true when your render engine started correctly, false when that failed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_StartRenderer.htm)

#### `public static void UnregisterDisplayModes(Assembly assembly, Guid pluginId)`



**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.UnregisterDisplayModes(System.Reflection.Assembly,System.Guid)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.UnregisterDisplayModes(System.Reflection.Assembly,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_UnregisterDisplayModes_1.htm)

#### `public static void UnregisterDisplayModes(PlugIn plugin)`



**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.RealtimeDisplayMode.UnregisterDisplayModes(Rhino.PlugIns.PlugIn)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_UnregisterDisplayModes.htm)

#### `public virtual bool UseFastDraw()`

Implement and return true if you want the display pipeline to not wait for IsFramebufferAvailable during the MiddleGround draw phase. This will also tell the pipeline to draw a complete middleground pass in OpenGL.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RealtimeDisplayMode.UseFastDraw"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RealtimeDisplayMode_UseFastDraw.htm)

### Properties
- `LinearWorkflow` (LinearWorkflow, get) — Returns the LinearWorkflow data for this realitime display mode.
- `Locked` (Boolean, get/set) — 
- `MaxPasses` (Int32, get/set) — 
- `Paused` (Boolean, get/set) — 
- `PostEffectsOn` (Boolean, get/set) — 

### Events
#### `HudLockButtonDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudLockButtonDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudLockButtonDoubleClicked.htm)

#### `HudLockButtonLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudLockButtonLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudLockButtonLeftClicked.htm)

#### `HudLockButtonPressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDLockButtonLeftClicked instead")] public event EventHandler HudLockButtonPressed`

Listen tot his event if you want to handle the lock button control

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudLockButtonPressed.htm)

#### `HudLockButtonRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudLockButtonRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudLockButtonRightClicked.htm)

#### `HudPauseButtonDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPauseButtonDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPauseButtonDoubleClicked.htm)

#### `HudPauseButtonLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPauseButtonLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPauseButtonLeftClicked.htm)

#### `HudPauseButtonPressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDPauseButtonLeftClicked instead")] public event EventHandler HudPauseButtonPressed`

Listen tot his event if you want to handle the pause button control

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPauseButtonPressed.htm)

#### `HudPauseButtonRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPauseButtonRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPauseButtonRightClicked.htm)

#### `HudPlayButtonDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPlayButtonDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPlayButtonDoubleClicked.htm)

#### `HudPlayButtonLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPlayButtonLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPlayButtonLeftClicked.htm)

#### `HudPlayButtonPressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDPlayButtonLeftClicked instead")] public event EventHandler HudPlayButtonPressed`

Listen to this event if you want to handle the play button control.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPlayButtonPressed.htm)

#### `HudPlayButtonRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPlayButtonRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPlayButtonRightClicked.htm)

#### `HudPostEffectsOffButtonDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPostEffectsOffButtonDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPostEffectsOffButtonDoubleClicked.htm)

#### `HudPostEffectsOffButtonLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPostEffectsOffButtonLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPostEffectsOffButtonLeftClicked.htm)

#### `HudPostEffectsOffButtonRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPostEffectsOffButtonRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPostEffectsOffButtonRightClicked.htm)

#### `HudPostEffectsOnButtonDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPostEffectsOnButtonDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPostEffectsOnButtonDoubleClicked.htm)

#### `HudPostEffectsOnButtonLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPostEffectsOnButtonLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPostEffectsOnButtonLeftClicked.htm)

#### `HudPostEffectsOnButtonRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudPostEffectsOnButtonRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudPostEffectsOnButtonRightClicked.htm)

#### `HudProductNameDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudProductNameDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudProductNameDoubleClicked.htm)

#### `HudProductNameLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudProductNameLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudProductNameLeftClicked.htm)

#### `HudProductNamePressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDProductNameLeftClicked instead")] public event EventHandler HudProductNamePressed`

Listen tot his event if you want to handle a press on the product name component

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudProductNamePressed.htm)

#### `HudProductNameRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudProductNameRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudProductNameRightClicked.htm)

#### `HudStatusTextDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudStatusTextDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudStatusTextDoubleClicked.htm)

#### `HudStatusTextLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudStatusTextLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudStatusTextLeftClicked.htm)

#### `HudStatusTextPressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDStatusTextLeftClicked instead")] public event EventHandler HudStatusTextPressed`

Listen tot his event if you want to handle a press on the status text component

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudStatusTextPressed.htm)

#### `HudStatusTextRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudStatusTextRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudStatusTextRightClicked.htm)

#### `HudTimeDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudTimeDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudTimeDoubleClicked.htm)

#### `HudTimeLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudTimeLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudTimeLeftClicked.htm)

#### `HudTimePressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDTimeLeftClicked instead")] public event EventHandler HudTimePressed`

Listen tot his event if you want to handle a press press on the time component

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudTimePressed.htm)

#### `HudTimeRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudTimeRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudTimeRightClicked.htm)

#### `HudUnlockButtonDoubleClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudUnlockButtonDoubleClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudUnlockButtonDoubleClicked.htm)

#### `HudUnlockButtonLeftClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudUnlockButtonLeftClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudUnlockButtonLeftClicked.htm)

#### `HudUnlockButtonPressed` (`System.EventHandler`)

**Signature:** `[ObsoleteAttribute("Use HUDUnlockButtonLeftClicked instead")] public event EventHandler HudUnlockButtonPressed`

Listen tot his event if you want to handle the unlock button control

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudUnlockButtonPressed.htm)

#### `HudUnlockButtonRightClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HudUnlockButtonRightClicked`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_HudUnlockButtonRightClicked.htm)

#### `MaxPassesChanged` (`System.EventHandler<RealtimeDisplayMode.HudMaxPassesChangedEventArgs>`)

**Signature:** `public event EventHandler<RealtimeDisplayMode.HudMaxPassesChangedEventArgs> MaxPassesChanged`

Listen to this if you want to handle changes in maximum pass count through the HUD.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_MaxPassesChanged.htm)

#### `OnDisplayPipelineSettingsChanged` (`System.EventHandler<RealtimeDisplayMode.DisplayPipelineSettingsChangedEventArgs>`)

**Signature:** `public event EventHandler<RealtimeDisplayMode.DisplayPipelineSettingsChangedEventArgs> OnDisplayPipelineSettingsChanged`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_OnDisplayPipelineSettingsChanged.htm)

#### `OnDrawMiddleground` (`System.EventHandler<RealtimeDisplayMode.DrawMiddlegroundEventArgs>`)

**Signature:** `public event EventHandler<RealtimeDisplayMode.DrawMiddlegroundEventArgs> OnDrawMiddleground`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_OnDrawMiddleground.htm)

#### `OnInitFramebuffer` (`System.EventHandler<RealtimeDisplayMode.InitFramebufferEventArgs>`)

**Signature:** `public event EventHandler<RealtimeDisplayMode.InitFramebufferEventArgs> OnInitFramebuffer`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RealtimeDisplayMode_OnInitFramebuffer.htm)

## RealtimeDisplayMode.DisplayPipelineSettingsChangedEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.RealtimeDisplayMode.DisplayPipelineSettingsChangedEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RealtimeDisplayMode_DisplayPipelineSettingsChangedEventArgs.htm)

### Constructors
- `public DisplayPipelineSettingsChangedEventArgs(DisplayPipelineAttributes dpa)` — Initializes a new instance of the RealtimeDisplayMode.DisplayPipelineSettingsChangedEventArgs class

### Properties
- `Attributes` (DisplayPipelineAttributes, get) — 

## RealtimeDisplayMode.DrawMiddlegroundEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.RealtimeDisplayMode.DrawMiddlegroundEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RealtimeDisplayMode_DrawMiddlegroundEventArgs.htm)

### Constructors
- `public DrawMiddlegroundEventArgs(DisplayPipeline dp)` — Initializes a new instance of the RealtimeDisplayMode.DrawMiddlegroundEventArgs class

### Properties
- `Pipeline` (DisplayPipeline, get) — 

## RealtimeDisplayMode.HudMaxPassesChangedEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.RealtimeDisplayMode.HudMaxPassesChangedEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RealtimeDisplayMode_HudMaxPassesChangedEventArgs.htm)

### Constructors
- `public HudMaxPassesChangedEventArgs(int mp)` — Initializes a new instance of the RealtimeDisplayMode.HudMaxPassesChangedEventArgs class

### Properties
- `MaxPasses` (Int32, get) — 

## RealtimeDisplayMode.InitFramebufferEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.RealtimeDisplayMode.InitFramebufferEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RealtimeDisplayMode_InitFramebufferEventArgs.htm)

### Constructors
- `public InitFramebufferEventArgs(DisplayPipeline dp)` — Initializes a new instance of the RealtimeDisplayMode.InitFramebufferEventArgs class

### Properties
- `Pipeline` (DisplayPipeline, get) — 

## RealtimeDisplayModeClassInfo (class)

Class information obligatory for registering RealtimeDisplayMode implementations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RealtimeDisplayModeClassInfo.htm)

### Constructors
- `protected RealtimeDisplayModeClassInfo()` — Initializes a new instance of the RealtimeDisplayModeClassInfo class

### Properties
- `DontRegisterAttributesOnStart` (Boolean, get) — Override and return true when you don't want your class info to cause display attributes to be registered with the system.
- `DrawOpenGl` (Boolean, get) — Return true if the RealtimeDisplayMode draws its result using OpenGL. RenderWindow usage will then be skipped.
- `GUID` (Guid, get) — Get the RealtimeDisplayMode implementation GUID
- `Name` (String, get) — Get human-facing class description for RealtimeDisplayMode implementation. This string might show up in the Rhino UI.
- `RealtimeDisplayModeType` (Type, get) — Get the type being registered.

## RenderChannels (class)

Render Channels. This corresponds to the user's settings in the Rendering panel.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderChannels.htm)

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.RenderChannels.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderChannels_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `protected override void Finalize()`

Handle destruction of the un-managed CPP object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_Finalize.htm)

### Properties
- `CustomList` (Guid[], get/set) — 
- `Mode` (RenderChannels.Modes, get/set) — 

### Events
#### `Changed` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> Changed`

This event is raised when a Render Channels property value is changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderChannels_Changed.htm)

## RenderChannels.Modes (enum)

Mode.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderChannels_Modes.htm)

### Values
- `Automatic` = `0` — Render-channels are managed automatically
- `Custom` = `1` — Render-channels are specified by the user

## RenderContent (class)

Base class for all RenderContent - RenderMaterial, RenderTexture and RenderEnvironment Contents have a unique type id which is the same for all instances of the same class and an instance id which is unique for each instance.They know how to provide a shader for rendering, how to read and write their state as XML and how to create their own user interfaces. There are two flavors of content in the RDK -- temporary and persistent.It is very important to understand the distinction between a temporary content instance and a persistent content instance, and the fact that a temporary instance (and all its children) can become persistent.Persistent content is registered with a document and is usually(but not always) owned by it. Temporary contents get created and deleted very often during the normal operation of the RDK.In fact, just about anything the user clicks on might result in a temporary content being created and deleted again.They are created by the content browser, the thumbnail rendering, and so on.They are 'free floating' and are owned by whomever created them.They do not appear in the modeless UI, they do not get saved in the 3dm file, and they can freely be deleted again after use. Contrast this with persistent contents which are attached to a document.They are always owned by RDK, appear in the modeless UI and get saved in the 3dm file. Pointers to persistent contents should never be stored by clients; you should only store their instance ids and look them up again using RenderContent.FromId. They can be deleted only after detaching them from the document. RenderContent::Create is the highest-level function for creating a content.It creates it, initializes it, adds it to the document and sends many events.It even records undo.You cannot call this method from just anywhere. It must only be called by 'UI code'; scripts or buttons on a dialog.It results in a persistent (usually top-level) content being attached to the document and appearing in all the RDK UI elements that display contents, like the thumbnail and tree views.If you call this method and specify a parent and child-slot name, your new content will be attached to the document-resident parent as a child and the UI will be updated accordingly. The important point is that everything is temporary while the content structure is being built. Only after the whole structure is complete will the top-level parent be attached to the document making the whole structure persistent.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContent.htm)

### Methods
#### `public bool AddAutomaticUserInterfaceSection(string caption, int id)`

Add a new automatic user interface section, Field values which include prompts will be automatically added to this section.

**Parameters:**
- `caption` (System.String) — Expandable tab caption.
- `id` (System.Int32) — Tab id which may be used later on to reference this tab.

**Returns:** `Boolean` — Returns true if the automatic tab section was added otherwise; returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddAutomaticUserInterfaceSection.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent)`

Obsolete.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent, string childSlotName)`

Obsolete.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild_1.htm)

#### `[ObsoleteAttribute("Use RhinoDoc.RenderMaterials.Add")] public static bool AddPersistentRenderContent(RenderContent renderContent)`

Add a material, environment or texture to the internal RDK document lists as top level content. The content must have been returned from RenderContent::MakeCopy, NewContentFromType or a similar function that returns a non-document content. Obsolete - use RhinoDoc.RenderMaterials.Add or similar.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — The render content.

**Returns:** `Boolean` — true on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddPersistentRenderContent.htm)

#### `[ObsoleteAttribute("Use RhinoDoc.RenderMaterials.Add")] public static bool AddPersistentRenderContent(RhinoDoc document, RenderContent renderContent)`

Add a material, environment or texture to the internal RDK document lists as top level content. The content must have been returned from RenderContent::MakeCopy, NewContentFromType or a similar function that returns a non-document content. Obsolete - use RhinoDoc.RenderMaterials.Add or similar.

**Parameters:**
- `document` (Rhino.RhinoDoc) — The document to attach the render content to.
- `renderContent` (Rhino.Render.RenderContent) — The render content.

**Returns:** `Boolean` — true on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddPersistentRenderContent_1.htm)

#### `public bool AddUserInterfaceSection(ICollapsibleSection section)`



**Parameters:**
- `section` (Rhino.UI.Controls.ICollapsibleSection) — [Missing <param name="section"/> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection.htm)

#### `[ObsoleteAttribute("Use AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection) below instead. This function will not work on the Mac and is not type-safe.")] public UserInterfaceSection AddUserInterfaceSection(Type classType, string caption, bool createExpanded, bool createVisible)`

Add a new .NET control to an content expandable tab section, the height of the createExpanded tabs client area will be the initial height of the specified control.

**Parameters:**
- `classType` (System.Type) — The control class to create and embed as a child window in the expandable tab client area. This class type must be derived from IWin32Window or this method will throw an ArgumentException. Implement the IUserInterfaceSection interface in your classType to get UserInterfaceSection notification.
- `caption` (System.String) — Expandable tab caption.
- `createExpanded` (System.Boolean) — If this value is true then the new expandable tab section will initially be expanded, if it is false it will be collapsed.
- `createVisible` (System.Boolean) — If this value is true then the new expandable tab section will initially be visible, if it is false it will be hidden.

**Returns:** `UserInterfaceSection` — Returns the UserInterfaceSection object used to manage the new user control object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection_1.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`

Begins a change or batch of changes. It may also make a copy of the content state allowing EndChange() to send an event with the old and new contents. Calls to this method are counted; you must call EndChange() once for every call to BeginChange(). Note: If Changed() was called between the calls to BeginChange() and EndChange(), the last call to EndChange() may cause the ContentChanged event to be sent.

**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — the change context. If this is ChangeContexts.UI, ChangeContexts.Program,ChangeContexts.Drop or ChangeContexts.Tree, the content will be copied. EndChange() will then send the copy as 'old' in the OnContentChanged event. EndChange()ContentChanged

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginChange.htm)

#### `public void BeginCreateDynamicFields(bool automatic)`

Automatic Dynamic Field support. Dynamic fields are typically created in the constructor of RenderContent and they are therefore created automatically whenever the content is created. However, some advanced users require the fields to be created in response to some user action which occurs much later. This creates the problem that the fields do not exist by default and therefore cannot be loaded when the document is loaded. These methods are provided to solve that problem by making it possible to automatically create the dynamic fields on loading if they don't already exist. Dynamic fields that have this auto-create-on-load behavior are referred to as automatic dynamic fields. Dynamic fields that do not require the advanced automatic feature can still be created by using these methods (recommended), or they can be created manually (legacy usage). You must call this before creating any dynamic fields. Calls to this method are counted; you must call EndCreateDynamicFields() once for every call to BeginCreateDynamicFields().

**Parameters:**
- `automatic` (System.Boolean) — automatic specifies if the dynamic fields are automatic. If so, they will be created automatically during loading of the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginCreateDynamicFields.htm)

#### `public void BindParameterToField(string parameterName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField.htm)

#### `public void BindParameterToField(string parameterName, string childSlotName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField_1.htm)

#### `protected virtual uint CalculateRenderHash(ulong rcrcFlags)`

Override this method to calculate the render hash of the state that affects how the content is rendered. Does not include children or perform any caching. Render hash values are now automatically cached by the content framework and you do not have to worry about caching. You also do not have to worry about iterating into children. This method is now only called internally by the framework, use the RenderHash property to get the current hash value.

**Parameters:**
- `rcrcFlags` (System.UInt64) — [Missing <param name="rcrcFlags"/> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CalculateRenderHash.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool ChangeChild(RenderContent oldContent, RenderContent newContent)`

Obsolete.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — [Missing <param name="oldContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]
- `newContent` (Rhino.Render.RenderContent) — [Missing <param name="newContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChangeChild.htm)

#### `public double ChildSlotAmount(string childSlotName)`

Gets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Double` — The requested amount value. Values are typically from 0.0 to 100.0

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotAmount.htm)

#### `public string ChildSlotNameFromParamName(string paramName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `paramName` (System.String) — The name of a parameter field. Since child textures will usually correspond with some parameter (they generally either replace or modify a parameter over UV space) these functions are used to specify which parameter corresponded with child slot. If there is no correspondence, return the empty string.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotNameFromParamName.htm)

#### `public bool ChildSlotOn(string childSlotName)`

Gets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotOn.htm)

#### `public virtual void ConvertUnits(UnitSystem from, UnitSystem to)`

Modify your content so that it is converted from meters into the units of the unit system. No need to call the base class when you override this, and no need to recurse into children.

**Parameters:**
- `from` (Rhino.UnitSystem) — [Missing <param name="from"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]
- `to` (Rhino.UnitSystem) — [Missing <param name="to"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ConvertUnits.htm)

#### `public static RenderContent Create(Guid type, RenderContent parent, string childSlotName, RenderContent.ShowContentChooserFlags flags, RhinoDoc doc)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `type` (System.Guid) — The type of the content to create.
- `parent` (Rhino.Render.RenderContent) — Parent is the parent content. If not null, this must be an RDK-owned content that is attached to the document (either top-level or child). The new content then becomes its child. If null, the new content is added to the top-level document content list instead.
- `childSlotName` (System.String) — ChildSlotName is the unique child identifier to use for the new content when creating it as a child of parent (i.e., when parent is not null)
- `flags` (Rhino.Render.RenderContent.ShowContentChooserFlags) — Flags for future use (please always pass ShowContentChooserFlags::None).
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_4.htm)

#### `public static RenderContent Create(Guid type, RenderContent.ShowContentChooserFlags flags, RhinoDoc doc)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `type` (System.Guid) — The type of the content to create.
- `flags` (Rhino.Render.RenderContent.ShowContentChooserFlags) — Flags for future use (please always pass ShowContentChooserFlags::None).
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_5.htm)

#### `public static RenderContent Create(RhinoDoc doc, Guid type)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.
- `type` (System.Guid) — The type of the content to create.

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create.htm)

#### `public static RenderContent Create(RhinoDoc doc, Guid type, RenderContent parent, string childSlotName)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.
- `type` (System.Guid) — The type of the content to create.
- `parent` (Rhino.Render.RenderContent) — Parent is the parent content. If not null, this must be an RDK-owned content that is attached to the document (either top-level or child). The new content then becomes its child. If null, the new content is added to the top-level document content list instead.
- `childSlotName` (System.String) — ChildSlotName is the unique child identifier to use for the new content when creating it as a child of parent (i.e., when parent is not null)

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_1.htm)

#### `public static RenderContent Create(RhinoDoc doc, Type type)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.
- `type` (System.Type) — The type of the content to create.

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_2.htm)

#### `public static RenderContent Create(RhinoDoc doc, Type type, RenderContent parent, string childSlotName)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.
- `type` (System.Type) — The type of the content to create.
- `parent` (Rhino.Render.RenderContent) — The parent content. If not null, this must be an RDK-owned content that is attached to the document (either top-level or child). The new content then becomes its child. If null, the new content is added to the top-level document content list instead.
- `childSlotName` (System.String) — ChildSlotName is the unique child identifier to use for the new content when creating it as a child of parent (i.e., when parent is not null)

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_3.htm)

#### `public static RenderContent Create(Type type, RenderContent parent, string childSlotName, RenderContent.ShowContentChooserFlags flags, RhinoDoc doc)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `type` (System.Type) — The type of the content to create.
- `parent` (Rhino.Render.RenderContent) — The parent content. If not null, this must be an RDK-owned content that is attached to the document (either top-level or child). The new content then becomes its child. If null, the new content is added to the top-level document content list instead.
- `childSlotName` (System.String) — ChildSlotName is the unique child identifier to use for the new content when creating it as a child of parent (i.e., when parent is not null)
- `flags` (Rhino.Render.RenderContent.ShowContentChooserFlags) — Flags for future use (please always pass ShowContentChooserFlags::None).
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_6.htm)

#### `public static RenderContent Create(Type type, RenderContent.ShowContentChooserFlags flags, RhinoDoc doc)`

Constructs a new content of the specified type and attaches it to a document. This function cannot be used to create temporary content that you delete after use. Content created by this function is owned by RDK and appears in the content editor. To create a temporary content which is owned by you, call RenderContentType.NewContentFromTypeId().

**Parameters:**
- `type` (System.Type) — The type of the content to create.
- `flags` (Rhino.Render.RenderContent.ShowContentChooserFlags) — Flags for future use (please always pass ShowContentChooserFlags::None).
- `doc` (Rhino.RhinoDoc) — The Rhino document to attach the new render content to.

**Returns:** `RenderContent` — A new document-resident render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Create_7.htm)

#### `public bool CreateDynamicField(string internalName, string localName, string englishName, Object value, Object minValue, Object maxValue, int sectionId)`

Create a dynamic field with an initial value and min and max limits.

**Parameters:**
- `internalName` (System.String) — is the internal name of the field. Not localized.
- `localName` (System.String) — is the localized user-friendly name of the field.
- `englishName` (System.String) — is the English user-friendly name of the field.
- `value` (System.Object) — is the initial value of the field.
- `minValue` (System.Object) — is the minimum value of the field. Must be the same type as vValue.
- `maxValue` (System.Object) — is the maximum value of the field. Must be the same type as vValue.
- `sectionId` (System.Int32) — is used for filtering fields between sections. Zero if not needed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CreateDynamicField(System.String,System.String,System.String,System.Object,System.Object,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CreateDynamicField.htm)

#### `public void DeleteAllChildren(RenderContent.ChangeContexts changeContexts)`



**Parameters:**
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteAllChildren(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteAllChildren.htm)

#### `public bool DeleteChild(string childSlotName, RenderContent.ChangeContexts changeContexts)`



**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteChild.htm)

#### `public void Dispose()`

Releases all resources used by the RenderContent

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Dispose

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose_1.htm)

#### `public virtual bool DynamicIcon(Size size, out Bitmap bitmap, DynamicIconUsage usage)`



**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `usage` (Rhino.Render.DynamicIconUsage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DynamicIcon.htm)

#### `public RenderContent Edit()`

This method allows a render content hierarchy to be edited using a modal (AKA 'pop-up') editor. If the original render content is in a document, it will remain there, and the edited one will be 'free-floating'. Therefore it is the caller's responsibility to do any replacement in the document if required. The returned new content will be owned by the caller.

**Returns:** `RenderContent` — Returns an edited version of the content hierarchy if successful, else null. The method always edits the entire hierarchy. It places a copy of the hierarchy in the editor and selects the copied item that corresponds to this one. Therefore, editing a child will return a top-level render content, not a child.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Edit.htm)

#### `public void EndChange()`

Ends a change or batch of changes. Calls to this method are counted; you must call this method once for every call to BeginChange(RenderContent.ChangeContexts). Note: If BeginChange(RenderContent.ChangeContexts) was called with ChangeContexts.UI, ChangeContexts.Program, ChangeContexts.Drop or ChangeContexts.UI.Tree and Changed() was called between the calls to BeginChange(RenderContent.ChangeContexts) and EndChange(), the last call to EndChange() will raise the ContentChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndChange.htm)

#### `public void EndCreateDynamicFields()`

You must call this after creating dynamic fields. Calls to this method are counted; you must call BeginCreateDynamicFields() once for every call to EndCreateDynamicFields().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndCreateDynamicFields.htm)

#### `public ContentFactory Factory()`



**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Factory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Factory.htm)

#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Finalize.htm)

#### `public RenderContent FindChild(string childSlotName)`



**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FindChild.htm)

#### `public RenderContent ForDisplay()`

The only place a single proxy can be displayed is in the New Content Control main thumbnail. All other attempts to use a single proxy in a UI require it to be replaced with the corresponding multi proxy. Single proxies override this to find the corresponding multi proxy.

**Returns:** `RenderContent` — The cotnent.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ForDisplay.htm)

#### `public static RenderContent FromId(RhinoDoc document, Guid id)`

Search for a content object based on its Id

**Parameters:**
- `document` (Rhino.RhinoDoc) — The Rhino document containing the content.
- `id` (System.Guid) — Id of the content instance to search for.

**Returns:** `RenderContent` — Returns the content object with the specified Id if it is found otherwise it returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FromId.htm)

#### `[ObsoleteAttribute] public static RenderContent FromXml(string xml)`

Obsolete.

**Parameters:**
- `xml` (System.String) — [Missing <param name="xml"/> documentation for "M:Rhino.Render.RenderContent.FromXml(System.String)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FromXml(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FromXml.htm)

#### `public static RenderContent FromXml(string xml, RhinoDoc doc)`

Creates a new content from the XML data. The resulting content will not be attached to the document.

**Parameters:**
- `xml` (System.String) — The input XML data.
- `doc` (Rhino.RhinoDoc) — The document that the content will be associated with for units, linear workflow purposes.

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FromXml(System.String,Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FromXml_1.htm)

#### `public virtual Object GetChildSlotParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to specify additional functionality for automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call GetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetChildSlotParameter.htm)

#### `public string[] GetEmbeddedFilesList()`



**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetEmbeddedFilesList"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetEmbeddedFilesList.htm)

#### `public Object GetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetExtraRequirementParameter.htm)

#### `public virtual Object GetParameter(string parameterName)`

Query the content instance for the value of a given named parameter. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — Name of the parameter

**Returns:** `Object` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetParameter.htm)

#### `public virtual ulong GetUiHash()`

This allows a content to have more than one UI for the same content type.

**Returns:** `UInt64` — Default is zero and is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUiHash.htm)

#### `public bool GetUnderlyingInstances(ref RenderContentCollection collection)`



**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUnderlyingInstances.htm)

#### `public virtual bool Icon(Size size, out Bitmap bitmap)`



**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Icon.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public bool Initialize()`

Obsolete.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Initialize.htm)

#### `public virtual bool IsCompatible(Guid renderEngineId)`



**Parameters:**
- `renderEngineId` (System.Guid) — [Missing <param name="renderEngineId"/> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsCompatible.htm)

#### `public virtual bool IsContentTypeAcceptableAsChild(Guid type, string childSlotName)`



**Parameters:**
- `type` (System.Guid) — [Missing <param name="type"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsContentTypeAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(ContentFactory factory, string childSlotName)`



**Parameters:**
- `factory` (Rhino.Render.DataSources.ContentFactory) — [Missing <param name="factory"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(Guid kindId, string factoryKind, string childSlotName)`

Override this method to restrict the type of acceptable child content. The default implementation of this method just returns true.

**Parameters:**
- `kindId` (System.Guid) — [Missing <param name="kindId"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `factoryKind` (System.String) — [Missing <param name="factoryKind"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]

**Returns:** `Boolean` — Return true only if content with the specified kindId can be accepted as a child in the specified child slot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild_1.htm)

#### `public bool IsReference()`

Query whether or not the content or any of its ancestors is a reference content.

**Returns:** `Boolean` — true if the content is a reference, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsReference.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public bool IsRenderHashCached()`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Returns:** `Boolean` — bool

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsRenderHashCached.htm)

#### `public static RenderContent LoadFromFile(string filename)`

Loads content from a library file. Does not add the content to the document. Use RhinoDoc.RenderMaterials.Add or similar.

**Parameters:**
- `filename` (System.String) — full path to the file to be loaded.

**Returns:** `RenderContent` — The loaded content or null if an error occurred.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_LoadFromFile.htm)

#### `public RenderContent MakeCopy()`

Create a copy of the render content. All content is the same, except for the instance Id.

**Returns:** `RenderContent` — The new RenderContent

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeCopy.htm)

#### `public RenderContent MakeGroupInstance()`

Create an 'instance' of the content hierarchy and group the new hierarchy with this hierarchy. If the instance is subsequently attached to the same document, the state of all members of the group will be kept synchronized. With the exception of the instance ids, all state is exactly preserved in the new instance hierarchy. \note The grouping will have no effect until the new instance is attached to the same document.

**Returns:** `RenderContent` — A grouped instance of the content hierarchy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeGroupInstance.htm)

#### `public virtual RenderContent.MatchDataResult MatchData(RenderContent oldContent)`

Implement to transfer data from another content to this content during creation.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — An old content object from which the implementation may harvest data.

**Returns:** `RenderContent.MatchDataResult` — Information about how much data was matched.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MatchData.htm)

#### `protected void ModifyRenderContentStyles(RenderContentStyles stylesToAdd, RenderContentStyles stylesToRemove)`

ModifyRenderContentStyles

**Parameters:**
- `stylesToAdd` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToAdd"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]
- `stylesToRemove` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToRemove"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ModifyRenderContentStyles.htm)

#### `public PreviewSceneServer NewPreviewSceneServer(SceneServerData ssd)`

Gets the PreviewSceneServer of the content

**Parameters:**
- `ssd` (Rhino.Render.SceneServerData) — SceneServerData

**Returns:** `PreviewSceneServer` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.NewPreviewSceneServer(Rhino.Render.SceneServerData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_NewPreviewSceneServer.htm)

#### `protected virtual void OnAddUserInterfaceSections()`

Override to provide UI sections to display in the editor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnAddUserInterfaceSections.htm)

#### `protected virtual bool OnGetDefaultsInteractive()`

Override this method to prompt user for information necessary to create a new content object. For example, if you are created a textured material you may prompt the user for a bitmap file name prior to creating the textured material.

**Returns:** `Boolean` — If true is returned the content is created otherwise the creation is aborted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnGetDefaultsInteractive.htm)

#### `protected virtual void OnMakeCopy(ref RenderContent newContent)`

Override this function to supplement the standard copying behavour for your RenderContent.

**Parameters:**
- `newContent` (Rhino.Render.RenderContent) — Is the content that will be returned from MakeCopy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnMakeCopy.htm)

#### `public bool OpenInEditor()`

Call this method to open the content in the relevant thumbnail editor and select it for editing by the user. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInEditor.htm)

#### `[ObsoleteAttribute("Obsolete, use Edit a version that returns a RenderContent", false)] public bool OpenInModalEditor()`

Call this method to open the content in the a modal version of the editor. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInModalEditor.htm)

#### `public string ParamNameFromChildSlotName(string childSlotName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `childSlotName` (System.String) — The named of the child slot to receive the parameter name for.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ParamNameFromChildSlotName.htm)

#### `public static Type[] RegisterContent(Assembly assembly, Guid pluginId)`

Call RegisterContent in your plug-in's OnLoad function in order to register all of the custom RenderContent classes in your assembly.

**Parameters:**
- `assembly` (System.Reflection.Assembly) — Assembly where custom content is defined, this may be a plug-in assembly or another assembly referenced by the plug-in.
- `pluginId` (System.Guid) — Parent plug-in for this assembly.

**Returns:** `Type[]` — array of render content types registered on success. null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RegisterContent_1.htm)

#### `public static Type[] RegisterContent(PlugIn plugin)`

Call RegisterContent in your plug-in's OnLoad function in order to register all of the custom RenderContent classes in your assembly.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.RenderContent.RegisterContent(Rhino.PlugIns.PlugIn)"]

**Returns:** `Type[]` — array of render content types registered on success. null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RegisterContent.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames, LinearWorkflow lw)`

As RenderHash, but ignore parameter names given. Use this version of the function to calculate a render hash when you have the linear workflow information and you are not running on the main thread. Access to LinearWorkflow data requires document access. CrcRenderHashFlags.ExcludeLinearWorkflow must be specified.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string
- `lw` (Rhino.Render.LinearWorkflow) — Linear Workflow to use for CRC

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_1.htm)

#### `public uint RenderHashExclude(TextureRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.TextureRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_2.htm)

#### `public bool Replace(RenderContent newcontent)`



**Parameters:**
- `newcontent` (Rhino.Render.RenderContent) — [Missing <param name="newcontent"/> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Replace.htm)

#### `public bool SetChild(RenderContent renderContent, string childSlotName)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild.htm)

#### `[ObsoleteAttribute("Use SetChild without ChangeContexts and Begin/EndChange")] public bool SetChild(RenderContent renderContent, string childSlotName, RenderContent.ChangeContexts changeContexts)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.SetChild(Rhino.Render.RenderContent,System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild_1.htm)

#### `public void SetChildSlotAmount(string childSlotName, double amount, RenderContent.ChangeContexts cc)`

Sets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `amount` (System.Double) — The texture amount. Values are typically from 0.0 to 100.0
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — The context of the change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotAmount.htm)

#### `public void SetChildSlotOn(string childSlotName, bool bOn, RenderContent.ChangeContexts cc)`

Sets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `bOn` (System.Boolean) — Value for the on-ness property.
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Context of the change

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotOn.htm)

#### `public virtual bool SetChildSlotParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to support values being set from automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call SetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotParameter.htm)

#### `public bool SetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetExtraRequirementParameter.htm)

#### `public void SetIsRenderHashRecursive(bool recursive)`

By default, RenderHash() recurses into children when computing the render CRC. However, some applications may require children to be excluded from the render CRC calculation. Call this method to enable or disable recursing into children. see RenderHash

**Parameters:**
- `recursive` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetIsRenderHashRecursive.htm)

#### `public void SetName(string name, bool renameEvents, bool ensureNameUnique)`

Set instance name for this content

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `renameEvents` (System.Boolean) — [Missing <param name="renameEvents"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `ensureNameUnique` (System.Boolean) — [Missing <param name="ensureNameUnique"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetName.htm)

#### `public virtual bool SetParameter(string parameterName, Object value)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter.htm)

#### `[ObsoleteAttribute("Use SetParameter without ChangeContexts and Begin/EndChange")] public virtual bool SetParameter(string parameterName, Object value, RenderContent.ChangeContexts changeContext)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter_1.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public void SetRenderHash(uint hash)`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Parameters:**
- `hash` (System.UInt32)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetRenderHash.htm)

#### `public bool SmartUngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. If any content in the same document is left alone in the group, that content is also ungrouped. Records undo and sends events OnContentChanged and OnContentGroupIdChanged. \note This method is designed to be called from a content UI and is intended for RDK internal use but may be used as an expert user override.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SmartUngroupRecursive.htm)

#### `public bool Ungroup()`

Remove this content from any instance group it may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if content was ungrouped, \e false if it was not part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Ungroup.htm)

#### `public bool UngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UngroupRecursive.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public void Uninitialize()`

Obsolete.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Uninitialize.htm)

#### `public int UseCount()`

UseCount returns how many times the content is used

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.UseCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UseCount.htm)

#### `public virtual bool VirtualIcon(Size size, out Bitmap bitmap)`

Icon to display in the content browser, this bitmap needs to be valid for the life of this content object, the content object that returns the bitmap is responsible for disposing of the bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Requested icon size
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.VirtualIcon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — Return Icon to display in the content browser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_VirtualIcon.htm)

### Properties
- `CanBeEdited` (Boolean, get) — Determines if the content can be edited.
- `Category` (String, get) — Category for this content.
- `ChildSlotDisplayName` (String, get) — Returns the localized display name of the child slot name
- `ChildSlotName` (String, get/set) — 
- `CppPointer` (IntPtr, get) — 
- `DisplayName` (String, get) — Display name for this content.
- `Document` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `DocumentAssoc` (RhinoDoc, get/set) — If this render content is associated with a document in any way, the document will be returned. This includes copies of render contents that were attached to a document when the copy was made. Otherwise returns null.
- `DocumentOwner` (RhinoDoc, get) — If this render content is owned by a document, the document will be returned. This is the same as getting the document the render content is attached to. Otherwise returns null.
- `DocumentRegistered` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `Fields` (FieldDictionary, get) — Rhino.Render.Fields FieldDictionary which provides access to setting and retrieving field values.
- `Filename` (String, get/set) — If the content is file based, this function can be overridden to deal with setting/getting the filename. Corresponds to IRhRdkFileBasedContent in the C++ SDK
- `FilesToEmbed` (IEnumerable<String>, get) — A string array of full paths to files used by the content that may be embedded in .3dm files and library files (.rmtl, .renv, .rtex). The default implementation returns an empty string list. Override this to return the file name or file names used by your content. This is typically used by textures that reference files containing the texture imagery.
- `FirstChild` (RenderContent, get) — Return First child of this content or nullptr if none.
- `GroupId` (Guid, get/set) — Group ID of the content
- `Hidden` (Boolean, get/set) — Gets or sets the render content's 'hidden' state. This feature only works for top-level render contents because it hides the entire hierarchy. It is normally used for 'implementation detail' render contents. For expert use only. Hidden render contents are never shown in the UI, with the exception of the Object (or Layer) Material Properties UI which always shows whatever is assigned to the object (or layer). In the Object (or Layer) Material Properties UI, if the user drops down the list, hidden render contents are not listed. Hidden render contents, being part of the document content list, will be listed by any scripts or other code that iterates over the document render content list. It is recommended that you set IsHidden once when you create your render content and leave it on to prevent flicker or slow performance.
- `Id` (Guid, get/set) — Instance identifier for this content.
- `IsDefaultInstance` (Boolean, get) — Checks if render content is default instance.
- `IsHiddenByAutoDelete` (Boolean, get) — Contents can be created as 'auto-delete' by certain commands such as 'Picture'. These contents are automatically hidden from the user when the associated Rhino object is deleted. They are later deleted when the document is saved.
- `IsLocked` (Boolean, get/set) — Set this property to true prior to adding content to the document to lock the content browser editing UI methods. Setting this to true will keep the browser from allowing things like deleting, renaming or changing content. This is useful for custom child content that you want to be editable but persistent. Setting this after adding content to the document will cause an exception to be thrown.
- `Name` (String, get/set) — Instance 'raw' name for this content.
- `NextSibling` (RenderContent, get) — Return First sibling of this content or nullptr if none.
- `Notes` (String, get/set) — Notes for this content.
- `Parent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `ProxyType` (ProxyTypes, get) — Gets the proxy type of the render content
- `RenderHash` (UInt32, get) — Render hash for the content hierarchy. It iterates children and includes a caching mechanism which means the hash value can be retrieved quickly if it hasn't changed. The cache is invalidated when Changed() is called. You can override the CalculateRenderHash(UInt64) method to provide a custom hash value.
- `Styles` (RenderContentStyles, get) — 
- `Tags` (String, get/set) — Tags for this content.
- `TopLevel` (Boolean, get) — Returns true if this content has no parent, false if it is the child of another content.
- `TopLevelParent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `TypeDescription` (String, get) — Description for your content type. ie. "Procedural checker pattern"
- `TypeId` (Guid, get) — Type identifier for this content
- `TypeName` (String, get) — Name for your content type. ie. "My .net Texture"
- `Xml` (String, get) — 

### Events
#### `ContentAdded` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentAdded`

Used to monitor render content addition to the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentAdded.htm)

#### `ContentChanged` (`System.EventHandler<RenderContentChangedEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentChangedEventArgs> ContentChanged`

Used to monitor render content modifications.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentChanged.htm)

#### `ContentDeleted` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentDeleted`

Used to monitor render content deletion from the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentDeleted.htm)

#### `ContentDeleting` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentDeleting`

Used to monitor render content deletion from the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentDeleting.htm)

#### `ContentFieldChanged` (`System.EventHandler<RenderContentFieldChangedEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentFieldChangedEventArgs> ContentFieldChanged`

This event is raised when a field value is modified.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentFieldChanged.htm)

#### `ContentRenamed` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentRenamed`

Used to monitor render content renaming in the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentRenamed.htm)

#### `ContentReplaced` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentReplaced`

Used to monitor render content replacing in the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentReplaced.htm)

#### `ContentReplacing` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentReplacing`

Used to monitor render content replacing in the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentReplacing.htm)

#### `ContentUpdatePreview` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> ContentUpdatePreview`

Used to monitor render content preview updates.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_ContentUpdatePreview.htm)

#### `CurrentEnvironmentChanged` (`System.EventHandler<RenderContentEventArgs>`)

**Signature:** `public static event EventHandler<RenderContentEventArgs> CurrentEnvironmentChanged`

Event fired when changes to current environments have been made. This will be one of Background, ReflectionAndRefraction or Skylighting Since 6.11

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderContent_CurrentEnvironmentChanged.htm)

## RenderContent.ChangeContexts (enum)

Context of a change to content parameters.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContent_ChangeContexts.htm)

### Values
- `UI` = `0` — Change occurred as a result of user activity in the content's UI.
- `Drop` = `1` — Change occurred as a result of drag and drop.
- `Program` = `2` — Change occurred as a result of internal program activity.
- `Ignore` = `3` — Change can be disregarded.
- `Tree` = `4` — Change occurred within the content tree (e.g., nodes reordered).
- `Undo` = `5` — Change occurred as a result of an undo.
- `FieldInit` = `6` — OBSOLETE.
- `Serialize` = `7` — Change occurred during serialization (loading).
- `RealTimeUI` = `8` — Change occurred as a result of 'real-time' user activity in the (content) UI. The content's preview, UI, group members and registerable properties are not updated.
- `Script` = `9` — Change occurred as a result of executing a script.

## RenderContent.ExtraRequirementsSetContexts (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderContent.ExtraRequirementsSetContexts"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContent_ExtraRequirementsSetContexts.htm)

### Values
- `UI` = `0` — Setting extra requirement as a result of user activity.
- `Drop` = `1` — Setting extra requirement as a result of drag and drop.
- `Program` = `2` — Setting extra requirement as a result of other (non-user) program activity.

## RenderContent.MatchDataResult (enum)

Return values for MatchData function

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContent_MatchDataResult.htm)

### Values
- `None` = `0` — None
- `Some` = `1` — Some
- `All` = `2` — All

## RenderContent.ShowContentChooserFlags (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderContent.ShowContentChooserFlags"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContent_ShowContentChooserFlags.htm)

### Values
- `None` = `0` — Deprecated
- `HideNewTab` = `1` — Deprecated
- `HideExistingTab` = `2` — Deprecated
- `MultipleSelection` = `4` — Deprecated

## RenderContentChangeReason (enum)

Enumeration denoting type of change for attach or detach

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentChangeReason.htm)

### Values
- `None` = `0` — No attach or detach change
- `Attach` = `1` — Content is being attached by the RhinoDoc.AttachContent() or RenderContent.AttachChild() methods.
- `Detach` = `2` — Content is being detached by the RenderContent.DeleteContent() method.
- `ChangeAttach` = `3` — Content is being attached while changing.
- `ChangeDetach` = `4` — Content is being detached while changing.
- `AttachUndo` = `5` — Content is being attached during undo/redo
- `DetachUndo` = `6` — Content is being detached during undo/redo.
- `Open` = `7` — Content is being attached during open document
- `Delete` = `8` — Content is being detached during normal deletion.

## RenderContentChangedEventArgs (class)

EventArgs for the RenderContentChanged event

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentChangedEventArgs.htm)

### Properties
- `ChangeContext` (RenderContent.ChangeContexts, get) — 
- `Content` (RenderContent, get) — (Inherited from RenderContentEventArgs.)
- `Document` (RhinoDoc, get) — (Inherited from RenderContentEventArgs.)
- `EnvironmentUsage` (RenderEnvironment.Usage, get) — Meaningful for CurrentEnvironmentChanged event. Will be one of Background, ReflectionAndRefraction or Skylighting. Since 6.11
- `OldContent` (RenderContent, get) — 
- `Reason` (RenderContentChangeReason, get) — Not when used in CurrentEnvironmentChanged (defaults to None).

## RenderContentCollection (class)

A collection of Render content

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentCollection.htm)

### Constructors
- `public RenderContentCollection()` — Construct an empty collection of RenderContent objects
- `public RenderContentCollection(IntPtr nativePtr)` — Internal function to create collection from native pointer

### Methods
#### `public void Add(RenderContentCollection collection)`

Add an array of non-const contents to the collection.

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — The array of contents to add

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Add.htm)

#### `public void Append(RenderContent content)`

Append a RenderContent to the collection

**Parameters:**
- `content` (Rhino.Render.RenderContent) — The array of contents to append.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Append.htm)

#### `public void Clear()`

Clear the collection.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Clear.htm)

#### `public RenderContent ContentAt(int index)`

Content at index

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Render.RenderContentCollection.ContentAt(System.Int32)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentCollection.ContentAt(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_ContentAt.htm)

#### `public bool ContentNeedsPreviewThumbnail(RenderContent c, bool includeChildren)`

To be documented

**Parameters:**
- `c` (Rhino.Render.RenderContent) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderContentCollection.ContentNeedsPreviewThumbnail(Rhino.Render.RenderContent,System.Boolean)"]
- `includeChildren` (System.Boolean) — [Missing <param name="includeChildren"/> documentation for "M:Rhino.Render.RenderContentCollection.ContentNeedsPreviewThumbnail(Rhino.Render.RenderContent,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentCollection.ContentNeedsPreviewThumbnail(Rhino.Render.RenderContent,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_ContentNeedsPreviewThumbnail.htm)

#### `public int Count()`

The number of items in the collection.

**Returns:** `Int32` — The number of items in the collection.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Count.htm)

#### `public void Dispose()`

Dispose function for RenderContentCollection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Dispose.htm)

#### `protected override void Finalize()`

Finalizer for RenderContentCollection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Finalize.htm)

#### `public RenderContent Find_Sel(Guid uuid)`

TODO

**Parameters:**
- `uuid` (System.Guid) — [Missing <param name="uuid"/> documentation for "M:Rhino.Render.RenderContentCollection.Find_Sel(System.Guid)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentCollection.Find_Sel(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Find_Sel.htm)

#### `public string FirstTag()`

Gets the first tag

**Returns:** `String` — The first tag

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_FirstTag.htm)

#### `public IEnumerator GetEnumerator()`



**Returns:** `IEnumerator` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentCollection.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_GetEnumerator.htm)

#### `public FilterContentByUsage GetFilterContentByUsage()`

Gets usage filter type for collection

**Returns:** `FilterContentByUsage` — Usage filter type for collection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_GetFilterContentByUsage.htm)

#### `public bool GetForcedVaries()`

See SetForcedVaries

**Returns:** `Boolean` — Forced varies

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_GetForcedVaries.htm)

#### `public string GetSearchPattern()`

See SetSearchPattern

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentCollection.GetSearchPattern"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_GetSearchPattern.htm)

#### `public ContentCollectionIterator Iterator()`

Gets an iterator for the collection

**Returns:** `ContentCollectionIterator` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentCollection.Iterator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Iterator.htm)

#### `public string NextTag()`

Call FirstTag first - this gets the next tag

**Returns:** `String` — The next tag

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_NextTag.htm)

#### `public void Remove(RenderContentCollection collection)`

Remove an array of contents from the collection.

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — Collection of contents to remove

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Remove.htm)

#### `public void Set(RenderContentCollection collection)`

Set an array of const contents as the collection.

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — The array of contents to set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_Set.htm)

#### `public void SetForcedVaries(bool b)`

Set the collection to 'varies'. Only valid if the collection is a 'selection' collection. Useful for clients that only support single content selections.

**Parameters:**
- `b` (System.Boolean) — Varies if true

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_SetForcedVaries.htm)

#### `public void SetSearchPattern(string pattern)`

Sets a search pattern for filtering contents. This is not actually used by the iterator, but is stored for use by any UI that wants to filter contents based on a search string by using the function RhRdkCheckSearchPattern()

**Parameters:**
- `pattern` (System.String) — The search pattern. See RhRdkCheckSearchPattern() for details

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentCollection_SetSearchPattern.htm)

### Properties
- `CppPointer` (IntPtr, get) — Internal function to get native pointer

## RenderContentEventArgs (class)

Event args for RenderContent

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentEventArgs.htm)

### Properties
- `Content` (RenderContent, get) — 
- `Document` (RhinoDoc, get) — 
- `EnvironmentUsage` (RenderEnvironment.Usage, get) — Meaningful for CurrentEnvironmentChanged event. Will be one of Background, ReflectionAndRefraction or Skylighting. Since 6.11
- `Reason` (RenderContentChangeReason, get) — Not when used in CurrentEnvironmentChanged (defaults to None).

## RenderContentFieldChangedEventArgs (class)

EventArgs for the RenderContentFieldChanged event

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentFieldChangedEventArgs.htm)

### Properties
- `ChangeContext` (RenderContent.ChangeContexts, get) — (Inherited from RenderContentChangedEventArgs.)
- `Content` (RenderContent, get) — (Inherited from RenderContentEventArgs.)
- `Document` (RhinoDoc, get) — (Inherited from RenderContentEventArgs.)
- `EnvironmentUsage` (RenderEnvironment.Usage, get) — Meaningful for CurrentEnvironmentChanged event. Will be one of Background, ReflectionAndRefraction or Skylighting. Since 6.11
- `FieldName` (String, get) — 
- `OldContent` (RenderContent, get) — (Inherited from RenderContentChangedEventArgs.)
- `Reason` (RenderContentChangeReason, get) — Not when used in CurrentEnvironmentChanged (defaults to None).

## RenderContentKind (enum)

Defines constant values for all render content kinds, such as material, environment or texture.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentKind.htm)

### Values
- `None` = `0` — Specifies no RenderContent kind.
- `Material` = `1` — Specifies a material RenderContent kind.
- `Environment` = `2` — Specifies an environment RenderContent kind.
- `Texture` = `4` — Specifies a texture RenderContent kind.

## RenderContentKindList (class)

Models a collection of kinds.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentKindList.htm)

### Constructors
- `public RenderContentKindList()` — Construct an empty kind list
- `public RenderContentKindList(IntPtr pRdkRenderContentKindList)` — Construct from native pointer - internal use only.
- `public RenderContentKindList(RenderContentKindList kind_list)` — Construct a kind list from another.

### Methods
#### `public void Add(RenderContentKind kind)`

Add a kind to a kind list.

**Parameters:**
- `kind` (Rhino.Render.RenderContentKind) — The RenderContent kind to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentKindList_Add.htm)

#### `public bool Contains(RenderContentKind kind)`

Query whether or not the collection contains a particular kind designation.

**Parameters:**
- `kind` (Rhino.Render.RenderContentKind) — [Missing <param name="kind"/> documentation for "M:Rhino.Render.RenderContentKindList.Contains(Rhino.Render.RenderContentKind)"]

**Returns:** `Boolean` — true if the kind is in the collection.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentKindList_Contains.htm)

#### `public int Count()`

The number of kinds in the collection

**Returns:** `Int32` — The number of kinds in the collection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentKindList_Count.htm)

#### `public void Dispose()`

Dispose a kind list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentKindList_Dispose.htm)

#### `protected override void Finalize()`

Kind list finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentKindList_Finalize.htm)

#### `public RenderContentKind SingleKind()`

The single kind in the list. If the list does not contain exactly one kind, returns 'None'.

**Returns:** `RenderContentKind` — A RenderContentKind

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentKindList_SingleKind.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## RenderContentManager (class)

RenderContentManager's RestoreRenderContents method unpacks the default render contents from the from the application and places them in the User's folder. Only available on Mac at the moment.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentManager.htm)

### Methods
#### `public static bool RestoreRenderContent()`

Unpacks the default render contents from the from the application and places them in the User's folder. This will restore the default versions if the version of Rhino that is running is newer than the contents of the last Rhino that wrote the version.txt file. If the version.txt file is not present, it will automatically restore the default contents. This does not overwrite files that the user has changed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentManager.RestoreRenderContent"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentManager_RestoreRenderContent.htm)

### Properties
- `UserRenderContentPath` (String, get) — Get the path to: Windows: C:\Users\user\AppData\Roaming\McNeel\Rhinoceros\6.0\Localization\en-US\Render Content macOS: ~/Library/Application Support/McNeel/Rhinoceros/6.0/Render Content If a CustomLibraryPath is set, this is returned

## RenderContentSerializer (class)

Used to import and export custom render content such as materials, environments and textures. You must override RenderPlugIn.RenderContentSerializers() and return an array of derived RenderContentSerializer class types to add to the content browsers.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentSerializer.htm)

### Constructors
- `protected RenderContentSerializer(string fileExtension, RenderContentKind contentKind, bool canRead, bool canWrite)` — Protected constructor to be called from derived class

### Methods
#### `public virtual bool CanLoadMultiple()`

If true the plug-in is capable of loading multiple contents.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentSerializer.CanLoadMultiple"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_CanLoadMultiple.htm)

#### `public virtual bool LoadMultiple(RhinoDoc doc, IEnumerable<string> fileNames, RenderContentKind contentKind, RenderContentSerializer.LoadMultipleFlags flags)`

Create any number of new render contents loaded from any number of files.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Rhino document
- `fileNames` (System.Collections.Generic.IEnumerable<String>) — A list of filenames to load from. Each file can contain any number of render contents.
- `contentKind` (Rhino.Render.RenderContentKind) — Only used by I/O plug-ins that support multiple kinds. It tells the plug-in which content kind to create. If the plug-in only supports a single content kind, it can ignore this parameter.
- `flags` (Rhino.Render.RenderContentSerializer.LoadMultipleFlags) — A set of flags from the enum above.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentSerializer.LoadMultiple(Rhino.RhinoDoc,System.Collections.Generic.IEnumerable{System.String},Rhino.Render.RenderContentKind,Rhino.Render.RenderContentSerializer.LoadMultipleFlags)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_LoadMultiple.htm)

#### `public abstract RenderContent Read(string pathToFile)`

Called to when importing a file, file should be parsed and converted to a valid RenderContent object.

**Parameters:**
- `pathToFile` (System.String) — Full path of the file to load.

**Returns:** `RenderContent` — Returns a valid RenderContent object such as RenderMaterial if the file was successfully parsed otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_Read.htm)

#### `public bool RegisterSerializer(Guid id)`

Register the RenderContentSerializer

**Parameters:**
- `id` (System.Guid) — Plug-in id

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentSerializer.RegisterSerializer(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_RegisterSerializer.htm)

#### `public virtual void ReportContentAndFile(RenderContent renderContent, string pathToFile, int flags)`

This is called from your implementation of LoadMultiple() to add a content and the file it was loaded from when the LoadMultipleFlags.Preload flag is NOT set. See LoadMultiple() for an explanation of this method's use.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Render content that was loaded from the file.
- `pathToFile` (System.String) — Full path of the file that the render content was loaded from.
- `flags` (System.Int32) — Flags for future use; should be passed as zero.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_ReportContentAndFile.htm)

#### `public virtual void ReportDeferredContentAndFile(RenderContent renderContent, string pathToFile, int flags)`

This is called from your implementation of LoadMultiple() to add a 'deferred' content and the file it will be loaded from, when the LoadMultipleFlags.Preload flag is set. See LoadMultiple() for an explanation of this method's use. \param c is the deferred content. \param wszFullPath is the full path to the file that 'c' will be loaded from. \param flags is reserved for future use; you should pass zero. \param pReserved is reserved for future use; you should pass nullptr. */

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Deferred render content - represents all contents that will be loaded from the file.
- `pathToFile` (System.String) — Full path of the file that render contents will be loaded from.
- `flags` (System.Int32) — Flags for future use; should be passed as zero.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_ReportDeferredContentAndFile.htm)

#### `public abstract bool Write(string pathToFile, RenderContent renderContent, CreatePreviewEventArgs previewArgs)`

Called to save a custom RenderContent object as an external file.

**Parameters:**
- `pathToFile` (System.String) — Full path of file to write
- `renderContent` (Rhino.Render.RenderContent) — Render content to save
- `previewArgs` (Rhino.Render.CreatePreviewEventArgs) — Parameters used to generate a preview image which may be embedded in the exported file.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentSerializer.Write(System.String,Rhino.Render.RenderContent,Rhino.Render.CreatePreviewEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentSerializer_Write.htm)

### Properties
- `CanRead` (Boolean, get) — If true then the file type can be imported and will be included in the file open box when importing the specified render content kind.
- `CanWrite` (Boolean, get) — If true then the file type can be exported and will be included in the file save box when exporting the specified render content kind.
- `ContentKind` (RenderContentKind, get) — Kind of content created when importing or exporting this file type - ie, Material, Texture or Environment
- `ContentType` (RenderContentKind, get) — Kind of content created when importing or exporting this file type. Obsolete - use ContentKind
- `EnglishDescription` (String, get) — English string describing this plug-in
- `FileExtension` (String, get) — File extension associated with this serialize object
- `LocalDescription` (String, get) — Localized plug-in description

## RenderContentSerializer.LoadMultipleFlags (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderContentSerializer.LoadMultipleFlags"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentSerializer_LoadMultipleFlags.htm)

### Values
- `Normal` = `0`
- `Preload` = `1`

## RenderContentStyles (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderContentStyles"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentStyles.htm)

### Values
- `None` = `0` — No defined styles
- `TextureSummary` = `1` — Texture UI includes an auto texture summary section. See AddAutoParameters().
- `QuickPreview` = `2` — Editor displays an instant preview before preview cycle begins.
- `PreviewCache` = `4` — Content's preview imagery can be stored in the preview cache.
- `ProgressivePreview` = `8` — Content's preview imagery can be rendered progressively.
- `LocalTextureMapping` = `16` — Texture UI includes an auto local mapping section for textures. See AddAutoParameters()
- `GraphDisplay` = `32` — Texture UI includes a graph section.
- `NameTypeSection` = `2048` — Render content UI includes a 'Name' and 'Type' section.
- `SharedUI` = `64` — Obsolete. OBSOLETE: The UI is always shared between render contents of the same type id.
- `Adjustment` = `128` — Texture UI includes an adjustment section.
- `Fields` = `256` — Render content uses fields to facilitate data storage and undo support. See Fields()
- `ModalEditing` = `512` — Render content supports editing in a modal editor. OBSOLETE: All contents support modal editing.
- `DynamicFields` = `1024` — The render content's fields are dynamic. OBSOLETE: Dynamic fields can co-exist with static fields.

## RenderContentType (class)

Represents one of the render content types registered with Rhino.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderContentType.htm)

### Constructors
- `public RenderContentType(Guid typeId)` — Initializes a new instance of the RenderContentType class

### Methods
#### `public void Dispose()`

Releases all resources used by the RenderContentType

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the RenderContentType and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_Finalize.htm)

#### `public static RenderContentType[] GetAllAvailableTypes()`

Gets an array of all available render content types registered with Rhino.

**Returns:** `RenderContentType[]` — An array with all types.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_GetAllAvailableTypes.htm)

#### `public static RenderContent NewContentFromTypeId(Guid typeId)`

Create a new content specified by the Guid. This function can be used to create temporary content, as it calls ::RhRdkContentFactories().NewContentFromType().

**Parameters:**
- `typeId` (System.Guid) — [Missing <param name="typeId"/> documentation for "M:Rhino.Render.RenderContentType.NewContentFromTypeId(System.Guid)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentType.NewContentFromTypeId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_NewContentFromTypeId.htm)

#### `public static RenderContent NewContentFromTypeId(Guid typeId, RhinoDoc doc)`



**Parameters:**
- `typeId` (System.Guid) — [Missing <param name="typeId"/> documentation for "M:Rhino.Render.RenderContentType.NewContentFromTypeId(System.Guid,Rhino.RhinoDoc)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.RenderContentType.NewContentFromTypeId(System.Guid,Rhino.RhinoDoc)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContentType.NewContentFromTypeId(System.Guid,Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_NewContentFromTypeId_1.htm)

#### `public RenderContent NewRenderContent()`

Returns a new instance of the render content of this type. This content can be added to a persistant list.

**Returns:** `RenderContent` — A new render content instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContentType_NewRenderContent.htm)

### Properties
- `Id` (Guid, get) — Returns the type identifier associated with this type.
- `InternalName` (String, get) — Returns the internal name identifier associated with this type.
- `PlugInId` (Guid, get) — 
- `RenderEngineId` (Guid, get) — 

## RenderEndEventArgs (class)

Contains information about why OnRenderEnd was called

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderEndEventArgs.htm)

### Constructors
- `public RenderEndEventArgs()` — Initializes a new instance of the RenderEndEventArgs class

## RenderEnvironment (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderEnvironment"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderEnvironment.htm)

### Constructors
- `protected RenderEnvironment()` — Initializes a new instance of the RenderEnvironment class

### Methods
#### `public bool AddAutomaticUserInterfaceSection(string caption, int id)`

Add a new automatic user interface section, Field values which include prompts will be automatically added to this section.

**Parameters:**
- `caption` (System.String) — Expandable tab caption.
- `id` (System.Int32) — Tab id which may be used later on to reference this tab.

**Returns:** `Boolean` — Returns true if the automatic tab section was added otherwise; returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddAutomaticUserInterfaceSection.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent, string childSlotName)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild_1.htm)

#### `public bool AddUserInterfaceSection(ICollapsibleSection section)`

(Inherited from RenderContent.)

**Parameters:**
- `section` (Rhino.UI.Controls.ICollapsibleSection) — [Missing <param name="section"/> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection.htm)

#### `[ObsoleteAttribute("Use AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection) below instead. This function will not work on the Mac and is not type-safe.")] public UserInterfaceSection AddUserInterfaceSection(Type classType, string caption, bool createExpanded, bool createVisible)`

Add a new .NET control to an content expandable tab section, the height of the createExpanded tabs client area will be the initial height of the specified control.

**Parameters:**
- `classType` (System.Type) — The control class to create and embed as a child window in the expandable tab client area. This class type must be derived from IWin32Window or this method will throw an ArgumentException. Implement the IUserInterfaceSection interface in your classType to get UserInterfaceSection notification.
- `caption` (System.String) — Expandable tab caption.
- `createExpanded` (System.Boolean) — If this value is true then the new expandable tab section will initially be expanded, if it is false it will be collapsed.
- `createVisible` (System.Boolean) — If this value is true then the new expandable tab section will initially be visible, if it is false it will be hidden.

**Returns:** `UserInterfaceSection` — Returns the UserInterfaceSection object used to manage the new user control object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection_1.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`

Begins a change or batch of changes. It may also make a copy of the content state allowing EndChange() to send an event with the old and new contents. Calls to this method are counted; you must call EndChange() once for every call to BeginChange(). Note: If Changed() was called between the calls to BeginChange() and EndChange(), the last call to EndChange() may cause the ContentChanged event to be sent.

**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — the change context. If this is ChangeContexts.UI, ChangeContexts.Program,ChangeContexts.Drop or ChangeContexts.Tree, the content will be copied. EndChange() will then send the copy as 'old' in the OnContentChanged event. EndChange()ContentChanged

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginChange.htm)

#### `public void BeginCreateDynamicFields(bool automatic)`

Automatic Dynamic Field support. Dynamic fields are typically created in the constructor of RenderContent and they are therefore created automatically whenever the content is created. However, some advanced users require the fields to be created in response to some user action which occurs much later. This creates the problem that the fields do not exist by default and therefore cannot be loaded when the document is loaded. These methods are provided to solve that problem by making it possible to automatically create the dynamic fields on loading if they don't already exist. Dynamic fields that have this auto-create-on-load behavior are referred to as automatic dynamic fields. Dynamic fields that do not require the advanced automatic feature can still be created by using these methods (recommended), or they can be created manually (legacy usage). You must call this before creating any dynamic fields. Calls to this method are counted; you must call EndCreateDynamicFields() once for every call to BeginCreateDynamicFields().

**Parameters:**
- `automatic` (System.Boolean) — automatic specifies if the dynamic fields are automatic. If so, they will be created automatically during loading of the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginCreateDynamicFields.htm)

#### `public void BindParameterToField(string parameterName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField.htm)

#### `public void BindParameterToField(string parameterName, string childSlotName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField_1.htm)

#### `protected virtual uint CalculateRenderHash(ulong rcrcFlags)`

Override this method to calculate the render hash of the state that affects how the content is rendered. Does not include children or perform any caching. Render hash values are now automatically cached by the content framework and you do not have to worry about caching. You also do not have to worry about iterating into children. This method is now only called internally by the framework, use the RenderHash property to get the current hash value.

**Parameters:**
- `rcrcFlags` (System.UInt64) — [Missing <param name="rcrcFlags"/> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CalculateRenderHash.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool ChangeChild(RenderContent oldContent, RenderContent newContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — [Missing <param name="oldContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]
- `newContent` (Rhino.Render.RenderContent) — [Missing <param name="newContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChangeChild.htm)

#### `public double ChildSlotAmount(string childSlotName)`

Gets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Double` — The requested amount value. Values are typically from 0.0 to 100.0

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotAmount.htm)

#### `public string ChildSlotNameFromParamName(string paramName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `paramName` (System.String) — The name of a parameter field. Since child textures will usually correspond with some parameter (they generally either replace or modify a parameter over UV space) these functions are used to specify which parameter corresponded with child slot. If there is no correspondence, return the empty string.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotNameFromParamName.htm)

#### `public bool ChildSlotOn(string childSlotName)`

Gets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotOn.htm)

#### `public virtual void ConvertUnits(UnitSystem from, UnitSystem to)`

Modify your content so that it is converted from meters into the units of the unit system. No need to call the base class when you override this, and no need to recurse into children.

**Parameters:**
- `from` (Rhino.UnitSystem) — [Missing <param name="from"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]
- `to` (Rhino.UnitSystem) — [Missing <param name="to"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ConvertUnits.htm)

#### `public bool CreateDynamicField(string internalName, string localName, string englishName, Object value, Object minValue, Object maxValue, int sectionId)`

Create a dynamic field with an initial value and min and max limits.

**Parameters:**
- `internalName` (System.String) — is the internal name of the field. Not localized.
- `localName` (System.String) — is the localized user-friendly name of the field.
- `englishName` (System.String) — is the English user-friendly name of the field.
- `value` (System.Object) — is the initial value of the field.
- `minValue` (System.Object) — is the minimum value of the field. Must be the same type as vValue.
- `maxValue` (System.Object) — is the maximum value of the field. Must be the same type as vValue.
- `sectionId` (System.Int32) — is used for filtering fields between sections. Zero if not needed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CreateDynamicField(System.String,System.String,System.String,System.Object,System.Object,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CreateDynamicField.htm)

#### `public void DeleteAllChildren(RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteAllChildren(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteAllChildren.htm)

#### `public bool DeleteChild(string childSlotName, RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteChild.htm)

#### `public void Dispose()`

(Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Dispose

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose_1.htm)

#### `public virtual bool DynamicIcon(Size size, out Bitmap bitmap, DynamicIconUsage usage)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `usage` (Rhino.Render.DynamicIconUsage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DynamicIcon.htm)

#### `public RenderContent Edit()`

This method allows a render content hierarchy to be edited using a modal (AKA 'pop-up') editor. If the original render content is in a document, it will remain there, and the edited one will be 'free-floating'. Therefore it is the caller's responsibility to do any replacement in the document if required. The returned new content will be owned by the caller.

**Returns:** `RenderContent` — Returns an edited version of the content hierarchy if successful, else null. The method always edits the entire hierarchy. It places a copy of the hierarchy in the editor and selects the copied item that corresponds to this one. Therefore, editing a child will return a top-level render content, not a child.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Edit.htm)

#### `public void EndChange()`

Ends a change or batch of changes. Calls to this method are counted; you must call this method once for every call to BeginChange(RenderContent.ChangeContexts). Note: If BeginChange(RenderContent.ChangeContexts) was called with ChangeContexts.UI, ChangeContexts.Program, ChangeContexts.Drop or ChangeContexts.UI.Tree and Changed() was called between the calls to BeginChange(RenderContent.ChangeContexts) and EndChange(), the last call to EndChange() will raise the ContentChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndChange.htm)

#### `public void EndCreateDynamicFields()`

You must call this after creating dynamic fields. Calls to this method are counted; you must call BeginCreateDynamicFields() once for every call to EndCreateDynamicFields().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndCreateDynamicFields.htm)

#### `public ContentFactory Factory()`

(Inherited from RenderContent.)

**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Factory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Factory.htm)

#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Finalize.htm)

#### `public RenderContent FindChild(string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FindChild.htm)

#### `public RenderContent ForDisplay()`

The only place a single proxy can be displayed is in the New Content Control main thumbnail. All other attempts to use a single proxy in a UI require it to be replaced with the corresponding multi proxy. Single proxies override this to find the corresponding multi proxy.

**Returns:** `RenderContent` — The cotnent.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ForDisplay.htm)

#### `public virtual Object GetChildSlotParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to specify additional functionality for automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call GetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetChildSlotParameter.htm)

#### `public string[] GetEmbeddedFilesList()`

(Inherited from RenderContent.)

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetEmbeddedFilesList"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetEmbeddedFilesList.htm)

#### `public Object GetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetExtraRequirementParameter.htm)

#### `public virtual Object GetParameter(string parameterName)`

Query the content instance for the value of a given named parameter. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — Name of the parameter

**Returns:** `Object` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetParameter.htm)

#### `public virtual ulong GetUiHash()`

This allows a content to have more than one UI for the same content type.

**Returns:** `UInt64` — Default is zero and is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUiHash.htm)

#### `public bool GetUnderlyingInstances(ref RenderContentCollection collection)`

(Inherited from RenderContent.)

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUnderlyingInstances.htm)

#### `public virtual bool Icon(Size size, out Bitmap bitmap)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Icon.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public bool Initialize()`

Obsolete. (Inherited from RenderContent.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Initialize.htm)

#### `public virtual bool IsCompatible(Guid renderEngineId)`

(Inherited from RenderContent.)

**Parameters:**
- `renderEngineId` (System.Guid) — [Missing <param name="renderEngineId"/> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsCompatible.htm)

#### `public virtual bool IsContentTypeAcceptableAsChild(Guid type, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `type` (System.Guid) — [Missing <param name="type"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsContentTypeAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(ContentFactory factory, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `factory` (Rhino.Render.DataSources.ContentFactory) — [Missing <param name="factory"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(Guid kindId, string factoryKind, string childSlotName)`

Override this method to restrict the type of acceptable child content. The default implementation of this method just returns true.

**Parameters:**
- `kindId` (System.Guid) — [Missing <param name="kindId"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `factoryKind` (System.String) — [Missing <param name="factoryKind"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]

**Returns:** `Boolean` — Return true only if content with the specified kindId can be accepted as a child in the specified child slot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild_1.htm)

#### `public bool IsReference()`

Query whether or not the content or any of its ancestors is a reference content.

**Returns:** `Boolean` — true if the content is a reference, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsReference.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public bool IsRenderHashCached()`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Returns:** `Boolean` — bool

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsRenderHashCached.htm)

#### `public RenderContent MakeCopy()`

Create a copy of the render content. All content is the same, except for the instance Id.

**Returns:** `RenderContent` — The new RenderContent

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeCopy.htm)

#### `public RenderContent MakeGroupInstance()`

Create an 'instance' of the content hierarchy and group the new hierarchy with this hierarchy. If the instance is subsequently attached to the same document, the state of all members of the group will be kept synchronized. With the exception of the instance ids, all state is exactly preserved in the new instance hierarchy. \note The grouping will have no effect until the new instance is attached to the same document.

**Returns:** `RenderContent` — A grouped instance of the content hierarchy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeGroupInstance.htm)

#### `public virtual RenderContent.MatchDataResult MatchData(RenderContent oldContent)`

Implement to transfer data from another content to this content during creation.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — An old content object from which the implementation may harvest data.

**Returns:** `RenderContent.MatchDataResult` — Information about how much data was matched.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MatchData.htm)

#### `protected void ModifyRenderContentStyles(RenderContentStyles stylesToAdd, RenderContentStyles stylesToRemove)`

ModifyRenderContentStyles

**Parameters:**
- `stylesToAdd` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToAdd"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]
- `stylesToRemove` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToRemove"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ModifyRenderContentStyles.htm)

#### `public static RenderEnvironment NewBasicEnvironment(SimulatedEnvironment environment)`

Constructs a new RenderEnvironment from a SimulatedEnvironment.

**Parameters:**
- `environment` (Rhino.Render.SimulatedEnvironment) — The environment to create the basic environment from.

**Returns:** `RenderEnvironment` — A new basic environment.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironment_NewBasicEnvironment.htm)

#### `public static RenderEnvironment NewBasicEnvironment(SimulatedEnvironment environment, RhinoDoc doc)`



**Parameters:**
- `environment` (Rhino.Render.SimulatedEnvironment) — [Missing <param name="environment"/> documentation for "M:Rhino.Render.RenderEnvironment.NewBasicEnvironment(Rhino.Render.SimulatedEnvironment,Rhino.RhinoDoc)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.RenderEnvironment.NewBasicEnvironment(Rhino.Render.SimulatedEnvironment,Rhino.RhinoDoc)"]

**Returns:** `RenderEnvironment` — [Missing <returns> documentation for "M:Rhino.Render.RenderEnvironment.NewBasicEnvironment(Rhino.Render.SimulatedEnvironment,Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironment_NewBasicEnvironment_1.htm)

#### `public PreviewSceneServer NewPreviewSceneServer(SceneServerData ssd)`

Gets the PreviewSceneServer of the content

**Parameters:**
- `ssd` (Rhino.Render.SceneServerData) — SceneServerData

**Returns:** `PreviewSceneServer` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.NewPreviewSceneServer(Rhino.Render.SceneServerData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_NewPreviewSceneServer.htm)

#### `protected virtual void OnAddUserInterfaceSections()`

Override to provide UI sections to display in the editor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnAddUserInterfaceSections.htm)

#### `protected virtual bool OnGetDefaultsInteractive()`

Override this method to prompt user for information necessary to create a new content object. For example, if you are created a textured material you may prompt the user for a bitmap file name prior to creating the textured material.

**Returns:** `Boolean` — If true is returned the content is created otherwise the creation is aborted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnGetDefaultsInteractive.htm)

#### `protected virtual void OnMakeCopy(ref RenderContent newContent)`

Override this function to supplement the standard copying behavour for your RenderContent.

**Parameters:**
- `newContent` (Rhino.Render.RenderContent) — Is the content that will be returned from MakeCopy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnMakeCopy.htm)

#### `public bool OpenInEditor()`

Call this method to open the content in the relevant thumbnail editor and select it for editing by the user. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInEditor.htm)

#### `[ObsoleteAttribute("Obsolete, use Edit a version that returns a RenderContent", false)] public bool OpenInModalEditor()`

Call this method to open the content in the a modal version of the editor. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInModalEditor.htm)

#### `public string ParamNameFromChildSlotName(string childSlotName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `childSlotName` (System.String) — The named of the child slot to receive the parameter name for.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ParamNameFromChildSlotName.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames, LinearWorkflow lw)`

As RenderHash, but ignore parameter names given. Use this version of the function to calculate a render hash when you have the linear workflow information and you are not running on the main thread. Access to LinearWorkflow data requires document access. CrcRenderHashFlags.ExcludeLinearWorkflow must be specified.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string
- `lw` (Rhino.Render.LinearWorkflow) — Linear Workflow to use for CRC

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_1.htm)

#### `public uint RenderHashExclude(TextureRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.TextureRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_2.htm)

#### `public bool Replace(RenderContent newcontent)`

(Inherited from RenderContent.)

**Parameters:**
- `newcontent` (Rhino.Render.RenderContent) — [Missing <param name="newcontent"/> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Replace.htm)

#### `public bool SetChild(RenderContent renderContent, string childSlotName)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild.htm)

#### `[ObsoleteAttribute("Use SetChild without ChangeContexts and Begin/EndChange")] public bool SetChild(RenderContent renderContent, string childSlotName, RenderContent.ChangeContexts changeContexts)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.SetChild(Rhino.Render.RenderContent,System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild_1.htm)

#### `public void SetChildSlotAmount(string childSlotName, double amount, RenderContent.ChangeContexts cc)`

Sets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `amount` (System.Double) — The texture amount. Values are typically from 0.0 to 100.0
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — The context of the change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotAmount.htm)

#### `public void SetChildSlotOn(string childSlotName, bool bOn, RenderContent.ChangeContexts cc)`

Sets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `bOn` (System.Boolean) — Value for the on-ness property.
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Context of the change

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotOn.htm)

#### `public virtual bool SetChildSlotParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to support values being set from automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call SetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotParameter.htm)

#### `public bool SetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetExtraRequirementParameter.htm)

#### `public void SetIsRenderHashRecursive(bool recursive)`

By default, RenderHash() recurses into children when computing the render CRC. However, some applications may require children to be excluded from the render CRC calculation. Call this method to enable or disable recursing into children. see RenderHash

**Parameters:**
- `recursive` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetIsRenderHashRecursive.htm)

#### `public void SetName(string name, bool renameEvents, bool ensureNameUnique)`

Set instance name for this content

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `renameEvents` (System.Boolean) — [Missing <param name="renameEvents"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `ensureNameUnique` (System.Boolean) — [Missing <param name="ensureNameUnique"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetName.htm)

#### `public virtual bool SetParameter(string parameterName, Object value)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter.htm)

#### `[ObsoleteAttribute("Use SetParameter without ChangeContexts and Begin/EndChange")] public virtual bool SetParameter(string parameterName, Object value, RenderContent.ChangeContexts changeContext)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter_1.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public void SetRenderHash(uint hash)`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Parameters:**
- `hash` (System.UInt32)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetRenderHash.htm)

#### `public virtual SimulatedEnvironment SimulateEnvironment(bool isForDataOnly)`



**Parameters:**
- `isForDataOnly` (System.Boolean) — [Missing <param name="isForDataOnly"/> documentation for "M:Rhino.Render.RenderEnvironment.SimulateEnvironment(System.Boolean)"]

**Returns:** `SimulatedEnvironment` — [Missing <returns> documentation for "M:Rhino.Render.RenderEnvironment.SimulateEnvironment(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironment_SimulateEnvironment_1.htm)

#### `public virtual void SimulateEnvironment(ref SimulatedEnvironment simulation, bool isForDataOnly)`



**Parameters:**
- `simulation` (Rhino.Render.SimulatedEnvironment) — [Missing <param name="simulation"/> documentation for "M:Rhino.Render.RenderEnvironment.SimulateEnvironment(Rhino.Render.SimulatedEnvironment@,System.Boolean)"]
- `isForDataOnly` (System.Boolean) — [Missing <param name="isForDataOnly"/> documentation for "M:Rhino.Render.RenderEnvironment.SimulateEnvironment(Rhino.Render.SimulatedEnvironment@,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironment_SimulateEnvironment.htm)

#### `public bool SmartUngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. If any content in the same document is left alone in the group, that content is also ungrouped. Records undo and sends events OnContentChanged and OnContentGroupIdChanged. \note This method is designed to be called from a content UI and is intended for RDK internal use but may be used as an expert user override.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SmartUngroupRecursive.htm)

#### `public bool Ungroup()`

Remove this content from any instance group it may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if content was ungrouped, \e false if it was not part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Ungroup.htm)

#### `public bool UngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UngroupRecursive.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public void Uninitialize()`

Obsolete. (Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Uninitialize.htm)

#### `public int UseCount()`

UseCount returns how many times the content is used

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.UseCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UseCount.htm)

#### `public virtual bool VirtualIcon(Size size, out Bitmap bitmap)`

Icon to display in the content browser, this bitmap needs to be valid for the life of this content object, the content object that returns the bitmap is responsible for disposing of the bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Requested icon size
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.VirtualIcon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — Return Icon to display in the content browser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_VirtualIcon.htm)

### Properties
- `CanBeEdited` (Boolean, get) — Determines if the content can be edited.
- `Category` (String, get) — Category for this content.
- `ChildSlotDisplayName` (String, get) — Returns the localized display name of the child slot name
- `ChildSlotName` (String, get/set) — (Inherited from RenderContent.)
- `CppPointer` (IntPtr, get) — (Inherited from RenderContent.)
- `CurrentEnvironment` (RenderEnvironment, get/set) — Obsolete.
- `DisplayName` (String, get) — Display name for this content.
- `Document` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `DocumentAssoc` (RhinoDoc, get/set) — If this render content is associated with a document in any way, the document will be returned. This includes copies of render contents that were attached to a document when the copy was made. Otherwise returns null.
- `DocumentOwner` (RhinoDoc, get) — If this render content is owned by a document, the document will be returned. This is the same as getting the document the render content is attached to. Otherwise returns null.
- `DocumentRegistered` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `Fields` (FieldDictionary, get) — Rhino.Render.Fields FieldDictionary which provides access to setting and retrieving field values.
- `Filename` (String, get/set) — If the content is file based, this function can be overridden to deal with setting/getting the filename. Corresponds to IRhRdkFileBasedContent in the C++ SDK
- `FilesToEmbed` (IEnumerable<String>, get) — A string array of full paths to files used by the content that may be embedded in .3dm files and library files (.rmtl, .renv, .rtex). The default implementation returns an empty string list. Override this to return the file name or file names used by your content. This is typically used by textures that reference files containing the texture imagery.
- `FirstChild` (RenderContent, get) — Return First child of this content or nullptr if none.
- `GroupId` (Guid, get/set) — Group ID of the content
- `Hidden` (Boolean, get/set) — Gets or sets the render content's 'hidden' state. This feature only works for top-level render contents because it hides the entire hierarchy. It is normally used for 'implementation detail' render contents. For expert use only. Hidden render contents are never shown in the UI, with the exception of the Object (or Layer) Material Properties UI which always shows whatever is assigned to the object (or layer). In the Object (or Layer) Material Properties UI, if the user drops down the list, hidden render contents are not listed. Hidden render contents, being part of the document content list, will be listed by any scripts or other code that iterates over the document render content list. It is recommended that you set IsHidden once when you create your render content and leave it on to prevent flicker or slow performance.
- `Id` (Guid, get/set) — Instance identifier for this content.
- `IsDefaultInstance` (Boolean, get) — Checks if render content is default instance.
- `IsHiddenByAutoDelete` (Boolean, get) — Contents can be created as 'auto-delete' by certain commands such as 'Picture'. These contents are automatically hidden from the user when the associated Rhino object is deleted. They are later deleted when the document is saved.
- `IsLocked` (Boolean, get/set) — Set this property to true prior to adding content to the document to lock the content browser editing UI methods. Setting this to true will keep the browser from allowing things like deleting, renaming or changing content. This is useful for custom child content that you want to be editable but persistent. Setting this after adding content to the document will cause an exception to be thrown.
- `Name` (String, get/set) — Instance 'raw' name for this content.
- `NextSibling` (RenderContent, get) — Return First sibling of this content or nullptr if none.
- `Notes` (String, get/set) — Notes for this content.
- `Parent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `ProxyType` (ProxyTypes, get) — Gets the proxy type of the render content
- `RenderHash` (UInt32, get) — Render hash for the content hierarchy. It iterates children and includes a caching mechanism which means the hash value can be retrieved quickly if it hasn't changed. The cache is invalidated when Changed() is called. You can override the CalculateRenderHash(UInt64) method to provide a custom hash value.
- `Styles` (RenderContentStyles, get) — (Inherited from RenderContent.)
- `Tags` (String, get/set) — Tags for this content.
- `TextureChildSlotName` (String, get) — 
- `TopLevel` (Boolean, get) — Returns true if this content has no parent, false if it is the child of another content.
- `TopLevelParent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `TypeDescription` (String, get) — Description for your content type. ie. "Procedural checker pattern"
- `TypeId` (Guid, get) — Type identifier for this content
- `TypeName` (String, get) — Name for your content type. ie. "My .net Texture"
- `Xml` (String, get) — (Inherited from RenderContent.)

## RenderEnvironment.Usage (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderEnvironment.Usage"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderEnvironment_Usage.htm)

### Values
- `None` = `0`
- `Background` = `1`
- `ReflectionAndRefraction` = `2`
- `Skylighting` = `4`
- `AnyUsage` = `7`

## RenderEnvironmentTable (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderEnvironmentTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderEnvironmentTable.htm)

### Methods
#### `public bool Add(RenderEnvironment c)`



**Parameters:**
- `c` (Rhino.Render.RenderEnvironment) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderEnvironmentTable.Add(Rhino.Render.RenderEnvironment)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderEnvironmentTable.Add(Rhino.Render.RenderEnvironment)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironmentTable_Add.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderEnvironmentTable.BeginChange(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironmentTable_BeginChange.htm)

#### `public void EndChange()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironmentTable_EndChange.htm)

#### `public IEnumerator<RenderEnvironment> GetEnumerator()`



**Returns:** `IEnumerator<RenderEnvironment>` — [Missing <returns> documentation for "M:Rhino.Render.RenderEnvironmentTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironmentTable_GetEnumerator.htm)

#### `public bool Remove(RenderEnvironment c)`



**Parameters:**
- `c` (Rhino.Render.RenderEnvironment) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderEnvironmentTable.Remove(Rhino.Render.RenderEnvironment)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderEnvironmentTable.Remove(Rhino.Render.RenderEnvironment)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderEnvironmentTable_Remove.htm)

### Properties
- `Count` (Int32, get) — 
- `Item` (RenderEnvironment, get) — 

## RenderMaterial (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderMaterial"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial.htm)

### Constructors
- `protected RenderMaterial()` — Initializes a new instance of the RenderMaterial class

### Methods
#### `public bool AddAutomaticUserInterfaceSection(string caption, int id)`

Add a new automatic user interface section, Field values which include prompts will be automatically added to this section.

**Parameters:**
- `caption` (System.String) — Expandable tab caption.
- `id` (System.Int32) — Tab id which may be used later on to reference this tab.

**Returns:** `Boolean` — Returns true if the automatic tab section was added otherwise; returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddAutomaticUserInterfaceSection.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent, string childSlotName)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild_1.htm)

#### `public bool AddUserInterfaceSection(ICollapsibleSection section)`

(Inherited from RenderContent.)

**Parameters:**
- `section` (Rhino.UI.Controls.ICollapsibleSection) — [Missing <param name="section"/> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection.htm)

#### `[ObsoleteAttribute("Use AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection) below instead. This function will not work on the Mac and is not type-safe.")] public UserInterfaceSection AddUserInterfaceSection(Type classType, string caption, bool createExpanded, bool createVisible)`

Add a new .NET control to an content expandable tab section, the height of the createExpanded tabs client area will be the initial height of the specified control.

**Parameters:**
- `classType` (System.Type) — The control class to create and embed as a child window in the expandable tab client area. This class type must be derived from IWin32Window or this method will throw an ArgumentException. Implement the IUserInterfaceSection interface in your classType to get UserInterfaceSection notification.
- `caption` (System.String) — Expandable tab caption.
- `createExpanded` (System.Boolean) — If this value is true then the new expandable tab section will initially be expanded, if it is false it will be collapsed.
- `createVisible` (System.Boolean) — If this value is true then the new expandable tab section will initially be visible, if it is false it will be hidden.

**Returns:** `UserInterfaceSection` — Returns the UserInterfaceSection object used to manage the new user control object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection_1.htm)

#### `public bool AssignTo(IEnumerable<ObjRef> objrefs, RenderMaterial.AssignToSubFaceChoices sfc, RenderMaterial.AssignToBlockChoices bc, bool bInteractive)`



**Parameters:**
- `objrefs` (System.Collections.Generic.IEnumerable<ObjRef>) — [Missing <param name="objrefs"/> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef},Rhino.Render.RenderMaterial.AssignToSubFaceChoices,Rhino.Render.RenderMaterial.AssignToBlockChoices,System.Boolean)"]
- `sfc` (Rhino.Render.RenderMaterial.AssignToSubFaceChoices) — [Missing <param name="sfc"/> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef},Rhino.Render.RenderMaterial.AssignToSubFaceChoices,Rhino.Render.RenderMaterial.AssignToBlockChoices,System.Boolean)"]
- `bc` (Rhino.Render.RenderMaterial.AssignToBlockChoices) — [Missing <param name="bc"/> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef},Rhino.Render.RenderMaterial.AssignToSubFaceChoices,Rhino.Render.RenderMaterial.AssignToBlockChoices,System.Boolean)"]
- `bInteractive` (System.Boolean) — [Missing <param name="bInteractive"/> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef},Rhino.Render.RenderMaterial.AssignToSubFaceChoices,Rhino.Render.RenderMaterial.AssignToBlockChoices,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(System.Collections.Generic.IEnumerable{Rhino.DocObjects.ObjRef},Rhino.Render.RenderMaterial.AssignToSubFaceChoices,Rhino.Render.RenderMaterial.AssignToBlockChoices,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_AssignTo_1.htm)

#### `public bool AssignTo(ObjRef or)`



**Parameters:**
- `or` (Rhino.DocObjects.ObjRef) — [Missing <param name="or"/> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(Rhino.DocObjects.ObjRef)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.AssignTo(Rhino.DocObjects.ObjRef)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_AssignTo.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`

Begins a change or batch of changes. It may also make a copy of the content state allowing EndChange() to send an event with the old and new contents. Calls to this method are counted; you must call EndChange() once for every call to BeginChange(). Note: If Changed() was called between the calls to BeginChange() and EndChange(), the last call to EndChange() may cause the ContentChanged event to be sent.

**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — the change context. If this is ChangeContexts.UI, ChangeContexts.Program,ChangeContexts.Drop or ChangeContexts.Tree, the content will be copied. EndChange() will then send the copy as 'old' in the OnContentChanged event. EndChange()ContentChanged

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginChange.htm)

#### `public void BeginCreateDynamicFields(bool automatic)`

Automatic Dynamic Field support. Dynamic fields are typically created in the constructor of RenderContent and they are therefore created automatically whenever the content is created. However, some advanced users require the fields to be created in response to some user action which occurs much later. This creates the problem that the fields do not exist by default and therefore cannot be loaded when the document is loaded. These methods are provided to solve that problem by making it possible to automatically create the dynamic fields on loading if they don't already exist. Dynamic fields that have this auto-create-on-load behavior are referred to as automatic dynamic fields. Dynamic fields that do not require the advanced automatic feature can still be created by using these methods (recommended), or they can be created manually (legacy usage). You must call this before creating any dynamic fields. Calls to this method are counted; you must call EndCreateDynamicFields() once for every call to BeginCreateDynamicFields().

**Parameters:**
- `automatic` (System.Boolean) — automatic specifies if the dynamic fields are automatic. If so, they will be created automatically during loading of the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginCreateDynamicFields.htm)

#### `public void BindParameterToField(string parameterName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField.htm)

#### `public void BindParameterToField(string parameterName, string childSlotName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField_1.htm)

#### `protected virtual uint CalculateRenderHash(ulong rcrcFlags)`

Override this method to calculate the render hash of the state that affects how the content is rendered. Does not include children or perform any caching. Render hash values are now automatically cached by the content framework and you do not have to worry about caching. You also do not have to worry about iterating into children. This method is now only called internally by the framework, use the RenderHash property to get the current hash value.

**Parameters:**
- `rcrcFlags` (System.UInt64) — [Missing <param name="rcrcFlags"/> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CalculateRenderHash.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool ChangeChild(RenderContent oldContent, RenderContent newContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — [Missing <param name="oldContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]
- `newContent` (Rhino.Render.RenderContent) — [Missing <param name="newContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChangeChild.htm)

#### `public double ChildSlotAmount(string childSlotName)`

Gets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Double` — The requested amount value. Values are typically from 0.0 to 100.0

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotAmount.htm)

#### `public string ChildSlotNameFromParamName(string paramName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `paramName` (System.String) — The name of a parameter field. Since child textures will usually correspond with some parameter (they generally either replace or modify a parameter over UV space) these functions are used to specify which parameter corresponded with child slot. If there is no correspondence, return the empty string.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotNameFromParamName.htm)

#### `public bool ChildSlotOn(string childSlotName)`

Gets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotOn.htm)

#### `public PhysicallyBasedMaterial ConvertToPhysicallyBased(RenderTexture.TextureGeneration tg)`

Returns a material that is the best approximation of the original, but as a physically based material.

**Parameters:**
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — Determines whether simulated textures will be generated as files.

**Returns:** `PhysicallyBasedMaterial` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.ConvertToPhysicallyBased(Rhino.Render.RenderTexture.TextureGeneration)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_ConvertToPhysicallyBased.htm)

#### `public virtual void ConvertUnits(UnitSystem from, UnitSystem to)`

Modify your content so that it is converted from meters into the units of the unit system. No need to call the base class when you override this, and no need to recurse into children.

**Parameters:**
- `from` (Rhino.UnitSystem) — [Missing <param name="from"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]
- `to` (Rhino.UnitSystem) — [Missing <param name="to"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ConvertUnits.htm)

#### `[ObsoleteAttribute("Use CreateBasicMaterial(DocObjects.Material material, RhinoDoc doc) instead")] public static RenderMaterial CreateBasicMaterial(Material material)`

Constructs a new basic material from a Material.

**Parameters:**
- `material` (Rhino.DocObjects.Material) — (optional)The material to create the basic material from.

**Returns:** `RenderMaterial` — A new basic material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_CreateBasicMaterial.htm)

#### `public static RenderMaterial CreateBasicMaterial(Material material, RhinoDoc doc)`



**Parameters:**
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.Render.RenderMaterial.CreateBasicMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.RenderMaterial.CreateBasicMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc)"]

**Returns:** `RenderMaterial` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.CreateBasicMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_CreateBasicMaterial_1.htm)

#### `public bool CreateDynamicField(string internalName, string localName, string englishName, Object value, Object minValue, Object maxValue, int sectionId)`

Create a dynamic field with an initial value and min and max limits.

**Parameters:**
- `internalName` (System.String) — is the internal name of the field. Not localized.
- `localName` (System.String) — is the localized user-friendly name of the field.
- `englishName` (System.String) — is the English user-friendly name of the field.
- `value` (System.Object) — is the initial value of the field.
- `minValue` (System.Object) — is the minimum value of the field. Must be the same type as vValue.
- `maxValue` (System.Object) — is the maximum value of the field. Must be the same type as vValue.
- `sectionId` (System.Int32) — is used for filtering fields between sections. Zero if not needed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CreateDynamicField(System.String,System.String,System.String,System.Object,System.Object,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CreateDynamicField.htm)

#### `public static RenderMaterial CreateImportedMaterial(Material material, RhinoDoc doc, bool reference)`



**Parameters:**
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.Render.RenderMaterial.CreateImportedMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc,System.Boolean)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.RenderMaterial.CreateImportedMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.Render.RenderMaterial.CreateImportedMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc,System.Boolean)"]

**Returns:** `RenderMaterial` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.CreateImportedMaterial(Rhino.DocObjects.Material,Rhino.RhinoDoc,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_CreateImportedMaterial.htm)

#### `public void DeleteAllChildren(RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteAllChildren(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteAllChildren.htm)

#### `public bool DeleteChild(string childSlotName, RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteChild.htm)

#### `public void Dispose()`

(Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Dispose

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose_1.htm)

#### `public virtual bool DynamicIcon(Size size, out Bitmap bitmap, DynamicIconUsage usage)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `usage` (Rhino.Render.DynamicIconUsage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DynamicIcon.htm)

#### `public RenderContent Edit()`

This method allows a render content hierarchy to be edited using a modal (AKA 'pop-up') editor. If the original render content is in a document, it will remain there, and the edited one will be 'free-floating'. Therefore it is the caller's responsibility to do any replacement in the document if required. The returned new content will be owned by the caller.

**Returns:** `RenderContent` — Returns an edited version of the content hierarchy if successful, else null. The method always edits the entire hierarchy. It places a copy of the hierarchy in the editor and selects the copied item that corresponds to this one. Therefore, editing a child will return a top-level render content, not a child.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Edit.htm)

#### `public void EndChange()`

Ends a change or batch of changes. Calls to this method are counted; you must call this method once for every call to BeginChange(RenderContent.ChangeContexts). Note: If BeginChange(RenderContent.ChangeContexts) was called with ChangeContexts.UI, ChangeContexts.Program, ChangeContexts.Drop or ChangeContexts.UI.Tree and Changed() was called between the calls to BeginChange(RenderContent.ChangeContexts) and EndChange(), the last call to EndChange() will raise the ContentChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndChange.htm)

#### `public void EndCreateDynamicFields()`

You must call this after creating dynamic fields. Calls to this method are counted; you must call BeginCreateDynamicFields() once for every call to EndCreateDynamicFields().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndCreateDynamicFields.htm)

#### `public ContentFactory Factory()`

(Inherited from RenderContent.)

**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Factory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Factory.htm)

#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Finalize.htm)

#### `public RenderContent FindChild(string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FindChild.htm)

#### `public RenderContent ForDisplay()`

The only place a single proxy can be displayed is in the New Content Control main thumbnail. All other attempts to use a single proxy in a UI require it to be replaced with the corresponding multi proxy. Single proxies override this to find the corresponding multi proxy.

**Returns:** `RenderContent` — The cotnent.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ForDisplay.htm)

#### `public static RenderMaterial FromMaterial(Material material, RhinoDoc doc)`

Constructs a new material from a Material.

**Parameters:**
- `material` (Rhino.DocObjects.Material) — (optional)The material to create the material from.
- `doc` (Rhino.RhinoDoc) — The document to associate this material with.

**Returns:** `RenderMaterial` — A new material - either a "Custom" material or a "Physically Based" material depending on the return value of material.IsPhysicallyBased.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_FromMaterial.htm)

#### `public virtual Object GetChildSlotParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to specify additional functionality for automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call GetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetChildSlotParameter.htm)

#### `public string[] GetEmbeddedFilesList()`

(Inherited from RenderContent.)

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetEmbeddedFilesList"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetEmbeddedFilesList.htm)

#### `public Object GetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetExtraRequirementParameter.htm)

#### `public virtual Object GetParameter(string parameterName)`

Query the content instance for the value of a given named parameter. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — Name of the parameter

**Returns:** `Object` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetParameter.htm)

#### `public double GetTextureAmountFromUsage(RenderMaterial.StandardChildSlots slot)`



**Parameters:**
- `slot` (Rhino.Render.RenderMaterial.StandardChildSlots) — [Missing <param name="slot"/> documentation for "M:Rhino.Render.RenderMaterial.GetTextureAmountFromUsage(Rhino.Render.RenderMaterial.StandardChildSlots)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.GetTextureAmountFromUsage(Rhino.Render.RenderMaterial.StandardChildSlots)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_GetTextureAmountFromUsage.htm)

#### `public RenderTexture GetTextureFromUsage(RenderMaterial.StandardChildSlots slot)`



**Parameters:**
- `slot` (Rhino.Render.RenderMaterial.StandardChildSlots) — [Missing <param name="slot"/> documentation for "M:Rhino.Render.RenderMaterial.GetTextureFromUsage(Rhino.Render.RenderMaterial.StandardChildSlots)"]

**Returns:** `RenderTexture` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.GetTextureFromUsage(Rhino.Render.RenderMaterial.StandardChildSlots)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_GetTextureFromUsage.htm)

#### `public bool GetTextureOnFromUsage(RenderMaterial.StandardChildSlots slot)`



**Parameters:**
- `slot` (Rhino.Render.RenderMaterial.StandardChildSlots) — [Missing <param name="slot"/> documentation for "M:Rhino.Render.RenderMaterial.GetTextureOnFromUsage(Rhino.Render.RenderMaterial.StandardChildSlots)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.GetTextureOnFromUsage(Rhino.Render.RenderMaterial.StandardChildSlots)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_GetTextureOnFromUsage.htm)

#### `public virtual ulong GetUiHash()`

This allows a content to have more than one UI for the same content type.

**Returns:** `UInt64` — Default is zero and is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUiHash.htm)

#### `public bool GetUnderlyingInstances(ref RenderContentCollection collection)`

(Inherited from RenderContent.)

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUnderlyingInstances.htm)

#### `public bool HandleTexturedValue<T>(string slotname, TexturedValue<T> tc)`

Handle a textured content field. Values will be read into an instance of TexturedColor

**Parameters:**
- `slotname` (System.String) — [Missing <param name="slotname"/> documentation for "M:Rhino.Render.RenderMaterial.HandleTexturedValue``1(System.String,Rhino.Render.TexturedValue{``0})"]
- `tc` (Rhino.Render.TexturedValue<T>) — [Missing <param name="tc"/> documentation for "M:Rhino.Render.RenderMaterial.HandleTexturedValue``1(System.String,Rhino.Render.TexturedValue{``0})"]

**Returns:** `Boolean` — true if reading the base value succeeded

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_HandleTexturedValue__1.htm)

#### `public virtual bool Icon(Size size, out Bitmap bitmap)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Icon.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public bool Initialize()`

Obsolete. (Inherited from RenderContent.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Initialize.htm)

#### `public virtual bool IsCompatible(Guid renderEngineId)`

(Inherited from RenderContent.)

**Parameters:**
- `renderEngineId` (System.Guid) — [Missing <param name="renderEngineId"/> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsCompatible.htm)

#### `public virtual bool IsContentTypeAcceptableAsChild(Guid type, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `type` (System.Guid) — [Missing <param name="type"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsContentTypeAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(ContentFactory factory, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `factory` (Rhino.Render.DataSources.ContentFactory) — [Missing <param name="factory"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(Guid kindId, string factoryKind, string childSlotName)`

Override this method to restrict the type of acceptable child content. The default implementation of this method just returns true.

**Parameters:**
- `kindId` (System.Guid) — [Missing <param name="kindId"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `factoryKind` (System.String) — [Missing <param name="factoryKind"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]

**Returns:** `Boolean` — Return true only if content with the specified kindId can be accepted as a child in the specified child slot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild_1.htm)

#### `public bool IsReference()`

Query whether or not the content or any of its ancestors is a reference content.

**Returns:** `Boolean` — true if the content is a reference, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsReference.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public bool IsRenderHashCached()`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Returns:** `Boolean` — bool

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsRenderHashCached.htm)

#### `public RenderContent MakeCopy()`

Create a copy of the render content. All content is the same, except for the instance Id.

**Returns:** `RenderContent` — The new RenderContent

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeCopy.htm)

#### `public RenderContent MakeGroupInstance()`

Create an 'instance' of the content hierarchy and group the new hierarchy with this hierarchy. If the instance is subsequently attached to the same document, the state of all members of the group will be kept synchronized. With the exception of the instance ids, all state is exactly preserved in the new instance hierarchy. \note The grouping will have no effect until the new instance is attached to the same document.

**Returns:** `RenderContent` — A grouped instance of the content hierarchy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeGroupInstance.htm)

#### `public virtual RenderContent.MatchDataResult MatchData(RenderContent oldContent)`

Implement to transfer data from another content to this content during creation.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — An old content object from which the implementation may harvest data.

**Returns:** `RenderContent.MatchDataResult` — Information about how much data was matched.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MatchData.htm)

#### `protected void ModifyRenderContentStyles(RenderContentStyles stylesToAdd, RenderContentStyles stylesToRemove)`

ModifyRenderContentStyles

**Parameters:**
- `stylesToAdd` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToAdd"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]
- `stylesToRemove` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToRemove"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ModifyRenderContentStyles.htm)

#### `public PreviewSceneServer NewPreviewSceneServer(SceneServerData ssd)`

Gets the PreviewSceneServer of the content

**Parameters:**
- `ssd` (Rhino.Render.SceneServerData) — SceneServerData

**Returns:** `PreviewSceneServer` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.NewPreviewSceneServer(Rhino.Render.SceneServerData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_NewPreviewSceneServer.htm)

#### `protected virtual void OnAddUserInterfaceSections()`

Override to provide UI sections to display in the editor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnAddUserInterfaceSections.htm)

#### `protected virtual bool OnGetDefaultsInteractive()`

Override this method to prompt user for information necessary to create a new content object. For example, if you are created a textured material you may prompt the user for a bitmap file name prior to creating the textured material.

**Returns:** `Boolean` — If true is returned the content is created otherwise the creation is aborted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnGetDefaultsInteractive.htm)

#### `protected virtual void OnMakeCopy(ref RenderContent newContent)`

Override this function to supplement the standard copying behavour for your RenderContent.

**Parameters:**
- `newContent` (Rhino.Render.RenderContent) — Is the content that will be returned from MakeCopy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnMakeCopy.htm)

#### `public bool OpenInEditor()`

Call this method to open the content in the relevant thumbnail editor and select it for editing by the user. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInEditor.htm)

#### `[ObsoleteAttribute("Obsolete, use Edit a version that returns a RenderContent", false)] public bool OpenInModalEditor()`

Call this method to open the content in the a modal version of the editor. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInModalEditor.htm)

#### `public string ParamNameFromChildSlotName(string childSlotName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `childSlotName` (System.String) — The named of the child slot to receive the parameter name for.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ParamNameFromChildSlotName.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames, LinearWorkflow lw)`

As RenderHash, but ignore parameter names given. Use this version of the function to calculate a render hash when you have the linear workflow information and you are not running on the main thread. Access to LinearWorkflow data requires document access. CrcRenderHashFlags.ExcludeLinearWorkflow must be specified.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string
- `lw` (Rhino.Render.LinearWorkflow) — Linear Workflow to use for CRC

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_1.htm)

#### `public uint RenderHashExclude(TextureRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.TextureRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_2.htm)

#### `public bool Replace(RenderContent newcontent)`

(Inherited from RenderContent.)

**Parameters:**
- `newcontent` (Rhino.Render.RenderContent) — [Missing <param name="newcontent"/> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Replace.htm)

#### `public bool SetChild(RenderContent renderContent, string childSlotName)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild.htm)

#### `[ObsoleteAttribute("Use SetChild without ChangeContexts and Begin/EndChange")] public bool SetChild(RenderContent renderContent, string childSlotName, RenderContent.ChangeContexts changeContexts)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.SetChild(Rhino.Render.RenderContent,System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild_1.htm)

#### `public void SetChildSlotAmount(string childSlotName, double amount, RenderContent.ChangeContexts cc)`

Sets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `amount` (System.Double) — The texture amount. Values are typically from 0.0 to 100.0
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — The context of the change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotAmount.htm)

#### `public void SetChildSlotOn(string childSlotName, bool bOn, RenderContent.ChangeContexts cc)`

Sets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `bOn` (System.Boolean) — Value for the on-ness property.
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Context of the change

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotOn.htm)

#### `public virtual bool SetChildSlotParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to support values being set from automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call SetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotParameter.htm)

#### `public bool SetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetExtraRequirementParameter.htm)

#### `public void SetIsRenderHashRecursive(bool recursive)`

By default, RenderHash() recurses into children when computing the render CRC. However, some applications may require children to be excluded from the render CRC calculation. Call this method to enable or disable recursing into children. see RenderHash

**Parameters:**
- `recursive` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetIsRenderHashRecursive.htm)

#### `public void SetName(string name, bool renameEvents, bool ensureNameUnique)`

Set instance name for this content

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `renameEvents` (System.Boolean) — [Missing <param name="renameEvents"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `ensureNameUnique` (System.Boolean) — [Missing <param name="ensureNameUnique"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetName.htm)

#### `public virtual bool SetParameter(string parameterName, Object value)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter.htm)

#### `[ObsoleteAttribute("Use SetParameter without ChangeContexts and Begin/EndChange")] public virtual bool SetParameter(string parameterName, Object value, RenderContent.ChangeContexts changeContext)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter_1.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public void SetRenderHash(uint hash)`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Parameters:**
- `hash` (System.UInt32)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetRenderHash.htm)

#### `[ObsoleteAttribute("Incorrect spelling - use SimulatedMaterial instead, and don't override this virtual function")] public virtual Material SimulateMaterial(bool isForDataOnly)`

Call this function to receive the simulation for a RenderMaterial used by the display and other rendering engines.

**Parameters:**
- `isForDataOnly` (System.Boolean) — Called when only asking for a hash - don't write any textures to the disk - just provide the filenames they will get.

**Returns:** `Material` — The simulation of the render material

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_SimulateMaterial_2.htm)

#### `public virtual void SimulateMaterial(ref Material simulation, bool isForDataOnly)`

Override this function to provide a Rhino.DocObjects.Material definition for this material to be used by other rendering engines including the display.

**Parameters:**
- `simulation` (Rhino.DocObjects.Material) — Set the properties of the input basic material to provide the simulation for this material.
- `isForDataOnly` (System.Boolean) — Called when only asking for a hash - don't write any textures to the disk - just provide the filenames they will get.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_SimulateMaterial_1.htm)

#### `public virtual void SimulateMaterial(ref Material simulation, RenderTexture.TextureGeneration tg)`

Override this function to provide a Rhino.DocObjects.Material definition for this material to be used by other rendering engines including the display.

**Parameters:**
- `simulation` (Rhino.DocObjects.Material) — Set the properties of the input basic material to provide the simulation for this material.
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — See RenderTexture.TextureGeneration.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_SimulateMaterial.htm)

#### `public Material SimulatedMaterial(RenderTexture.TextureGeneration tg)`



**Parameters:**
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — [Missing <param name="tg"/> documentation for "M:Rhino.Render.RenderMaterial.SimulatedMaterial(Rhino.Render.RenderTexture.TextureGeneration)"]

**Returns:** `Material` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.SimulatedMaterial(Rhino.Render.RenderTexture.TextureGeneration)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_SimulatedMaterial.htm)

#### `public static RenderMaterial.StandardChildSlots SlotFromTextureType(TextureType tt)`



**Parameters:**
- `tt` (Rhino.DocObjects.TextureType) — [Missing <param name="tt"/> documentation for "M:Rhino.Render.RenderMaterial.SlotFromTextureType(Rhino.DocObjects.TextureType)"]

**Returns:** `RenderMaterial.StandardChildSlots` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.SlotFromTextureType(Rhino.DocObjects.TextureType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_SlotFromTextureType.htm)

#### `public bool SmartUngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. If any content in the same document is left alone in the group, that content is also ungrouped. Records undo and sends events OnContentChanged and OnContentGroupIdChanged. \note This method is designed to be called from a content UI and is intended for RDK internal use but may be used as an expert user override.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SmartUngroupRecursive.htm)

#### `public virtual string TextureChildSlotName(RenderMaterial.StandardChildSlots slot)`

Override this function to provide information about which texture is used for the standard (ie - defined in ON_Texture) texture channels.

**Parameters:**
- `slot` (Rhino.Render.RenderMaterial.StandardChildSlots) — An valid slot.

**Returns:** `String` — The texture used for the channel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_TextureChildSlotName.htm)

#### `public static TextureType TextureTypeFromSlot(RenderMaterial.StandardChildSlots slot)`



**Parameters:**
- `slot` (Rhino.Render.RenderMaterial.StandardChildSlots) — [Missing <param name="slot"/> documentation for "M:Rhino.Render.RenderMaterial.TextureTypeFromSlot(Rhino.Render.RenderMaterial.StandardChildSlots)"]

**Returns:** `TextureType` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterial.TextureTypeFromSlot(Rhino.Render.RenderMaterial.StandardChildSlots)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterial_TextureTypeFromSlot.htm)

#### `public bool Ungroup()`

Remove this content from any instance group it may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if content was ungrouped, \e false if it was not part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Ungroup.htm)

#### `public bool UngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UngroupRecursive.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public void Uninitialize()`

Obsolete. (Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Uninitialize.htm)

#### `public int UseCount()`

UseCount returns how many times the content is used

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.UseCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UseCount.htm)

#### `public virtual bool VirtualIcon(Size size, out Bitmap bitmap)`

Icon to display in the content browser, this bitmap needs to be valid for the life of this content object, the content object that returns the bitmap is responsible for disposing of the bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Requested icon size
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.VirtualIcon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — Return Icon to display in the content browser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_VirtualIcon.htm)

### Properties
- `CanBeEdited` (Boolean, get) — Determines if the content can be edited.
- `Category` (String, get) — Category for this content.
- `ChildSlotDisplayName` (String, get) — Returns the localized display name of the child slot name
- `ChildSlotName` (String, get/set) — (Inherited from RenderContent.)
- `CppPointer` (IntPtr, get) — (Inherited from RenderContent.)
- `DefaultPreviewBackgroundType` (RenderMaterial.PreviewBackgroundType, get/set) — Set or get the default scene background for the image that appears in preview panes
- `DefaultPreviewGeometryType` (RenderMaterial.PreviewGeometryType, get/set) — Set or get the default geometry that appears in preview panes
- `DefaultPreviewSize` (Double, get/set) — The default preview geometry size
- `DisplayName` (String, get) — Display name for this content.
- `Document` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `DocumentAssoc` (RhinoDoc, get/set) — If this render content is associated with a document in any way, the document will be returned. This includes copies of render contents that were attached to a document when the copy was made. Otherwise returns null.
- `DocumentOwner` (RhinoDoc, get) — If this render content is owned by a document, the document will be returned. This is the same as getting the document the render content is attached to. Otherwise returns null.
- `DocumentRegistered` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `Fields` (FieldDictionary, get) — Rhino.Render.Fields FieldDictionary which provides access to setting and retrieving field values.
- `Filename` (String, get/set) — If the content is file based, this function can be overridden to deal with setting/getting the filename. Corresponds to IRhRdkFileBasedContent in the C++ SDK
- `FilesToEmbed` (IEnumerable<String>, get) — A string array of full paths to files used by the content that may be embedded in .3dm files and library files (.rmtl, .renv, .rtex). The default implementation returns an empty string list. Override this to return the file name or file names used by your content. This is typically used by textures that reference files containing the texture imagery.
- `FirstChild` (RenderContent, get) — Return First child of this content or nullptr if none.
- `GemMaterialGuid` (Guid, get) — 
- `GlassMaterialGuid` (Guid, get) — 
- `GroupId` (Guid, get/set) — Group ID of the content
- `Hidden` (Boolean, get/set) — Gets or sets the render content's 'hidden' state. This feature only works for top-level render contents because it hides the entire hierarchy. It is normally used for 'implementation detail' render contents. For expert use only. Hidden render contents are never shown in the UI, with the exception of the Object (or Layer) Material Properties UI which always shows whatever is assigned to the object (or layer). In the Object (or Layer) Material Properties UI, if the user drops down the list, hidden render contents are not listed. Hidden render contents, being part of the document content list, will be listed by any scripts or other code that iterates over the document render content list. It is recommended that you set IsHidden once when you create your render content and leave it on to prevent flicker or slow performance.
- `Id` (Guid, get/set) — Instance identifier for this content.
- `IsDefaultInstance` (Boolean, get) — Checks if render content is default instance.
- `IsHiddenByAutoDelete` (Boolean, get) — Contents can be created as 'auto-delete' by certain commands such as 'Picture'. These contents are automatically hidden from the user when the associated Rhino object is deleted. They are later deleted when the document is saved.
- `IsLocked` (Boolean, get/set) — Set this property to true prior to adding content to the document to lock the content browser editing UI methods. Setting this to true will keep the browser from allowing things like deleting, renaming or changing content. This is useful for custom child content that you want to be editable but persistent. Setting this after adding content to the document will cause an exception to be thrown.
- `MetalMaterialGuid` (Guid, get) — 
- `Name` (String, get/set) — Instance 'raw' name for this content.
- `NextSibling` (RenderContent, get) — Return First sibling of this content or nullptr if none.
- `Notes` (String, get/set) — Notes for this content.
- `PaintMaterialGuid` (Guid, get) — 
- `Parent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `PictureMaterialGuid` (Guid, get) — 
- `PlasterMaterialGuid` (Guid, get) — 
- `PlasticMaterialGuid` (Guid, get) — 
- `ProxyType` (ProxyTypes, get) — Gets the proxy type of the render content
- `RenderHash` (UInt32, get) — Render hash for the content hierarchy. It iterates children and includes a caching mechanism which means the hash value can be retrieved quickly if it hasn't changed. The cache is invalidated when Changed() is called. You can override the CalculateRenderHash(UInt64) method to provide a custom hash value.
- `SmellsLikeGem` (Boolean, get) — 
- `SmellsLikeGlass` (Boolean, get) — 
- `SmellsLikeMetal` (Boolean, get) — 
- `SmellsLikePaint` (Boolean, get) — 
- `SmellsLikePlaster` (Boolean, get) — 
- `SmellsLikePlastic` (Boolean, get) — 
- `SmellsLikeTexturedGem` (Boolean, get) — 
- `SmellsLikeTexturedGlass` (Boolean, get) — 
- `SmellsLikeTexturedMetal` (Boolean, get) — 
- `SmellsLikeTexturedPaint` (Boolean, get) — 
- `SmellsLikeTexturedPlaster` (Boolean, get) — 
- `SmellsLikeTexturedPlastic` (Boolean, get) — 
- `Styles` (RenderContentStyles, get) — (Inherited from RenderContent.)
- `Tags` (String, get/set) — Tags for this content.
- `TopLevel` (Boolean, get) — Returns true if this content has no parent, false if it is the child of another content.
- `TopLevelParent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `TypeDescription` (String, get) — Description for your content type. ie. "Procedural checker pattern"
- `TypeId` (Guid, get) — Type identifier for this content
- `TypeName` (String, get) — Name for your content type. ie. "My .net Texture"
- `Xml` (String, get) — (Inherited from RenderContent.)

## RenderMaterial.AssignToBlockChoices (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderMaterial.AssignToBlockChoices"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial_AssignToBlockChoices.htm)

### Values
- `Always` = `0` — Always assign to blocks recursively.
- `Never` = `1` — Never assign to blocks recursively.
- `Ask` = `2` — Ask the user what to do.

## RenderMaterial.AssignToSubFaceChoices (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderMaterial.AssignToSubFaceChoices"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial_AssignToSubFaceChoices.htm)

### Values
- `Keep` = `0` — Keep any existing sub-face assignments.
- `Remove` = `1` — Remove any existing sub-face assignments first.
- `Ask` = `2` — Ask the user what to do.

## RenderMaterial.BasicMaterialParameterNames (class)

Parameter names for use in GetNamedParameter and SetNamedParameter with basic materials.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial_BasicMaterialParameterNames.htm)

### Constructors
- `public BasicMaterialParameterNames()` — Initializes a new instance of the RenderMaterial.BasicMaterialParameterNames class

### Properties
- `Ambient` (String, get) — 
- `Diffuse` (String, get) — 
- `DisableLighting` (String, get) — 
- `Emission` (String, get) — 
- `FlamingoLibrary` (String, get) — 
- `Ior` (String, get) — 
- `Reflectivity` (String, get) — 
- `ReflectivityColor` (String, get) — 
- `Shine` (String, get) — 
- `Specular` (String, get) — 
- `Transparency` (String, get) — 
- `TransparencyColor` (String, get) — 

## RenderMaterial.PreviewBackgroundType (enum)

The default scene background for the image that appears in preview panes

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial_PreviewBackgroundType.htm)

### Values
- `None` = `1`
- `Checkered` = `2`
- `Scene` = `4`

## RenderMaterial.PreviewGeometryType (enum)

Geometry that appears in preview panes

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial_PreviewGeometryType.htm)

### Values
- `Cone` = `2`
- `Cube` = `1`
- `Plane` = `5`
- `Pyramid` = `3`
- `Sphere` = `0`
- `Torus` = `4`
- `Scene` = `7`

## RenderMaterial.StandardChildSlots (enum)

Defines enumerated constant values for use in TextureChildSlotName(RenderMaterial.StandardChildSlots) method. NOTE WELL - these values cannot be changed. https://mcneel.myjetbrains.com/youtrack/issue/RH-57752

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterial_StandardChildSlots.htm)

### Values
- `None` = `0` — Corresponds to CRhRdkMaterial::ChildSlotUsage::None.
- `Diffuse` = `100` — Corresponds to CRhRdkMaterial::ChildSlotUsage::Diffuse.
- `Transparency` = `101` — Corresponds to CRhRdkMaterial::ChildSlotUsage::Transparency
- `Bump` = `102` — Corresponds to CRhRdkMaterial::ChildSlotUsage::Bump
- `Environment` = `103` — Corresponds to CRhRdkMaterial::ChildSlotUsage::Environment
- `PbrBaseColor` = `100` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_base_color
- `PbrSubsurface` = `104` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_subsurface
- `PbrSubSurfaceScattering` = `105` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_subsurface_scattering_color
- `PbrSubsurfaceScatteringRadius` = `106` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_subsurface_scattering_radius
- `PbrMetallic` = `107` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_metallic
- `PbrSpecular` = `108` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_specular
- `PbrSpecularTint` = `109` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_specular_tint
- `PbrRoughness` = `110` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_roughness
- `PbrAnisotropic` = `111` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_anisotropic
- `PbrAnisotropicRotation` = `112` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_anisotropic_rotation
- `PbrSheen` = `113` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_sheen
- `PbrSheenTint` = `114` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_sheen_tint
- `PbrClearcoat` = `115` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_clearcoat
- `PbrClearcoatRoughness` = `116` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_clearcoat_roughness
- `PbrOpacityIor` = `117` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_opacity_ior
- `PbrOpacity` = `101` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_opacity
- `PbrOpacityRoughness` = `118` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_opacity_roughness
- `PbrEmission` = `119` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_emission
- `PbrAmbientOcclusion` = `120` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_ambient_occlusion
- `PbrDisplacement` = `121` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_smudge
- `PbrClearcoatBump` = `122` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_clearcoat_roughness
- `PbrAlpha` = `123` — Corresponds to CRhRdkMaterial::ChildSlotUsage::PBR_alpha

## RenderMaterialTable (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderMaterialTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderMaterialTable.htm)

### Methods
#### `public bool Add(RenderMaterial c)`



**Parameters:**
- `c` (Rhino.Render.RenderMaterial) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderMaterialTable.Add(Rhino.Render.RenderMaterial)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterialTable.Add(Rhino.Render.RenderMaterial)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterialTable_Add.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderMaterialTable.BeginChange(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterialTable_BeginChange.htm)

#### `public void EndChange()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterialTable_EndChange.htm)

#### `public IEnumerator<RenderMaterial> GetEnumerator()`



**Returns:** `IEnumerator<RenderMaterial>` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterialTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterialTable_GetEnumerator.htm)

#### `public bool Remove(RenderMaterial c)`



**Parameters:**
- `c` (Rhino.Render.RenderMaterial) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderMaterialTable.Remove(Rhino.Render.RenderMaterial)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderMaterialTable.Remove(Rhino.Render.RenderMaterial)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderMaterialTable_Remove.htm)

### Properties
- `Count` (Int32, get) — 
- `Item` (RenderMaterial, get) — 

## RenderPanelType (enum)

Contains the custom user interfaces that may be provided

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPanelType.htm)

### Values
- `RenderWindow` = `0` — A custom control panel added to the render output window.

## RenderPanels (class)

This class is used to extend the standard Render user interface

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPanels.htm)

### Methods
#### `public static Object FromRenderSessionId(PlugIn plugIn, Type panelType, Guid renderSessionId)`

Get the instance of a render panel associated with a specific render session, this is useful when it is necessary to update a control from a RenderPipeline

**Parameters:**
- `plugIn` (Rhino.PlugIns.PlugIn) — The plug-in that registered the custom user interface
- `panelType` (System.Type) — The type of panel to return
- `renderSessionId` (System.Guid) — The RenderSessionId of a specific render session.

**Returns:** `Object` — Returns the custom panel object if found; otherwise null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPanels_FromRenderSessionId.htm)

#### `public void RegisterPanel(PlugIn plugin, RenderPanelType renderPanelType, Type panelType, Guid renderEngineId, string caption, bool alwaysShow, bool initialShow)`

Register custom render user interface with Rhino. This should only be done in RegisterRenderPanels(RenderPanels). Panels registered after RegisterRenderPanels(RenderPanels) is called will be ignored.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — The plug-in providing the custom user interface
- `renderPanelType` (Rhino.Render.RenderPanelType) — See RenderPanelType for supported user interface types.
- `panelType` (System.Type) — The type of object to be created and added to the render container.
- `renderEngineId` (System.Guid) — The render engine id allowing the UI to be shown
- `caption` (System.String) — The caption for the custom user interface.
- `alwaysShow` (System.Boolean) — If true the custom user interface will always be visible, if false then it may be hidden or shown as requested by the user.
- `initialShow` (System.Boolean) — Initial visibility state of the custom user interface control.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPanels_RegisterPanel.htm)

#### `[ObsoleteAttribute("Use RegisterPanel(PlugIn plugin, RenderPanelType renderPanelType, Type panelType, Guid renderEngineId, string caption, bool alwaysShow, bool initialShow)")] public void RegisterPanel(PlugIn plugin, RenderPanelType renderPanelType, Type panelType, string caption, bool alwaysShow, bool initialShow)`

Register custom render user interface with Rhino. This should only be done in RegisterRenderPanels(RenderPanels). Panels registered after RegisterRenderPanels(RenderPanels) is called will be ignored.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — The plug-in providing the custom user interface
- `renderPanelType` (Rhino.Render.RenderPanelType) — See RenderPanelType for supported user interface types.
- `panelType` (System.Type) — The type of object to be created and added to the render container.
- `caption` (System.String) — The caption for the custom user interface.
- `alwaysShow` (System.Boolean) — If true the custom user interface will always be visible, if false then it may be hidden or shown as requested by the user.
- `initialShow` (System.Boolean) — Initial visibility state of the custom user interface control.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPanels_RegisterPanel_1.htm)

## RenderPipeline (class)

Provides facilities to a render plug-in for integrating with the standard Rhino render window. Also adds helper functions for processing a render scene. This is the suggested class to use when integrating a renderer with Rhino and maintaining a "standard" user interface that users will expect.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPipeline.htm)

### Constructors
- `protected RenderPipeline(RhinoDoc doc, RunMode mode, PlugIn plugin, Size sizeRendering, string caption, RenderWindow.StandardChannels channels, bool reuseRenderWindow, bool clearLastRendering)` — Constructs a subclass of this object on the stack in your Rhino plug-in's Render() or RenderWindow() implementation.
- `protected RenderPipeline(RhinoDoc doc, RunMode mode, PlugIn plugin, Size sizeRendering, string caption, RenderWindow.StandardChannels channels, bool reuseRenderWindow, bool clearLastRendering, ref AsyncRenderContext aRC)` — Constructs a subclass of this object on the stack in your Rhino plug-in's Render() or RenderWindow() implementation. This constructor should be used when a non-blocking RenderWindow is required. Note that the asynchronous render context will not be used when mode is Scripted.

### Methods
#### `protected virtual bool AddLightToScene(LightObject light)`



**Parameters:**
- `light` (Rhino.DocObjects.LightObject) — [Missing <param name="light"/> documentation for "M:Rhino.Render.RenderPipeline.AddLightToScene(Rhino.DocObjects.LightObject)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.AddLightToScene(Rhino.DocObjects.LightObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_AddLightToScene.htm)

#### `protected virtual bool AddRenderMeshToScene(RhinoObject obj, Material material, Mesh mesh)`



**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.RenderPipeline.AddRenderMeshToScene(Rhino.DocObjects.RhinoObject,Rhino.DocObjects.Material,Rhino.Geometry.Mesh)"]
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.Render.RenderPipeline.AddRenderMeshToScene(Rhino.DocObjects.RhinoObject,Rhino.DocObjects.Material,Rhino.Geometry.Mesh)"]
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.Render.RenderPipeline.AddRenderMeshToScene(Rhino.DocObjects.RhinoObject,Rhino.DocObjects.Material,Rhino.Geometry.Mesh)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.AddRenderMeshToScene(Rhino.DocObjects.RhinoObject,Rhino.DocObjects.Material,Rhino.Geometry.Mesh)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_AddRenderMeshToScene.htm)

#### `public bool CloseWindow()`

Closes the render window associated with this render instance.

**Returns:** `Boolean` — Return true if successful or false if not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_CloseWindow.htm)

#### `public Result CommandResult()`



**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.CommandResult"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_CommandResult.htm)

#### `protected abstract bool ContinueModal()`

Frequently called during a rendering by the frame work in order to determine if the rendering should continue.

**Returns:** `Boolean` — Returns true if the rendering should continue.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_ContinueModal.htm)

#### `public void Dispose()`

Releases all resources used by the RenderPipeline

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the RenderPipeline and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_Finalize.htm)

#### `public RenderWindow GetRenderWindow()`

Get the RenderWindow associated with this RenderPipeline instance. This is virtual rather than abstract for V5 compat

**Returns:** `RenderWindow` — RenderWindow if one exists, null otherwise (i.e. rendering has already completed).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_GetRenderWindow.htm)

#### `public RenderWindow GetRenderWindow(bool withWireframeChannel)`

As GetRenderWindow(), but if withWireframeChannel is true the returned RenderWindow will have the channel added.

**Parameters:**
- `withWireframeChannel` (System.Boolean) — [Missing <param name="withWireframeChannel"/> documentation for "M:Rhino.Render.RenderPipeline.GetRenderWindow(System.Boolean)"]

**Returns:** `RenderWindow` — RenderWindow with wireframe channel enabled, or null if no RenderWindow can be found (i.e. rendering has completed already)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_GetRenderWindow_3.htm)

#### `public RenderWindow GetRenderWindow(bool withWireframeChannel, bool fromRenderViewSource)`

As GetRenderWindow(). The parameter withWireframeChannel controls whether the returned RenderWindow will have the channel added. The parameter fromRenderViewSource controls from where the RenderSize is queried.

**Parameters:**
- `withWireframeChannel` (System.Boolean) — true if the RenderWindow needs to have a wireframe channel.
- `fromRenderViewSource` (System.Boolean) — true if the RenderWindow size needs to be set from RenderViewSource size. false will use the active view.

**Returns:** `RenderWindow` — RenderWindow if one exists, null otherwise (i.e. rendering has already completed).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_GetRenderWindow_4.htm)

#### `public RenderWindow GetRenderWindow(ViewportInfo viewportInfo, bool fromRenderViewSource)`

As GetRenderWindow(). The parameter withWireframeChannel controls whether the returned RenderWindow will have the channel added. The parameter fromRenderViewSource controls from where the RenderSize is queried. The viewportInfo instance will be used to set up wireframe channel. This is necessary for rendering different view than is currently active in the viewport

**Parameters:**
- `viewportInfo` (Rhino.DocObjects.ViewportInfo) — ViewportInfo used to generate the wireframe channel for
- `fromRenderViewSource` (System.Boolean) — true if the RenderWindow size needs to be set from RenderViewSource size. false will use the active view.

**Returns:** `RenderWindow` — RenderWindow if one exists, null otherwise (i.e. rendering has already completed).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_GetRenderWindow_1.htm)

#### `public RenderWindow GetRenderWindow(ViewportInfo viewportInfo, bool fromRenderViewSource, Rectangle region)`

As GetRenderWindow(). The parameter withWireframeChannel controls whether the returned RenderWindow will have the channel added. The parameter fromRenderViewSource controls from where the RenderSize is queried. The viewportInfo instance will be used to set up wireframe channel. This is necessary for rendering different view than is currently active in the viewport

**Parameters:**
- `viewportInfo` (Rhino.DocObjects.ViewportInfo) — ViewportInfo used to generate the wireframe channel
- `fromRenderViewSource` (System.Boolean) — true if the RenderWindow size needs to be set from RenderViewSource size. false will use the active view.
- `region` (System.Drawing.Rectangle) — The region to render. Usually the same as the full render window but in the case of RenderWindow and RenderInWindow, it is the region the user selected in the viewport.

**Returns:** `RenderWindow` — RenderWindow if one exists, null otherwise (i.e. rendering has already completed).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_GetRenderWindow_2.htm)

#### `public RenderWindow GetRenderWindowFromRenderViewSource(bool fromRenderViewSource)`

Like GetRenderWindow(), but with the size for RenderWindow set from RenderViewSources if fromRenderViewSource is set to true

**Parameters:**
- `fromRenderViewSource` (System.Boolean) — true if

**Returns:** `RenderWindow` — RenderWindow if one exists, null otherwise (i.e. rendering has already completed).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_GetRenderWindowFromRenderViewSource.htm)

#### `protected virtual bool IgnoreRhinoObject(RhinoObject obj)`



**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.RenderPipeline.IgnoreRhinoObject(Rhino.DocObjects.RhinoObject)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.IgnoreRhinoObject(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_IgnoreRhinoObject.htm)

#### `public static string LocalizeRenderReturnCode(RenderPipeline.RenderReturnCode rc)`



**Parameters:**
- `rc` (Rhino.Render.RenderPipeline.RenderReturnCode) — [Missing <param name="rc"/> documentation for "M:Rhino.Render.RenderPipeline.LocalizeRenderReturnCode(Rhino.Render.RenderPipeline.RenderReturnCode)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.LocalizeRenderReturnCode(Rhino.Render.RenderPipeline.RenderReturnCode)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_LocalizeRenderReturnCode.htm)

#### `protected virtual bool NeedToProcessGeometryTable()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.NeedToProcessGeometryTable"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_NeedToProcessGeometryTable.htm)

#### `protected virtual bool NeedToProcessLightTable()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.NeedToProcessLightTable"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_NeedToProcessLightTable.htm)

#### `protected abstract bool OnRenderBegin()`

Called by the framework when it is time to start rendering, the render window will be created at this point and it is safe to start

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.OnRenderBegin"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_OnRenderBegin.htm)

#### `protected virtual bool OnRenderBeginQuiet(Size imageSize)`

Called by the framework when it is time to start rendering quietly, there is no user interface when rendering in this mode and the default post process effects will get applied to the scene when the rendering is complete.

**Parameters:**
- `imageSize` (System.Drawing.Size) — [Missing <param name="imageSize"/> documentation for "M:Rhino.Render.RenderPipeline.OnRenderBeginQuiet(System.Drawing.Size)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.OnRenderBeginQuiet(System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_OnRenderBeginQuiet.htm)

#### `protected abstract void OnRenderEnd(RenderEndEventArgs e)`

Called by the framework when the user closes the render window or clicks on the stop button in the render window.

**Parameters:**
- `e` (Rhino.Render.RenderEndEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.Render.RenderPipeline.OnRenderEnd(Rhino.Render.RenderEndEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_OnRenderEnd.htm)

#### `protected abstract bool OnRenderWindowBegin(RhinoView view, Rectangle rectangle)`



**Parameters:**
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.Render.RenderPipeline.OnRenderWindowBegin(Rhino.Display.RhinoView,System.Drawing.Rectangle)"]
- `rectangle` (System.Drawing.Rectangle) — [Missing <param name="rectangle"/> documentation for "M:Rhino.Render.RenderPipeline.OnRenderWindowBegin(Rhino.Display.RhinoView,System.Drawing.Rectangle)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.OnRenderWindowBegin(Rhino.Display.RhinoView,System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_OnRenderWindowBegin.htm)

#### `public virtual void PauseRendering()`

Implement to pause the current render session

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_PauseRendering.htm)

#### `public RenderPipeline.RenderReturnCode Render()`

Call this function to render the scene normally. The function returns when rendering is complete (or cancelled).

**Returns:** `RenderPipeline.RenderReturnCode` — A code that explains how rendering completed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_Render.htm)

#### `protected virtual bool RenderEnterModalLoop()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.RenderEnterModalLoop"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderEnterModalLoop.htm)

#### `protected virtual bool RenderExitModalLoop()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.RenderExitModalLoop"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderExitModalLoop.htm)

#### `protected virtual bool RenderPreCreateWindow()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.RenderPreCreateWindow"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderPreCreateWindow.htm)

#### `protected virtual bool RenderSceneWithNoMeshes()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.RenderSceneWithNoMeshes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderSceneWithNoMeshes.htm)

#### `[ObsoleteAttribute] public static Size RenderSize()`

Obsolete.

**Returns:** `Size` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.RenderSize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderSize.htm)

#### `public static Size RenderSize(RhinoDoc doc)`

Get the render size as specified in the ON_3dmRenderSettings. Will automatically return the correct size based on the ActiveView or custom settings.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.RenderPipeline.RenderSize(Rhino.RhinoDoc)"]

**Returns:** `Size` — The render size.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderSize_1.htm)

#### `public static Size RenderSize(RhinoDoc doc, bool fromRenderSources)`

Get the render size as specified in the ON_3dmRenderSettings, and from RenderSources when fromRenderSources is true.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.RenderPipeline.RenderSize(Rhino.RhinoDoc,System.Boolean)"]
- `fromRenderSources` (System.Boolean) — [Missing <param name="fromRenderSources"/> documentation for "M:Rhino.Render.RenderPipeline.RenderSize(Rhino.RhinoDoc,System.Boolean)"]

**Returns:** `Size` — The render size.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderSize_2.htm)

#### `public RenderPipeline.RenderReturnCode RenderWindow(RhinoView view, Rectangle rect, bool inWindow)`

Call this function to render the scene in a view window. The function returns when rendering is complete (or cancelled).

**Parameters:**
- `view` (Rhino.Display.RhinoView) — the view that the user selected a rectangle in.
- `rect` (System.Drawing.Rectangle) — rectangle that the user selected.
- `inWindow` (System.Boolean) — true to render directly into the view window.

**Returns:** `RenderPipeline.RenderReturnCode` — A code that explains how rendering completed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_RenderWindow.htm)

#### `public virtual void ResumeRendering()`

Implement to resume current render session

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_ResumeRendering.htm)

#### `public bool SaveImage(string fileName, bool saveAlpha)`

Saves the rendered image to a file. Does not prompt the user in any way.

**Parameters:**
- `fileName` (System.String) — Full path to the file name to save to.
- `saveAlpha` (System.Boolean) — Determines if alpha will be saved in files that support it (e.g., PNG).

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPipeline.SaveImage(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_SaveImage.htm)

#### `public void SetAsyncRenderContext(ref AsyncRenderContext aRC)`



**Parameters:**
- `aRC` (Rhino.Render.AsyncRenderContext) — [Missing <param name="aRC"/> documentation for "M:Rhino.Render.RenderPipeline.SetAsyncRenderContext(Rhino.Render.AsyncRenderContext@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_SetAsyncRenderContext.htm)

#### `public virtual bool SupportsPause()`

Override and return true if the renderer supports pausing

**Returns:** `Boolean` — true if pausing is supported, false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPipeline_SupportsPause.htm)

### Properties
- `ConfirmationSeconds` (Int32, set) — Sets the number of seconds that need to elapse during rendering before the user is asked if the rendered image should be saved.
- `PlugIn` (PlugIn, get) — 
- `RenderSessionId` (Guid, get) — Get the Id associated with this render session, this is useful when looking up Rhino.Render.RenderPanels.

## RenderPipeline.RenderReturnCode (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderPipeline.RenderReturnCode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPipeline_RenderReturnCode.htm)

### Values
- `Ok` = `0`
- `EmptyScene` = `1`
- `Cancel` = `2`
- `NoActiveView` = `3`
- `OnPreCreateWindow` = `4`
- `NoFrameWndPointer` = `5`
- `ErrorCreatingWindow` = `6`
- `ErrorStartingRender` = `7`
- `EnterModalLoop` = `8`
- `ExitModalLoop` = `9`
- `ExitRhino` = `10`
- `InternalError` = `11`

## RenderPlugInInfo (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderPlugInInfo"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPlugInInfo.htm)

### Properties
- `Name` (String, get/set) — 
- `PlugInId` (Guid, get/set) — 

## RenderPlugInList (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderPlugInList"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPlugInList.htm)

### Constructors
- `public RenderPlugInList()` — Initializes a new instance of the RenderPlugInList class

## RenderPrimitive (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderPrimitive"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPrimitive.htm)

### Methods
#### `public void Dispose()`

Releases all resources used by the RenderPrimitive

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_Dispose.htm)

#### `protected void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the RenderPrimitive and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_Finalize.htm)

#### `public Mesh Mesh()`

Returns the mesh associated with the object, this will mesh primitives and always return a mesh.

**Returns:** `Mesh` — [Missing <returns> documentation for "M:Rhino.Render.RenderPrimitive.Mesh"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_Mesh.htm)

#### `public bool TryGetBox(out Box box)`

Call this method to get a Box primitive for this mesh. If this meshes PrimitiveType is not a Box then the box parameter is set to Empty.

**Parameters:**
- `box` (Rhino.Geometry.Box) — Gets set to the box primitive for this object on success or Empty on error.

**Returns:** `Boolean` — Returns true if PrimitiveType is Box and the box parameter was initialized otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_TryGetBox.htm)

#### `public bool TryGetCone(out Cone cone, out Plane truncation)`

Call this method to get a Cone primitive for this mesh. If this meshes PrimitiveType is not a Cone then the cone parameter is set to Unset and the truncation parameter is set to Unset.

**Parameters:**
- `cone` (Rhino.Geometry.Cone) — Gets set to the cone primitive for this object on success or Unset on error.
- `truncation` (Rhino.Geometry.Plane) — Gets set to the truncation plane for this object on success or Unset on error.

**Returns:** `Boolean` — Returns true if PrimitiveType is Cone and the cone and truncation parameters were initialized otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_TryGetCone.htm)

#### `public bool TryGetPlane(out PlaneSurface plane)`

Call this method to get a Plane primitive for this mesh. If this meshes PrimitiveType is not a Plane then the plane parameter is set to null.

**Parameters:**
- `plane` (Rhino.Geometry.PlaneSurface) — Gets set to the plane primitive for this object on success or null on error.

**Returns:** `Boolean` — Returns true if PrimitiveType is Plane and the plane parameter was initialized otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_TryGetPlane.htm)

#### `public bool TryGetSphere(out Sphere sphere)`

Call this method to get a sphere primitive for this mesh. If this meshes PrimitiveType is not a Sphere then the sphere parameter is set to Unset.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — Gets set to the primitive sphere for this object on success.

**Returns:** `Boolean` — Returns true if PrimitiveType is Sphere and the sphere parameter was initialized otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitive_TryGetSphere.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — The bounding box for this primitive.
- `InstanceTransform` (Transform, get) — Instance reference transform or Identity if not an instance reference.
- `PrimitiveType` (RenderPrimitiveType, get) — Call this before extracting meshes if you support render primitives to get the RenderPrimitiveType of this mesh then call the associated TryGetSphere(Sphere), TryGetPlane(PlaneSurface), TryGetCone(Cone, Plane), or TryGetBox(Box) method. Calling the Mesh() property will mesh the primitive and return a mesh always.
- `RenderMaterial` (RenderMaterial, get) — The RenderMaterial associated with this mesh or null if there is not one.
- `RhinoObject` (RhinoObject, get) — The Rhino object associated with this render primitive.

## RenderPrimitiveList (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderPrimitiveList"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPrimitiveList.htm)

### Methods
#### `public void Add(Box box, RenderMaterial material)`

Add primitive box and material.

**Parameters:**
- `box` (Rhino.Geometry.Box) — Box to add.
- `material` (Rhino.Render.RenderMaterial) — Material to add, may be null if not needed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add.htm)

#### `public void Add(Cone cone, Plane truncation, RenderMaterial material)`

Add primitive cone and material.

**Parameters:**
- `cone` (Rhino.Geometry.Cone) — Cone to add.
- `truncation` (Rhino.Geometry.Plane) — The plane used to cut the cone (the non-apex end is kept). Should be equal to cone.plane if not truncated.
- `material` (Rhino.Render.RenderMaterial) — Material to add, may be null if not needed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add_1.htm)

#### `public void Add(IEnumerable<Mesh> meshes, RenderMaterial material)`



**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — [Missing <param name="meshes"/> documentation for "M:Rhino.Render.RenderPrimitiveList.Add(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Render.RenderMaterial)"]
- `material` (Rhino.Render.RenderMaterial) — [Missing <param name="material"/> documentation for "M:Rhino.Render.RenderPrimitiveList.Add(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.Render.RenderMaterial)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add_6.htm)

#### `public void Add(Mesh mesh, RenderMaterial material)`

Add mesh and material.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to add.
- `material` (Rhino.Render.RenderMaterial) — Material to add, may be null if not needed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add_2.htm)

#### `public void Add(Mesh mesh, RenderMaterial material, Transform t)`

Add mesh and material.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to add.
- `material` (Rhino.Render.RenderMaterial) — Transformation of this mesh. Material to add, may be null if not needed.
- `t` (Rhino.Geometry.Transform) — [Missing <param name="t"/> documentation for "M:Rhino.Render.RenderPrimitiveList.Add(Rhino.Geometry.Mesh,Rhino.Render.RenderMaterial,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add_3.htm)

#### `public void Add(PlaneSurface plane, RenderMaterial material)`

Add primitive plane and material.

**Parameters:**
- `plane` (Rhino.Geometry.PlaneSurface) — Plane to add.
- `material` (Rhino.Render.RenderMaterial) — Material to add, may be null if not needed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add_4.htm)

#### `public void Add(Sphere sphere, RenderMaterial material)`

Add primitive sphere and material.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — Sphere to add.
- `material` (Rhino.Render.RenderMaterial) — Material to add, may be null if not needed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Add_5.htm)

#### `public bool AutoDeleteMaterialsOn()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPrimitiveList.AutoDeleteMaterialsOn"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_AutoDeleteMaterialsOn.htm)

#### `public bool AutoDeleteMeshesOn()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderPrimitiveList.AutoDeleteMeshesOn"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_AutoDeleteMeshesOn.htm)

#### `public void Clear()`

Remove all primitives from this list

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Clear.htm)

#### `public void ConvertMeshesToTriangles()`

Convert mesh quad faces to triangle faces.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_ConvertMeshesToTriangles.htm)

#### `public void Dispose()`

Releases all resources used by the RenderPrimitiveList

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Finalize.htm)

#### `public Transform GetInstanceTransform(int index)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Render.RenderPrimitiveList.GetInstanceTransform(System.Int32)"]

**Returns:** `Transform` — [Missing <returns> documentation for "M:Rhino.Render.RenderPrimitiveList.GetInstanceTransform(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_GetInstanceTransform.htm)

#### `public RenderMaterial Material(int index)`

Call this method to get the render material associated with the mesh at the specified index. Will return null if there is no material associated with the requested mesh.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.

**Returns:** `RenderMaterial` — If there is a render material associated at the requested index then the material is returned otherwise null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Material.htm)

#### `public Mesh Mesh(int index)`

Get the mesh for the primitive at the specified index. If the item at this index is a primitive type other than a mesh then it mesh representation is returned.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.

**Returns:** `Mesh` — Returns the mesh for the primitive at the specified index. If the item at this index is a primitive type other than a mesh then it mesh representation is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_Mesh.htm)

#### `public Mesh MeshInstance(int index, out Transform instance_transform)`

Get the mesh for the primitive at the specified index. If the item at this index is a primitive type other than a mesh then it mesh representation is returned.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.
- `instance_transform` (Rhino.Geometry.Transform) — Receives the transformation of this mesh.

**Returns:** `Mesh` — Returns the mesh for the primitive at the specified index. If the item at this index is a primitive type other than a mesh then it mesh representation is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_MeshInstance.htm)

#### `public RenderPrimitiveType PrimitiveType(int index)`

Type of primitive object at this index.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.

**Returns:** `RenderPrimitiveType` — Primitive type of the item at this index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_PrimitiveType.htm)

#### `public void SetInstanceTransform(int index, Transform xform)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Render.RenderPrimitiveList.SetInstanceTransform(System.Int32,Rhino.Geometry.Transform)"]
- `xform` (Rhino.Geometry.Transform) — [Missing <param name="xform"/> documentation for "M:Rhino.Render.RenderPrimitiveList.SetInstanceTransform(System.Int32,Rhino.Geometry.Transform)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_SetInstanceTransform.htm)

#### `public RenderMaterial[] ToMaterialArray()`

Call this method to see if there are any RenderMaterials associated with the meshes. Each primitive can optionally have a RenderMaterial associated with it, if the RenderMaterial is null then check for a RhinoObject.RenderMaterial.

**Returns:** `RenderMaterial[]` — Return an array that of the same size as the ToMeshArray() containing the RenderMaterial associated with the mesh, may contain null entries if there is no RenderMaterial associated with the custom mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_ToMaterialArray.htm)

#### `public Mesh[] ToMeshArray()`

Call this method to get a array of meshes, all primitives will get meshed and the meshes will get included in the returned array.

**Returns:** `Mesh[]` — Return an array of meshes from this list, this will convert all primitives to meshes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_ToMeshArray.htm)

#### `public bool TryGetBox(int index, out Box box)`

Call this method to get a box at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.
- `box` (Rhino.Geometry.Box) — Will contain the box at the requested index if the index is in range and the primitive at the requested index is a box.

**Returns:** `Boolean` — Return true if the index is in range and the primitive at the requested index is a box otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_TryGetBox.htm)

#### `public bool TryGetCone(int index, out Cone cone, out Plane truncation)`

Call this method to get a box at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.
- `cone` (Rhino.Geometry.Cone) — Will contain the cone at the requested index if the index is in range and the primitive at the requested index is a box.
- `truncation` (Rhino.Geometry.Plane) — [Missing <param name="truncation"/> documentation for "M:Rhino.Render.RenderPrimitiveList.TryGetCone(System.Int32,Rhino.Geometry.Cone@,Rhino.Geometry.Plane@)"]

**Returns:** `Boolean` — Return true if the index is in range and the primitive at the requested index is a box otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_TryGetCone.htm)

#### `public bool TryGetPlane(int index, out PlaneSurface plane)`

Call this method to get a box at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.
- `plane` (Rhino.Geometry.PlaneSurface) — Will contain the plane at the requested index if the index is in range and the primitive at the requested index is a plane.

**Returns:** `Boolean` — Return true if the index is in range and the primitive at the requested index is a plane otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_TryGetPlane.htm)

#### `public bool TryGetSphere(int index, out Sphere sphere)`

Call this method to get a box at the specified index.

**Parameters:**
- `index` (System.Int32) — The zero based index of the item in the list. Valid values are greater than or equal to 0 and less than Count.
- `sphere` (Rhino.Geometry.Sphere) — Will contain the sphere at the requested index if the index is in range and the primitive at the requested index is a box.

**Returns:** `Boolean` — Return true if the index is in range and the primitive at the requested index is a box otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderPrimitiveList_TryGetSphere.htm)

### Properties
- `Count` (Int32, get) — Number of meshes in this list
- `RhinoObject` (RhinoObject, get) — The Rhino object associated with this list
- `UseObjectsMappingChannels` (Boolean, get/set) — Returns true if the texture mapping will be taken from the Rhino object otherwise; the texture mapping will use the texture coordinates on the mesh only.

## RenderPrimitiveType (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderPrimitiveType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPrimitiveType.htm)

### Values
- `None` = `0`
- `Mesh` = `1`
- `Sphere` = `2`
- `Plane` = `3`
- `Box` = `4`
- `Cone` = `5`

## RenderPropertyChangedEvent (class)

Used by Rhino.Render object property value has changed events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderPropertyChangedEvent.htm)

### Properties
- `Context` (Int32, get) — Optional argument which may specify the property being modified.
- `Document` (RhinoDoc, get) — The document triggering the event.

## RenderSettings (class)

Contains settings used in rendering.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderSettings.htm)

### Constructors
- `public RenderSettings()` — Initialize a new instance of the RenderSettings class.
- `public RenderSettings(RenderSettings source)` — Initialize new instance of the RenderSettings class.

### Methods
#### `protected void ConstructConstObject(Object parentObject, int subobjectIndex)`

Assigns a parent object and a sub-object index to this.

**Parameters:**
- `parentObject` (System.Object) — The parent object.
- `subobjectIndex` (System.Int32) — The sub-object index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ConstructConstObject.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Dispose_1.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Finalize.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

#### `public bool IsValidWithLog(out string log)`

Determines if an object is valid. Also provides a report on errors if this object happens not to be valid.

**Parameters:**
- `log` (System.String) — A textual log. This out parameter is assigned during this call.

**Returns:** `Boolean` — true if this object is valid; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_IsValidWithLog.htm)

#### `protected virtual void NonConstOperation()`

For derived classes implementers. Defines the necessary implementation to free the instance from being constant.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_NonConstOperation.htm)

#### `protected virtual void OnSwitchToNonConst()`

Is called when a non-constant operation first occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_OnSwitchToNonConst.htm)

#### `public string ToJSON(SerializationOptions options)`

Create a JSON string representation of this object

**Parameters:**
- `options` (Rhino.FileIO.SerializationOptions) — [Missing <param name="options"/> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.CommonObject.ToJSON(Rhino.FileIO.SerializationOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_ToJSON.htm)

### Properties
- `AmbientLight` (Color, get/set) — Gets or sets the ambient light color used in rendering.
- `AntialiasLevel` (AntialiasLevel, get/set) — Gets or sets anti-alias level, used for render quality
- `BackgroundColorBottom` (Color, get/set) — Gets or sets the background bottom color used in rendering.
- `BackgroundColorTop` (Color, get/set) — Gets or sets the background top color used in rendering. Sets also the background color if a solid background color is set.
- `BackgroundStyle` (BackgroundStyle, get/set) — How the viewport's background should be filled.
- `DepthCue` (Boolean, get/set) — Gets or sets a value indicating whether to render using depth cues. These are clues to help the perception of position and orientation of objects in the image.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Dithering` (Dithering, get) — Get the document dithering interface
- `FlatShade` (Boolean, get/set) — Gets or sets a value indicating whether to render using flat shading.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `ImageDpi` (Double, get/set) — Number of dots/inch (dots=pixels) to use when printing and saving bitmaps. The default is 72.0 dots/inch.
- `ImageSize` (Size, get/set) — Gets or sets a value indicating the size of the rendering result if UseViewportSize is set to false. If UseViewportSize is set to true, then this value is ignored.
- `ImageUnitSystem` (UnitSystem, get/set) — unit system to use when converting image pixel size and DPI information into a print size. Default = inches
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `LinearWorkflow` (LinearWorkflow, get) — Get the document linear workflow interface
- `NamedView` (String, get/set) — Get or set the given named view
- `RenderAnnotations` (Boolean, get/set) — Gets or sets a value indicating whether to instruct the rendering engine to show annotations, such as linear dimensions or angular dimensions.
- `RenderBackfaces` (Boolean, get/set) — Gets or sets a value indicating whether to render back faces.
- `RenderChannels` (RenderChannels, get) — Get the document render channels interface
- `RenderCurves` (Boolean, get/set) — Gets or sets a value indicating whether to instruct the rendering engine to show curves.
- `RenderIsoparams` (Boolean, get/set) — Gets or sets a value indicating whether to instruct the rendering engine to show isocurves.
- `RenderMeshEdges` (Boolean, get/set) — Gets or sets a value indicating whether to instruct the rendering engine to show mesh edges.
- `RenderPoints` (Boolean, get/set) — Gets or sets a value indicating whether to instruct the rendering engine to show points.
- `RenderSource` (RenderSettings.RenderingSources, get/set) — Gets or sets the render source RenderSettings.RenderingSources enumeration.
- `ScaleBackgroundToFit` (Boolean, get/set) — Gets or sets a value indicating whether to scale the wallpaper in the background or not. This is meaningful only if the viewport has a wallpaper and render settings are set to render Wallpaper into the background.
- `ShadowmapLevel` (Int32, get/set) — 0=none, 1=normal, 2=best.
- `Snapshot` (String, get/set) — Set or get the given snapshot view
- `SpecificViewport` (String, get/set) — Set or get the given specific viewport
- `TransparentBackground` (Boolean, get/set) — Gets or sets whether rendering should be done with transparent background.
- `UseHiddenLights` (Boolean, get/set) — Gets or sets a value indicating whether to render using lights that are on layers that are off.
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `UseViewportSize` (Boolean, get/set) — Gets or sets a value indicating whether to use the resolution of the viewport being rendered or ImageSize when rendering

## RenderSettings.RenderingSources (enum)

Rendering source (render directly from a NamedView or Snapshot)

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderSettings_RenderingSources.htm)

### Values
- `ActiveViewport` = `0` — Get the rendering view from the currently active viewport (as in all previous versions of Rhino)
- `SpecificViewport` = `1` — Get the rendering view from the named viewport (see NamedViewport below)
- `NamedView` = `2` — Get the rendering view from a specific named view (see NamedView below)
- `SnapShot` = `3` — Before rendering, restore the Snapshot specified in Snapshot below, then render.

## RenderSourceView (class)

Helper class to get the correct view from the Render View Source settings. An instance of this class is supposed to be used with the using() {} construct.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderSourceView.htm)

### Constructors
- `public RenderSourceView(RhinoDoc doc)` — Create a new RenderSourceView for the given doc. Note that this should be done with using(var rsv = new RenderSourceView(doc)) {} If the RenderSettings have the source view set to for instance a SnapShot this construct will ensure that the (active) view is set to the correct snapshot, and reverted back to the original once this instance goes out of scope.

### Methods
#### `public void Dispose()`

Releases all resources used by the RenderSourceView

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderSourceView_Dispose.htm)

#### `protected virtual void Dispose(bool isDisposing)`

Releases the unmanaged resources used by the RenderSourceView and optionally releases the managed resources

**Parameters:**
- `isDisposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderSourceView_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderSourceView_Finalize.htm)

#### `public ViewInfo GetViewInfo()`

Get the ViewInfo as specified by the render source view settings.

**Returns:** `ViewInfo` — ViewInfo if view was found, null otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderSourceView_GetViewInfo.htm)

## RenderTabs (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderTabs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderTabs.htm)

### Methods
#### `public static Object FromRenderSessionId(PlugIn plugIn, Type tabType, Guid renderSessionId)`

Get the instance of a render tab associated with a specific render session, this is useful when it is necessary to update a control from a RenderPipeline

**Parameters:**
- `plugIn` (Rhino.PlugIns.PlugIn) — The plug-in that registered the custom user interface
- `tabType` (System.Type) — The type of tab to return
- `renderSessionId` (System.Guid) — The RenderSessionId of a specific render session.

**Returns:** `Object` — Returns the custom tab object if found; otherwise null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTabs_FromRenderSessionId.htm)

#### `public void RegisterTab(PlugIn plugin, Type tabType, Guid renderEngineId, string caption, Icon icon)`



**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — [Missing <param name="plugin"/> documentation for "M:Rhino.Render.RenderTabs.RegisterTab(Rhino.PlugIns.PlugIn,System.Type,System.Guid,System.String,System.Drawing.Icon)"]
- `tabType` (System.Type) — [Missing <param name="tabType"/> documentation for "M:Rhino.Render.RenderTabs.RegisterTab(Rhino.PlugIns.PlugIn,System.Type,System.Guid,System.String,System.Drawing.Icon)"]
- `renderEngineId` (System.Guid) — [Missing <param name="renderEngineId"/> documentation for "M:Rhino.Render.RenderTabs.RegisterTab(Rhino.PlugIns.PlugIn,System.Type,System.Guid,System.String,System.Drawing.Icon)"]
- `caption` (System.String) — [Missing <param name="caption"/> documentation for "M:Rhino.Render.RenderTabs.RegisterTab(Rhino.PlugIns.PlugIn,System.Type,System.Guid,System.String,System.Drawing.Icon)"]
- `icon` (System.Drawing.Icon) — [Missing <param name="icon"/> documentation for "M:Rhino.Render.RenderTabs.RegisterTab(Rhino.PlugIns.PlugIn,System.Type,System.Guid,System.String,System.Drawing.Icon)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTabs_RegisterTab.htm)

#### `[ObsoleteAttribute("Use RegisterTabEx(PlugIn plugin, Type tabType, Guid renderEngineId, string caption, Icon icon)")] public void RegisterTab(PlugIn plugin, Type tabType, string caption, Icon icon)`

Register custom render user interface with Rhino. This should only be done in RegisterRenderTabs(RenderTabs). Panels registered after RegisterRenderTabs(RenderTabs) is called will be ignored. If the class includes a public method "void DoHelp()" the method will get called when F1 is pressed and the custom tab is active.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — The plug-in providing the custom user interface
- `tabType` (System.Type) — The type of object to be created and added to the render container.
- `caption` (System.String) — The caption for the custom user interface.
- `icon` (System.Drawing.Icon)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTabs_RegisterTab_1.htm)

#### `public static Guid SessionIdFromTab(Object tab)`

Get the session Id that created the specified tab object.

**Parameters:**
- `tab` (System.Object) — [Missing <param name="tab"/> documentation for "M:Rhino.Render.RenderTabs.SessionIdFromTab(System.Object)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Render.RenderTabs.SessionIdFromTab(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTabs_SessionIdFromTab.htm)

## RenderTexture (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderTexture"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderTexture.htm)

### Constructors
- `protected RenderTexture()` — Initializes a new instance of the RenderTexture class

### Methods
#### `public bool AddAutomaticUserInterfaceSection(string caption, int id)`

Add a new automatic user interface section, Field values which include prompts will be automatically added to this section.

**Parameters:**
- `caption` (System.String) — Expandable tab caption.
- `id` (System.Int32) — Tab id which may be used later on to reference this tab.

**Returns:** `Boolean` — Returns true if the automatic tab section was added otherwise; returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddAutomaticUserInterfaceSection.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent, string childSlotName)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild_1.htm)

#### `public bool AddUserInterfaceSection(ICollapsibleSection section)`

(Inherited from RenderContent.)

**Parameters:**
- `section` (Rhino.UI.Controls.ICollapsibleSection) — [Missing <param name="section"/> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection.htm)

#### `[ObsoleteAttribute("Use AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection) below instead. This function will not work on the Mac and is not type-safe.")] public UserInterfaceSection AddUserInterfaceSection(Type classType, string caption, bool createExpanded, bool createVisible)`

Add a new .NET control to an content expandable tab section, the height of the createExpanded tabs client area will be the initial height of the specified control.

**Parameters:**
- `classType` (System.Type) — The control class to create and embed as a child window in the expandable tab client area. This class type must be derived from IWin32Window or this method will throw an ArgumentException. Implement the IUserInterfaceSection interface in your classType to get UserInterfaceSection notification.
- `caption` (System.String) — Expandable tab caption.
- `createExpanded` (System.Boolean) — If this value is true then the new expandable tab section will initially be expanded, if it is false it will be collapsed.
- `createVisible` (System.Boolean) — If this value is true then the new expandable tab section will initially be visible, if it is false it will be hidden.

**Returns:** `UserInterfaceSection` — Returns the UserInterfaceSection object used to manage the new user control object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection_1.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`

Begins a change or batch of changes. It may also make a copy of the content state allowing EndChange() to send an event with the old and new contents. Calls to this method are counted; you must call EndChange() once for every call to BeginChange(). Note: If Changed() was called between the calls to BeginChange() and EndChange(), the last call to EndChange() may cause the ContentChanged event to be sent.

**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — the change context. If this is ChangeContexts.UI, ChangeContexts.Program,ChangeContexts.Drop or ChangeContexts.Tree, the content will be copied. EndChange() will then send the copy as 'old' in the OnContentChanged event. EndChange()ContentChanged

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginChange.htm)

#### `public void BeginCreateDynamicFields(bool automatic)`

Automatic Dynamic Field support. Dynamic fields are typically created in the constructor of RenderContent and they are therefore created automatically whenever the content is created. However, some advanced users require the fields to be created in response to some user action which occurs much later. This creates the problem that the fields do not exist by default and therefore cannot be loaded when the document is loaded. These methods are provided to solve that problem by making it possible to automatically create the dynamic fields on loading if they don't already exist. Dynamic fields that have this auto-create-on-load behavior are referred to as automatic dynamic fields. Dynamic fields that do not require the advanced automatic feature can still be created by using these methods (recommended), or they can be created manually (legacy usage). You must call this before creating any dynamic fields. Calls to this method are counted; you must call EndCreateDynamicFields() once for every call to BeginCreateDynamicFields().

**Parameters:**
- `automatic` (System.Boolean) — automatic specifies if the dynamic fields are automatic. If so, they will be created automatically during loading of the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginCreateDynamicFields.htm)

#### `public void BindParameterToField(string parameterName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField.htm)

#### `public void BindParameterToField(string parameterName, string childSlotName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField_1.htm)

#### `protected virtual uint CalculateRenderHash(ulong rcrcFlags)`

Override this method to calculate the render hash of the state that affects how the content is rendered. Does not include children or perform any caching. Render hash values are now automatically cached by the content framework and you do not have to worry about caching. You also do not have to worry about iterating into children. This method is now only called internally by the framework, use the RenderHash property to get the current hash value.

**Parameters:**
- `rcrcFlags` (System.UInt64) — [Missing <param name="rcrcFlags"/> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CalculateRenderHash.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool ChangeChild(RenderContent oldContent, RenderContent newContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — [Missing <param name="oldContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]
- `newContent` (Rhino.Render.RenderContent) — [Missing <param name="newContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChangeChild.htm)

#### `public double ChildSlotAmount(string childSlotName)`

Gets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Double` — The requested amount value. Values are typically from 0.0 to 100.0

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotAmount.htm)

#### `public string ChildSlotNameFromParamName(string paramName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `paramName` (System.String) — The name of a parameter field. Since child textures will usually correspond with some parameter (they generally either replace or modify a parameter over UV space) these functions are used to specify which parameter corresponded with child slot. If there is no correspondence, return the empty string.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotNameFromParamName.htm)

#### `public bool ChildSlotOn(string childSlotName)`

Gets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotOn.htm)

#### `public virtual void ConvertUnits(UnitSystem from, UnitSystem to)`

Modify your content so that it is converted from meters into the units of the unit system. No need to call the base class when you override this, and no need to recurse into children.

**Parameters:**
- `from` (Rhino.UnitSystem) — [Missing <param name="from"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]
- `to` (Rhino.UnitSystem) — [Missing <param name="to"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ConvertUnits.htm)

#### `public bool CreateDynamicField(string internalName, string localName, string englishName, Object value, Object minValue, Object maxValue, int sectionId)`

Create a dynamic field with an initial value and min and max limits.

**Parameters:**
- `internalName` (System.String) — is the internal name of the field. Not localized.
- `localName` (System.String) — is the localized user-friendly name of the field.
- `englishName` (System.String) — is the English user-friendly name of the field.
- `value` (System.Object) — is the initial value of the field.
- `minValue` (System.Object) — is the minimum value of the field. Must be the same type as vValue.
- `maxValue` (System.Object) — is the maximum value of the field. Must be the same type as vValue.
- `sectionId` (System.Int32) — is used for filtering fields between sections. Zero if not needed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CreateDynamicField(System.String,System.String,System.String,System.Object,System.Object,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CreateDynamicField.htm)

#### `[ObsoleteAttribute("Use version that takes TextureEvaluatorFlags enum")] public virtual TextureEvaluator CreateEvaluator()`

Constructs a texture evaluator. This is an independent lightweight object capable of evaluating texture color throughout uvw space. May be called from within a rendering shade pipeline.

**Returns:** `TextureEvaluator` — A texture evaluator instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_CreateEvaluator.htm)

#### `public virtual TextureEvaluator CreateEvaluator(RenderTexture.TextureEvaluatorFlags evaluatorFlags)`

Constructs a texture evaluator. This is an independent lightweight object capable of evaluating texture color throughout uvw space. May be called from within a rendering shade pipeline.

**Parameters:**
- `evaluatorFlags` (Rhino.Render.RenderTexture.TextureEvaluatorFlags) — [Missing <param name="evaluatorFlags"/> documentation for "M:Rhino.Render.RenderTexture.CreateEvaluator(Rhino.Render.RenderTexture.TextureEvaluatorFlags)"]

**Returns:** `TextureEvaluator` — A texture evaluator instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_CreateEvaluator_1.htm)

#### `public void DeleteAllChildren(RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteAllChildren(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteAllChildren.htm)

#### `public bool DeleteChild(string childSlotName, RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteChild.htm)

#### `public void Dispose()`

(Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Dispose

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose_1.htm)

#### `public virtual bool DynamicIcon(Size size, out Bitmap bitmap, DynamicIconUsage usage)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `usage` (Rhino.Render.DynamicIconUsage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DynamicIcon.htm)

#### `public RenderContent Edit()`

This method allows a render content hierarchy to be edited using a modal (AKA 'pop-up') editor. If the original render content is in a document, it will remain there, and the edited one will be 'free-floating'. Therefore it is the caller's responsibility to do any replacement in the document if required. The returned new content will be owned by the caller.

**Returns:** `RenderContent` — Returns an edited version of the content hierarchy if successful, else null. The method always edits the entire hierarchy. It places a copy of the hierarchy in the editor and selects the copied item that corresponds to this one. Therefore, editing a child will return a top-level render content, not a child.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Edit.htm)

#### `public void EndChange()`

Ends a change or batch of changes. Calls to this method are counted; you must call this method once for every call to BeginChange(RenderContent.ChangeContexts). Note: If BeginChange(RenderContent.ChangeContexts) was called with ChangeContexts.UI, ChangeContexts.Program, ChangeContexts.Drop or ChangeContexts.UI.Tree and Changed() was called between the calls to BeginChange(RenderContent.ChangeContexts) and EndChange(), the last call to EndChange() will raise the ContentChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndChange.htm)

#### `public void EndCreateDynamicFields()`

You must call this after creating dynamic fields. Calls to this method are counted; you must call BeginCreateDynamicFields() once for every call to EndCreateDynamicFields().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndCreateDynamicFields.htm)

#### `public ContentFactory Factory()`

(Inherited from RenderContent.)

**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Factory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Factory.htm)

#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Finalize.htm)

#### `public RenderContent FindChild(string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FindChild.htm)

#### `public RenderContent ForDisplay()`

The only place a single proxy can be displayed is in the New Content Control main thumbnail. All other attempts to use a single proxy in a UI require it to be replaced with the corresponding multi proxy. Single proxies override this to find the corresponding multi proxy.

**Returns:** `RenderContent` — The cotnent.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ForDisplay.htm)

#### `public virtual Object GetChildSlotParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to specify additional functionality for automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call GetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetChildSlotParameter.htm)

#### `public bool GetDisplayInViewport()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetDisplayInViewport"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetDisplayInViewport.htm)

#### `public string[] GetEmbeddedFilesList()`

(Inherited from RenderContent.)

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetEmbeddedFilesList"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetEmbeddedFilesList.htm)

#### `public TextureEnvironmentMappingMode GetEnvironmentMappingMode()`



**Returns:** `TextureEnvironmentMappingMode` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingMode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetEnvironmentMappingMode.htm)

#### `public static bool GetEnvironmentMappingProjection(TextureEnvironmentMappingMode mode, Vector3d reflectionVector, out float u, out float v)`



**Parameters:**
- `mode` (Rhino.Render.TextureEnvironmentMappingMode) — [Missing <param name="mode"/> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingProjection(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Geometry.Vector3d,System.Single@,System.Single@)"]
- `reflectionVector` (Rhino.Geometry.Vector3d) — [Missing <param name="reflectionVector"/> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingProjection(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Geometry.Vector3d,System.Single@,System.Single@)"]
- `u` (System.Single) — [Missing <param name="u"/> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingProjection(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Geometry.Vector3d,System.Single@,System.Single@)"]
- `v` (System.Single) — [Missing <param name="v"/> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingProjection(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Geometry.Vector3d,System.Single@,System.Single@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingProjection(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Geometry.Vector3d,System.Single@,System.Single@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetEnvironmentMappingProjection.htm)

#### `public Object GetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetExtraRequirementParameter.htm)

#### `public TextureEnvironmentMappingMode GetInternalEnvironmentMappingMode()`



**Returns:** `TextureEnvironmentMappingMode` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetInternalEnvironmentMappingMode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetInternalEnvironmentMappingMode.htm)

#### `public RenderTexture.eLocalMappingType GetLocalMappingType()`



**Returns:** `RenderTexture.eLocalMappingType` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetLocalMappingType"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetLocalMappingType.htm)

#### `public virtual int GetMappingChannel()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetMappingChannel"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetMappingChannel.htm)

#### `public virtual Vector3d GetOffset()`

Get offset value across UVW space. If the projection type is WCS or other type specified in model units, then this is the offset in meters.

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetOffset"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetOffset.htm)

#### `public virtual bool GetOffsetLocked()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetOffsetLocked"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetOffsetLocked.htm)

#### `public virtual Object GetParameter(string parameterName)`

Query the content instance for the value of a given named parameter. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — Name of the parameter

**Returns:** `Object` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetParameter.htm)

#### `public virtual bool GetPreviewIn3D()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetPreviewIn3D"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetPreviewIn3D.htm)

#### `public bool GetPreviewLocalMapping()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetPreviewLocalMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetPreviewLocalMapping.htm)

#### `public virtual TextureProjectionMode GetProjectionMode()`



**Returns:** `TextureProjectionMode` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetProjectionMode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetProjectionMode.htm)

#### `public virtual Vector3d GetRepeat()`

Get repeat value across UVW space. If the projection type is WCS or other type specified in model units, then this is the repeat across 1 meter of the model.

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetRepeat"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetRepeat.htm)

#### `public virtual bool GetRepeatLocked()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetRepeatLocked"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetRepeatLocked.htm)

#### `public virtual Vector3d GetRotation()`



**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetRotation"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetRotation.htm)

#### `public virtual ulong GetUiHash()`

This allows a content to have more than one UI for the same content type.

**Returns:** `UInt64` — Default is zero and is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUiHash.htm)

#### `public bool GetUnderlyingInstances(ref RenderContentCollection collection)`

(Inherited from RenderContent.)

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUnderlyingInstances.htm)

#### `public static Point3d GetWcsBoxMapping(Point3d worldXyz, Vector3d normal)`



**Parameters:**
- `worldXyz` (Rhino.Geometry.Point3d) — [Missing <param name="worldXyz"/> documentation for "M:Rhino.Render.RenderTexture.GetWcsBoxMapping(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d)"]
- `normal` (Rhino.Geometry.Vector3d) — [Missing <param name="normal"/> documentation for "M:Rhino.Render.RenderTexture.GetWcsBoxMapping(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetWcsBoxMapping(Rhino.Geometry.Point3d,Rhino.Geometry.Vector3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetWcsBoxMapping.htm)

#### `public virtual TextureWrapType GetWrapType()`



**Returns:** `TextureWrapType` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetWrapType"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetWrapType.htm)

#### `public void GraphInfo(ref TextureGraphInfo tgi)`



**Parameters:**
- `tgi` (Rhino.Render.TextureGraphInfo) — [Missing <param name="tgi"/> documentation for "M:Rhino.Render.RenderTexture.GraphInfo(Rhino.Render.TextureGraphInfo@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GraphInfo.htm)

#### `public virtual bool Icon(Size size, out Bitmap bitmap)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Icon.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public bool Initialize()`

Obsolete. (Inherited from RenderContent.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Initialize.htm)

#### `public virtual bool IsCompatible(Guid renderEngineId)`

(Inherited from RenderContent.)

**Parameters:**
- `renderEngineId` (System.Guid) — [Missing <param name="renderEngineId"/> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsCompatible.htm)

#### `public virtual bool IsContentTypeAcceptableAsChild(Guid type, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `type` (System.Guid) — [Missing <param name="type"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsContentTypeAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(ContentFactory factory, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `factory` (Rhino.Render.DataSources.ContentFactory) — [Missing <param name="factory"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(Guid kindId, string factoryKind, string childSlotName)`

Override this method to restrict the type of acceptable child content. The default implementation of this method just returns true.

**Parameters:**
- `kindId` (System.Guid) — [Missing <param name="kindId"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `factoryKind` (System.String) — [Missing <param name="factoryKind"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]

**Returns:** `Boolean` — Return true only if content with the specified kindId can be accepted as a child in the specified child slot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild_1.htm)

#### `public bool IsHdrCapable()`

Return true if the texture is HDR capable. When creating a custom RenderTexture implementation that is HDR capable set the appropriate property on the CustomRenderContentAttribute decorator on that clas.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.IsHdrCapable"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsHdrCapable.htm)

#### `public virtual bool IsImageBased()`

Query if the texture is image based. When creating a custom RenderTexture implementation of an image-based texture set the appropriate property on the CustomRenderContentAttribute decorator on that class. Do not override this function

**Returns:** `Boolean` — true if the texture is image-based.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsImageBased.htm)

#### `public virtual bool IsLinear()`

Return true if the texture color data is linear. NOTE: this function is marked as virtual, but the correct way to make a custom RenderTexture linear is by setting the correct property for the CustomRenderContentAttribute decorator on the class.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.IsLinear"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsLinear.htm)

#### `public bool IsNormalMap()`

Return true if the texture is a normalmap. When creating a custom RenderTexture implementation of a normal map set the appropriate property on the CustomRenderContentAttribute decorator on that clas.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.IsNormalMap"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsNormalMap.htm)

#### `public bool IsReference()`

Query whether or not the content or any of its ancestors is a reference content.

**Returns:** `Boolean` — true if the content is a reference, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsReference.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public bool IsRenderHashCached()`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Returns:** `Boolean` — bool

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsRenderHashCached.htm)

#### `public RenderContent MakeCopy()`

Create a copy of the render content. All content is the same, except for the instance Id.

**Returns:** `RenderContent` — The new RenderContent

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeCopy.htm)

#### `public RenderContent MakeGroupInstance()`

Create an 'instance' of the content hierarchy and group the new hierarchy with this hierarchy. If the instance is subsequently attached to the same document, the state of all members of the group will be kept synchronized. With the exception of the instance ids, all state is exactly preserved in the new instance hierarchy. \note The grouping will have no effect until the new instance is attached to the same document.

**Returns:** `RenderContent` — A grouped instance of the content hierarchy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeGroupInstance.htm)

#### `public virtual RenderContent.MatchDataResult MatchData(RenderContent oldContent)`

Implement to transfer data from another content to this content during creation.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — An old content object from which the implementation may harvest data.

**Returns:** `RenderContent.MatchDataResult` — Information about how much data was matched.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MatchData.htm)

#### `protected void ModifyRenderContentStyles(RenderContentStyles stylesToAdd, RenderContentStyles stylesToRemove)`

ModifyRenderContentStyles

**Parameters:**
- `stylesToAdd` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToAdd"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]
- `stylesToRemove` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToRemove"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ModifyRenderContentStyles.htm)

#### `public static RenderTexture NewBitmapTexture(Bitmap bitmap, RhinoDoc doc)`

Constructs a new basic texture from a Bitmap.

**Remarks:** The returned bitmap texture holds a copy of 'bitmap'.

**Parameters:**
- `bitmap` (System.Drawing.Bitmap) — The bitmap to create the basic texture from.
- `doc` (Rhino.RhinoDoc) — The document to associate the texture with.

**Returns:** `RenderTexture` — A new render texture.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_NewBitmapTexture_2.htm)

#### `[ObsoleteAttribute] public static RenderTexture NewBitmapTexture(SimulatedTexture texture)`

Constructs a new basic texture from a SimulatedTexture.

**Parameters:**
- `texture` (Rhino.Render.SimulatedTexture) — The texture to create the basic texture from.

**Returns:** `RenderTexture` — A new render texture.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_NewBitmapTexture.htm)

#### `public static RenderTexture NewBitmapTexture(SimulatedTexture texture, RhinoDoc doc)`

Constructs a new basic texture from a SimulatedTexture.

**Parameters:**
- `texture` (Rhino.Render.SimulatedTexture) — The texture to create the basic texture from.
- `doc` (Rhino.RhinoDoc) — The document to associate the texture with.

**Returns:** `RenderTexture` — A new render texture.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_NewBitmapTexture_1.htm)

#### `public PreviewSceneServer NewPreviewSceneServer(SceneServerData ssd)`

Gets the PreviewSceneServer of the content

**Parameters:**
- `ssd` (Rhino.Render.SceneServerData) — SceneServerData

**Returns:** `PreviewSceneServer` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.NewPreviewSceneServer(Rhino.Render.SceneServerData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_NewPreviewSceneServer.htm)

#### `protected virtual void OnAddUserInterfaceSections()`

Override to provide UI sections to display in the editor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnAddUserInterfaceSections.htm)

#### `protected virtual bool OnGetDefaultsInteractive()`

Override this method to prompt user for information necessary to create a new content object. For example, if you are created a textured material you may prompt the user for a bitmap file name prior to creating the textured material.

**Returns:** `Boolean` — If true is returned the content is created otherwise the creation is aborted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnGetDefaultsInteractive.htm)

#### `protected virtual void OnMakeCopy(ref RenderContent newContent)`

Override this function to supplement the standard copying behavour for your RenderContent.

**Parameters:**
- `newContent` (Rhino.Render.RenderContent) — Is the content that will be returned from MakeCopy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnMakeCopy.htm)

#### `public bool OpenInEditor()`

Call this method to open the content in the relevant thumbnail editor and select it for editing by the user. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInEditor.htm)

#### `[ObsoleteAttribute("Obsolete, use Edit a version that returns a RenderContent", false)] public bool OpenInModalEditor()`

Call this method to open the content in the a modal version of the editor. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInModalEditor.htm)

#### `public string ParamNameFromChildSlotName(string childSlotName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `childSlotName` (System.String) — The named of the child slot to receive the parameter name for.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ParamNameFromChildSlotName.htm)

#### `public void PixelSize(out int u, out int v, out int w)`

Get the texture dimensions for the RenderTexture.

**Parameters:**
- `u` (System.Int32) — width
- `v` (System.Int32) — height
- `w` (System.Int32) — depth, used for 3D textures

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_PixelSize.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames, LinearWorkflow lw)`

As RenderHash, but ignore parameter names given. Use this version of the function to calculate a render hash when you have the linear workflow information and you are not running on the main thread. Access to LinearWorkflow data requires document access. CrcRenderHashFlags.ExcludeLinearWorkflow must be specified.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string
- `lw` (Rhino.Render.LinearWorkflow) — Linear Workflow to use for CRC

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_1.htm)

#### `public uint RenderHashExclude(TextureRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.TextureRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_2.htm)

#### `public bool Replace(RenderContent newcontent)`

(Inherited from RenderContent.)

**Parameters:**
- `newcontent` (Rhino.Render.RenderContent) — [Missing <param name="newcontent"/> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Replace.htm)

#### `public bool SaveAsImage(string FullPath, int width, int height, int depth)`

Save texture as image

**Parameters:**
- `FullPath` (System.String) — The full path of the file
- `width` (System.Int32) — Image width
- `height` (System.Int32) — Image height
- `depth` (System.Int32) — Image depth

**Returns:** `Boolean` — returns true if file was saved, otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SaveAsImage.htm)

#### `public bool SetChild(RenderContent renderContent, string childSlotName)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild.htm)

#### `[ObsoleteAttribute("Use SetChild without ChangeContexts and Begin/EndChange")] public bool SetChild(RenderContent renderContent, string childSlotName, RenderContent.ChangeContexts changeContexts)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.SetChild(Rhino.Render.RenderContent,System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild_1.htm)

#### `public void SetChildSlotAmount(string childSlotName, double amount, RenderContent.ChangeContexts cc)`

Sets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `amount` (System.Double) — The texture amount. Values are typically from 0.0 to 100.0
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — The context of the change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotAmount.htm)

#### `public void SetChildSlotOn(string childSlotName, bool bOn, RenderContent.ChangeContexts cc)`

Sets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `bOn` (System.Boolean) — Value for the on-ness property.
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Context of the change

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotOn.htm)

#### `public virtual bool SetChildSlotParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to support values being set from automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call SetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotParameter.htm)

#### `public void SetDisplayInViewport(bool value)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetDisplayInViewport(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetDisplayInViewport.htm)

#### `public void SetDisplayInViewport(bool value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetDisplayInViewport(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetDisplayInViewport(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetDisplayInViewport_1.htm)

#### `public void SetEnvironmentMappingMode(TextureEnvironmentMappingMode value)`



**Parameters:**
- `value` (Rhino.Render.TextureEnvironmentMappingMode) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetEnvironmentMappingMode(Rhino.Render.TextureEnvironmentMappingMode)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetEnvironmentMappingMode.htm)

#### `public void SetEnvironmentMappingMode(TextureEnvironmentMappingMode value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (Rhino.Render.TextureEnvironmentMappingMode) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetEnvironmentMappingMode(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetEnvironmentMappingMode(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetEnvironmentMappingMode_1.htm)

#### `public bool SetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetExtraRequirementParameter.htm)

#### `public void SetGraphInfo(TextureGraphInfo tgi)`



**Parameters:**
- `tgi` (Rhino.Render.TextureGraphInfo) — [Missing <param name="tgi"/> documentation for "M:Rhino.Render.RenderTexture.SetGraphInfo(Rhino.Render.TextureGraphInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetGraphInfo.htm)

#### `public void SetIsRenderHashRecursive(bool recursive)`

By default, RenderHash() recurses into children when computing the render CRC. However, some applications may require children to be excluded from the render CRC calculation. Call this method to enable or disable recursing into children. see RenderHash

**Parameters:**
- `recursive` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetIsRenderHashRecursive.htm)

#### `public virtual void SetMappingChannel(int value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetMappingChannel(System.Int32,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetMappingChannel(System.Int32,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetMappingChannel.htm)

#### `public void SetName(string name, bool renameEvents, bool ensureNameUnique)`

Set instance name for this content

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `renameEvents` (System.Boolean) — [Missing <param name="renameEvents"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `ensureNameUnique` (System.Boolean) — [Missing <param name="ensureNameUnique"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetName.htm)

#### `public virtual void SetOffset(Vector3d value, RenderContent.ChangeContexts changeContext)`

Set offset value across UVW space. If the projection type is WCS or other type specified in model units, then this is the offset in meters.

**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetOffset(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetOffset(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetOffset.htm)

#### `public virtual void SetOffsetLocked(bool value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetOffsetLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetOffsetLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetOffsetLocked.htm)

#### `public virtual bool SetParameter(string parameterName, Object value)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter.htm)

#### `[ObsoleteAttribute("Use SetParameter without ChangeContexts and Begin/EndChange")] public virtual bool SetParameter(string parameterName, Object value, RenderContent.ChangeContexts changeContext)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter_1.htm)

#### `public virtual void SetPreviewIn3D(bool value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewIn3D(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewIn3D(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetPreviewIn3D.htm)

#### `public void SetPreviewLocalMapping(bool value)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewLocalMapping(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetPreviewLocalMapping.htm)

#### `public void SetPreviewLocalMapping(bool value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewLocalMapping(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewLocalMapping(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetPreviewLocalMapping_1.htm)

#### `public virtual void SetProjectionMode(TextureProjectionMode value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (Rhino.Render.TextureProjectionMode) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetProjectionMode(Rhino.Render.TextureProjectionMode,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetProjectionMode(Rhino.Render.TextureProjectionMode,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetProjectionMode.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public void SetRenderHash(uint hash)`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Parameters:**
- `hash` (System.UInt32)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetRenderHash.htm)

#### `public virtual void SetRepeat(Vector3d value, RenderContent.ChangeContexts changeContext)`

Set repeat value across UVW space. If the projection type is WCS or other type specified in model units, then this is the repeat across 1 meter of the model.

**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeat(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeat(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetRepeat.htm)

#### `public virtual void SetRepeatLocked(bool value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeatLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeatLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetRepeatLocked.htm)

#### `public virtual void SetRotation(Vector3d value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetRotation(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetRotation(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetRotation.htm)

#### `public virtual void SetWrapType(TextureWrapType value, RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `value` (Rhino.Render.TextureWrapType) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetWrapType(Rhino.Render.TextureWrapType,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetWrapType(Rhino.Render.TextureWrapType,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetWrapType.htm)

#### `[ObsoleteAttribute("Use SimulateTexture with size, TextureGeneration and object instead")] public virtual void SimulateTexture(ref SimulatedTexture simulation, bool isForDataOnly)`

Obsolete.

**Parameters:**
- `simulation` (Rhino.Render.SimulatedTexture) — [Missing <param name="simulation"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,System.Boolean)"]
- `isForDataOnly` (System.Boolean) — [Missing <param name="isForDataOnly"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SimulateTexture_1.htm)

#### `public virtual void SimulateTexture(ref SimulatedTexture simulation, RenderTexture.TextureGeneration tg, int size = -1, RhinoObject obj = null)`



**Parameters:**
- `simulation` (Rhino.Render.SimulatedTexture) — [Missing <param name="simulation"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — [Missing <param name="tg"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `size` (System.Int32) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SimulateTexture.htm)

#### `public virtual SimulatedTexture SimulatedTexture(RenderTexture.TextureGeneration tg, int size = -1, RhinoObject obj = null)`



**Parameters:**
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — [Missing <param name="tg"/> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `size` (System.Int32) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]

**Returns:** `SimulatedTexture` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SimulatedTexture.htm)

#### `public bool SmartUngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. If any content in the same document is left alone in the group, that content is also ungrouped. Records undo and sends events OnContentChanged and OnContentGroupIdChanged. \note This method is designed to be called from a content UI and is intended for RDK internal use but may be used as an expert user override.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SmartUngroupRecursive.htm)

#### `public bool Ungroup()`

Remove this content from any instance group it may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if content was ungrouped, \e false if it was not part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Ungroup.htm)

#### `public bool UngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UngroupRecursive.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public void Uninitialize()`

Obsolete. (Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Uninitialize.htm)

#### `public int UseCount()`

UseCount returns how many times the content is used

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.UseCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UseCount.htm)

#### `public virtual bool VirtualIcon(Size size, out Bitmap bitmap)`

Icon to display in the content browser, this bitmap needs to be valid for the life of this content object, the content object that returns the bitmap is responsible for disposing of the bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Requested icon size
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.VirtualIcon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — Return Icon to display in the content browser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_VirtualIcon.htm)

### Properties
- `CanBeEdited` (Boolean, get) — Determines if the content can be edited.
- `Category` (String, get) — Category for this content.
- `ChildSlotDisplayName` (String, get) — Returns the localized display name of the child slot name
- `ChildSlotName` (String, get/set) — (Inherited from RenderContent.)
- `CppPointer` (IntPtr, get) — (Inherited from RenderContent.)
- `DisplayName` (String, get) — Display name for this content.
- `Document` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `DocumentAssoc` (RhinoDoc, get/set) — If this render content is associated with a document in any way, the document will be returned. This includes copies of render contents that were attached to a document when the copy was made. Otherwise returns null.
- `DocumentOwner` (RhinoDoc, get) — If this render content is owned by a document, the document will be returned. This is the same as getting the document the render content is attached to. Otherwise returns null.
- `DocumentRegistered` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `Fields` (FieldDictionary, get) — Rhino.Render.Fields FieldDictionary which provides access to setting and retrieving field values.
- `Filename` (String, get/set) — If the content is file based, this function can be overridden to deal with setting/getting the filename. Corresponds to IRhRdkFileBasedContent in the C++ SDK
- `FilesToEmbed` (IEnumerable<String>, get) — A string array of full paths to files used by the content that may be embedded in .3dm files and library files (.rmtl, .renv, .rtex). The default implementation returns an empty string list. Override this to return the file name or file names used by your content. This is typically used by textures that reference files containing the texture imagery.
- `FirstChild` (RenderContent, get) — Return First child of this content or nullptr if none.
- `GroupId` (Guid, get/set) — Group ID of the content
- `Hidden` (Boolean, get/set) — Gets or sets the render content's 'hidden' state. This feature only works for top-level render contents because it hides the entire hierarchy. It is normally used for 'implementation detail' render contents. For expert use only. Hidden render contents are never shown in the UI, with the exception of the Object (or Layer) Material Properties UI which always shows whatever is assigned to the object (or layer). In the Object (or Layer) Material Properties UI, if the user drops down the list, hidden render contents are not listed. Hidden render contents, being part of the document content list, will be listed by any scripts or other code that iterates over the document render content list. It is recommended that you set IsHidden once when you create your render content and leave it on to prevent flicker or slow performance.
- `Id` (Guid, get/set) — Instance identifier for this content.
- `IsDefaultInstance` (Boolean, get) — Checks if render content is default instance.
- `IsHiddenByAutoDelete` (Boolean, get) — Contents can be created as 'auto-delete' by certain commands such as 'Picture'. These contents are automatically hidden from the user when the associated Rhino object is deleted. They are later deleted when the document is saved.
- `IsLocked` (Boolean, get/set) — Set this property to true prior to adding content to the document to lock the content browser editing UI methods. Setting this to true will keep the browser from allowing things like deleting, renaming or changing content. This is useful for custom child content that you want to be editable but persistent. Setting this after adding content to the document will cause an exception to be thrown.
- `LocalMappingTransform` (Transform, get) — Gets the transformation that can be applied to the UVW vector to convert it from normalized texture space into locally mapped space (ie - with repeat, offset and rotation applied.)
- `Name` (String, get/set) — Instance 'raw' name for this content.
- `NextSibling` (RenderContent, get) — Return First sibling of this content or nullptr if none.
- `Notes` (String, get/set) — Notes for this content.
- `Parent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `PixelSize2` (Nullable<ValueTuple<T1,T2,T3>>, get) — Override to provide details on the actual pixel sizes of this texture in UVW directions return null if the texture should not be treated as a bitmap.
- `ProxyType` (ProxyTypes, get) — Gets the proxy type of the render content
- `RenderHash` (UInt32, get) — Render hash for the content hierarchy. It iterates children and includes a caching mechanism which means the hash value can be retrieved quickly if it hasn't changed. The cache is invalidated when Changed() is called. You can override the CalculateRenderHash(UInt64) method to provide a custom hash value.
- `RenderHashWithoutLocalMapping` (UInt32, get) — Render hash for texture excluding local mapping.
- `Styles` (RenderContentStyles, get) — (Inherited from RenderContent.)
- `Tags` (String, get/set) — Tags for this content.
- `TopLevel` (Boolean, get) — Returns true if this content has no parent, false if it is the child of another content.
- `TopLevelParent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `TypeDescription` (String, get) — Description for your content type. ie. "Procedural checker pattern"
- `TypeId` (Guid, get) — Type identifier for this content
- `TypeName` (String, get) — Name for your content type. ie. "My .net Texture"
- `Xml` (String, get) — (Inherited from RenderContent.)

## RenderTexture.TextureEvaluatorFlags (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderTexture.TextureEvaluatorFlags"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderTexture_TextureEvaluatorFlags.htm)

### Values
- `Normal` = `0`
- `DisableFiltering` = `1`
- `DisableLocalMapping` = `2`
- `DisableAdjustment` = `4`
- `DisableProjectionChange` = `8`

## RenderTexture.TextureGeneration (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderTexture.TextureGeneration"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderTexture_TextureGeneration.htm)

### Values
- `Allow` = `1`
- `Disallow` = `2`
- `Skip` = `3`

## RenderTexture.eLocalMappingType (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderTexture.eLocalMappingType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderTexture_eLocalMappingType.htm)

### Values
- `lmt_none` = `0`
- `lmt_2D` = `1`
- `lmt_3D` = `2`
- `lmt_force32bit` = `4294967295`

## RenderTextureTable (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderTextureTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderTextureTable.htm)

### Methods
#### `public bool Add(RenderTexture c)`



**Parameters:**
- `c` (Rhino.Render.RenderTexture) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderTextureTable.Add(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTextureTable.Add(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTextureTable_Add.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`



**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTextureTable.BeginChange(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTextureTable_BeginChange.htm)

#### `public void EndChange()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTextureTable_EndChange.htm)

#### `public IEnumerator<RenderTexture> GetEnumerator()`



**Returns:** `IEnumerator<RenderTexture>` — [Missing <returns> documentation for "M:Rhino.Render.RenderTextureTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTextureTable_GetEnumerator.htm)

#### `public bool Remove(RenderTexture c)`



**Parameters:**
- `c` (Rhino.Render.RenderTexture) — [Missing <param name="c"/> documentation for "M:Rhino.Render.RenderTextureTable.Remove(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTextureTable.Remove(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTextureTable_Remove.htm)

### Properties
- `Count` (Int32, get) — 
- `Item` (RenderTexture, get) — 

## RenderWindow (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderWindow"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindow.htm)

### Methods
#### `public bool AddChannel(RenderWindow.StandardChannels channel)`

Add a channel to the frame buffer in addition to the fixed Red, Green, Blue and Alpha channels.

**Parameters:**
- `channel` (Rhino.Render.RenderWindow.StandardChannels) — Channel to add

**Returns:** `Boolean` — If the channel existed then true is returned otherwise; returns true if the channel was added or false if not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_AddChannel.htm)

#### `public bool AddWireframeChannel(RhinoDoc doc, ViewportInfo viewport, Size size, Rectangle region)`

This method sets the frame buffer size and adds all the necessary wireframe channels automatically. It also creates the wireframe channel data automatically so that your renderer doesn't have to. You typically call this method only when your renderer does not support wireframe rendering itself. If you call this method, then you should not add any wireframe channels returned by GetRenderChannels(). If your renderer is capable of rendering the wireframe channels itself, you should not call this method. Instead, you must make sure you add the wireframe channels if GetRenderChannels() requests them. See IRhRdkRenderWindow::GetRenderChannels(). After the wires are rendered, the wireframe post effects will composite them into the final rendered image. Note: This method should really be called AddWireframeChannels(). [SDK_UNFREEZE] */

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The document to display
- `viewport` (Rhino.DocObjects.ViewportInfo) — The view to display
- `size` (System.Drawing.Size) — The size of the image without clipping (i.e., if you have a region, it is the size of the image before you cut the region out.
- `region` (System.Drawing.Rectangle) — The area of the rendering you want to display. This should match the size of the render window itself (i.e, - the one set using SetSize)

**Returns:** `Boolean` — Returns true if all of the wireframe channels were added successfully.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_AddWireframeChannel.htm)

#### `public static Guid ChannelId(RenderWindow.StandardChannels ch)`



**Parameters:**
- `ch` (Rhino.Render.RenderWindow.StandardChannels) — [Missing <param name="ch"/> documentation for "M:Rhino.Render.RenderWindow.ChannelId(Rhino.Render.RenderWindow.StandardChannels)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelId(Rhino.Render.RenderWindow.StandardChannels)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelId.htm)

#### `public static RenderWindow Create(Size szSize)`



**Parameters:**
- `szSize` (System.Drawing.Size) — [Missing <param name="szSize"/> documentation for "M:Rhino.Render.RenderWindow.Create(System.Drawing.Size)"]

**Returns:** `RenderWindow` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.Create(System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Create.htm)

#### `public void Dispose()`

Releases all resources used by the RenderWindow

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Dispose.htm)

#### `public void EndAsyncRender(RenderWindow.RenderSuccessCode successCode)`

Must be called when an asynchronous render has finished or ended for any reason.

**Parameters:**
- `successCode` (Rhino.Render.RenderWindow.RenderSuccessCode) — Completed for a correct or canceled render, Failed for errors.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_EndAsyncRender.htm)

#### `public static RenderWindow FromSessionId(Guid sessionId)`



**Parameters:**
- `sessionId` (System.Guid) — [Missing <param name="sessionId"/> documentation for "M:Rhino.Render.RenderWindow.FromSessionId(System.Guid)"]

**Returns:** `RenderWindow` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.FromSessionId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_FromSessionId.htm)

#### `[ObsoleteAttribute("The Render Window UI uses post effects for gamma, tone mapping etc. now.")] public RenderWindow.ImageAdjust GetAdjust()`

Get an ImageAdjust instance containing current image adjusting settings for this RenderWindow

**Returns:** `RenderWindow.ImageAdjust` — ImageAdjust

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_GetAdjust.htm)

#### `public Bitmap GetBitmap()`



**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.GetBitmap"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_GetBitmap.htm)

#### `public Guid[] GetRequestedRenderChannels()`

Get array of Guids representing the channels requested by the post effect pipeline for this RenderWindow.

**Returns:** `Guid[]` — Array of Guid

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_GetRequestedRenderChannels.htm)

#### `public RenderWindow.StandardChannels[] GetRequestedRenderChannelsAsStandardChannels()`

Get array of StandardChannels enum values representing the channels requested by the post effect pipeline for this RenderWindow.

**Returns:** `RenderWindow.StandardChannels[]` — Array of StandardChannels

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_GetRequestedRenderChannelsAsStandardChannels.htm)

#### `public void Invalidate()`

Invalidate the entire view window so that the pixels get painted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Invalidate.htm)

#### `public void InvalidateArea(Rectangle rect)`



**Parameters:**
- `rect` (System.Drawing.Rectangle) — [Missing <param name="rect"/> documentation for "M:Rhino.Render.RenderWindow.InvalidateArea(System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_InvalidateArea.htm)

#### `public bool IsChannelAvailable(Guid id)`

Query if a channel is available.

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Render.RenderWindow.IsChannelAvailable(System.Guid)"]

**Returns:** `Boolean` — Returns true if the channel is available.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_IsChannelAvailable.htm)

#### `public bool IsChannelShown(Guid id)`

Query if a channel is being shown.

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Render.RenderWindow.IsChannelShown(System.Guid)"]

**Returns:** `Boolean` — Returns true if the channel is being shown.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_IsChannelShown.htm)

#### `public RenderWindow.Channel OpenChannel(RenderWindow.StandardChannels id)`



**Parameters:**
- `id` (Rhino.Render.RenderWindow.StandardChannels) — [Missing <param name="id"/> documentation for "M:Rhino.Render.RenderWindow.OpenChannel(Rhino.Render.RenderWindow.StandardChannels)"]

**Returns:** `RenderWindow.Channel` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.OpenChannel(Rhino.Render.RenderWindow.StandardChannels)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_OpenChannel.htm)

#### `public void SaveDibAsBitmap(string filename)`

Save current Dib of RenderWindow as file Helper function for debugging purposes.

**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.Render.RenderWindow.SaveDibAsBitmap(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SaveDibAsBitmap.htm)

#### `public void SaveRenderImageAs(string filename, bool saveAlpha)`

Like RenderWindow.SaveRenderImageAs(string,Guid,bool), but with Guid set to Guid.Empty.

**Parameters:**
- `filename` (System.String) — Filename of image file to be created
- `saveAlpha` (System.Boolean) — True if alpha channel should be saved.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SaveRenderImageAs.htm)

#### `public void SaveRenderImageAs(string filename, Guid renderEngineGuid, bool saveAlpha)`

Save current RenderWindow contents as an image file with the given name. The filetype will be determine. Pass in render engine Guid if an engine implements saving to a format that is not supported by Rhino. Guid.Empty if there is no need for that.

**Parameters:**
- `filename` (System.String) — Filename of image file to be created
- `renderEngineGuid` (System.Guid) — render engine ID
- `saveAlpha` (System.Boolean) — True if alpha channel should be saved.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SaveRenderImageAs_1.htm)

#### `[ObsoleteAttribute("The Render Window UI uses post effects for gamma, tone mapping etc. now.")] public void SetAdjust(RenderWindow.ImageAdjust imageAdjust)`

Set new ImageAdjust to use. An ImageAdjust instance can be obtained by first querying for one using GetAdjust()

**Parameters:**
- `imageAdjust` (Rhino.Render.RenderWindow.ImageAdjust) — ImageAdjust

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetAdjust.htm)

#### `public void SetIsRendering(bool is_rendering)`

Set whether or not rendering is in progress. Added to support raytraced mode.

**Parameters:**
- `is_rendering` (System.Boolean) — [Missing <param name="is_rendering"/> documentation for "M:Rhino.Render.RenderWindow.SetIsRendering(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetIsRendering.htm)

#### `public void SetProgress(string text, float progress)`

Accepts a rendering progress value to inform the user of the rendering advances.

**Parameters:**
- `text` (System.String) — The progress text.
- `progress` (System.Single) — A progress value in the domain [0.0f; 1.0f].

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetProgress.htm)

#### `public void SetRGBAChannelColors(Rectangle rectangle, Color4f[] colors)`

Call this method to open the RenderWindow.StandardChannels.RGBA channel and set a block of color values

**Parameters:**
- `rectangle` (System.Drawing.Rectangle) — rectangle.X is the horizontal pixel position of the left edge. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer. rectangle.Y is the vertical pixel position of the top edge. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer. rectangle.Width is the width of the rectangle in pixels. No validation is done on this value. rectangle.Height is the height of the rectangle in pixels. No validation is done on this value.
- `colors` (Rhino.Display.Color4f[]) — Array of Color4f values used to set the RenderWindow.StandardChannels.RGBA

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetRGBAChannelColors.htm)

#### `public void SetRGBAChannelColors(Size size, Color4f[] colors)`

Call this method to open the RenderWindow.StandardChannels.RGBA channel and set a block of color values

**Parameters:**
- `size` (System.Drawing.Size) — Size of the area to set. No validation is done on this value
- `colors` (Rhino.Display.Color4f[]) — Array of Color4f values used to set the RenderWindow.StandardChannels.RGBA

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetRGBAChannelColors_1.htm)

#### `public void SetSize(Size size)`



**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderWindow.SetSize(System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetSize.htm)

#### `public void SetView(ViewInfo view)`



**Parameters:**
- `view` (Rhino.DocObjects.ViewInfo) — [Missing <param name="view"/> documentation for "M:Rhino.Render.RenderWindow.SetView(Rhino.DocObjects.ViewInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_SetView.htm)

#### `public Size Size()`



**Returns:** `Size` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.Size"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Size.htm)

#### `public static RenderWindow.StandardChannels StandardChannelForGuid(Guid id)`

Get the StandardChannels for the given Guid

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.Render.RenderWindow.StandardChannelForGuid(System.Guid)"]

**Returns:** `RenderWindow.StandardChannels` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.StandardChannelForGuid(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_StandardChannelForGuid.htm)

### Properties
- `SessionId` (Guid, get) — 

### Events
#### `Cloned` (`System.EventHandler<RenderWindowClonedEventArgs>`)

**Signature:** `public static event EventHandler<RenderWindowClonedEventArgs> Cloned`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_RenderWindow_Cloned.htm)

## RenderWindow.Channel (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderWindow.Channel"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindow_Channel.htm)

### Methods
#### `public void AddValue(int x, int y, Color4f value)`

Add value to existing values at point x, y. If x or y are out of range, the function will fail and may crash Rhino.

**Parameters:**
- `x` (System.Int32) — The horizontal pixel position. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer.
- `y` (System.Int32) — The vertical pixel position. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer.
- `value` (Rhino.Display.Color4f) — The color to store in the channel at the specified position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_AddValue.htm)

#### `public RenderWindow.Channel Clone()`

Return a clone of the channel.

**Returns:** `RenderWindow.Channel` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.Channel.Clone"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_Clone.htm)

#### `public void Dispose()`

Releases all resources used by the RenderWindow.Channel

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the RenderWindow.Channel and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_Dispose_1.htm)

#### `public void GetMinMaxValues(out float min, out float max)`

Get the minimum and maximum values in the channel. minThe channel's minimum value. maxThe channel's maximum value.

**Parameters:**
- `min` (System.Single) — [Missing <param name="min"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetMinMaxValues(System.Single@,System.Single@)"]
- `max` (System.Single) — [Missing <param name="max"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetMinMaxValues(System.Single@,System.Single@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_GetMinMaxValues.htm)

#### `public void GetValue(int x, int y, ComponentOrders componentOrder, ref float[] values)`

Get values from the channel.

**Parameters:**
- `x` (System.Int32) — [Missing <param name="x"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValue(System.Int32,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]
- `y` (System.Int32) — [Missing <param name="y"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValue(System.Int32,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]
- `componentOrder` (Rhino.Render.ComponentOrders) — [Missing <param name="componentOrder"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValue(System.Int32,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]
- `values` (System.Single[]) — [Missing <param name="values"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValue(System.Int32,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_GetValue.htm)

#### `public void GetValues(Rectangle rectangle, int stride, ComponentOrders componentOrder, ref float[] values)`

Get a rectangle of values from the channel. The input parameters are checked for validity. If the rectangle is not fully inside the frame buffer, the function will fail.

**Parameters:**
- `rectangle` (System.Drawing.Rectangle) — [Missing <param name="rectangle"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValues(System.Drawing.Rectangle,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]
- `stride` (System.Int32) — [Missing <param name="stride"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValues(System.Drawing.Rectangle,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]
- `componentOrder` (Rhino.Render.ComponentOrders) — [Missing <param name="componentOrder"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValues(System.Drawing.Rectangle,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]
- `values` (System.Single[]) — [Missing <param name="values"/> documentation for "M:Rhino.Render.RenderWindow.Channel.GetValues(System.Drawing.Rectangle,System.Int32,Rhino.Render.ComponentOrders,System.Single[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_GetValues.htm)

#### `public int PixelSize()`

Returns the size of the data in one pixel in the channel. For RDK standard channels, this value is always sizeof(float). For the special chanRGBA collective channel, this value is 4 * sizeof(float).

**Returns:** `Int32` — The size of a pixel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_PixelSize.htm)

#### `public void SetValue(int x, int y, Color4f value)`

If x or y are out of range, the function will fail and may crash Rhino.

**Parameters:**
- `x` (System.Int32) — The horizontal pixel position. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer.
- `y` (System.Int32) — The vertical pixel position. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer.
- `value` (Rhino.Display.Color4f) — The color to store in the channel at the specified position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_SetValue.htm)

#### `public void SetValue(int x, int y, float value)`

Assign value to a pixel at coordinate (x, y).

**Parameters:**
- `x` (System.Int32) — The horizontal pixel position. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer.
- `y` (System.Int32) — the vertical pixel position. No validation is done on this value. The caller is responsible for ensuring that it is within the frame buffer.
- `value` (System.Single) — The value to store in the channel at the specified position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_SetValue_1.htm)

#### `public void SetValues(Rectangle rectangle, Size bufferResolution, PixelBuffer colorBuffer)`

Set a pixel buffer

**Parameters:**
- `rectangle` (System.Drawing.Rectangle) — [Missing <param name="rectangle"/> documentation for "M:Rhino.Render.RenderWindow.Channel.SetValues(System.Drawing.Rectangle,System.Drawing.Size,Rhino.Render.PixelBuffer)"]
- `bufferResolution` (System.Drawing.Size) — [Missing <param name="bufferResolution"/> documentation for "M:Rhino.Render.RenderWindow.Channel.SetValues(System.Drawing.Rectangle,System.Drawing.Size,Rhino.Render.PixelBuffer)"]
- `colorBuffer` (Rhino.Render.PixelBuffer) — PixelBuffer to a color buffer of floats in RGBA format.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_SetValues.htm)

#### `public void SetValuesFlipped(Rectangle rectangle, Size bufferResolution, PixelBuffer colorBuffer)`

Set a pixel buffer y-flipped.

**Parameters:**
- `rectangle` (System.Drawing.Rectangle) — [Missing <param name="rectangle"/> documentation for "M:Rhino.Render.RenderWindow.Channel.SetValuesFlipped(System.Drawing.Rectangle,System.Drawing.Size,Rhino.Render.PixelBuffer)"]
- `bufferResolution` (System.Drawing.Size) — [Missing <param name="bufferResolution"/> documentation for "M:Rhino.Render.RenderWindow.Channel.SetValuesFlipped(System.Drawing.Rectangle,System.Drawing.Size,Rhino.Render.PixelBuffer)"]
- `colorBuffer` (Rhino.Render.PixelBuffer) — PixelBuffer to a color buffer of floats in RGBA format.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_Channel_SetValuesFlipped.htm)

### Properties
- `Height` (Int32, get) — Return the channel height.
- `Id` (Guid, get) — Return the channel id.
- `Width` (Int32, get) — Return the channel width.

## RenderWindow.ChannelGPU (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderWindow.ChannelGPU"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindow_ChannelGPU.htm)

### Methods
#### `public RenderWindow.ChannelGPU Clone()`

Return a clone of the channel.

**Returns:** `RenderWindow.ChannelGPU` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelGPU.Clone"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Clone.htm)

#### `public void Close()`

Close the channel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Close.htm)

#### `public void CopyTo(RenderWindow.Channel channel)`

Copy the channel to a 'CPU' channel.

**Parameters:**
- `channel` (Rhino.Render.RenderWindow.Channel) — The channel to copy to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_CopyTo.htm)

#### `public void Dispose()`

Dispose of the channel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Finalize.htm)

#### `public int Height()`

Return the channel height.

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelGPU.Height"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Height.htm)

#### `public Guid Id()`

Return the channel id.

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelGPU.Id"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Id.htm)

#### `public uint PixelSize()`

Return the size of one pixel's data in the channel.

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelGPU.PixelSize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_PixelSize.htm)

#### `public uint TextureHandle()`

Return the channel's texture handle.

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelGPU.TextureHandle"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_TextureHandle.htm)

#### `public int Width()`

Return the channel width.

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderWindow.ChannelGPU.Width"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderWindow_ChannelGPU_Width.htm)

### Properties
- `DisplayTechnology` (DisplayTechnology, get) — Return the type of API used for this channel's texture handle.

## RenderWindow.ImageAdjust (class)

Class to set image adjusting settings to a RenderWindow. This can't be directly created. Instead, one should use RenderWindow.GetAdjust() to get an ImageAdjust instance that can be modified, then passed into RenderWindow.SetAdjust() to update image adjusting settings.1

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindow_ImageAdjust.htm)

### Properties
- `Dither` (Dithering.Methods, get/set) — Set the Dithering method to use when adjusting the RenderWindow content.
- `Gamma` (Single, get/set) — Set the gamma value. The inverse of this will be used to apply gamma correction to the RenderWindow RGBA channel when necessary.

## RenderWindow.RenderSuccessCode (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderWindow.RenderSuccessCode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindow_RenderSuccessCode.htm)

### Values
- `Completed` = `0`
- `Failed` = `1`

## RenderWindow.StandardChannels (enum)

[Missing <summary> documentation for "T:Rhino.Render.RenderWindow.StandardChannels"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindow_StandardChannels.htm)

### Values
- `None` = `0`
- `Red` = `1`
- `Green` = `2`
- `Blue` = `4`
- `Alpha` = `8`
- `RGBA` = `14`
- `RGB` = `15`
- `DistanceFromCamera` = `16`
- `NormalX` = `32`
- `NormalY` = `64`
- `NormalZ` = `128`
- `NormalXYZ` = `240`
- `LuminanceRed` = `256`
- `LuminanceGreen` = `512`
- `LuminanceBlue` = `1024`
- `BackgroundLuminanceRed` = `4096`
- `BackgroundLuminanceGreen` = `8192`
- `BackgroundLuminanceBlue` = `16384`
- `MaterialIds` = `65536`
- `ObjectIds` = `131072`
- `Wireframe` = `262144`
- `AlbedoRed` = `1048576`
- `AlbedoGreen` = `2097152`
- `AlbedoBlue` = `4194304`
- `AlbedoRGB` = `7340032`
- `WireframePointsRGBA` = `8388608`
- `WireframeIsocurvesRGBA` = `16777216`
- `WireframeCurvesRGBA` = `33554432`
- `WireframeAnnotationsRGBA` = `67108864`

## RenderWindowClonedEventArgs (class)

[Missing <summary> documentation for "T:Rhino.Render.RenderWindowClonedEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_RenderWindowClonedEventArgs.htm)

### Properties
- `NewRenderWindow` (RenderWindow, get) — 
- `NewSessionId` (Guid, get) — 
- `OldRenderWindow` (RenderWindow, get) — 
- `OldSessionId` (Guid, get) — 

## SafeFrame (class)

SafeFrame

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SafeFrame.htm)

### Constructors
- `public SafeFrame()` — Create an utility object not associated with any document
- `public SafeFrame(RhinoDoc doc)` — Create the SafeFrame object which is associated with the document
- `public SafeFrame(SafeFrame g)` — Create an utility object not associated with any document from another object

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.SafeFrame.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SafeFrame_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `protected override void Finalize()`

Handle destruction of the un-managed CPP object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_FreeFloatingBase_Finalize.htm)

### Properties
- `ActionFrameLinked` (Boolean, get/set) — Action Frame Linked, On = Use the same scale for X and Y. Off = use different scales for X and Y.
- `ActionFrameOn` (Boolean, get/set) — Turn on the user specified action area, which shown with blue frames.
- `ActionFrameXScale` (Double, get/set) — Action Frame Scale X
- `ActionFrameYScale` (Double, get/set) — Action Frame Scale Y
- `Enabled` (Boolean, get/set) — Determines whether the safeframe is enabled.
- `FieldsOn` (Boolean, get/set) — Show a 4 by 3 grid in the safe-frame.
- `LiveFrameOn` (Boolean, get/set) — Turn on the live area, which shows the size of the rendered view as a yellow frame in the viewport.
- `PerspectiveOnly` (Boolean, get/set) — Show the safe-frame only in perspective views.
- `TitleFrameLinked` (Boolean, get/set) — Title Frame Linked, On = Use the same scale for X and Y. Off = use different scales for X and Y.
- `TitleFrameOn` (Boolean, get/set) — Show a user specified title area frame in orange.
- `TitleFrameXScale` (Double, get/set) — Title Frame Scale X
- `TitleFrameYScale` (Double, get/set) — Title Frame Scale Y

### Events
#### `Changed` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> Changed`

This event is raised when a SafeFrame property value is changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_SafeFrame_Changed.htm)

## SceneServerData (class)

The Scene Server Data used by the PreviewSceneServer

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SceneServerData.htm)

### Constructors
- `public SceneServerData(PreviewGeometry geo, PreviewBackground back, PreviewLighting light, SceneServerDataUsage usage)` — Constructor for SceneServerData

### Methods
#### `public void Dispose()`

Dispose for SceneServerData

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SceneServerData_Dispose.htm)

#### `protected override void Finalize()`

Destructor for SceneServerData

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SceneServerData_Finalize.htm)

### Properties
- `CppPointer` (IntPtr, get) — The CppPointer of SceneServerData

## SceneServerDataUsage (enum)

SceneServerData Usage (Synchronous or Asynchronous)

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SceneServerDataUsage.htm)

### Values
- `Synchronous` = `0`
- `Asynchronous` = `1`

## SimulatedEnvironment (class)

[Missing <summary> documentation for "T:Rhino.Render.SimulatedEnvironment"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SimulatedEnvironment.htm)

### Constructors
- `public SimulatedEnvironment()` — Initializes a new instance of the SimulatedEnvironment class

### Methods
#### `public IntPtr ConstPointer()`



**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedEnvironment.ConstPointer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedEnvironment_ConstPointer.htm)

#### `public void Dispose()`

Releases all resources used by the SimulatedEnvironment

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedEnvironment_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimulatedEnvironment and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedEnvironment_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedEnvironment_Finalize.htm)

#### `public static SimulatedEnvironment.BackgroundProjections ProjectionFromString(string projection)`



**Parameters:**
- `projection` (System.String) — [Missing <param name="projection"/> documentation for "M:Rhino.Render.SimulatedEnvironment.ProjectionFromString(System.String)"]

**Returns:** `SimulatedEnvironment.BackgroundProjections` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedEnvironment.ProjectionFromString(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedEnvironment_ProjectionFromString.htm)

#### `public static string StringFromProjection(SimulatedEnvironment.BackgroundProjections projection)`



**Parameters:**
- `projection` (Rhino.Render.SimulatedEnvironment.BackgroundProjections) — [Missing <param name="projection"/> documentation for "M:Rhino.Render.SimulatedEnvironment.StringFromProjection(Rhino.Render.SimulatedEnvironment.BackgroundProjections)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedEnvironment.StringFromProjection(Rhino.Render.SimulatedEnvironment.BackgroundProjections)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedEnvironment_StringFromProjection.htm)

### Properties
- `BackgroundColor` (Color, get/set) — 
- `BackgroundImage` (SimulatedTexture, get/set) — 
- `BackgroundProjection` (SimulatedEnvironment.BackgroundProjections, get/set) — 

## SimulatedEnvironment.BackgroundProjections (enum)

[Missing <summary> documentation for "T:Rhino.Render.SimulatedEnvironment.BackgroundProjections"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SimulatedEnvironment_BackgroundProjections.htm)

### Values
- `Planar` = `0`
- `Spherical` = `1`
- `Emap` = `2`
- `Box` = `3`
- `Automatic` = `4`
- `Lightprobe` = `5`
- `Cubemap` = `6`
- `VerticalCrossCubemap` = `7`
- `HorizontalCrossCubemap` = `8`

## SimulatedTexture (class)

[Missing <summary> documentation for "T:Rhino.Render.SimulatedTexture"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SimulatedTexture.htm)

### Constructors
- `[ObsoleteAttribute] public SimulatedTexture()` — Initializes a new instance of the SimulatedTexture class
- `public SimulatedTexture(RhinoDoc doc)` — Initializes a new instance of the SimulatedTexture class
- `[ObsoleteAttribute] public SimulatedTexture(Texture texture)` — Initializes a new instance of the SimulatedTexture class
- `public SimulatedTexture(RhinoDoc doc, Texture texture)` — Initializes a new instance of the SimulatedTexture class

### Methods
#### `public IntPtr ConstPointer()`



**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedTexture.ConstPointer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_ConstPointer.htm)

#### `public void Dispose()`

Releases all resources used by the SimulatedTexture

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the SimulatedTexture and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_Finalize.htm)

#### `[ObsoleteAttribute("Obsolete, use version that requires a document")] public double MetersToUnits(double units)`

Obsolete.

**Parameters:**
- `units` (System.Double) — [Missing <param name="units"/> documentation for "M:Rhino.Render.SimulatedTexture.MetersToUnits(System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedTexture.MetersToUnits(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_MetersToUnits_1.htm)

#### `public double MetersToUnits(RhinoDoc doc, double units)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.SimulatedTexture.MetersToUnits(Rhino.RhinoDoc,System.Double)"]
- `units` (System.Double) — [Missing <param name="units"/> documentation for "M:Rhino.Render.SimulatedTexture.MetersToUnits(Rhino.RhinoDoc,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedTexture.MetersToUnits(Rhino.RhinoDoc,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_MetersToUnits.htm)

#### `public void SetMappingChannelAndProjectionMode(SimulatedTexture.ProjectionModes pm, int mappingChannel, SimulatedTexture.EnvironmentMappingModes emm)`



**Parameters:**
- `pm` (Rhino.Render.SimulatedTexture.ProjectionModes) — [Missing <param name="pm"/> documentation for "M:Rhino.Render.SimulatedTexture.SetMappingChannelAndProjectionMode(Rhino.Render.SimulatedTexture.ProjectionModes,System.Int32,Rhino.Render.SimulatedTexture.EnvironmentMappingModes)"]
- `mappingChannel` (System.Int32) — [Missing <param name="mappingChannel"/> documentation for "M:Rhino.Render.SimulatedTexture.SetMappingChannelAndProjectionMode(Rhino.Render.SimulatedTexture.ProjectionModes,System.Int32,Rhino.Render.SimulatedTexture.EnvironmentMappingModes)"]
- `emm` (Rhino.Render.SimulatedTexture.EnvironmentMappingModes) — [Missing <param name="emm"/> documentation for "M:Rhino.Render.SimulatedTexture.SetMappingChannelAndProjectionMode(Rhino.Render.SimulatedTexture.ProjectionModes,System.Int32,Rhino.Render.SimulatedTexture.EnvironmentMappingModes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_SetMappingChannelAndProjectionMode.htm)

#### `public Texture Texture()`



**Returns:** `Texture` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedTexture.Texture"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_Texture.htm)

#### `[ObsoleteAttribute("Obsolete, use version that requires a document")] public double UnitsToMeters(double units)`

Obsolete.

**Parameters:**
- `units` (System.Double) — [Missing <param name="units"/> documentation for "M:Rhino.Render.SimulatedTexture.UnitsToMeters(System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedTexture.UnitsToMeters(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_UnitsToMeters_1.htm)

#### `public double UnitsToMeters(RhinoDoc doc, double units)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.SimulatedTexture.UnitsToMeters(Rhino.RhinoDoc,System.Double)"]
- `units` (System.Double) — [Missing <param name="units"/> documentation for "M:Rhino.Render.SimulatedTexture.UnitsToMeters(Rhino.RhinoDoc,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.SimulatedTexture.UnitsToMeters(Rhino.RhinoDoc,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SimulatedTexture_UnitsToMeters.htm)

### Properties
- `BitmapSize` (Int32, get/set) — 
- `Filename` (String, get/set) — 
- `Filtered` (Boolean, get/set) — 
- `HasTransparentColor` (Boolean, get/set) — 
- `LocalMappingTransform` (Transform, get) — 
- `MappingChannel` (Int32, get/set) — 
- `Offset` (Vector2d, get/set) — 
- `OriginalFilename` (String, get) — 
- `ProjectionMode` (SimulatedTexture.ProjectionModes, get/set) — 
- `Repeat` (Vector2d, get/set) — 
- `Repeating` (Boolean, get/set) — 
- `Rotation` (Double, get/set) — 
- `TransparentColor` (Color4f, get/set) — 
- `TransparentColorSensitivity` (Double, get/set) — 

## SimulatedTexture.EnvironmentMappingModes (enum)

[Missing <summary> documentation for "T:Rhino.Render.SimulatedTexture.EnvironmentMappingModes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SimulatedTexture_EnvironmentMappingModes.htm)

### Values
- `Automatic` = `0`
- `Spherical` = `1`
- `Emap` = `2`
- `Box` = `3`
- `Lightprobe` = `5`
- `Cubemap` = `6`
- `VerticalCrossCubemap` = `7`
- `HorizontalCrossCubemap` = `8`
- `Hemispherical` = `9`

## SimulatedTexture.ProjectionModes (enum)

[Missing <summary> documentation for "T:Rhino.Render.SimulatedTexture.ProjectionModes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SimulatedTexture_ProjectionModes.htm)

### Values
- `MappingChannel` = `0`
- `View` = `1`
- `Wcs` = `2`
- `Emap` = `3`
- `WcsBox` = `4`
- `Screen` = `5`

## Skylight (class)

[Missing <summary> documentation for "T:Rhino.Render.Skylight"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Skylight.htm)

### Constructors
- `public Skylight()` — Create an utility object not associated with any document
- `public Skylight(Skylight src)` — Create an utility object not associated with any document from another object

### Methods
#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.Skylight.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Skylight_CopyFrom.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

### Properties
- `CustomEnvironment` (Guid, get/set) — 
- `CustomEnvironmentOn` (Boolean, get/set) — 
- `Enabled` (Boolean, get/set) — 
- `ShadowIntensity` (Double, get/set) — 

### Events
#### `Changed` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> Changed`

This event is raised when a Skylight property value is changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_Skylight_Changed.htm)

## Sun (class)

Represents the Sun on a little portion of Earth.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Sun.htm)

### Constructors
- `public Sun()` — Create a non-document controlled Sun

### Methods
#### `public static double AltitudeFromValues(double latitude, double longitude, double timezoneHours, int daylightMinutes, DateTime when, double hours, bool fast = false)`



**Parameters:**
- `latitude` (System.Double) — [Missing <param name="latitude"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]
- `longitude` (System.Double) — [Missing <param name="longitude"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]
- `timezoneHours` (System.Double) — [Missing <param name="timezoneHours"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]
- `daylightMinutes` (System.Int32) — [Missing <param name="daylightMinutes"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]
- `when` (System.DateTime) — [Missing <param name="when"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]
- `hours` (System.Double) — [Missing <param name="hours"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]
- `fast` (System.Boolean) — [Missing <param name="fast"/> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.Sun.AltitudeFromValues(System.Double,System.Double,System.Double,System.Int32,System.DateTime,System.Double,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_AltitudeFromValues.htm)

#### `public void BeginChange(RenderContent.ChangeContexts cc)`

Call this function before making any change to this object (calling a setter) otherwise undo will not work correctly. Calls to BeginChange must be paired with a call to EndChange.

**Parameters:**
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Change context

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_BeginChange.htm)

#### `public static Color ColorFromAltitude(double altitudeDegrees)`

Get sun color based on altitude.

**Parameters:**
- `altitudeDegrees` (System.Double) — The altitude sun angle in degrees.

**Returns:** `Color` — Returns color for altitude.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_ColorFromAltitude.htm)

#### `public override void CopyFrom(FreeFloatingBase src)`

(Overrides FreeFloatingBase.CopyFrom(FreeFloatingBase).)

**Parameters:**
- `src` (Rhino.Render.FreeFloatingBase) — [Missing <param name="src"/> documentation for "M:Rhino.Render.Sun.CopyFrom(Rhino.Render.FreeFloatingBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_CopyFrom.htm)

#### `public void Dispose()`

Releases all resources used by the Sun

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_Dispose.htm)

#### `public bool EndChange()`

See BeginChange

**Returns:** `Boolean` — true if the object has returned to no-changes mode.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_DocumentOrFreeFloatingBase_EndChange.htm)

#### `protected override void Finalize()`

(Overrides FreeFloatingBase.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_Finalize.htm)

#### `public DateTime GetDateTime(DateTimeKind kind)`



**Parameters:**
- `kind` (System.DateTimeKind) — [Missing <param name="kind"/> documentation for "M:Rhino.Render.Sun.GetDateTime(System.DateTimeKind)"]

**Returns:** `DateTime` — [Missing <returns> documentation for "M:Rhino.Render.Sun.GetDateTime(System.DateTimeKind)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_GetDateTime.htm)

#### `public static bool Here(out double latitude, out double longitude)`



**Parameters:**
- `latitude` (System.Double) — [Missing <param name="latitude"/> documentation for "M:Rhino.Render.Sun.Here(System.Double@,System.Double@)"]
- `longitude` (System.Double) — [Missing <param name="longitude"/> documentation for "M:Rhino.Render.Sun.Here(System.Double@,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.Sun.Here(System.Double@,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_Here.htm)

#### `public static double JulianDay(double timezoneHours, int daylightMinutes, DateTime when, double hours)`



**Parameters:**
- `timezoneHours` (System.Double) — [Missing <param name="timezoneHours"/> documentation for "M:Rhino.Render.Sun.JulianDay(System.Double,System.Int32,System.DateTime,System.Double)"]
- `daylightMinutes` (System.Int32) — [Missing <param name="daylightMinutes"/> documentation for "M:Rhino.Render.Sun.JulianDay(System.Double,System.Int32,System.DateTime,System.Double)"]
- `when` (System.DateTime) — [Missing <param name="when"/> documentation for "M:Rhino.Render.Sun.JulianDay(System.Double,System.Int32,System.DateTime,System.Double)"]
- `hours` (System.Double) — [Missing <param name="hours"/> documentation for "M:Rhino.Render.Sun.JulianDay(System.Double,System.Int32,System.DateTime,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.Sun.JulianDay(System.Double,System.Int32,System.DateTime,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_JulianDay.htm)

#### `public void SetDateTime(DateTime time, DateTimeKind kind)`



**Parameters:**
- `time` (System.DateTime) — [Missing <param name="time"/> documentation for "M:Rhino.Render.Sun.SetDateTime(System.DateTime,System.DateTimeKind)"]
- `kind` (System.DateTimeKind) — [Missing <param name="kind"/> documentation for "M:Rhino.Render.Sun.SetDateTime(System.DateTime,System.DateTimeKind)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_SetDateTime.htm)

#### `public void SetPosition(DateTime when, double latitudeDegrees, double longitudeDegrees)`

Sets position of the sun based on physical location and time.

**Parameters:**
- `when` (System.DateTime) — A DateTime instance. If the date Kind is DateTimeKind.Local, or DateTimeKind.Unspecified, the date is considered local.
- `latitudeDegrees` (System.Double) — The latitude, in degrees, of the location on Earth.
- `longitudeDegrees` (System.Double) — The longitude, in degrees, of the location on Earth.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_SetPosition.htm)

#### `public void SetPosition(double azimuthDegrees, double altitudeDegrees)`

Sets position of the Sun based on azimuth and altitude values. Using this function will also set sun to manual.

**Parameters:**
- `azimuthDegrees` (System.Double) — The azimuth sun angle in degrees.
- `altitudeDegrees` (System.Double) — The altitude sun angle in degrees.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_SetPosition_1.htm)

#### `public static Vector3d SunDirection(double latitude, double longitude, DateTime when)`



**Parameters:**
- `latitude` (System.Double) — [Missing <param name="latitude"/> documentation for "M:Rhino.Render.Sun.SunDirection(System.Double,System.Double,System.DateTime)"]
- `longitude` (System.Double) — [Missing <param name="longitude"/> documentation for "M:Rhino.Render.Sun.SunDirection(System.Double,System.Double,System.DateTime)"]
- `when` (System.DateTime) — [Missing <param name="when"/> documentation for "M:Rhino.Render.Sun.SunDirection(System.Double,System.Double,System.DateTime)"]

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.Sun.SunDirection(System.Double,System.Double,System.DateTime)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_SunDirection.htm)

#### `public static double TwilightZone()`



**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.Sun.TwilightZone"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Sun_TwilightZone.htm)

### Properties
- `Altitude` (Double, get) — Get the altitude for the sun. To set use SetPosition(azimuth, altitude)
- `Azimuth` (Double, get) — Get the azimuth for the sun. To set use SetPosition(azimuth, altitude)
- `DaylightSaving` (Boolean, get/set) — Daylight savings time
- `DaylightSavingMinutes` (Int32, get/set) — Daylight saving minutes
- `Enabled` (Boolean, get/set) — Turn the sun on/off in this document.
- `Intensity` (Double, get/set) — Sun intensity.
- `Latitude` (Double, get) — 
- `Light` (Light, get) — Get a Light which represents the sun. If manual control is in effect, no sun calculation is performed; the function uses the values last used in calls to Azimuth, Altitude or Vector. If manual control is not in effect, the observer's position, date, time, time zone and daylight saving values are used to calculate the position of the sun.
- `Longitude` (Double, get) — 
- `ManualControl` (Boolean, get/set) — Set angles directly or use place/date/time
- `North` (Double, get/set) — Angle in degrees on world X-Y plane that should be considered north in the model. Angle is measured starting at X-Axis and travels counterclockwise. Y-Axis would be a north angle of 90 degrees.
- `SkylightOn` (Boolean, get/set) — Turn skylight on or off.
- `TimeZone` (Double, get/set) — Measured in hours += UTC
- `Vector` (Vector3d, get) — 

### Events
#### `Changed` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> Changed`

This event is raised when a Sun property value is changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_Sun_Changed.htm)

## SupportOptions (class)

[Missing <summary> documentation for "T:Rhino.Render.SupportOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SupportOptions.htm)

### Methods
#### `public static bool AlwaysShowSunPreview()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.AlwaysShowSunPreview"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_AlwaysShowSunPreview.htm)

#### `public static int AutoSaveKeepAmount()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.AutoSaveKeepAmount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_AutoSaveKeepAmount.htm)

#### `public static bool AutoSaveRenderings()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.AutoSaveRenderings"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_AutoSaveRenderings.htm)

#### `public static bool CheckSupportFilesBeforeRendering()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.CheckSupportFilesBeforeRendering"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_CheckSupportFilesBeforeRendering.htm)

#### `public static bool CombineEditors()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.CombineEditors"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_CombineEditors.htm)

#### `public static string CustomLibraryPath()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.CustomLibraryPath"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_CustomLibraryPath.htm)

#### `public static string CustomPaths()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.CustomPaths"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_CustomPaths.htm)

#### `public static int DarkPreviewCheckerColor()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.DarkPreviewCheckerColor"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_DarkPreviewCheckerColor.htm)

#### `public static bool EnablePreviewJobLog()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.EnablePreviewJobLog"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_EnablePreviewJobLog.htm)

#### `public static bool HarvestContentParameters()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.HarvestContentParameters"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_HarvestContentParameters.htm)

#### `public static int LabelFormatLoc()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.LabelFormatLoc"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_LabelFormatLoc.htm)

#### `public static int LabelFormatUtc()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.LabelFormatUtc"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_LabelFormatUtc.htm)

#### `public static string LastNavigatedLocation()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.LastNavigatedLocation"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_LastNavigatedLocation.htm)

#### `public static SupportOptions.RdkInitialLocation LibrariesInitialLocation()`



**Returns:** `SupportOptions.RdkInitialLocation` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.LibrariesInitialLocation"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_LibrariesInitialLocation.htm)

#### `public static string LibrariesInitialLocationCustomFolder()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.LibrariesInitialLocationCustomFolder"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_LibrariesInitialLocationCustomFolder.htm)

#### `public static int LightPreviewCheckerColor()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.LightPreviewCheckerColor"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_LightPreviewCheckerColor.htm)

#### `public static int MaxPreviewCacheMB()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.MaxPreviewCacheMB"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_MaxPreviewCacheMB.htm)

#### `public static int MaxPreviewSeconds()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.MaxPreviewSeconds"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_MaxPreviewSeconds.htm)

#### `public static bool MultithreadedTextureEvaluation()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.MultithreadedTextureEvaluation"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_MultithreadedTextureEvaluation.htm)

#### `public static bool PreferNativeRenderer()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.PreferNativeRenderer"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_PreferNativeRenderer.htm)

#### `public static string PreferredUnpackFolder()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.PreferredUnpackFolder"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_PreferredUnpackFolder.htm)

#### `public static bool PreviewCustomRenderMeshes()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.PreviewCustomRenderMeshes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_PreviewCustomRenderMeshes.htm)

#### `public static void SetAlwaysShowSunPreview(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetAlwaysShowSunPreview(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetAlwaysShowSunPreview.htm)

#### `public static void SetAutoSaveKeepAmount(int value)`



**Parameters:**
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Render.SupportOptions.SetAutoSaveKeepAmount(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetAutoSaveKeepAmount.htm)

#### `public static void SetAutoSaveRenderings(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetAutoSaveRenderings(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetAutoSaveRenderings.htm)

#### `public static void SetCheckSupportFilesBeforeRendering(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetCheckSupportFilesBeforeRendering(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetCheckSupportFilesBeforeRendering.htm)

#### `public static void SetCombineEditors(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetCombineEditors(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetCombineEditors.htm)

#### `public static void SetCustomLibraryPath(string path)`



**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.Render.SupportOptions.SetCustomLibraryPath(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetCustomLibraryPath.htm)

#### `public static void SetCustomPaths(string path)`



**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.Render.SupportOptions.SetCustomPaths(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetCustomPaths.htm)

#### `public static void SetHarvestContentParameters(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetHarvestContentParameters(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetHarvestContentParameters.htm)

#### `public static void SetLabelFormatLoc(int value)`



**Parameters:**
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Render.SupportOptions.SetLabelFormatLoc(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetLabelFormatLoc.htm)

#### `public static void SetLabelFormatUtc(int value)`



**Parameters:**
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Render.SupportOptions.SetLabelFormatUtc(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetLabelFormatUtc.htm)

#### `public static void SetLastNavigatedLocation(string folder)`



**Parameters:**
- `folder` (System.String) — [Missing <param name="folder"/> documentation for "M:Rhino.Render.SupportOptions.SetLastNavigatedLocation(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetLastNavigatedLocation.htm)

#### `public static void SetLibrariesInitialLocation(SupportOptions.RdkInitialLocation l)`



**Parameters:**
- `l` (Rhino.Render.SupportOptions.RdkInitialLocation) — [Missing <param name="l"/> documentation for "M:Rhino.Render.SupportOptions.SetLibrariesInitialLocation(Rhino.Render.SupportOptions.RdkInitialLocation)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetLibrariesInitialLocation.htm)

#### `public static void SetLibrariesInitialLocationCustomFolder(string path)`



**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.Render.SupportOptions.SetLibrariesInitialLocationCustomFolder(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetLibrariesInitialLocationCustomFolder.htm)

#### `public static void SetMultithreadedTextureEvaluation(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetMultithreadedTextureEvaluation(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetMultithreadedTextureEvaluation.htm)

#### `public static void SetPreferNativeRenderer(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetPreferNativeRenderer(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetPreferNativeRenderer.htm)

#### `public static void SetPreferredUnpackFolder(string path)`



**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.Render.SupportOptions.SetPreferredUnpackFolder(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetPreferredUnpackFolder.htm)

#### `public static void SetPreviewCustomRenderMeshes(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetPreviewCustomRenderMeshes(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetPreviewCustomRenderMeshes.htm)

#### `public static void SetShowCustom(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetShowCustom(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetShowCustom.htm)

#### `public static void SetShowDetailsPanel(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetShowDetailsPanel(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetShowDetailsPanel.htm)

#### `public static void SetShowDocuments(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetShowDocuments(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetShowDocuments.htm)

#### `public static void SetShowRenderContent(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetShowRenderContent(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetShowRenderContent.htm)

#### `public static void SetSupportSharedUIs(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetSupportSharedUIs(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetSupportSharedUIs.htm)

#### `[ObsoleteAttribute("Support for changing the texture size programatically will disappear in a future version of Rhino")] public static void SetTextureSize(SupportOptions.RdkTextureSize size, bool bSendEvent)`

Obsolete.

**Parameters:**
- `size` (Rhino.Render.SupportOptions.RdkTextureSize) — [Missing <param name="size"/> documentation for "M:Rhino.Render.SupportOptions.SetTextureSize(Rhino.Render.SupportOptions.RdkTextureSize,System.Boolean)"]
- `bSendEvent` (System.Boolean) — [Missing <param name="bSendEvent"/> documentation for "M:Rhino.Render.SupportOptions.SetTextureSize(Rhino.Render.SupportOptions.RdkTextureSize,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetTextureSize.htm)

#### `public static void SetUseDefaultLibraryPath(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetUseDefaultLibraryPath(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetUseDefaultLibraryPath.htm)

#### `public static void SetUsePreviewCache(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetUsePreviewCache(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetUsePreviewCache.htm)

#### `public static void SetUseQuickInitialPreview(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.Render.SupportOptions.SetUseQuickInitialPreview(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SetUseQuickInitialPreview.htm)

#### `public static bool ShowCustom()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.ShowCustom"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_ShowCustom.htm)

#### `public static bool ShowDetailsPanel()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.ShowDetailsPanel"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_ShowDetailsPanel.htm)

#### `public static bool ShowDocuments()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.ShowDocuments"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_ShowDocuments.htm)

#### `public static bool ShowRenderContent()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.ShowRenderContent"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_ShowRenderContent.htm)

#### `public static bool SupportSharedUIs()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.SupportSharedUIs"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SupportSharedUIs.htm)

#### `public static bool SupportSharedUIsNoCache()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.SupportSharedUIsNoCache"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_SupportSharedUIsNoCache.htm)

#### `public static int TextureSize()`



**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.TextureSize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_TextureSize.htm)

#### `public static bool UseDefaultLibraryPath()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.UseDefaultLibraryPath"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_UseDefaultLibraryPath.htm)

#### `public static bool UsePreview()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.UsePreview"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_UsePreview.htm)

#### `public static bool UsePreviewCache()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.UsePreviewCache"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_UsePreviewCache.htm)

#### `public static bool UseQuickInitialPreview()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.UseQuickInitialPreview"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_UseQuickInitialPreview.htm)

#### `public static bool UseRenderedPreview()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.SupportOptions.UseRenderedPreview"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_SupportOptions_UseRenderedPreview.htm)

## SupportOptions.RdkInitialLocation (enum)

[Missing <summary> documentation for "T:Rhino.Render.SupportOptions.RdkInitialLocation"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SupportOptions_RdkInitialLocation.htm)

### Values
- `RenderContent` = `0`
- `LastOpenedFolder` = `1`
- `CustomFolder` = `2`

## SupportOptions.RdkTextureSize (enum)

[Missing <summary> documentation for "T:Rhino.Render.SupportOptions.RdkTextureSize"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_SupportOptions_RdkTextureSize.htm)

### Values
- `Size1` = `128`
- `Size2` = `256`
- `Size3` = `512`
- `Size4` = `1024`
- `Size5` = `2048`

## TextureEnvironmentMappingMode (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureEnvironmentMappingMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureEnvironmentMappingMode.htm)

### Values
- `Automatic` = `0`
- `Spherical` = `1`
- `EnvironmentMap` = `2`
- `Box` = `3`
- `LightProbe` = `5`
- `Cube` = `6`
- `VerticalCrossCube` = `7`
- `HorizontalCrossCube` = `8`
- `Hemispherical` = `9`

## TextureEvaluator (class)

This is the interface to a lightweight object capable of evaluating texture color throughout uvw space. Derive from this class to create your own texture evaluator to return from a custom RenderTexture.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureEvaluator.htm)

### Constructors
- `[ObsoleteAttribute] protected TextureEvaluator()` — Base class constructor
- `protected TextureEvaluator(RenderTexture.TextureEvaluatorFlags evaluatorFlags)` — Base class constructor

### Methods
#### `public void Dispose()`

For Dispose pattern

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For Dispose pattern

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_Dispose_1.htm)

#### `protected override void Finalize()`

For Dispose pattern

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_Finalize.htm)

#### `public virtual Color4f GetColor(Point3d uvw, Vector3d duvwdx, Vector3d duvwdy)`

Get the color of the texture at a particular point in uvw space. May be called from within a rendering shade pipeline. note For ray differentials see Pharr Humphreys, "Physically Based Rendering", chapter 11.

**Parameters:**
- `uvw` (Rhino.Geometry.Point3d) — is the point for which to evaluate the texture.
- `duvwdx` (Rhino.Geometry.Vector3d) — duvwdx is a ray differential.
- `duvwdy` (Rhino.Geometry.Vector3d) — duvwdy is a ray differential.

**Returns:** `Color4f` — The texture color at this point in UV space.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_GetColor.htm)

#### `public bool GetColor(Point3d uvw, Vector3d duvwdx, Vector3d duvwdy, ref Color4f color)`

Optimized version of GetColor for callers. Much faster in the case of a native (C++) evaluator.

**Parameters:**
- `uvw` (Rhino.Geometry.Point3d) — is the point for which to evaluate the texture.
- `duvwdx` (Rhino.Geometry.Vector3d) — duvwdx is a ray differential.
- `duvwdy` (Rhino.Geometry.Vector3d) — duvwdy is a ray differential.
- `color` (Rhino.Display.Color4f) — The texture color at this point in UV space.

**Returns:** `Boolean` — True if the evaluator returned a valid color. Otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_GetColor_1.htm)

#### `public virtual bool Initialize()`

Call this function before calling GetColor for the first time. Ideally, this should be on the main thread, but you can also call it on a worker thread as long as you are sure that Initialize() or GetColor() cannot be called at the same time on another thread.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.TextureEvaluator.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_Initialize.htm)

#### `public virtual SimpleArrayByte WriteToByteArray(int width, int height)`

Fast access to bitmap evaluator - supply size (which you will probably have received from CRhRdkTexture::PixelSize) to see if the data can be extracted direct to a width*height*4 array of unsigned chars.

**Parameters:**
- `width` (System.Int32) — is the point for which to evaluate the texture.
- `height` (System.Int32) — duvwdx is a ray differential.

**Returns:** `SimpleArrayByte` — A ByteArray full of the byte values in RGBA order, or null if the function did not succeed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_WriteToByteArray.htm)

#### `public virtual SimpleArrayFloat WriteToFloatArray(int width, int height)`

Fast access to bitmap evaluator - supply size (which you will probably have received from CRhRdkTexture::PixelSize) to see if the data can be extracted direct to a width*height*4 array of unsigned chars.

**Parameters:**
- `width` (System.Int32) — is the point for which to evaluate the texture.
- `height` (System.Int32) — duvwdx is a ray differential.

**Returns:** `SimpleArrayFloat` — A FloatArray full of the float values in RGBA order, or null if the function did not succeed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureEvaluator_WriteToFloatArray.htm)

## TextureGeneration (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureGeneration"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureGeneration.htm)

### Values
- `Allow` = `1`
- `Disallow` = `2`

## TextureGraphInfo (class)

[Missing <summary> documentation for "T:Rhino.Render.TextureGraphInfo"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureGraphInfo.htm)

### Constructors
- `public TextureGraphInfo()` — Initializes a new instance of the TextureGraphInfo class

### Methods
#### `public TextureGraphInfo.Axis ActiveAxis()`



**Returns:** `TextureGraphInfo.Axis` — [Missing <returns> documentation for "M:Rhino.Render.TextureGraphInfo.ActiveAxis"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_ActiveAxis.htm)

#### `public TextureGraphInfo.Channel ActiveChannel()`



**Returns:** `TextureGraphInfo.Channel` — [Missing <returns> documentation for "M:Rhino.Render.TextureGraphInfo.ActiveChannel"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_ActiveChannel.htm)

#### `public double AmountU()`



**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.TextureGraphInfo.AmountU"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_AmountU.htm)

#### `public double AmountV()`



**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.TextureGraphInfo.AmountV"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_AmountV.htm)

#### `public double AmountW()`



**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.Render.TextureGraphInfo.AmountW"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_AmountW.htm)

#### `public void Dispose()`

Releases all resources used by the TextureGraphInfo

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_Finalize.htm)

#### `public void SetActiveAxis(TextureGraphInfo.Axis axis)`



**Parameters:**
- `axis` (Rhino.Render.TextureGraphInfo.Axis) — [Missing <param name="axis"/> documentation for "M:Rhino.Render.TextureGraphInfo.SetActiveAxis(Rhino.Render.TextureGraphInfo.Axis)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_SetActiveAxis.htm)

#### `public void SetActiveChannel(TextureGraphInfo.Channel channel)`



**Parameters:**
- `channel` (Rhino.Render.TextureGraphInfo.Channel) — [Missing <param name="channel"/> documentation for "M:Rhino.Render.TextureGraphInfo.SetActiveChannel(Rhino.Render.TextureGraphInfo.Channel)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_SetActiveChannel.htm)

#### `public void SetAmountU(double d)`



**Parameters:**
- `d` (System.Double) — [Missing <param name="d"/> documentation for "M:Rhino.Render.TextureGraphInfo.SetAmountU(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_SetAmountU.htm)

#### `public void SetAmountV(double d)`



**Parameters:**
- `d` (System.Double) — [Missing <param name="d"/> documentation for "M:Rhino.Render.TextureGraphInfo.SetAmountV(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_SetAmountV.htm)

#### `public void SetAmountW(double d)`



**Parameters:**
- `d` (System.Double) — [Missing <param name="d"/> documentation for "M:Rhino.Render.TextureGraphInfo.SetAmountW(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureGraphInfo_SetAmountW.htm)

### Properties
- `CppPointer` (IntPtr, get) — 

## TextureGraphInfo.Axis (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureGraphInfo.Axis"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureGraphInfo_Axis.htm)

### Values
- `kU` = `0`
- `kV` = `1`
- `kW` = `2`

## TextureGraphInfo.Channel (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureGraphInfo.Channel"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureGraphInfo_Channel.htm)

### Values
- `kRed` = `0`
- `kGrn` = `1`
- `kBlu` = `2`
- `kAlp` = `3`
- `kLum` = `4`

## TextureMapping (class)

Represents a texture mapping.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureMapping.htm)

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

#### `public static TextureMapping CreateBoxMapping(Plane plane, Interval dx, Interval dy, Interval dz, bool capped)`

Create a box projection texture mapping.

**Remarks:** When m_texture_space = divided, the box is mapped to texture space as follows: If the box is not capped, then each side maps to 1/4 of the texture map. v=1+---------+---------+---------+---------+ | x=dx[1] | y=dy[1] | x=dx[0] | y=dy[0] | | Front | Right | Back | Left | | --y-> | <-x-- | <-y-- | --x-> | v=0+---------+---------+---------+---------+ 0/4 <=u<= 1/4 <=u<= 2/4 <=u<= 3/4 <=u<= 4/4 If the box is capped, then each side and cap gets 1/6 of the texture map. v=1+---------+---------+---------+---------+---------+---------+ | x=dx[1] | y=dy[1] | x=dx[0] | y=dy[0] | z=dx[1] | z=dz[0] | | Front | Right | Back | Left | Top | Bottom | | --y-> | <x-- | <-y-- | --x-> | --x-> | --x-> | v=0+---------+---------+---------+---------+---------+---------+ 0/6 <=u<= 1/6 <=u<= 2/6 <=u<= 3/6 <=u<= 4/6 <=u<= 5/6 <=u<= 6/6

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The sides of the box the box are parallel to the plane's coordinate planes. The dx, dy, dz intervals determine the location of the sides.
- `dx` (Rhino.Geometry.Interval) — Determines the location of the front and back planes. The vector plane.xaxis is perpendicular to these planes and they pass through plane.PointAt(dx[0],0,0) and plane.PointAt(dx[1],0,0), respectively.
- `dy` (Rhino.Geometry.Interval) — Determines the location of the left and right planes. The vector plane.yaxis is perpendicular to these planes and they pass through plane.PointAt(0,dy[0],0) and plane.PointAt(0,dy[1],0), respectively.
- `dz` (Rhino.Geometry.Interval) — Determines the location of the top and bottom planes. The vector plane.zaxis is perpendicular to these planes and they pass through plane.PointAt(0,0,dz[0]) and plane.PointAt(0,0,dz[1]), respectively.
- `capped` (System.Boolean) — If true, the box is treated as a finite capped box.

**Returns:** `TextureMapping` — TextureMapping instance if input is valid

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreateBoxMapping.htm)

#### `public static TextureMapping CreateCustomMeshMapping(Mesh mesh)`

Create custom mesh mapping

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh with texture coordinates

**Returns:** `TextureMapping` — TextureMapping instance

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreateCustomMeshMapping.htm)

#### `public static TextureMapping CreateCylinderMapping(Cylinder cylinder, bool capped)`

Create a cylindrical projection texture mapping.

**Remarks:** When the cylinder is capped and m_texture_space = divided, the cylinder is mapped to texture space as follows: The side is mapped to 0 <= "u" <= 2/3. The bottom is mapped to 2/3 <= "u" <= 5/6. The top is mapped to 5/6 <= "u" <= 5/6. This is the same convention box mapping uses.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — cylinder in world space used to define a cylindrical coordinate system. The angular parameter maps (0,2pi) to texture "u" (0,1), The height parameter maps (height[0],height[1]) to texture "v" (0,1), and the radial parameter maps (0,r) to texture "w" (0,1).
- `capped` (System.Boolean) — If true, the cylinder is treated as a finite capped cylinder

**Returns:** `TextureMapping` — TextureMapping instance if input is valid

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreateCylinderMapping.htm)

#### `public static TextureMapping CreateOcsMapping(Plane plane)`

Create a Ocs texture mapping. Note that OCS mappings must be placed on mapping channel ON_ObjectRenderingAttributes::OCSMappingChannelId() otherwise they will not work.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane to use for mapping.

**Returns:** `TextureMapping` — [Missing <returns> documentation for "M:Rhino.Render.TextureMapping.CreateOcsMapping(Rhino.Geometry.Plane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreateOcsMapping.htm)

#### `public static TextureMapping CreatePlaneMapping(Plane plane, Interval dx, Interval dy, Interval dz)`

Create a planar UV projection texture mapping

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane to use for mapping.
- `dx` (Rhino.Geometry.Interval) — portion of the plane's x axis that is mapped to [0,1] (can be a decreasing interval)
- `dy` (Rhino.Geometry.Interval) — portion of the plane's y axis that is mapped to [0,1] (can be a decreasing interval)
- `dz` (Rhino.Geometry.Interval) — portion of the plane's z axis that is mapped to [0,1] (can be a decreasing interval)

**Returns:** `TextureMapping` — TextureMapping instance if input is valid

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreatePlaneMapping.htm)

#### `public static TextureMapping CreatePlaneMapping(Plane plane, Interval dx, Interval dy, Interval dz, bool capped)`

Create a planar projection texture mapping

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane to use for mapping.
- `dx` (Rhino.Geometry.Interval) — portion of the plane's x axis that is mapped to [0,1] (can be a decreasing interval)
- `dy` (Rhino.Geometry.Interval) — portion of the plane's y axis that is mapped to [0,1] (can be a decreasing interval)
- `dz` (Rhino.Geometry.Interval) — portion of the plane's z axis that is mapped to [0,1] (can be a decreasing interval)
- `capped` (System.Boolean) — set to true if planar UVW is meant, false for planar UV

**Returns:** `TextureMapping` — TextureMapping instance if input is valid

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreatePlaneMapping_1.htm)

#### `public static TextureMapping CreateSphereMapping(Sphere sphere)`

Create a spherical projection texture mapping.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — sphere in world space used to define a spherical coordinate system. The longitude parameter maps (0,2pi) to texture "u" (0,1). The latitude parameter maps (-pi/2,+pi/2) to texture "v" (0,1). The radial parameter maps (0,r) to texture "w" (0,1).

**Returns:** `TextureMapping` — TextureMapping instance if input is valid

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreateSphereMapping.htm)

#### `public static TextureMapping CreateSurfaceParameterMapping()`

Create a mapping that will convert surface parameters into normalized(0,1)x(0,1) texture coordinates.

**Returns:** `TextureMapping` — TextureMapping instance or null if failed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_CreateSurfaceParameterMapping.htm)

#### `public uint DataCRC(uint currentRemainder)`

Increments the Cyclic Redundancy Check value by this instance.

**Parameters:**
- `currentRemainder` (System.UInt32) — The current remainder value.

**Returns:** `UInt32` — The updated remainder value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_ModelComponent_DataCRC.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_Dispose.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_EnsurePrivateCopy.htm)

#### `public int Evaluate(Point3d p, Vector3d n, out Point3d t)`

Evaluate the mapping to get a texture coordinate

**Parameters:**
- `p` (Rhino.Geometry.Point3d) — Vertex location
- `n` (Rhino.Geometry.Vector3d) — If the mapping projection is ray_projection, then this is the vertex unit normal. Otherwise n is ignored.
- `t` (Rhino.Geometry.Point3d) — Texture coordinate (u,v,w)

**Returns:** `Int32` — Nonzero if evaluation is successful. When the mapping is a box or capped cylinder mapping, the value indicates which side was evaluated. Cylinder mapping: 1 = cylinder wall, 2 = bottom cap, 3 = top cap Box mapping: 1 = front, 2 = right, 3 = back, 4 = left, 5 = bottom, 6 = top

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_Evaluate.htm)

#### `public int Evaluate(Point3d p, Vector3d n, out Point3d t, Transform pXform, Transform nXform)`

Evaluate the mapping to get a texture coordinate

**Parameters:**
- `p` (Rhino.Geometry.Point3d) — Vertex location
- `n` (Rhino.Geometry.Vector3d) — If the mapping projection is ray_projection, then this is the vertex unit normal. Otherwise n is ignored.
- `t` (Rhino.Geometry.Point3d) — Texture coordinate (u,v,w)
- `pXform` (Rhino.Geometry.Transform) — Transformation to be applied to P before performing the mapping calculation.
- `nXform` (Rhino.Geometry.Transform) — Transformation to be applied to N before performing the mapping calculation. One way to calculate nXxform is to use the call pXform::GetVectorTransform(nXform).

**Returns:** `Int32` — Nonzero if evaluation is successful. When the mapping is a box or capped cylinder mapping, the value indicates which side was evaluated. Cylinder mapping: 1 = cylinder wall, 2 = bottom cap, 3 = top cap Box mapping: 1 = front, 2 = right, 3 = back, 4 = left, 5 = bottom, 6 = top

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_Evaluate_1.htm)

#### `public virtual void GetObjectData(SerializationInfo info, StreamingContext context)`

Populates a System.Runtime.Serialization.SerializationInfo with the data needed to serialize the target object.

**Parameters:**
- `info` (System.Runtime.Serialization.SerializationInfo) — The System.Runtime.Serialization.SerializationInfo to populate with data.
- `context` (System.Runtime.Serialization.StreamingContext) — The destination (see System.Runtime.Serialization.StreamingContext) for this serialization.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Runtime_CommonObject_GetObjectData.htm)

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

#### `public bool TryGetMappingBox(out Plane plane, out Interval dx, out Interval dy, out Interval dz)`

Get a box projection from the texture mapping.

**Remarks:** Generally, GetMappingBox will not return the same parameters passed to SetBoxMapping. However, the location of the box will be the same.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The center of the box is at plane.origin and the sides of the box are parallel to the plane's coordinate planes.
- `dx` (Rhino.Geometry.Interval) — The "front" and "back" sides of the box are in spanned by the vectors plane.yaxis and plane.zaxis. The back plane contains the point plane.PointAt(dx[0],0,0) and the front plane contains the point plane.PointAt(dx[1],0,0).
- `dy` (Rhino.Geometry.Interval) — The "left" and "right" sides of the box are in spanned by the vectors plane.zaxis and plane.xaxis. The left plane contains the point plane.PointAt(0,dx[0],0) and the back plane contains the point plane.PointAt(0,dy[1],0).
- `dz` (Rhino.Geometry.Interval) — The "top" and "bottom" sides of the box are in spanned by the vectors plane.xaxis and plane.yaxis. The bottom plane contains the point plane.PointAt(0,0,dz[0]) and the top plane contains the point plane.PointAt(0,0,dz[1]).

**Returns:** `Boolean` — Returns true if a valid box is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingBox.htm)

#### `public bool TryGetMappingBox(out Plane plane, out Interval dx, out Interval dy, out Interval dz, out bool capped)`

Get a box projection from the texture mapping, including capped information

**Remarks:** Generally, GetMappingBox will not return the same parameters passed to SetBoxMapping. However, the location of the box will be the same.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The center of the box is at plane.origin and the sides of the box are parallel to the plane's coordinate planes.
- `dx` (Rhino.Geometry.Interval) — The "front" and "back" sides of the box are in spanned by the vectors plane.yaxis and plane.zaxis. The back plane contains the point plane.PointAt(dx[0],0,0) and the front plane contains the point plane.PointAt(dx[1],0,0).
- `dy` (Rhino.Geometry.Interval) — The "left" and "right" sides of the box are in spanned by the vectors plane.zaxis and plane.xaxis. The left plane contains the point plane.PointAt(0,dx[0],0) and the back plane contains the point plane.PointAt(0,dy[1],0).
- `dz` (Rhino.Geometry.Interval) — The "top" and "bottom" sides of the box are in spanned by the vectors plane.xaxis and plane.yaxis. The bottom plane contains the point plane.PointAt(0,0,dz[0]) and the top plane contains the point plane.PointAt(0,0,dz[1]).
- `capped` (System.Boolean) — true if box mapping is capped.

**Returns:** `Boolean` — Returns true if a valid box is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingBox_1.htm)

#### `public bool TryGetMappingCylinder(out Cylinder cylinder)`

Get a cylindrical projection parameters from this texture mapping.

**Remarks:** Generally, GetMappingCylinder will not return the same parameters passed to SetCylinderMapping. However, the location of the cylinder will be the same. If this mapping is not cylindrical, the cylinder will approximate the actual mapping primitive.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — [Missing <param name="cylinder"/> documentation for "M:Rhino.Render.TextureMapping.TryGetMappingCylinder(Rhino.Geometry.Cylinder@)"]

**Returns:** `Boolean` — Returns true if a valid cylinder is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingCylinder.htm)

#### `public bool TryGetMappingCylinder(out Cylinder cylinder, out bool capped)`

Get a cylindrical projection parameters from this texture mapping.

**Remarks:** Generally, GetMappingCylinder will not return the same parameters passed to SetCylinderMapping. However, the location of the cylinder will be the same. If this mapping is not cylindrical, the cylinder will approximate the actual mapping primitive.

**Parameters:**
- `cylinder` (Rhino.Geometry.Cylinder) — [Missing <param name="cylinder"/> documentation for "M:Rhino.Render.TextureMapping.TryGetMappingCylinder(Rhino.Geometry.Cylinder@,System.Boolean@)"]
- `capped` (System.Boolean) — will be true if capped

**Returns:** `Boolean` — Returns true if a valid cylinder is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingCylinder_1.htm)

#### `public bool TryGetMappingMesh(out Mesh mesh)`

Get custom mapping mesh from this texture mapping.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.Render.TextureMapping.TryGetMappingMesh(Rhino.Geometry.Mesh@)"]

**Returns:** `Boolean` — True if custom mapping mesh was returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingMesh.htm)

#### `public bool TryGetMappingPlane(out Plane plane, out Interval dx, out Interval dy, out Interval dz)`

Get plane mapping parameters from this texture mapping.

**Remarks:** NOTE WELL: Generally, GetMappingPlane will not return the same parameters passed to SetPlaneMapping. However, the location of the plane will be the same.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Render.TextureMapping.TryGetMappingPlane(Rhino.Geometry.Plane@,Rhino.Geometry.Interval@,Rhino.Geometry.Interval@,Rhino.Geometry.Interval@)"]
- `dx` (Rhino.Geometry.Interval) — Portion of the plane's x axis that is mapped to [0,1]
- `dy` (Rhino.Geometry.Interval) — Portion of the plane's y axis that is mapped to [0,1]
- `dz` (Rhino.Geometry.Interval) — Portion of the plane's z axis that is mapped to [0,1]

**Returns:** `Boolean` — Return true if valid plane mapping parameters were returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingPlane.htm)

#### `public bool TryGetMappingPlane(out Plane plane, out Interval dx, out Interval dy, out Interval dz, out bool capped)`

Get plane mapping parameters from this texture mapping, including capping information

**Remarks:** NOTE WELL: Generally, GetMappingPlane will not return the same parameters passed to SetPlaneMapping. However, the location of the plane will be the same.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.Render.TextureMapping.TryGetMappingPlane(Rhino.Geometry.Plane@,Rhino.Geometry.Interval@,Rhino.Geometry.Interval@,Rhino.Geometry.Interval@,System.Boolean@)"]
- `dx` (Rhino.Geometry.Interval) — Portion of the plane's x axis that is mapped to [0,1]
- `dy` (Rhino.Geometry.Interval) — Portion of the plane's y axis that is mapped to [0,1]
- `dz` (Rhino.Geometry.Interval) — Portion of the plane's z axis that is mapped to [0,1]
- `capped` (System.Boolean) — [Missing <param name="capped"/> documentation for "M:Rhino.Render.TextureMapping.TryGetMappingPlane(Rhino.Geometry.Plane@,Rhino.Geometry.Interval@,Rhino.Geometry.Interval@,Rhino.Geometry.Interval@,System.Boolean@)"]

**Returns:** `Boolean` — Return true if valid plane mapping parameters were returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingPlane_1.htm)

#### `public bool TryGetMappingSphere(out Sphere sphere)`

Get a spherical projection parameters from this texture mapping.

**Remarks:** Generally, GetMappingShere will not return the same parameters passed to SetSphereMapping. However, the location of the sphere will be the same. If this mapping is not cylindrical, the cylinder will approximate the actual mapping primitive.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — ///

**Returns:** `Boolean` — Returns true if a valid sphere is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TextureMapping_TryGetMappingSphere.htm)

### Properties
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns TextureMapping.
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get) — The unique Id for this texture mapping object.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — Gets or sets the model component index attribute.
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `MappingType` (TextureMappingType, get) — Texture mapping type associated with this Mapping object.
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Name` (String, get/set) — Gets or sets the name
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `NormalTransform` (Transform, get/set) — For primitive based mappings, these transformations are used to map the world coordinate (x,y,z) point P and surface normal N before it is projected to the normalized mapping primitive. The surface normal transformation, m_Nxyz, is always calculated from m_Pxyz. It is a runtime setting that is not saved in 3dm files. If m_type is srfp_mapping, then m_Pxyz and m_Nxyz are ignored.
- `PrimativeTransform` (Transform, get/set) — For primitive based mappings, these transformations are used to map the world coordinate (x,y,z) point P and surface normal N before it is projected to the normalized mapping primitive. The surface normal transformation, m_Nxyz, is always calculated from m_Pxyz. It is a runtime setting that is not saved in 3dm files. If m_type is srfp_mapping, then m_Pxyz and m_Nxyz are ignored.
- `PrimitiveTransform` (Transform, get/set) — For primitive based mappings, these transformations are used to map the world coordinate (x,y,z) point P and surface normal N before it is projected to the normalized mapping primitive. The surface normal transformation, m_Nxyz, is always calculated from m_Pxyz. It is a runtime setting that is not saved in 3dm files. If m_type is srfp_mapping, then m_Pxyz and m_Nxyz are ignored.
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.
- `UvwTransform` (Transform, get/set) — Transform applied to mapping coordinate (u,v,w) to convert it into a texture coordinate.

## TextureMappingType (enum)

Defines enumerated constants for mapping types such as planar, cylindrical or spherical.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureMappingType.htm)

### Values
- `None` = `0` — No mapping is selected.
- `SurfaceParameters` = `1` — (u, v) = linear transform of surface params, w = 0.
- `PlaneMapping` = `2` — (u, v, w) = 3d coordinates wrt frame.
- `CylinderMapping` = `3` — (u, v, w) = longitude, height, radius.
- `SphereMapping` = `4` — (u, v, w) = longitude,latitude,radius.
- `BoxMapping` = `5` — Box mapping type.
- `MeshMappingPrimitive` = `6` — Mapping primitive is a mesh.
- `SurfaceMappingPrimitive` = `7` — Mapping primitive is a surface.
- `BrepMappingPrimitive` = `8` — Mapping primitive is a brep.
- `OcsMapping` = `9` — OCS mapping type (WCS/WCS Box with object frame).

## TextureMode (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureMode.htm)

### Values
- `PROJECTION_MODE` = `0`
- `MAPPING_CHANNEL_MODE` = `1`
- `WRAP_TYPE_MODE` = `2`
- `REPEAT_LOCKED_MODE` = `3`
- `OFFSET_LOCKED_MODE` = `4`
- `PREVIEW_IN_3D_MODE` = `5`
- `REPEAT_MODE` = `6`
- `OFFSET_MODE` = `7`
- `ROTATION_MODE` = `8`
- `ENVIRONMENT_MAPPING_MODE` = `9`
- `INTERNAL_ENVIRONMENT_MAPPING_MODE` = `10`
- `PREVIEW_LOCAL_MAPPING_MODE` = `11`
- `DISPLAY_IN_VIEWPORT_MODE` = `12`
- `IS_HDR_CAPABLE_MODE` = `13`
- `IS_LINEAR_MODE` = `14`
- `IS_IMAGE_BASED` = `15`
- `IS_NORMALMAP` = `16`
- `PIXELSIZE_AS_3DVECTOR` = `17`

## TextureProjectionMode (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureProjectionMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureProjectionMode.htm)

### Values
- `MappingChannel` = `0`
- `View` = `1`
- `Wcs` = `2`
- `EnvironmentMap` = `3`
- `WcsBox` = `4`
- `Screen` = `5`

## TextureRenderHashFlags (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureRenderHashFlags"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureRenderHashFlags.htm)

### Values
- `ExcludeLocalMapping` = `1`

## TextureWrapType (enum)

[Missing <summary> documentation for "T:Rhino.Render.TextureWrapType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TextureWrapType.htm)

### Values
- `Clamped` = `0`
- `Repeating` = `1`

## TexturedColor (class)

Color4f specialization of TexturedValue.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TexturedColor.htm)

### Constructors
- `public TexturedColor(string name, Color4f value, bool on, float amount)` — Initializes a new instance of the TexturedColor class

### Properties
- `Amount` (Single, get) — (Inherited from TexturedValue<T>.)
- `Name` (String, get) — (Inherited from TexturedValue<T>.)
- `On` (Boolean, get) — (Inherited from TexturedValue<T>.)
- `Texture` (RenderTexture, get) — (Inherited from TexturedValue<T>.)
- `Value` (T, get) — (Inherited from TexturedValue<T>.)

## TexturedFloat (class)

float specialization of TexturedValue.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TexturedFloat.htm)

### Constructors
- `public TexturedFloat(string name, float value, bool on, float amount)` — Initializes a new instance of the TexturedFloat class

### Properties
- `Amount` (Single, get) — (Inherited from TexturedValue<T>.)
- `Name` (String, get) — (Inherited from TexturedValue<T>.)
- `On` (Boolean, get) — (Inherited from TexturedValue<T>.)
- `Texture` (RenderTexture, get) — (Inherited from TexturedValue<T>.)
- `Value` (T, get) — (Inherited from TexturedValue<T>.)

## TexturedValue<T> (class)

Generic class to help holding on to related values. This can be used to get data from textured content fields with the HandleTexturedValue function.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TexturedValue_1.htm)

### Constructors
- `public TexturedValue(string name, T value, bool on, float amount)` — Initializes a new instance of the TexturedValue<T> class

### Properties
- `Amount` (Single, get) — 
- `Name` (String, get) — 
- `On` (Boolean, get) — 
- `Texture` (RenderTexture, get) — 
- `Value` (T, get) — 

## TimeZone (class)

TimeZone

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TimeZone.htm)

### Methods
#### `public static TimeZone TimeZoneAt(int index)`

Returns a time zone at given index.

**Parameters:**
- `index` (System.Int32) — index.

**Returns:** `TimeZone` — Time zone at index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TimeZone_TimeZoneAt.htm)

#### `public static int TimeZones()`

Returns number of available time zones.

**Returns:** `Int32` — Time zone count.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TimeZone_TimeZones.htm)

### Properties
- `Hours` (Double, get) — Gets hours of a time zone.
- `Latitude` (Double, get) — Returns the latitude of a major city nearby
- `Longitude` (Double, get) — Returns the Longitude of a major city nearby
- `Name` (String, get) — Gets name of a time zone.

## TwoColorRenderTexture (class)

[Missing <summary> documentation for "T:Rhino.Render.TwoColorRenderTexture"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_TwoColorRenderTexture.htm)

### Constructors
- `protected TwoColorRenderTexture()` — Initializes a new instance of the TwoColorRenderTexture class

### Methods
#### `protected abstract void AddAdditionalUISections()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TwoColorRenderTexture_AddAdditionalUISections.htm)

#### `public bool AddAutomaticUserInterfaceSection(string caption, int id)`

Add a new automatic user interface section, Field values which include prompts will be automatically added to this section.

**Parameters:**
- `caption` (System.String) — Expandable tab caption.
- `id` (System.Int32) — Tab id which may be used later on to reference this tab.

**Returns:** `Boolean` — Returns true if the automatic tab section was added otherwise; returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddAutomaticUserInterfaceSection.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool AddChild(RenderContent renderContent, string childSlotName)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — [Missing <param name="renderContent"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddChild(Rhino.Render.RenderContent,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddChild_1.htm)

#### `public bool AddUserInterfaceSection(ICollapsibleSection section)`

(Inherited from RenderContent.)

**Parameters:**
- `section` (Rhino.UI.Controls.ICollapsibleSection) — [Missing <param name="section"/> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection.htm)

#### `[ObsoleteAttribute("Use AddUserInterfaceSection(Rhino.UI.Controls.ICollapsibleSection) below instead. This function will not work on the Mac and is not type-safe.")] public UserInterfaceSection AddUserInterfaceSection(Type classType, string caption, bool createExpanded, bool createVisible)`

Add a new .NET control to an content expandable tab section, the height of the createExpanded tabs client area will be the initial height of the specified control.

**Parameters:**
- `classType` (System.Type) — The control class to create and embed as a child window in the expandable tab client area. This class type must be derived from IWin32Window or this method will throw an ArgumentException. Implement the IUserInterfaceSection interface in your classType to get UserInterfaceSection notification.
- `caption` (System.String) — Expandable tab caption.
- `createExpanded` (System.Boolean) — If this value is true then the new expandable tab section will initially be expanded, if it is false it will be collapsed.
- `createVisible` (System.Boolean) — If this value is true then the new expandable tab section will initially be visible, if it is false it will be hidden.

**Returns:** `UserInterfaceSection` — Returns the UserInterfaceSection object used to manage the new user control object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_AddUserInterfaceSection_1.htm)

#### `public void BeginChange(RenderContent.ChangeContexts changeContext)`

Begins a change or batch of changes. It may also make a copy of the content state allowing EndChange() to send an event with the old and new contents. Calls to this method are counted; you must call EndChange() once for every call to BeginChange(). Note: If Changed() was called between the calls to BeginChange() and EndChange(), the last call to EndChange() may cause the ContentChanged event to be sent.

**Parameters:**
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — the change context. If this is ChangeContexts.UI, ChangeContexts.Program,ChangeContexts.Drop or ChangeContexts.Tree, the content will be copied. EndChange() will then send the copy as 'old' in the OnContentChanged event. EndChange()ContentChanged

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginChange.htm)

#### `public void BeginCreateDynamicFields(bool automatic)`

Automatic Dynamic Field support. Dynamic fields are typically created in the constructor of RenderContent and they are therefore created automatically whenever the content is created. However, some advanced users require the fields to be created in response to some user action which occurs much later. This creates the problem that the fields do not exist by default and therefore cannot be loaded when the document is loaded. These methods are provided to solve that problem by making it possible to automatically create the dynamic fields on loading if they don't already exist. Dynamic fields that have this auto-create-on-load behavior are referred to as automatic dynamic fields. Dynamic fields that do not require the advanced automatic feature can still be created by using these methods (recommended), or they can be created manually (legacy usage). You must call this before creating any dynamic fields. Calls to this method are counted; you must call EndCreateDynamicFields() once for every call to BeginCreateDynamicFields().

**Parameters:**
- `automatic` (System.Boolean) — automatic specifies if the dynamic fields are automatic. If so, they will be created automatically during loading of the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BeginCreateDynamicFields.htm)

#### `public void BindParameterToField(string parameterName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField.htm)

#### `public void BindParameterToField(string parameterName, string childSlotName, Field field, RenderContent.ChangeContexts setEvent)`

Use bindings to automatically wire parameters to fields

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `field` (Rhino.Render.Fields.Field) — [Missing <param name="field"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]
- `setEvent` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="setEvent"/> documentation for "M:Rhino.Render.RenderContent.BindParameterToField(System.String,System.String,Rhino.Render.Fields.Field,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_BindParameterToField_1.htm)

#### `protected virtual uint CalculateRenderHash(ulong rcrcFlags)`

Override this method to calculate the render hash of the state that affects how the content is rendered. Does not include children or perform any caching. Render hash values are now automatically cached by the content framework and you do not have to worry about caching. You also do not have to worry about iterating into children. This method is now only called internally by the framework, use the RenderHash property to get the current hash value.

**Parameters:**
- `rcrcFlags` (System.UInt64) — [Missing <param name="rcrcFlags"/> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CalculateRenderHash(System.UInt64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CalculateRenderHash.htm)

#### `[ObsoleteAttribute("This method is obsolete and simply calls SetChild")] public bool ChangeChild(RenderContent oldContent, RenderContent newContent)`

Obsolete. (Inherited from RenderContent.)

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — [Missing <param name="oldContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]
- `newContent` (Rhino.Render.RenderContent) — [Missing <param name="newContent"/> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.ChangeChild(Rhino.Render.RenderContent,Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChangeChild.htm)

#### `public double ChildSlotAmount(string childSlotName)`

Gets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Double` — The requested amount value. Values are typically from 0.0 to 100.0

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotAmount.htm)

#### `public string ChildSlotNameFromParamName(string paramName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `paramName` (System.String) — The name of a parameter field. Since child textures will usually correspond with some parameter (they generally either replace or modify a parameter over UV space) these functions are used to specify which parameter corresponded with child slot. If there is no correspondence, return the empty string.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotNameFromParamName.htm)

#### `public bool ChildSlotOn(string childSlotName)`

Gets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ChildSlotOn.htm)

#### `public virtual void ConvertUnits(UnitSystem from, UnitSystem to)`

Modify your content so that it is converted from meters into the units of the unit system. No need to call the base class when you override this, and no need to recurse into children.

**Parameters:**
- `from` (Rhino.UnitSystem) — [Missing <param name="from"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]
- `to` (Rhino.UnitSystem) — [Missing <param name="to"/> documentation for "M:Rhino.Render.RenderContent.ConvertUnits(Rhino.UnitSystem,Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ConvertUnits.htm)

#### `public bool CreateDynamicField(string internalName, string localName, string englishName, Object value, Object minValue, Object maxValue, int sectionId)`

Create a dynamic field with an initial value and min and max limits.

**Parameters:**
- `internalName` (System.String) — is the internal name of the field. Not localized.
- `localName` (System.String) — is the localized user-friendly name of the field.
- `englishName` (System.String) — is the English user-friendly name of the field.
- `value` (System.Object) — is the initial value of the field.
- `minValue` (System.Object) — is the minimum value of the field. Must be the same type as vValue.
- `maxValue` (System.Object) — is the maximum value of the field. Must be the same type as vValue.
- `sectionId` (System.Int32) — is used for filtering fields between sections. Zero if not needed.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.CreateDynamicField(System.String,System.String,System.String,System.Object,System.Object,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_CreateDynamicField.htm)

#### `[ObsoleteAttribute("Use version that takes TextureEvaluatorFlags enum")] public virtual TextureEvaluator CreateEvaluator()`

Constructs a texture evaluator. This is an independent lightweight object capable of evaluating texture color throughout uvw space. May be called from within a rendering shade pipeline.

**Returns:** `TextureEvaluator` — A texture evaluator instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_CreateEvaluator.htm)

#### `public virtual TextureEvaluator CreateEvaluator(RenderTexture.TextureEvaluatorFlags evaluatorFlags)`

Constructs a texture evaluator. This is an independent lightweight object capable of evaluating texture color throughout uvw space. May be called from within a rendering shade pipeline.

**Parameters:**
- `evaluatorFlags` (Rhino.Render.RenderTexture.TextureEvaluatorFlags) — [Missing <param name="evaluatorFlags"/> documentation for "M:Rhino.Render.RenderTexture.CreateEvaluator(Rhino.Render.RenderTexture.TextureEvaluatorFlags)"]

**Returns:** `TextureEvaluator` — A texture evaluator instance.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_CreateEvaluator_1.htm)

#### `public void DeleteAllChildren(RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteAllChildren(Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteAllChildren.htm)

#### `public bool DeleteChild(string childSlotName, RenderContent.ChangeContexts changeContexts)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DeleteChild(System.String,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DeleteChild.htm)

#### `public void Dispose()`

(Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Dispose

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Dispose_1.htm)

#### `public virtual bool DynamicIcon(Size size, out Bitmap bitmap, DynamicIconUsage usage)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]
- `usage` (Rhino.Render.DynamicIconUsage) — [Missing <param name="usage"/> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.DynamicIcon(System.Drawing.Size,System.Drawing.Bitmap@,Rhino.Render.DynamicIconUsage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_DynamicIcon.htm)

#### `public RenderContent Edit()`

This method allows a render content hierarchy to be edited using a modal (AKA 'pop-up') editor. If the original render content is in a document, it will remain there, and the edited one will be 'free-floating'. Therefore it is the caller's responsibility to do any replacement in the document if required. The returned new content will be owned by the caller.

**Returns:** `RenderContent` — Returns an edited version of the content hierarchy if successful, else null. The method always edits the entire hierarchy. It places a copy of the hierarchy in the editor and selects the copied item that corresponds to this one. Therefore, editing a child will return a top-level render content, not a child.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Edit.htm)

#### `public void EndChange()`

Ends a change or batch of changes. Calls to this method are counted; you must call this method once for every call to BeginChange(RenderContent.ChangeContexts). Note: If BeginChange(RenderContent.ChangeContexts) was called with ChangeContexts.UI, ChangeContexts.Program, ChangeContexts.Drop or ChangeContexts.UI.Tree and Changed() was called between the calls to BeginChange(RenderContent.ChangeContexts) and EndChange(), the last call to EndChange() will raise the ContentChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndChange.htm)

#### `public void EndCreateDynamicFields()`

You must call this after creating dynamic fields. Calls to this method are counted; you must call BeginCreateDynamicFields() once for every call to EndCreateDynamicFields().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_EndCreateDynamicFields.htm)

#### `public ContentFactory Factory()`

(Inherited from RenderContent.)

**Returns:** `ContentFactory` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Factory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Factory.htm)

#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Finalize.htm)

#### `public RenderContent FindChild(string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.FindChild(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_FindChild.htm)

#### `public RenderContent ForDisplay()`

The only place a single proxy can be displayed is in the New Content Control main thumbnail. All other attempts to use a single proxy in a UI require it to be replaced with the corresponding multi proxy. Single proxies override this to find the corresponding multi proxy.

**Returns:** `RenderContent` — The cotnent.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ForDisplay.htm)

#### `public virtual Object GetChildSlotParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to specify additional functionality for automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call GetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetChildSlotParameter.htm)

#### `public bool GetDisplayInViewport()`

(Inherited from RenderTexture.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetDisplayInViewport"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetDisplayInViewport.htm)

#### `public string[] GetEmbeddedFilesList()`

(Inherited from RenderContent.)

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetEmbeddedFilesList"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetEmbeddedFilesList.htm)

#### `public TextureEnvironmentMappingMode GetEnvironmentMappingMode()`

(Inherited from RenderTexture.)

**Returns:** `TextureEnvironmentMappingMode` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetEnvironmentMappingMode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetEnvironmentMappingMode.htm)

#### `public Object GetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.

**Returns:** `Object` — The value of the requested extra requirement parameter or null if the parameter does not exist. Current supported return values are (int, bool, float, double, string, Guid, Color, Vector3d, Point3d, DateTime).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetExtraRequirementParameter.htm)

#### `public TextureEnvironmentMappingMode GetInternalEnvironmentMappingMode()`

(Inherited from RenderTexture.)

**Returns:** `TextureEnvironmentMappingMode` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetInternalEnvironmentMappingMode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetInternalEnvironmentMappingMode.htm)

#### `public RenderTexture.eLocalMappingType GetLocalMappingType()`

(Inherited from RenderTexture.)

**Returns:** `RenderTexture.eLocalMappingType` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetLocalMappingType"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetLocalMappingType.htm)

#### `public virtual int GetMappingChannel()`

(Inherited from RenderTexture.)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetMappingChannel"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetMappingChannel.htm)

#### `public virtual Vector3d GetOffset()`

Get offset value across UVW space. If the projection type is WCS or other type specified in model units, then this is the offset in meters.

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetOffset"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetOffset.htm)

#### `public virtual bool GetOffsetLocked()`

(Inherited from RenderTexture.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetOffsetLocked"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetOffsetLocked.htm)

#### `public virtual Object GetParameter(string parameterName)`

Query the content instance for the value of a given named parameter. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — Name of the parameter

**Returns:** `Object` — IConvertible. Note that you can't directly cast from object, instead you have to use the Convert mechanism.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetParameter.htm)

#### `public virtual bool GetPreviewIn3D()`

(Inherited from RenderTexture.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetPreviewIn3D"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetPreviewIn3D.htm)

#### `public bool GetPreviewLocalMapping()`

(Inherited from RenderTexture.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetPreviewLocalMapping"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetPreviewLocalMapping.htm)

#### `public virtual TextureProjectionMode GetProjectionMode()`

(Inherited from RenderTexture.)

**Returns:** `TextureProjectionMode` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetProjectionMode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetProjectionMode.htm)

#### `public virtual Vector3d GetRepeat()`

Get repeat value across UVW space. If the projection type is WCS or other type specified in model units, then this is the repeat across 1 meter of the model.

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetRepeat"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetRepeat.htm)

#### `public virtual bool GetRepeatLocked()`

(Inherited from RenderTexture.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetRepeatLocked"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetRepeatLocked.htm)

#### `public virtual Vector3d GetRotation()`

(Inherited from RenderTexture.)

**Returns:** `Vector3d` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetRotation"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetRotation.htm)

#### `public virtual ulong GetUiHash()`

This allows a content to have more than one UI for the same content type.

**Returns:** `UInt64` — Default is zero and is ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUiHash.htm)

#### `public bool GetUnderlyingInstances(ref RenderContentCollection collection)`

(Inherited from RenderContent.)

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — [Missing <param name="collection"/> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.GetUnderlyingInstances(Rhino.Render.RenderContentCollection@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_GetUnderlyingInstances.htm)

#### `public virtual TextureWrapType GetWrapType()`

(Inherited from RenderTexture.)

**Returns:** `TextureWrapType` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.GetWrapType"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GetWrapType.htm)

#### `public void GraphInfo(ref TextureGraphInfo tgi)`

(Inherited from RenderTexture.)

**Parameters:**
- `tgi` (Rhino.Render.TextureGraphInfo) — [Missing <param name="tgi"/> documentation for "M:Rhino.Render.RenderTexture.GraphInfo(Rhino.Render.TextureGraphInfo@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_GraphInfo.htm)

#### `public virtual bool Icon(Size size, out Bitmap bitmap)`

(Inherited from RenderContent.)

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Icon(System.Drawing.Size,System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Icon.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public bool Initialize()`

Obsolete. (Inherited from RenderContent.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Initialize.htm)

#### `public virtual bool IsCompatible(Guid renderEngineId)`

(Inherited from RenderContent.)

**Parameters:**
- `renderEngineId` (System.Guid) — [Missing <param name="renderEngineId"/> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsCompatible(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsCompatible.htm)

#### `public virtual bool IsContentTypeAcceptableAsChild(Guid type, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `type` (System.Guid) — [Missing <param name="type"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsContentTypeAcceptableAsChild(System.Guid,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsContentTypeAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(ContentFactory factory, string childSlotName)`

(Inherited from RenderContent.)

**Parameters:**
- `factory` (Rhino.Render.DataSources.ContentFactory) — [Missing <param name="factory"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(Rhino.Render.DataSources.ContentFactory,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild.htm)

#### `public virtual bool IsFactoryProductAcceptableAsChild(Guid kindId, string factoryKind, string childSlotName)`

Override this method to restrict the type of acceptable child content. The default implementation of this method just returns true.

**Parameters:**
- `kindId` (System.Guid) — [Missing <param name="kindId"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `factoryKind` (System.String) — [Missing <param name="factoryKind"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]
- `childSlotName` (System.String) — [Missing <param name="childSlotName"/> documentation for "M:Rhino.Render.RenderContent.IsFactoryProductAcceptableAsChild(System.Guid,System.String,System.String)"]

**Returns:** `Boolean` — Return true only if content with the specified kindId can be accepted as a child in the specified child slot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsFactoryProductAcceptableAsChild_1.htm)

#### `public bool IsHdrCapable()`

Return true if the texture is HDR capable. When creating a custom RenderTexture implementation that is HDR capable set the appropriate property on the CustomRenderContentAttribute decorator on that clas.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.IsHdrCapable"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsHdrCapable.htm)

#### `public virtual bool IsImageBased()`

Query if the texture is image based. When creating a custom RenderTexture implementation of an image-based texture set the appropriate property on the CustomRenderContentAttribute decorator on that class. Do not override this function

**Returns:** `Boolean` — true if the texture is image-based.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsImageBased.htm)

#### `public virtual bool IsLinear()`

Return true if the texture color data is linear. NOTE: this function is marked as virtual, but the correct way to make a custom RenderTexture linear is by setting the correct property for the CustomRenderContentAttribute decorator on the class.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.IsLinear"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsLinear.htm)

#### `public bool IsNormalMap()`

Return true if the texture is a normalmap. When creating a custom RenderTexture implementation of a normal map set the appropriate property on the CustomRenderContentAttribute decorator on that clas.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.IsNormalMap"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_IsNormalMap.htm)

#### `public bool IsReference()`

Query whether or not the content or any of its ancestors is a reference content.

**Returns:** `Boolean` — true if the content is a reference, else false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsReference.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public bool IsRenderHashCached()`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Returns:** `Boolean` — bool

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_IsRenderHashCached.htm)

#### `public RenderContent MakeCopy()`

Create a copy of the render content. All content is the same, except for the instance Id.

**Returns:** `RenderContent` — The new RenderContent

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeCopy.htm)

#### `public RenderContent MakeGroupInstance()`

Create an 'instance' of the content hierarchy and group the new hierarchy with this hierarchy. If the instance is subsequently attached to the same document, the state of all members of the group will be kept synchronized. With the exception of the instance ids, all state is exactly preserved in the new instance hierarchy. \note The grouping will have no effect until the new instance is attached to the same document.

**Returns:** `RenderContent` — A grouped instance of the content hierarchy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MakeGroupInstance.htm)

#### `public virtual RenderContent.MatchDataResult MatchData(RenderContent oldContent)`

Implement to transfer data from another content to this content during creation.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — An old content object from which the implementation may harvest data.

**Returns:** `RenderContent.MatchDataResult` — Information about how much data was matched.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_MatchData.htm)

#### `protected void ModifyRenderContentStyles(RenderContentStyles stylesToAdd, RenderContentStyles stylesToRemove)`

ModifyRenderContentStyles

**Parameters:**
- `stylesToAdd` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToAdd"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]
- `stylesToRemove` (Rhino.Render.RenderContentStyles) — [Missing <param name="stylesToRemove"/> documentation for "M:Rhino.Render.RenderContent.ModifyRenderContentStyles(Rhino.Render.RenderContentStyles,Rhino.Render.RenderContentStyles)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ModifyRenderContentStyles.htm)

#### `public PreviewSceneServer NewPreviewSceneServer(SceneServerData ssd)`

Gets the PreviewSceneServer of the content

**Parameters:**
- `ssd` (Rhino.Render.SceneServerData) — SceneServerData

**Returns:** `PreviewSceneServer` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.NewPreviewSceneServer(Rhino.Render.SceneServerData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_NewPreviewSceneServer.htm)

#### `protected override sealed void OnAddUserInterfaceSections()`

(Overrides RenderContent.OnAddUserInterfaceSections().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_TwoColorRenderTexture_OnAddUserInterfaceSections.htm)

#### `protected virtual bool OnGetDefaultsInteractive()`

Override this method to prompt user for information necessary to create a new content object. For example, if you are created a textured material you may prompt the user for a bitmap file name prior to creating the textured material.

**Returns:** `Boolean` — If true is returned the content is created otherwise the creation is aborted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnGetDefaultsInteractive.htm)

#### `protected virtual void OnMakeCopy(ref RenderContent newContent)`

Override this function to supplement the standard copying behavour for your RenderContent.

**Parameters:**
- `newContent` (Rhino.Render.RenderContent) — Is the content that will be returned from MakeCopy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OnMakeCopy.htm)

#### `public bool OpenInEditor()`

Call this method to open the content in the relevant thumbnail editor and select it for editing by the user. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInEditor.htm)

#### `[ObsoleteAttribute("Obsolete, use Edit a version that returns a RenderContent", false)] public bool OpenInModalEditor()`

Call this method to open the content in the a modal version of the editor. The content must be in the document or the call will fail.

**Returns:** `Boolean` — Returns true on success or false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_OpenInModalEditor.htm)

#### `public string ParamNameFromChildSlotName(string childSlotName)`

A "child slot" is the specific "slot" that a child (usually a texture) occupies. This is generally the "use" of the child - in other words, the thing the child operates on. Some examples are "color", "transparency".

**Parameters:**
- `childSlotName` (System.String) — The named of the child slot to receive the parameter name for.

**Returns:** `String` — The default behavior for these functions is to return the input string. Sub-classes may (in the future) override these functions to provide different mappings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_ParamNameFromChildSlotName.htm)

#### `public void PixelSize(out int u, out int v, out int w)`

Get the texture dimensions for the RenderTexture.

**Parameters:**
- `u` (System.Int32) — width
- `v` (System.Int32) — height
- `w` (System.Int32) — depth, used for 3D textures

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_PixelSize.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude.htm)

#### `public uint RenderHashExclude(CrcRenderHashFlags flags, string excludeParameterNames, LinearWorkflow lw)`

As RenderHash, but ignore parameter names given. Use this version of the function to calculate a render hash when you have the linear workflow information and you are not running on the main thread. Access to LinearWorkflow data requires document access. CrcRenderHashFlags.ExcludeLinearWorkflow must be specified.

**Parameters:**
- `flags` (Rhino.Render.CrcRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string
- `lw` (Rhino.Render.LinearWorkflow) — Linear Workflow to use for CRC

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_1.htm)

#### `public uint RenderHashExclude(TextureRenderHashFlags flags, string excludeParameterNames)`

As RenderHash, but ignore parameter names given.

**Parameters:**
- `flags` (Rhino.Render.TextureRenderHashFlags) — Flags to ignore
- `excludeParameterNames` (System.String) — semicolon-delimited string

**Returns:** `UInt32` — Render hash

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_RenderHashExclude_2.htm)

#### `public bool Replace(RenderContent newcontent)`

(Inherited from RenderContent.)

**Parameters:**
- `newcontent` (Rhino.Render.RenderContent) — [Missing <param name="newcontent"/> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.Replace(Rhino.Render.RenderContent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Replace.htm)

#### `public bool SaveAsImage(string FullPath, int width, int height, int depth)`

Save texture as image

**Parameters:**
- `FullPath` (System.String) — The full path of the file
- `width` (System.Int32) — Image width
- `height` (System.Int32) — Image height
- `depth` (System.Int32) — Image depth

**Returns:** `Boolean` — returns true if file was saved, otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SaveAsImage.htm)

#### `public bool SetChild(RenderContent renderContent, string childSlotName)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild.htm)

#### `[ObsoleteAttribute("Use SetChild without ChangeContexts and Begin/EndChange")] public bool SetChild(RenderContent renderContent, string childSlotName, RenderContent.ChangeContexts changeContexts)`

Set another content as a child of this content. This content may or may not be attached to a document. If this content already has a child with the specified child slot name, that child will be deleted. If this content is not attached to a document, the child will be added without sending any events. If this content is attached to a document, the necessary events will be sent to update the UI. Note: Do not call this method to add children in your constructor. If you want to add default children, you should override Initialize() and add them there.

**Parameters:**
- `renderContent` (Rhino.Render.RenderContent) — Child content to add to this content. If pChild is NULL, the function will fail. If pChild is already attached to a document, the function will fail. If pChild is already a child of this or another content, the function will fail.
- `childSlotName` (System.String) — The name that will be assigned to this child slot. The child slot name cannot be an empty string. If it is, the function will fail.
- `changeContexts` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContexts"/> documentation for "M:Rhino.Render.RenderContent.SetChild(Rhino.Render.RenderContent,System.String,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — Returns true if the content was added or the child slot with this name was modified otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChild_1.htm)

#### `public void SetChildSlotAmount(string childSlotName, double amount, RenderContent.ChangeContexts cc)`

Sets the amount property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `amount` (System.Double) — The texture amount. Values are typically from 0.0 to 100.0
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — The context of the change.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotAmount.htm)

#### `public void SetChildSlotOn(string childSlotName, bool bOn, RenderContent.ChangeContexts cc)`

Sets the on-ness property for the texture in the specified child slot.

**Parameters:**
- `childSlotName` (System.String) — The child slot name of the texture.
- `bOn` (System.Boolean) — Value for the on-ness property.
- `cc` (Rhino.Render.RenderContent.ChangeContexts) — Context of the change

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotOn.htm)

#### `public virtual bool SetChildSlotParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. Override this function to support values being set from automatic UI sections or the texture summary. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names. Call the base class from your override if you do not support the extra requirement parameter. Please do not call this function. It is only retained for backward compatibility. You should instead call SetExtraRequirementParameter().

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetChildSlotParameter.htm)

#### `public void SetDisplayInViewport(bool value)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetDisplayInViewport(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetDisplayInViewport.htm)

#### `public void SetDisplayInViewport(bool value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetDisplayInViewport(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetDisplayInViewport(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetDisplayInViewport_1.htm)

#### `public void SetEnvironmentMappingMode(TextureEnvironmentMappingMode value)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (Rhino.Render.TextureEnvironmentMappingMode) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetEnvironmentMappingMode(Rhino.Render.TextureEnvironmentMappingMode)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetEnvironmentMappingMode.htm)

#### `public void SetEnvironmentMappingMode(TextureEnvironmentMappingMode value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (Rhino.Render.TextureEnvironmentMappingMode) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetEnvironmentMappingMode(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetEnvironmentMappingMode(Rhino.Render.TextureEnvironmentMappingMode,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetEnvironmentMappingMode_1.htm)

#### `public bool SetExtraRequirementParameter(string contentParameterName, string extraRequirementParameter, Object value, RenderContent.ExtraRequirementsSetContexts sc)`

Extra requirements are a way of specifying extra functionality on parameters in the automatic UI. See IAutoUIExtraRequirements.h in the C++ RDK SDK for string definitions for the parameter names.

**Parameters:**
- `contentParameterName` (System.String) — The parameter or field internal name to which this query applies.
- `extraRequirementParameter` (System.String) — The extra requirement parameter, as listed in IAutoUIExtraRequirements.h in the C++ RDK SDK.
- `value` (System.Object) — The value to set this extra requirement parameter. You will typically use System.Convert to convert the value to the type you need.
- `sc` (Rhino.Render.RenderContent.ExtraRequirementsSetContexts) — The context of this operation.

**Returns:** `Boolean` — true if successful, else false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetExtraRequirementParameter.htm)

#### `public void SetGraphInfo(TextureGraphInfo tgi)`

(Inherited from RenderTexture.)

**Parameters:**
- `tgi` (Rhino.Render.TextureGraphInfo) — [Missing <param name="tgi"/> documentation for "M:Rhino.Render.RenderTexture.SetGraphInfo(Rhino.Render.TextureGraphInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetGraphInfo.htm)

#### `public void SetIsRenderHashRecursive(bool recursive)`

By default, RenderHash() recurses into children when computing the render CRC. However, some applications may require children to be excluded from the render CRC calculation. Call this method to enable or disable recursing into children. see RenderHash

**Parameters:**
- `recursive` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetIsRenderHashRecursive.htm)

#### `public virtual void SetMappingChannel(int value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetMappingChannel(System.Int32,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetMappingChannel(System.Int32,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetMappingChannel.htm)

#### `public void SetName(string name, bool renameEvents, bool ensureNameUnique)`

Set instance name for this content

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `renameEvents` (System.Boolean) — [Missing <param name="renameEvents"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]
- `ensureNameUnique` (System.Boolean) — [Missing <param name="ensureNameUnique"/> documentation for "M:Rhino.Render.RenderContent.SetName(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetName.htm)

#### `public virtual void SetOffset(Vector3d value, RenderContent.ChangeContexts changeContext)`

Set offset value across UVW space. If the projection type is WCS or other type specified in model units, then this is the offset in meters.

**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetOffset(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetOffset(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetOffset.htm)

#### `public virtual void SetOffsetLocked(bool value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetOffsetLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetOffsetLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetOffsetLocked.htm)

#### `public virtual bool SetParameter(string parameterName, Object value)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter.htm)

#### `[ObsoleteAttribute("Use SetParameter without ChangeContexts and Begin/EndChange")] public virtual bool SetParameter(string parameterName, Object value, RenderContent.ChangeContexts changeContext)`

Set the named parameter value for this content instance. If you do not support this parameter, call the base class.

**Parameters:**
- `parameterName` (System.String) — [Missing <param name="parameterName"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `value` (System.Object) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.SetParameter(System.String,System.Object,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetParameter_1.htm)

#### `public virtual void SetPreviewIn3D(bool value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewIn3D(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewIn3D(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetPreviewIn3D.htm)

#### `public void SetPreviewLocalMapping(bool value)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewLocalMapping(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetPreviewLocalMapping.htm)

#### `public void SetPreviewLocalMapping(bool value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewLocalMapping(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetPreviewLocalMapping(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetPreviewLocalMapping_1.htm)

#### `public virtual void SetProjectionMode(TextureProjectionMode value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (Rhino.Render.TextureProjectionMode) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetProjectionMode(Rhino.Render.TextureProjectionMode,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetProjectionMode(Rhino.Render.TextureProjectionMode,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetProjectionMode.htm)

#### `[ObsoleteAttribute("This method is deprecated and no longer called. For more information see CalculateRenderHash")] public void SetRenderHash(uint hash)`

This method is deprecated and no longer called. For more information see CalculateRenderHash(UInt64)

**Parameters:**
- `hash` (System.UInt32)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SetRenderHash.htm)

#### `public virtual void SetRepeat(Vector3d value, RenderContent.ChangeContexts changeContext)`

Set repeat value across UVW space. If the projection type is WCS or other type specified in model units, then this is the repeat across 1 meter of the model.

**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeat(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeat(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetRepeat.htm)

#### `public virtual void SetRepeatLocked(bool value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeatLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetRepeatLocked(System.Boolean,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetRepeatLocked.htm)

#### `public virtual void SetRotation(Vector3d value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetRotation(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetRotation(Rhino.Geometry.Vector3d,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetRotation.htm)

#### `public virtual void SetWrapType(TextureWrapType value, RenderContent.ChangeContexts changeContext)`

(Inherited from RenderTexture.)

**Parameters:**
- `value` (Rhino.Render.TextureWrapType) — [Missing <param name="value"/> documentation for "M:Rhino.Render.RenderTexture.SetWrapType(Rhino.Render.TextureWrapType,Rhino.Render.RenderContent.ChangeContexts)"]
- `changeContext` (Rhino.Render.RenderContent.ChangeContexts) — [Missing <param name="changeContext"/> documentation for "M:Rhino.Render.RenderTexture.SetWrapType(Rhino.Render.TextureWrapType,Rhino.Render.RenderContent.ChangeContexts)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SetWrapType.htm)

#### `[ObsoleteAttribute("Use SimulateTexture with size, TextureGeneration and object instead")] public virtual void SimulateTexture(ref SimulatedTexture simulation, bool isForDataOnly)`

Obsolete. (Inherited from RenderTexture.)

**Parameters:**
- `simulation` (Rhino.Render.SimulatedTexture) — [Missing <param name="simulation"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,System.Boolean)"]
- `isForDataOnly` (System.Boolean) — [Missing <param name="isForDataOnly"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SimulateTexture_1.htm)

#### `public virtual void SimulateTexture(ref SimulatedTexture simulation, RenderTexture.TextureGeneration tg, int size = -1, RhinoObject obj = null)`

(Inherited from RenderTexture.)

**Parameters:**
- `simulation` (Rhino.Render.SimulatedTexture) — [Missing <param name="simulation"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — [Missing <param name="tg"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `size` (System.Int32) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.RenderTexture.SimulateTexture(Rhino.Render.SimulatedTexture@,Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SimulateTexture.htm)

#### `public virtual SimulatedTexture SimulatedTexture(RenderTexture.TextureGeneration tg, int size = -1, RhinoObject obj = null)`

(Inherited from RenderTexture.)

**Parameters:**
- `tg` (Rhino.Render.RenderTexture.TextureGeneration) — [Missing <param name="tg"/> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `size` (System.Int32) — [Missing <param name="size"/> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]

**Returns:** `SimulatedTexture` — [Missing <returns> documentation for "M:Rhino.Render.RenderTexture.SimulatedTexture(Rhino.Render.RenderTexture.TextureGeneration,System.Int32,Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderTexture_SimulatedTexture.htm)

#### `public bool SmartUngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. If any content in the same document is left alone in the group, that content is also ungrouped. Records undo and sends events OnContentChanged and OnContentGroupIdChanged. \note This method is designed to be called from a content UI and is intended for RDK internal use but may be used as an expert user override.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_SmartUngroupRecursive.htm)

#### `public bool Ungroup()`

Remove this content from any instance group it may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if content was ungrouped, \e false if it was not part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Ungroup.htm)

#### `public bool UngroupRecursive()`

Remove this content and all its children from any instance groups they may be a member of. Does not record undo but does send the OnContentGroupIdChanged event.

**Returns:** `Boolean` — true if a content was ungrouped, \e false if no content or child was part of a group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UngroupRecursive.htm)

#### `[ObsoleteAttribute("This method should not be called.")] public void Uninitialize()`

Obsolete. (Inherited from RenderContent.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_Uninitialize.htm)

#### `public int UseCount()`

UseCount returns how many times the content is used

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.Render.RenderContent.UseCount"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_UseCount.htm)

#### `public virtual bool VirtualIcon(Size size, out Bitmap bitmap)`

Icon to display in the content browser, this bitmap needs to be valid for the life of this content object, the content object that returns the bitmap is responsible for disposing of the bitmap.

**Parameters:**
- `size` (System.Drawing.Size) — Requested icon size
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.Render.RenderContent.VirtualIcon(System.Drawing.Size,System.Drawing.Bitmap@)"]

**Returns:** `Boolean` — Return Icon to display in the content browser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_RenderContent_VirtualIcon.htm)

### Properties
- `CanBeEdited` (Boolean, get) — Determines if the content can be edited.
- `Category` (String, get) — Category for this content.
- `ChildSlotDisplayName` (String, get) — Returns the localized display name of the child slot name
- `ChildSlotName` (String, get/set) — (Inherited from RenderContent.)
- `Color1` (Color4f, get/set) — 
- `Color2` (Color4f, get/set) — 
- `CppPointer` (IntPtr, get) — (Inherited from RenderContent.)
- `DisplayName` (String, get) — Display name for this content.
- `Document` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `DocumentAssoc` (RhinoDoc, get/set) — If this render content is associated with a document in any way, the document will be returned. This includes copies of render contents that were attached to a document when the copy was made. Otherwise returns null.
- `DocumentOwner` (RhinoDoc, get) — If this render content is owned by a document, the document will be returned. This is the same as getting the document the render content is attached to. Otherwise returns null.
- `DocumentRegistered` (RhinoDoc, get) — Obsolete. Do not use. You should use DocumentOwner instead.
- `Fields` (FieldDictionary, get) — Rhino.Render.Fields FieldDictionary which provides access to setting and retrieving field values.
- `Filename` (String, get/set) — If the content is file based, this function can be overridden to deal with setting/getting the filename. Corresponds to IRhRdkFileBasedContent in the C++ SDK
- `FilesToEmbed` (IEnumerable<String>, get) — A string array of full paths to files used by the content that may be embedded in .3dm files and library files (.rmtl, .renv, .rtex). The default implementation returns an empty string list. Override this to return the file name or file names used by your content. This is typically used by textures that reference files containing the texture imagery.
- `FirstChild` (RenderContent, get) — Return First child of this content or nullptr if none.
- `GroupId` (Guid, get/set) — Group ID of the content
- `Hidden` (Boolean, get/set) — Gets or sets the render content's 'hidden' state. This feature only works for top-level render contents because it hides the entire hierarchy. It is normally used for 'implementation detail' render contents. For expert use only. Hidden render contents are never shown in the UI, with the exception of the Object (or Layer) Material Properties UI which always shows whatever is assigned to the object (or layer). In the Object (or Layer) Material Properties UI, if the user drops down the list, hidden render contents are not listed. Hidden render contents, being part of the document content list, will be listed by any scripts or other code that iterates over the document render content list. It is recommended that you set IsHidden once when you create your render content and leave it on to prevent flicker or slow performance.
- `Id` (Guid, get/set) — Instance identifier for this content.
- `IsDefaultInstance` (Boolean, get) — Checks if render content is default instance.
- `IsHiddenByAutoDelete` (Boolean, get) — Contents can be created as 'auto-delete' by certain commands such as 'Picture'. These contents are automatically hidden from the user when the associated Rhino object is deleted. They are later deleted when the document is saved.
- `IsLocked` (Boolean, get/set) — Set this property to true prior to adding content to the document to lock the content browser editing UI methods. Setting this to true will keep the browser from allowing things like deleting, renaming or changing content. This is useful for custom child content that you want to be editable but persistent. Setting this after adding content to the document will cause an exception to be thrown.
- `LocalMappingTransform` (Transform, get) — Gets the transformation that can be applied to the UVW vector to convert it from normalized texture space into locally mapped space (ie - with repeat, offset and rotation applied.)
- `Name` (String, get/set) — Instance 'raw' name for this content.
- `NextSibling` (RenderContent, get) — Return First sibling of this content or nullptr if none.
- `Notes` (String, get/set) — Notes for this content.
- `Parent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `PixelSize2` (Nullable<ValueTuple<T1,T2,T3>>, get) — Override to provide details on the actual pixel sizes of this texture in UVW directions return null if the texture should not be treated as a bitmap.
- `ProxyType` (ProxyTypes, get) — Gets the proxy type of the render content
- `RenderHash` (UInt32, get) — Render hash for the content hierarchy. It iterates children and includes a caching mechanism which means the hash value can be retrieved quickly if it hasn't changed. The cache is invalidated when Changed() is called. You can override the CalculateRenderHash(UInt64) method to provide a custom hash value.
- `RenderHashWithoutLocalMapping` (UInt32, get) — Render hash for texture excluding local mapping.
- `Styles` (RenderContentStyles, get) — (Inherited from RenderContent.)
- `SuperSample` (Boolean, get/set) — 
- `SwapColors` (Boolean, get/set) — 
- `Tags` (String, get/set) — Tags for this content.
- `Texture1Amount` (Double, get/set) — 
- `Texture1On` (Boolean, get/set) — 
- `Texture2Amount` (Double, get/set) — 
- `Texture2On` (Boolean, get/set) — 
- `TopLevel` (Boolean, get) — Returns true if this content has no parent, false if it is the child of another content.
- `TopLevelParent` (RenderContent, get) — Returns the top content in this parent/child chain.
- `TypeDescription` (String, get) — Description for your content type. ie. "Procedural checker pattern"
- `TypeId` (Guid, get) — Type identifier for this content
- `TypeName` (String, get) — Name for your content type. ie. "My .net Texture"
- `Xml` (String, get) — (Inherited from RenderContent.)

## UndoRedo (class)

This class contains the event for UndoRedoChanged that is fired from RDK .

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_UndoRedo.htm)

### Events
#### `UndoRedoChanged` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> UndoRedoChanged`

Called after undo or redo has occurred for document settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_UndoRedo_UndoRedoChanged.htm)

#### `UndoRedoEndedChanged` (`System.EventHandler<RenderPropertyChangedEvent>`)

**Signature:** `public static event EventHandler<RenderPropertyChangedEvent> UndoRedoEndedChanged`

This event is raised when undo/redo ends in rdk.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_Render_UndoRedo_UndoRedoEndedChanged.htm)

## Utilities (class)

[Missing <summary> documentation for "T:Rhino.Render.Utilities"]

**Remarks:** This is a replacement for CRhinoFileUtilities::FindFile().

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Utilities.htm)

### Methods
#### `public static RenderContent ChangeContentType(RenderContent oldContent, Guid newType, bool harvestParameters)`

Changes the type of a content. This deletes the content and creates a replacement of the specified type allowing the caller to decide about harvesting.

**Parameters:**
- `oldContent` (Rhino.Render.RenderContent) — oldContent is the old content which is deleted.
- `newType` (System.Guid) — The type of content to replace pOldContent with.
- `harvestParameters` (System.Boolean) — Determines whether or not parameter harvesting will be performed.

**Returns:** `RenderContent` — A new persistent render content.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_ChangeContentType.htm)

#### `public static string FindFile(RhinoDoc doc, string fullPathToFile)`

Finds a file and also handles network shares. RemarksThis is a replacement for CRhinoFileUtilities::FindFile().

**Remarks:** This is a replacement for CRhinoFileUtilities::FindFile().

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Document to use for locating .3dm file's folder.
- `fullPathToFile` (System.String) — The file to be found.

**Returns:** `String` — The found file.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_FindFile.htm)

#### `public static string FindFile(RhinoDoc doc, string fullPathToFile, bool unpackFromBitmapTableIfNecessary)`

Finds a file and also handles network shares. RemarksThis is a replacement for CRhinoFileUtilities::FindFile().

**Remarks:** This is a replacement for CRhinoFileUtilities::FindFile().

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Document to use for locating .3dm file's folder.
- `fullPathToFile` (System.String) — The file to be found.
- `unpackFromBitmapTableIfNecessary` (System.Boolean) — True to seasch for the file in the bitmap table and unpack it into the temp folder if not found in the initial search.

**Returns:** `String` — The found file.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_FindFile_1.htm)

#### `public static bool IsCachedTextureFileInUse(string textureFileName)`

Determines if any texture in any persistent content list is using the specified file name for caching.

**Parameters:**
- `textureFileName` (System.String) — The file name to check for. The extension is ignored.

**Returns:** `Boolean` — true if the texture is present.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_IsCachedTextureFileInUse.htm)

#### `public static RenderContent LoadPersistentRenderContentFromFile(uint docSerialNumber, string filename)`

Loads a content from a library file and adds it to the persistent content list of a particular document.

**Parameters:**
- `docSerialNumber` (System.UInt32) — identifies the document into which the content should be loaded.
- `filename` (System.String) — is the full path to the file to be loaded.

**Returns:** `RenderContent` — The loaded content or null if an error occurred.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_LoadPersistentRenderContentFromFile.htm)

#### `public static void MoveWindow(IntPtr hwnd, Rectangle rect, bool bRepaint, bool bRepaintNC)`



**Parameters:**
- `hwnd` (System.IntPtr) — [Missing <param name="hwnd"/> documentation for "M:Rhino.Render.Utilities.MoveWindow(System.IntPtr,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `rect` (System.Drawing.Rectangle) — [Missing <param name="rect"/> documentation for "M:Rhino.Render.Utilities.MoveWindow(System.IntPtr,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `bRepaint` (System.Boolean) — [Missing <param name="bRepaint"/> documentation for "M:Rhino.Render.Utilities.MoveWindow(System.IntPtr,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `bRepaintNC` (System.Boolean) — [Missing <param name="bRepaintNC"/> documentation for "M:Rhino.Render.Utilities.MoveWindow(System.IntPtr,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_MoveWindow.htm)

#### `public static string PromptForSaveImageFileParameters(string filename, ref int width, ref int height, ref int colorDepth)`

Prompts the user for a save file name and the width, height and depth of an image to be saved.

**Parameters:**
- `filename` (System.String) — The original file path.
- `width` (System.Int32) — A width.
- `height` (System.Int32) — An height.
- `colorDepth` (System.Int32) — A color depth.

**Returns:** `String` — The new file name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_PromptForSaveImageFileParameters.htm)

#### `public static bool SafeFrameEnabled(RhinoDoc doc)`

Queries whether or not the Safe Frame is visible.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.Render.Utilities.SafeFrameEnabled(Rhino.RhinoDoc)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.Utilities.SafeFrameEnabled(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_SafeFrameEnabled.htm)

#### `public static bool SetDefaultRenderPlugIn(Guid pluginId)`

Set default render application

**Parameters:**
- `pluginId` (System.Guid) — ID of render plug-in

**Returns:** `Boolean` — True if plug-in found and loaded successfully. False if pluginId is invalid or was unable to load plug-in

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_SetDefaultRenderPlugIn.htm)

#### `public static Utilities.ShowContentChooserResults ShowContentChooser(Guid defaultType, Guid defaultInstanceId, RenderContentKind kinds, ref Guid instanceIdOut, RenderContent.ShowContentChooserFlags flags, RhinoDoc doc)`

Shows the content chooser to allow the user to select a new or existing content.

**Parameters:**
- `defaultType` (System.Guid) — The content type that will be initially selected in the 'New' tab.
- `defaultInstanceId` (System.Guid) — The content instance that will be initially selected in the 'Existing' tab.
- `kinds` (Rhino.Render.RenderContentKind) — Semicolon-delimited string specifying which content kinds will be displayed.
- `instanceIdOut` (System.Guid) — The UUID of the chosen item. Depending on eRhRdkSccResult, this can be the type id of a content type or the instance id of an existing content.
- `flags` (Rhino.Render.RenderContent.ShowContentChooserFlags) — Flags controlling the chooser. Multiple selection is not valid for this call.
- `doc` (Rhino.RhinoDoc) — A Rhino document.

**Returns:** `Utilities.ShowContentChooserResults` — The result of displaying the chooser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_ShowContentChooser_2.htm)

#### `public static Utilities.ShowContentChooserResults ShowContentChooser(RhinoDoc doc, Guid defaultType, Guid defaultInstanceId, RenderContentKind kinds, RenderContent.ShowContentChooserFlags flags, string presetCategory, IEnumerable<string> categories, IEnumerable<Guid> types, out Guid[] instanceIdsOut)`

Shows the content chooser to allow the user to select a new or existing content, or multiple contents.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document.
- `defaultType` (System.Guid) — The content type that will be initially selected in the 'New' tab.
- `defaultInstanceId` (System.Guid) — The content instance that will be initially selected in the 'Existing' tab.
- `kinds` (Rhino.Render.RenderContentKind) — Semicolon-delimited string specifying which content kinds will be displayed.
- `flags` (Rhino.Render.RenderContent.ShowContentChooserFlags) — Flags controlling the chooser (tabs to hide, type of selection).
- `presetCategory` (System.String) — Specifies the category to preset in the drop-down list on the 'New' tab. If this string is empty, the preset category will be 'All'.
- `categories` (System.Collections.Generic.IEnumerable<String>) — Specifies the categories to display. Render contents that do not belong to one of these categories will not be listed. If this array is empty, all render contents from all categories will be listed.
- `types` (System.Collections.Generic.IEnumerable<Guid>) — Specifies content types to display. Contents that are not one of these types will not be listed. If this array is empty, all content types will be listed.
- `instanceIdsOut` (System.Guid[]) — Array of UUID(s) of the chosen item(s). Depending on eRhRdkSccResult, this can accept type id(s) of content type(s) or instance id(s) of existing content(s).

**Returns:** `Utilities.ShowContentChooserResults` — The result of displaying the chooser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_ShowContentChooser.htm)

#### `public static Utilities.ShowContentChooserResults ShowContentChooser(RhinoDoc doc, Guid defaultType, Guid defaultInstanceId, RenderContentKind kinds, Utilities.ContentChooserFlags flags, string presetCategory, IEnumerable<string> categories, IEnumerable<Guid> types, out Guid[] instanceIdsOut)`

Shows the content chooser to allow the user to select a new or existing content, or multiple contents.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document.
- `defaultType` (System.Guid) — The content type that will be initially selected in the 'New' tab.
- `defaultInstanceId` (System.Guid) — The content instance that will be initially selected in the 'Existing' tab.
- `kinds` (Rhino.Render.RenderContentKind) — Semicolon-delimited string specifying which content kinds will be displayed.
- `flags` (Rhino.Render.Utilities.ContentChooserFlags) — Flags controlling the chooser (tabs to hide, type of selection).
- `presetCategory` (System.String) — Specifies the category to preset in the drop-down list on the 'New' tab. If this string is empty, the preset category will be 'All'.
- `categories` (System.Collections.Generic.IEnumerable<String>) — Specifies the categories to display. Render contents that do not belong to one of these categories will not be listed. If this array is empty, all render contents from all categories will be listed.
- `types` (System.Collections.Generic.IEnumerable<Guid>) — Specifies content types to display. Contents that are not one of these types will not be listed. If this array is empty, all content types will be listed.
- `instanceIdsOut` (System.Guid[]) — Array of UUID(s) of the chosen item(s). Depending on eRhRdkSccResult, this can accept type id(s) of content type(s) or instance id(s) of existing content(s).

**Returns:** `Utilities.ShowContentChooserResults` — The result of displaying the chooser.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_ShowContentChooser_1.htm)

#### `public static bool ShowIORMenu(IntPtr hwnd, Point pt, ref double outIOR, ref string outString)`

Display and track the context menu.

**Parameters:**
- `hwnd` (System.IntPtr) — The window that displays the menu, for example an edit box.
- `pt` (System.Drawing.Point) — The position to display the menu at inside the window
- `outIOR` (System.Double) — Accepts the IOR value of the user's chosen substance
- `outString` (System.String) — Accepts the name of the user's chosen substance. Can be null if not required.

**Returns:** `Boolean` — true if the function showed the IOR menu and something was picked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_Render_Utilities_ShowIORMenu.htm)

### Properties
- `DefaultRenderPlugInId` (Guid, get) — Get the plug-in Id for the default render plug-in
- `ShowIncompatibleEnvironments` (Boolean, get/set) — Specifies whether incompatible content should be shown in the corresponding editor.
- `ShowIncompatibleMaterials` (Boolean, get/set) — Specifies whether incompatible content should be shown in the corresponding editor.
- `ShowIncompatibleTextures` (Boolean, get/set) — Specifies whether incompatible content should be shown in the corresponding editor.

## Utilities.ContentChooserFlags (enum)

[Missing <summary> documentation for "T:Rhino.Render.Utilities.ContentChooserFlags"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Utilities_ContentChooserFlags.htm)

### Values
- `None` = `0` — No flags; normal usage.
- `HideNewTab` = `1` — Hides the 'New' tab.
- `HideExistingTab` = `2` — Hides the 'Existing' tab.
- `MultipleSelection` = `4` — Enables multiple selection.
- `DisableImportButton` = `16` — Disables the import button.

## Utilities.ShowContentChooserResults (enum)

[Missing <summary> documentation for "T:Rhino.Render.Utilities.ShowContentChooserResults"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_Utilities_ShowContentChooserResults.htm)

### Values
- `None` = `0` — No choice (user cancelled).
- `New` = `1` — User chose from 'New' tab. uuidOut is the type id.
- `Copy` = `2` — User chose from 'Existing' tab with 'copy' radio button checked. uuidOut is the instance id.
- `Instance` = `3` — User chose from 'Existing' tab with 'instance' radio button checked. uuidOut is the instance id.

## it_strategy (enum)

Defines the collection type to iterate.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_Render_it_strategy.htm)

### Values
- `ContentDataBase` = `0` — This type represents all the render contents in the database.
- `ContentSelection` = `1` — This type represents the selected render content collection.

