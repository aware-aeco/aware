---
name: grasshopper-grasshopper-kernel-undo
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Undo namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_ArchivedUndoAction, GH_ObjectUndoAction, GH_UndoException, GH_UndoAction, GH_UndoRecord, GH_UndoServer, IGH_UndoAction, GH_UndoState.
---

# Grasshopper.Kernel.Undo

Auto-generated from vendor docs for grasshopper 7.0. 8 types in this namespace.

## GH_ArchivedUndoAction (class)

Base class implementation for undo actions that require (de)serialization of data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction.htm)

### Constructors
- `protected GH_ArchivedUndoAction()` — Initializes a new instance of the GH_ArchivedUndoAction class

### Methods
#### `protected void Deserialize(GH_ISerializable obj)`

Deserializes the obj from the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Deserialize.htm)

#### `protected abstract void Internal_Redo(GH_Document doc)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Internal_Redo.htm)

#### `protected abstract void Internal_Undo(GH_Document doc)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Internal_Undo.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_UndoAction.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Read.htm)

#### `public void Redo(GH_Document doc)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Redo.htm)

#### `protected void Serialize(GH_ISerializable obj)`

Serializes the obj into the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable) — Object to serialize

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Serialize.htm)

#### `protected byte[] SerializeToByteArray(GH_ISerializable obj)`

Serializes the obj into the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable) — Object to serialize

**Returns:** `Byte[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_SerializeToByteArray.htm)

#### `public void Undo(GH_Document doc)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Undo.htm)

#### `public override bool Write(GH_IWriter writer)`

(Overrides GH_UndoAction.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — Override this property if you want the Grasshopper solution to refresh upon undo completion.
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)
- `m_data` (Byte[], get) — Internal data storage for serialized archives.

## GH_ObjectUndoAction (class)

Utility base class for undo actions that operate on a single IGH_DocumentObject.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_ObjectUndoAction.htm)

### Constructors
- `protected GH_ObjectUndoAction(Guid object_id)` — Initializes a new instance of the GH_ObjectUndoAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected abstract void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Object_Redo.htm)

#### `protected abstract void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Object_Undo.htm)

#### `public virtual bool Read(GH_IReader reader)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Read.htm)

#### `public void Redo(GH_Document doc)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Redo.htm)

#### `public void Undo(GH_Document doc)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Undo.htm)

#### `public virtual bool Write(GH_IWriter writer)`

(Inherited from GH_UndoAction.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — Override this property if you want the Grasshopper solution to refresh upon undo completion.
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_UndoAction (class)

Base class implementation for undo actions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_UndoAction.htm)

### Constructors
- `protected GH_UndoAction()` — Initializes a new instance of the GH_UndoAction class

### Methods
#### `protected abstract void Internal_Redo(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Internal_Redo.htm)

#### `protected abstract void Internal_Undo(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Internal_Undo.htm)

#### `public virtual bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Read.htm)

#### `public void Redo(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Redo.htm)

#### `public void Undo(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Undo.htm)

#### `public virtual bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — Override this property if you want the Grasshopper solution to refresh upon undo completion.
- `State` (GH_UndoState, get) — 

## GH_UndoException (class)

Exception associated with undo events

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_UndoException.htm)

### Constructors
- `public GH_UndoException(string message)` — Initializes a new instance of the GH_UndoException class
- `public GH_UndoException(string message, params Object[] args)` — Initializes a new instance of the GH_UndoException class

## GH_UndoRecord (class)

Represents a single undo/redo record. A single record may contain any number of undo actions.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_UndoRecord.htm)

### Constructors
- `public GH_UndoRecord()` — Initializes a new instance of the GH_UndoRecord class
- `public GH_UndoRecord(string name)` — Initializes a new instance of the GH_UndoRecord class
- `public GH_UndoRecord(string name, IGH_UndoAction action)` — Initializes a new instance of the GH_UndoRecord class
- `public GH_UndoRecord(string name, params IGH_UndoAction[] actions)` — Initializes a new instance of the GH_UndoRecord class
- `public GH_UndoRecord(string name, IEnumerable<IGH_UndoAction> actions)` — Initializes a new instance of the GH_UndoRecord class

### Methods
#### `public void AddAction(IGH_UndoAction action)`

Append an undo action to this record. You should only do this prior to calling undo for the first time.

**Parameters:**
- `action` (Grasshopper.Kernel.Undo.IGH_UndoAction) — Action to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoRecord_AddAction.htm)

#### `public void Redo(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoRecord_Redo.htm)

#### `public void Undo(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoRecord_Undo.htm)

### Properties
- `ActionCount` (Int32, get) — Gets the number of actions stored in this record.
- `Actions` (IList<IGH_UndoAction>, get) — Gets or sets the actions inside this record.
- `ExpiresDisplay` (Boolean, get) — Gets the display expiration flag for this event. If True, the Rhino viewports will be redrawn once the entire undo record has been completed. If ExpiresSolution is set to true, ExpriresDisplay is implied.
- `ExpiresSolution` (Boolean, get) — Gets the solution expiration flag for this event. If True, the solution needs to be recalculated once the entire undo record has been completed.
- `Guid` (Guid, get) — Gets the ID of this undo item. Every undo record has a unique ID.
- `Name` (String, get/set) — Gets or sets the name of the undo record (as displayed in the menu)
- `State` (GH_UndoState, get) — Gets the undo state of this record. The state dictates which action are legal.
- `Time` (DateTime, get) — Gets the time at which this record was created.

## GH_UndoServer (class)

Provides access to a documents undo data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_UndoServer.htm)

### Constructors
- `public GH_UndoServer(GH_Document owner)` — Initializes a new instance of the GH_UndoServer class

### Methods
#### `public void AppendToDebugLog(GH_DebugDescriptionWriter writer)`



**Parameters:**
- `writer` (Grasshopper.Kernel.GH_DebugDescriptionWriter)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_AppendToDebugLog.htm)

#### `public void Clear()`

Clear both undo and redo lists.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_Clear.htm)

#### `public void ClearRedo()`

Clear the undo list.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_ClearRedo.htm)

#### `public void ClearUndo()`

Clear the redo list.

**Remarks:** See AlsoReferenceGH_UndoServer ClassGrasshopper.Kernel.Undo Namespace

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_ClearUndo.htm)

#### `public bool MergeRecords(int count)`

Attempt to merge the N most recent records into one. The name of the merged record will be identical to the name of the oldest record.

**Parameters:**
- `count` (System.Int32) — Number of most recent records to merge.

**Returns:** `Boolean` — True if merge was successful.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_MergeRecords.htm)

#### `public void PerformRedo()`

Performs a single Redo step if possible and migrates the record onto the undo stack. This function may throw all kinds of exceptions, if you're calling it from a UI thread, use a Try..Catch block to prevent crashes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_PerformRedo.htm)

#### `public void PerformUndo()`

Performs a single Undo step when possible and migrates the record onto the redo stack. This function may throw all kinds of exceptions, if you're calling it from a UI thread, use a Try..Catch block to prevent crashes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_PerformUndo.htm)

#### `public void PushUndoRecord(GH_UndoRecord record)`

Add a new undo record to the undo stack, this function clears the Redo stack.

**Parameters:**
- `record` (Grasshopper.Kernel.Undo.GH_UndoRecord) — Record to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_PushUndoRecord.htm)

#### `public Guid PushUndoRecord(string name, GH_UndoAction action)`

Add a new undo record to the undo stack, this function clears the Redo stack.

**Parameters:**
- `name` (System.String) — Name of undo record.
- `action` (Grasshopper.Kernel.Undo.GH_UndoAction) — Undo action.

**Returns:** `Guid` — The ID of the newly created undo record.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_PushUndoRecord_1.htm)

#### `public bool RemoveRecord(Guid id)`

Remove the record with the specified ID from the undo or redo stack.

**Parameters:**
- `id` (System.Guid) — ID of record to remove.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_UndoServer_RemoveRecord.htm)

### Properties
- `FirstRedoName` (String, get) — Gets the name of the last item in the redo stack (the first item to be redone).
- `FirstUndoName` (String, get) — Gets the name of the last item in the undo stack (the first item to be undone).
- `MaxRecords` (Int32, get/set) — Gets or sets the maximum number of undo records that can be stored.
- `RedoCount` (Int32, get) — Number of redo records currently in the stack.
- `RedoGuids` (List<Guid>, get) — Gets a sorted list of all the redo Guids in this server.
- `RedoNames` (List<String>, get) — Gets a sorted list of all the redo Guids in this server.
- `UndoCount` (Int32, get) — Number of undo records currently in the stack.
- `UndoGuids` (List<Guid>, get) — Gets a sorted list of all the undo Guids in this server.
- `UndoNames` (List<String>, get) — Gets a sorted list of all the undo Guids in this server.

## GH_UndoState (enum)

State enumeration for undo records.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_GH_UndoState.htm)

### Values
- `undo` = `0` — Record can be undone.
- `redo` = `1` — Record can be redone

## IGH_UndoAction (interface)

Base interface for all undo/redo actions

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_IGH_UndoAction.htm)

### Methods
#### `bool Read(GH_IReader reader)`

This method is called whenever the instance is required to deserialize itself.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Reader object to deserialize from.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Read.htm)

#### `void Redo(GH_Document doc)`

Redo the action stored in this record.

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document) — The target document.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_IGH_UndoAction_Redo.htm)

#### `void Undo(GH_Document doc)`

Undo the action stored in this record.

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document) — The target document.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_IGH_UndoAction_Undo.htm)

#### `bool Write(GH_IWriter writer)`

This method is called whenever the instance is required to serialize itself.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer object to serialize with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Gets the display expiration flag for this event. If True, the Rhino viewports will be redrawn once the entire undo record has been completed. If ExpiresSolution is set to true, ExpriresDisplay is implied.
- `ExpiresSolution` (Boolean, get) — Gets the solution expiration flag for this event. If True, the solution needs to be recalculated once the entire undo record has been completed.
- `State` (GH_UndoState, get) — Gets the current state of the undo action.

