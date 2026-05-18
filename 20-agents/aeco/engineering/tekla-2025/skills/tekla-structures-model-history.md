---
name: tekla-tekla-structures-model-history
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Model.History namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModificationStamp, ModelHistory, ModificationInfo, ModifiedObjectInfo.
---

# Tekla.Structures.Model.History

Auto-generated from vendor docs for tekla 2025.0. 4 types in this namespace.

## ModelHistory (class)

The ModelHistory class provides history information about the objects of the model that is currently open in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5a668281-0f9c-ade2-1c56-79c088d75a34)

### Methods
#### `GetCurrentModificationStamp(...)`

Gets the current modification stamp from the model.

[Docs](https://developer.tekla.com/topic/en/18/43/28d800d0-50b6-521e-b252-add1b7a3baef)

#### `GetDeletedObjects(...)`

Gets a list of the objects that have been deleted after the modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/43/8e9cf21c-7ac8-3fa2-40a0-25a3bee15316)

#### `GetDeletedObjectsWithType(...)`

Gets based on type a list of the objects that have been deleted after the modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/43/e1c9f788-42fe-130d-13aa-265d640884b7)

#### `GetLocalChanges(...)`

Get changes which are not written out.

[Docs](https://developer.tekla.com/topic/en/18/43/f8ecb33f-ac31-cc71-f7fc-016d53149a5c)

#### `GetModifications(String, IEnumerable.ModelObject.ModelObjectEnum., ModificationStamp)(...)`

Lookup modifications since previous call of TakeModifications() without resetting the modstamp, with object type filtering

[Docs](https://developer.tekla.com/topic/en/18/43/8c2480d8-c493-0d4b-fee8-9671f447392d)

#### `GetModifications(String, ModificationStamp)(...)`

Lookup modifications since previous call of TakeModifications() without resetting the modstamp

[Docs](https://developer.tekla.com/topic/en/18/43/f11c9055-8f2d-0b10-c2ba-4c8ee603e71f)

#### `GetModifiedObjects(...)`

Gets a list of the objects that have been added or modified after the modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/43/a8255b6d-95d4-e938-e3a2-c88e86018c8f)

#### `GetModifiedObjectsWithType(...)`

Gets based on type a list of the objects that have been added or modified after the modification stamp.

[Docs](https://developer.tekla.com/topic/en/18/43/03657be1-bce3-21b4-6563-ad3c91ffdf5f)

#### `GetNotSharedObjects(...)`

Get object which are was created or modified since last ModelSharing WriteOut.

[Docs](https://developer.tekla.com/topic/en/18/43/416c88f3-4e62-2ec5-ec11-9a2b10921f3e)

#### `TakeModifications(String, IEnumerable.ModelObject.ModelObjectEnum., ModificationStamp)(...)`

Take modifications since previous call, with object type filtering.

[Docs](https://developer.tekla.com/topic/en/18/43/780d29e6-388d-2137-c685-b603abcbeb7f)

#### `TakeModifications(String, ModificationStamp)(...)`

Take modifications since previous call.

[Docs](https://developer.tekla.com/topic/en/18/43/bef670da-b23b-8b62-bcc6-259c2bc5c819)

#### `UpdateModificationStampToLatest(...)`

Updates the modification stamp to latest.

[Docs](https://developer.tekla.com/topic/en/18/43/27c4394a-6ec7-41e4-ddc0-48de631b64a6)

## ModificationInfo (struct)

Modification info returned by History.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/86390ddc-23f8-85bc-6542-260025ddcb52)

## ModificationStamp (class)

The ModificationStamp class provides the modification stamp for the objects of the model that is currently open in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d09ad3fe-1eb4-eb58-4017-0eed1f2e042a)

### Constructors
- `ModificationStamp(...)` — Creates a new modification stamp instance where the current modification stamp is 0. Can be used for fetching all the created or deleted model objects from the model history.
- `ModificationStamp(...)` — Creates a new modification stamp instance with the given local and server stamps. Can be used for fetching from the model history all the model objects that have been created or deleted after the specified stamp.

### Properties
- `Guid` (object, get/set) — Guid representation
- `LocalStamp` (object, get/set) — The modification stamp for the local changes.
- `ServerStamp` (object, get/set) — The modification stamp from the multi-user server for the external changes caused by the saving of the multi-user model.

## ModifiedObjectInfo (struct)

Modification info returned by History.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f6d9a86b-4488-1d50-ad4c-2c74d8a42f75)

