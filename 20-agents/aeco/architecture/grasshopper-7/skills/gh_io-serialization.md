---
name: grasshopper-gh_io-serialization
description: This skill encodes the grasshopper 7.0 surface of the GH_IO.Serialization namespace — 13 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_Archive, GH_Chunk, GH_Chunk.ChunkKeyedCollection, GH_Compression, GH_LooseChunk, GH_Message, GH_IChunk, GH_IBinarySupport, and 5 more types.
---

# GH_IO.Serialization

Auto-generated from vendor docs for grasshopper 7.0. 13 types in this namespace.

## GH_Archive (class)

This is the base archive class which takes care of all (de)serialization and messaging.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_Archive.htm)

### Constructors
- `public GH_Archive()` — Initializes a new instance of the GH_Archive class

### Methods
#### `public void AddMessage(GH_Message message)`

Add a new message to the record.

**Parameters:**
- `message` (GH_IO.Serialization.GH_Message) — Message to add

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_AddMessage.htm)

#### `public void AddMessage(string messageText, GH_Message_Type messageType)`

Add a new message to the record.

**Parameters:**
- `messageText` (System.String) — Message text.
- `messageType` (GH_IO.Serialization.GH_Message_Type) — Message type.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_AddMessage_1.htm)

#### `public bool AppendObject(GH_ISerializable target, string targetName)`

Appends a target object into the root node of this archive tree. If the root doesn't exist yet, it will be created. Existing objects at the root scope will not be affected.

**Parameters:**
- `target` (GH_IO.GH_ISerializable) — Object to append. Cannot be null.
- `targetName` (System.String) — Name of object to append, name must be unique.

**Returns:** `Boolean` — True on succes, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_AppendObject.htm)

#### `public void ClearMessages()`

Remove all messages from the log.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_ClearMessages.htm)

#### `public void CreateNewRoot(bool forWriting)`

Discards the current data tree and instantiates a new root node. This root node contains some comments, a version value containing the current version of GH_IO.dll and a DateTime value containing the current date and time.

**Parameters:**
- `forWriting` (System.Boolean) — If true, all data fields are reset.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_CreateNewRoot.htm)

#### `public GH_IWriter CreateTopLevelNode(string rootName)`

Creates and returns a new root node for this archive in the form of a GH_IWriter instance. Typically you do not call this method. If you want to add an object to the archive, use AppendObject() instead.

**Parameters:**
- `rootName` (System.String) — Name of root object.

**Returns:** `GH_IWriter` — The GH_IWriter incarnation of the GH_Chunk that represents the new Node.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_CreateTopLevelNode.htm)

#### `public bool Deserialize_Binary(byte[] data)`

Deserializes an array of bytes.

**Parameters:**
- `data` (System.Byte[]) — Byte array to deserialize.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_Deserialize_Binary.htm)

#### `public bool Deserialize_Xml(string xmlContent)`

Deserializes an Xml string.

**Parameters:**
- `xmlContent` (System.String) — Xml to deserialize.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_Deserialize_Xml.htm)

#### `public bool ExtractObject(GH_ISerializable target, string targetName)`

Extract a target object from the data tree.

**Parameters:**
- `target` (GH_IO.GH_ISerializable) — Object to extract. Cannot be null.
- `targetName` (System.String) — Name of object to extract, name must identify an existing chunk.

**Returns:** `Boolean` — True on succes, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_ExtractObject.htm)

#### `public int MessageCount()`

Gets the number of messages recorded since the most recent IO operation began.

**Returns:** `Int32` — The number of recorded messages.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_MessageCount.htm)

#### `public int MessageCount(bool includeInfo, bool includeWarnings, bool includeErrors)`

Gets the number of messages recorded since the most recent IO operation began. Message count can be filtered by message type.

**Parameters:**
- `includeInfo` (System.Boolean) — If true, info messages will be included in the count.
- `includeWarnings` (System.Boolean) — If true, warnings will be included in the count.
- `includeErrors` (System.Boolean) — If true, errors will be included in the count.

**Returns:** `Int32` — The number of recoded messages that pass the type filters.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_MessageCount_1.htm)

#### `public static bool OpenFileDialog(string title, ref string filePath, List<string> additionalFilters)`

Displays a standard OpenFileDialog with all the fields set to cater for Grasshopper files.

**Parameters:**
- `title` (System.String) — Title of the dialog. If null, the default title will be used.
- `filePath` (System.String) — Path to file picked by user. If file_path represents a valid string, the dialog is set up to match.
- `additionalFilters` (System.Collections.Generic.List<String>) — A list of additional file format filters or null. Filter strings must be in typical FileDialog.Filter syntax: "File Name (*.ext)|*.ext"

**Returns:** `Boolean` — True is user picked a path, False if user cancelled.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_OpenFileDialog.htm)

#### `public bool ReadFromFile(string fileName)`

Reads a new archive tree from a file.

**Parameters:**
- `fileName` (System.String) — Path of file to parse.

**Returns:** `Boolean` — True on success, false on failure. If the read operation fails, all the member fields of this archive ought to be treated as invalid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_ReadFromFile.htm)

#### `public static bool SaveFileDialog(string title, ref string filePath, List<string> additionalFilters)`

Displays a standard SaveFileDialog with all the fields set to cater for Grasshopper files.

**Parameters:**
- `title` (System.String) — Title of the dialog. If null, the default title will be used.
- `filePath` (System.String) — Path to file picked by user. If file_path represents a valid string, the dialog is set up to match.
- `additionalFilters` (System.Collections.Generic.List<String>) — A list of additional file format filters or null. Filter strings must be in typical FileDialog.Filter syntax: "File Name (*.ext)|*.ext"

**Returns:** `Boolean` — True is user picked a new path, False if user cancelled.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_SaveFileDialog.htm)

#### `public byte[] Serialize_Binary()`

Serializes the data tree into a Binary byte array.

**Returns:** `Byte[]` — A byte array containing the Binary stream.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_Serialize_Binary.htm)

#### `public string Serialize_Xml()`

Serializes the data tree into an Xml string.

**Returns:** `String` — A string containing the Xml content.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_Serialize_Xml.htm)

#### `public DialogResult ShowMessageLog()`

Displays the message log viewer. You should typically only display the viewer if the WorstCaseMessageType equals GH_Message_Type.warning or GH_Message_Type.error

**Returns:** `DialogResult` — The closing flag for the displayed log form.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_ShowMessageLog.htm)

#### `public bool WriteToFile(string fileName, bool overwrite, bool rememberPath)`

Writes the current tree to a file.

**Parameters:**
- `fileName` (System.String) — Path of file to write to. If the extension is not a recognized Grasshopper extension, an exception will be thrown.
- `overwrite` (System.Boolean) — True to overwrite file at specified location.
- `rememberPath` (System.Boolean) — If True, the MRU path field will be updated to reflect the new path.

**Returns:** `Boolean` — True on succes, false if file already exists and overwrite is set to false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Archive_WriteToFile.htm)

### Properties
- `FileName` (String, get) — Gets the filename (without extension) of the currently loaded data tree. If the path field has not been set, "unnamed" is returned.
- `GetRootNode` (GH_Chunk, get) — Gets the root node of this archive. Typically you do not need to modify the Root. Use functions like CreateTopLevelNode(), AppendObject() and ExtractObject() instead. If you modify the Root node, you may corrupt the archive.
- `GH_IO_Version` (GH_Version, get) — Gets the version number of the current GH_IO build.
- `GrasshopperBinaryExtension` (String, get) — Gets the file extension (including the dot) associated with Grasshopper® Binary data.
- `GrasshopperUserExtension` (String, get) — Gets the file extension (including the dot) associated with Grasshopper® User Object file.
- `GrasshopperXmlExtension` (String, get) — Gets the file extension (including the dot) associated with Grasshopper® XML data.
- `IsBinaryPath` (Boolean, get) — Gets a value that indicates whether the Path field points to a Binary Grasshopper file.
- `IsPath` (Boolean, get) — Gets a value that indicates whether or not the path field has been set.
- `IsXmlPath` (Boolean, get) — Gets a value that indicates whether the Path field points to an Xml Grasshopper file.
- `Messages` (List<GH_Message>, get) — Gets the internal list of messages.
- `Path` (String, get/set) — Gets the path to the file from which this archive was read and/or written to. If the archive hasn't been read or written yet, this field will be null.
- `WorstCaseMessageType` (GH_Message_Type, get) — Gets the worst case message type. If the record contains at least 1 error, the worst case is GH_Message_Type.error. If the record contains no errors, but at least 1 warning, the worst case is GH_Message_Type.warning. If the record contains no messages or only infos, the worst case type is GH_Message_Type.info.

## GH_Chunk (class)

Full implementation of GH_IChunk, GH_IReader, GH_IWriter, GH_IBinarySupport and GH_IXmlSupport. Instances of this class are usually disguised as one of the interfaces it implements.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_Chunk.htm)

### Constructors
- `protected GH_Chunk()` — Blank constructor. You typically do not need to create your own Chunks. The GH_Archive class will create top-level chunks and existing chunks can create child chunks.
- `protected GH_Chunk(GH_Archive chunk_archive)` — Constructor. You typically do not need to create your own Chunks. The GH_Archive class will create top-level chunks and existing chunks can create child chunks.
- `protected GH_Chunk(GH_Archive chunk_archive, string chunk_name)` — Constructor. You typically do not need to create your own Chunks. The GH_Archive class will create top-level chunks and existing chunks can create child chunks.
- `protected GH_Chunk(GH_Archive chunk_archive, string chunk_name, int chunk_index)` — Constructor. You typically do not need to create your own Chunks. The GH_Archive class will create top-level chunks and existing chunks can create child chunks.

### Methods
#### `public void AddComment(string comment_text)`

Adds a text comment to this chunk. Comments are serialized only if the output flavour is a human readable format. Comments are never deserialized, they are purely for the benefit of the humans reading the file data.

**Parameters:**
- `comment_text` (System.String) — The content of the comment, text might be altered if it contains invalid characters for a chosen format flavour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_AddComment.htm)

#### `public void AddMessage(string m, GH_Message_Type t)`

Log a new message with the top-level archive. Messages are collected during read/write operations, and can be displayed to the user upon completion using GH_Archive.ShowMessageLog().

**Parameters:**
- `m` (System.String) — Message text.
- `t` (GH_IO.Serialization.GH_Message_Type) — Message type.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_AddMessage.htm)

#### `public bool ChunkExists(string name)`

Checks whether a chunk with the specified name exists in the litter. Only chunks without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of chunk to test for.

**Returns:** `Boolean` — True if a chunk with the specified name exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkExists.htm)

#### `public bool ChunkExists(string name, int index)`

Checks whether a chunk with the specified name and index exists in the litter. Only chunks with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of chunk to test for.
- `index` (System.Int32) — Index of chunk to test for. If less than zero, ChunkExists(string name) is called instead.

**Returns:** `Boolean` — True if a chunk with the specified name and index exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkExists_1.htm)

#### `public void CopyValuesFromChunk(GH_Chunk other)`

Copy all values and sub-chunks from another chunk.

**Parameters:**
- `other` (GH_IO.Serialization.GH_Chunk)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CopyValuesFromChunk.htm)

#### `public GH_Chunk CopyValuesToChunk()`

Copy all values and sub-chunks in this chunk to another chunk which does not point to the same archive.

**Returns:** `GH_Chunk` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CopyValuesToChunk.htm)

#### `public GH_IWriter CreateChunk(string chunk_name)`

Create a new child chunk with the specified name and without an index qualifier. If another chunk already exists with similar properties, an exception will be thrown.

**Parameters:**
- `chunk_name` (System.String) — Name of new child.

**Returns:** `GH_IWriter` — The newly created child chunk.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CreateChunk.htm)

#### `public GH_IWriter CreateChunk(string chunk_name, int chunk_index)`

Create a new child chunk with the specified name and index qualifier. If another chunk already exists with similar properties, an exception will be thrown.

**Parameters:**
- `chunk_name` (System.String) — Name of new child.
- `chunk_index` (System.Int32) — Index of new child.

**Returns:** `GH_IWriter` — The newly created child chunk.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CreateChunk_1.htm)

#### `public GH_IReader FindChunk(string name)`

Finds the first chunk in the litter that matches the given name. Only chunks without index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of chunk to search for.

**Returns:** `GH_IReader` — The child chunk that matches the given name, or null of no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindChunk.htm)

#### `public GH_IReader FindChunk(string name, int index)`

Finds the first chunk in the list that matches the given name and index. Only chunks with index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of chunk to search for.
- `index` (System.Int32) — Index of chunk to search for. If less than zero, FindChunk(string chunk_name) is called instead.

**Returns:** `GH_IReader` — The child chunk that matches the given name and index, or null of no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindChunk_1.htm)

#### `public GH_Item FindItem(string name)`

Finds the first item that matches the given name. Only items without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to search for.

**Returns:** `GH_Item` — The item that matches the given name, or null of no matching item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindItem.htm)

#### `public GH_Item FindItem(string name, int index)`

Finds the first item that matches the given name and index. Only items with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to search for.
- `index` (System.Int32) — Index of item to search for. If less than zero, then FindItem(string name) is called instead.

**Returns:** `GH_Item` — The item that matches the given name and index, or null of no matching item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindItem_1.htm)

#### `public bool GetBoolean(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Boolean` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoolean.htm)

#### `public bool GetBoolean(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Boolean` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoolean_1.htm)

#### `public GH_BoundingBox GetBoundingBox(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_BoundingBox` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoundingBox.htm)

#### `public GH_BoundingBox GetBoundingBox(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_BoundingBox` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoundingBox_1.htm)

#### `public byte GetByte(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Byte` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByte.htm)

#### `public byte GetByte(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Byte` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByte_1.htm)

#### `public byte[] GetByteArray(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Byte[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByteArray.htm)

#### `public byte[] GetByteArray(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Byte[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByteArray_1.htm)

#### `public DateTime GetDate(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `DateTime` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDate.htm)

#### `public DateTime GetDate(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `DateTime` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDate_1.htm)

#### `public decimal GetDecimal(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Decimal` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDecimal.htm)

#### `public decimal GetDecimal(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Decimal` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDecimal_1.htm)

#### `public double GetDouble(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Double` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDouble.htm)

#### `public double GetDouble(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Double` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDouble_1.htm)

#### `public double[] GetDoubleArray(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Double[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDoubleArray.htm)

#### `public double[] GetDoubleArray(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Double[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDoubleArray_1.htm)

#### `public Bitmap GetDrawingBitmap(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Bitmap` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingBitmap.htm)

#### `public Bitmap GetDrawingBitmap(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Bitmap` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingBitmap_1.htm)

#### `public Color GetDrawingColor(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Color` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingColor.htm)

#### `public Color GetDrawingColor(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Color` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingColor_1.htm)

#### `public Point GetDrawingPoint(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Point` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPoint.htm)

#### `public Point GetDrawingPoint(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Point` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPoint_1.htm)

#### `public PointF GetDrawingPointF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `PointF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPointF.htm)

#### `public PointF GetDrawingPointF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `PointF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPointF_1.htm)

#### `public Rectangle GetDrawingRectangle(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Rectangle` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangle.htm)

#### `public Rectangle GetDrawingRectangle(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Rectangle` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangle_1.htm)

#### `public RectangleF GetDrawingRectangleF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `RectangleF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangleF.htm)

#### `public RectangleF GetDrawingRectangleF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `RectangleF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangleF_1.htm)

#### `public Size GetDrawingSize(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Size` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSize.htm)

#### `public Size GetDrawingSize(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Size` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSize_1.htm)

#### `public SizeF GetDrawingSizeF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `SizeF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSizeF.htm)

#### `public SizeF GetDrawingSizeF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `SizeF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSizeF_1.htm)

#### `public Guid GetGuid(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Guid` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetGuid.htm)

#### `public Guid GetGuid(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Guid` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetGuid_1.htm)

#### `public int GetInt32(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Int32` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt32.htm)

#### `public int GetInt32(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Int32` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt32_1.htm)

#### `public long GetInt64(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Int64` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt64.htm)

#### `public long GetInt64(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Int64` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt64_1.htm)

#### `public GH_Interval1D GetInterval1D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Interval1D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval1D.htm)

#### `public GH_Interval1D GetInterval1D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Interval1D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval1D_1.htm)

#### `public GH_Interval2D GetInterval2D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Interval2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval2D.htm)

#### `public GH_Interval2D GetInterval2D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Interval2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval2D_1.htm)

#### `public GH_Line GetLine(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Line` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetLine.htm)

#### `public GH_Line GetLine(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Line` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetLine_1.htm)

#### `public string[] GetPath(string item_name, int item_index, string basePath)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `basePath` (System.String) — Base path for relative path resolution.

**Returns:** `String[]` — An array of path strings. If the resolved relative path is different from the stored absolute path, an array of two strings will be returned [absolute, relative].

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPath.htm)

#### `public string[] GetPath(string item_name, string basePath)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `basePath` (System.String) — Base path for relative path resolution.

**Returns:** `String[]` — An array of path strings. If the resolved relative path is different from the stored absolute path, an array of two strings will be returned [absolute, relative].

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPath_1.htm)

#### `public GH_Plane GetPlane(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Plane` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPlane.htm)

#### `public GH_Plane GetPlane(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Plane` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPlane_1.htm)

#### `public GH_Point2D GetPoint2D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint2D.htm)

#### `public GH_Point2D GetPoint2D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint2D_1.htm)

#### `public GH_Point3D GetPoint3D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point3D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint3D.htm)

#### `public GH_Point3D GetPoint3D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point3D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint3D_1.htm)

#### `public GH_Point4D GetPoint4D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point4D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint4D.htm)

#### `public GH_Point4D GetPoint4D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point4D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint4D_1.htm)

#### `public float GetSingle(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Single` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetSingle.htm)

#### `public float GetSingle(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Single` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetSingle_1.htm)

#### `public string GetString(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `String` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetString.htm)

#### `public string GetString(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `String` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetString_1.htm)

#### `public GH_Version GetVersion(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Version` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetVersion.htm)

#### `public GH_Version GetVersion(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Version` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetVersion_1.htm)

#### `public static GH_Chunk InstantiateRoot(GH_Archive owner)`

Construct a new instance of GH_Chunk which acts as a Root node. If you must create a chunk from scratch, use this static method to create one.

**Parameters:**
- `owner` (GH_IO.Serialization.GH_Archive) — The archive which owns this chunk.

**Returns:** `GH_Chunk` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_InstantiateRoot.htm)

#### `public bool ItemExists(string name)`

Gets the occupancy for a specific item name. Only items without index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of item to check for.

**Returns:** `Boolean` — True if an item with a similar name already exists, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ItemExists.htm)

#### `public bool ItemExists(string name, int index)`

Checks whether an item with the specified name and index exists. Only items with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to test for.
- `index` (System.Int32) — Index of item to test for. If index is less than zero, then ItemExists(string name) is called instead.

**Returns:** `Boolean` — True if an item with the specified name and index exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ItemExists_1.htm)

#### `public void Read(BinaryReader reader)`

Read this chunk and all child chunks from a binary stream.

**Parameters:**
- `reader` (System.IO.BinaryReader) — The Binary reader to use, cannot be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Read.htm)

#### `public void Read(XmlNode node)`

Read this chunk and all child chunks from an Xml node.

**Parameters:**
- `node` (System.Xml.XmlNode) — The Xml node to deserialize from, cannot be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Read_1.htm)

#### `public bool RemoveChunk(GH_IChunk chunk)`

Remove the specified chunk from the litter.

**Parameters:**
- `chunk` (GH_IO.Serialization.GH_IChunk) — Chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if chunk is not part of this litter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveChunk.htm)

#### `public bool RemoveChunk(string chunk_name)`

Remove the first chunk with a matching name. Only chunks without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `chunk_name` (System.String) — Name of chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveChunk_1.htm)

#### `public bool RemoveChunk(string chunk_name, int chunk_index)`

Remove the first chunk with a matching name and index. Only chunks with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `chunk_name` (System.String) — Name of chunk to remove.
- `chunk_index` (System.Int32) — Index of chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveChunk_2.htm)

#### `public bool RemoveItem(string itemName)`

Remove an unindexed item from this chunk.

**Parameters:**
- `itemName` (System.String) — Name of item.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveItem.htm)

#### `public bool RemoveItem(string itemName, int itemIndex)`

Remove an indexed item from this chunk.

**Parameters:**
- `itemName` (System.String) — Name of item.
- `itemIndex` (System.Int32) — Index of item.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveItem_1.htm)

#### `public void SetBoolean(string item_name, bool item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Boolean) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoolean.htm)

#### `public void SetBoolean(string item_name, int item_index, bool item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Boolean) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoolean_1.htm)

#### `public void SetBoundingBox(string item_name, GH_BoundingBox item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_BoundingBox) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoundingBox.htm)

#### `public void SetBoundingBox(string item_name, int item_index, GH_BoundingBox item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_BoundingBox) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoundingBox_1.htm)

#### `public void SetByte(string item_name, byte item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Byte) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByte.htm)

#### `public void SetByte(string item_name, int item_index, byte item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Byte) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByte_1.htm)

#### `public void SetByteArray(string item_name, byte[] item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Byte[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByteArray.htm)

#### `public void SetByteArray(string item_name, int item_index, byte[] item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Byte[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByteArray_1.htm)

#### `public void SetDate(string item_name, DateTime item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.DateTime) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDate.htm)

#### `public void SetDate(string item_name, int item_index, DateTime item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.DateTime) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDate_1.htm)

#### `public void SetDecimal(string item_name, decimal item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Decimal) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDecimal.htm)

#### `public void SetDecimal(string item_name, int item_index, decimal item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Decimal) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDecimal_1.htm)

#### `public void SetDouble(string item_name, double item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Double) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDouble.htm)

#### `public void SetDouble(string item_name, int item_index, double item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Double) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDouble_1.htm)

#### `public void SetDoubleArray(string item_name, double[] item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Double[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDoubleArray.htm)

#### `public void SetDoubleArray(string item_name, int item_index, double[] item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Double[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDoubleArray_1.htm)

#### `public void SetDrawingBitmap(string item_name, Bitmap item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Bitmap) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingBitmap.htm)

#### `public void SetDrawingBitmap(string item_name, int item_index, Bitmap item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Bitmap) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingBitmap_1.htm)

#### `public void SetDrawingColor(string item_name, Color item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Color) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingColor.htm)

#### `public void SetDrawingColor(string item_name, int item_index, Color item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Color) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingColor_1.htm)

#### `public void SetDrawingPoint(string item_name, int item_index, Point item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Point) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPoint_1.htm)

#### `public void SetDrawingPoint(string item_name, Point item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Point) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPoint.htm)

#### `public void SetDrawingPointF(string item_name, int item_index, PointF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.PointF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPointF_1.htm)

#### `public void SetDrawingPointF(string item_name, PointF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.PointF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPointF.htm)

#### `public void SetDrawingRectangle(string item_name, int item_index, Rectangle item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Rectangle) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangle_1.htm)

#### `public void SetDrawingRectangle(string item_name, Rectangle item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Rectangle) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangle.htm)

#### `public void SetDrawingRectangleF(string item_name, int item_index, RectangleF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.RectangleF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangleF_1.htm)

#### `public void SetDrawingRectangleF(string item_name, RectangleF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.RectangleF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangleF.htm)

#### `public void SetDrawingSize(string item_name, int item_index, Size item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Size) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSize_1.htm)

#### `public void SetDrawingSize(string item_name, Size item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Size) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSize.htm)

#### `public void SetDrawingSizeF(string item_name, int item_index, SizeF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.SizeF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSizeF_1.htm)

#### `public void SetDrawingSizeF(string item_name, SizeF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.SizeF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSizeF.htm)

#### `public void SetGuid(string item_name, Guid item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Guid) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetGuid.htm)

#### `public void SetGuid(string item_name, int item_index, Guid item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Guid) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetGuid_1.htm)

#### `public void SetInt32(string item_name, int item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Int32) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt32.htm)

#### `public void SetInt32(string item_name, int item_index, int item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Int32) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt32_1.htm)

#### `public void SetInt64(string item_name, int item_index, long item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Int64) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt64.htm)

#### `public void SetInt64(string item_name, long item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Int64) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt64_1.htm)

#### `public void SetInterval1D(string item_name, GH_Interval1D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Interval1D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval1D.htm)

#### `public void SetInterval1D(string item_name, int item_index, GH_Interval1D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Interval1D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval1D_1.htm)

#### `public void SetInterval2D(string item_name, GH_Interval2D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Interval2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval2D.htm)

#### `public void SetInterval2D(string item_name, int item_index, GH_Interval2D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Interval2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval2D_1.htm)

#### `public void SetLine(string item_name, GH_Line item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Line) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetLine.htm)

#### `public void SetLine(string item_name, int item_index, GH_Line item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Line) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetLine_1.htm)

#### `public void SetPath(string item_name, int item_index, string absolutePath, string basePath)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `absolutePath` (System.String) — Absolute path to store.
- `basePath` (System.String) — Base path. This will be used to also store a relative path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPath.htm)

#### `public void SetPath(string item_name, string absolutePath, string basePath)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `absolutePath` (System.String) — Absolute path to store.
- `basePath` (System.String) — Base path. This will be used to also store a relative path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPath_1.htm)

#### `public void SetPlane(string item_name, GH_Plane item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Plane) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPlane.htm)

#### `public void SetPlane(string item_name, int item_index, GH_Plane item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Plane) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPlane_1.htm)

#### `public void SetPoint2D(string item_name, GH_Point2D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint2D.htm)

#### `public void SetPoint2D(string item_name, int item_index, GH_Point2D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint2D_1.htm)

#### `public void SetPoint3D(string item_name, GH_Point3D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point3D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint3D.htm)

#### `public void SetPoint3D(string item_name, int item_index, GH_Point3D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point3D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint3D_1.htm)

#### `public void SetPoint4D(string item_name, GH_Point4D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point4D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint4D.htm)

#### `public void SetPoint4D(string item_name, int item_index, GH_Point4D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point4D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint4D_1.htm)

#### `public void SetSingle(string item_name, int item_index, float item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Single) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetSingle.htm)

#### `public void SetSingle(string item_name, float item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Single) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetSingle_1.htm)

#### `public void SetString(string item_name, int item_index, string item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.String) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetString.htm)

#### `public void SetString(string item_name, string item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.String) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetString_1.htm)

#### `public void SetVersion(string item_name, GH_Version item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Version) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion.htm)

#### `public void SetVersion(string item_name, int item_index, GH_Version item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Version) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion_1.htm)

#### `public void SetVersion(string item_name, int major, int minor, int revision)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `major` (System.Int32) — Major component of Version structure.
- `minor` (System.Int32) — Minor component of Version structure.
- `revision` (System.Int32) — Revision component of Version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion_2.htm)

#### `public void SetVersion(string item_name, int item_index, int major, int minor, int revision)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `major` (System.Int32) — Major component of Version structure.
- `minor` (System.Int32) — Minor component of Version structure.
- `revision` (System.Int32) — Revision component of Version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion_3.htm)

#### `public bool TryGetBoolean(string item_name, ref bool value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Boolean) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoolean.htm)

#### `public bool TryGetBoolean(string item_name, int item_index, ref bool value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Boolean) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoolean_1.htm)

#### `public bool TryGetBoundingBox(string item_name, ref GH_BoundingBox value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_BoundingBox) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoundingBox.htm)

#### `public bool TryGetBoundingBox(string item_name, int item_index, ref GH_BoundingBox value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_BoundingBox) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoundingBox_1.htm)

#### `public bool TryGetByte(string item_name, ref byte value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Byte) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetByte.htm)

#### `public bool TryGetByte(string item_name, int item_index, ref byte value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Byte) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetByte_1.htm)

#### `public bool TryGetDate(string item_name, ref DateTime value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.DateTime) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDate.htm)

#### `public bool TryGetDate(string item_name, int item_index, ref DateTime value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.DateTime) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDate_1.htm)

#### `public bool TryGetDecimal(string item_name, ref decimal value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Decimal) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDecimal.htm)

#### `public bool TryGetDecimal(string item_name, int item_index, ref decimal value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Decimal) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDecimal_1.htm)

#### `public bool TryGetDouble(string item_name, ref double value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Double) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDouble.htm)

#### `public bool TryGetDouble(string item_name, int item_index, ref double value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Double) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDouble_1.htm)

#### `public bool TryGetDrawingColor(string item_name, ref Color value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Color) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingColor.htm)

#### `public bool TryGetDrawingColor(string item_name, int item_index, ref Color value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Color) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingColor_1.htm)

#### `public bool TryGetDrawingPoint(string item_name, int item_index, ref Point value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Point) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPoint_1.htm)

#### `public bool TryGetDrawingPoint(string item_name, ref Point value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Point) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPoint.htm)

#### `public bool TryGetDrawingPointF(string item_name, int item_index, ref PointF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.PointF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPointF_1.htm)

#### `public bool TryGetDrawingPointF(string item_name, ref PointF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.PointF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPointF.htm)

#### `public bool TryGetDrawingRectangle(string item_name, int item_index, ref Rectangle value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Rectangle) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangle_1.htm)

#### `public bool TryGetDrawingRectangle(string item_name, ref Rectangle value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Rectangle) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangle.htm)

#### `public bool TryGetDrawingRectangleF(string item_name, int item_index, ref RectangleF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.RectangleF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangleF_1.htm)

#### `public bool TryGetDrawingRectangleF(string item_name, ref RectangleF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.RectangleF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangleF.htm)

#### `public bool TryGetDrawingSize(string item_name, int item_index, ref Size value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Size) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSize_1.htm)

#### `public bool TryGetDrawingSize(string item_name, ref Size value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Size) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSize.htm)

#### `public bool TryGetDrawingSizeF(string item_name, int item_index, ref SizeF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.SizeF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSizeF_1.htm)

#### `public bool TryGetDrawingSizeF(string item_name, ref SizeF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.SizeF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSizeF.htm)

#### `public bool TryGetGuid(string item_name, ref Guid value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Guid) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetGuid.htm)

#### `public bool TryGetGuid(string item_name, int item_index, ref Guid value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Guid) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetGuid_1.htm)

#### `public bool TryGetInt32(string item_name, ref int value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Int32) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt32_1.htm)

#### `public bool TryGetInt32(string item_name, int item_index, ref int value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Int32) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt32.htm)

#### `public bool TryGetInt64(string item_name, int item_index, ref long value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Int64) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt64.htm)

#### `public bool TryGetInt64(string item_name, ref long value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Int64) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt64_1.htm)

#### `public bool TryGetInterval1D(string item_name, ref GH_Interval1D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Interval1D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval1D.htm)

#### `public bool TryGetInterval1D(string item_name, int item_index, ref GH_Interval1D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Interval1D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval1D_1.htm)

#### `public bool TryGetInterval2D(string item_name, ref GH_Interval2D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Interval2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval2D.htm)

#### `public bool TryGetInterval2D(string item_name, int item_index, ref GH_Interval2D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Interval2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval2D_1.htm)

#### `public bool TryGetLine(string item_name, ref GH_Line value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Line) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetLine.htm)

#### `public bool TryGetLine(string item_name, int item_index, ref GH_Line value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Line) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetLine_1.htm)

#### `public bool TryGetPlane(string item_name, ref GH_Plane value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Plane) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPlane.htm)

#### `public bool TryGetPlane(string item_name, int item_index, ref GH_Plane value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Plane) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPlane_1.htm)

#### `public bool TryGetPoint2D(string item_name, ref GH_Point2D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint2D.htm)

#### `public bool TryGetPoint2D(string item_name, int item_index, ref GH_Point2D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint2D_1.htm)

#### `public bool TryGetPoint3D(string item_name, ref GH_Point3D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point3D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint3D.htm)

#### `public bool TryGetPoint3D(string item_name, int item_index, ref GH_Point3D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point3D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint3D_1.htm)

#### `public bool TryGetPoint4D(string item_name, ref GH_Point4D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point4D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint4D.htm)

#### `public bool TryGetPoint4D(string item_name, int item_index, ref GH_Point4D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point4D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint4D_1.htm)

#### `public bool TryGetSingle(string item_name, int item_index, ref float value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Single) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetSingle.htm)

#### `public bool TryGetSingle(string item_name, ref float value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Single) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetSingle_1.htm)

#### `public bool TryGetString(string item_name, int item_index, ref string value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.String) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetString.htm)

#### `public bool TryGetString(string item_name, ref string value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.String) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetString_1.htm)

#### `public bool TryGetVersion(string item_name, ref GH_Version value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Version) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetVersion.htm)

#### `public bool TryGetVersion(string item_name, int item_index, ref GH_Version value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Version) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetVersion_1.htm)

#### `public void Write(BinaryWriter writer)`

Write this chunk and all child chunks to a binary stream.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — The Binary writer to use, cannot be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Write.htm)

#### `public void Write(XmlWriter writer)`

Serialize this chunk into an Xml stream.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Xml writer used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Write_1.htm)

### Properties
- `Archive` (GH_Archive, get) — Gets a pointer to the archive that owns the Root of the tree this chunk belongs to.
- `ArchiveLocation` (String, get) — Gets a string representing the URI with which the archive is associated. The location may be a null string.
- `ChunkCount` (Int32, get) — Gets the number of child chunks contained in this chunk. The set of all child chunks is referred to as a 'litter'.
- `Chunks` (List<GH_IChunk>, get) — Gets a pointer to the internal list of child chunks. Do not access this list unless you know what you are doing.
- `HasComments` (Boolean, get) — Gets a value that indicates whether or not comments have been stored in this chunk.
- `HasIndex` (Boolean, get) — Gets the index existence implication. The item is considered to have an index qualifier if the index value is larger than or equal to zero.
- `HasName` (Boolean, get) — Gets the name validity of this item. The item is considered to have an invalid name if string.IsNullOrEmpty(name)
- `Index` (Int32, get) — Gets the index of this chunk. The index is set by the owner of this chunk. Indices smaller than zero imply no index has been set. The combination of name+index is always unique among a set of chunks in the same litter.
- `ItemCount` (Int32, get) — Gets the number of items contained in this chunk.
- `Items` (List<GH_Item>, get) — Gets a pointer to the internal list of items. Do not access this list unless you know what you are doing.
- `Name` (String, get) — Gets the name of this chunk. The name is set by the owner of this chunk. Names must be at least 1 character long. The combination of name+index is always unique among a set of chunks in a single litter.
- `m_archive` (GH_Archive, get) — Pointer to the archive that owns the Root of the tree this chunk belongs to.
- `m_chunks` (GH_Chunk.ChunkKeyedCollection, get) — Dictionary of sub-chunks contained within this chunk.
- `m_comments` (List<String>, get) — List of text comments. This list is automatically instantiated once the first comment is added.
- `m_index` (Int32, get) — Index of this chunk. This field is set only once, during construction. A negative index indicates no index qualifier has been set.
- `m_items` (SortedDictionary<ID,GH_Item>, get) — The list of nodes contained within this chunk.
- `m_name` (String, get) — Name of this chunk. This field is set only once, during construction.
- `name_comp` (StringComparison, get) — Comparison flag to use when comparing names.

## GH_Chunk.ChunkKeyedCollection (class)

Represents a collection of chunks with associated IDs

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection.htm)

### Constructors
- `public ChunkKeyedCollection()` — Create a new collection.

### Methods
#### `public void Add(GH_Chunk chunk)`

Adds a new chunk.

**Parameters:**
- `chunk` (GH_IO.Serialization.GH_Chunk) — The chunk to be added.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection_Add.htm)

#### `public void Clear()`

Removes all chunks from the collection.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection_Clear.htm)

#### `public IEnumerator<GH_Chunk> GetEnumerator()`

Gets an enumerator that iterates over all chunks in the collection.

**Returns:** `IEnumerator<GH_Chunk>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection_GetEnumerator.htm)

#### `public bool Remove(GH_Chunk chunk)`

Removes a chunk.

**Parameters:**
- `chunk` (GH_IO.Serialization.GH_Chunk) — The chunk to be removed.

**Returns:** `Boolean` — True if the operation was successful; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection_Remove.htm)

#### `public bool Remove(ID id)`

Removes an ID.

**Parameters:**
- `id` (GH_IO.Serialization.ID) — The ID to be removed.

**Returns:** `Boolean` — True if the operation was successful; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection_Remove_1.htm)

#### `public bool TryGetValue(ID id, out GH_Chunk chunk)`

Try and find the chunk that belongs to a given ID.

**Parameters:**
- `id` (GH_IO.Serialization.ID) — ID to look for.
- `chunk` (GH_IO.Serialization.GH_Chunk) — Chunk result.

**Returns:** `Boolean` — True is a chunk was found, false if otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkKeyedCollection_TryGetValue.htm)

### Properties
- `Count` (Int32, get) — Returns the amount in the collection.

## GH_Compression (class)

Provides static methods for compression of byte-arrays.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_Compression.htm)

### Methods
#### `public static byte[] Compress(byte[] data)`

Compress an array of bytes using the Deflate algorithm.

**Parameters:**
- `data` (System.Byte[]) — Byte array to compress.

**Returns:** `Byte[]` — An array of compressed bytes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Compression_Compress.htm)

#### `public static byte[] Decompress(byte[] compressedData)`

Decompress an array of bytes using the Deflate algorithm.

**Parameters:**
- `compressedData` (System.Byte[]) — Byte array to decompress.

**Returns:** `Byte[]` — An array of decompressed bytes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Compression_Decompress.htm)

## GH_IBinarySupport (interface)

Interface which declares all methods required for objects that can be (de)serialized to and from a binary archive.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_IBinarySupport.htm)

### Methods
#### `void Read(BinaryReader reader)`

Called when an object is required to deserialize itself.

**Parameters:**
- `reader` (System.IO.BinaryReader) — Reader object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Read.htm)

#### `void Write(BinaryWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Write.htm)

## GH_IChunk (interface)

Base interface for all Archive Chunks.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_IChunk.htm)

### Methods
#### `void AddMessage(string m, GH_Message_Type t)`

Log a new message with the top-level archive. Messages are collected during read/write operations, and can be displayed to the user upon completion using GH_Archive.ShowMessageLog().

**Parameters:**
- `m` (System.String) — Message text.
- `t` (GH_IO.Serialization.GH_Message_Type) — Message type.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IChunk_AddMessage.htm)

#### `void Read(BinaryReader reader)`

Called when an object is required to deserialize itself.

**Parameters:**
- `reader` (System.IO.BinaryReader) — Reader object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Read.htm)

#### `void Read(XmlNode node)`

Called when an object is required to deserialize itself.

**Parameters:**
- `node` (System.Xml.XmlNode) — Node object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Read.htm)

#### `void Write(BinaryWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Write.htm)

#### `void Write(XmlWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Write.htm)

### Properties
- `Archive` (GH_Archive, get) — Gets a pointer to the archive that owns the Root of the tree this chunk belongs to.
- `ArchiveLocation` (String, get) — Gets a string representing the URI with which the archive is associated. The location may be a null string.
- `ChunkCount` (Int32, get) — Gets the number of child chunks contained in this chunk. The set of all child chunks is referred to as a 'litter'.
- `Chunks` (List<GH_IChunk>, get) — Gets a pointer to the internal list of child chunks. Do not access this list unless you know what you are doing.
- `Index` (Int32, get) — Gets the index of this chunk. The index is set by the owner of this chunk. Indices smaller than zero imply no index has been set. The combination of name+index is always unique among a set of chunks in the same litter.
- `ItemCount` (Int32, get) — Gets the number of items contained in this chunk.
- `Items` (List<GH_Item>, get) — Gets a pointer to the internal list of items. Do not access this list unless you know what you are doing.
- `Name` (String, get) — Gets the name of this chunk. The name is set by the owner of this chunk. Names must be at least 1 character long. The combination of name+index is always unique among a set of chunks in a single litter.

## GH_IReader (interface)

Provides access to a subset of GH_Chunk methods used for reading archives.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_IReader.htm)

### Methods
#### `void AddMessage(string m, GH_Message_Type t)`

Log a new message with the top-level archive. Messages are collected during read/write operations, and can be displayed to the user upon completion using GH_Archive.ShowMessageLog().

**Parameters:**
- `m` (System.String) — Message text.
- `t` (GH_IO.Serialization.GH_Message_Type) — Message type.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IChunk_AddMessage.htm)

#### `bool ChunkExists(string name)`

Checks whether a chunk with the specified name exists in the litter. Only chunks without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of chunk to test for.

**Returns:** `Boolean` — True if a chunk with the specified name exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_ChunkExists.htm)

#### `bool ChunkExists(string name, int index)`

Checks whether a chunk with the specified name and index exists in the litter. Only chunks with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of chunk to test for.
- `index` (System.Int32) — Index of chunk to test for. If less than zero, ChunkExists(string chunk_name) is called instead.

**Returns:** `Boolean` — True if a chunk with the specified name and index exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_ChunkExists_1.htm)

#### `GH_IReader FindChunk(string name)`

Finds the first chunk in the litter that matches the given name. Only chunks without index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of chunk to search for.

**Returns:** `GH_IReader` — The child chunk that matches the given name, or null of no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_FindChunk.htm)

#### `GH_IReader FindChunk(string name, int index)`

Finds the first chunk in the list that matches the given name and index. Only chunks with index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of chunk to search for.
- `index` (System.Int32) — Index of chunk to search for. If less than zero, FindChunk(string chunk_name) is called instead.

**Returns:** `GH_IReader` — The child chunk that matches the given name and index, or null of no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_FindChunk_1.htm)

#### `GH_Item FindItem(string name)`

Finds the first item that matches the given name. Only items without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to search for.

**Returns:** `GH_Item` — The item that matches the given name, or null of no matching item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_FindItem.htm)

#### `GH_Item FindItem(string name, int index)`

Finds the first item that matches the given name and index. Only items with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to search for.
- `index` (System.Int32) — Index of item to search for. If less than zero, then FindItem(string name) is called instead.

**Returns:** `GH_Item` — The item that matches the given name and index, or null of no matching item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_FindItem_1.htm)

#### `bool GetBoolean(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Boolean` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetBoolean.htm)

#### `bool GetBoolean(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Boolean` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetBoolean_1.htm)

#### `GH_BoundingBox GetBoundingBox(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_BoundingBox` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetBoundingBox.htm)

#### `GH_BoundingBox GetBoundingBox(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_BoundingBox` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetBoundingBox_1.htm)

#### `byte GetByte(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Byte` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetByte.htm)

#### `byte GetByte(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Byte` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetByte_1.htm)

#### `byte[] GetByteArray(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Byte[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetByteArray.htm)

#### `byte[] GetByteArray(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Byte[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetByteArray_1.htm)

#### `DateTime GetDate(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `DateTime` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDate.htm)

#### `DateTime GetDate(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `DateTime` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDate_1.htm)

#### `decimal GetDecimal(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Decimal` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDecimal.htm)

#### `decimal GetDecimal(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Decimal` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDecimal_1.htm)

#### `double GetDouble(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Double` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDouble.htm)

#### `double GetDouble(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Double` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDouble_1.htm)

#### `double[] GetDoubleArray(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Double[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDoubleArray.htm)

#### `double[] GetDoubleArray(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Double[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDoubleArray_1.htm)

#### `Bitmap GetDrawingBitmap(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Bitmap` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingBitmap.htm)

#### `Bitmap GetDrawingBitmap(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Bitmap` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingBitmap_1.htm)

#### `Color GetDrawingColor(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Color` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingColor.htm)

#### `Color GetDrawingColor(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Color` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingColor_1.htm)

#### `Point GetDrawingPoint(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Point` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingPoint.htm)

#### `Point GetDrawingPoint(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Point` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingPoint_1.htm)

#### `PointF GetDrawingPointF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `PointF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingPointF.htm)

#### `PointF GetDrawingPointF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `PointF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingPointF_1.htm)

#### `Rectangle GetDrawingRectangle(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Rectangle` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingRectangle.htm)

#### `Rectangle GetDrawingRectangle(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Rectangle` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingRectangle_1.htm)

#### `RectangleF GetDrawingRectangleF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `RectangleF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingRectangleF.htm)

#### `RectangleF GetDrawingRectangleF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `RectangleF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingRectangleF_1.htm)

#### `Size GetDrawingSize(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Size` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingSize.htm)

#### `Size GetDrawingSize(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Size` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingSize_1.htm)

#### `SizeF GetDrawingSizeF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `SizeF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingSizeF.htm)

#### `SizeF GetDrawingSizeF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `SizeF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetDrawingSizeF_1.htm)

#### `Guid GetGuid(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Guid` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetGuid.htm)

#### `Guid GetGuid(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Guid` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetGuid_1.htm)

#### `int GetInt32(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Int32` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInt32.htm)

#### `int GetInt32(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Int32` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInt32_1.htm)

#### `long GetInt64(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Int64` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInt64.htm)

#### `long GetInt64(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Int64` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInt64_1.htm)

#### `GH_Interval1D GetInterval1D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Interval1D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInterval1D.htm)

#### `GH_Interval1D GetInterval1D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Interval1D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInterval1D_1.htm)

#### `GH_Interval2D GetInterval2D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Interval2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInterval2D.htm)

#### `GH_Interval2D GetInterval2D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Interval2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetInterval2D_1.htm)

#### `GH_Line GetLine(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Line` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetLine.htm)

#### `GH_Line GetLine(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Line` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetLine_1.htm)

#### `string[] GetPath(string item_name, int item_index, string basePath)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `basePath` (System.String) — Base path for relative path resolution.

**Returns:** `String[]` — An array of path strings. If the resolved relative path is different from the stored absolute path, two strings will be returned.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPath.htm)

#### `string[] GetPath(string item_name, string basePath)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `basePath` (System.String) — Base path for relative path resolution.

**Returns:** `String[]` — An array of path strings. If the resolved relative path is different from the stored absolute path, two strings will be returned.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPath_1.htm)

#### `GH_Plane GetPlane(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Plane` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPlane.htm)

#### `GH_Plane GetPlane(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Plane` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPlane_1.htm)

#### `GH_Point2D GetPoint2D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPoint2D.htm)

#### `GH_Point2D GetPoint2D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPoint2D_1.htm)

#### `GH_Point3D GetPoint3D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point3D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPoint3D.htm)

#### `GH_Point3D GetPoint3D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point3D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPoint3D_1.htm)

#### `GH_Point4D GetPoint4D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point4D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPoint4D.htm)

#### `GH_Point4D GetPoint4D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point4D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetPoint4D_1.htm)

#### `float GetSingle(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Single` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetSingle.htm)

#### `float GetSingle(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Single` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetSingle_1.htm)

#### `string GetString(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `String` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetString.htm)

#### `string GetString(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `String` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetString_1.htm)

#### `GH_Version GetVersion(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Version` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetVersion.htm)

#### `GH_Version GetVersion(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Version` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_GetVersion_1.htm)

#### `bool ItemExists(string name)`

Checks whether an item with the specified name exists. Only items without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to test for.

**Returns:** `Boolean` — True if an item with the specified name exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_ItemExists.htm)

#### `bool ItemExists(string name, int index)`

Checks whether an item with the specified name and index exists. Only items with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to test for.
- `index` (System.Int32) — Index of item to test for. If index is less than zero, then ItemExists(string name) is called instead.

**Returns:** `Boolean` — True if an item with the specified name and index exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_ItemExists_1.htm)

#### `void Read(BinaryReader reader)`

Called when an object is required to deserialize itself.

**Parameters:**
- `reader` (System.IO.BinaryReader) — Reader object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Read.htm)

#### `void Read(XmlNode node)`

Called when an object is required to deserialize itself.

**Parameters:**
- `node` (System.Xml.XmlNode) — Node object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Read.htm)

#### `bool TryGetBoolean(string item_name, ref bool value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Boolean) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetBoolean.htm)

#### `bool TryGetBoolean(string item_name, int item_index, ref bool value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Boolean) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetBoolean_1.htm)

#### `bool TryGetBoundingBox(string item_name, ref GH_BoundingBox value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_BoundingBox) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetBoundingBox.htm)

#### `bool TryGetBoundingBox(string item_name, int item_index, ref GH_BoundingBox value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_BoundingBox) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetBoundingBox_1.htm)

#### `bool TryGetByte(string item_name, ref byte value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Byte) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetByte.htm)

#### `bool TryGetByte(string item_name, int item_index, ref byte value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Byte) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetByte_1.htm)

#### `bool TryGetDate(string item_name, ref DateTime value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.DateTime) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDate.htm)

#### `bool TryGetDate(string item_name, int item_index, ref DateTime value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.DateTime) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDate_1.htm)

#### `bool TryGetDecimal(string item_name, ref decimal value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Decimal) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDecimal.htm)

#### `bool TryGetDecimal(string item_name, int item_index, ref decimal value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Decimal) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDecimal_1.htm)

#### `bool TryGetDouble(string item_name, ref double value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Double) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDouble.htm)

#### `bool TryGetDouble(string item_name, int item_index, ref double value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Double) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDouble_1.htm)

#### `bool TryGetDrawingColor(string item_name, ref Color value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Color) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingColor.htm)

#### `bool TryGetDrawingColor(string item_name, int item_index, ref Color value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Color) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingColor_1.htm)

#### `bool TryGetDrawingPoint(string item_name, int item_index, ref Point value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Point) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingPoint_1.htm)

#### `bool TryGetDrawingPoint(string item_name, ref Point value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Point) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingPoint.htm)

#### `bool TryGetDrawingPointF(string item_name, int item_index, ref PointF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.PointF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingPointF_1.htm)

#### `bool TryGetDrawingPointF(string item_name, ref PointF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.PointF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingPointF.htm)

#### `bool TryGetDrawingRectangle(string item_name, int item_index, ref Rectangle value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Rectangle) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingRectangle_1.htm)

#### `bool TryGetDrawingRectangle(string item_name, ref Rectangle value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Rectangle) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingRectangle.htm)

#### `bool TryGetDrawingRectangleF(string item_name, int item_index, ref RectangleF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.RectangleF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingRectangleF_1.htm)

#### `bool TryGetDrawingRectangleF(string item_name, ref RectangleF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.RectangleF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingRectangleF.htm)

#### `bool TryGetDrawingSize(string item_name, int item_index, ref Size value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Size) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingSize_1.htm)

#### `bool TryGetDrawingSize(string item_name, ref Size value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Size) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingSize.htm)

#### `bool TryGetDrawingSizeF(string item_name, int item_index, ref SizeF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.SizeF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingSizeF_1.htm)

#### `bool TryGetDrawingSizeF(string item_name, ref SizeF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.SizeF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetDrawingSizeF.htm)

#### `bool TryGetGuid(string item_name, ref Guid value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Guid) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetGuid.htm)

#### `bool TryGetGuid(string item_name, int item_index, ref Guid value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Guid) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetGuid_1.htm)

#### `bool TryGetInt32(string item_name, ref int value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Int32) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInt32_1.htm)

#### `bool TryGetInt32(string item_name, int item_index, ref int value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Int32) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInt32.htm)

#### `bool TryGetInt64(string item_name, int item_index, ref long value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Int64) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInt64.htm)

#### `bool TryGetInt64(string item_name, ref long value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Int64) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInt64_1.htm)

#### `bool TryGetInterval1D(string item_name, ref GH_Interval1D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Interval1D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInterval1D.htm)

#### `bool TryGetInterval1D(string item_name, int item_index, ref GH_Interval1D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Interval1D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInterval1D_1.htm)

#### `bool TryGetInterval2D(string item_name, ref GH_Interval2D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Interval2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInterval2D.htm)

#### `bool TryGetInterval2D(string item_name, int item_index, ref GH_Interval2D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Interval2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetInterval2D_1.htm)

#### `bool TryGetLine(string item_name, ref GH_Line value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Line) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetLine.htm)

#### `bool TryGetLine(string item_name, int item_index, ref GH_Line value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Line) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetLine_1.htm)

#### `bool TryGetPlane(string item_name, ref GH_Plane value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Plane) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPlane.htm)

#### `bool TryGetPlane(string item_name, int item_index, ref GH_Plane value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Plane) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPlane_1.htm)

#### `bool TryGetPoint2D(string item_name, ref GH_Point2D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPoint2D.htm)

#### `bool TryGetPoint2D(string item_name, int item_index, ref GH_Point2D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPoint2D_1.htm)

#### `bool TryGetPoint3D(string item_name, ref GH_Point3D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point3D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPoint3D.htm)

#### `bool TryGetPoint3D(string item_name, int item_index, ref GH_Point3D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point3D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPoint3D_1.htm)

#### `bool TryGetPoint4D(string item_name, ref GH_Point4D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point4D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPoint4D.htm)

#### `bool TryGetPoint4D(string item_name, int item_index, ref GH_Point4D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point4D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetPoint4D_1.htm)

#### `bool TryGetSingle(string item_name, int item_index, ref float value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Single) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetSingle.htm)

#### `bool TryGetSingle(string item_name, ref float value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Single) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetSingle_1.htm)

#### `bool TryGetString(string item_name, int item_index, ref string value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.String) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetString.htm)

#### `bool TryGetString(string item_name, ref string value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.String) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetString_1.htm)

#### `bool TryGetVersion(string item_name, ref GH_Version value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Version) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetVersion.htm)

#### `bool TryGetVersion(string item_name, int item_index, ref GH_Version value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Version) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IReader_TryGetVersion_1.htm)

#### `void Write(BinaryWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Write.htm)

#### `void Write(XmlWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Write.htm)

### Properties
- `Archive` (GH_Archive, get) — Gets a pointer to the archive that owns the Root of the tree this chunk belongs to.
- `ArchiveLocation` (String, get) — Gets a string representing the URI with which the archive is associated. The location may be a null string.
- `ChunkCount` (Int32, get) — Gets the number of child chunks contained in this chunk. The set of all child chunks is referred to as a 'litter'.
- `Chunks` (List<GH_IChunk>, get) — Gets a pointer to the internal list of child chunks. Do not access this list unless you know what you are doing.
- `Index` (Int32, get) — Gets the index of this chunk. The index is set by the owner of this chunk. Indices smaller than zero imply no index has been set. The combination of name+index is always unique among a set of chunks in the same litter.
- `ItemCount` (Int32, get) — Gets the number of items contained in this chunk.
- `Items` (List<GH_Item>, get) — Gets a pointer to the internal list of items. Do not access this list unless you know what you are doing.
- `Name` (String, get) — Gets the name of this chunk. The name is set by the owner of this chunk. Names must be at least 1 character long. The combination of name+index is always unique among a set of chunks in a single litter.

## GH_IWriter (interface)

Provides access to a subset of GH_Chunk methods used for writing archives.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_IWriter.htm)

### Methods
#### `void AddComment(string comment_text)`

Adds a text comment to this chunk. Comments are serialized only if the output flavour is a human readable format. Comments are never deserialized, they are purely for the benefit of the humans reading the file data.

**Parameters:**
- `comment_text` (System.String) — The content of the comment, text might be altered if it contains invalid characters for a chosen format flavour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_AddComment.htm)

#### `void AddMessage(string m, GH_Message_Type t)`

Log a new message with the top-level archive. Messages are collected during read/write operations, and can be displayed to the user upon completion using GH_Archive.ShowMessageLog().

**Parameters:**
- `m` (System.String) — Message text.
- `t` (GH_IO.Serialization.GH_Message_Type) — Message type.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IChunk_AddMessage.htm)

#### `GH_IWriter CreateChunk(string chunk_name)`

Create a new child chunk with the specified name but without an index qualifier. If another chunk already exists with similar properties, an exception will be thrown.

**Parameters:**
- `chunk_name` (System.String) — Name of new child chunk.

**Returns:** `GH_IWriter` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_CreateChunk.htm)

#### `GH_IWriter CreateChunk(string chunk_name, int chunk_index)`

Create a new child chunk with the specified name and index qualifier. If another chunk already exists with similar properties, an exception will be thrown.

**Parameters:**
- `chunk_name` (System.String) — Name of new child.
- `chunk_index` (System.Int32) — Index of new child.

**Returns:** `GH_IWriter` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_CreateChunk_1.htm)

#### `void Read(BinaryReader reader)`

Called when an object is required to deserialize itself.

**Parameters:**
- `reader` (System.IO.BinaryReader) — Reader object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Read.htm)

#### `void Read(XmlNode node)`

Called when an object is required to deserialize itself.

**Parameters:**
- `node` (System.Xml.XmlNode) — Node object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Read.htm)

#### `bool RemoveChunk(GH_IChunk chunk)`

Remove the specified chunk from the litter.

**Parameters:**
- `chunk` (GH_IO.Serialization.GH_IChunk) — Chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if chunk is not part of this litter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_RemoveChunk.htm)

#### `bool RemoveChunk(string chunk_name)`

Remove the first chunk with a matching name. Only chunks without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `chunk_name` (System.String) — Name of chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_RemoveChunk_1.htm)

#### `bool RemoveChunk(string chunk_name, int chunk_index)`

Remove the first chunk with a matching name and index. Only chunks with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `chunk_name` (System.String) — Name of chunk to remove.
- `chunk_index` (System.Int32) — Index of chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_RemoveChunk_2.htm)

#### `bool RemoveItem(string itemName)`

Remove an unindexed item from this chunk.

**Parameters:**
- `itemName` (System.String) — Name of item.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_RemoveItem.htm)

#### `bool RemoveItem(string itemName, int itemIndex)`

Remove an indexed item from this chunk.

**Parameters:**
- `itemName` (System.String) — Name of item.
- `itemIndex` (System.Int32) — Index of item.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_RemoveItem_1.htm)

#### `void SetBoolean(string item_name, bool item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Boolean) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetBoolean.htm)

#### `void SetBoolean(string item_name, int item_index, bool item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Boolean) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetBoolean_1.htm)

#### `void SetBoundingBox(string item_name, GH_BoundingBox item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_BoundingBox) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetBoundingBox.htm)

#### `void SetBoundingBox(string item_name, int item_index, GH_BoundingBox item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_BoundingBox) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetBoundingBox_1.htm)

#### `void SetByte(string item_name, byte item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Byte) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetByte.htm)

#### `void SetByte(string item_name, int item_index, byte item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Byte) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetByte_1.htm)

#### `void SetByteArray(string item_name, byte[] item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Byte[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetByteArray.htm)

#### `void SetByteArray(string item_name, int item_index, byte[] item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Byte[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetByteArray_1.htm)

#### `void SetDate(string item_name, DateTime item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.DateTime) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDate.htm)

#### `void SetDate(string item_name, int item_index, DateTime item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.DateTime) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDate_1.htm)

#### `void SetDecimal(string item_name, decimal item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Decimal) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDecimal.htm)

#### `void SetDecimal(string item_name, int item_index, decimal item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Decimal) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDecimal_1.htm)

#### `void SetDouble(string item_name, double item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Double) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDouble.htm)

#### `void SetDouble(string item_name, int item_index, double item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Double) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDouble_1.htm)

#### `void SetDoubleArray(string item_name, double[] item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Double[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDoubleArray.htm)

#### `void SetDoubleArray(string item_name, int item_index, double[] item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Double[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDoubleArray_1.htm)

#### `void SetDrawingBitmap(string item_name, Bitmap item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Bitmap) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingBitmap.htm)

#### `void SetDrawingBitmap(string item_name, int item_index, Bitmap item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Bitmap) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingBitmap_1.htm)

#### `void SetDrawingColor(string item_name, Color item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Color) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingColor.htm)

#### `void SetDrawingColor(string item_name, int item_index, Color item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Color) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingColor_1.htm)

#### `void SetDrawingPoint(string item_name, int item_index, Point item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Point) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingPoint_1.htm)

#### `void SetDrawingPoint(string item_name, Point item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Point) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingPoint.htm)

#### `void SetDrawingPointF(string item_name, int item_index, PointF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.PointF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingPointF_1.htm)

#### `void SetDrawingPointF(string item_name, PointF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.PointF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingPointF.htm)

#### `void SetDrawingRectangle(string item_name, int item_index, Rectangle item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Rectangle) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingRectangle_1.htm)

#### `void SetDrawingRectangle(string item_name, Rectangle item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Rectangle) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingRectangle.htm)

#### `void SetDrawingRectangleF(string item_name, int item_index, RectangleF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.RectangleF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingRectangleF_1.htm)

#### `void SetDrawingRectangleF(string item_name, RectangleF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.RectangleF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingRectangleF.htm)

#### `void SetDrawingSize(string item_name, int item_index, Size item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Size) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingSize_1.htm)

#### `void SetDrawingSize(string item_name, Size item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Size) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingSize.htm)

#### `void SetDrawingSizeF(string item_name, int item_index, SizeF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.SizeF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingSizeF_1.htm)

#### `void SetDrawingSizeF(string item_name, SizeF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.SizeF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetDrawingSizeF.htm)

#### `void SetGuid(string item_name, Guid item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Guid) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetGuid.htm)

#### `void SetGuid(string item_name, int item_index, Guid item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Guid) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetGuid_1.htm)

#### `void SetInt32(string item_name, int item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Int32) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInt32.htm)

#### `void SetInt32(string item_name, int item_index, int item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Int32) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInt32_1.htm)

#### `void SetInt64(string item_name, int item_index, long item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Int64) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInt64.htm)

#### `void SetInt64(string item_name, long item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Int64) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInt64_1.htm)

#### `void SetInterval1D(string item_name, GH_Interval1D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Interval1D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInterval1D.htm)

#### `void SetInterval1D(string item_name, int item_index, GH_Interval1D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Interval1D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInterval1D_1.htm)

#### `void SetInterval2D(string item_name, GH_Interval2D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Interval2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInterval2D.htm)

#### `void SetInterval2D(string item_name, int item_index, GH_Interval2D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Interval2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetInterval2D_1.htm)

#### `void SetLine(string item_name, GH_Line item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Line) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetLine.htm)

#### `void SetLine(string item_name, int item_index, GH_Line item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Line) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetLine_1.htm)

#### `void SetPath(string item_name, int item_index, string absolutePath, string basePath)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `absolutePath` (System.String) — Absolute path to store.
- `basePath` (System.String) — Base path. This will be used to also store a relative path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPath.htm)

#### `void SetPath(string item_name, string absolutePath, string basePath)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `absolutePath` (System.String) — Absolute path to store.
- `basePath` (System.String) — Base path. This will be used to also store a relative path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPath_1.htm)

#### `void SetPlane(string item_name, GH_Plane item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Plane) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPlane.htm)

#### `void SetPlane(string item_name, int item_index, GH_Plane item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Plane) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPlane_1.htm)

#### `void SetPoint2D(string item_name, GH_Point2D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPoint2D.htm)

#### `void SetPoint2D(string item_name, int item_index, GH_Point2D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPoint2D_1.htm)

#### `void SetPoint3D(string item_name, GH_Point3D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point3D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPoint3D.htm)

#### `void SetPoint3D(string item_name, int item_index, GH_Point3D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point3D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPoint3D_1.htm)

#### `void SetPoint4D(string item_name, GH_Point4D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point4D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPoint4D.htm)

#### `void SetPoint4D(string item_name, int item_index, GH_Point4D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point4D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetPoint4D_1.htm)

#### `void SetSingle(string item_name, int item_index, float item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Single) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetSingle.htm)

#### `void SetSingle(string item_name, float item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Single) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetSingle_1.htm)

#### `void SetString(string item_name, int item_index, string item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.String) — Value of item to add. If a null String is supplied, a zero-length String will be serialized.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetString.htm)

#### `void SetString(string item_name, string item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.String) — Value of item to add. If a null String is supplied, a zero-length String will be serialized.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetString_1.htm)

#### `void SetVersion(string item_name, GH_Version item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Version) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetVersion.htm)

#### `void SetVersion(string item_name, int item_index, GH_Version item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Version) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetVersion_1.htm)

#### `void SetVersion(string item_name, int major, int minor, int revision)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `major` (System.Int32) — Major component of version structure.
- `minor` (System.Int32) — Minor component of version structure.
- `revision` (System.Int32) — Revision component of version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetVersion_2.htm)

#### `void SetVersion(string item_name, int item_index, int major, int minor, int revision)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `major` (System.Int32) — Major component of version structure.
- `minor` (System.Int32) — Minor component of version structure.
- `revision` (System.Int32) — Revision component of version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IWriter_SetVersion_3.htm)

#### `void Write(BinaryWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IBinarySupport_Write.htm)

#### `void Write(XmlWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Write.htm)

### Properties
- `Archive` (GH_Archive, get) — Gets a pointer to the archive that owns the Root of the tree this chunk belongs to.
- `ArchiveLocation` (String, get) — Gets a string representing the URI with which the archive is associated. The location may be a null string.
- `ChunkCount` (Int32, get) — Gets the number of child chunks contained in this chunk. The set of all child chunks is referred to as a 'litter'.
- `Chunks` (List<GH_IChunk>, get) — Gets a pointer to the internal list of child chunks. Do not access this list unless you know what you are doing.
- `Index` (Int32, get) — Gets the index of this chunk. The index is set by the owner of this chunk. Indices smaller than zero imply no index has been set. The combination of name+index is always unique among a set of chunks in the same litter.
- `ItemCount` (Int32, get) — Gets the number of items contained in this chunk.
- `Items` (List<GH_Item>, get) — Gets a pointer to the internal list of items. Do not access this list unless you know what you are doing.
- `Name` (String, get) — Gets the name of this chunk. The name is set by the owner of this chunk. Names must be at least 1 character long. The combination of name+index is always unique among a set of chunks in a single litter.

## GH_IXmlSupport (interface)

Interface which declares all methods required for objects that can be (de)serialized to and from an Xml archive.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_IXmlSupport.htm)

### Methods
#### `void Read(XmlNode node)`

Called when an object is required to deserialize itself.

**Parameters:**
- `node` (System.Xml.XmlNode) — Node object to be used for deserialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Read.htm)

#### `void Write(XmlWriter writer)`

Called when an object is required to serialize itself.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Writer object to be used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_IXmlSupport_Write.htm)

## GH_LooseChunk (class)

A utility class for creating partial archives.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_LooseChunk.htm)

### Constructors
- `public GH_LooseChunk(string chunk_name)` — Create a new loose chunk. A loose chunk does not have an owner archive and thus cannot store runtime messages.
- `public GH_LooseChunk(string chunk_name, int chunk_index)` — Create a new loose chunk. A loose chunk does not have an owner archive and thus cannot store runtime messages.

### Methods
#### `public void AddComment(string comment_text)`

Adds a text comment to this chunk. Comments are serialized only if the output flavour is a human readable format. Comments are never deserialized, they are purely for the benefit of the humans reading the file data.

**Parameters:**
- `comment_text` (System.String) — The content of the comment, text might be altered if it contains invalid characters for a chosen format flavour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_AddComment.htm)

#### `public void AddMessage(string m, GH_Message_Type t)`

Log a new message with the top-level archive. Messages are collected during read/write operations, and can be displayed to the user upon completion using GH_Archive.ShowMessageLog().

**Parameters:**
- `m` (System.String) — Message text.
- `t` (GH_IO.Serialization.GH_Message_Type) — Message type.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_AddMessage.htm)

#### `public bool ChunkExists(string name)`

Checks whether a chunk with the specified name exists in the litter. Only chunks without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of chunk to test for.

**Returns:** `Boolean` — True if a chunk with the specified name exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkExists.htm)

#### `public bool ChunkExists(string name, int index)`

Checks whether a chunk with the specified name and index exists in the litter. Only chunks with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of chunk to test for.
- `index` (System.Int32) — Index of chunk to test for. If less than zero, ChunkExists(string name) is called instead.

**Returns:** `Boolean` — True if a chunk with the specified name and index exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ChunkExists_1.htm)

#### `public void CopyValuesFromChunk(GH_Chunk other)`

Copy all values and sub-chunks from another chunk.

**Parameters:**
- `other` (GH_IO.Serialization.GH_Chunk)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CopyValuesFromChunk.htm)

#### `public GH_Chunk CopyValuesToChunk()`

Copy all values and sub-chunks in this chunk to another chunk which does not point to the same archive.

**Returns:** `GH_Chunk` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CopyValuesToChunk.htm)

#### `public GH_IWriter CreateChunk(string chunk_name)`

Create a new child chunk with the specified name and without an index qualifier. If another chunk already exists with similar properties, an exception will be thrown.

**Parameters:**
- `chunk_name` (System.String) — Name of new child.

**Returns:** `GH_IWriter` — The newly created child chunk.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CreateChunk.htm)

#### `public GH_IWriter CreateChunk(string chunk_name, int chunk_index)`

Create a new child chunk with the specified name and index qualifier. If another chunk already exists with similar properties, an exception will be thrown.

**Parameters:**
- `chunk_name` (System.String) — Name of new child.
- `chunk_index` (System.Int32) — Index of new child.

**Returns:** `GH_IWriter` — The newly created child chunk.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_CreateChunk_1.htm)

#### `public void Deserialize_Binary(byte[] content)`

Deserializes a byte array.

**Parameters:**
- `content` (System.Byte[]) — Byte array to deserialize.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_LooseChunk_Deserialize_Binary.htm)

#### `public void Deserialize_Xml(string xml_content)`

Deserializes an Xml string.

**Parameters:**
- `xml_content` (System.String) — Xml to deserialize.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_LooseChunk_Deserialize_Xml.htm)

#### `public GH_IReader FindChunk(string name)`

Finds the first chunk in the litter that matches the given name. Only chunks without index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of chunk to search for.

**Returns:** `GH_IReader` — The child chunk that matches the given name, or null of no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindChunk.htm)

#### `public GH_IReader FindChunk(string name, int index)`

Finds the first chunk in the list that matches the given name and index. Only chunks with index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of chunk to search for.
- `index` (System.Int32) — Index of chunk to search for. If less than zero, FindChunk(string chunk_name) is called instead.

**Returns:** `GH_IReader` — The child chunk that matches the given name and index, or null of no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindChunk_1.htm)

#### `public GH_Item FindItem(string name)`

Finds the first item that matches the given name. Only items without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to search for.

**Returns:** `GH_Item` — The item that matches the given name, or null of no matching item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindItem.htm)

#### `public GH_Item FindItem(string name, int index)`

Finds the first item that matches the given name and index. Only items with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to search for.
- `index` (System.Int32) — Index of item to search for. If less than zero, then FindItem(string name) is called instead.

**Returns:** `GH_Item` — The item that matches the given name and index, or null of no matching item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_FindItem_1.htm)

#### `public bool GetBoolean(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Boolean` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoolean.htm)

#### `public bool GetBoolean(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Boolean` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoolean_1.htm)

#### `public GH_BoundingBox GetBoundingBox(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_BoundingBox` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoundingBox.htm)

#### `public GH_BoundingBox GetBoundingBox(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_BoundingBox` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetBoundingBox_1.htm)

#### `public byte GetByte(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Byte` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByte.htm)

#### `public byte GetByte(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Byte` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByte_1.htm)

#### `public byte[] GetByteArray(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Byte[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByteArray.htm)

#### `public byte[] GetByteArray(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Byte[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetByteArray_1.htm)

#### `public DateTime GetDate(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `DateTime` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDate.htm)

#### `public DateTime GetDate(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `DateTime` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDate_1.htm)

#### `public decimal GetDecimal(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Decimal` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDecimal.htm)

#### `public decimal GetDecimal(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Decimal` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDecimal_1.htm)

#### `public double GetDouble(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Double` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDouble.htm)

#### `public double GetDouble(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Double` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDouble_1.htm)

#### `public double[] GetDoubleArray(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Double[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDoubleArray.htm)

#### `public double[] GetDoubleArray(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Double[]` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDoubleArray_1.htm)

#### `public Bitmap GetDrawingBitmap(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Bitmap` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingBitmap.htm)

#### `public Bitmap GetDrawingBitmap(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Bitmap` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingBitmap_1.htm)

#### `public Color GetDrawingColor(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Color` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingColor.htm)

#### `public Color GetDrawingColor(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Color` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingColor_1.htm)

#### `public Point GetDrawingPoint(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Point` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPoint.htm)

#### `public Point GetDrawingPoint(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Point` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPoint_1.htm)

#### `public PointF GetDrawingPointF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `PointF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPointF.htm)

#### `public PointF GetDrawingPointF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `PointF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingPointF_1.htm)

#### `public Rectangle GetDrawingRectangle(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Rectangle` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangle.htm)

#### `public Rectangle GetDrawingRectangle(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Rectangle` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangle_1.htm)

#### `public RectangleF GetDrawingRectangleF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `RectangleF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangleF.htm)

#### `public RectangleF GetDrawingRectangleF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `RectangleF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingRectangleF_1.htm)

#### `public Size GetDrawingSize(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Size` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSize.htm)

#### `public Size GetDrawingSize(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Size` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSize_1.htm)

#### `public SizeF GetDrawingSizeF(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `SizeF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSizeF.htm)

#### `public SizeF GetDrawingSizeF(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `SizeF` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetDrawingSizeF_1.htm)

#### `public Guid GetGuid(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Guid` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetGuid.htm)

#### `public Guid GetGuid(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Guid` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetGuid_1.htm)

#### `public int GetInt32(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Int32` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt32.htm)

#### `public int GetInt32(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Int32` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt32_1.htm)

#### `public long GetInt64(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Int64` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt64.htm)

#### `public long GetInt64(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Int64` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInt64_1.htm)

#### `public GH_Interval1D GetInterval1D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Interval1D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval1D.htm)

#### `public GH_Interval1D GetInterval1D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Interval1D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval1D_1.htm)

#### `public GH_Interval2D GetInterval2D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Interval2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval2D.htm)

#### `public GH_Interval2D GetInterval2D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Interval2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetInterval2D_1.htm)

#### `public GH_Line GetLine(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Line` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetLine.htm)

#### `public GH_Line GetLine(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Line` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetLine_1.htm)

#### `public string[] GetPath(string item_name, int item_index, string basePath)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `basePath` (System.String) — Base path for relative path resolution.

**Returns:** `String[]` — An array of path strings. If the resolved relative path is different from the stored absolute path, an array of two strings will be returned [absolute, relative].

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPath.htm)

#### `public string[] GetPath(string item_name, string basePath)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `basePath` (System.String) — Base path for relative path resolution.

**Returns:** `String[]` — An array of path strings. If the resolved relative path is different from the stored absolute path, an array of two strings will be returned [absolute, relative].

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPath_1.htm)

#### `public GH_Plane GetPlane(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Plane` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPlane.htm)

#### `public GH_Plane GetPlane(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Plane` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPlane_1.htm)

#### `public GH_Point2D GetPoint2D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint2D.htm)

#### `public GH_Point2D GetPoint2D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point2D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint2D_1.htm)

#### `public GH_Point3D GetPoint3D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point3D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint3D.htm)

#### `public GH_Point3D GetPoint3D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point3D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint3D_1.htm)

#### `public GH_Point4D GetPoint4D(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Point4D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint4D.htm)

#### `public GH_Point4D GetPoint4D(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Point4D` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetPoint4D_1.htm)

#### `public float GetSingle(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `Single` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetSingle.htm)

#### `public float GetSingle(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `Single` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetSingle_1.htm)

#### `public string GetString(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `String` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetString.htm)

#### `public string GetString(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `String` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetString_1.htm)

#### `public GH_Version GetVersion(string item_name)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.

**Returns:** `GH_Version` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetVersion.htm)

#### `public GH_Version GetVersion(string item_name, int item_index)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.

**Returns:** `GH_Version` — The inner value of the item.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_GetVersion_1.htm)

#### `public bool ItemExists(string name)`

Gets the occupancy for a specific item name. Only items without index qualifiers are considered.

**Parameters:**
- `name` (System.String) — Name of item to check for.

**Returns:** `Boolean` — True if an item with a similar name already exists, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ItemExists.htm)

#### `public bool ItemExists(string name, int index)`

Checks whether an item with the specified name and index exists. Only items with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `name` (System.String) — Name of item to test for.
- `index` (System.Int32) — Index of item to test for. If index is less than zero, then ItemExists(string name) is called instead.

**Returns:** `Boolean` — True if an item with the specified name and index exists, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_ItemExists_1.htm)

#### `public void Read(BinaryReader reader)`

Read this chunk and all child chunks from a binary stream.

**Parameters:**
- `reader` (System.IO.BinaryReader) — The Binary reader to use, cannot be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Read.htm)

#### `public void Read(XmlNode node)`

Read this chunk and all child chunks from an Xml node.

**Parameters:**
- `node` (System.Xml.XmlNode) — The Xml node to deserialize from, cannot be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Read_1.htm)

#### `public bool RemoveChunk(GH_IChunk chunk)`

Remove the specified chunk from the litter.

**Parameters:**
- `chunk` (GH_IO.Serialization.GH_IChunk) — Chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if chunk is not part of this litter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveChunk.htm)

#### `public bool RemoveChunk(string chunk_name)`

Remove the first chunk with a matching name. Only chunks without index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `chunk_name` (System.String) — Name of chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveChunk_1.htm)

#### `public bool RemoveChunk(string chunk_name, int chunk_index)`

Remove the first chunk with a matching name and index. Only chunks with index qualifiers are considered. Name comparisons are not case-sensitive.

**Parameters:**
- `chunk_name` (System.String) — Name of chunk to remove.
- `chunk_index` (System.Int32) — Index of chunk to remove.

**Returns:** `Boolean` — True is chunk was removed, false if no matching chunk could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveChunk_2.htm)

#### `public bool RemoveItem(string itemName)`

Remove an unindexed item from this chunk.

**Parameters:**
- `itemName` (System.String) — Name of item.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveItem.htm)

#### `public bool RemoveItem(string itemName, int itemIndex)`

Remove an indexed item from this chunk.

**Parameters:**
- `itemName` (System.String) — Name of item.
- `itemIndex` (System.Int32) — Index of item.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_RemoveItem_1.htm)

#### `public byte[] Serialize_Binary()`

Serializes the data tree into a byte array.

**Returns:** `Byte[]` — An array of bytes representing the Loose chunk.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_LooseChunk_Serialize_Binary.htm)

#### `public string Serialize_Xml()`

Serializes the data tree into an Xml string.

**Returns:** `String` — A string containing the Xml content.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_LooseChunk_Serialize_Xml.htm)

#### `public void SetBoolean(string item_name, bool item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Boolean) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoolean.htm)

#### `public void SetBoolean(string item_name, int item_index, bool item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Boolean) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoolean_1.htm)

#### `public void SetBoundingBox(string item_name, GH_BoundingBox item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_BoundingBox) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoundingBox.htm)

#### `public void SetBoundingBox(string item_name, int item_index, GH_BoundingBox item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_BoundingBox) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetBoundingBox_1.htm)

#### `public void SetByte(string item_name, byte item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Byte) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByte.htm)

#### `public void SetByte(string item_name, int item_index, byte item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Byte) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByte_1.htm)

#### `public void SetByteArray(string item_name, byte[] item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Byte[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByteArray.htm)

#### `public void SetByteArray(string item_name, int item_index, byte[] item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Byte[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetByteArray_1.htm)

#### `public void SetDate(string item_name, DateTime item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.DateTime) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDate.htm)

#### `public void SetDate(string item_name, int item_index, DateTime item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.DateTime) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDate_1.htm)

#### `public void SetDecimal(string item_name, decimal item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Decimal) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDecimal.htm)

#### `public void SetDecimal(string item_name, int item_index, decimal item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Decimal) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDecimal_1.htm)

#### `public void SetDouble(string item_name, double item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Double) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDouble.htm)

#### `public void SetDouble(string item_name, int item_index, double item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Double) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDouble_1.htm)

#### `public void SetDoubleArray(string item_name, double[] item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Double[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDoubleArray.htm)

#### `public void SetDoubleArray(string item_name, int item_index, double[] item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Double[]) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDoubleArray_1.htm)

#### `public void SetDrawingBitmap(string item_name, Bitmap item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Bitmap) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingBitmap.htm)

#### `public void SetDrawingBitmap(string item_name, int item_index, Bitmap item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Bitmap) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingBitmap_1.htm)

#### `public void SetDrawingColor(string item_name, Color item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Color) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingColor.htm)

#### `public void SetDrawingColor(string item_name, int item_index, Color item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Color) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingColor_1.htm)

#### `public void SetDrawingPoint(string item_name, int item_index, Point item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Point) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPoint_1.htm)

#### `public void SetDrawingPoint(string item_name, Point item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Point) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPoint.htm)

#### `public void SetDrawingPointF(string item_name, int item_index, PointF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.PointF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPointF_1.htm)

#### `public void SetDrawingPointF(string item_name, PointF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.PointF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingPointF.htm)

#### `public void SetDrawingRectangle(string item_name, int item_index, Rectangle item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Rectangle) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangle_1.htm)

#### `public void SetDrawingRectangle(string item_name, Rectangle item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Rectangle) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangle.htm)

#### `public void SetDrawingRectangleF(string item_name, int item_index, RectangleF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.RectangleF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangleF_1.htm)

#### `public void SetDrawingRectangleF(string item_name, RectangleF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.RectangleF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingRectangleF.htm)

#### `public void SetDrawingSize(string item_name, int item_index, Size item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.Size) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSize_1.htm)

#### `public void SetDrawingSize(string item_name, Size item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.Size) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSize.htm)

#### `public void SetDrawingSizeF(string item_name, int item_index, SizeF item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Drawing.SizeF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSizeF_1.htm)

#### `public void SetDrawingSizeF(string item_name, SizeF item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Drawing.SizeF) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetDrawingSizeF.htm)

#### `public void SetGuid(string item_name, Guid item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Guid) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetGuid.htm)

#### `public void SetGuid(string item_name, int item_index, Guid item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Guid) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetGuid_1.htm)

#### `public void SetInt32(string item_name, int item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Int32) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt32.htm)

#### `public void SetInt32(string item_name, int item_index, int item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Int32) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt32_1.htm)

#### `public void SetInt64(string item_name, int item_index, long item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Int64) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt64.htm)

#### `public void SetInt64(string item_name, long item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Int64) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInt64_1.htm)

#### `public void SetInterval1D(string item_name, GH_Interval1D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Interval1D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval1D.htm)

#### `public void SetInterval1D(string item_name, int item_index, GH_Interval1D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Interval1D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval1D_1.htm)

#### `public void SetInterval2D(string item_name, GH_Interval2D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Interval2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval2D.htm)

#### `public void SetInterval2D(string item_name, int item_index, GH_Interval2D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Interval2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetInterval2D_1.htm)

#### `public void SetLine(string item_name, GH_Line item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Line) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetLine.htm)

#### `public void SetLine(string item_name, int item_index, GH_Line item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Line) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetLine_1.htm)

#### `public void SetPath(string item_name, int item_index, string absolutePath, string basePath)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `absolutePath` (System.String) — Absolute path to store.
- `basePath` (System.String) — Base path. This will be used to also store a relative path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPath.htm)

#### `public void SetPath(string item_name, string absolutePath, string basePath)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `absolutePath` (System.String) — Absolute path to store.
- `basePath` (System.String) — Base path. This will be used to also store a relative path.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPath_1.htm)

#### `public void SetPlane(string item_name, GH_Plane item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Plane) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPlane.htm)

#### `public void SetPlane(string item_name, int item_index, GH_Plane item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Plane) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPlane_1.htm)

#### `public void SetPoint2D(string item_name, GH_Point2D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint2D.htm)

#### `public void SetPoint2D(string item_name, int item_index, GH_Point2D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point2D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint2D_1.htm)

#### `public void SetPoint3D(string item_name, GH_Point3D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point3D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint3D.htm)

#### `public void SetPoint3D(string item_name, int item_index, GH_Point3D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point3D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint3D_1.htm)

#### `public void SetPoint4D(string item_name, GH_Point4D item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Point4D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint4D.htm)

#### `public void SetPoint4D(string item_name, int item_index, GH_Point4D item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Point4D) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetPoint4D_1.htm)

#### `public void SetSingle(string item_name, int item_index, float item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.Single) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetSingle.htm)

#### `public void SetSingle(string item_name, float item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.Single) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetSingle_1.htm)

#### `public void SetString(string item_name, int item_index, string item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (System.String) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetString.htm)

#### `public void SetString(string item_name, string item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (System.String) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetString_1.htm)

#### `public void SetVersion(string item_name, GH_Version item_value)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_value` (GH_IO.Types.GH_Version) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion.htm)

#### `public void SetVersion(string item_name, int item_index, GH_Version item_value)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `item_value` (GH_IO.Types.GH_Version) — Value of item to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion_1.htm)

#### `public void SetVersion(string item_name, int major, int minor, int revision)`

Add a new data item to this chunk. The name must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `major` (System.Int32) — Major component of Version structure.
- `minor` (System.Int32) — Minor component of Version structure.
- `revision` (System.Int32) — Revision component of Version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion_2.htm)

#### `public void SetVersion(string item_name, int item_index, int major, int minor, int revision)`

Add a new data item to this chunk. The combination of name and index must be unique or an exception will be thrown.

**Parameters:**
- `item_name` (System.String) — Name of item to add.
- `item_index` (System.Int32) — Index of item to add.
- `major` (System.Int32) — Major component of Version structure.
- `minor` (System.Int32) — Minor component of Version structure.
- `revision` (System.Int32) — Revision component of Version structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_SetVersion_3.htm)

#### `public bool TryGetBoolean(string item_name, ref bool value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Boolean) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoolean.htm)

#### `public bool TryGetBoolean(string item_name, int item_index, ref bool value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Boolean) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoolean_1.htm)

#### `public bool TryGetBoundingBox(string item_name, ref GH_BoundingBox value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_BoundingBox) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoundingBox.htm)

#### `public bool TryGetBoundingBox(string item_name, int item_index, ref GH_BoundingBox value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_BoundingBox) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetBoundingBox_1.htm)

#### `public bool TryGetByte(string item_name, ref byte value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Byte) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetByte.htm)

#### `public bool TryGetByte(string item_name, int item_index, ref byte value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Byte) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetByte_1.htm)

#### `public bool TryGetDate(string item_name, ref DateTime value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.DateTime) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDate.htm)

#### `public bool TryGetDate(string item_name, int item_index, ref DateTime value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.DateTime) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDate_1.htm)

#### `public bool TryGetDecimal(string item_name, ref decimal value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Decimal) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDecimal.htm)

#### `public bool TryGetDecimal(string item_name, int item_index, ref decimal value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Decimal) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDecimal_1.htm)

#### `public bool TryGetDouble(string item_name, ref double value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Double) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDouble.htm)

#### `public bool TryGetDouble(string item_name, int item_index, ref double value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Double) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDouble_1.htm)

#### `public bool TryGetDrawingColor(string item_name, ref Color value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Color) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingColor.htm)

#### `public bool TryGetDrawingColor(string item_name, int item_index, ref Color value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Color) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingColor_1.htm)

#### `public bool TryGetDrawingPoint(string item_name, int item_index, ref Point value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Point) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPoint_1.htm)

#### `public bool TryGetDrawingPoint(string item_name, ref Point value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Point) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPoint.htm)

#### `public bool TryGetDrawingPointF(string item_name, int item_index, ref PointF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.PointF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPointF_1.htm)

#### `public bool TryGetDrawingPointF(string item_name, ref PointF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.PointF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingPointF.htm)

#### `public bool TryGetDrawingRectangle(string item_name, int item_index, ref Rectangle value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Rectangle) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangle_1.htm)

#### `public bool TryGetDrawingRectangle(string item_name, ref Rectangle value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Rectangle) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangle.htm)

#### `public bool TryGetDrawingRectangleF(string item_name, int item_index, ref RectangleF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.RectangleF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangleF_1.htm)

#### `public bool TryGetDrawingRectangleF(string item_name, ref RectangleF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.RectangleF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingRectangleF.htm)

#### `public bool TryGetDrawingSize(string item_name, int item_index, ref Size value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.Size) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSize_1.htm)

#### `public bool TryGetDrawingSize(string item_name, ref Size value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.Size) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSize.htm)

#### `public bool TryGetDrawingSizeF(string item_name, int item_index, ref SizeF value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Drawing.SizeF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSizeF_1.htm)

#### `public bool TryGetDrawingSizeF(string item_name, ref SizeF value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Drawing.SizeF) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetDrawingSizeF.htm)

#### `public bool TryGetGuid(string item_name, ref Guid value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Guid) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetGuid.htm)

#### `public bool TryGetGuid(string item_name, int item_index, ref Guid value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Guid) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetGuid_1.htm)

#### `public bool TryGetInt32(string item_name, ref int value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Int32) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt32_1.htm)

#### `public bool TryGetInt32(string item_name, int item_index, ref int value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Int32) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt32.htm)

#### `public bool TryGetInt64(string item_name, int item_index, ref long value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Int64) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt64.htm)

#### `public bool TryGetInt64(string item_name, ref long value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Int64) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInt64_1.htm)

#### `public bool TryGetInterval1D(string item_name, ref GH_Interval1D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Interval1D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval1D.htm)

#### `public bool TryGetInterval1D(string item_name, int item_index, ref GH_Interval1D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Interval1D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval1D_1.htm)

#### `public bool TryGetInterval2D(string item_name, ref GH_Interval2D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Interval2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval2D.htm)

#### `public bool TryGetInterval2D(string item_name, int item_index, ref GH_Interval2D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Interval2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetInterval2D_1.htm)

#### `public bool TryGetLine(string item_name, ref GH_Line value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Line) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetLine.htm)

#### `public bool TryGetLine(string item_name, int item_index, ref GH_Line value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Line) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetLine_1.htm)

#### `public bool TryGetPlane(string item_name, ref GH_Plane value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Plane) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPlane.htm)

#### `public bool TryGetPlane(string item_name, int item_index, ref GH_Plane value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Plane) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPlane_1.htm)

#### `public bool TryGetPoint2D(string item_name, ref GH_Point2D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint2D.htm)

#### `public bool TryGetPoint2D(string item_name, int item_index, ref GH_Point2D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point2D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint2D_1.htm)

#### `public bool TryGetPoint3D(string item_name, ref GH_Point3D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point3D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint3D.htm)

#### `public bool TryGetPoint3D(string item_name, int item_index, ref GH_Point3D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point3D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint3D_1.htm)

#### `public bool TryGetPoint4D(string item_name, ref GH_Point4D value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Point4D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint4D.htm)

#### `public bool TryGetPoint4D(string item_name, int item_index, ref GH_Point4D value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Point4D) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetPoint4D_1.htm)

#### `public bool TryGetSingle(string item_name, int item_index, ref float value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.Single) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetSingle.htm)

#### `public bool TryGetSingle(string item_name, ref float value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.Single) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetSingle_1.htm)

#### `public bool TryGetString(string item_name, int item_index, ref string value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (System.String) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetString.htm)

#### `public bool TryGetString(string item_name, ref string value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (System.String) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetString_1.htm)

#### `public bool TryGetVersion(string item_name, ref GH_Version value)`

Gets the value of the item with the specified name. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `value` (GH_IO.Types.GH_Version) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetVersion.htm)

#### `public bool TryGetVersion(string item_name, int item_index, ref GH_Version value)`

Gets the value of the item with the specified name and index. Name comparison is not case-sensitive.

**Parameters:**
- `item_name` (System.String) — Name of item to retrieve.
- `item_index` (System.Int32) — Index of item to retrieve.
- `value` (GH_IO.Types.GH_Version) — Target of assignment.

**Returns:** `Boolean` — True if the value was set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_TryGetVersion_1.htm)

#### `public void Write(BinaryWriter writer)`

Write this chunk and all child chunks to a binary stream.

**Parameters:**
- `writer` (System.IO.BinaryWriter) — The Binary writer to use, cannot be null.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Write.htm)

#### `public void Write(XmlWriter writer)`

Serialize this chunk into an Xml stream.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — Xml writer used for serialization.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_GH_Chunk_Write_1.htm)

### Properties
- `Archive` (GH_Archive, get) — Gets a pointer to the archive that owns the Root of the tree this chunk belongs to.
- `ArchiveLocation` (String, get) — Gets a string representing the URI with which the archive is associated. The location may be a null string.
- `ChunkCount` (Int32, get) — Gets the number of child chunks contained in this chunk. The set of all child chunks is referred to as a 'litter'.
- `Chunks` (List<GH_IChunk>, get) — Gets a pointer to the internal list of child chunks. Do not access this list unless you know what you are doing.
- `HasComments` (Boolean, get) — Gets a value that indicates whether or not comments have been stored in this chunk.
- `HasIndex` (Boolean, get) — Gets the index existence implication. The item is considered to have an index qualifier if the index value is larger than or equal to zero.
- `HasName` (Boolean, get) — Gets the name validity of this item. The item is considered to have an invalid name if string.IsNullOrEmpty(name)
- `Index` (Int32, get) — Gets the index of this chunk. The index is set by the owner of this chunk. Indices smaller than zero imply no index has been set. The combination of name+index is always unique among a set of chunks in the same litter.
- `ItemCount` (Int32, get) — Gets the number of items contained in this chunk.
- `Items` (List<GH_Item>, get) — Gets a pointer to the internal list of items. Do not access this list unless you know what you are doing.
- `Name` (String, get) — Gets the name of this chunk. The name is set by the owner of this chunk. Names must be at least 1 character long. The combination of name+index is always unique among a set of chunks in a single litter.
- `m_archive` (GH_Archive, get) — Pointer to the archive that owns the Root of the tree this chunk belongs to.
- `m_chunks` (GH_Chunk.ChunkKeyedCollection, get) — Dictionary of sub-chunks contained within this chunk.
- `m_comments` (List<String>, get) — List of text comments. This list is automatically instantiated once the first comment is added.
- `m_index` (Int32, get) — Index of this chunk. This field is set only once, during construction. A negative index indicates no index qualifier has been set.
- `m_items` (SortedDictionary<ID,GH_Item>, get) — The list of nodes contained within this chunk.
- `m_name` (String, get) — Name of this chunk. This field is set only once, during construction.

## GH_Message (class)

Represents an archive log message. Messages are collected during read/write operations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_Message.htm)

### Constructors
- `public GH_Message()` — Fake constructor to avoid new instances being created by a PropertyGrid.
- `public GH_Message(string message_content)` — Create a new message of type GH_Message_Type.info
- `public GH_Message(string message_content, GH_Message_Type message_type)` — Create a new message.

### Properties
- `Message` (String, get) — Gets the text content of this message.
- `Type` (GH_Message_Type, get) — Gets the type of this message.

## GH_Message_Type (enum)

Message type flag.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_GH_Message_Type.htm)

### Values
- `info` = `0` — Indicates a message represents information.
- `warning` = `1` — Indicates the message represents a warning. Warnings are not severe enough to be regarded as IO errors.
- `error` = `2` — Indicates the message represents an error. Errors mean (de)serialization failed (partially).

## ID (struct)

An ID is used to uniquely identify a specific item.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_GH_IO_Serialization_ID.htm)

### Constructors
- `public ID(string name)` — Constuctor for IDs.
- `public ID(string name, int index)` — Constructor for IDs.

### Methods
#### `public int CompareTo(ID other)`

Compares this ID to another ID.

**Parameters:**
- `other` (GH_IO.Serialization.ID) — The other ID.

**Returns:** `Int32` — Less than 0, if other comes before this. 0 is other is at the same place. More than zero otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_ID_CompareTo.htm)

#### `public bool Equals(ID other)`

Determines if two IDs are equal.

**Parameters:**
- `other` (GH_IO.Serialization.ID) — The other ID.

**Returns:** `Boolean` — true is both are equal.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_ID_Equals.htm)

#### `public override int GetHashCode()`

Gets the hash code.

**Returns:** `Int32` — A number representing the hash code.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_ID_GetHashCode.htm)

#### `public override string ToString()`

Gets a string representation for this ID.

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_Serialization_ID_ToString.htm)

### Properties
- `Hash` (Int32, get) — Gets the hash code.
- `HasIndex` (Boolean, get) — Gets whether the index has been set.
- `HasName` (Boolean, get) — Gets whether the name has been set. Every valid ID must have a name.
- `Index` (Int32, get) — Gets the index of this ID, if there is no valid index then -1 is returned.
- `Name` (String, get) — Gets the name of this ID.

