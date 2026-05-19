---
name: rhino-rhino-docobjects-tables
description: This skill encodes the rhino 7.0 surface of the Rhino.DocObjects.Tables namespace — 34 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BitmapTable, DimStyleTable, DimStyleTableEventArgs, FontTable, GroupTableEventArgs, GroupTable, InstanceDefinitionTable, LayerTable, and 26 more types.
---

# Rhino.DocObjects.Tables

Auto-generated from vendor docs for rhino 7.0. 34 types in this namespace.

## BitmapTable (class)

Stores the list of bitmaps in a Rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_BitmapTable.htm)

### Methods
#### `public int AddBitmap(string bitmapFilename, bool replaceExisting)`

Adds a new bitmap with specified name to the bitmap table.

**Parameters:**
- `bitmapFilename` (System.String) — If NULL or empty, then a unique name of the form "Bitmap 01" will be automatically created.
- `replaceExisting` (System.Boolean) — If true and the there is already a bitmap using the specified name, then that bitmap is replaced. If false and there is already a bitmap using the specified name, then -1 is returned.

**Returns:** `Int32` — index of new bitmap in table on success. -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_AddBitmap.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public override bool Delete(BitmapEntry item)`

Removes the bitmap from the table.

**Parameters:**
- `item` (Rhino.DocObjects.BitmapEntry) — The item to remove. Null will always return false.

**Returns:** `Boolean` — True if the item could be deleted; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_Delete.htm)

#### `public bool DeleteBitmap(string bitmapFilename)`

Deletes a bitmap.

**Parameters:**
- `bitmapFilename` (System.String) — The bitmap file name.

**Returns:** `Boolean` — true if successful. false if the bitmap cannot be deleted because it is the current bitmap or because it bitmap contains active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_DeleteBitmap.htm)

#### `public bool ExportToFile(int index, string path)`

Writes a bitmap to a file.

**Parameters:**
- `index` (System.Int32) — The index of the bitmap to be written.
- `path` (System.String) — The full path, including file name and extension, name of the file to write.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_ExportToFile.htm)

#### `public int ExportToFiles(string directoryPath, int overwrite)`

Exports all the bitmaps in the table to files.

**Parameters:**
- `directoryPath` (System.String) — full path to the directory where the bitmaps should be saved. If NULL, a dialog is used to interactively get the directory name.
- `overwrite` (System.Int32) — 0 = no, 1 = yes, 2 = ask.

**Returns:** `Int32` — Number of bitmaps written.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_ExportToFiles.htm)

#### `public BitmapEntry Find(string name, bool createFile, out string fileName)`

This function first attempts to find the file with "name" on the disk. If it does find it, "fileName" is set to the full path of the file and the BitmapEntry returned will be null, even if there was a BitmapEntry with "name" in the bitmap table. If the function cannot find the file on the disk, it searches the bitmap table. If it finds it, the returned BitmapEntry entry will be the entry in the table with that name. Additionally, if "createFile" is true, and an entry is found, the file will be written to the disk and it's full path will be contained in "fileName".

**Parameters:**
- `name` (System.String) — Name of the file to search for including file extension.
- `createFile` (System.Boolean) — If this is true, and the file is not found on the disk but is found in the BitmapTable, then the BitmapEntry will get saved to the Rhino bitmap file cache and fileName will contain the full path to the cached file.
- `fileName` (System.String) — The full path to the current location of this file or an empty string if the file was not found and/or not extracted successfully.

**Returns:** `BitmapEntry` — Returns null if "name" was found on the disk. If name was not found on the disk, returns the BitmapEntry with the specified name if it is found in the bitmap table and null if it was not found in the bitmap table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_Find.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public BitmapEntry FindIndex(int index)`

Retrieves a BitmapEntry object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `BitmapEntry` — A BitmapEntry object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_BitmapTable_FindIndex.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns Image.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (BitmapEntry, get) — Conceptually, the bitmap table is an array of bitmaps. The operator[] can be used to get individual bitmaps.

## DimStyleTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.DimStyleTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_DimStyleTable.htm)

### Methods
#### `public int Add(DimensionStyle dimstyle, bool reference)`

Adds a new DimensionStyle to the document.

**Parameters:**
- `dimstyle` (Rhino.DocObjects.DimensionStyle) — The dimension style to add
- `reference` (System.Boolean) — if true the dimstyle will not be saved in files.

**Returns:** `Int32` — index of new AnnotationStyle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Add.htm)

#### `public int Add(string name)`

Adds a new AnnotationStyle to the document. The new AnnotationStyle will be initialized with the current default AnnotationStyle properties.

**Parameters:**
- `name` (System.String) — Name of the new AnnotationStyle. If null or empty, Rhino automatically generates the name.

**Returns:** `Int32` — index of new AnnotationStyle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Add_1.htm)

#### `public int Add(string name, bool reference)`

Adds a new AnnotationStyle to the document. The new AnnotationStyle will be initialized with the current default AnnotationStyle properties.

**Parameters:**
- `name` (System.String) — Name of the new AnnotationStyle. If null or empty, Rhino automatically generates the name.
- `reference` (System.Boolean) — if true the dimstyle will not be saved in files.

**Returns:** `Int32` — index of new AnnotationStyle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Add_2.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public override bool Delete(DimensionStyle item)`

Removes an annotation style.

**Parameters:**
- `item` (Rhino.DocObjects.DimensionStyle) — The item to remove.

**Returns:** `Boolean` — True if the item was removed; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Delete.htm)

#### `public bool Delete(int index, bool quiet)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Delete(System.Int32,System.Boolean)"]
- `quiet` (System.Boolean) — [Missing <param name="quiet"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Delete(System.Int32,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Delete(System.Int32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Delete_1.htm)

#### `public DimensionStyle Find(Guid styleId, bool ignoreDeleted)`



**Parameters:**
- `styleId` (System.Guid) — [Missing <param name="styleId"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Find(System.Guid,System.Boolean)"]
- `ignoreDeleted` (System.Boolean) — [Missing <param name="ignoreDeleted"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Find(System.Guid,System.Boolean)"]

**Returns:** `DimensionStyle` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Find(System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Find.htm)

#### `[ObsoleteAttribute("ignoreDeleted is always considered true now. Use FindName.")] public DimensionStyle Find(string name, bool ignoreDeleted)`

Obsolete.

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Find(System.String,System.Boolean)"]
- `ignoreDeleted` (System.Boolean) — [Missing <param name="ignoreDeleted"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Find(System.String,System.Boolean)"]

**Returns:** `DimensionStyle` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Find(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Find_1.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public DimensionStyle FindIndex(int index)`

Retrieves a DimensionStyle object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `DimensionStyle` — A DimensionStyle object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_FindIndex.htm)

#### `public DimensionStyle FindName(string name)`

Finds the DimensionStyle with a given name and returns it. None is returned if no DimensionStyle is found.

**Parameters:**
- `name` (System.String) — The string to search. Deleted styles are ignored.

**Returns:** `DimensionStyle` — The instance, or null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_FindName.htm)

#### `public DimensionStyle FindRoot(Guid styleId, bool ignoreDeleted)`



**Parameters:**
- `styleId` (System.Guid) — [Missing <param name="styleId"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.FindRoot(System.Guid,System.Boolean)"]
- `ignoreDeleted` (System.Boolean) — [Missing <param name="ignoreDeleted"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.FindRoot(System.Guid,System.Boolean)"]

**Returns:** `DimensionStyle` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.FindRoot(System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_FindRoot.htm)

#### `public override IEnumerator<DimensionStyle> GetEnumerator()`

(Overrides CommonComponentTable<T>.GetEnumerator().)

**Returns:** `IEnumerator<DimensionStyle>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_GetEnumerator.htm)

#### `public string GetUnusedStyleName()`

Get a unique name for a style that does not already exist in the DimStyle table

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.GetUnusedStyleName"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_GetUnusedStyleName.htm)

#### `public string GetUnusedStyleName(string rootName)`

Get a unique name for a dimension style that does not already exist in the DimStyle table

**Parameters:**
- `rootName` (System.String) — prefix in name; typically the parent style name

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.GetUnusedStyleName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_GetUnusedStyleName_1.htm)

#### `public ModifyType Modify(DimensionStyle dimstyle, AnnotationBase annotation)`



**Parameters:**
- `dimstyle` (Rhino.DocObjects.DimensionStyle) — [Missing <param name="dimstyle"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Modify(Rhino.DocObjects.DimensionStyle,Rhino.Geometry.AnnotationBase)"]
- `annotation` (Rhino.Geometry.AnnotationBase) — [Missing <param name="annotation"/> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Modify(Rhino.DocObjects.DimensionStyle,Rhino.Geometry.AnnotationBase)"]

**Returns:** `ModifyType` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.DimStyleTable.Modify(Rhino.DocObjects.DimensionStyle,Rhino.Geometry.AnnotationBase)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Modify.htm)

#### `public bool Modify(DimensionStyle newSettings, Guid dimstyleId, bool quiet)`

Modifies dimension style settings.

**Parameters:**
- `newSettings` (Rhino.DocObjects.DimensionStyle) — This information is copied.
- `dimstyleId` (System.Guid) — Id of dimension style
- `quiet` (System.Boolean) — if true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful. false if Id is not already in table

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Modify_1.htm)

#### `public bool Modify(DimensionStyle newSettings, int dimstyleIndex, bool quiet)`

Modifies dimension style settings.

**Parameters:**
- `newSettings` (Rhino.DocObjects.DimensionStyle) — This information is copied.
- `dimstyleIndex` (System.Int32) — zero based index of dimension to set. Must be in the range 0 <= dimstyleIndex < DimStyleTable.Count.
- `quiet` (System.Boolean) — if true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful. false if dimstyleIndex is out of range

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_Modify_2.htm)

#### `public bool SetCurrent(int index, bool quiet)`

Sets the Current property.

**Parameters:**
- `index` (System.Int32) — The index of the current DimStyle.
- `quiet` (System.Boolean) — true if error dialog boxes are disabled. False if they are enabled.

**Returns:** `Boolean` — true if the method achieved its goal; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_SetCurrent.htm)

#### `[ObsoleteAttribute("Use the SetCurrent property.")] public bool SetCurrentDimensionStyleIndex(int index, bool quiet)`

Do not use. Use the SetCurrent(Int32, Boolean) method.

**Parameters:**
- `index` (System.Int32) — Do not use.
- `quiet` (System.Boolean) — Do not use.

**Returns:** `Boolean` — Do not use.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_DimStyleTable_SetCurrentDimensionStyleIndex.htm)

### Properties
- `BuiltInStyles` (DimensionStyle[], get) — Creates an array of default AnnotationStyle objects
- `ComponentType` (ModelComponentType, get) — Returns DimStyle.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `Current` (DimensionStyle, get) — Returns an instance of the current DimensionStyle.
- `CurrentDimensionStyle` (DimensionStyle, get) — Do not use. Use the Current property.
- `CurrentId` (Guid, get) — 
- `CurrentIndex` (Int32, get) — 
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (DimensionStyle, get) — 

## DimStyleTableEventArgs (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.DimStyleTableEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_DimStyleTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `EventType` (DimStyleTableEventType, get) — 
- `Index` (Int32, get) — 
- `NewState` (DimensionStyle, get) — 
- `OldState` (DimensionStyle, get) — 

## DimStyleTableEventType (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.DimStyleTableEventType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_DimStyleTableEventType.htm)

### Values
- `Added` = `0`
- `Deleted` = `1`
- `Undeleted` = `2`
- `Modified` = `3` — name, color, etc., change
- `Sorted` = `4` — doc.m_dimstyle_table.Sort() potentially changed sort order
- `Current` = `5` — current dim style change

## FontTable (class)

Font tables store the list of fonts in a Rhino document. RemarksThe FontTable is now just a wrapper around the DimStyles table.

**Remarks:** The FontTable is now just a wrapper around the DimStyles table.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_FontTable.htm)

### Methods
#### `public int FindOrCreate(string face, bool bold, bool italic)`

Get a DimensionStyle with the specified characteristics the settings other than face, bold and italic are copied from the current style

**Parameters:**
- `face` (System.String) — [Missing <param name="face"/> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_FontTable_FindOrCreate.htm)

#### `public int FindOrCreate(string face, bool bold, bool italic, DimensionStyle template_style)`

Get a DimensionStyle with the specified characteristics

**Parameters:**
- `face` (System.String) — [Missing <param name="face"/> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean,Rhino.DocObjects.DimensionStyle)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean,Rhino.DocObjects.DimensionStyle)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean,Rhino.DocObjects.DimensionStyle)"]
- `template_style` (Rhino.DocObjects.DimensionStyle) — the settings other than face, bold and italic are copied from the template_style

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.FontTable.FindOrCreate(System.String,System.Boolean,System.Boolean,Rhino.DocObjects.DimensionStyle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_FontTable_FindOrCreate_1.htm)

#### `public IEnumerator<Font> GetEnumerator()`



**Returns:** `IEnumerator<Font>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.FontTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_FontTable_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Number of fonts in the table.
- `CurrentIndex` (Int32, get) — At all times, there is a "current" font. Unless otherwise specified, new dimension objects are assigned to the current font. The current font is never deleted. Returns: Zero based font index of the current font.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (Font, get) — Gets the font at a position.

## GroupTable (class)

Group tables store the list of groups in a Rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_GroupTable.htm)

### Methods
#### `public int Add()`

Adds a new empty group to the group table.

**Remarks:** In some cases, calling Add() can cause the group indices to become invalid.

**Returns:** `Int32` — >=0 index of new group. -1 group not added because a group with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Add.htm)

#### `public int Add(IEnumerable<Guid> objectIds)`

Adds a new group to the group table with a set of objects.

**Remarks:** In some cases, calling Add() can cause the group indices to become invalid.

**Parameters:**
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — An array, a list or any enumerable set of object IDs.

**Returns:** `Int32` — >=0 index of new group. -1 group not added because a group with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Add_1.htm)

#### `public int Add(string groupName)`

Adds a new empty group to the group table.

**Remarks:** In some cases, calling Add() can cause the group indices to become invalid.

**Parameters:**
- `groupName` (System.String) — name of new group.

**Returns:** `Int32` — >=0 index of new group. -1 group not added because a group with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Add_2.htm)

#### `public int Add(string groupName, IEnumerable<Guid> objectIds)`

Adds a new group to the group table with a set of objects.

**Remarks:** In some cases, calling Add() can cause the group indices to become invalid.

**Parameters:**
- `groupName` (System.String) — Name of new group.
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — An array, a list or any enumerable set of object IDs.

**Returns:** `Int32` — >=0 index of new group. -1 group not added because a group with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Add_3.htm)

#### `public bool AddToGroup(int groupIndex, Guid objectId)`

Adds an object to an existing group.

**Parameters:**
- `groupIndex` (System.Int32) — The group index value.
- `objectId` (System.Guid) — An ID of an object.

**Returns:** `Boolean` — true if the operation was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_AddToGroup_1.htm)

#### `public bool AddToGroup(int groupIndex, IEnumerable<Guid> objectIds)`

Adds a list of objects to an existing group.

**Parameters:**
- `groupIndex` (System.Int32) — The group index value.
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — An array, a list or any enumerable set of IDs to objects.

**Returns:** `Boolean` — true if at least an operation was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_AddToGroup.htm)

#### `public bool ChangeGroupName(int groupIndex, string newName)`

Changes the name of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.
- `newName` (System.String) — The new group name.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_ChangeGroupName.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public override bool Delete(Group item)`

(Overrides CommonComponentTable<T>.Delete(T).)

**Parameters:**
- `item` (Rhino.DocObjects.Group) — [Missing <param name="item"/> documentation for "M:Rhino.DocObjects.Tables.GroupTable.Delete(Rhino.DocObjects.Group)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.GroupTable.Delete(Rhino.DocObjects.Group)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Delete.htm)

#### `public bool Delete(int groupIndex)`

Deletes a group from this table. Deleted groups are kept in the runtime group table so that undo will work with groups. Call IsDeleted() to determine if a group is deleted.

**Parameters:**
- `groupIndex` (System.Int32) — An group index to be deleted.

**Returns:** `Boolean` — true if the operation was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Delete_1.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public Group FindIndex(int index)`

Retrieves a Group object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `Group` — A Group object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_FindIndex.htm)

#### `public Group FindName(string name)`

Finds a group given its name. Returns the instance, rather than the index.

**Parameters:**
- `name` (System.String) — The name of the group to be searched.

**Returns:** `Group` — An group, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_FindName.htm)

#### `public Group FindNameHash(NameHash nameHash)`

Finds a group given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the group to be searched.

**Returns:** `Group` — An group, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

#### `public RhinoObject[] GroupMembers(int groupIndex)`

Gets an array of all of the objects in a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group in this table.

**Returns:** `RhinoObject[]` — An array with all the objects in the specified group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_GroupMembers.htm)

#### `public string GroupName(int groupIndex)`

Returns the name of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `String` — The group name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_GroupName.htm)

#### `public string[] GroupNames(bool ignoreDeletedGroups)`

Returns an array of all group names.

**Parameters:**
- `ignoreDeletedGroups` (System.Boolean) — Ignore any groups that were deleted.

**Returns:** `String[]` — An array if group names if succesful, null if there are no groups.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_GroupNames.htm)

#### `public int GroupObjectCount(int groupIndex)`

Returns the number of objects that are members of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Int32` — The nnumber of objects that are members of the group.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_GroupObjectCount.htm)

#### `public int Hide(int groupIndex)`

Hides all objects that are members of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Int32` — The number of objects that were hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Hide.htm)

#### `public bool IsDeleted(int groupIndex)`

Verifies a group is deleted.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Boolean` — true if the group is deleted, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_IsDeleted.htm)

#### `public int Lock(int groupIndex)`

Locks all objects that are members of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Int32` — The number of objects that were locked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Lock.htm)

#### `public int Show(int groupIndex)`

Shows, or unhides, all objects that are members of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Int32` — The number of objects that were shown.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Show.htm)

#### `public bool Undelete(int groupIndex)`

Undeletes a previously deleted group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Undelete.htm)

#### `public int Unlock(int groupIndex)`

Unlocks all objects that are members of a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group.

**Returns:** `Int32` — The number of objects that were unlocked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_GroupTable_Unlock.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `Document` (RhinoDoc, get) — Document that owns this table.

## GroupTableEventArgs (class)

Contains group table event data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_GroupTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — The document in which the event occurred.
- `EventType` (GroupTableEventType, get) — The event type.
- `GroupIndex` (Int32, get) — The index of the Group that has changed.
- `NewState` (Group, get) — The Group that has changed.
- `OldState` (Group, get) — If the event is GroupTableEventType.Modified, then the old Group.

## GroupTableEventType (enum)

Defines the types of group table events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_GroupTableEventType.htm)

### Values
- `Added` = `0` — A group was added.
- `Deleted` = `1` — A group was deleted.
- `Undeleted` = `2` — A group was undeleted.
- `Modified` = `3` — A group was modified.
- `Sorted` = `4` — The group table was sorted.

## HatchPatternTable (class)

All of the hatch pattern definitions contained in a rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_HatchPatternTable.htm)

### Methods
#### `public int Add(HatchPattern pattern)`

Adds a new hatch pattern with specified definition to the table.

**Parameters:**
- `pattern` (Rhino.DocObjects.HatchPattern) — definition of new hatch pattern. The information in pattern is copied. If patern.Name is empty the a unique name of the form "HatchPattern 01" will be automatically created.

**Returns:** `Int32` — >=0 index of new hatch pattern -1 not added because a hatch pattern with that name already exists or some other problem occurred.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public override bool Delete(HatchPattern item)`

Deletes a hatch pattern from the table.

**Parameters:**
- `item` (Rhino.DocObjects.HatchPattern) — The hatch pattern to delete.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Delete.htm)

#### `public bool Delete(HatchPattern item, bool quiet)`

Deletes a hatch pattern from the table.

**Parameters:**
- `item` (Rhino.DocObjects.HatchPattern) — The hatch pattern to delete.
- `quiet` (System.Boolean) — If true, no warning message box appears if hatch pattern cannot be deleted.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Delete_1.htm)

#### `public bool Delete(int hatchPatternIndex)`

Deletes a hatch pattern from the table.

**Parameters:**
- `hatchPatternIndex` (System.Int32) — The index of the hatch pattern to delete.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Delete_2.htm)

#### `public bool Delete(int hatchPatternIndex, bool quiet)`

Deletes a hatch pattern from the table.

**Parameters:**
- `hatchPatternIndex` (System.Int32) — The index of the hatch pattern to delete.
- `quiet` (System.Boolean) — If true, no warning message box appears if hatch pattern cannot be deleted.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Delete_3.htm)

#### `[ObsoleteAttribute("ignoreDeleted is now ignored. Items are removed permanently now. Use FindName.")] public int Find(string name, bool ignoreDeleted)`

Finds the hatch pattern with a given name. Search ignores case.

**Parameters:**
- `name` (System.String) — The name of the hatch patter to be found.
- `ignoreDeleted` (System.Boolean) — true means don't search deleted hatch patterns.

**Returns:** `Int32` — Index of the hatch pattern with the given name. -1 if no hatch pattern found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Find.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public HatchPattern FindIndex(int index)`

Retrieves a HatchPattern object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `HatchPattern` — A HatchPattern object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_FindIndex.htm)

#### `public HatchPattern FindName(string name)`

Finds the hatch pattern with a given name. Search ignores case.

**Parameters:**
- `name` (System.String) — The name of the hatch patter to be found.

**Returns:** `HatchPattern` — Hatch pattern with the given name. Null if no hatch pattern found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_FindName.htm)

#### `public HatchPattern FindNameHash(NameHash nameHash)`

Finds a HatchPattern given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the HatchPattern to be searched.

**Returns:** `HatchPattern` — An Linetype, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

#### `public bool Rename(HatchPattern item, string hatchPatternName)`

Renames a hatch pattern in the table.

**Parameters:**
- `item` (Rhino.DocObjects.HatchPattern) — The hatch pattern to rename
- `hatchPatternName` (System.String) — The new hatch pattern name.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Rename.htm)

#### `public bool Rename(int hatchPatternIndex, string hatchPatternName)`

Renames a hatch pattern in the table.

**Parameters:**
- `hatchPatternIndex` (System.Int32) — The index of the hatch pattern to rename.
- `hatchPatternName` (System.String) — The new hatch pattern name.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_HatchPatternTable_Rename_1.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `CurrentHatchPatternIndex` (Int32, get/set) — At all times, there is a "current" hatch pattern. Unless otherwise specified, new objects are assigned to the current hatch pattern. The current hatch pattern is never locked, hidden, or deleted.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (HatchPattern, get) — Conceptually, the hatch pattern table is an array of hatch patterns. The operator[] can be used to get individual hatch patterns. A hatch pattern is either active or deleted and this state is reported by HatchPattern.IsDeleted.

## InstanceDefinitionTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.InstanceDefinitionTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_InstanceDefinitionTable.htm)

### Methods
#### `public int Add(string name, string description, Point3d basePoint, GeometryBase geometry, ObjectAttributes attributes)`

Adds an instance definition to the instance definition table.

**Parameters:**
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.
- `basePoint` (Rhino.Geometry.Point3d) — A base point.
- `geometry` (Rhino.Geometry.GeometryBase) — An element.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — An attribute.

**Returns:** `Int32` — >=0 index of instance definition in the instance definition table. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Add.htm)

#### `public int Add(string name, string description, Point3d basePoint, IEnumerable<GeometryBase> geometry)`

Adds an instance definition to the instance definition table.

**Parameters:**
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.
- `basePoint` (Rhino.Geometry.Point3d) — A base point.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — An array, a list or any enumerable set of geometry.

**Returns:** `Int32` — >=0 index of instance definition in the instance definition table. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Add_1.htm)

#### `public int Add(string name, string description, Point3d basePoint, IEnumerable<GeometryBase> geometry, IEnumerable<ObjectAttributes> attributes)`

Adds an instance definition to the instance definition table.

**Parameters:**
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.
- `basePoint` (Rhino.Geometry.Point3d) — A base point.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — An array, a list or any enumerable set of geometry.
- `attributes` (System.Collections.Generic.IEnumerable<ObjectAttributes>) — An array, a list or any enumerable set of attributes.

**Returns:** `Int32` — >=0 index of instance definition in the instance definition table. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Add_2.htm)

#### `public int Add(string name, string description, string url, string urlTag, Point3d basePoint, IEnumerable<GeometryBase> geometry, IEnumerable<ObjectAttributes> attributes)`

Adds an instance definition to the instance definition table.

**Parameters:**
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.
- `url` (System.String) — A URL or hyperlink.
- `urlTag` (System.String) — A description of the URL or hyperlink.
- `basePoint` (Rhino.Geometry.Point3d) — A base point.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — An array, a list or any enumerable set of geometry.
- `attributes` (System.Collections.Generic.IEnumerable<ObjectAttributes>) — An array, a list or any enumerable set of attributes.

**Returns:** `Int32` — >=0 index of instance definition in the instance definition table. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Add_3.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public void Compact(bool ignoreUndoReferences)`

Purge deleted instance definition information that is not in use. This function is time consuming and should be used in a thoughtful manner.

**Parameters:**
- `ignoreUndoReferences` (System.Boolean) — If false, then deleted instance definition information that could possibly be undeleted by the Undo command will not be deleted. If true, then all deleted instance definition information is deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Compact.htm)

#### `public override bool Delete(InstanceDefinition item)`

Deletes the instance definition. This deletes all references too.

**Parameters:**
- `item` (Rhino.DocObjects.InstanceDefinition) — The item to delete.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Delete.htm)

#### `public bool Delete(int idefIndex, bool deleteReferences, bool quiet)`

Deletes the instance definition.

**Parameters:**
- `idefIndex` (System.Int32) — zero based index of instance definition to delete. This must be in the range 0 <= idefIndex < InstanceDefinitionTable.Count.
- `deleteReferences` (System.Boolean) — true to delete all references to this definition. false to delete definition only if there are no references.
- `quiet` (System.Boolean) — If true, no warning message box appears if an instance definition cannot be deleted because it is the current layer or it contains active geometry.

**Returns:** `Boolean` — true if successful. false if the instance definition has active references and bDeleteReferences is false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Delete_1.htm)

#### `public bool DestroySourceArchive(InstanceDefinition definition, bool quiet)`

Destroys all source archive information. Specifically: * SourceArchive is set to the empty string. * SourceRelativePath is set to false * The alternative source archive path is set to the empty string. * Checksum.Zero() is used to private destroy all checksum information. * UpdateType is set to Static.

**Parameters:**
- `definition` (Rhino.DocObjects.InstanceDefinition) — The instance definition to be modified.
- `quiet` (System.Boolean) — If true, then message boxes about erroneous parameters will not be shown.

**Returns:** `Boolean` — Returns true if the definition was successfully modified otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_DestroySourceArchive.htm)

#### `public InstanceDefinition Find(Guid instanceId, bool ignoreDeletedInstanceDefinitions)`

Finds the instance definition with a given id.

**Parameters:**
- `instanceId` (System.Guid) — Unique id of the instance definition to search for.
- `ignoreDeletedInstanceDefinitions` (System.Boolean) — true means don't search deleted instance definitions.

**Returns:** `InstanceDefinition` — The specified instance definition, or null if nothing matching was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Find.htm)

#### `public InstanceDefinition Find(string instanceDefinitionName)`

Finds the instance definition with a given name.

**Parameters:**
- `instanceDefinitionName` (System.String) — name of instance definition to search for (ignores case)

**Returns:** `InstanceDefinition` — The specified instance definition, or null if nothing matching was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Find_1.htm)

#### `[ObsoleteAttribute("ignoreDeletedInstanceDefinitions is now redundant. Remove the second argument. Definitions are now always deleted permanently.")] public InstanceDefinition Find(string instanceDefinitionName, bool ignoreDeletedInstanceDefinitions)`

Finds the instance definition with a given name.

**Parameters:**
- `instanceDefinitionName` (System.String) — name of instance definition to search for (ignores case)
- `ignoreDeletedInstanceDefinitions` (System.Boolean) — true means don't search deleted instance definitions.

**Returns:** `InstanceDefinition` — The specified instance definition, or null if nothing matching was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Find_2.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public override IEnumerator<InstanceDefinition> GetEnumerator()`

(Overrides CommonComponentTable<T>.GetEnumerator().)

**Returns:** `IEnumerator<InstanceDefinition>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_GetEnumerator.htm)

#### `public InstanceDefinition[] GetList(bool ignoreDeleted)`

Gets an array of instance definitions.

**Parameters:**
- `ignoreDeleted` (System.Boolean) — If true then deleted instance definitions are filtered out.

**Returns:** `InstanceDefinition[]` — An array of instance definitions. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_GetList.htm)

#### `public string GetUnusedInstanceDefinitionName()`

Gets unused instance definition name used as default when creating new instance definitions.

**Returns:** `String` — An unused instance definition name string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_GetUnusedInstanceDefinitionName.htm)

#### `public string GetUnusedInstanceDefinitionName(string root)`

Gets unused instance definition name used as default when creating new instance definitions.

**Parameters:**
- `root` (System.String) — The returned name is 'root nn' If root is empty, then 'Block' (localized) is used.

**Returns:** `String` — An unused instance definition name string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_GetUnusedInstanceDefinitionName_1.htm)

#### `[ObsoleteAttribute("The defaultSuffix parameter is now ignored. Remove the second argument.")] public string GetUnusedInstanceDefinitionName(string root, uint defaultSuffix)`

Gets unused instance definition name used as default when creating new instance definitions.

**Parameters:**
- `root` (System.String) — The returned name is 'root nn' If root is empty, then 'Block' (localized) is used.
- `defaultSuffix` (System.UInt32) — Unique names are created by appending a decimal number to the localized term for "Block" as in "Block 01", "Block 02", and so on. When defaultSuffix is supplied, the search for an unused name begins at "Block suffix".

**Returns:** `String` — An unused instance definition name string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_GetUnusedInstanceDefinitionName_2.htm)

#### `public int InstanceDefinitionIndex(Guid instanceId, bool ignoreDeletedInstanceDefinitions)`

Get the index of the instance definition with a given id.

**Parameters:**
- `instanceId` (System.Guid) — Unique id of the instance definition to search for
- `ignoreDeletedInstanceDefinitions` (System.Boolean) — true means don't search deleted instance definitions.

**Returns:** `Int32` — index > -1 if instance definition was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_InstanceDefinitionIndex.htm)

#### `[ObsoleteAttribute("Source paths are always absolute at runtime. They cannot be changed.")] public bool MakeSourcePathRelative(InstanceDefinition idef, bool relative, bool quiet)`

Obsolete method that always returns false. Marks the source path for a linked instance definition as relative or absolute.

**Parameters:**
- `idef` (Rhino.DocObjects.InstanceDefinition) — The instance definition to be marked.
- `relative` (System.Boolean) — If true, the path should be considered as relative.If false, the path should be considered as absolute.
- `quiet` (System.Boolean) — If true, then message boxes about erroneous parameters will not be shown.

**Returns:** `Boolean` — true if the instance definition could be modified.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_MakeSourcePathRelative.htm)

#### `public bool Modify(InstanceDefinition idef, string newName, string newDescription, bool quiet)`

Modifies the instance definition name and description. Does not change instance definition ID or geometry.

**Parameters:**
- `idef` (Rhino.DocObjects.InstanceDefinition) — The instance definition to be modified.
- `newName` (System.String) — The new name.
- `newDescription` (System.String) — The new description string.
- `quiet` (System.Boolean) — If true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Modify.htm)

#### `public bool Modify(InstanceDefinition idef, string newName, string newDescription, string newUrl, string newUrlTag, bool quiet)`

Modifies the instance definition name, description, and url. Does not change instance definition ID or geometry.

**Parameters:**
- `idef` (Rhino.DocObjects.InstanceDefinition) — The instance definition to be modified.
- `newName` (System.String) — The new name.
- `newDescription` (System.String) — The new description string.
- `newUrl` (System.String) — The new URL or hyperlink.
- `newUrlTag` (System.String) — The new description of the URL or hyperlink.
- `quiet` (System.Boolean) — If true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Modify_1.htm)

#### `public bool Modify(int idefIndex, string newName, string newDescription, bool quiet)`

Modifies the instance definition name and description. Does not change instance definition ID or geometry.

**Parameters:**
- `idefIndex` (System.Int32) — The index of the instance definition to be modified.
- `newName` (System.String) — The new name.
- `newDescription` (System.String) — The new description string.
- `quiet` (System.Boolean) — If true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Modify_2.htm)

#### `public bool Modify(int idefIndex, string newName, string newDescription, string newUrl, string newUrlTag, bool quiet)`

Modifies the instance definition name, description, and url. Does not change instance definition ID or geometry.

**Parameters:**
- `idefIndex` (System.Int32) — The index of the instance definition to be modified.
- `newName` (System.String) — The new name.
- `newDescription` (System.String) — The new description string.
- `newUrl` (System.String) — The new URL or hyperlink.
- `newUrlTag` (System.String) — The new description of the URL or hyperlink.
- `quiet` (System.Boolean) — If true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Modify_3.htm)

#### `public bool ModifyGeometry(int idefIndex, GeometryBase newGeometry, ObjectAttributes newAttributes)`



**Parameters:**
- `idefIndex` (System.Int32) — [Missing <param name="idefIndex"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,Rhino.Geometry.GeometryBase,Rhino.DocObjects.ObjectAttributes)"]
- `newGeometry` (Rhino.Geometry.GeometryBase) — [Missing <param name="newGeometry"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,Rhino.Geometry.GeometryBase,Rhino.DocObjects.ObjectAttributes)"]
- `newAttributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="newAttributes"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,Rhino.Geometry.GeometryBase,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,Rhino.Geometry.GeometryBase,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_ModifyGeometry.htm)

#### `public bool ModifyGeometry(int idefIndex, IEnumerable<GeometryBase> newGeometry)`



**Parameters:**
- `idefIndex` (System.Int32) — [Missing <param name="idefIndex"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase})"]
- `newGeometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — [Missing <param name="newGeometry"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifyGeometry(System.Int32,System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_ModifyGeometry_1.htm)

#### `public bool ModifyGeometry(int idefIndex, IEnumerable<GeometryBase> newGeometry, IEnumerable<ObjectAttributes> newAttributes)`

Modifies the instance definition geometry and replaces all references to the current definition with references to the new definition.

**Parameters:**
- `idefIndex` (System.Int32) — The index of the instance definition to be modified.
- `newGeometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — The new geometry.
- `newAttributes` (System.Collections.Generic.IEnumerable<ObjectAttributes>) — The new attributes.

**Returns:** `Boolean` — true if operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_ModifyGeometry_2.htm)

#### `public bool ModifySourceArchive(int idefIndex, FileReference sourceArchive, InstanceDefinitionUpdateType updateType, bool quiet)`

If the instance definition is linked or embedded, use SetSource to specify the source archive.

**Parameters:**
- `idefIndex` (System.Int32) — The index of the instance definition to be modified.
- `sourceArchive` (Rhino.FileIO.FileReference) — The new source archive file name.
- `updateType` (Rhino.DocObjects.InstanceDefinitionUpdateType) — [Missing <param name="updateType"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.ModifySourceArchive(System.Int32,Rhino.FileIO.FileReference,Rhino.DocObjects.InstanceDefinitionUpdateType,System.Boolean)"]
- `quiet` (System.Boolean) — If true, then message boxes about erroneous parameters will not be shown.

**Returns:** `Boolean` — Returns true if the definition was successfully modified otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_ModifySourceArchive.htm)

#### `public bool Purge(int idefIndex)`

Purges an instance definition and its definition geometry.

**Parameters:**
- `idefIndex` (System.Int32) — zero based index of instance definition to delete. This must be in the range 0 <= idefIndex < InstanceDefinitionTable.Count.

**Returns:** `Boolean` — True if successful. False if the instance definition cannot be purged because it is in use by reference objects or undo information.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Purge.htm)

#### `public bool RefreshLinkedBlock(InstanceDefinition definition)`

Reload linked block definitions and update the Rhino display.

**Parameters:**
- `definition` (Rhino.DocObjects.InstanceDefinition) — Instance definition to reload.

**Returns:** `Boolean` — Returns true if the linked file was successfully read and updated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_RefreshLinkedBlock.htm)

#### `public bool Undelete(int idefIndex)`

Undeletes an instance definition that has been deleted by Delete()

**Parameters:**
- `idefIndex` (System.Int32) — zero based index of instance definition to delete. This must be in the range 0 <= idefIndex < InstanceDefinitionTable.Count.

**Returns:** `Boolean` — true if successful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_Undelete.htm)

#### `public bool UndoModify(int idefIndex)`

Restores the instance definition to its previous state, if the instance definition has been modified and the modification can be undone.

**Parameters:**
- `idefIndex` (System.Int32) — The index of the instance definition to be restored.

**Returns:** `Boolean` — true if operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_UndoModify.htm)

#### `public bool UpdateLinkedInstanceDefinition(int idefIndex, string filename, bool updateNestedLinks, bool quiet)`

Read the objects from a file and use them as the instance's definition geometry.

**Parameters:**
- `idefIndex` (System.Int32) — zero based index of instance definition to delete. This must be in the range 0 <= idefIndex < InstanceDefinitionTable.Count.
- `filename` (System.String) — name of file (can be any type of file that Rhino or a plug-in can read)
- `updateNestedLinks` (System.Boolean) — If true and the instance definition references to a linked instance definition, that needs to be updated, then the nested definition is also updated. If false, nested updates are skipped.
- `quiet` (System.Boolean) — [Missing <param name="quiet"/> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.UpdateLinkedInstanceDefinition(System.Int32,System.String,System.Boolean,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.InstanceDefinitionTable.UpdateLinkedInstanceDefinition(System.Int32,System.String,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_InstanceDefinitionTable_UpdateLinkedInstanceDefinition.htm)

### Properties
- `ActiveCount` (Int32, get) — Number of items in the instance definitions table, excluding deleted definitions.
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Number of items in the instance definitions table.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (InstanceDefinition, get) — Conceptually, the InstanceDefinition table is an array of Instance definitions. The operator[] can be used to get individual instance definition. An instance definition is either active or deleted and this state is reported by IsDeleted or will be null if it has been purged from the document.

## InstanceDefinitionTableEventArgs (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.InstanceDefinitionTableEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_InstanceDefinitionTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `EventType` (InstanceDefinitionTableEventType, get) — 
- `InstanceDefinitionIndex` (Int32, get) — 
- `NewState` (InstanceDefinition, get) — 
- `OldState` (InstanceDefinitionGeometry, get) — 

## InstanceDefinitionTableEventType (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.InstanceDefinitionTableEventType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_InstanceDefinitionTableEventType.htm)

### Values
- `Added` = `0`
- `Deleted` = `1`
- `Undeleted` = `2`
- `Modified` = `3`
- `Sorted` = `4` — InstanceDefinitionTable.Sort() potentially changed sort order.

## LayerTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LayerTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LayerTable.htm)

### Methods
#### `public int Add()`

Adds a new layer with default definition to the layer table.

**Returns:** `Int32` — index of new layer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Add.htm)

#### `public int Add(Layer layer)`

Adds a new layer with specified definition to the layer table.

**Parameters:**
- `layer` (Rhino.DocObjects.Layer) — definition of new layer. The information in layer is copied. If layer.Name is empty the a unique name of the form "Layer 01" will be automatically created.

**Returns:** `Int32` — >=0 index of new layer -1 layer not added because a layer with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Add_1.htm)

#### `public int Add(string layerName, Color layerColor)`

Adds a new layer with specified definition to the layer table.

**Parameters:**
- `layerName` (System.String) — Name for new layer. Cannot be a null or zero-length string.
- `layerColor` (System.Drawing.Color) — Color of new layer. Alpha components will be ignored.

**Returns:** `Int32` — >=0 index of new layer -1 layer not added because a layer with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Add_2.htm)

#### `public int AddReferenceLayer()`

Adds a new reference layer with default definition to the layer table. Reference layers are not saved in files.

**Returns:** `Int32` — index of new layer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_AddReferenceLayer.htm)

#### `public int AddReferenceLayer(Layer layer)`

Adds a new reference layer with specified definition to the layer table Reference layers are not saved in files.

**Parameters:**
- `layer` (Rhino.DocObjects.Layer) — definition of new layer. The information in layer is copied. If layer.Name is empty the a unique name of the form "Layer 01" will be automatically created.

**Returns:** `Int32` — >=0 index of new layer -1 layer not added because a layer with that name already exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_AddReferenceLayer_1.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public bool Delete(Guid layerId, bool quiet)`

Deletes layer.

**Parameters:**
- `layerId` (System.Guid) — Id of the layer to be deleted.
- `quiet` (System.Boolean) — If true, no warning message box appears if a layer the layer cannot be deleted because it is the current layer or it contains active geometry.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the layer cannot be deleted because it is the current layer or because it layer contains active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Delete_2.htm)

#### `public bool Delete(int layerIndex, bool quiet)`

Deletes layer.

**Parameters:**
- `layerIndex` (System.Int32) — zero based index of layer to delete. This must be in the range 0 <= layerIndex < LayerTable.Count.
- `quiet` (System.Boolean) — If true, no warning message box appears if a layer the layer cannot be deleted because it is the current layer or it contains active geometry.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the layer cannot be deleted because it is the current layer or because it layer contains active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Delete_3.htm)

#### `public override bool Delete(Layer layer)`

Deletes layer.

**Parameters:**
- `layer` (Rhino.DocObjects.Layer) — Layer to be deleted.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the layer cannot be deleted because it is the current layer or because it layer contains active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Delete.htm)

#### `public bool Delete(Layer layer, bool quiet)`

Deletes layer.

**Parameters:**
- `layer` (Rhino.DocObjects.Layer) — Layer to be deleted.
- `quiet` (System.Boolean) — If true, no warning message box appears if a layer the layer cannot be deleted because it is the current layer or it contains active geometry.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the layer cannot be deleted because it is the current layer or because it layer contains active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Delete_1.htm)

#### `public int[] Duplicate(IEnumerable<int> layerIndices, bool duplicateObjects, bool duplicateSublayers)`

Duplicates, or copies, one or more layers. Duplicated layers are added to the document.

**Parameters:**
- `layerIndices` (System.Collections.Generic.IEnumerable<Int32>) — The indices of layers to duplicate.
- `duplicateObjects` (System.Boolean) — If true, then layer objects will also be duplicated and added to the document.
- `duplicateSublayers` (System.Boolean) — If true, then all sub-layers of the layer will be duplicated.

**Returns:** `Int32[]` — The indices of the newly added layers if successful, an empty array on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Duplicate.htm)

#### `public int[] Duplicate(int layerIndex, bool duplicateObjects, bool duplicateSublayers)`

Duplicates, or copies, a layer. Duplicated layers are added to the document.

**Parameters:**
- `layerIndex` (System.Int32) — The index of the layer to duplicate.
- `duplicateObjects` (System.Boolean) — If true, then layer objects will also be duplicated and added to the document.
- `duplicateSublayers` (System.Boolean) — If true, then all sub-layers of the layer will be duplicated.

**Returns:** `Int32[]` — The indices of the newly added layers if successful, an empty array on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Duplicate_1.htm)

#### `public int FindByFullPath(string layerPath, int notFoundReturnValue)`

Searches for a layer using the fully qualified name, that includes ancestors. Deleted layers have no name.

**Parameters:**
- `layerPath` (System.String) — The full layer name.
- `notFoundReturnValue` (System.Int32) — Should be -1 to get the index of the OpenNURBS default layer, or UnsetIntIndex to get an always-out-of-bound value.

**Returns:** `Int32` — The index of the found layer, or notFoundReturnValue.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_FindByFullPath.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public Layer FindIndex(int index)`

Retrieves a Layer object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `Layer` — A Layer object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_FindIndex.htm)

#### `public Layer FindName(string layerName)`

Finds the layer with a given name. If multiple layers exist that have the same name, the first match layer index will be returned. Deleted layers have no name.The default layer is NOT included in the search. If required, use the overload with startIndex input.

**Parameters:**
- `layerName` (System.String) — name of layer to search for. The search ignores case.

**Returns:** `Layer` — A layer, or null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_FindName.htm)

#### `public Layer FindName(string layerName, int startIndex)`

Finds the next layer that has an index equal or higher than the searched value. Search in case-insensitive.

**Parameters:**
- `layerName` (System.String) — The layer to search for.
- `startIndex` (System.Int32) — If you specify RhinoMath.UnsetIntIndex, then also default layers will be included. This is the first index that will be tested.

**Returns:** `Layer` — A layer, or null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_FindName_1.htm)

#### `public Layer FindNameHash(NameHash nameHash)`

Finds a Layer given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the Layer to be searched.

**Returns:** `Layer` — An Layer, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_FindNameHash.htm)

#### `public bool ForceLayerVisible(Guid layerId)`

Makes a layer and all of its parent layers visible.

**Parameters:**
- `layerId` (System.Guid) — The layer ID to be made visible.

**Returns:** `Boolean` — true if the operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_ForceLayerVisible.htm)

#### `public bool ForceLayerVisible(int layerIndex)`

Makes a layer and all of its parent layers visible.

**Parameters:**
- `layerIndex` (System.Int32) — The layer index to be made visible.

**Returns:** `Boolean` — true if the operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_ForceLayerVisible_1.htm)

#### `public override IEnumerator<Layer> GetEnumerator()`

(Overrides CommonComponentTable<T>.GetEnumerator().)

**Returns:** `IEnumerator<Layer>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LayerTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_GetEnumerator.htm)

#### `public bool GetSelected(out List<int> layerIndices)`

Returns the indices of layers that are selected on the Layer user interface.

**Parameters:**
- `layerIndices` (System.Collections.Generic.List<Int32>) — The indices of selected layers.

**Returns:** `Boolean` — true if the layer user interface is visible, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_GetSelected.htm)

#### `public string GetUnusedLayerName()`

Gets the next unused layer name used as default when creating new layers.

**Returns:** `String` — An unused layer name string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_GetUnusedLayerName.htm)

#### `[ObsoleteAttribute("'ignoreDeleted' is now redundant. Layers are now permanently removed. Use the overload with this argument.")] public string GetUnusedLayerName(bool ignoreDeleted)`

Gets the next unused layer name used as default when creating new layers.

**Parameters:**
- `ignoreDeleted` (System.Boolean) — If this is true then Rhino may use a name used by a deleted layer.

**Returns:** `String` — An unused layer name string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_GetUnusedLayerName_1.htm)

#### `public bool Modify(Layer newSettings, Guid layerId, bool quiet)`

Modifies layer settings.

**Parameters:**
- `newSettings` (Rhino.DocObjects.Layer) — This information is copied.
- `layerId` (System.Guid) — Id of layer.
- `quiet` (System.Boolean) — if false, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the settings attempt to lock or hide the current layer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Modify.htm)

#### `public bool Modify(Layer newSettings, int layerIndex, bool quiet)`

Modifies layer settings.

**Parameters:**
- `newSettings` (Rhino.DocObjects.Layer) — This information is copied.
- `layerIndex` (System.Int32) — zero based index of layer to set. This must be in the range 0 <= layerIndex < LayerTable.Count.
- `quiet` (System.Boolean) — if false, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the settings attempt to lock or hide the current layer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Modify_1.htm)

#### `public bool Purge(Guid layerId, bool quiet)`

Deletes a layer and all geometry objects on a layer.

**Parameters:**
- `layerId` (System.Guid) — Id of the layer to purge.
- `quiet` (System.Boolean) — If true, no warning message box appears if a layer the layer cannot be deleted because it is the current layer.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the layer cannot be deleted because it is the current layer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Purge.htm)

#### `public bool Purge(int layerIndex, bool quiet)`

Deletes a layer and all geometry objects on a layer

**Parameters:**
- `layerIndex` (System.Int32) — zero based index of layer to delete. This must be in the range 0 <= layerIndex < LayerTable.Count.
- `quiet` (System.Boolean) — If true, no warning message box appears if a layer the layer cannot be deleted because it is the current layer.

**Returns:** `Boolean` — true if successful. false if layerIndex is out of range or the layer cannot be deleted because it is the current layer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Purge_1.htm)

#### `public bool Select(IEnumerable<int> layerIndices, bool bDeselect)`

Selects layers in the Layer user interface.

**Parameters:**
- `layerIndices` (System.Collections.Generic.IEnumerable<Int32>) — The indices of layers to select.
- `bDeselect` (System.Boolean) — If true, then any previously selected layers will be unselected.

**Returns:** `Boolean` — true if the layer user interface is visible, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Select.htm)

#### `public bool SetCurrentLayerIndex(int layerIndex, bool quiet)`

At all times, there is a "current" layer. Unless otherwise specified, new objects are assigned to the current layer. The current layer is never locked, hidden, or deleted.

**Parameters:**
- `layerIndex` (System.Int32) — Value for new current layer. 0 <= layerIndex < LayerTable.Count. The layer's mode is automatically set to NormalMode.
- `quiet` (System.Boolean) — if true, then no warning message box pops up if the current layer request can't be satisfied.

**Returns:** `Boolean` — true if current layer index successfully set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_SetCurrentLayerIndex.htm)

#### `public bool Undelete(int layerIndex)`

Undeletes a layer that has been deleted by DeleteLayer().

**Parameters:**
- `layerIndex` (System.Int32) — zero based index of layer to undelete. This must be in the range 0 <= layerIndex < LayerTable.Count.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_Undelete.htm)

#### `public bool UndoModify(Guid layerId)`

Restores the layer to its previous state, if the layer has been modified and the modification can be undone.

**Parameters:**
- `layerId` (System.Guid) — The layer Id to be used.

**Returns:** `Boolean` — true if this layer had been modified and the modifications were undone.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_UndoModify.htm)

#### `public bool UndoModify(Guid layerId, uint undoRecordSerialNumber)`

Restores the layer to its previous state, if the layer has been modified and the modification can be undone.

**Parameters:**
- `layerId` (System.Guid) — The layer Id to be used.
- `undoRecordSerialNumber` (System.UInt32) — The undo record serial number. Pass 0 not to specify one.

**Returns:** `Boolean` — true if this layer had been modified and the modifications were undone.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_UndoModify_1.htm)

#### `public bool UndoModify(int layerIndex)`

Restores the layer to its previous state, if the layer has been modified and the modification can be undone.

**Parameters:**
- `layerIndex` (System.Int32) — The layer index to be used.

**Returns:** `Boolean` — true if this layer had been modified and the modifications were undone.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_UndoModify_2.htm)

#### `public bool UndoModify(int layerIndex, uint undoRecordSerialNumber)`

Restores the layer to its previous state, if the layer has been modified and the modification can be undone.

**Parameters:**
- `layerIndex` (System.Int32) — The layer index to be used.
- `undoRecordSerialNumber` (System.UInt32) — The undo record serial number. Pass 0 not to specify one.

**Returns:** `Boolean` — true if this layer had been modified and the modifications were undone.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LayerTable_UndoModify_3.htm)

### Properties
- `ActiveCount` (Int32, get) — Returns number of layers in the layer table, excluding deleted layers.
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Returns number of layers in the layer table, including deleted layers.
- `CurrentLayer` (Layer, get) — At all times, there is a "current" layer. Unless otherwise specified, new objects are assigned to the current layer. The current layer is never locked, hidden, or deleted. Returns reference to the current layer. Note that this reference may become invalid after a call to AddLayer().
- `CurrentLayerIndex` (Int32, get) — At all times, there is a "current" layer. Unless otherwise specified, new objects are assigned to the current layer. The current layer is never locked, hidden, or deleted. Returns: Zero based layer table index of the current layer.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (Layer, get) — Conceptually, the layer table is an array of layers. The operator[] can be used to get individual layers. A layer is either active or deleted and this state is reported by Layer.IsDeleted.

## LayerTableEventArgs (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LayerTableEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LayerTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `EventType` (LayerTableEventType, get) — 
- `LayerIndex` (Int32, get) — 
- `NewState` (Layer, get) — 
- `OldState` (Layer, get) — 

## LayerTableEventType (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LayerTableEventType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LayerTableEventType.htm)

### Values
- `Added` = `0`
- `Deleted` = `1`
- `Undeleted` = `2`
- `Modified` = `3`
- `Sorted` = `4` — LayerTable.Sort() potentially changed sort order.
- `Current` = `5` — Current layer change.

## LightTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LightTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LightTable.htm)

### Methods
#### `public int Add(Light light)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Add(Rhino.Geometry.Light)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Add(Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Add.htm)

#### `public int Add(Light light, ObjectAttributes attributes)`



**Parameters:**
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Add(Rhino.Geometry.Light,Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Add(Rhino.Geometry.Light,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Add(Rhino.Geometry.Light,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Add_1.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public bool Delete(int index, bool quiet)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Delete(System.Int32,System.Boolean)"]
- `quiet` (System.Boolean) — [Missing <param name="quiet"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Delete(System.Int32,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Delete(System.Int32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Delete_1.htm)

#### `public override bool Delete(LightObject item)`

(Overrides CommonComponentTable<T>.Delete(T).)

**Parameters:**
- `item` (Rhino.DocObjects.LightObject) — [Missing <param name="item"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Delete(Rhino.DocObjects.LightObject)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Delete(Rhino.DocObjects.LightObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Delete.htm)

#### `public int Find(Guid id, bool ignoreDeleted)`



**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Find(System.Guid,System.Boolean)"]
- `ignoreDeleted` (System.Boolean) — [Missing <param name="ignoreDeleted"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Find(System.Guid,System.Boolean)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Find(System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Find.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public LightObject FindIndex(int index)`

Retrieves a object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `LightObject` — A object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_FindIndex.htm)

#### `public LightObject FindName(string name)`

Finds the LightObject with a given name. Deleted lights have no name.

**Parameters:**
- `name` (System.String) — Name to search.

**Returns:** `LightObject` — A layer. If no layer is found, null is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_FindName.htm)

#### `public LightObject FindNameHash(NameHash nameHash)`

Finds a LightObject given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the LightObject to be searched.

**Returns:** `LightObject` — A LightObject, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_FindNameHash.htm)

#### `public override IEnumerator<LightObject> GetEnumerator()`

(Overrides CommonComponentTable<T>.GetEnumerator().)

**Returns:** `IEnumerator<LightObject>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_GetEnumerator.htm)

#### `public bool Modify(Guid id, Light light)`



**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Modify(System.Guid,Rhino.Geometry.Light)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Modify(System.Guid,Rhino.Geometry.Light)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Modify(System.Guid,Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Modify.htm)

#### `public bool Modify(int index, Light light)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Modify(System.Int32,Rhino.Geometry.Light)"]
- `light` (Rhino.Geometry.Light) — [Missing <param name="light"/> documentation for "M:Rhino.DocObjects.Tables.LightTable.Modify(System.Int32,Rhino.Geometry.Light)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LightTable.Modify(System.Int32,Rhino.Geometry.Light)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LightTable_Modify_1.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Number of lights in the light table. Does not include Sun or Skylight.
- `Document` (RhinoDoc, get) — Document that owns this light table.
- `Item` (LightObject, get) — 
- `Skylight` (Skylight, get) — 
- `Sun` (Sun, get) — Gets the Sun instance that is applied to the document. If the RDK is loaded, an instance is always returned.

## LightTableEventArgs (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LightTableEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LightTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `EventType` (LightTableEventType, get) — 
- `LightIndex` (Int32, get) — 
- `NewState` (LightObject, get) — 
- `OldState` (Light, get) — 

## LightTableEventType (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LightTableEventType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LightTableEventType.htm)

### Values
- `Added` = `0`
- `Deleted` = `1`
- `Undeleted` = `2`
- `Modified` = `3`
- `Sorted` = `4` — LightTable.Sort() potentially changed sort order.

## LinetypeTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.LinetypeTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_LinetypeTable.htm)

### Methods
#### `public int Add(Linetype linetype)`

Adds a new linetype with specified definition to the linetype table.

**Parameters:**
- `linetype` (Rhino.DocObjects.Linetype) — Definition of new linetype. The information in linetype is copied. If linetype.Name is empty then a unique name of the form "Linetype 01" will be automatically created.

**Returns:** `Int32` — Index of newline type or -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Add.htm)

#### `public int Add(string name, IEnumerable<double> segmentLengths)`

Adds a new linetype with specified definition to the linetype table.

**Parameters:**
- `name` (System.String) — A name for the new linetype.
- `segmentLengths` (System.Collections.Generic.IEnumerable<Double>) — Positive values are dashes, negative values are gaps.

**Returns:** `Int32` — Index of new linetype or -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Add_1.htm)

#### `public int AddReferenceLinetype(Linetype linetype)`

Adds a reference linetypes that will not be saved in files.

**Parameters:**
- `linetype` (Rhino.DocObjects.Linetype) — Definition of new linetype. The information in linetype is copied. If linetype.Name is empty then a unique name of the form "Linetype 01" will be automatically created.

**Returns:** `Int32` — Index of new linetype or -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_AddReferenceLinetype.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public bool Delete(IEnumerable<int> indices, bool quiet)`

Deletes multiple linetypes.

**Parameters:**
- `indices` (System.Collections.Generic.IEnumerable<Int32>) — An array, a list or any enumerable instance of linetype indices.
- `quiet` (System.Boolean) — If true, no warning message box appears if a linetype the linetype cannot be deleted because it is the current linetype or it contains active geometry.

**Returns:** `Boolean` — true if operation succeeded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Delete_1.htm)

#### `public bool Delete(int index, bool quiet)`

Deletes linetype.

**Parameters:**
- `index` (System.Int32) — zero based index of linetype to delete.
- `quiet` (System.Boolean) — If true, no warning message box appears if a linetype the linetype cannot be deleted because it is the current linetype or it contains active geometry.

**Returns:** `Boolean` — true if successful. false if linetypeIndex is out of range or the linetype cannot be deleted because it is the current linetype or because it linetype is referenced by active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Delete_2.htm)

#### `public override bool Delete(Linetype item)`

(Overrides CommonComponentTable<T>.Delete(T).)

**Parameters:**
- `item` (Rhino.DocObjects.Linetype) — [Missing <param name="item"/> documentation for "M:Rhino.DocObjects.Tables.LinetypeTable.Delete(Rhino.DocObjects.Linetype)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LinetypeTable.Delete(Rhino.DocObjects.Linetype)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Delete.htm)

#### `public int Find(Guid id, bool ignoreDeletedLinetypes)`

Finds a linetype with a matching ID.

**Parameters:**
- `id` (System.Guid) — The ID of the line type to be found.
- `ignoreDeletedLinetypes` (System.Boolean) — If true, deleted linetypes are not checked.

**Returns:** `Int32` — If the linetype was found, the linetype index, >=0, is returned. If the linetype was not found, -1 is returned. Note, the linetype index of -1 denotes the default, or "Continuous" linetype.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Find.htm)

#### `public int Find(string name)`

Finds the linetype with a given name.

**Parameters:**
- `name` (System.String) — The name of the linetype to find. The search ignores case.

**Returns:** `Int32` — If the linetype was found, the linetype index, >=0, is returned. If the linetype was not found, -1 is returned. Note, the linetype index of -1 denotes the default, or "Continuous" linetype.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Find_1.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public Linetype FindIndex(int index)`

Retrieves a Linetype object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `Linetype` — A Linetype object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_FindIndex.htm)

#### `public Linetype FindName(string name)`

Finds the linetype with a given name.

**Parameters:**
- `name` (System.String) — he name of the linetype to find.

**Returns:** `Linetype` — The linetype. If the linetype was not found, then the default, or or "Continuous" linetype is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_FindName.htm)

#### `public override IEnumerator<Linetype> GetEnumerator()`

(Overrides CommonComponentTable<T>.GetEnumerator().)

**Returns:** `IEnumerator<Linetype>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.LinetypeTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_GetEnumerator.htm)

#### `public string GetUnusedLinetypeName()`

Gets unused linetype name used as default when creating new linetypes.

**Returns:** `String` — The unused linetype name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_GetUnusedLinetypeName.htm)

#### `[ObsoleteAttribute("ignoreDeleted is now ignored. Items are removed permanently now. Remove the second method argument.")] public string GetUnusedLinetypeName(bool ignoreDeleted)`

Obsolete. Use the other overload. Gets unused linetype name used as default when creating new linetypes.

**Parameters:**
- `ignoreDeleted` (System.Boolean) — If this is true then a name used by a deleted linetype is allowed.

**Returns:** `String` — The unused linetype name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_GetUnusedLinetypeName_1.htm)

#### `public int LinetypeIndexForObject(RhinoObject rhinoObject)`

Returns the effective linetype index to be used to find the linetype definition to draw an object. If an object's linetype source is LinetypeFromObject, the linetype index in the object's attributes is used. If an object's linetype source is LinetypeFromLayer the linetype index from the object's layer is used.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — The Rhino object to use in the query.

**Returns:** `Int32` — The effective linetype index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_LinetypeIndexForObject.htm)

#### `public int LoadDefaultLinetypes()`

Fills in the linetype table with any default linetypes not already included.

**Remarks:** New documents only contain the continuous linetype. Other default linetypes are added, on demand, when the user needs them. Calling this function ensures that the linetype table is populated with the default linetypes.

**Returns:** `Int32` — The number of default linetypes added to the linetype table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_LoadDefaultLinetypes.htm)

#### `public bool Modify(Linetype linetype, int index, bool quiet)`

Modify linetype settings.

**Parameters:**
- `linetype` (Rhino.DocObjects.Linetype) — New linetype settings. This information is copied.
- `index` (System.Int32) — Zero based index of linetype to set.
- `quiet` (System.Boolean) — if true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful. false if linetype_index is out of range or the settings attempt to lock or hide the current linetype.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Modify.htm)

#### `public bool SetCurrentLinetypeIndex(int linetypeIndex, bool quiet)`

At all times, there is a "current" linetype. Unless otherwise specified, new objects are assigned to the current linetype. The current linetype is never deleted.

**Parameters:**
- `linetypeIndex` (System.Int32) — Value for new current linetype. 0 <= linetypeIndex < LinetypeTable.Count.
- `quiet` (System.Boolean) — if true, then no warning message box pops up if the current linetype request can't be satisfied.

**Returns:** `Boolean` — true if current linetype index successfully set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_SetCurrentLinetypeIndex.htm)

#### `public bool Undelete(int index)`

Restores a linetype that has been deleted.

**Parameters:**
- `index` (System.Int32) — A linetype index to be undeleted.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_Undelete.htm)

#### `public bool UndoModify(int index)`

If the linetype has been modified and the modification can be undone, then UndoModify() will restore the linetype to its previous state.

**Parameters:**
- `index` (System.Int32) — Zero based index of linetype for which to undo changes.

**Returns:** `Boolean` — true if this linetype had been modified and the modifications were undone.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_LinetypeTable_UndoModify.htm)

### Properties
- `ActiveCount` (Int32, get) — Returns number of linetypes in the linetypes table, excluding deleted linetypes.
- `ByLayerLinetypeName` (String, get) — Returns the text name of the by-layer linetype.
- `ByParentLinetypeName` (String, get) — Returns the text name of the by-parent linetype.
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `ContinuousLinetypeName` (String, get) — Returns the text name of the continuous linetype.
- `Count` (Int32, get) — Returns number of linetypes in the linetypes table, including deleted linetypes.
- `CurrentLinetype` (Linetype, get) — Returns reference to the current linetype. Note that this reference may become invalid after a call to AddLinetype().
- `CurrentLinetypeIndex` (Int32, get) — At all times, there is a "current" linetype. Unless otherwise specified, new objects are assigned to the current linetype. If the current linetype source is LinetypeFromLayer the object's layer's linetype is used instead.
- `CurrentLinetypeSource` (ObjectLinetypeSource, get/set) — Source used by an object to determine its current linetype to be used by new objects.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (Linetype, get) — Conceptually, the linetype table is an array of linetypes. The operator[] can be used to get individual linetypes. A linetype is either active or deleted and this state is reported by Linetype.IsDeleted.
- `LinetypeScale` (Double, get/set) — For display in Rhino viewports, the linetypes are scaled by a single scale factor for all viewports. This is not used for printing, where all linetype patterns are scaled to print in their defined size 1:1 on the paper.

## MaterialTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.MaterialTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_MaterialTable.htm)

### Methods
#### `public int Add()`

Adds a new material to the table based on the default material.

**Returns:** `Int32` — The position of the new material in the table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Add.htm)

#### `public int Add(Material material)`

Adds a new material to the table based on a given material.

**Parameters:**
- `material` (Rhino.DocObjects.Material) — A model of the material to be added.

**Returns:** `Int32` — The position of the new material in the table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Add_1.htm)

#### `public int Add(Material material, bool reference)`

Adds a new material to the table based on a given material.

**Parameters:**
- `material` (Rhino.DocObjects.Material) — A model of the material to be added.
- `reference` (System.Boolean) — true if this material is supposed to be a reference material. Reference materials are not saved in the file.

**Returns:** `Int32` — The position of the new material in the table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Add_2.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public override bool Delete(Material item)`

(Overrides CommonComponentTable<T>.Delete(T).)

**Parameters:**
- `item` (Rhino.DocObjects.Material) — [Missing <param name="item"/> documentation for "M:Rhino.DocObjects.Tables.MaterialTable.Delete(Rhino.DocObjects.Material)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.MaterialTable.Delete(Rhino.DocObjects.Material)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Delete.htm)

#### `public bool DeleteAt(int materialIndex)`

Removes a material at a specific position from this material table.

**Parameters:**
- `materialIndex` (System.Int32) — The position to be removed.

**Returns:** `Boolean` — true if successful. false if materialIndex is out of range or the material cannot be deleted because it is the current material or because it material contains active geometry.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_DeleteAt.htm)

#### `public int Find(Guid materialId, bool ignoreDeletedMaterials)`

Finds a material with a matching id.

**Parameters:**
- `materialId` (System.Guid) — A material ID to be found.
- `ignoreDeletedMaterials` (System.Boolean) — If true, deleted materials are not checked.

**Returns:** `Int32` — >=0 index of the material with the given name -1 no material has the given name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Find.htm)

#### `public int Find(string materialName, bool ignoreDeletedMaterials)`

Finds a material with a given name.

**Parameters:**
- `materialName` (System.String) — Name of the material to search for. The search ignores case.
- `ignoreDeletedMaterials` (System.Boolean) — true means don't search deleted materials.

**Returns:** `Int32` — >=0 index of the material with the given name -1 no material has the given name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Find_1.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public Material FindIndex(int index)`

Retrieves a Material object based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `Material` — A Material object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_FindIndex.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

#### `public bool Modify(Material newSettings, int materialIndex, bool quiet)`

Modify material settings.

**Parameters:**
- `newSettings` (Rhino.DocObjects.Material) — This information is copied.
- `materialIndex` (System.Int32) — zero based index of material to set. This must be in the range 0 <= layerIndex < MaterialTable.Count.
- `quiet` (System.Boolean) — if true, information message boxes pop up when illegal changes are attempted.

**Returns:** `Boolean` — true if successful. false if materialIndex is out of range or the settings attempt to lock or hide the current material.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_Modify.htm)

#### `public bool ResetMaterial(int materialIndex)`



**Parameters:**
- `materialIndex` (System.Int32) — [Missing <param name="materialIndex"/> documentation for "M:Rhino.DocObjects.Tables.MaterialTable.ResetMaterial(System.Int32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.MaterialTable.ResetMaterial(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_MaterialTable_ResetMaterial.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `CurrentMaterialIndex` (Int32, get/set) — At all times, there is a "current" material. Unless otherwise specified, new objects are assigned to the current material. The current material is never locked, hidden, or deleted.
- `CurrentMaterialSource` (ObjectMaterialSource, get/set) — Gets or sets the current material source.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (Material, get) — Conceptually, the material table is an array of materials. The operator[] can be used to get individual materials. A material is either active or deleted and this state is reported by Material.IsDeleted.

## MaterialTableEventArgs (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.MaterialTableEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_MaterialTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `EventType` (MaterialTableEventType, get) — 
- `Index` (Int32, get) — 
- `OldSettings` (Material, get) — 

## MaterialTableEventType (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.MaterialTableEventType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_MaterialTableEventType.htm)

### Values
- `Added` = `0`
- `Deleted` = `1`
- `Undeleted` = `2`
- `Modified` = `3`
- `Sorted` = `4`
- `Current` = `5`

## ModifyType (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.ModifyType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_ModifyType.htm)

### Values
- `Modify` = `0`
- `Override` = `1`
- `NotSaved` = `2`

## NamedConstructionPlaneTable (class)

Contains all named construction planes in a rhino document. This class cannot be inherited.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_NamedConstructionPlaneTable.htm)

### Methods
#### `public int Add(string name, Plane plane)`

Adds named construction plane to document.

**Parameters:**
- `name` (System.String) — If name is empty, a unique name is automatically created. If there is already a named construction plane with the same name, that construction plane is replaced.
- `plane` (Rhino.Geometry.Plane) — The plane value.

**Returns:** `Int32` — 0 based index of named construction plane. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedConstructionPlaneTable_Add.htm)

#### `public bool Delete(int index)`

Remove named construction plane from the document.

**Parameters:**
- `index` (System.Int32) — zero based array index.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedConstructionPlaneTable_Delete.htm)

#### `public bool Delete(string name)`

Remove named construction plane from the document.

**Parameters:**
- `name` (System.String) — name of the construction plane.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedConstructionPlaneTable_Delete_1.htm)

#### `public int Find(string name)`

Finds a named construction plane.

**Parameters:**
- `name` (System.String) — Name of construction plane to search for.

**Returns:** `Int32` — >=0 index of the construction plane with the given name. -1 no construction plane found with the given name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedConstructionPlaneTable_Find.htm)

#### `public IEnumerator<ConstructionPlane> GetEnumerator()`



**Returns:** `IEnumerator<ConstructionPlane>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedConstructionPlaneTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedConstructionPlaneTable_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Number of construction planes in the table.
- `Document` (RhinoDoc, get) — Gets the document that owns this table.
- `Item` (ConstructionPlane, get) — Conceptually, the named construction plane table is an array of ConstructionPlanes and their associated names. The operator[] can be used to get individual ConstructionPlanes.

## NamedLayerStateTable (class)

All named layer states in a Rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_NamedLayerStateTable.htm)

### Methods
#### `public bool Delete(string name)`

Deletes an existing named layer state.

**Parameters:**
- `name` (System.String) — The name of the layer state.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Delete.htm)

#### `public int FindName(string name)`

Returns the index of an existing named layer state.

**Parameters:**
- `name` (System.String) — The name of the layer state.

**Returns:** `Int32` — >0 if successful, -1 if not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_FindName.htm)

#### `public int Import(string filename)`

Imports named layer states from a 3dm file.

**Parameters:**
- `filename` (System.String) — The name of the file to import.

**Returns:** `Int32` — The number of named layers states imported.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Import.htm)

#### `public bool Rename(string oldName, string newName)`

Renames an existing named layer state.

**Parameters:**
- `oldName` (System.String) — The name of the layer state.
- `newName` (System.String) — The new name

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Rename.htm)

#### `public bool Restore(string name, RestoreLayerProperties properties)`

Restores a named layer state.

**Parameters:**
- `name` (System.String) — The name of the layer state.
- `properties` (Rhino.DocObjects.Tables.RestoreLayerProperties) — The layer properties to restore.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Restore.htm)

#### `public bool Restore(string name, RestoreLayerProperties properties, Guid viewportId)`

Restores a named layer state.

**Parameters:**
- `name` (System.String) — The name of the layer state.
- `properties` (Rhino.DocObjects.Tables.RestoreLayerProperties) — The layer properties to restore.
- `viewportId` (System.Guid) — The id of the layout or detail viewport to restore the per-viewport layer properties.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedLayerStateTable.Restore(System.String,Rhino.DocObjects.Tables.RestoreLayerProperties,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Restore_1.htm)

#### `public int Save(string name)`

Saves or updates a named layer state.

**Parameters:**
- `name` (System.String) — The name of the layer state. If the named layer state already exists, it will be updated.

**Returns:** `Int32` — The index of the newly added, or updated, layer state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Save.htm)

#### `public int Save(string name, Guid viewportId)`

Saves or updates a named layer state.

**Parameters:**
- `name` (System.String) — The name of the layer state. If the named layer state already exists, it will be updated.
- `viewportId` (System.Guid) — The id of the layout or detail viewport, required to save per viewport layer state properties.

**Returns:** `Int32` — The index of the newly added, or updated, layer state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedLayerStateTable_Save_1.htm)

### Properties
- `Count` (Int32, get) — Returns the number of named layers states in the document.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Names` (String[], get) — Returns the names of named layer states in the document.

## NamedPositionTable (class)

All named positions in a rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_NamedPositionTable.htm)

### Methods
#### `public bool Append(Guid id, IEnumerable<Guid> objectIds)`

Append objects to a Named Position.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position which you want to append to.
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — New object ids to be included in this Named Position.

**Returns:** `Boolean` — True or False depending on whether the Append was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Append_1.htm)

#### `public bool Append(Guid id, IEnumerable<RhinoObject> objects)`

Append objects to a Named Position.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position which you want to append to.
- `objects` (System.Collections.Generic.IEnumerable<RhinoObject>) — Collection of Rhino Objects to be included in this Named Position.

**Returns:** `Boolean` — True or False depending on whether the Append was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Append.htm)

#### `public bool Append(string name, IEnumerable<Guid> objectIds)`

Append objects to a Named Position.

**Parameters:**
- `name` (System.String) — Name of the Named Position which you want to append to.
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — New object Guids to be included in this Named Position.

**Returns:** `Boolean` — True or False depending on whether the Append was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Append_3.htm)

#### `public bool Append(string name, IEnumerable<RhinoObject> objects)`

Append objects to a Named Position.

**Parameters:**
- `name` (System.String) — Name of the Named Position which you want to append to.
- `objects` (System.Collections.Generic.IEnumerable<RhinoObject>) — Collection of Rhino Objects to be included in this Named Position.

**Returns:** `Boolean` — True or False depending on whether the Append was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Append_2.htm)

#### `public bool Delete(Guid id)`

Delete a Named Position.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position which you want to delete.

**Returns:** `Boolean` — True or False depending on whether the Delete was successful, Null in case the id does not exist as a Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Delete.htm)

#### `public bool Delete(string name)`

Delete a Named Position.

**Parameters:**
- `name` (System.String) — Name of the Named Position which you want to delete.

**Returns:** `Boolean` — True or False depending on whether the Delete was successful, Null in case the id does not exist as a Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Delete_1.htm)

#### `public Guid Id(string name)`

Guid of a Named Position.

**Parameters:**
- `name` (System.String) — Name of the Named Position for which you want to retrieve the Guid.

**Returns:** `Guid` — The Guid of the Named Position. If not found, an empty Guid is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Id.htm)

#### `public string Name(Guid id)`

Name of a Named Position.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position for which you want to retrieve the name.

**Returns:** `String` — The name of the Named Position as a string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Name.htm)

#### `public Guid[] ObjectIds(Guid id)`

Array of Rhino Object Guids related to a Named Position.

**Parameters:**
- `id` (System.Guid) — The Guid of the named position from which you want to retrieve the objects.

**Returns:** `Guid[]` — Array of Guid which pertain to the objects tracked by the Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_ObjectIds.htm)

#### `public Guid[] ObjectIds(string name)`

Array of Rhino Object Guids related to a Named Position.

**Parameters:**
- `name` (System.String) — The name of the Named Position from which you want to retrieve the objects.

**Returns:** `Guid[]` — Array of Guid which pertain to the objects tracked by the Named Position, or null in case no such Named Position is found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_ObjectIds_1.htm)

#### `public bool ObjectXform(Guid id, Guid objId, ref Transform xform)`

Retrieve the Transform of a Rhino Object relate to a Named Position.

**Parameters:**
- `id` (System.Guid) — The Guid of the Named Position
- `objId` (System.Guid) — The Guid of the Rhino Object from which to retrieve the Transform.
- `xform` (Rhino.Geometry.Transform) — The Transform to retrieve.

**Returns:** `Boolean` — Transform of the RhinoObject related to the Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_ObjectXform_1.htm)

#### `public bool ObjectXform(Guid id, RhinoObject obj, ref Transform xform)`

Retrieve the Transform of a Rhino Object relate to a Named Position.

**Parameters:**
- `id` (System.Guid) — The Guid of the Named Position
- `obj` (Rhino.DocObjects.RhinoObject) — The Rhino Object from which to retrieve the Transform.
- `xform` (Rhino.Geometry.Transform) — The Transform to retrieve.

**Returns:** `Boolean` — Transform of the RhinoObject related to the Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_ObjectXform.htm)

#### `public RhinoObject[] Objects(Guid id)`

Array of Rhino Objects related to a Named Position.

**Parameters:**
- `id` (System.Guid) — The Guid of the named position from which you want to retrieve the objects.

**Returns:** `RhinoObject[]` — Array of Rhino Objects which are tracked by the Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Objects.htm)

#### `public RhinoObject[] Objects(string name)`

Array of Rhino Objects related to a Named Position.

**Parameters:**
- `name` (System.String) — The name of the Named Position from which you want to retrieve the objects.

**Returns:** `RhinoObject[]` — Array of Rhino Objects which are tracked by the Named Position if successful, null if no such Named Position exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Objects_1.htm)

#### `public bool Rename(Guid id, string name)`

Rename a Named Position.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position which you want to rename.
- `name` (System.String) — New name for the Named Position.

**Returns:** `Boolean` — True or False depending on whether the Rename was successful. For example, this method might return False if you attempt to rename the Named Position with the currently assigned name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Rename.htm)

#### `public bool Rename(string oldName, string name)`

Rename a Named Position.

**Parameters:**
- `oldName` (System.String) — Current name of the Named Position which you want to rename.
- `name` (System.String) — New name for the Named Position.

**Returns:** `Boolean` — True or False depending on whether the Rename was successful. For example, this method might return False if you attempt to rename the Named Position with the currently assigned name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Rename_1.htm)

#### `public bool Restore(Guid id)`

Restore a Named Position.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position to restore.

**Returns:** `Boolean` — True or False based on whether the Named Position was able to be restored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Restore.htm)

#### `public bool Restore(string name)`

Restore a Named Position.

**Parameters:**
- `name` (System.String) — Name of the Named Position to restore.

**Returns:** `Boolean` — True or False based on whether the Named Position was able to be restored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Restore_1.htm)

#### `public Guid Save(string name, IEnumerable<Guid> objectIds)`

Save a new Named Position.

**Parameters:**
- `name` (System.String) — Name for this Named Position.
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — Array of Rhino Object Ids which should be included in this Named Position.

**Returns:** `Guid` — Guid of the newly saved Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Save_1.htm)

#### `public Guid Save(string name, IEnumerable<RhinoObject> objects)`

Save a new Named Position.

**Parameters:**
- `name` (System.String) — Name for this Named Position.
- `objects` (System.Collections.Generic.IEnumerable<RhinoObject>) — Array of Rhino Objects which should be included in this Named Position.

**Returns:** `Guid` — Guid of the newly saved Named Position.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Save.htm)

#### `public bool Update(Guid id)`

Updates a Named Position, effectively storing the current positions of the objects which the Named Position is tracking.

**Parameters:**
- `id` (System.Guid) — Guid of the Named Position which you want to update.

**Returns:** `Boolean` — True or False depending on whether the Update was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Update.htm)

#### `public bool Update(string name)`

Updates a Named Position, effectively storing the current positions of the objects which the Named Position is tracking.

**Parameters:**
- `name` (System.String) — Name of the Named Position which you want to update.

**Returns:** `Boolean` — True or False depending on whether the Update was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedPositionTable_Update_1.htm)

### Properties
- `Count` (Int32, get) — Number of Named Positions in the table.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Ids` (Guid[], get) — Array of Named Position Guids.
- `Names` (String[], get) — Array of Named Position names.

## NamedViewTable (class)

All named views in a rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_NamedViewTable.htm)

### Methods
#### `public int Add(string name, Guid viewportId)`

Adds named view to document which is based on an existing viewport.

**Parameters:**
- `name` (System.String) — If name is empty, a unique name is automatically created. If there is already a named view with the same name, that view is replaced.
- `viewportId` (System.Guid) — Id of an existing viewport in the document. View information is copied from this viewport.

**Returns:** `Int32` — 0 based index of named view. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Add_1.htm)

#### `public int Add(ViewInfo view)`



**Parameters:**
- `view` (Rhino.DocObjects.ViewInfo) — [Missing <param name="view"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Add(Rhino.DocObjects.ViewInfo)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Add(Rhino.DocObjects.ViewInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Add.htm)

#### `public bool Delete(int index)`

Remove named view from the document.

**Parameters:**
- `index` (System.Int32) — index of the named view in the named view table.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Delete.htm)

#### `public bool Delete(string name)`

Remove named view from the document.

**Parameters:**
- `name` (System.String) — name of the view.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Delete_1.htm)

#### `public int FindByName(string name)`

Finds a named view.

**Parameters:**
- `name` (System.String) — name to search for.

**Returns:** `Int32` — >=0 index of the found named view -1 no named view found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_FindByName.htm)

#### `public IEnumerator<ViewInfo> GetEnumerator()`



**Returns:** `IEnumerator<ViewInfo>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_GetEnumerator.htm)

#### `public bool Rename(int index, string newName)`

Renames a named view.

**Parameters:**
- `index` (System.Int32) — Index of the named view in the named view table.
- `newName` (System.String) — The new name.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Rename.htm)

#### `public bool Rename(string oldName, string newName)`

Renames a named view.

**Parameters:**
- `oldName` (System.String) — The name of a named view in the named view table.
- `newName` (System.String) — The new name.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Rename_1.htm)

#### `[ObsoleteAttribute("Support for backgroundBitmap is ended")] public bool Restore(int index, RhinoView view, bool backgroundBitmap)`

Obsolete.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]
- `backgroundBitmap` (System.Boolean) — [Missing <param name="backgroundBitmap"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Restore.htm)

#### `public bool Restore(int index, RhinoViewport viewport)`

Sets the MainViewport of a standard RhinoView to a named views settings

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Restore_1.htm)

#### `[ObsoleteAttribute("Support for backgroundBitmap is ended")] public bool Restore(int index, RhinoViewport viewport, bool backgroundBitmap)`

Obsolete.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]
- `backgroundBitmap` (System.Boolean) — [Missing <param name="backgroundBitmap"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.Restore(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_Restore_2.htm)

#### `[ObsoleteAttribute("Support for backgroundBitmap is ended")] public bool RestoreAnimated(int index, RhinoView view, bool backgroundBitmap)`

Obsolete.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]
- `backgroundBitmap` (System.Boolean) — [Missing <param name="backgroundBitmap"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreAnimated.htm)

#### `[ObsoleteAttribute("Support for backgroundBitmap is ended")] public bool RestoreAnimated(int index, RhinoView view, bool backgroundBitmap, int frames, int frameRate)`

Obsolete.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean,System.Int32,System.Int32)"]
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean,System.Int32,System.Int32)"]
- `backgroundBitmap` (System.Boolean) — [Missing <param name="backgroundBitmap"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean,System.Int32,System.Int32)"]
- `frames` (System.Int32) — [Missing <param name="frames"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean,System.Int32,System.Int32)"]
- `frameRate` (System.Int32) — [Missing <param name="frameRate"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean,System.Int32,System.Int32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoView,System.Boolean,System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreAnimated_1.htm)

#### `[ObsoleteAttribute("Support for backgroundBitmap is ended")] public bool RestoreAnimated(int index, RhinoViewport viewport, bool backgroundBitmap)`

Obsolete.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]
- `backgroundBitmap` (System.Boolean) — [Missing <param name="backgroundBitmap"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreAnimated_2.htm)

#### `[ObsoleteAttribute("Support for backgroundBitmap is ended")] public bool RestoreAnimated(int index, RhinoViewport viewport, bool backgroundBitmap, int frames, int frameRate)`

Obsolete.

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean,System.Int32,System.Int32)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean,System.Int32,System.Int32)"]
- `backgroundBitmap` (System.Boolean) — [Missing <param name="backgroundBitmap"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean,System.Int32,System.Int32)"]
- `frames` (System.Int32) — [Missing <param name="frames"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean,System.Int32,System.Int32)"]
- `frameRate` (System.Int32) — [Missing <param name="frameRate"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean,System.Int32,System.Int32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimated(System.Int32,Rhino.Display.RhinoViewport,System.Boolean,System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreAnimated_3.htm)

#### `public bool RestoreAnimatedConstantSpeed(int index, RhinoViewport viewport, double units_per_frame, int ms_delay)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantSpeed(System.Int32,Rhino.Display.RhinoViewport,System.Double,System.Int32)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantSpeed(System.Int32,Rhino.Display.RhinoViewport,System.Double,System.Int32)"]
- `units_per_frame` (System.Double) — [Missing <param name="units_per_frame"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantSpeed(System.Int32,Rhino.Display.RhinoViewport,System.Double,System.Int32)"]
- `ms_delay` (System.Int32) — [Missing <param name="ms_delay"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantSpeed(System.Int32,Rhino.Display.RhinoViewport,System.Double,System.Int32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantSpeed(System.Int32,Rhino.Display.RhinoViewport,System.Double,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreAnimatedConstantSpeed.htm)

#### `public bool RestoreAnimatedConstantTime(int index, RhinoViewport viewport, int frames, int ms_delay)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantTime(System.Int32,Rhino.Display.RhinoViewport,System.Int32,System.Int32)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantTime(System.Int32,Rhino.Display.RhinoViewport,System.Int32,System.Int32)"]
- `frames` (System.Int32) — [Missing <param name="frames"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantTime(System.Int32,Rhino.Display.RhinoViewport,System.Int32,System.Int32)"]
- `ms_delay` (System.Int32) — [Missing <param name="ms_delay"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantTime(System.Int32,Rhino.Display.RhinoViewport,System.Int32,System.Int32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreAnimatedConstantTime(System.Int32,Rhino.Display.RhinoViewport,System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreAnimatedConstantTime.htm)

#### `public bool RestoreWithAspectRatio(int index, RhinoViewport viewport)`



**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreWithAspectRatio(System.Int32,Rhino.Display.RhinoViewport)"]
- `viewport` (Rhino.Display.RhinoViewport) — [Missing <param name="viewport"/> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreWithAspectRatio(System.Int32,Rhino.Display.RhinoViewport)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.NamedViewTable.RestoreWithAspectRatio(System.Int32,Rhino.Display.RhinoViewport)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_NamedViewTable_RestoreWithAspectRatio.htm)

### Properties
- `Count` (Int32, get) — Number of named views in the table.
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Item` (ViewInfo, get) — Conceptually, the named view table is an array of ViewInfo and their associated names. The indexing operator ([] in C#) can be used to get individual ViewInfo items.

## ObjectTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.ObjectTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_ObjectTable.htm)

### Methods
#### `public Guid Add(GeometryBase geometry)`

Adds geometry that is not further specified. This is meant, for example, to handle addition of sets of different geometrical entities.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — The base geometry. This cannot be null.

**Returns:** `Guid` — The new object ID on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Add.htm)

#### `public Guid Add(GeometryBase geometry, ObjectAttributes attributes)`

Adds geometry that is not further specified. This is meant, for example, to handle addition of sets of different geometrical entities.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — The base geometry. This cannot be null.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — The object attributes. This can be null.

**Returns:** `Guid` — The new object ID on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Add_1.htm)

#### `public Guid Add(GeometryBase geometry, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds geometry that is not further specified. This is meant, for example, to handle addition of sets of different geometrical entities.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — The base geometry. This cannot be null.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — The object attributes. This can be null.
- `history` (Rhino.DocObjects.HistoryRecord) — The history information that will be saved.
- `reference` (System.Boolean) — If reference is true, object will not be saved in the 3dm file.

**Returns:** `Guid` — The new object ID on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Add_2.htm)

#### `public Guid AddAngularDimension(AngularDimension dimension)`

Adds a angular dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.AngularDimension) — Dimension object to add.

**Returns:** `Guid` — The Id of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddAngularDimension.htm)

#### `public Guid AddAngularDimension(AngularDimension dimension, ObjectAttributes attributes)`

Adds a angular dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.AngularDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddAngularDimension(Rhino.Geometry.AngularDimension,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddAngularDimension_1.htm)

#### `public Guid AddAngularDimension(AngularDimension dimension, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds an angular dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.AngularDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddAngularDimension_2.htm)

#### `public Guid AddArc(Arc arc)`

Adds a curve object to the document representing an arc.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc value.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddArc.htm)

#### `public Guid AddArc(Arc arc, ObjectAttributes attributes)`

Adds a curve object to the document representing an arc.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc value.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to arc.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddArc_1.htm)

#### `public Guid AddArc(Arc arc, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `arc` (Rhino.Geometry.Arc) — [Missing <param name="arc"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddArc(Rhino.Geometry.Arc,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddArc(Rhino.Geometry.Arc,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddArc(Rhino.Geometry.Arc,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddArc(Rhino.Geometry.Arc,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddArc(Rhino.Geometry.Arc,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddArc_2.htm)

#### `public Guid AddBox(Box box)`

Adds a box to the object table.

**Parameters:**
- `box` (Rhino.Geometry.Box) — The box.

**Returns:** `Guid` — The ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBox.htm)

#### `public Guid AddBox(Box box, ObjectAttributes attributes)`

Adds a box to the object table.

**Parameters:**
- `box` (Rhino.Geometry.Box) — The box.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the surface object.

**Returns:** `Guid` — The ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBox_1.htm)

#### `public Guid AddBox(Box box, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a box to the object table, as an extrusion.

**Parameters:**
- `box` (Rhino.Geometry.Box) — The box.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the surface object.
- `history` (Rhino.DocObjects.HistoryRecord) — History data records.
- `reference` (System.Boolean) — If a reference, object will not be saved in the document.

**Returns:** `Guid` — The ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBox_2.htm)

#### `public Guid AddBrep(Brep brep)`

Adds a brep object to Rhino.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A duplicate of this brep is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBrep.htm)

#### `public Guid AddBrep(Brep brep, ObjectAttributes attributes)`

Adds a brep object to Rhino.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A duplicate of this brep is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to brep.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBrep_1.htm)

#### `public Guid AddBrep(Brep brep, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `brep` (Rhino.Geometry.Brep) — [Missing <param name="brep"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBrep_2.htm)

#### `public Guid AddBrep(Brep brep, ObjectAttributes attributes, HistoryRecord history, bool reference, bool splitKinkySurfaces)`



**Parameters:**
- `brep` (Rhino.Geometry.Brep) — [Missing <param name="brep"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `splitKinkySurfaces` (System.Boolean) — [Missing <param name="splitKinkySurfaces"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddBrep(Rhino.Geometry.Brep,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddBrep_3.htm)

#### `public Guid AddCentermark(Centermark centermark, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds an ordinate dimension object to the document.

**Parameters:**
- `centermark` (Rhino.Geometry.Centermark) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCentermark.htm)

#### `public Guid AddCircle(Circle circle)`

Adds a curve object to the document representing a circle.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle value.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCircle.htm)

#### `public Guid AddCircle(Circle circle, ObjectAttributes attributes)`

Adds a curve object to the document representing a circle.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle value.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to circle.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCircle_1.htm)

#### `public Guid AddCircle(Circle circle, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `circle` (Rhino.Geometry.Circle) — [Missing <param name="circle"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCircle(Rhino.Geometry.Circle,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCircle(Rhino.Geometry.Circle,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCircle(Rhino.Geometry.Circle,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCircle(Rhino.Geometry.Circle,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCircle(Rhino.Geometry.Circle,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCircle_2.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, Guid clippedViewportId)`

Adds a clipping plane object to Rhino.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane value.
- `uMagnitude` (System.Double) — The size in the U direction.
- `vMagnitude` (System.Double) — The size in the V direction.
- `clippedViewportId` (System.Guid) — Viewport ID that the new clipping plane will clip.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddClippingPlane_3.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, Guid clippedViewportId, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `uMagnitude` (System.Double) — [Missing <param name="uMagnitude"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `vMagnitude` (System.Double) — [Missing <param name="vMagnitude"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `clippedViewportId` (System.Guid) — [Missing <param name="clippedViewportId"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Guid,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddClippingPlane_4.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, IEnumerable<Guid> clippedViewportIds)`

Adds a clipping plane object to Rhino.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane value.
- `uMagnitude` (System.Double) — The size in the U direction.
- `vMagnitude` (System.Double) — The size in the V direction.
- `clippedViewportIds` (System.Collections.Generic.IEnumerable<Guid>) — A list, an array or any enumerable set of viewport IDs that the new clipping plane will clip.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddClippingPlane.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, IEnumerable<Guid> clippedViewportIds, ObjectAttributes attributes)`

Adds a clipping plane object to Rhino.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane value.
- `uMagnitude` (System.Double) — The size in the U direction.
- `vMagnitude` (System.Double) — The size in the V direction.
- `clippedViewportIds` (System.Collections.Generic.IEnumerable<Guid>) — A list, an array or any enumerable set of viewport IDs that the new clipping plane will clip.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Document attributes for the plane.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddClippingPlane_1.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, IEnumerable<Guid> clippedViewportIds, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `uMagnitude` (System.Double) — [Missing <param name="uMagnitude"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `vMagnitude` (System.Double) — [Missing <param name="vMagnitude"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `clippedViewportIds` (System.Collections.Generic.IEnumerable<Guid>) — [Missing <param name="clippedViewportIds"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddClippingPlane(Rhino.Geometry.Plane,System.Double,System.Double,System.Collections.Generic.IEnumerable{System.Guid},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddClippingPlane_2.htm)

#### `public Guid AddCurve(Curve curve)`

Adds a curve object to Rhino.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A curve. A duplicate of this curve is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCurve.htm)

#### `public Guid AddCurve(Curve curve, ObjectAttributes attributes)`

Adds a curve object to Rhino.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A curve. A duplicate of this curve is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to curve.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCurve_1.htm)

#### `public Guid AddCurve(Curve curve, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `curve` (Rhino.Geometry.Curve) — [Missing <param name="curve"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCurve(Rhino.Geometry.Curve,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCurve(Rhino.Geometry.Curve,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCurve(Rhino.Geometry.Curve,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCurve(Rhino.Geometry.Curve,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddCurve(Rhino.Geometry.Curve,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddCurve_2.htm)

#### `public Guid AddEllipse(Ellipse ellipse)`

Adds a curve object to the document representing an ellipse.

**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — An ellipse value.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddEllipse.htm)

#### `public Guid AddEllipse(Ellipse ellipse, ObjectAttributes attributes)`

Adds a curve object to the document representing an ellipse.

**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — An ellipse value.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to ellipse.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddEllipse_1.htm)

#### `public Guid AddEllipse(Ellipse ellipse, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — [Missing <param name="ellipse"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddEllipse(Rhino.Geometry.Ellipse,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddEllipse(Rhino.Geometry.Ellipse,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddEllipse(Rhino.Geometry.Ellipse,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddEllipse(Rhino.Geometry.Ellipse,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddEllipse(Rhino.Geometry.Ellipse,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddEllipse_2.htm)

#### `public Guid[] AddExplodedInstancePieces(InstanceObject instance, bool explodeNestedInstances = true, bool deleteInstance = false)`



**Parameters:**
- `instance` (Rhino.DocObjects.InstanceObject) — [Missing <param name="instance"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExplodedInstancePieces(Rhino.DocObjects.InstanceObject,System.Boolean,System.Boolean)"]
- `explodeNestedInstances` (System.Boolean) — [Missing <param name="explodeNestedInstances"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExplodedInstancePieces(Rhino.DocObjects.InstanceObject,System.Boolean,System.Boolean)"]
- `deleteInstance` (System.Boolean) — [Missing <param name="deleteInstance"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExplodedInstancePieces(Rhino.DocObjects.InstanceObject,System.Boolean,System.Boolean)"]

**Returns:** `Guid[]` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExplodedInstancePieces(Rhino.DocObjects.InstanceObject,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddExplodedInstancePieces.htm)

#### `public Guid AddExtrusion(Extrusion extrusion)`

Adds an extrusion object to Rhino.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — A duplicate of this extrusion is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddExtrusion.htm)

#### `public Guid AddExtrusion(Extrusion extrusion, ObjectAttributes attributes)`

Adds an extrusion object to Rhino.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — A duplicate of this extrusion is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the extrusion object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddExtrusion_1.htm)

#### `public Guid AddExtrusion(Extrusion extrusion, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — [Missing <param name="extrusion"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExtrusion(Rhino.Geometry.Extrusion,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExtrusion(Rhino.Geometry.Extrusion,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExtrusion(Rhino.Geometry.Extrusion,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExtrusion(Rhino.Geometry.Extrusion,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddExtrusion(Rhino.Geometry.Extrusion,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddExtrusion_2.htm)

#### `public Guid AddHatch(Hatch hatch)`



**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddHatch.htm)

#### `public Guid AddHatch(Hatch hatch, ObjectAttributes attributes)`



**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddHatch_1.htm)

#### `public Guid AddHatch(Hatch hatch, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — [Missing <param name="hatch"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddHatch(Rhino.Geometry.Hatch,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddHatch_2.htm)

#### `public Guid AddInstanceObject(int instanceDefinitionIndex, Transform instanceXform)`

Adds an instance object to the document.

**Parameters:**
- `instanceDefinitionIndex` (System.Int32) — The index of the instance definition.
- `instanceXform` (Rhino.Geometry.Transform) — The instance transformation.

**Returns:** `Guid` — A unique identifier for the object if successful. Guid.Empty it not successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddInstanceObject.htm)

#### `public Guid AddInstanceObject(int instanceDefinitionIndex, Transform instanceXform, ObjectAttributes attributes)`

Adds an instance object to the document.

**Parameters:**
- `instanceDefinitionIndex` (System.Int32) — The index of the instance definition.
- `instanceXform` (Rhino.Geometry.Transform) — The instance transformation.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — The attributes to apply to the instance object.

**Returns:** `Guid` — A unique identifier for the object if successful. Guid.Empty it not successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddInstanceObject_1.htm)

#### `public Guid AddInstanceObject(int instanceDefinitionIndex, Transform instanceXform, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds an instance object to the document.

**Parameters:**
- `instanceDefinitionIndex` (System.Int32) — The index of the instance definition.
- `instanceXform` (Rhino.Geometry.Transform) — The instance transformation.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — The attributes to apply to the instance object.
- `history` (Rhino.DocObjects.HistoryRecord) — The history record associated with this instance object.
- `reference` (System.Boolean) — true if the object is from a reference file. Reference objects do not persist in archives.

**Returns:** `Guid` — A unique identifier for the object if successful. Guid.Empty it not successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddInstanceObject_2.htm)

#### `public Guid AddLeader(IEnumerable<Point3d> points)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_5.htm)

#### `public Guid AddLeader(Leader leader)`

Adds a Leader object to the document.

**Parameters:**
- `leader` (Rhino.Geometry.Leader) — The leader object.

**Returns:** `Guid` — The Id of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader.htm)

#### `public Guid AddLeader(Leader leader, ObjectAttributes attributes)`

Adds Leader object to the document.

**Parameters:**
- `leader` (Rhino.Geometry.Leader) — The leader object.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to rich text.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_1.htm)

#### `public Guid AddLeader(Leader leader, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a Leader object to the document.

**Parameters:**
- `leader` (Rhino.Geometry.Leader) — The leader object.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to rich text.
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_2.htm)

#### `public Guid AddLeader(Plane plane, IEnumerable<Point2d> points)`



**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_3.htm)

#### `public Guid AddLeader(Plane plane, IEnumerable<Point2d> points, ObjectAttributes attributes)`



**Parameters:**
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_4.htm)

#### `public Guid AddLeader(string text, IEnumerable<Point3d> points)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_9.htm)

#### `public Guid AddLeader(string text, Plane plane, IEnumerable<Point2d> points)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_6.htm)

#### `public Guid AddLeader(string text, Plane plane, IEnumerable<Point2d> points, ObjectAttributes attributes)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_7.htm)

#### `public Guid AddLeader(string text, Plane plane, IEnumerable<Point2d> points, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLeader(System.String,Rhino.Geometry.Plane,System.Collections.Generic.IEnumerable{Rhino.Geometry.Point2d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLeader_8.htm)

#### `public Guid AddLine(Line line)`

Adds a line object to Rhino.

**Parameters:**
- `line` (Rhino.Geometry.Line) — [Missing <param name="line"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Line)"]

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLine.htm)

#### `public Guid AddLine(Line line, ObjectAttributes attributes)`

Adds a line object to Rhino.

**Parameters:**
- `line` (Rhino.Geometry.Line) — The line value.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to line.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLine_1.htm)

#### `public Guid AddLine(Point3d from, Point3d to)`

Adds a line object to Rhino.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — The line origin.
- `to` (Rhino.Geometry.Point3d) — The line end.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLine_2.htm)

#### `public Guid AddLine(Point3d from, Point3d to, ObjectAttributes attributes)`

Adds a line object to Rhino.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — The line origin.
- `to` (Rhino.Geometry.Point3d) — The line end.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to line.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLine_3.htm)

#### `public Guid AddLine(Point3d from, Point3d to, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `from` (Rhino.Geometry.Point3d) — [Missing <param name="from"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `to` (Rhino.Geometry.Point3d) — [Missing <param name="to"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLine(Rhino.Geometry.Point3d,Rhino.Geometry.Point3d,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLine_4.htm)

#### `public Guid AddLinearDimension(LinearDimension dimension)`

Adds a linear dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.LinearDimension) — Dimension object to add.

**Returns:** `Guid` — The Id of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLinearDimension.htm)

#### `public Guid AddLinearDimension(LinearDimension dimension, ObjectAttributes attributes)`

Adds a linear dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.LinearDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddLinearDimension(Rhino.Geometry.LinearDimension,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLinearDimension_1.htm)

#### `public Guid AddLinearDimension(LinearDimension dimension, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a linear dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.LinearDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddLinearDimension_2.htm)

#### `public Guid AddMesh(Mesh mesh)`

Adds a mesh object to Rhino.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A duplicate of this mesh is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMesh.htm)

#### `public Guid AddMesh(Mesh mesh, ObjectAttributes attributes)`

Adds a mesh object to Rhino.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A duplicate of this mesh is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the mesh object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMesh_1.htm)

#### `public Guid AddMesh(Mesh mesh, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMesh_2.htm)

#### `public Guid AddMesh(Mesh mesh, ObjectAttributes attributes, HistoryRecord history, bool reference, bool requireValidMesh)`



**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]
- `requireValidMesh` (System.Boolean) — [Missing <param name="requireValidMesh"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMesh(Rhino.Geometry.Mesh,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMesh_3.htm)

#### `public Guid AddMorphControl(MorphControl morphControl)`



**Parameters:**
- `morphControl` (Rhino.Geometry.MorphControl) — [Missing <param name="morphControl"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMorphControl.htm)

#### `public Guid AddMorphControl(MorphControl morphControl, ObjectAttributes attributes)`



**Parameters:**
- `morphControl` (Rhino.Geometry.MorphControl) — [Missing <param name="morphControl"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMorphControl_1.htm)

#### `public Guid AddMorphControl(MorphControl morphControl, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `morphControl` (Rhino.Geometry.MorphControl) — [Missing <param name="morphControl"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddMorphControl(Rhino.Geometry.MorphControl,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddMorphControl_2.htm)

#### `public Guid AddOrdinateDimension(OrdinateDimension dimordinate, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds an ordinate dimension object to the document.

**Parameters:**
- `dimordinate` (Rhino.Geometry.OrdinateDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddOrdinateDimension.htm)

#### `public Guid AddPictureFrame(Plane plane, string texturePath, bool asMesh, double width, double height, bool selfIllumination, bool embedBitmap)`

Creates a PictureFrame object from a plane and a path to an image file, Note, a PictureFrame object is just a Plane surface or mesh that has a material with a texture assigned to it that displays in all display modes.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane in which the PictureFrame will be created. Bottom left corner of picture will be at plane's origin, width will be in the plane's x axis direction, height will be in the plane's y axis direction.
- `texturePath` (System.String) — path to an image file
- `asMesh` (System.Boolean) — If true, the function will make a MeshObject rather than a surface
- `width` (System.Double) — Width of the resulting PictureFrame. If 0.0, the width of the picture frame is the width of the image if height is also 0.0 or calculated from the height and aspect ratio of the image if height is not 0.0.
- `height` (System.Double) — Height of the resulting PictureFrame. If 0.0, the height of the picture frame is the height of the image if width is also 0.0 or calculated from the width and aspect ratio of the image if width is not 0.0.
- `selfIllumination` (System.Boolean) — If true, the image mapped to the picture frame plane always displays at full intensity and is not affected by light or shadow.
- `embedBitmap` (System.Boolean) — If true, the function adds the image to the bitmap table of the document to which the PictureFrame will be added

**Returns:** `Guid` — A unique identifier for the object

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPictureFrame.htm)

#### `public Guid AddPoint(double x, double y, double z)`

Adds a point object to the document.

**Parameters:**
- `x` (System.Double) — X component of point coordinate.
- `y` (System.Double) — Y component of point coordinate.
- `z` (System.Double) — Z component of point coordinate.

**Returns:** `Guid` — A unique identifier for the object..

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint_6.htm)

#### `public Guid AddPoint(Point point, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a point object and its geometry-linked information to the document

**Parameters:**
- `point` (Rhino.Geometry.Point) — A point geometry class.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point. null is acceptable
- `history` (Rhino.DocObjects.HistoryRecord) — history associated with this point. null is acceptable
- `reference` (System.Boolean) — true if the object is from a reference file. Reference objects do not persist in archives

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint.htm)

#### `public Guid AddPoint(Point3d point)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — location of point.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint_1.htm)

#### `public Guid AddPoint(Point3d point, ObjectAttributes attributes)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — location of point.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point. null is acceptable

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint_2.htm)

#### `public Guid AddPoint(Point3d point, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a point object to the document

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — location of point
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point. null is acceptable
- `history` (Rhino.DocObjects.HistoryRecord) — history associated with this point. null is acceptable
- `reference` (System.Boolean) — true if the object is from a reference file. Reference objects do not persist in archives

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint_3.htm)

#### `public Guid AddPoint(Point3f point)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3f) — location of point.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint_4.htm)

#### `public Guid AddPoint(Point3f point, ObjectAttributes attributes)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3f) — location of point.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoint_5.htm)

#### `public Guid AddPointCloud(IEnumerable<Point3d> points)`

Adds a point cloud object to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of points.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPointCloud_3.htm)

#### `public Guid AddPointCloud(IEnumerable<Point3d> points, ObjectAttributes attributes)`

Adds a point cloud object to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of points.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point cloud.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPointCloud_4.htm)

#### `public Guid AddPointCloud(IEnumerable<Point3d> points, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a point cloud object to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of points
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point cloud. null is acceptable
- `history` (Rhino.DocObjects.HistoryRecord) — history associated with this point cloud. null is acceptable
- `reference` (System.Boolean) — true if the object is from a reference file. Reference objects do not persist in archives

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPointCloud_5.htm)

#### `public Guid AddPointCloud(PointCloud cloud)`

Adds a point cloud object to the document.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — PointCloud to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPointCloud.htm)

#### `public Guid AddPointCloud(PointCloud cloud, ObjectAttributes attributes)`

Adds a point cloud object to the document.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — PointCloud to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point cloud.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPointCloud_1.htm)

#### `public Guid AddPointCloud(PointCloud cloud, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a point cloud object to the document.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — PointCloud to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point cloud. null is acceptable
- `history` (Rhino.DocObjects.HistoryRecord) — history associated with this point cloud. null is acceptable
- `reference` (System.Boolean) — true if the object is from a reference file. Reference objects do not persist in archives

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPointCloud_2.htm)

#### `public RhinoList<Guid> AddPoints(IEnumerable<Point3d> points)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.

**Returns:** `RhinoList<Guid>` — List of object ids.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoints.htm)

#### `public RhinoList<Guid> AddPoints(IEnumerable<Point3d> points, ObjectAttributes attributes)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point objects.

**Returns:** `RhinoList<Guid>` — List of object ids.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoints_1.htm)

#### `public RhinoList<Guid> AddPoints(IEnumerable<Point3f> points)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3f>) — Points to add.

**Returns:** `RhinoList<Guid>` — List of object ids.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoints_2.htm)

#### `public RhinoList<Guid> AddPoints(IEnumerable<Point3f> points, ObjectAttributes attributes)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3f>) — Points to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point objects.

**Returns:** `RhinoList<Guid>` — List of object ids.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPoints_3.htm)

#### `public Guid AddPolyline(IEnumerable<Point3d> points)`

Adds a polyline object to Rhino.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A Polyline; a list, an array, or any enumerable set of Point3d.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPolyline.htm)

#### `public Guid AddPolyline(IEnumerable<Point3d> points, ObjectAttributes attributes)`

Adds a polyline object to Rhino.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A Polyline; a list, an array, or any enumerable set of Point3d.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to line.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPolyline_1.htm)

#### `public Guid AddPolyline(IEnumerable<Point3d> points, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — [Missing <param name="points"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddPolyline(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddPolyline(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddPolyline(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddPolyline(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddPolyline(System.Collections.Generic.IEnumerable{Rhino.Geometry.Point3d},Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddPolyline_2.htm)

#### `public Guid AddRadialDimension(RadialDimension dimension)`



**Parameters:**
- `dimension` (Rhino.Geometry.RadialDimension) — [Missing <param name="dimension"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRadialDimension(Rhino.Geometry.RadialDimension)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRadialDimension(Rhino.Geometry.RadialDimension)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRadialDimension.htm)

#### `public Guid AddRadialDimension(RadialDimension dimension, ObjectAttributes attributes)`



**Parameters:**
- `dimension` (Rhino.Geometry.RadialDimension) — [Missing <param name="dimension"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRadialDimension(Rhino.Geometry.RadialDimension,Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRadialDimension(Rhino.Geometry.RadialDimension,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRadialDimension(Rhino.Geometry.RadialDimension,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRadialDimension_1.htm)

#### `public Guid AddRadialDimension(RadialDimension dimension, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a radial dimension object to the document.

**Parameters:**
- `dimension` (Rhino.Geometry.RadialDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRadialDimension_2.htm)

#### `public Guid AddRectangle(Rectangle3d rectangle)`

Adds a rectangle to the object table.

**Parameters:**
- `rectangle` (Rhino.Geometry.Rectangle3d) — The rectangle.

**Returns:** `Guid` — The ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRectangle.htm)

#### `public Guid AddRectangle(Rectangle3d rectangle, ObjectAttributes attributes)`

Adds a rectangle to the object table.

**Parameters:**
- `rectangle` (Rhino.Geometry.Rectangle3d) — The rectangle.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the surface object.

**Returns:** `Guid` — The ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRectangle_1.htm)

#### `public Guid AddRectangle(Rectangle3d rectangle, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a rectangle to the object table.

**Parameters:**
- `rectangle` (Rhino.Geometry.Rectangle3d) — The rectangle.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the surface object.
- `history` (Rhino.DocObjects.HistoryRecord) — History data records.
- `reference` (System.Boolean) — If a reference, object will not be saved in the document.

**Returns:** `Guid` — The ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRectangle_2.htm)

#### `public void AddRhinoObject(BrepObject brepObject, Brep brep)`



**Parameters:**
- `brepObject` (Rhino.DocObjects.BrepObject) — [Missing <param name="brepObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.BrepObject,Rhino.Geometry.Brep)"]
- `brep` (Rhino.Geometry.Brep) — [Missing <param name="brep"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.BrepObject,Rhino.Geometry.Brep)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject.htm)

#### `public void AddRhinoObject(CurveObject curveObject, Curve curve)`



**Parameters:**
- `curveObject` (Rhino.DocObjects.CurveObject) — [Missing <param name="curveObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.CurveObject,Rhino.Geometry.Curve)"]
- `curve` (Rhino.Geometry.Curve) — [Missing <param name="curve"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.CurveObject,Rhino.Geometry.Curve)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_1.htm)

#### `public void AddRhinoObject(CustomBrepObject brepObject)`



**Parameters:**
- `brepObject` (Rhino.DocObjects.Custom.CustomBrepObject) — [Missing <param name="brepObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomBrepObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_2.htm)

#### `public void AddRhinoObject(CustomBrepObject brepObject, HistoryRecord history)`



**Parameters:**
- `brepObject` (Rhino.DocObjects.Custom.CustomBrepObject) — [Missing <param name="brepObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomBrepObject,Rhino.DocObjects.HistoryRecord)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomBrepObject,Rhino.DocObjects.HistoryRecord)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_3.htm)

#### `public void AddRhinoObject(CustomCurveObject curveObject, HistoryRecord history)`



**Parameters:**
- `curveObject` (Rhino.DocObjects.Custom.CustomCurveObject) — [Missing <param name="curveObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomCurveObject,Rhino.DocObjects.HistoryRecord)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomCurveObject,Rhino.DocObjects.HistoryRecord)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_4.htm)

#### `public void AddRhinoObject(CustomMeshObject meshObject)`



**Parameters:**
- `meshObject` (Rhino.DocObjects.Custom.CustomMeshObject) — [Missing <param name="meshObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomMeshObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_5.htm)

#### `public void AddRhinoObject(CustomMeshObject meshObject, HistoryRecord history)`



**Parameters:**
- `meshObject` (Rhino.DocObjects.Custom.CustomMeshObject) — [Missing <param name="meshObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomMeshObject,Rhino.DocObjects.HistoryRecord)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomMeshObject,Rhino.DocObjects.HistoryRecord)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_6.htm)

#### `public void AddRhinoObject(CustomPointObject pointObject)`



**Parameters:**
- `pointObject` (Rhino.DocObjects.Custom.CustomPointObject) — [Missing <param name="pointObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomPointObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_7.htm)

#### `public void AddRhinoObject(CustomPointObject pointObject, HistoryRecord history)`



**Parameters:**
- `pointObject` (Rhino.DocObjects.Custom.CustomPointObject) — [Missing <param name="pointObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomPointObject,Rhino.DocObjects.HistoryRecord)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.Custom.CustomPointObject,Rhino.DocObjects.HistoryRecord)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_8.htm)

#### `public void AddRhinoObject(MeshObject meshObject, Mesh mesh)`



**Parameters:**
- `meshObject` (Rhino.DocObjects.MeshObject) — [Missing <param name="meshObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.MeshObject,Rhino.Geometry.Mesh)"]
- `mesh` (Rhino.Geometry.Mesh) — [Missing <param name="mesh"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.MeshObject,Rhino.Geometry.Mesh)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_9.htm)

#### `public void AddRhinoObject(PointObject pointObject, Point point)`



**Parameters:**
- `pointObject` (Rhino.DocObjects.PointObject) — [Missing <param name="pointObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.PointObject,Rhino.Geometry.Point)"]
- `point` (Rhino.Geometry.Point) — [Missing <param name="point"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddRhinoObject(Rhino.DocObjects.PointObject,Rhino.Geometry.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddRhinoObject_10.htm)

#### `public Guid AddSphere(Sphere sphere)`



**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — [Missing <param name="sphere"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSphere.htm)

#### `public Guid AddSphere(Sphere sphere, ObjectAttributes attributes)`



**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — [Missing <param name="sphere"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSphere_1.htm)

#### `public Guid AddSphere(Sphere sphere, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — [Missing <param name="sphere"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSphere(Rhino.Geometry.Sphere,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSphere_2.htm)

#### `public Guid AddSubD(SubD subD)`

Adds a SubD object to Rhino.

**Parameters:**
- `subD` (Rhino.Geometry.SubD) — A duplicate of this SubD is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSubD.htm)

#### `public Guid AddSubD(SubD subD, ObjectAttributes attributes)`

Adds a SubD object to Rhino.

**Parameters:**
- `subD` (Rhino.Geometry.SubD) — A duplicate of this SubD is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSubD_1.htm)

#### `public Guid AddSubD(SubD subD, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a SubD object to Rhino.

**Parameters:**
- `subD` (Rhino.Geometry.SubD) — A duplicate of this SubD is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the object.
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSubD(Rhino.Geometry.SubD,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSubD(Rhino.Geometry.SubD,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSubD_2.htm)

#### `public Guid AddSurface(Surface surface)`

Adds a surface object to Rhino.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — A duplicate of this surface is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSurface.htm)

#### `public Guid AddSurface(Surface surface, ObjectAttributes attributes)`

Adds a surface object to Rhino.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — A duplicate of this surface is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the surface object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSurface_1.htm)

#### `public Guid AddSurface(Surface surface, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `surface` (Rhino.Geometry.Surface) — [Missing <param name="surface"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSurface(Rhino.Geometry.Surface,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSurface(Rhino.Geometry.Surface,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSurface(Rhino.Geometry.Surface,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSurface(Rhino.Geometry.Surface,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddSurface(Rhino.Geometry.Surface,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddSurface_2.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic)`

Adds an annotation text object to the document.

**Parameters:**
- `text` (System.String) — Text string.
- `plane` (Rhino.Geometry.Plane) — Plane of text.
- `height` (System.Double) — Height of text.
- `fontName` (System.String) — Name of FontFace.
- `bold` (System.Boolean) — Bold flag.
- `italic` (System.Boolean) — Italic flag.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_5.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, ObjectAttributes attributes)`

Adds an annotation text object to the document.

**Parameters:**
- `text` (System.String) — Text string.
- `plane` (Rhino.Geometry.Plane) — Plane of text.
- `height` (System.Double) — Height of text.
- `fontName` (System.String) — Name of FontFace.
- `bold` (System.Boolean) — Bold flag.
- `italic` (System.Boolean) — Italic flag.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes that will be linked with the object.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_6.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `fontName` (System.String) — [Missing <param name="fontName"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_7.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, TextJustification justification)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]
- `fontName` (System.String) — [Missing <param name="fontName"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]
- `justification` (Rhino.Geometry.TextJustification) — [Missing <param name="justification"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_8.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, TextJustification justification, ObjectAttributes attributes)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `fontName` (System.String) — [Missing <param name="fontName"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `justification` (Rhino.Geometry.TextJustification) — [Missing <param name="justification"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_9.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, TextJustification justification, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `plane` (Rhino.Geometry.Plane) — [Missing <param name="plane"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `height` (System.Double) — [Missing <param name="height"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `fontName` (System.String) — [Missing <param name="fontName"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `bold` (System.Boolean) — [Missing <param name="bold"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `italic` (System.Boolean) — [Missing <param name="italic"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `justification` (Rhino.Geometry.TextJustification) — [Missing <param name="justification"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(System.String,Rhino.Geometry.Plane,System.Double,System.String,System.Boolean,System.Boolean,Rhino.Geometry.TextJustification,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_10.htm)

#### `public Guid AddText(Text3d text3d)`

Adds an annotation text object to the document.

**Parameters:**
- `text3d` (Rhino.Display.Text3d) — The text object to add.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText.htm)

#### `public Guid AddText(Text3d text3d, ObjectAttributes attributes)`

Adds an annotation text object to the document.

**Parameters:**
- `text3d` (Rhino.Display.Text3d) — The text object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Object Attributes.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_1.htm)

#### `public Guid AddText(TextEntity text)`

Adds a v6_TextObject to the document.

**Parameters:**
- `text` (Rhino.Geometry.TextEntity) — Text object to add.

**Returns:** `Guid` — The Id of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_2.htm)

#### `public Guid AddText(TextEntity text, ObjectAttributes attributes)`

Adds a text object to the document.

**Parameters:**
- `text` (Rhino.Geometry.TextEntity) — Text object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(Rhino.Geometry.TextEntity,Rhino.DocObjects.ObjectAttributes)"]

**Returns:** `Guid` — The Id of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_3.htm)

#### `public Guid AddText(TextEntity text, ObjectAttributes attributes, HistoryRecord history, bool reference)`

Adds a text object to the document.

**Parameters:**
- `text` (Rhino.Geometry.TextEntity) — Text object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddText(Rhino.Geometry.TextEntity,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — Object history to save.
- `reference` (System.Boolean) — If reference, then object will not be saved into the 3dm file.

**Returns:** `Guid` — The Id of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddText_4.htm)

#### `public Guid AddTextDot(string text, Point3d location)`

Adds a text dot object to Rhino.

**Parameters:**
- `text` (System.String) — A text string.
- `location` (Rhino.Geometry.Point3d) — A point position.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddTextDot_3.htm)

#### `public Guid AddTextDot(string text, Point3d location, ObjectAttributes attributes)`

Adds a text dot object to Rhino.

**Parameters:**
- `text` (System.String) — A text string.
- `location` (Rhino.Geometry.Point3d) — A point position.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to curve.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddTextDot_4.htm)

#### `public Guid AddTextDot(TextDot dot)`

Adds a text dot object to Rhino.

**Parameters:**
- `dot` (Rhino.Geometry.TextDot) — A text dot that will be copied.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddTextDot.htm)

#### `public Guid AddTextDot(TextDot dot, ObjectAttributes attributes)`

Adds a text dot object to Rhino.

**Parameters:**
- `dot` (Rhino.Geometry.TextDot) — A text dot that will be copied.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to text dot.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddTextDot_1.htm)

#### `public Guid AddTextDot(TextDot dot, ObjectAttributes attributes, HistoryRecord history, bool reference)`



**Parameters:**
- `dot` (Rhino.Geometry.TextDot) — [Missing <param name="dot"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddTextDot(Rhino.Geometry.TextDot,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `attributes` (Rhino.DocObjects.ObjectAttributes) — [Missing <param name="attributes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddTextDot(Rhino.Geometry.TextDot,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `history` (Rhino.DocObjects.HistoryRecord) — [Missing <param name="history"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddTextDot(Rhino.Geometry.TextDot,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]
- `reference` (System.Boolean) — [Missing <param name="reference"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddTextDot(Rhino.Geometry.TextDot,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.AddTextDot(Rhino.Geometry.TextDot,Rhino.DocObjects.ObjectAttributes,Rhino.DocObjects.HistoryRecord,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AddTextDot_2.htm)

#### `public RhinoObject[] AllObjectsSince(uint runtimeSerialNumber)`

Gets all the objects that have been added to the document since a given runtime serial number.

**Parameters:**
- `runtimeSerialNumber` (System.UInt32) — Runtime serial number of the last object not to include in the list.

**Returns:** `RhinoObject[]` — An array of objects or null if no objects were added since the given runtime serial number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_AllObjectsSince.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public bool Delete(Guid objectId, bool quiet)`

Deletes object from document. The deletion can be undone by calling UndeleteObject().

**Parameters:**
- `objectId` (System.Guid) — Id of the object to delete.
- `quiet` (System.Boolean) — If false, a message box will appear when an object cannot be deleted.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete_6.htm)

#### `public int Delete(IEnumerable<Guid> objectIds, bool quiet)`

Deletes a collection of objects from the document.

**Parameters:**
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — Ids of all objects to delete.
- `quiet` (System.Boolean) — If false, a message box will appear when an object cannot be deleted.

**Returns:** `Int32` — The number of successfully deleted objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete_5.htm)

#### `public bool Delete(ObjRef objref, bool quiet)`

Deletes objref.Object(). The deletion can be undone by calling UndeleteObject().

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — objref.Object() will be deleted.
- `quiet` (System.Boolean) — If false, a message box will appear when an object cannot be deleted.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete.htm)

#### `public bool Delete(ObjRef objref, bool quiet, bool ignoreModes)`

Deletes objref.Object(). The deletion can be undone by calling UndeleteObject().

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — objref.Object() will be deleted.
- `quiet` (System.Boolean) — If false, a message box will appear when an object cannot be deleted.
- `ignoreModes` (System.Boolean) — If true, locked and hidden objects are deleted. If false objects that are locked, hidden, or on locked or hidden layers are not deleted.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete_1.htm)

#### `public override bool Delete(RhinoObject item)`

Deletes an object, taking into account modes and not showing error message boxes.

**Parameters:**
- `item` (Rhino.DocObjects.RhinoObject) — The object to delete.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete_2.htm)

#### `public bool Delete(RhinoObject obj, bool quiet)`

Deletes object from document. The deletion can be undone by calling UndeleteObject().

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — The object to delete.
- `quiet` (System.Boolean) — If false, a message box will appear when an object cannot be deleted.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete_3.htm)

#### `public bool Delete(RhinoObject obj, bool quiet, bool ignoreModes)`

Deletes object from document. The deletion can be undone by calling UndeleteObject().

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — The object to delete.
- `quiet` (System.Boolean) — If false, a message box will appear when an object cannot be deleted.
- `ignoreModes` (System.Boolean) — If true, locked and hidden objects are deleted. If false objects that are locked, hidden, or on locked or hidden layers are not deleted.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Delete_4.htm)

#### `public bool DeleteGrip(GripObject grip)`

Deletes a grip object.

**Parameters:**
- `grip` (Rhino.DocObjects.GripObject) — The grip object to delete.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrip.htm)

#### `public bool DeleteGrip(Guid gripId)`

Deletes a grip object.

**Parameters:**
- `gripId` (System.Guid) — The id of the grip object to delete.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrip_2.htm)

#### `public bool DeleteGrip(ObjRef gripRef)`

Deletes a grip object.

**Parameters:**
- `gripRef` (Rhino.DocObjects.ObjRef) — A reference to the grip object to delete.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrip_1.htm)

#### `public int DeleteGrips(IEnumerable<GripObject> grips)`

Deletes one or more grip objects.

**Parameters:**
- `grips` (System.Collections.Generic.IEnumerable<GripObject>) — The grip objects to delete.

**Returns:** `Int32` — The number of successfully deleted grip objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrips_1.htm)

#### `public int DeleteGrips(IEnumerable<Guid> gripIds)`

Deletes one or more grip objects.

**Parameters:**
- `gripIds` (System.Collections.Generic.IEnumerable<Guid>) — The ids of the grip objects to delete.

**Returns:** `Int32` — The number of successfully deleted grip objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrips_3.htm)

#### `public int DeleteGrips(IEnumerable<ObjRef> gripRefs)`

Deletes one or more grip objects.

**Parameters:**
- `gripRefs` (System.Collections.Generic.IEnumerable<ObjRef>) — References to the grip objects to delete.

**Returns:** `Int32` — The number of successfully deleted grip objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrips_2.htm)

#### `public int DeleteGrips(RhinoObject owner, IEnumerable<int> gripIndices)`

Deletes one or more grip objects.

**Parameters:**
- `owner` (Rhino.DocObjects.RhinoObject) — The owner of the grip objects.
- `gripIndices` (System.Collections.Generic.IEnumerable<Int32>) — The indices of the grip objects to delete.

**Returns:** `Int32` — The number of successfully deleted grip objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_DeleteGrips.htm)

#### `public Guid Duplicate(Guid objectId)`

Same as TransformObject(objref, ON_Xform.Identity, false)

**Parameters:**
- `objectId` (System.Guid) — An ID to an object in the document that needs to be duplicated.

**Returns:** `Guid` — The new object ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Duplicate_2.htm)

#### `public Guid Duplicate(ObjRef objref)`

Duplicates the object that is referenced by objref. Same as Transform(objref, Transform.Identity, false)

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — A Rhino object reference to follow for object duplication.

**Returns:** `Guid` — The new object ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Duplicate.htm)

#### `public Guid Duplicate(RhinoObject obj)`

Duplicates the object that is referenced by obj. Same as TransformObject(obj, Transform.Identityy, false)

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — A Rhino object to duplicate.

**Returns:** `Guid` — The new object ID.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Duplicate_1.htm)

#### `public RhinoObject Find(uint runtimeSerialNumber)`

Use the object runtime serial number to find a rhino object in the document. This is the value stored on RhinoObject.RuntimeObjectSerialNumber. The RhinoObject constructor sets the runtime serial number and every instance of a RhinoObject class will have a unique serial number for the duration of the Rhino application. If an object is replaced with a new object, then the new object will have a different runtime serial number. Deleted objects stored in the undo list maintain their runtime serial numbers and this function will return pointers to these objects. Call RhinoObject.IsDeleted if you need to determine if the returned object is active or deleted. The runtime serial number is not saved in files.

**Parameters:**
- `runtimeSerialNumber` (System.UInt32) — Runtime serial number to search for.

**Returns:** `RhinoObject` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Find.htm)

#### `public RhinoObject[] FindByCrossingWindowRegion(RhinoViewport viewport, IEnumerable<Point3d> region, bool inside, ObjectType filter)`

Finds objects bounded by a polyline region

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — viewport to use for selection
- `region` (System.Collections.Generic.IEnumerable<Point3d>) — list of points that define the
- `inside` (System.Boolean) — should objects returned be the ones inside of this region (or outside)
- `filter` (Rhino.DocObjects.ObjectType) — filter down list by object type

**Returns:** `RhinoObject[]` — An array of RhinoObjects that are inside of this region

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByCrossingWindowRegion_1.htm)

#### `public RhinoObject[] FindByCrossingWindowRegion(RhinoViewport viewport, Point2d screen1, Point2d screen2, bool inside, ObjectType filter)`

Finds objects bounded by a region

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — viewport to use for selection
- `screen1` (Rhino.Geometry.Point2d) — first screen corner
- `screen2` (Rhino.Geometry.Point2d) — second screen corner
- `inside` (System.Boolean) — should objects returned be the ones inside of this region (or outside)
- `filter` (Rhino.DocObjects.ObjectType) — filter down list by object type

**Returns:** `RhinoObject[]` — An array of RhinoObjects that are inside of this region

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByCrossingWindowRegion.htm)

#### `public RhinoObject[] FindByDrawColor(Color drawColor, bool includeLights)`

Finds all objects whose draw color matches a given color.

**Parameters:**
- `drawColor` (System.Drawing.Color) — The alpha value of this color is ignored.
- `includeLights` (System.Boolean) — true if lights should be included.

**Returns:** `RhinoObject[]` — An array of Rhino document objects. This array can be empty.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByDrawColor.htm)

#### `public RhinoObject[] FindByFilter(ObjectEnumeratorSettings filter)`

Same as GetObjectList but converts the result to an array.

**Parameters:**
- `filter` (Rhino.DocObjects.ObjectEnumeratorSettings) — The ObjectEnumeratorSettings filter to customize inclusion requirements.

**Returns:** `RhinoObject[]` — A Rhino object array. This array can be empty but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByFilter.htm)

#### `public RhinoObject[] FindByGroup(int groupIndex)`

Finds all RhinoObjects that are in a given group.

**Parameters:**
- `groupIndex` (System.Int32) — Index of group to search for.

**Returns:** `RhinoObject[]` — An array of objects that belong to the specified group or null if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByGroup.htm)

#### `public RhinoObject[] FindByLayer(Layer layer)`

Finds all RhinoObjects that are in a given layer.

**Parameters:**
- `layer` (Rhino.DocObjects.Layer) — Layer to search.

**Returns:** `RhinoObject[]` — Array of objects that belong to the specified group or null if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByLayer.htm)

#### `public RhinoObject[] FindByLayer(string layerName)`

Finds all RhinoObjects that are in a given layer.

**Parameters:**
- `layerName` (System.String) — Name of layer to search.

**Returns:** `RhinoObject[]` — Array of objects that belong to the specified group or null if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByLayer_1.htm)

#### `public RhinoObject[] FindByObjectType(ObjectType typeFilter)`



**Parameters:**
- `typeFilter` (Rhino.DocObjects.ObjectType) — [Missing <param name="typeFilter"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.FindByObjectType(Rhino.DocObjects.ObjectType)"]

**Returns:** `RhinoObject[]` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.FindByObjectType(Rhino.DocObjects.ObjectType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByObjectType.htm)

#### `public RhinoObject[] FindByUserString(string key, string value, bool caseSensitive)`

Finds all objects whose UserString matches the search patterns.

**Parameters:**
- `key` (System.String) — Search pattern for UserString keys (supported wildcards are: ? = any single character, * = any sequence of characters).
- `value` (System.String) — Search pattern for UserString values (supported wildcards are: ? = any single character, * = any sequence of characters).
- `caseSensitive` (System.Boolean) — If true, string comparison will be case sensitive.

**Returns:** `RhinoObject[]` — An array of all objects whose UserString matches with the search patterns or null when no such objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByUserString.htm)

#### `public RhinoObject[] FindByUserString(string key, string value, bool caseSensitive, bool searchGeometry, bool searchAttributes, ObjectEnumeratorSettings filter)`

Finds all objects whose UserString matches the search patterns.

**Parameters:**
- `key` (System.String) — Search pattern for UserString keys (supported wildcards are: ? = any single character, * = any sequence of characters).
- `value` (System.String) — Search pattern for UserString values (supported wildcards are: ? = any single character, * = any sequence of characters).
- `caseSensitive` (System.Boolean) — If true, string comparison will be case sensitive.
- `searchGeometry` (System.Boolean) — If true, UserStrings attached to the geometry of an object will be searched.
- `searchAttributes` (System.Boolean) — If true, UserStrings attached to the attributes of an object will be searched.
- `filter` (Rhino.DocObjects.ObjectEnumeratorSettings) — ObjectEnumeratorSettings filter used to restrict the number of objects searched.

**Returns:** `RhinoObject[]` — An array of all objects whose UserString matches with the search patterns or null when no such objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByUserString_1.htm)

#### `public RhinoObject[] FindByUserString(string key, string value, bool caseSensitive, bool searchGeometry, bool searchAttributes, ObjectType filter)`

Finds all objects whose UserString matches the search patterns.

**Parameters:**
- `key` (System.String) — Search pattern for UserString keys (supported wildcards are: ? = any single character, * = any sequence of characters).
- `value` (System.String) — Search pattern for UserString values (supported wildcards are: ? = any single character, * = any sequence of characters).
- `caseSensitive` (System.Boolean) — If true, string comparison will be case sensitive.
- `searchGeometry` (System.Boolean) — If true, UserStrings attached to the geometry of an object will be searched.
- `searchAttributes` (System.Boolean) — If true, UserStrings attached to the attributes of an object will be searched.
- `filter` (Rhino.DocObjects.ObjectType) — Object type filter.

**Returns:** `RhinoObject[]` — An array of all objects whose UserString matches with the search patterns or null when no such objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByUserString_2.htm)

#### `public RhinoObject[] FindByWindowRegion(RhinoViewport viewport, IEnumerable<Point3d> region, bool inside, ObjectType filter)`

Finds objects bounded by a polyline region

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — viewport to use for selection
- `region` (System.Collections.Generic.IEnumerable<Point3d>) — list of points that define the
- `inside` (System.Boolean) — should objects returned be the ones inside of this region (or outside)
- `filter` (Rhino.DocObjects.ObjectType) — filter down list by object type

**Returns:** `RhinoObject[]` — An array of RhinoObjects that are inside of this region

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByWindowRegion_1.htm)

#### `public RhinoObject[] FindByWindowRegion(RhinoViewport viewport, Point2d screen1, Point2d screen2, bool inside, ObjectType filter)`

Finds objects bounded by a polyline region

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — viewport to use for selection
- `screen1` (Rhino.Geometry.Point2d) — first screen corner
- `screen2` (Rhino.Geometry.Point2d) — second screen corner
- `inside` (System.Boolean) — should objects returned be the ones inside of this region (or outside)
- `filter` (Rhino.DocObjects.ObjectType) — filter down list by object type

**Returns:** `RhinoObject[]` — An array of RhinoObjects that are inside of this region

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindByWindowRegion.htm)

#### `public ClippingPlaneObject[] FindClippingPlanesForViewport(RhinoViewport viewport)`

Finds all of the clipping plane objects that actively clip a viewport.

**Parameters:**
- `viewport` (Rhino.Display.RhinoViewport) — The viewport in which clipping planes are searched.

**Returns:** `ClippingPlaneObject[]` — An array of clipping plane objects. The array can be empty but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindClippingPlanesForViewport.htm)

#### `public GeometryBase FindGeometry(Guid id)`

Same as FindId, but returns the Geometry property directly, if it exists.

**Parameters:**
- `id` (System.Guid) — ID of object to search for.

**Returns:** `GeometryBase` — Reference to the geometry in the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindGeometry.htm)

#### `public override RhinoObject FindId(Guid id)`

Uses the object guid to find a rhino object. Deleted objects cannot be found by id. The guid is the value that is stored on RhinoObject.Id In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits it's guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of object to search for.

**Returns:** `RhinoObject` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_FindId.htm)

#### `public override IEnumerator<RhinoObject> GetEnumerator()`

(Overrides CommonComponentTable<T>.GetEnumerator().)

**Returns:** `IEnumerator<RhinoObject>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetEnumerator.htm)

#### `public IEnumerable<RhinoObject> GetObjectList(ObjectEnumeratorSettings settings)`

Returns an enumerable based on a ObjectEnumeratorSettings filter.

**Parameters:**
- `settings` (Rhino.DocObjects.ObjectEnumeratorSettings) — The ObjectEnumeratorSettings settings.

**Returns:** `IEnumerable<RhinoObject>` — The enumerable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetObjectList.htm)

#### `public IEnumerable<RhinoObject> GetObjectList(ObjectType typeFilter)`



**Parameters:**
- `typeFilter` (Rhino.DocObjects.ObjectType) — [Missing <param name="typeFilter"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetObjectList(Rhino.DocObjects.ObjectType)"]

**Returns:** `IEnumerable<RhinoObject>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetObjectList(Rhino.DocObjects.ObjectType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetObjectList_1.htm)

#### `public IEnumerable<RhinoObject> GetObjectList(Type typeFilter)`



**Parameters:**
- `typeFilter` (System.Type) — [Missing <param name="typeFilter"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetObjectList(System.Type)"]

**Returns:** `IEnumerable<RhinoObject>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetObjectList(System.Type)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetObjectList_2.htm)

#### `public IEnumerable<T> GetObjectsByType<T>() where T : RhinoObject`

Returns Rhino object by type.

**Returns:** `IEnumerable<T>` — The enumerable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetObjectsByType__1.htm)

#### `public IEnumerable<T> GetObjectsByType<T>(ObjectEnumeratorSettings settings) where T : RhinoObject`

Returns Rhino object by type.

**Parameters:**
- `settings` (Rhino.DocObjects.ObjectEnumeratorSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetObjectsByType``1(Rhino.DocObjects.ObjectEnumeratorSettings)"]

**Returns:** `IEnumerable<T>` — The enumerable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetObjectsByType__1_1.htm)

#### `public ObjectType GetSelectedObjectTypes()`



**Returns:** `ObjectType` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetSelectedObjectTypes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetSelectedObjectTypes.htm)

#### `public IEnumerable<RhinoObject> GetSelectedObjects(bool includeLights, bool includeGrips)`



**Parameters:**
- `includeLights` (System.Boolean) — [Missing <param name="includeLights"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetSelectedObjects(System.Boolean,System.Boolean)"]
- `includeGrips` (System.Boolean) — [Missing <param name="includeGrips"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetSelectedObjects(System.Boolean,System.Boolean)"]

**Returns:** `IEnumerable<RhinoObject>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.GetSelectedObjects(System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GetSelectedObjects.htm)

#### `public RhinoObject GripUpdate(RhinoObject obj, bool deleteOriginal)`

Altered grip positions on a RhinoObject are used to calculate an updated object that is added to the document.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — object with modified grips to update.
- `deleteOriginal` (System.Boolean) — if true, obj is deleted from the document.

**Returns:** `RhinoObject` — new RhinoObject on success; otherwise null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_GripUpdate.htm)

#### `public bool Hide(Guid objectId, bool ignoreLayerMode)`

If Object().IsNormal() is true, then the object will be hidden.

**Parameters:**
- `objectId` (System.Guid) — Id of object to hide.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be hidden even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Hide_2.htm)

#### `public bool Hide(ObjRef objref, bool ignoreLayerMode)`

If objref.Object().IsNormal() is true, then the object will be hidden.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to object to hide.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be hidden even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Hide.htm)

#### `public bool Hide(RhinoObject obj, bool ignoreLayerMode)`

If obj.IsNormal() is true, then the object will be hidden.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — object to hide.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be hidden even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Hide_1.htm)

#### `public bool Lock(Guid objectId, bool ignoreLayerMode)`

If objref.Object().IsNormal() is true, then the object will be locked.

**Parameters:**
- `objectId` (System.Guid) — Id of normal object to lock.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be locked even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully locked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Lock_2.htm)

#### `public bool Lock(ObjRef objref, bool ignoreLayerMode)`

If objref.Object().IsNormal() is true, then the object will be locked.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to normal object to lock.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be locked even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully locked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Lock.htm)

#### `public bool Lock(RhinoObject obj, bool ignoreLayerMode)`

If obj.IsNormal() is true, then the object will be locked.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — normal object to lock.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be locked even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully locked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Lock_1.htm)

#### `public bool ModifyAttributes(Guid objectId, ObjectAttributes newAttributes, bool quiet)`

Modifies an object's attributes. Cannot be used to change object id.

**Parameters:**
- `objectId` (System.Guid) — Id of object to modify.
- `newAttributes` (Rhino.DocObjects.ObjectAttributes) — new attributes.
- `quiet` (System.Boolean) — if true, then warning message boxes are disabled.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyAttributes_2.htm)

#### `public bool ModifyAttributes(ObjRef objref, ObjectAttributes newAttributes, bool quiet)`

Modifies an object's attributes. Cannot be used to change object id.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to object to modify.
- `newAttributes` (Rhino.DocObjects.ObjectAttributes) — new attributes.
- `quiet` (System.Boolean) — if true, then warning message boxes are disabled.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyAttributes.htm)

#### `public bool ModifyAttributes(RhinoObject obj, ObjectAttributes newAttributes, bool quiet)`

Modifies an object's attributes. Cannot be used to change object id.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — object to modify.
- `newAttributes` (Rhino.DocObjects.ObjectAttributes) — new attributes.
- `quiet` (System.Boolean) — if true, then warning message boxes are disabled.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyAttributes_1.htm)

#### `public bool ModifyRenderMaterial(Guid objectId, RenderMaterial material)`

Modifies an object's render material assignment, this will set the objects material source to ObjectMaterialSource.MaterialFromObject.

**Parameters:**
- `objectId` (System.Guid) — Id of object to modify.
- `material` (Rhino.Render.RenderMaterial) — Material to assign to this object.

**Returns:** `Boolean` — Returns true on success otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyRenderMaterial_2.htm)

#### `public bool ModifyRenderMaterial(ObjRef objRef, RenderMaterial material)`

Modifies an object's render material assignment, this will set the objects material source to ObjectMaterialSource.MaterialFromObject.

**Parameters:**
- `objRef` (Rhino.DocObjects.ObjRef) — Object to modify.
- `material` (Rhino.Render.RenderMaterial) — Material to assign to this object.

**Returns:** `Boolean` — Returns true on success otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyRenderMaterial.htm)

#### `public bool ModifyRenderMaterial(RhinoObject obj, RenderMaterial material)`

Modifies an object's render material assignment, this will set the objects material source to ObjectMaterialSource.MaterialFromObject.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — Object to modify.
- `material` (Rhino.Render.RenderMaterial) — Material to assign to this object.

**Returns:** `Boolean` — Returns true on success otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyRenderMaterial_1.htm)

#### `public bool ModifyTextureMapping(Guid objId, int channel, TextureMapping mapping)`



**Parameters:**
- `objId` (System.Guid) — [Missing <param name="objId"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(System.Guid,System.Int32,Rhino.Render.TextureMapping)"]
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(System.Guid,System.Int32,Rhino.Render.TextureMapping)"]
- `mapping` (Rhino.Render.TextureMapping) — [Missing <param name="mapping"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(System.Guid,System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(System.Guid,System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyTextureMapping_2.htm)

#### `public bool ModifyTextureMapping(ObjRef objRef, int channel, TextureMapping mapping)`



**Parameters:**
- `objRef` (Rhino.DocObjects.ObjRef) — [Missing <param name="objRef"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.ObjRef,System.Int32,Rhino.Render.TextureMapping)"]
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.ObjRef,System.Int32,Rhino.Render.TextureMapping)"]
- `mapping` (Rhino.Render.TextureMapping) — [Missing <param name="mapping"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.ObjRef,System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.ObjRef,System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyTextureMapping.htm)

#### `public bool ModifyTextureMapping(RhinoObject obj, int channel, TextureMapping mapping)`



**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="obj"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.RhinoObject,System.Int32,Rhino.Render.TextureMapping)"]
- `channel` (System.Int32) — [Missing <param name="channel"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.RhinoObject,System.Int32,Rhino.Render.TextureMapping)"]
- `mapping` (Rhino.Render.TextureMapping) — [Missing <param name="mapping"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.RhinoObject,System.Int32,Rhino.Render.TextureMapping)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.ModifyTextureMapping(Rhino.DocObjects.RhinoObject,System.Int32,Rhino.Render.TextureMapping)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ModifyTextureMapping_1.htm)

#### `public RhinoObject MostRecentObject()`

Gets the most recently added object that is still in the Document.

**Returns:** `RhinoObject` — The most recent (non-deleted) object in the document, or null if no such object exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_MostRecentObject.htm)

#### `public int ObjectCount(ObjectEnumeratorSettings filter)`

Returns the number objects that pass a filter.

**Parameters:**
- `filter` (Rhino.DocObjects.ObjectEnumeratorSettings) — The ObjectEnumeratorSettings filter.

**Returns:** `Int32` — The number of objects that pass the filter.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ObjectCount.htm)

#### `public ObjRef[] PickObjects(PickContext pickContext)`

Pick one or more objects based on a given pick context

**Parameters:**
- `pickContext` (Rhino.Input.Custom.PickContext) — settings used to define what is picked

**Returns:** `ObjRef[]` — zero or more objects

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_PickObjects.htm)

#### `public bool Purge(RhinoObject rhinoObject)`

Removes object from document and deletes the pointer. Typically you will want to call Delete instead in order to keep the object on the undo list.

**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — A Rhino object that will be deleted.

**Returns:** `Boolean` — true if the object was purged; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Purge.htm)

#### `public bool Purge(uint runtimeSerialNumber)`

Removes object from document and deletes the pointer. Typically you will want to call Delete instead in order to keep the object on the undo list.

**Parameters:**
- `runtimeSerialNumber` (System.UInt32) — A runtime serial number of the object that will be deleted.

**Returns:** `Boolean` — true if the object was purged; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Purge_1.htm)

#### `public bool Replace(Guid objectId, Arc arc)`

Replaces one object with new curve object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `arc` (Rhino.Geometry.Arc) — new arc to be added. The arc is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_20.htm)

#### `public bool Replace(Guid objectId, Brep brep)`

Replaces one object with new brep object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `brep` (Rhino.Geometry.Brep) — new surface to be added A duplicate of the brep is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_21.htm)

#### `public bool Replace(Guid objectId, Brep brep, bool splitKinkySurfaces)`



**Parameters:**
- `objectId` (System.Guid) — [Missing <param name="objectId"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(System.Guid,Rhino.Geometry.Brep,System.Boolean)"]
- `brep` (Rhino.Geometry.Brep) — [Missing <param name="brep"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(System.Guid,Rhino.Geometry.Brep,System.Boolean)"]
- `splitKinkySurfaces` (System.Boolean) — [Missing <param name="splitKinkySurfaces"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(System.Guid,Rhino.Geometry.Brep,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(System.Guid,Rhino.Geometry.Brep,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_22.htm)

#### `public bool Replace(Guid objectId, Circle circle)`

Replaces one object with new curve object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `circle` (Rhino.Geometry.Circle) — new circle to be added. The circle is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_23.htm)

#### `public bool Replace(Guid objectId, Curve curve)`

Replaces one object with new curve object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `curve` (Rhino.Geometry.Curve) — New curve to be added. A duplicate of the curve is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_24.htm)

#### `public bool Replace(Guid objectId, Extrusion extrusion)`

Replaces one object with new extrusion object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `extrusion` (Rhino.Geometry.Extrusion) — New extrusion to be added. A duplicate of the extrusion is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_25.htm)

#### `public bool Replace(Guid objectId, GeometryBase geometry, bool ignoreModes)`

Replaces the geometry in one object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `geometry` (Rhino.Geometry.GeometryBase) — [Missing <param name="geometry"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(System.Guid,Rhino.Geometry.GeometryBase,System.Boolean)"]
- `ignoreModes` (System.Boolean) — [Missing <param name="ignoreModes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(System.Guid,Rhino.Geometry.GeometryBase,System.Boolean)"]

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_26.htm)

#### `public bool Replace(Guid objectId, Hatch hatch)`

Replaces one object with new hatch object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `hatch` (Rhino.Geometry.Hatch) — new hatch to be added. The hatch is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_27.htm)

#### `public bool Replace(Guid objectId, Leader leader)`

Replaces one object with new text object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `leader` (Rhino.Geometry.Leader) — new leader to be added. The leader is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_28.htm)

#### `public bool Replace(Guid objectId, Line line)`

Replaces one object with new line curve object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `line` (Rhino.Geometry.Line) — new line to be added. The line is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_29.htm)

#### `public bool Replace(Guid objectId, Mesh mesh)`

Replaces one object with new mesh object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `mesh` (Rhino.Geometry.Mesh) — new mesh to be added A duplicate of the mesh is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_30.htm)

#### `public bool Replace(Guid objectId, Point point)`

Replaces one object with new point object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `point` (Rhino.Geometry.Point) — new point to be added. The point is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_31.htm)

#### `public bool Replace(Guid objectId, Point3d point)`

Replaces one object with new point object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `point` (Rhino.Geometry.Point3d) — new point to be added. The point is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_32.htm)

#### `public bool Replace(Guid objectId, PointCloud pointcloud)`

Replaces one object with new point cloud object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `pointcloud` (Rhino.Geometry.PointCloud) — new point cloud to be added A duplicate of the point cloud is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_33.htm)

#### `public bool Replace(Guid objectId, Polyline polyline)`

Replaces one object with new curve object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `polyline` (Rhino.Geometry.Polyline) — new polyline to be added. The polyline is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_34.htm)

#### `public bool Replace(Guid objectId, SubD subD)`

Replaces one object with new subd object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `subD` (Rhino.Geometry.SubD) — new mesh to be added A duplicate of the mesh is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_35.htm)

#### `public bool Replace(Guid objectId, Surface surface)`

Replaces one object with new surface object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `surface` (Rhino.Geometry.Surface) — new surface to be added A duplicate of the surface is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_36.htm)

#### `public bool Replace(Guid objectId, TextDot dot)`

Replaces one object with new text dot object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `dot` (Rhino.Geometry.TextDot) — new text dot to be added. The text dot is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_37.htm)

#### `public bool Replace(Guid objectId, TextEntity text)`

Replaces one object with new text object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to be replaced.
- `text` (Rhino.Geometry.TextEntity) — new text to be added. The text is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_38.htm)

#### `public bool Replace(ObjRef objref, Arc arc)`

Replaces one object with new curve object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `arc` (Rhino.Geometry.Arc) — new arc to be added. The arc is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_1.htm)

#### `public bool Replace(ObjRef objref, Brep brep)`

Replaces one object with new brep object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `brep` (Rhino.Geometry.Brep) — new brep to be added A duplicate of the brep is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_2.htm)

#### `public bool Replace(ObjRef objref, Brep brep, bool splitKinkySurfaces)`



**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — [Missing <param name="objref"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(Rhino.DocObjects.ObjRef,Rhino.Geometry.Brep,System.Boolean)"]
- `brep` (Rhino.Geometry.Brep) — [Missing <param name="brep"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(Rhino.DocObjects.ObjRef,Rhino.Geometry.Brep,System.Boolean)"]
- `splitKinkySurfaces` (System.Boolean) — [Missing <param name="splitKinkySurfaces"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(Rhino.DocObjects.ObjRef,Rhino.Geometry.Brep,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(Rhino.DocObjects.ObjRef,Rhino.Geometry.Brep,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_3.htm)

#### `public bool Replace(ObjRef objref, Circle circle)`

Replaces one object with new curve object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `circle` (Rhino.Geometry.Circle) — new circle to be added. The circle is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_4.htm)

#### `public bool Replace(ObjRef objref, Curve curve)`

Replaces one object with new curve object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `curve` (Rhino.Geometry.Curve) — New curve to be added. A duplicate of the curve is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_5.htm)

#### `public bool Replace(ObjRef objref, Extrusion extrusion)`

Replaces one object with new extrusion object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `extrusion` (Rhino.Geometry.Extrusion) — New extrusion to be added. A duplicate of the extrusion is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_6.htm)

#### `public bool Replace(ObjRef objref, GeometryBase geometry, bool ignoreModes)`

Replaces the geometry in one object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `geometry` (Rhino.Geometry.GeometryBase) — [Missing <param name="geometry"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(Rhino.DocObjects.ObjRef,Rhino.Geometry.GeometryBase,System.Boolean)"]
- `ignoreModes` (System.Boolean) — [Missing <param name="ignoreModes"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Replace(Rhino.DocObjects.ObjRef,Rhino.Geometry.GeometryBase,System.Boolean)"]

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_7.htm)

#### `public bool Replace(ObjRef objref, Hatch hatch)`

Replaces one object with new hatch object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `hatch` (Rhino.Geometry.Hatch) — new hatch to be added. The hatch is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_8.htm)

#### `public bool Replace(ObjRef objref, Leader leader)`

Replaces one object with new text object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `leader` (Rhino.Geometry.Leader) — new leader to be added. The leader is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_9.htm)

#### `public bool Replace(ObjRef objref, Line line)`

Replaces one object with new line curve object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `line` (Rhino.Geometry.Line) — new line to be added. The line is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_10.htm)

#### `public bool Replace(ObjRef objref, Mesh mesh)`

Replaces one object with new mesh object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `mesh` (Rhino.Geometry.Mesh) — new mesh to be added A duplicate of the mesh is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_11.htm)

#### `public bool Replace(ObjRef objref, Point point)`

Replaces one object with new point object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `point` (Rhino.Geometry.Point) — new point to be added. The point is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_12.htm)

#### `public bool Replace(ObjRef objref, Point3d point)`

Replaces one object with new point object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `point` (Rhino.Geometry.Point3d) — new point to be added. The point is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_13.htm)

#### `public bool Replace(ObjRef objref, PointCloud pointcloud)`

Replaces one object with new point cloud object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `pointcloud` (Rhino.Geometry.PointCloud) — new point cloud to be added A duplicate of the point cloud is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_14.htm)

#### `public bool Replace(ObjRef objref, Polyline polyline)`

Replaces one object with new curve object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `polyline` (Rhino.Geometry.Polyline) — new polyline to be added. The polyline is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_15.htm)

#### `public bool Replace(ObjRef objref, RhinoObject newObject)`

Replaces one object with another. Conceptually, this function is the same as calling Setting new_object attributes = old_object attributes DeleteObject(old_object); AddObject(old_object);

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `newObject` (Rhino.DocObjects.RhinoObject) — new replacement object - must not be in document.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace.htm)

#### `public bool Replace(ObjRef objref, SubD subD)`

Replaces one object with a new SubD object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `subD` (Rhino.Geometry.SubD) — new SubD to be added A duplicate of the SubD is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_16.htm)

#### `public bool Replace(ObjRef objref, Surface surface)`

Replaces one object with new surface object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to old object to be replaced. The objref.Object() will be deleted.
- `surface` (Rhino.Geometry.Surface) — new surface to be added A duplicate of the surface is added to the Rhino model.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_17.htm)

#### `public bool Replace(ObjRef objref, TextDot dot)`

Replaces one object with new text dot object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `dot` (Rhino.Geometry.TextDot) — new text dot to be added. The text dot is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_18.htm)

#### `public bool Replace(ObjRef objref, TextEntity text)`

Replaces one object with new text object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to old object to be replaced. The object objref.Object() will be deleted.
- `text` (Rhino.Geometry.TextEntity) — new text to be added. The text is copied.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Replace_19.htm)

#### `public bool ReplaceInstanceObject(Guid objectId, int instanceDefinitionIndex)`

Replaces the underlying instance definition of an instance object.

**Parameters:**
- `objectId` (System.Guid) — Id of the instance object to be replaced.
- `instanceDefinitionIndex` (System.Int32) — The index of the new instance definition to use.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ReplaceInstanceObject_1.htm)

#### `public bool ReplaceInstanceObject(ObjRef objref, int instanceDefinitionIndex)`

Replaces the underlying instance definition of an instance object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Reference to the instance object to be replaced. The objref.Object() will be deleted.
- `instanceDefinitionIndex` (System.Int32) — The index of the new instance definition to use.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_ReplaceInstanceObject.htm)

#### `public bool Select(Guid objectId)`

Select a single object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to select.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_9.htm)

#### `public bool Select(Guid objectId, bool select)`

Select or deselects a single object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to select.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_10.htm)

#### `public bool Select(Guid objectId, bool select, bool syncHighlight)`

Select or deselects a single object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to select.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_11.htm)

#### `public bool Select(Guid objectId, bool select, bool syncHighlight, bool persistentSelect)`

Select or deselects a single object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to select.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_12.htm)

#### `public bool Select(Guid objectId, bool select, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Select or deselects a single object.

**Parameters:**
- `objectId` (System.Guid) — Id of object to select.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_13.htm)

#### `public int Select(IEnumerable<Guid> objectIds)`

Selects a collection of objects.

**Parameters:**
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — Ids of objects to select.

**Returns:** `Int32` — Number of objects successfully selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_7.htm)

#### `public int Select(IEnumerable<Guid> objectIds, bool select)`

Selects or deselects a collection of objects.

**Parameters:**
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — Ids of objects to select or deselect.
- `select` (System.Boolean) — If true, objects will be selected. If false, objects will be deselected.

**Returns:** `Int32` — Number of objects successfully selected or deselected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_8.htm)

#### `public int Select(IEnumerable<ObjRef> objRefs)`

Selects a collection of objects.

**Parameters:**
- `objRefs` (System.Collections.Generic.IEnumerable<ObjRef>) — References to objects to select.

**Returns:** `Int32` — Number of objects successfully selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_5.htm)

#### `public int Select(IEnumerable<ObjRef> objRefs, bool select)`

Selects or deselects a collection of objects.

**Parameters:**
- `objRefs` (System.Collections.Generic.IEnumerable<ObjRef>) — References to objects to select or deselect.
- `select` (System.Boolean) — If true, objects will be selected. If false, objects will be deselected.

**Returns:** `Int32` — Number of objects successfully selected or deselected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_6.htm)

#### `public bool Select(ObjRef objref)`

Select a single object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Object represented by this ObjRef is selected.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select.htm)

#### `public bool Select(ObjRef objref, bool select)`

Select or deselects a single object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Object represented by this ObjRef is selected.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_1.htm)

#### `public bool Select(ObjRef objref, bool select, bool syncHighlight)`

Select or deselects a single object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Object represented by this ObjRef is selected.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_2.htm)

#### `public bool Select(ObjRef objref, bool select, bool syncHighlight, bool persistentSelect)`

Select or deselects a single object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Object represented by this ObjRef is selected.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_3.htm)

#### `public bool Select(ObjRef objref, bool select, bool syncHighlight, bool persistentSelect, bool ignoreGripsState, bool ignoreLayerLocking, bool ignoreLayerVisibility)`

Select or deselects a single object.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — Object represented by this ObjRef is selected.
- `select` (System.Boolean) — If true, the object will be selected, if false, it will be deselected.
- `syncHighlight` (System.Boolean) — If true, then the object is highlighted if it is selected and unhighlighted if is not selected.
- `persistentSelect` (System.Boolean) — Objects that are persistently selected stay selected when a command terminates.
- `ignoreGripsState` (System.Boolean) — If true, then objects with grips on can be selected. If false, then the value returned by the object's IsSelectableWithGripsOn() function decides if the object can be selected when it has grips turned on.
- `ignoreLayerLocking` (System.Boolean) — If true, then objects on locked layers can be selected.
- `ignoreLayerVisibility` (System.Boolean) — If true, then objects on hidden layers can be selectable.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Select_4.htm)

#### `public bool Show(Guid objectId, bool ignoreLayerMode)`

If Object().IsHidden() is true, then the object will be returned to normal (visible and selectable) mode.

**Parameters:**
- `objectId` (System.Guid) — Id of the normal object to show.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be shown even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully shown.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Show_2.htm)

#### `public bool Show(ObjRef objref, bool ignoreLayerMode)`

If objref.Object().IsHidden() is true, then the object will be returned to normal (visible and selectable) mode.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to normal object to show.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be shown even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully shown.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Show.htm)

#### `public bool Show(RhinoObject obj, bool ignoreLayerMode)`

If obj.IsHidden() is true, then the object will be returned to normal (visible and selectable) mode.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — the normal object to show.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be shown even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully shown.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Show_1.htm)

#### `public Guid Transform(Guid objectId, Transform xform, bool deleteOriginal)`

Constructs a new object that is the transformation of the existing object and deletes the existing object if deleteOriginal is true.

**Remarks:** If the object is locked or on a locked layer, then it cannot be transformed.

**Parameters:**
- `objectId` (System.Guid) — Id of rhino object to transform. This object will be deleted if deleteOriginal is true.
- `xform` (Rhino.Geometry.Transform) — transformation to apply.
- `deleteOriginal` (System.Boolean) — if true, the original object is deleted if false, the original object is not deleted.

**Returns:** `Guid` — Id of the new object that is the transformation of the existing_object. The new object has identical attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Transform_2.htm)

#### `public Guid Transform(ObjRef objref, Transform xform, bool deleteOriginal)`

Constructs a new object that is the transformation of the existing object and deletes the existing object if deleteOriginal is true.

**Remarks:** If the object is locked or on a locked layer, then it cannot be transformed.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to object to transform. The objref.Object() will be deleted if deleteOriginal is true.
- `xform` (Rhino.Geometry.Transform) — transformation to apply.
- `deleteOriginal` (System.Boolean) — if true, the original object is deleted if false, the original object is not deleted.

**Returns:** `Guid` — Id of the new object that is the transformation of the existing_object. The new object has identical attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Transform.htm)

#### `public Guid Transform(RhinoObject obj, Transform xform, bool deleteOriginal)`

Constructs a new object that is the transformation of the existing object and deletes the existing object if deleteOriginal is true.

**Remarks:** If the object is locked or on a locked layer, then it cannot be transformed.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — Rhino object to transform. This object will be deleted if deleteOriginal is true.
- `xform` (Rhino.Geometry.Transform) — transformation to apply.
- `deleteOriginal` (System.Boolean) — if true, the original object is deleted if false, the original object is not deleted.

**Returns:** `Guid` — Id of the new object that is the transformation of the existing_object. The new object has identical attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Transform_1.htm)

#### `public Guid TransformWithHistory(Guid objectId, Transform xform)`

Constructs a new object that is the transformation of the existing object and records history of the transformation if history recording is turned on. If history recording is not enabled, this function will act the same as Transform(objectId, xform, false)

**Remarks:** If the object is locked or on a locked layer, then it cannot be transformed.

**Parameters:**
- `objectId` (System.Guid) — Id of rhino object to transform.
- `xform` (Rhino.Geometry.Transform) — transformation to apply.

**Returns:** `Guid` — Id of the new object that is the transformation of the existing_object. The new object has identical attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_TransformWithHistory_2.htm)

#### `public Guid TransformWithHistory(ObjRef objref, Transform xform)`

Constructs a new object that is the transformation of the existing object and records history of the transformation if history recording is turned on. If history recording is not enabled, this function will act the same as Transform(objref, xform, false)

**Remarks:** If the object is locked or on a locked layer, then it cannot be transformed.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to object to transform.
- `xform` (Rhino.Geometry.Transform) — transformation to apply.

**Returns:** `Guid` — Id of the new object that is the transformation of the existing_object. The new object has identical attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_TransformWithHistory.htm)

#### `public Guid TransformWithHistory(RhinoObject obj, Transform xform)`

Constructs a new object that is the transformation of the existing object and records history of the transformation if history recording is turned on. If history recording is not enabled, this function will act the same as Transform(obj, xform, false)

**Remarks:** If the object is locked or on a locked layer, then it cannot be transformed.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — Rhino object to transform.
- `xform` (Rhino.Geometry.Transform) — transformation to apply.

**Returns:** `Guid` — Id of the new object that is the transformation of the existing_object. The new object has identical attributes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_TransformWithHistory_1.htm)

#### `public bool TryFindPoint(Guid id, out Point3d point)`

Finds the location of a point, if a point exists in the document.

**Parameters:**
- `id` (System.Guid) — ID of point object to search for.
- `point` (Rhino.Geometry.Point3d) — The point will be passed here.

**Returns:** `Boolean` — true on success; false if point was not found, id represented another geometry type, or on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_TryFindPoint.htm)

#### `public bool Undelete(RhinoObject rhinoObject)`



**Parameters:**
- `rhinoObject` (Rhino.DocObjects.RhinoObject) — [Missing <param name="rhinoObject"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Undelete(Rhino.DocObjects.RhinoObject)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Undelete(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Undelete.htm)

#### `public bool Undelete(uint runtimeSerialNumber)`



**Parameters:**
- `runtimeSerialNumber` (System.UInt32) — [Missing <param name="runtimeSerialNumber"/> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Undelete(System.UInt32)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ObjectTable.Undelete(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Undelete_1.htm)

#### `public bool Unlock(Guid objectId, bool ignoreLayerMode)`

If Object().IsLocked() is true, then the object will be returned to normal (visible and selectable) mode.

**Parameters:**
- `objectId` (System.Guid) — Id of locked object to unlock.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be unlocked even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully unlocked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Unlock_2.htm)

#### `public bool Unlock(ObjRef objref, bool ignoreLayerMode)`

If objref.Object().IsLocked() is true, then the object will be returned to normal (visible and selectable) mode.

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — reference to locked object to unlock.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be unlocked even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully unlocked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Unlock.htm)

#### `public bool Unlock(RhinoObject obj, bool ignoreLayerMode)`

If obj.IsLocked() is true, then the object will be returned to normal (visible and selectable) mode.

**Parameters:**
- `obj` (Rhino.DocObjects.RhinoObject) — locked object to unlock.
- `ignoreLayerMode` (System.Boolean) — if true, the object will be unlocked even if it is on a layer that is locked or off.

**Returns:** `Boolean` — true if the object was successfully unlocked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_Unlock_1.htm)

#### `public int UnselectAll()`

Unselect objects.

**Returns:** `Int32` — Number of object that were unselected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_UnselectAll.htm)

#### `public int UnselectAll(bool ignorePersistentSelections)`

Unselect objects.

**Parameters:**
- `ignorePersistentSelections` (System.Boolean) — if true, then objects that are persistently selected will not be unselected.

**Returns:** `Int32` — Number of object that were unselected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ObjectTable_UnselectAll_1.htm)

### Properties
- `BoundingBox` (BoundingBox, get) — Gets the bounding box for all objects (normal, locked and hidden) in this document that exist in "model" space. This bounding box does not include objects that exist in layout space.
- `BoundingBoxVisible` (BoundingBox, get) — Gets the bounding box for all visible objects (normal and locked) in this document that exist in "model" space. This bounding box does not include hidden objects or any objects that exist in layout space.
- `ComponentType` (ModelComponentType, get) — (Overrides CommonComponentTable<T>.ComponentType.)
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `Document` (RhinoDoc, get) — Gets the document that owns this object table.
- `HistoryRecordCount` (Int32, get) — Returns the amount of history records in this document.

## RestoreLayerProperties (enum)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.RestoreLayerProperties"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_RestoreLayerProperties.htm)

### Values
- `None` = `0` — Restore nothing
- `Current` = `1` — Restore current layer
- `Visible` = `2` — Restore layer visibility
- `Locked` = `4` — Restore layer locked status
- `Color` = `8` — Restore layer color
- `Linetype` = `16` — Restore layer linetype
- `PrintColor` = `32` — Restore layer print color
- `PrintWidth` = `64` — Restore layer print width
- `ViewportVisible` = `128` — Restore per-viewport layer visibility
- `ViewportColor` = `256` — Restore per-viewport layer color
- `ViewportPrintColor` = `512` — Restore per-viewport layer print color
- `ViewportPrintWidth` = `1024` — Restore per-viewport layer print width
- `RenderMaterial` = `2048` — Restore render material
- `Unused` = `4096` — Unused flag
- `All` = `4294967295` — Restore all layer properties

## RhinoDocCommonTable<T> (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.RhinoDocCommonTable`1"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_RhinoDocCommonTable_1.htm)

### Methods
#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public abstract bool Delete(T item)`

Deletes an item. Items that are deleted are still keeping their space, but the 'IsDeleted' flag is checked.

**Parameters:**
- `item` (T) — An item to delete.

**Returns:** `Boolean` — True if an items could be deleted (e.g., it was not locked).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Delete.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns the actual component type of a table.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.
- `Document` (RhinoDoc, get) — Document that owns this table.

## RuntimeDocumentDataTable (class)

Collection of document runtime data. This is a good place to put non-serialized, per document data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_RuntimeDocumentDataTable.htm)

### Methods
#### `public T GetValue<T>(Object key, Func<RhinoDoc, T> newT) where T : class`

Checks the dictionary for the specified key, if found and the value is not null then the value is returned. If the key is not found or its value is null then newT(Document) is called to create a new value instance which is put in the dictionary and returned.

**Parameters:**
- `key` (System.Object) — Key to search for.
- `newT` (System.Func<RhinoDoc,T>) — Function called to create new value

**Returns:** `T` — Returns the document specific instance of type T using the specified dictionary key.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_RuntimeDocumentDataTable_GetValue__1.htm)

#### `public T TryGetValue<T>(Object key) where T : class`

Check dictionary for value and return it properly cast if found.

**Parameters:**
- `key` (System.Object) — Key to search for.

**Returns:** `T` — Returns the document specific instance of type T using the specified dictionary key or null if not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_RuntimeDocumentDataTable_TryGetValue__1.htm)

### Properties
- `Document` (RhinoDoc, get) — 

## SnapshotTable (class)

All snapshots in a rhino document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_SnapshotTable.htm)

### Properties
- `Document` (RhinoDoc, get) — Document that owns this table.
- `Names` (String[], get) — Array of Snapshot names.

## StringTable (class)

Collection of document user data strings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_StringTable.htm)

### Methods
#### `public void Delete(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.DocObjects.Tables.StringTable.Delete(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_Delete.htm)

#### `public void Delete(string section, string entry)`

Removes user data strings from the document.

**Parameters:**
- `section` (System.String) — name of section to delete. If null, all sections will be deleted.
- `entry` (System.String) — name of entry to delete. If null, all entries will be deleted for a given section.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_Delete_1.htm)

#### `public string[] GetEntryNames(string section)`

Return list of all entry names for a given section of user data strings in the document.

**Parameters:**
- `section` (System.String) — The section from which to retrieve section names.

**Returns:** `String[]` — An array of section names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_GetEntryNames.htm)

#### `public string GetKey(int i)`



**Parameters:**
- `i` (System.Int32) — [Missing <param name="i"/> documentation for "M:Rhino.DocObjects.Tables.StringTable.GetKey(System.Int32)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.StringTable.GetKey(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_GetKey.htm)

#### `public string[] GetSectionNames()`

Returns a list of all the section names for user data strings in the document. By default a section name is a key that is prefixed with a string separated by a backslash.

**Returns:** `String[]` — An array of section names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_GetSectionNames.htm)

#### `public string GetValue(int i)`



**Parameters:**
- `i` (System.Int32) — [Missing <param name="i"/> documentation for "M:Rhino.DocObjects.Tables.StringTable.GetValue(System.Int32)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.StringTable.GetValue(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_GetValue.htm)

#### `public string GetValue(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.DocObjects.Tables.StringTable.GetValue(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.StringTable.GetValue(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_GetValue_1.htm)

#### `public string GetValue(string section, string entry)`

Gets a user data string from the document.

**Parameters:**
- `section` (System.String) — The section at which to get the value.
- `entry` (System.String) — The entry to search for.

**Returns:** `String` — The user data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_GetValue_2.htm)

#### `public string SetString(string key, string value)`

Adds or sets a user data string to the document.

**Parameters:**
- `key` (System.String) — The key.
- `value` (System.String) — The entry value.

**Returns:** `String` — The previous value if successful and a previous value existed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_SetString.htm)

#### `public string SetString(string section, string entry, string value)`

Adds or sets a user data string to the document.

**Parameters:**
- `section` (System.String) — The section.
- `entry` (System.String) — The entry name.
- `value` (System.String) — The entry value.

**Returns:** `String` — The previous value if successful and a previous value existed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_StringTable_SetString_1.htm)

### Properties
- `Count` (Int32, get) — The number of user data strings in the current document.
- `Document` (RhinoDoc, get) — Document that owns this object table.
- `DocumentDataCount` (Int32, get) — 
- `DocumentUserTextCount` (Int32, get) — 

## ViewTable (class)

[Missing <summary> documentation for "T:Rhino.DocObjects.Tables.ViewTable"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_DocObjects_Tables_ViewTable.htm)

### Methods
#### `public RhinoView Add(string title, DefinedViewportProjection projection, Rectangle position, bool floating)`

Constructs a new Rhino view and, at the same time, adds it to the list.

**Parameters:**
- `title` (System.String) — The title of the new Rhino view.
- `projection` (Rhino.Display.DefinedViewportProjection) — A basic projection type.
- `position` (System.Drawing.Rectangle) — A position.
- `floating` (System.Boolean) — true if the view floats; false if it is docked.

**Returns:** `RhinoView` — The newly constructed Rhino view; or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_Add.htm)

#### `public RhinoPageView AddPageView(string title)`

Constructs a new page view with a given title and, at the same time, adds it to the list.

**Parameters:**
- `title` (System.String) — If null or empty, a name will be generated as "Page #" where # is the largest page number.

**Returns:** `RhinoPageView` — The newly created page view on success; or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_AddPageView.htm)

#### `public RhinoPageView AddPageView(string title, double pageWidth, double pageHeight)`

Constructs a new page view with a given title and size and, at the same time, adds it to the list.

**Parameters:**
- `title` (System.String) — If null or empty, a name will be generated as "Page #" where # is the largest page number.
- `pageWidth` (System.Double) — The page total width.
- `pageHeight` (System.Double) — The page total height.

**Returns:** `RhinoPageView` — The newly created page view on success; or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_AddPageView_1.htm)

#### `public void DefaultViewLayout()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_DefaultViewLayout.htm)

#### `public void EnableCameraIcon(RhinoView view)`

Use to turn camera icon on and off

**Parameters:**
- `view` (Rhino.Display.RhinoView) — If null, any camera icon is turned off. If not null, the camera icon for that view is turned on.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_EnableCameraIcon.htm)

#### `public void EnableRedraw(bool enable, bool redrawDocument, bool redrawLayers)`

Enables or disables screen redrawing.

**Parameters:**
- `enable` (System.Boolean) — Enable redrawing.
- `redrawDocument` (System.Boolean) — If enabling, set to true to have the document redrawn.
- `redrawLayers` (System.Boolean) — If enabling, set to true to have the layer user interface redrawn.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_EnableRedraw.htm)

#### `public RhinoView Find(Guid mainViewportId)`

Finds a view in this document with a given main viewport Id.

**Parameters:**
- `mainViewportId` (System.Guid) — The ID of the main viewport looked for.

**Returns:** `RhinoView` — View on success. null if the view could not be found in this document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_Find.htm)

#### `public RhinoView Find(string mainViewportName, bool compareCase)`

Finds a view in this document with a main viewport that has a given name. Note that there may be multiple views in this document that have the same name. This function only returns the first view found. If you want to find all the views with a given name, use the GetViewList function and iterate through the views.

**Parameters:**
- `mainViewportName` (System.String) — The name of the main viewport.
- `compareCase` (System.Boolean) — true if capitalization influences comparison; otherwise, false.

**Returns:** `RhinoView` — A Rhino view on success; null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_Find_1.htm)

#### `public void FlashObjects(IEnumerable<RhinoObject> list, bool useSelectionColor)`

Cause objects selection state to change momentarily so the object appears to flash on the screen.

**Parameters:**
- `list` (System.Collections.Generic.IEnumerable<RhinoObject>) — An array, a list or any enumerable set of Rhino objects.
- `useSelectionColor` (System.Boolean) — If true, flash between object color and selection color. If false, flash between visible and invisible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_FlashObjects.htm)

#### `public void FourViewLayout(bool useMatchingViews)`



**Parameters:**
- `useMatchingViews` (System.Boolean) — [Missing <param name="useMatchingViews"/> documentation for "M:Rhino.DocObjects.Tables.ViewTable.FourViewLayout(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_FourViewLayout.htm)

#### `public IEnumerator<RhinoView> GetEnumerator()`



**Returns:** `IEnumerator<RhinoView>` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ViewTable.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_GetEnumerator.htm)

#### `public RhinoPageView[] GetPageViews()`

Gets all page views in the document.

**Returns:** `RhinoPageView[]` — An array with all page views. The return value can be an empty array but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_GetPageViews.htm)

#### `public RhinoView[] GetStandardRhinoViews()`



**Returns:** `RhinoView[]` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ViewTable.GetStandardRhinoViews"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_GetStandardRhinoViews.htm)

#### `public RhinoView[] GetViewList(bool includeStandardViews, bool includePageViews)`

Gets an array of all the views.

**Parameters:**
- `includeStandardViews` (System.Boolean) — true if "Right", "Perspective", etc., view should be included; false otherwise.
- `includePageViews` (System.Boolean) — true if page-related views should be included; false otherwise.

**Returns:** `RhinoView[]` — A array of Rhino views. This array can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_GetViewList.htm)

#### `public bool IsCameraIconVisible(RhinoView view)`

Determine if a camera icon is being shown

**Parameters:**
- `view` (Rhino.Display.RhinoView) — if null, then all views are tested. If not null, then just view is tested.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.DocObjects.Tables.ViewTable.IsCameraIconVisible(Rhino.Display.RhinoView)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_IsCameraIconVisible.htm)

#### `public void Redraw()`

Redraws all views.

**Remarks:** If you change something in the document -- like adding objects, deleting objects, modifying layer or object display attributes, etc., then you need to call Redraw to redraw all the views. If you change something in a particular view like the projection, construction plane, background bitmap, etc., then you need to call CRhinoView::Redraw to redraw that particular view.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_Redraw.htm)

#### `public void ThreeViewLayout(bool useMatchingViews)`



**Parameters:**
- `useMatchingViews` (System.Boolean) — [Missing <param name="useMatchingViews"/> documentation for "M:Rhino.DocObjects.Tables.ViewTable.ThreeViewLayout(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_DocObjects_Tables_ViewTable_ThreeViewLayout.htm)

### Properties
- `ActiveView` (RhinoView, get/set) — Gets or Sets the active view.
- `Document` (RhinoDoc, get) — Document that owns this object table.
- `ModelSpaceIsActive` (Boolean, get) — 
- `RedrawEnabled` (Boolean, get/set) — Returns or sets (enable or disables) screen redrawing.

