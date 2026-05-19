---
name: tekla-tekla-structures-model-history
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Model.History namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelHistory, ModificationInfo, ModificationStamp, ModifiedObjectInfo.
---

# Tekla.Structures.Model.History

Auto-generated from vendor docs for tekla 2026.0. 4 types in this namespace.

## ModelHistory (class)

The ModelHistory class provides history information about the objects of the model that is currently open in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5a668281-0f9c-ade2-1c56-79c088d75a34)

### Methods
#### `[ObsoleteAttribute("Use TakeModifications() instead.")] public static ModificationStamp GetCurrentModificationStamp()`

Gets the current modification stamp from the model.

**Returns:** `ModificationStamp` — The current modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/47/28d800d0-50b6-521e-b252-add1b7a3baef)

#### `[ObsoleteAttribute("Use TakeModifications() instead.")] public static ModelObjectEnumerator GetDeletedObjects(ModificationStamp ModStamp)`

Gets a list of the objects that have been deleted after the modification stamp.

**Parameters:**
- `ModStamp` (Tekla.Structures.Model.History.ModificationStamp) — The modification stamp.

**Returns:** `ModelObjectEnumerator` — A list of the objects that have been deleted after the given modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/47/8e9cf21c-7ac8-3fa2-40a0-25a3bee15316)

#### `[ObsoleteAttribute("Use TakeModifications() instead.")] public static ModelObjectEnumerator GetDeletedObjectsWithType(ModificationStamp ModStamp, ModelObject.ModelObjectEnum Enum)`

Gets based on type a list of the objects that have been deleted after the modification stamp.

**Parameters:**
- `ModStamp` (Tekla.Structures.Model.History.ModificationStamp) — The modification stamp.
- `Enum` (Tekla.Structures.Model.ModelObject.ModelObjectEnum) — The type of the objects to return.

**Returns:** `ModelObjectEnumerator` — A list of the objects that have been deleted after the given modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/47/e1c9f788-42fe-130d-13aa-265d640884b7)

#### `public static ModificationInfo GetLocalChanges()`

Get changes which are not written out.

**Returns:** `ModificationInfo` — Modifications which are not written out.

[Docs](https://developer.tekla.com/topic/en/18/47/f8ecb33f-ac31-cc71-f7fc-016d53149a5c)

#### `public static ModificationInfo GetModifications(string Name, IEnumerable<ModelObject.ModelObjectEnum> ObjectTypes, ModificationStamp PrevStamp = null)`

Lookup modifications since previous call of TakeModifications() without resetting the modstamp, with object type filtering

**Remarks:** Same as GetModifications(String, ModificationStamp), but returns only the types listed in ObjectTypes.

**Parameters:**
- `Name` (System.String) — Name of reference which is used for collecting modifications and which is updated. Intended use is to assign an unique name for each client application which needs to be informed of the model updates. Maximum length is 79 characters.
- `ObjectTypes` (System.Collections.Generic.IEnumerable<ModelObject.ModelObjectEnum>) — Types of object to select. Empty or null selects all types.
- `PrevStamp` (Tekla.Structures.Model.History.ModificationStamp) — When specified, default value of reference if is it not found

**Returns:** `ModificationInfo` — All modifications which happened since the last call

[Docs](https://developer.tekla.com/topic/en/18/47/8c2480d8-c493-0d4b-fee8-9671f447392d)

#### `public static ModificationInfo GetModifications(string Name, ModificationStamp PrevStamp = null)`

Lookup modifications since previous call of TakeModifications() without resetting the modstamp

**Remarks:** Takes modification which has happened to the model since the previous call to TakeModifications with the same Name argument. If there was no previous call of TakeModifications() with this Name, shows modifications since beginning, or since PrevStap, if it is specified.

**Parameters:**
- `Name` (System.String) — Name of reference which is used for collecting modifications and which is updated. Intended use is to assign an unique name for each client application which needs to be informed of the model updates. Maximum length is 79 characters.
- `PrevStamp` (Tekla.Structures.Model.History.ModificationStamp) — When specified, default value of reference if is it not found

**Returns:** `ModificationInfo` — All modifications which happened since the last call

[Docs](https://developer.tekla.com/topic/en/18/47/f11c9055-8f2d-0b10-c2ba-4c8ee603e71f)

#### `[ObsoleteAttribute("Use TakeModifications() instead.")] public static ModelObjectEnumerator GetModifiedObjects(ModificationStamp ModStamp)`

Gets a list of the objects that have been added or modified after the modification stamp.

**Parameters:**
- `ModStamp` (Tekla.Structures.Model.History.ModificationStamp) — The modification stamp.

**Returns:** `ModelObjectEnumerator` — A list of the objects that have been modified after the given modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/47/a8255b6d-95d4-e938-e3a2-c88e86018c8f)

#### `[ObsoleteAttribute("Use TakeModifications() instead.")] public static ModelObjectEnumerator GetModifiedObjectsWithType(ModificationStamp ModStamp, ModelObject.ModelObjectEnum Enum)`

Gets based on type a list of the objects that have been added or modified after the modification stamp.

**Parameters:**
- `ModStamp` (Tekla.Structures.Model.History.ModificationStamp) — The modification stamp.
- `Enum` (Tekla.Structures.Model.ModelObject.ModelObjectEnum) — The type of the objects to return.

**Returns:** `ModelObjectEnumerator` — A list of the objects that have been modified after the given modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/47/03657be1-bce3-21b4-6563-ad3c91ffdf5f)

#### `public static ModelObjectEnumerator GetNotSharedObjects()`

Get object which are was created or modified since last ModelSharing WriteOut.

**Remarks:** If the model is not shared, returns all objects.

**Returns:** `ModelObjectEnumerator` — A list of objects

[Docs](https://developer.tekla.com/topic/en/18/47/416c88f3-4e62-2ec5-ec11-9a2b10921f3e)

#### `public static ModificationInfo TakeModifications(string Name, IEnumerable<ModelObject.ModelObjectEnum> ObjectTypes, ModificationStamp PrevStamp = null)`

Take modifications since previous call, with object type filtering.

**Remarks:** Same as TakeModifications(String, ModificationStamp), but returns only the types listed in ObjectTypes. Note that since this function also sets base for the next invocation of for ALL objects, so using it discards change information for filtered objects.

**Parameters:**
- `Name` (System.String) — Name of reference which is used for collecting modifications and which is updated. Intended use is to assign an unique name for each client application which needs to be informed of the model updates. Maximum length is 79 characters.
- `ObjectTypes` (System.Collections.Generic.IEnumerable<ModelObject.ModelObjectEnum>) — Types of object to select. Empty or null selects all types.
- `PrevStamp` (Tekla.Structures.Model.History.ModificationStamp) — When specified, default value of reference if is it not found

**Returns:** `ModificationInfo` — All modifications which happened since the last call. See ModificationInfo reference

[Docs](https://developer.tekla.com/topic/en/18/47/780d29e6-388d-2137-c685-b603abcbeb7f)

#### `public static ModificationInfo TakeModifications(string Name, ModificationStamp PrevStamp = null)`

Take modifications since previous call.

**Remarks:** Takes modification which has happened to the model since the previous call to TakeModifications with the same Name argument. If there was no previous call with this Name, shows modifications since beginning, or since PrevStap, if it is specified. Updates internal reference for use when TakeModifications() is called next time.

**Parameters:**
- `Name` (System.String) — Name of reference which is used for collecting modifications and which is updated. Intended use is to assign an unique name for each client application which needs to be informed of the model updates. Maximum length is 79 characters.
- `PrevStamp` (Tekla.Structures.Model.History.ModificationStamp) — When specified, default value of reference if is it not found

**Returns:** `ModificationInfo` — All modifications which happened since the last call. See ModificationInfo reference

[Docs](https://developer.tekla.com/topic/en/18/47/bef670da-b23b-8b62-bcc6-259c2bc5c819)

#### `public static void UpdateModificationStampToLatest(string modificationStampKey)`

Updates the modification stamp to latest.

**Parameters:**
- `modificationStampKey` (System.String) — The modification stamp key.

[Docs](https://developer.tekla.com/topic/en/18/47/27c4394a-6ec7-41e4-ddc0-48de631b64a6)

## ModificationInfo (struct)

Modification info returned by History.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/86390ddc-23f8-85bc-6542-260025ddcb52)

## ModificationStamp (class)

The ModificationStamp class provides the modification stamp for the objects of the model that is currently open in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d09ad3fe-1eb4-eb58-4017-0eed1f2e042a)

### Constructors
- `public ModificationStamp()` — Creates a new modification stamp instance where the current modification stamp is 0. Can be used for fetching all the created or deleted model objects from the model history.
- `public ModificationStamp(int LocalStamp, int ServerStamp)` — Creates a new modification stamp instance with the given local and server stamps. Can be used for fetching from the model history all the model objects that have been created or deleted after the specified stamp.

### Properties
- `Guid` (String, get) — Guid representation
- `LocalStamp` (Int32, get/set) — The modification stamp for the local changes.
- `ServerStamp` (Int32, get/set) — The modification stamp from the multi-user server for the external changes caused by the saving of the multi-user model.

## ModifiedObjectInfo (struct)

Modification info returned by History.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f6d9a86b-4488-1d50-ad4c-2c74d8a42f75)

