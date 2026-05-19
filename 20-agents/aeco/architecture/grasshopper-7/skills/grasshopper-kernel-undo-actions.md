---
name: grasshopper-grasshopper-kernel-undo-actions
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Undo.Actions namespace — 17 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_AddStateAction, GH_AddObjectAction, GH_DataModificationAction, GH_DataMatchingAction, GH_HiddenAction, GH_GenericObjectAction, GH_IconDisplayAction, GH_IconOverrideAction, and 9 more types.
---

# Grasshopper.Kernel.Undo.Actions

Auto-generated from vendor docs for grasshopper 7.0. 17 types in this namespace.

## GH_AddObjectAction (class)

Records a single object addition.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_AddObjectAction.htm)

### Constructors
- `public GH_AddObjectAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_AddObjectAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_AddObjectAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_AddObjectAction_Internal_Undo.htm)

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
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_AddStateAction (class)

Record the addition of a single solution state to a document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_AddStateAction.htm)

### Constructors
- `public GH_AddStateAction(int index, GH_State state)` — Initializes a new instance of the GH_AddStateAction class

### Methods
#### `protected void Deserialize(GH_ISerializable obj)`

Deserializes the obj from the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Deserialize.htm)

#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_AddStateAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_AddStateAction_Internal_Undo.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_ArchivedUndoAction.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_AddStateAction_Read.htm)

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

(Overrides GH_ArchivedUndoAction.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_AddStateAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — Override this property if you want the Grasshopper solution to refresh upon undo completion.
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)
- `m_data` (Byte[], get) — Internal data storage for serialized archives.

## GH_DataMatchingAction (class)

Records a single Component DataComparison event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_DataMatchingAction.htm)

### Constructors
- `public GH_DataMatchingAction(IGH_Component obj)` — Initializes a new instance of the GH_DataMatchingAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_DataMatchingAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_DataMatchingAction_Object_Undo.htm)

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
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_DataModificationAction (class)

Records a single Parameter DataModification event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_DataModificationAction.htm)

### Constructors
- `public GH_DataModificationAction(IGH_Param obj)` — Initializes a new instance of the GH_DataModificationAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_DataModificationAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_DataModificationAction_Object_Undo.htm)

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
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_GenericObjectAction (class)

Records a single object change. Object changes are undone/redone by (de)serializing the objects. Object changes also keep track of all wires feeding into and coming out of the object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_GenericObjectAction.htm)

### Constructors
- `public GH_GenericObjectAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_GenericObjectAction class

### Methods
#### `protected void Deserialize(GH_ISerializable obj)`

Deserializes the obj from the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Deserialize.htm)

#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_GenericObjectAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_GenericObjectAction_Internal_Undo.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_ArchivedUndoAction.)

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

(Inherited from GH_ArchivedUndoAction.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)
- `m_data` (Byte[], get) — Internal data storage for serialized archives.

## GH_HiddenAction (class)

Records a single object Preview event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_HiddenAction.htm)

### Constructors
- `public GH_HiddenAction(IGH_ActiveObject obj)` — Initializes a new instance of the GH_HiddenAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_HiddenAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_HiddenAction_Object_Undo.htm)

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
- `ExpiresDisplay` (Boolean, get) — (Overrides GH_UndoAction.ExpiresDisplay.)
- `ExpiresSolution` (Boolean, get) — Override this property if you want the Grasshopper solution to refresh upon undo completion.
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_IconDisplayAction (class)

Records a single icon display mode event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_IconDisplayAction.htm)

### Constructors
- `public GH_IconDisplayAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_IconDisplayAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_IconDisplayAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_IconDisplayAction_Object_Undo.htm)

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

## GH_IconOverrideAction (class)

Records a single icon override event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_IconOverrideAction.htm)

### Constructors
- `public GH_IconOverrideAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_IconOverrideAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_IconOverrideAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_IconOverrideAction_Object_Undo.htm)

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

## GH_LayoutAction (class)

Records a single object layout event. Useful for tracking object resizes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_LayoutAction.htm)

### Constructors
- `public GH_LayoutAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_LayoutAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_LayoutAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_LayoutAction_Object_Undo.htm)

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

## GH_LockedAction (class)

Records a single object Enabled event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_LockedAction.htm)

### Constructors
- `public GH_LockedAction(IGH_ActiveObject obj)` — Initializes a new instance of the GH_LockedAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_LockedAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_LockedAction_Object_Undo.htm)

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
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_NickNameAction (class)

Records a single object NickName and IconDisplayMode event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_NickNameAction.htm)

### Constructors
- `public GH_NickNameAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_NickNameAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_NickNameAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_NickNameAction_Object_Undo.htm)

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

## GH_PersistentDataAction<T> (class)

Records the changes in the persistent data of a parameter.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_PersistentDataAction_1.htm)

### Constructors
- `public GH_PersistentDataAction(GH_PersistentParam<T> obj)` — Initializes a new instance of the GH_PersistentDataAction<T> class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_PersistentDataAction_1_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_PersistentDataAction_1_Object_Undo.htm)

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
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_PivotAction (class)

Records a single object pivot event. Useful for tracking object movement.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_PivotAction.htm)

### Constructors
- `public GH_PivotAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_PivotAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_PivotAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_PivotAction_Object_Undo.htm)

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

## GH_RemoveObjectAction (class)

Records a single object removal.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_RemoveObjectAction.htm)

### Constructors
- `public GH_RemoveObjectAction(IGH_DocumentObject obj)` — Initializes a new instance of the GH_RemoveObjectAction class

### Methods
#### `protected void Deserialize(GH_ISerializable obj)`

Deserializes the obj from the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Deserialize.htm)

#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_RemoveObjectAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_RemoveObjectAction_Internal_Undo.htm)

#### `public override bool Read(GH_IReader reader)`

(Inherited from GH_ArchivedUndoAction.)

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

(Inherited from GH_ArchivedUndoAction.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)
- `m_data` (Byte[], get) — Internal data storage for serialized archives.

## GH_RemoveStateAction (class)

Record the removal of a single solution state from a document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_RemoveStateAction.htm)

### Constructors
- `public GH_RemoveStateAction(int index, GH_State state)` — Initializes a new instance of the GH_RemoveStateAction class

### Methods
#### `protected void Deserialize(GH_ISerializable obj)`

Deserializes the obj from the local archive.

**Parameters:**
- `obj` (GH_IO.GH_ISerializable)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ArchivedUndoAction_Deserialize.htm)

#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_RemoveStateAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_RemoveStateAction_Internal_Undo.htm)

#### `public override bool Read(GH_IReader reader)`

(Overrides GH_ArchivedUndoAction.Read(GH_IReader).)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_RemoveStateAction_Read.htm)

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

(Overrides GH_ArchivedUndoAction.Write(GH_IWriter).)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_RemoveStateAction_Write.htm)

### Properties
- `ExpiresDisplay` (Boolean, get) — Override this property if you want the Rhino viewport display to refresh upon undo completion.
- `ExpiresSolution` (Boolean, get) — Override this property if you want the Grasshopper solution to refresh upon undo completion.
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)
- `m_data` (Byte[], get) — Internal data storage for serialized archives.

## GH_WireAction (class)

Records any change to the input wires of a given parameter.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_WireAction.htm)

### Constructors
- `public GH_WireAction(IGH_Param param)` — Initializes a new instance of the GH_WireAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Redo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_WireAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Overrides GH_UndoAction.Internal_Undo(GH_Document).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_WireAction_Internal_Undo.htm)

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
- `ExpiresSolution` (Boolean, get) — (Overrides GH_UndoAction.ExpiresSolution.)
- `State` (GH_UndoState, get) — (Inherited from GH_UndoAction.)

## GH_WireDisplayAction (class)

Records a single wire display mode event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Undo_Actions_GH_WireDisplayAction.htm)

### Constructors
- `public GH_WireDisplayAction(IGH_Param obj)` — Initializes a new instance of the GH_WireDisplayAction class

### Methods
#### `protected override void Internal_Redo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Redo.htm)

#### `protected override void Internal_Undo(GH_Document doc)`

(Inherited from GH_ObjectUndoAction.)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_GH_ObjectUndoAction_Internal_Undo.htm)

#### `protected override void Object_Redo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Redo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_WireDisplayAction_Object_Redo.htm)

#### `protected override void Object_Undo(GH_Document doc, IGH_DocumentObject obj)`

(Overrides GH_ObjectUndoAction.Object_Undo(GH_Document, IGH_DocumentObject).)

**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `obj` (Grasshopper.Kernel.IGH_DocumentObject)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Undo_Actions_GH_WireDisplayAction_Object_Undo.htm)

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

