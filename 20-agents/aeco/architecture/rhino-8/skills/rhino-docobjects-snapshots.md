---
name: rhino-rhino-docobjects-snapshots
description: This skill encodes the rhino 8.0 surface of the Rhino.DocObjects.SnapShots namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: SnapShotsClient.
---

# Rhino.DocObjects.SnapShots

Auto-generated from vendor docs for rhino 8.0. 1 types in this namespace.

## SnapShotsClient (class)

This is the abstract interface class for all Snapshot clients.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_DocObjects_SnapShots_SnapShotsClient.htm)

### Constructors
- `public SnapShotsClient()` — SnapShotsClient constructor

### Methods
#### `public abstract bool AnimateDocument(RhinoDoc doc, double dPos, BinaryArchiveReader archive_start, BinaryArchiveReader archive_stop)`

Called for each frame. Starting at 0.0.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `dPos` (System.Double) — dPos is the current frame. Starting at 0.0.
- `archive_start` (Rhino.FileIO.BinaryArchiveReader) — archive_start is a archive to the data of the starting position.
- `archive_stop` (Rhino.FileIO.BinaryArchiveReader) — archive_stop is a archive to the data of the ending position.

**Returns:** `Boolean` — true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_AnimateDocument.htm)

#### `public abstract bool AnimateObject(RhinoDoc doc, RhinoObject doc_object, ref Transform transform, double dPos, BinaryArchiveReader archive_start, BinaryArchiveReader archive_stop)`

Called for each frame. Starting at 0.0.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_obj is the current object.
- `transform` (Rhino.Geometry.Transform) — transform is a transformation matrix. The matrix is set to identity the first time an object is associated with a snapshot. After that the matrix is updated when the object is transformed(scale, rotate etc.).
- `dPos` (System.Double) — dPos is the current frame. Starting at 0.0.
- `archive_start` (Rhino.FileIO.BinaryArchiveReader) — archive_start is a archive to the data of the starting position.
- `archive_stop` (Rhino.FileIO.BinaryArchiveReader) — archive_stop is a archive to the data of the ending position.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.AnimateObject(Rhino.RhinoDoc,Rhino.DocObjects.RhinoObject,Rhino.Geometry.Transform@,System.Double,Rhino.FileIO.BinaryArchiveReader,Rhino.FileIO.BinaryArchiveReader)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_AnimateObject.htm)

#### `public abstract void AnimationStart(RhinoDoc doc, int iFrames)`

Called once at the start of an animation.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `iFrames` (System.Int32) — iFrames is the number of frames to be animated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_AnimationStart.htm)

#### `public abstract bool AnimationStop(RhinoDoc doc)`

Called once at the end of an animation.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.AnimationStop(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_AnimationStop.htm)

#### `public static string ApplicationCategory()`

Predefined application category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.ApplicationCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ApplicationCategory.htm)

#### `public abstract string Category()`

The category of this client. Usually one of the above predefined categories like e.g object, rendering or application category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.Category"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_Category.htm)

#### `public abstract Guid ClientId()`

The unique id of this client.

**Returns:** `Guid` — The unique id of this client.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ClientId.htm)

#### `public void Dispose()`

SnapShotsClient Dispose

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_Dispose.htm)

#### `public static string DocumentCategory()`

Predefined document category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.DocumentCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_DocumentCategory.htm)

#### `public abstract void ExtendBoundingBoxForDocumentAnimation(RhinoDoc doc, BinaryArchiveReader archive_start, BinaryArchiveReader archive_stop, ref BoundingBox bbox)`

Called once at the start of an animation. This can be used to extend the scene bounding box to avoid clipping.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `archive_start` (Rhino.FileIO.BinaryArchiveReader) — archive_start is a archive to the data of the starting position.
- `archive_stop` (Rhino.FileIO.BinaryArchiveReader) — archive_stop is a archive to the data of the ending position.
- `bbox` (Rhino.Geometry.BoundingBox) — bbox is the current scene bounding box.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ExtendBoundingBoxForDocumentAnimation.htm)

#### `public abstract void ExtendBoundingBoxForObjectAnimation(RhinoDoc doc, RhinoObject doc_object, ref Transform transform, BinaryArchiveReader archive_start, BinaryArchiveReader archive_stop, ref BoundingBox bbox)`

Called once at the start of an animation. This can be used to extend the scene bounding box to avoid clipping.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_obj is the current object.
- `transform` (Rhino.Geometry.Transform) — transform is a transformation matrix. The matrix is set to identity the first time an object is associated with a snapshot. After that the matrix is updated when the object is transformed(scale, rotate etc.).
- `archive_start` (Rhino.FileIO.BinaryArchiveReader) — archive_start is a archive to the data of the starting position.
- `archive_stop` (Rhino.FileIO.BinaryArchiveReader) — archive_stop is a archive to the data of the ending position.
- `bbox` (Rhino.Geometry.BoundingBox) — bbox is the current scene bounding box.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ExtendBoundingBoxForObjectAnimation.htm)

#### `public abstract bool IsCurrentModelStateInAnySnapshot(RhinoDoc doc, BinaryArchiveReader archive, SimpleArrayBinaryArchiveReader archive_array, TextLog text_log = null)`

Called before restoring a snapshot. Warns the user if the current model state is not already saved.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — archive is the current state of the model.
- `archive_array` (Rhino.Runtime.InteropWrappers.SimpleArrayBinaryArchiveReader) — archive_array is a list of client data.
- `text_log` (Rhino.FileIO.TextLog) — text_log is used to list the missing items that cannot be found in the current model.

**Returns:** `Boolean` — return true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_IsCurrentModelStateInAnySnapshot_1.htm)

#### `public abstract bool IsCurrentModelStateInAnySnapshot(RhinoDoc doc, RhinoObject doc_object, BinaryArchiveReader archive, SimpleArrayBinaryArchiveReader archive_array, TextLog text_log = null)`

Called before restoring a snapshot. Warns the user if the current model state is not already saved.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_object is the current object.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — archive is the current state of the model.
- `archive_array` (Rhino.Runtime.InteropWrappers.SimpleArrayBinaryArchiveReader) — archive_array is a list of client data.
- `text_log` (Rhino.FileIO.TextLog) — text_log is used to list the missing items that cannot be found in the current model.

**Returns:** `Boolean` — return true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_IsCurrentModelStateInAnySnapshot.htm)

#### `public static string LayersCategory()`

Predefined layers category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.LayersCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_LayersCategory.htm)

#### `public static string LightsCategory()`

Predefined lights category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.LightsCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_LightsCategory.htm)

#### `public abstract string Name()`

The client's name.

**Returns:** `String` — The client's name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_Name.htm)

#### `public abstract bool ObjectTransformNotification(RhinoDoc doc, RhinoObject doc_object, ref Transform transform, BinaryArchiveReader archive)`

Called for every object that is associated with a snapshot and gets transformed in Rhino. This is getting called for each stored snapshot and gives the client the possibility to update the stored data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_obj is the current object.
- `transform` (Rhino.Geometry.Transform) — transform is a transformation matrix. The matrix is set to identity the first time an object is associated with a snapshot. After that the matrix is updated when the object is transformed(scale, rotate etc.).
- `archive` (Rhino.FileIO.BinaryArchiveReader) — archive is a archive which can be used to update the stored data.

**Returns:** `Boolean` — true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ObjectTransformNotification.htm)

#### `public static string ObjectsCategory()`

Predefined objects category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.ObjectsCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ObjectsCategory.htm)

#### `public abstract Guid PlugInId()`

The plug-in id that registers this client.

**Returns:** `Guid` — The plug-in id that registers this client.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_PlugInId.htm)

#### `public abstract bool PrepareForDocumentAnimation(RhinoDoc doc, BinaryArchiveReader archive_start, BinaryArchiveReader archive_stop)`

Called once at the start of an animation.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `archive_start` (Rhino.FileIO.BinaryArchiveReader) — archive_start is a archive to the data of the starting position.
- `archive_stop` (Rhino.FileIO.BinaryArchiveReader) — archive_stop is a archive to the data of the ending position.

**Returns:** `Boolean` — true if successful, otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_PrepareForDocumentAnimation.htm)

#### `public abstract bool PrepareForObjectAnimation(RhinoDoc doc, RhinoObject doc_object, ref Transform transform, BinaryArchiveReader archive_start, BinaryArchiveReader archive_stop)`

Called once at the start of an animation.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_obj is the current object.
- `transform` (Rhino.Geometry.Transform) — transform is a transformation matrix. The matrix is set to identity the first time an object is associated with a snapshot. After that the matrix is updated when the object is transformed(scale, rotate etc.).
- `archive_start` (Rhino.FileIO.BinaryArchiveReader) — archive_start is a archive to the data of the starting position.
- `archive_stop` (Rhino.FileIO.BinaryArchiveReader) — archive_stop is a archive to the data of the ending position.

**Returns:** `Boolean` — true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_PrepareForObjectAnimation.htm)

#### `public static bool RegisterSnapShotClient(SnapShotsClient client)`

Function used to register snapshots client

**Parameters:**
- `client` (Rhino.DocObjects.SnapShots.SnapShotsClient) — [Missing <param name="client"/> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.RegisterSnapShotClient(Rhino.DocObjects.SnapShots.SnapShotsClient)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.RegisterSnapShotClient(Rhino.DocObjects.SnapShots.SnapShotsClient)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_RegisterSnapShotClient.htm)

#### `public static string RenderingCategory()`

Predefined rendering category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.RenderingCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_RenderingCategory.htm)

#### `public abstract bool RestoreDocument(RhinoDoc doc, BinaryArchiveReader archive)`

Called when the user restores a snapshot and SupportDocument() returns true.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document
- `archive` (Rhino.FileIO.BinaryArchiveReader) — archive is the archive to read the data from

**Returns:** `Boolean` — true if successful, otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_RestoreDocument.htm)

#### `public abstract bool RestoreObject(RhinoDoc doc, RhinoObject doc_object, ref Transform transform, BinaryArchiveReader archive)`

Called when the user restores a snapshot and SupportsObjects() and SupportsObject(Rhino.DocObjects.RhinoObject doc_object) returns true.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_obj is the current object.
- `transform` (Rhino.Geometry.Transform) — transform is a transformation matrix. The matrix is set to identity the first time an object is associated with a snapshot. After that the matrix is updated when the object is transformed(scale, rotate etc.).
- `archive` (Rhino.FileIO.BinaryArchiveReader) — archive is the archive to read the data from.

**Returns:** `Boolean` — true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_RestoreObject.htm)

#### `public abstract bool SaveDocument(RhinoDoc doc, BinaryArchiveWriter archive)`

Called when the user saves a snapshot and SupportDocument() returns true.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — archive is the archive to write the data to

**Returns:** `Boolean` — true if successful, otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SaveDocument.htm)

#### `public abstract bool SaveObject(RhinoDoc doc, RhinoObject doc_object, ref Transform transform, BinaryArchiveWriter archive)`

Called when the user saves a snapshot and SupportsObjects() and SupportsObject(Rhino.DocObjects.RhinoObject doc_object) returns true.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — doc is the current document.
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_obj is the current object.
- `transform` (Rhino.Geometry.Transform) — transform is a transformation matrix. The matrix is set to identity the first time an object is associated with a snapshot. After that the matrix is updated when the object is transformed(scale, rotate etc.).
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — archive is the archive to write the data to.

**Returns:** `Boolean` — true if successful, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SaveObject.htm)

#### `public abstract void SnapshotRestored(RhinoDoc doc)`

Called after all clients restored their data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.SnapshotRestored(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SnapshotRestored.htm)

#### `public abstract bool SupportsAnimation()`

Returns true if the client allows animation.

**Returns:** `Boolean` — true if the client allows animation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SupportsAnimation.htm)

#### `public abstract bool SupportsDocument()`

Defines if the client supports document user data or not

**Returns:** `Boolean` — true if the client saves/restores document user data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SupportsDocument.htm)

#### `public abstract bool SupportsObject(RhinoObject doc_object)`

Returns true if the client saves/restores object user data for the given object.

**Parameters:**
- `doc_object` (Rhino.DocObjects.RhinoObject) — doc_object is the given object

**Returns:** `Boolean` — true if the client saves/restores object user data for the given object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SupportsObject.htm)

#### `public abstract bool SupportsObjects()`

Returns true if the client saves/restores object user data.

**Returns:** `Boolean` — true if the client saves/restores object user data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_SupportsObjects.htm)

#### `public static string ViewsCategory()`

Predefined views category

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.SnapShots.SnapShotsClient.ViewsCategory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DocObjects_SnapShots_SnapShotsClient_ViewsCategory.htm)

### Properties
- `CppPointer` (IntPtr, get) — 
- `SerialNumber` (Int32, get/set) — 

